use semio_framework_job::{Generation, OperationId};
pub use semio_os_store_one_item_publication::ArtifactStoreOneItemLiveAuthority;
use semio_os_spr::HybridLogicalTimestamp;
use semio_framework_value::RetainedCloneProgress;
use std::sync::Arc;
use owner::{ArtifactStoreCanonicalOwner, ArtifactStoreCanonicalSource};

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
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::from_recovered_outcome(
        OperationId(128), Generation(0), [0; 32], 0, 1,
        HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 0 },
        "local".into(), None, None, None,
    ));
    let edit = Box::new(authority.next_edit(9u64, vec![1u64]));
    let mut owner = ArtifactStoreCanonicalSource::new(authority, edit);
    let mut observer = |_| true;
    let mut caller = NativeDecodeControl::new(1 << 24, &mut observer);
    let mut turns = 0;
    let mut blocked = 0;
    while !owner.ready() && turns < 8192 {
        let mut demand = owner.next_demand().unwrap();
        demand.depth += 1;
        let work = owner.next_work_demand().unwrap();
        let (step, _) = receive(&mut caller, |control| {
            control.with_allocated_effect_reported(demand, work, |_, effect| {
                owner.advance(&mut effect.nested(semio_framework_value::RetainedCloneGrant {
                    maximum_depth: demand.depth - 1, ..effect.grant()
                }))
            }).unwrap().unwrap()
        });
        let progress = step.unwrap().progress();
        if progress == RetainedCloneProgress::default() {
            blocked += 1;
            if blocked == 1 || blocked == 8 {
                eprintln!("Blocked at native source phase {} after {} funded turns. Demand {:?}, Work {}.", owner.phase, turns, demand, work);
            }
            if blocked == 8 { std::process::exit(1); }
        } else {
            blocked = 0;
        }
        turns += 1;
    }
    assert!(owner.ready(), "canonical source did not finish in 8192 turns");
    eprintln!("Canonical source reached ready after {} exactly funded turns.", turns);
    // The independent package law checks final original digest and settlement.
    std::process::exit(0);
}
