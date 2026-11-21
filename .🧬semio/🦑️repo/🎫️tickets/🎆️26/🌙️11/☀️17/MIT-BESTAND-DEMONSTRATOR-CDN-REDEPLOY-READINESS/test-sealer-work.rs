use semio_framework_job::{Generation, OperationId};
use semio_framework_value::factory::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_os_store_testing_prelude::{
    ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority,
    ArtifactStoreOneItemPreparationStep, HybridLogicalTimestamp,
};
use std::sync::Arc;

use semio_framework_value::retained_clone::{
    OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck,
    OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete,
    OriginalWorkRecipient,
};
use semio_framework_value::{NativeDecodeControl, ValueError};

struct Work;
impl OriginalWorkRecipient for Work {
    type Context<'turn,'source> = () where 'source:'turn;
    fn validate_context<'turn,'source>(&self, _: &()) -> Result<(), ValueError> where 'source:'turn { Ok(()) }
    fn source_turn<'turn,'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source:'turn { None }
    fn admits<'r,'turn,'source>(&mut self, _: &mut (), q: OriginalWorkQuote<'r>, spent: u64, next: u64) -> OriginalWorkQuoteComplete<'r> where 'source:'turn { q.complete(Ok(spent.saturating_add(next) <= 1 << 22)) }
    fn settle_partial<'r,'turn,'source>(&mut self, _: &mut (), c: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'source:'turn { c.complete(Ok(())) }
    fn recover_partial<'r,'turn,'source>(&mut self, _: &mut (), c: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'source:'turn { c.complete(Ok(())) }
}

fn receive<R>(control: &mut NativeDecodeControl<'_>, run: impl FnOnce(&mut NativeDecodeControl<'_>) -> R) -> R {
    let mut authority = OriginalWorkAuthority::new(Work);
    authority.with_loan(&mut (), |work|
        control.with_work_recipient(work, |control| Ok::<_, ValueError>(run(control)))).unwrap().unwrap()
}

fn main() {
    let source = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(OperationId(1), Generation(1),
        [0; 32], 0, 1, HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 0 },
        "cdn".into(), None, None, None));
    let mut stock = semio_framework_value::testing::physical_test_close_grant();
    stock.maximum_copy_bytes = stock.maximum_copy_bytes.max(semio_os_store_testing_prelude::ArtifactStoreOneItemSealer::<u64, u64>::constructor_copy_byte_demand());
    let edit = source.next_edit(9u64, vec![1u64]);
    let (mut owner, _) = source.begin_one_item_seal(edit, Arc::new(9u64),
        Arc::new(OwnedValueRetirementFactory::default()), Arc::new(SharedValueRetirementFactory::default()),
        stock).map_err(|error| error.0).unwrap();
    let mut observer = |_| true;
    let mut caller = NativeDecodeControl::new(1 << 24, &mut observer);
    let demand = owner.preparation_demands().unwrap();
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes,
        maximum_depth: stock.maximum_depth };
    receive(&mut caller, |control| {
        let before = control.actual_work();
        let step = owner.advance(grant, control).unwrap();
        assert!(matches!(step, ArtifactStoreOneItemPreparationStep::Progress(..)));
        assert_eq!(owner.last_turn_work(), 2);
        assert_eq!(u128::from(owner.last_turn_work()), control.actual_work() - before);
    });
    owner.begin_close();
    for _ in 0..5000 {
        if owner.terminal_is_empty() { println!("Verified native edit-seal receipt and teardown."); return; }
        receive(&mut caller, |control| {
            let step = control.with_effect(stock, 65536, |_, effect| owner.close_step(effect)).unwrap().unwrap();
            if let semio_framework_value::RetainedWorkPoll::Complete(turn) = step {
                turn.result.unwrap();
            }
        });
    }
    panic!("canonical sealer did not finish closing");
}
