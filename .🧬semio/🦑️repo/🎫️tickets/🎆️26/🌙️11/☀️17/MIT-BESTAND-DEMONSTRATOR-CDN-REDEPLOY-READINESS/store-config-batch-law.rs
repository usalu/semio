use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_job::{OperationId,OriginalRetirementPump};
use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement};
use semio_plugin_retained_command::typed_operation::OriginalPublicationFailure;
use semio_os_spr::ActorId;
use semio_os_store_command::HistoryLane;
use semio_os_store_config::lifted_store::create_config_envelope;
use semio_os_store_engine::lifted_store::config_store_from_initialized_runtime_with_owners;
use semio_os_store_engine_owners::{bounded_artifact_store_owners, complete_unscheduled_catalog};
use semio_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
use semio_os_store_one_item::ArtifactStoreOneItemGrant;
use semio_os_store_one_item_publication::ArtifactStoreOneItemAdvance;
use semio_os_store_snapshot_clone::config_apply::config_apply_preparation_factory;
use semio_s_artifact_procedural_3d_editor_config::{
    Generation3dConfig, Generation3dConfigMutation, SetCamera, SetSelectedGeneration,
};
use std::{future::Future, sync::Arc};

use semio_framework_value::retained_clone::{
    OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck,
    OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete,
    OriginalWorkRecipient,
};
use semio_framework_value::{NativeDecodeControl, ValueError};
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::native_decoding::{OriginalConstructionAuthority,NativeDecodeRetirementRecipient};

struct FixtureStoreConstruction { current: RetainedCloneGrant, submitted: RetainedCloneProgress }

impl semio_framework_value::native_decoding::OriginalConstructionRecipient for FixtureStoreConstruction {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<semio_framework_value::native_decoding::OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn grant<'a, 'turn, 'source>(&mut self, _: &mut (), quote: semio_framework_value::native_decoding::OriginalConstructionGrant<'a>) -> semio_framework_value::native_decoding::OriginalConstructionGrantComplete<'a> where 'source: 'turn {
        let demand = quote.demand();
        let result = if semio_framework_value::retained_clone::retained_clone_grant_admits(self.current, demand) { Ok(self.current) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original fixture construction demand exceeds its exact grant")) };
        quote.complete(result)
    }
    fn accept<'a, 'turn, 'source>(&mut self, _: &mut (), receipt: semio_framework_value::native_decoding::OriginalConstructionAccept<'a>) -> semio_framework_value::native_decoding::OriginalConstructionAcceptComplete<'a> where 'source: 'turn {
        let (granted, progress) = (receipt.grant(), receipt.progress());
        assert!(progress.fits(granted), "a store step spent more than the fixture receiving grant");
        let posted = self.submitted.checked_add(progress).map(|total| self.submitted = total);
        receipt.complete(posted)
    }
    fn submitted<'turn, 'source>(&self, _: &()) -> RetainedCloneProgress where 'source: 'turn { self.submitted }
}



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
    let mut construction=OriginalConstructionAuthority::new(FixtureStoreConstruction{current:STOCK,submitted:Default::default()});
    authority.with_loan(&mut (), |work|
        control.with_work_recipient(work, |control|
            construction.with_loan(&mut (),|loan|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(run(control)))).and_then(|inner|inner)
        )).unwrap().unwrap()
}




const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1,
    maximum_copy_bytes: 32_768 - 2 * std::mem::size_of::<Option<OriginalPublicationFailure>>(),
    maximum_capacity_bytes: 16_777_216,
    maximum_release_bytes: 1_048_576,
    maximum_depth: 64,
};

