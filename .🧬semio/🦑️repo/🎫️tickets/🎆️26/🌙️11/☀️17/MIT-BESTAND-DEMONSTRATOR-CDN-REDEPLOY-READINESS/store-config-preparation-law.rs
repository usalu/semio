use semio_framework_job::{Generation, OperationId};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ArtifactOwnedValueRetirementFactory, SnapshotRetirementFactory};
use semio_framework_value::factory::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle};
use semio_os_spr::HybridLogicalTimestamp;
use semio_os_store_command::HistoryLane;
use semio_os_store_one_item::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationRequest};
use semio_os_store_snapshot_clone::config_apply::{ConfigApplyMutation, config_apply_preparation_factory};
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



#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct Config {
    camera: String,
    selected_generation_id: Option<String>,
}

#[derive(semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct SetConfig {
    camera: Option<String>,
    selection: Option<String>,
}

impl ConfigApplyMutation<Config> for SetConfig {
    fn exchange(self, post:&mut Config) -> Result<Self, (ValueError, Self)> {
        Ok(match self.camera {
            Some(camera) => Self {camera:Some(std::mem::replace(&mut post.camera,camera)),selection:None},
            None => Self {camera:None,selection:std::mem::replace(&mut post.selected_generation_id,self.selection)},
        })
    }
}

const GRANT:RetainedCloneGrant=RetainedCloneGrant {
    maximum_items:1,maximum_copy_bytes:31_000,maximum_capacity_bytes:16_777_216,
    maximum_release_bytes:1_048_576,maximum_depth:64,
};

fn drive(base:Arc<Config>, mutation:SetConfig, seq:i32) -> Arc<Config> {
    let operation=OperationId((128+seq) as u64);
    let generation=Generation(0);
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::from_recovered_outcome(
        operation,generation,[0;32],0,seq,HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 0 },
        "local".into(),None,None,None,
    ));
    let registry=SnapshotReadRegistryHandle::new();
    let lease=registry.try_issue(Arc::clone(&base)).unwrap_or_else(|_|panic!("original Config snapshot ticket"));
    let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<SetConfig>>=Arc::new(OwnedValueRetirementFactory::default());
    let snapshots:Arc<dyn SnapshotRetirementFactory<Config>>=Arc::new(SharedValueRetirementFactory::default());
    let request=ArtifactStoreOneItemPreparationRequest{operation,generation,base_revision:[0;32],lane:HistoryLane::Document,authority,base:SnapshotRead::new(Arc::clone(&base),lease),mutation,mutation_retirement:Arc::clone(&mutations),snapshot_retirement:Arc::clone(&snapshots)};
    let factory=config_apply_preparation_factory::<Config,SetConfig>();
    let grant=ArtifactStoreOneItemGrant::from_retained(GRANT);
    let (mut owner,birth)=factory.begin(request,grant).unwrap_or_else(|(error,_)|panic!("Config begin: {error}"));
    assert!(birth.fits(GRANT));
    let mut observer=|_|true;
    let mut receiver=NativeDecodeControl::new(1<<24,&mut observer);
    let mut stagnant=0;
    for turn in 0..8192 {
        let last=owner.checkpoint();
        let step=receive(&mut receiver,|control|owner.advance_with_turn_ceiling(grant.maximum_copy_bytes,grant,GRANT,control));
        match step.unwrap_or_else(|error|panic!("Config step {turn}: {}",error.message())) {
            ArtifactStoreOneItemPreparationStep::Prepared(check,receipt)=>{
                assert!(receipt.fits(GRANT));
                let prepared=owner.take_prepared().expect("actual Config candidate");
                assert_eq!(prepared.edit_digest(),check.digest);
                assert_eq!(prepared.edit().inverse.len(),1);
                let result=Arc::clone(prepared.post_snapshot());
                eprintln!("[DEBUG] Config item {seq} reached prepared at turn {turn}: {check:?}");
                let prepared=prepared.into_parts();
                let mut edit=Some(prepared.edit);
                let (mut aggregate,admission)=factory.begin_batch_digest(&mut edit,GRANT).unwrap().unwrap();
                assert!(admission.fits(GRANT) && edit.is_none());
                for aggregate_turn in 0..8192 {
                    if aggregate.ready() {
                        let (edit,digest,ownership)=aggregate.take_edit(GRANT).unwrap().unwrap();
                        assert!(ownership.fits(GRANT));
                        assert_eq!(digest,prepared.edit_digest);
                        assert_eq!(edit.inverse.len(),1);
                        assert_eq!(edit.forwards.len(),1);
                        assert!(aggregate.terminal_is_empty());
                        eprintln!("[DEBUG] Config item {seq} aggregate digest reached handback at turn {aggregate_turn}: {digest:02x?}");
                        std::mem::forget((edit,prepared.outcome_messages,prepared.applied_edit_id,prepared.tail_edit_id,prepared.local_actor));
                        break;
                    }
                    assert!(aggregate_turn<8191,"aggregate digest exceeded turn limit");
                    let quote=aggregate.next_digest_work_demand().unwrap();
                    let demand=aggregate.next_digest_demand().unwrap();
                    let step=receive(&mut receiver,|control|control.with_effect(GRANT,quote,|_,effect|aggregate.advance_digest(effect))).unwrap().unwrap().unwrap();
                    assert!(step.progress().fits(GRANT), "aggregate {seq} invalid receipt at {aggregate_turn}, quote={quote}, demand={demand:?}");
                    stagnant=if step.progress()==Default::default(){stagnant+1}else{0};
                    assert!(stagnant<8,"aggregate {seq} stalled at {aggregate_turn}, quote={quote}, demand={demand:?}");
                }
                std::mem::forget((owner,registry,mutations,snapshots,factory));
                return result;
            }
            ArtifactStoreOneItemPreparationStep::Progress(_,receipt)=>{assert!(receipt.fits(GRANT));}
            ArtifactStoreOneItemPreparationStep::Blocked=>{}
        }
        if last==owner.checkpoint(){stagnant+=1}else{stagnant=0}
        if stagnant==8{
            eprintln!("[DEBUG] Config item {seq} stalled at turn {turn}: {:?} Work {}",owner.checkpoint(),owner.last_turn_work());
            std::process::exit(1)
        }
    }
    panic!("Config did not prepare in bounded turns")
}

fn main() {
    let initial=Arc::new(Config{camera:"{}".into(),selected_generation_id:None});
    let camera=r#"{"x":-22,"y":56,"zoom":0.75}"#.to_string();
    let first=drive(Arc::clone(&initial),SetConfig{camera:Some(camera.clone()),selection:None},1);
    let last=drive(first,SetConfig{camera:None,selection:Some("hexagonal-mushroom-column".into())},2);
    assert_eq!(last.camera,camera);
    assert_eq!(last.selected_generation_id.as_deref(),Some("hexagonal-mushroom-column"));
    assert_eq!(initial.camera,"{}");
    assert!(initial.selected_generation_id.is_none());
    std::process::exit(0);
}