fn main() {
    let schema="generation3dcfg";
    let document="local-test";
    let initial=Arc::new(Generation3dConfig::default());
    let mut envelope=Box::pin(create_config_envelope::<Generation3dConfig, Generation3dConfigMutation>(schema, document, (*initial).clone(), None));
    let envelope=match envelope.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(envelope)=>envelope,
        std::task::Poll::Pending=>panic!("fixture config envelope should resolve immediately"),
    };
    let runtime=ArtifactStoreInitializationRuntime::new(document,schema,Arc::clone(&initial),[0;32],ActorId("local".into()));
    let factory=config_apply_preparation_factory::<Generation3dConfig,Generation3dConfigMutation>();
    let (owners,_)=bounded_artifact_store_owners(STOCK).unwrap_or_else(|error|panic!("{}",error.error));
    let owners=complete_unscheduled_catalog(owners.with_one_item_preparation(Arc::clone(&factory), STOCK)).unwrap();
    let mut store=config_store_from_initialized_runtime_with_owners(envelope,runtime,0,owners);
    let camera=CameraJson { x:94.75581571737445, y:-97.50833134679668, zoom:1.7844325616011099 };
    let camera_op=Generation3dConfigMutation::SetCamera(SetCamera{camera:camera.clone()});
    let selection="hexagonal-mushroom-column".to_string();
    let selection_op=Generation3dConfigMutation::SetSelectedGeneration(SetSelectedGeneration{selected_generation_id:Some(selection.clone())});
    let mut observer=|_|true;
    let mut recipient=NativeDecodeRetirementRecipient::new();
    let mut receiver=NativeDecodeControl::new(1<<26,&mut observer);
    receiver.install_retirement_recipient(&mut recipient).unwrap();
    let mode=std::env::args().nth(1);
    let operations=if mode.as_deref()==Some("same-batch"){vec![vec![camera_op,selection_op]]}else{vec![vec![camera_op],vec![selection_op]]};
    for (offset, mut mutations) in operations.into_iter().enumerate(){
        let row_count=mutations.len();
        let generation=store.generation_now();
        let revision=store.content_revision_now();
        let mut actor=Some(ControlledRetirement::retain(store.local_actor_id().0.clone()));
        let operation=OperationId(128+offset as u64);
        let (mut batch, admission)=store.admit_apply_batch(
            operation,generation,revision,&mut actor,&mut mutations,HistoryLane::Document,
            Some(&factory),&mut None,Some("setActiveExample"),None,false,STOCK,
        ).expect("actual Config batch admission").expect("funded real Config ingress");
        assert!(admission.fits(STOCK));
        assert!(actor.is_none()&&mutations.is_empty());
        eprintln!("[DEBUG] Config batch {offset} actual scoped copy stock {}",STOCK.maximum_copy_bytes);
        let grant=ArtifactStoreOneItemGrant::from_retained(STOCK);
        let mut blocked=0;
        let mut seen=batch.phase();
        let mut folded=0;
        for turn in 0..8192 {
            let result=receive(&mut receiver,|control|store.advance_apply_batch_with_turn_ceiling(&mut batch,STOCK.maximum_copy_bytes,grant,RetainedCloneGrant{maximum_items:64,..STOCK},control));
            let step=result.unwrap_or_else(|error|panic!("Config batch {offset} at {turn} {:?}: {error}",batch.phase()));
            let phase=batch.phase();
            folded=folded.max(batch.staged_items());
            if phase!=seen {
                eprintln!("[DEBUG] Config batch {offset} turn {turn} entered {phase:?}, folded {}, {:?}",batch.staged_items(),batch.progress());
                seen=phase;
            }
            match step {
                ArtifactStoreOneItemAdvance::Published(receipt, ownership)=>{
                    assert!(ownership.fits(STOCK));
                    assert_eq!(folded,row_count);
                    assert_eq!(store.generation_now(),generation+1);
                    assert_ne!(store.content_revision_now(),revision);
                    assert_eq!(batch.accepted_receipt(),Some(receipt));
                    assert!(batch.acknowledge_granted(STOCK).unwrap().fits(STOCK));
                    assert!(batch.accepted_receipt().is_none());
                    eprintln!("[DEBUG] Config batch {offset} published and acknowledged at {turn}, {row_count} folded, generation {}",store.generation_now());
                    break;
                }
                ArtifactStoreOneItemAdvance::PreparationProgress(_,receipt)=>{assert!(receipt.fits(STOCK));}
                ArtifactStoreOneItemAdvance::Blocked=>{}
                _=>panic!("Config batch {offset} turn {turn} fault {:?}, {:?}",batch.fault(),batch.progress()),
            }
            assert_eq!(store.generation_now(),generation);
            assert_eq!(store.content_revision_now(),revision);
            blocked=if matches!(step,ArtifactStoreOneItemAdvance::Blocked){blocked+1}else{0};
            assert!(blocked<16,"Config batch {offset} blocked at {turn}: {phase:?} {:?} Work {}, folded {}",batch.progress(),batch.last_turn_work(),batch.staged_items());
            assert!(turn<8191,"Config batch {offset} exceeded turn budget: {phase:?} {:?}",batch.progress());
        }
        let mut closed=false;
        for turn in 0..1024 {
            let pump=OriginalRetirementPump::new();
            let mut effect = pump.turn(1 << 16, u64::MAX, || Some(0), STOCK).unwrap();
            let Some((step,_))=effect.with_effect(|effect|batch.close_step(effect)).unwrap()else{continue};
            let step=step.unwrap_or_else(|_|panic!("Config batch {offset} close refused at {turn}"));
            assert!(step.progress().fits(STOCK));
            if batch.terminal_is_empty(){
                closed=true;
                eprintln!("[DEBUG] Config batch {offset} shell closed at {turn}");
                break;
            }
        }
        assert!(closed, "Config batch {offset} shell close blocked on {}",batch.closing_owner_witness());
    }
    assert_eq!(store.snapshot_ref().camera,camera);
    assert_eq!(store.snapshot_ref().selected_generation_id.as_deref(),Some(selection.as_str()));
    assert_ne!(initial.camera,camera);
    assert!(initial.selected_generation_id.is_none());
    std::mem::forget((store,factory));
}
