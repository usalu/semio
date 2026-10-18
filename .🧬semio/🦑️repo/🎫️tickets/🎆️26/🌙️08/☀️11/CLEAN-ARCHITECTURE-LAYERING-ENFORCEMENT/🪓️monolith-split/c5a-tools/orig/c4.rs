//! 🔬️ Part 4 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_decl_views::media_contract::ARTIFACT_OUTPUT_CHUNK_BYTES;
use semio_framework_plugin_decl_views::AppOperationContext;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPoll;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportResult;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework_plugin_decl_emission::HostEvent;
use semio_framework_plugin_decl_emission::InteractionWrite;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::RetirementDemand;
use semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_reactor_app::addressed_action_view;
use semio_framework_plugin_reactor_app::admit_addressed_action_view;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_os_kernel as store;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_emission::AsyncTaskRun;
use semio_framework_plugin_decl_emission::AsyncTaskSource;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_abi::DerivedArtifactSpec;
use semio_framework_plugin_window_kits::LabelAxes;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_decl_emission::ToolCommandCatalog;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_config::WindowConfigReplaceMutation;
use semio_framework_plugin_window_kits::WindowKit;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{ContractApp, SingleDocumentApp, TEST_APP_WINDOW_BODY_KEY, TEST_COMMAND_RETAINED_GRANT, TEST_HOST_EVENTS, TEST_RESTART_SCHEMA, TEST_RESTART_TOOL, TEST_RETAINED_COMMAND_RAW_BYTES, TEST_RETAINED_COMMAND_TOOL, TestApp, TestCommand, app_maintenance_turn, archive_fault_text, close_member_admission_app, close_reserved_app, contract_app, contract_app_under_test, contract_composed_app, contract_composed_app_raw, drain_and_close_composed_fixture, drive_job_close, drive_single_document_archive, dv, inline_interaction_select, interaction_app_under_test, interaction_target_args, job_close_grant, label_in, meta, reads_child_count, register_test_child, reserved_action, settle_contract_app, settle_reserved, test_child_emit, test_retained_command_id, test_retained_command_payload, test_task};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_emission::{ActionMeta, Emit, TaskCtx, TaskResolution};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::{ArtifactApp, EphemeralSnapshot};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandle;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::{InteractionHoverState, NoDraftMutation};
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::{ActionKind, InteractionVerb, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID};
use semio_framework_job::InteractiveJob as _;
use semio_framework_ui_contract::{ActionId, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision};
use serde_json::json;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
include!("../🔬️plugin-runtime-plugin-builder-contract-part-4-x/🦀️.rs");


    /// 📨️ Takes the host events recorded on this test thread.
    fn take_test_host_events() -> Vec<HostEvent> {
        TEST_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
    }

    /// 🪟️ A view of the two-window roster `main-1`/`main-2` targeting `target` under `utility`.
    fn host_event_view(target: &str, utility: Option<&str>) -> ViewModel {
        let roster = vec![ViewWindowInstance { id: "main-1".into(), window_kind_id: "main".into() }, ViewWindowInstance { id: "main-2".into(), window_kind_id: "main".into() }];
        let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }.for_window_instance(target).expect("a roster window");
        ViewModel { active_utility_id: utility.map(str::to_string), ..view }
    }

    struct TestRetainedChildCommandWork {
        pub emit: Option<Emit<TestMutation, TestConfigMutation, NoDraftMutation>>,
    }

    impl semio_framework_plugin_retained_command::ArtifactCommandWork<TestApp> for TestRetainedChildCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(1)
        }

        fn step(&mut self, input: &semio_framework_plugin_retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_retained_command::ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
            let owner = input.snapshot_owner.expect("retained runtime supplies its immutable snapshot capability");
            assert!(std::ptr::eq(owner.as_ref(), input.snapshot));
            let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧪️tests/🧫️fixtures/🔒️immutable-reader.json")).expect("neutral immutable reader fixture");
            let observed = serde_json::to_value(owner.as_ref()).expect("independent snapshot projection");
            assert_eq!(observed, fixture["initial"]);
            let mut replacement = owner.clone();
            *std::sync::Arc::make_mut(&mut replacement) = serde_json::from_value(fixture["replacement"].clone()).expect("independent replacement snapshot");
            assert_eq!(serde_json::to_value(owner.as_ref()).unwrap(), fixture["initial"]);
            assert_eq!(serde_json::to_value(replacement.as_ref()).unwrap(), fixture["replacement"]);
            let semio_framework_plugin_retained_command::ArtifactCommandInputs { snapshot_owner: _, command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            self.emit.take().map(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Complete).ok_or_else(|| Fault::from("test-retained-child-work-repeated").into())
        }
    }

    async fn test_retained_child_command_payload(completion: ArtifactToolCompletion<TestApp>, emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation>) -> semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp> {
        semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin_retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot: std::sync::Arc::new(TestSnapshot::default()),
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 42, generation: 3, canonical_base_revision: [5; 32], retained: TEST_COMMAND_RETAINED_GRANT, authoring_seed: "authoring-seed-test".into() },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            1,
            Box::new(TestRetainedChildCommandWork { emit: Some(emit) }),
        )
    }

    /// 🎯️ The intent twin of {@link ContractApp::dispatch_typed}: `handle_intent_frame` answers with the
    /// same admission receipt every migrated dispatch answers with, so a law that reads the store right
    /// after it observed the previous revision. Settles the operation and rebuilds the settled
    /// mutations/inverse through the SAME `result_from_last_edit` the direct route uses.
    async fn handle_intent_settled(app: &mut ContractApp, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let verb = intent.action.name.to_string();
        let before_edit_id = app.0.test_last_edit_id();
        let before_tail = app.0.test_edit_tail_lengths();
        let mut admitted = PluginApp::handle_intent_frame(&mut app.0, intent, meta, identity).await?;
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
        admitted.requested_effects.extend(receipt.effects);
        admitted.events.extend(receipt.events);
        if let Some(scope) = receipt.ui_scope {
            admitted.ui_scope = scope;
        }
        let after_edit_id = app.0.test_last_edit_id();
        if after_edit_id.is_some() {
            let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
            let settled = app.0.test_result_from_last_edit(&verb, meta, tail_offset).await;
            admitted.mutations = settled.mutations;
            admitted.inverse_group = settled.inverse_group;
            admitted.history_patch = settled.history_patch;
        }
        Ok(admitted)
    }

    #[semio_framework_async_macros::async_test]
    async fn completed_media_cancel_moves_exact_output_to_bounded_cleanup() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let operation_id = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, 0);
        let handle = ArtifactMediaExportHandle {
            app_instance_id: meta().instance_id,
            parent_document_id: app.stores.store.envelope().id.clone(),
            operation_id,
            base_revision: semio_framework_job::RevisionId(9_007_199_254_740_993),
            generation: semio_framework_job::Generation(9_007_199_254_740_995),
        };
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES * 2);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.push(vec![2; 17]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES + 17));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES + 17));
        let mut donated_handle=Some(handle.clone());let mut source=Some(ArtifactMediaExportPoll::Complete(ArtifactMediaExportResult::structured(MediaType{class:MediaClass::Presentation,form:MediaForm::Sequence},"fixture.schema","audio/mpeg",chunks.clone()).expect("exact fixture media")));let mut received=None;
        assert!(ArtifactDownloadOutput::receive_media_export_original(&mut donated_handle,&mut source,&mut received,cx).expect("exact original media output"));let output=received.take().expect("funded download receiving");
        app.media.segmented_downloads.insert_admitted(operation_id.0, output);

        app.cancel_media_export(&handle).await.expect("completed output cancellation");
        assert!(!app.media.segmented_downloads.contains(operation_id.0));
        assert!(app.media.test_segmented_closure_contains(operation_id.0));
        assert_eq!(chunks.chunks_remaining(), 2, "cancellation only transfers ownership");
        assert_eq!(chunks.bytes_remaining(), ARTIFACT_OUTPUT_CHUNK_BYTES + 17, "cancellation performs no synchronous output destruction");

        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("first bounded cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.bytes_remaining(), 17);
        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("second bounded cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: 17, ..Default::default() }));
        assert_eq!(chunks.bytes_remaining(), 0);
        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("terminal cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert!(!app.media.test_segmented_closure_contains(operation_id.0));
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_command_download_seam_keeps_original_chunks_and_bounded_cancel_owner(){
        use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep};
        use semio_framework_plugin_retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactCommandInputs, ArtifactRetainedCommandJob};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["download"];
        struct DownloadWork{output:Option<ArtifactDownloadOutput>,first:bool,closing:bool,retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>}
        impl DownloadWork{fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.output)}}
        impl ArtifactCommandWork<TestApp> for DownloadWork{
            fn tool_id(&self)->&'static str{TEST_RETAINED_COMMAND_TOOL}
            fn extent(&self,_:&TestCommand,_:&TestSnapshot,_:&InteractionState,_:Option<&ArtifactOwnedToolJobContext<TestApp>>)->Option<usize>{Some(1)}
            fn step(&mut self,_:&ArtifactCommandInputs<'_,TestApp>,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {if self.first{self.first=false;return Ok(ArtifactCommandWorkStep::Progress{stage:"download-source",preview:b"{}"})}Ok(ArtifactCommandWorkStep::CompleteDownload{download:self.output.take().unwrap(),ephemeral:EphemeralEmit::default()})
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
            fn begin_close(&mut self){self.closing=true;}
            fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
                let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                    if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                    else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.output,&mut self.retirement,grant)}
                })?;
                step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
            }
            fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
            fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
            fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
            fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.depth)}
            fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_none()}
        }
        let make=||{let length=law["payloadBytes"].as_u64().unwrap()as usize;assert!(length>law["wireBytes"].as_u64().unwrap()as usize);let chunks=ArtifactOutputChunks::new(length);for index in 0..length.div_ceil(ArtifactOutputChunks::CHUNK_BYTES){let start=index*ArtifactOutputChunks::CHUNK_BYTES;let end=(start+ArtifactOutputChunks::CHUNK_BYTES).min(length);chunks.push((start..end).map(|index|(index%251)as u8).collect()).unwrap();}chunks.seal().unwrap();ArtifactDownloadOutput::new("original.txt","text/plain",None,chunks).unwrap()};
        for (busy,cancel) in [(false,false),(true,false),(false,true)]{
            let completion=ArtifactToolCompletion::<TestApp>::new();let consumer=completion.clone();let mut payload=test_retained_command_payload(completion.clone()).await;payload.work=Box::new(DownloadWork{output:Some(make()),first:true,closing:false,retirement:None});let mut job=ArtifactRetainedCommandJob::new(payload);let mut yielded=false;let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(41),semio_framework_job::Generation(3),policy.preparation,||Some(0),u64::MAX,1);
            for _ in 0..128{let result=caller.turn(policy.maintenance,|cx|Ok(if busy{completion.with_busy_test_lock(||semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut job,cx))}else{semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut job,cx)})).unwrap();match result{semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_)=>{yielded=true;if cancel{caller.cancel().cancel_now()}},semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint{..}=>{},semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete{..}=>break,semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_)=>break,semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled=>{assert!(cancel);break},_=>{}}}
            assert!(yielded);job.begin_close();for _ in 0..100000{if job.terminal_is_empty(){break}let grant=job_close_grant(&job);let before=caller.turn(semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},|cx|job.close_step(cx)).unwrap();assert!(matches!(before,InteractiveJobCloseStep::Pending{progress}if progress==Default::default())||matches!(before,InteractiveJobCloseStep::Blocked));let step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));assert!(!matches!(step,InteractiveJobCloseStep::Blocked))}assert!(job.terminal_is_empty());
            if busy||cancel{assert!(consumer.test_take_download().unwrap().is_none())}else{let (Ok(output),_)=consumer.test_take_download().unwrap().unwrap()else{panic!("original download completion")};let mut actual=Vec::new();while let Some(chunk)=output.chunks.take_chunk().unwrap(){assert!(chunk.len()<=law["chunkBytes"].as_u64().unwrap()as usize);actual.extend(chunk)}let expected:Vec<u8>=serde_json::from_value(serde_json::json!((0..law["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect::<Vec<_>>())).unwrap();assert_eq!(actual,expected);}
            caller.retire(policy.close);
        }
        let mut original=std::collections::VecDeque::with_capacity(law["queueSlots"].as_u64().unwrap()as usize);original.push_back("Mesh 😀".to_string());
        let payload_bytes=original[0].len();let backing_bytes=original.capacity()*std::mem::size_of::<String>();assert_eq!(std::mem::size_of::<String>(),law["vectorMetadataWords"].as_u64().unwrap()as usize*std::mem::size_of::<usize>());
        let birth=semio_framework_value::retirement::owned_retirement_birth_bytes::<std::collections::VecDeque<String>>();
        let admission=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_capacity_bytes:birth,..semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation};
        let (mut retirement,admitted)=semio_framework_value::retirement::admit_owned_retirement(original,admission).unwrap_or_else(|_|panic!("original queue typed constructor refused its funded grant"));
        assert!(admitted.fits(admission));let (mut born,mut released)=(admitted.retained_capacity_bytes,admitted.released_bytes);
        for _ in 0..100000{if retirement.terminal_is_empty(){break}
            let copy=retirement.next_copy_byte_demand().unwrap();
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap().max(1)};
            assert_eq!(retirement.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());
            let progress=retirement.close_step(grant).unwrap().progress();assert!(progress.fits(grant));born+=progress.retained_capacity_bytes;released+=progress.released_bytes;}
        assert!(retirement.terminal_is_empty());assert_eq!(released,payload_bytes+backing_bytes+born,"original queue backing allocation remains owned until its exact byte credits drain");

    }

    #[semio_framework_async_macros::async_test]
    async fn retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧪️tests/🧫️fixtures/🧩️child-prepublication-close.json")).expect("language-neutral child close fixture");
        let expected_ids = fixture["children"].as_array().expect("child close fixture rows").iter().map(|child| child["childId"].as_str().unwrap().to_string()).collect::<Vec<_>>();
        let expected_order = fixture["expectedRetirementOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        let build_emit = || {
            let children = fixture["children"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    let emitted = test_child_emit(child["slot"].as_str().unwrap(), child["childId"].as_str().unwrap(), &[TestMutation::SetCount(SetCount { value: index as i32 + 1 })]);
                    assert!(emitted.ops.iter().all(|op| !op.is_empty()), "real composite mutation owns nonempty encoded bytes");
                    emitted
                })
                .collect();
            Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: 7 })], child_emits: children, ..Default::default() }
        };
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,1);
        let drive_to_publish = |job:&mut semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>,caller:&mut semio_framework_job::OriginalJobFixtureCaller| {
            for expected_stage in ["preflight", "work"] {
                match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(job,cx))).unwrap() {
                    semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                    _ => panic!("retained child job failed at {expected_stage}"),
                }
            }
        };
        let close_and_observe = |job:&mut semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>,caller:&mut semio_framework_job::OriginalJobFixtureCaller| {
            use semio_framework_value::retained_clone::RetainedCloneGrant;
            let initial = job.test_pending_emit_shape().expect("pre-publication emit owner");
            assert_eq!(initial, (2, 1, expected_ids.clone()));
            job.begin_close();
            let quoted = job_close_grant(&*job);
            assert_eq!(caller.turn(RetainedCloneGrant { maximum_items: 0, ..quoted },|cx|job.close_step(cx)).unwrap(), semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "zero item grant preserves the exact emit owner");
            let raw = job_close_grant(&*job);
            let step = caller.turn(raw,|cx|job.close_step(cx)).unwrap();
            assert!(step.progress().fits(raw), "empty reserved raw allocation retires within its exact grant");
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "raw allocation retirement preserves the exact emit owner");
            let next = job_close_grant(&*job);
            let demand=semio_framework_value::RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().unwrap(),capacity_bytes:job.next_close_capacity_byte_demand(next.maximum_copy_bytes).unwrap(),release_bytes:job.next_close_release_byte_demand().unwrap(),depth:job.next_close_depth_demand().unwrap()};
            for starved in [
                (demand.copy_bytes != 0).then(|| RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..next }),
                (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..next }),
                (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..next }),
            ]
            .into_iter()
            .flatten()
            {
                assert_eq!(caller.turn(starved,|cx|job.close_step(cx)).unwrap(), semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "a grant short on any axis preserves the exact child payload");
            }
            let mut retired = Vec::new();
            let mut prior_children = expected_ids.len();
            for _ in 0..10_000 {
                let grant = job_close_grant(&*job);
                let step = caller.turn(grant,|cx|job.close_step(cx)).unwrap();
                assert!(step.progress().fits(grant) && grant.maximum_items <= fixture["maximumItems"].as_u64().unwrap() as usize);
                if let Some((children, parents, _)) = job.test_pending_emit_shape() {
                    assert_eq!(parents, 1, "parent lane remains owned until every child is terminal");
                    if children < prior_children {
                        retired.push(fixture["children"][prior_children - 1]["id"].as_str().unwrap());
                        prior_children = children;
                    }
                } else {
                    break;
                }
            }
            assert_eq!(retired, expected_order);
            drive_job_close(job,caller);
            assert!(job.terminal_is_empty());
        };

        let direct_completion = ArtifactToolCompletion::<TestApp>::new();
        let _direct_consumer = direct_completion.clone();
        let mut direct = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(direct_completion, build_emit()).await);
        drive_to_publish(&mut direct,&mut caller);
        close_and_observe(&mut direct,&mut caller);

        let duplicate_completion = ArtifactToolCompletion::<TestApp>::new();
        let duplicate_consumer = duplicate_completion.clone();
        duplicate_completion.complete(Ok(Emit::default()), EphemeralEmit::default()).expect("prefill completion once");
        let mut duplicate = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(duplicate_completion, build_emit()).await);
        drive_to_publish(&mut duplicate,&mut caller);
        
        match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut duplicate,cx))).unwrap() {
            semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
            _ => panic!("duplicate completion must reject the handoff"),
        }
        assert!(duplicate.test_pending_emit_shape().is_some(), "duplicate rejection returns the exact emit owner");
        close_and_observe(&mut duplicate,&mut caller);
        let (existing, _) = duplicate_consumer.take_emit().expect("prefilled completion remains readable").expect("prefilled completion remains assigned");
        assert!(existing.expect("prefilled completion remains successful").child_emits.is_empty());

        let busy_completion = ArtifactToolCompletion::<TestApp>::new();
        let busy_consumer = busy_completion.clone();
        let mut busy = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(busy_completion.clone(), build_emit()).await);
        drive_to_publish(&mut busy,&mut caller);
        busy_completion.with_busy_test_lock(|| {
            
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut busy,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
                _ => panic!("busy completion must reject the handoff"),
            }
        });
        assert!(busy.test_pending_emit_shape().is_some(), "busy rejection returns the exact emit owner");
        close_and_observe(&mut busy,&mut caller);
        assert!(busy_consumer.take_emit().expect("busy completion remains readable").is_none(), "busy completion remains untouched");
        caller.retire(policy.close);
    }

    fn interaction_empty_target_args(extra: Value) -> DslValue {
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String("[]".into())));
        }
        object
    }

    /// 🧪️ Same wire shape as {@link interaction_target_args} for a BATCH of targets — the shape a
    /// marquee release (or a pick that resolved several rendered instances) carries.
    fn interaction_targets_args(extra: Value, granularity: &str, ids: &[String]) -> DslValue {
        let targets = serde_json::to_string(&ids.iter().map(|id| InteractionTarget { granularity: granularity.into(), id: id.clone() }).collect::<Vec<_>>()).expect("targets serialize");
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String(targets)));
        }
        object
    }

    /// 🧾️ Neither ephemeral lane may appear in history: they have no edits, no undo, and no
    /// command-log rows of their own — the document's single edit is the only thing recorded.
    ///
    /// ↩️ Undo rolls back the DOCUMENT; the ephemeral lanes are not restored, because they
    /// were never part of the undoable gesture in the first place.
    #[semio_framework_async_macros::async_test]
    async fn a_command_reaches_both_ephemeral_lanes_without_touching_history() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧪️tests/🧫️fixtures/🧬️request-context.json")).expect("neutral captured-context fixture");
        let vectors = &fixture["ephemeralPublication"];
        let mut app = contract_app(mounted_policy, &mut identity).await;
        assert_eq!(app.stores.presence_store.generation().await, 0);
        assert_eq!(app.stores.transient_store.generation().await, 0);

        let frame = meta();
        let started = app.test_mount_typed_command(TestCommand::Increment, "increment", &frame, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("mount increment");
        assert!(started.mutations.is_empty(), "mounting does not publish a producer emission");
        assert_eq!(app.stores.presence_store.generation().await, 0, "dispatch did not run an ephemeral prelude");
        assert_eq!(app.stores.transient_store.generation().await, 0, "dispatch did not apply transient work before its worker completion");
        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentEdits": app.stores.test_store().await.envelope().vcs.edits.len() }), vectors["mounted"]);
        settle_contract_app(&mut app,&mut identity,cx).await;

        assert_eq!(app.stores.presence_store.generation().await, 1, "presence lane never received the command's emission");
        assert_eq!(app.stores.transient_store.generation().await, 1, "transient lane never received the command's emission");
        assert_eq!(
            app.ephemeral_snapshot().await,
            EphemeralSnapshot { presence: PublicationPresence { revision: 1 }.encode_pack(), presence_generation: 1, transient_generation: 1, interaction: Vec::new(), tool_run: None, history_edit: None },
            "object-safe channel snapshot must carry the typed presence pack, both generations, and (declaring no interaction domain) empty interaction bytes"
        );

        assert_eq!(app.stores.test_store().await.envelope().vcs.edits.len(), 1, "an ephemeral lane leaked into the document's edit log");

        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentEdits": app.stores.test_store().await.envelope().vcs.edits.len() }), vectors["completed"]);
        app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("undo");
        assert_eq!(app.stores.test_snapshot().await.count, 0);
        assert_eq!(app.stores.presence_store.generation().await, 1, "undo must not rewind presence");
        assert_eq!(app.stores.transient_store.generation().await, 1, "undo must not rewind transient");
        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentCount": app.stores.test_snapshot().await.count }), vectors["undone"]);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📥️ Drives one stepped document archive load on `app` to its terminal status and acknowledges it.
    async fn drive_test_archive_load(app: &mut VcsArtifactApp<TestApp, TestMembers>, operation: u64, archive: protocol::DocumentArchivePack) -> protocol::DocumentArchiveLoadStatus {
        PluginApp::begin_document_archive_load(app, operation, archive).expect("the runtime admits the archive");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(app, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(app, operation).expect("acknowledge");
        status
    }

    /// 🧒️ Registering a member is the RUNTIME's job; declaring it on the parent snapshot is the
    /// APP's: the runtime admits a member only under the identity its parent's snapshot declares.
    ///
    /// 📤️ Persist exactly what the host does: the document archive (`ReadDocumentArchive`) — the
    /// parent pack and `.spr` plus one owner-stamped entry per live member (design §21.6: the
    /// recursive archive is the single carrier).
    ///
    /// 📥️ Reload into a FRESH app through the stepped archive load (`LoadDocumentArchive`). The
    /// child comes back as its OWN live store, at the value its own history ended on, owned by the
    /// reloaded document.
    ///
    /// 🚫️ The members travel WITH the parent: the same archive without them is refused by the
    /// ownership closure (`closure-rejected`), so a composed parent never loads half — there is no
    /// "parent first, children later" route.
    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the migrated composite gesture settles before it is persisted");
        assert_eq!(app.stores.test_snapshot().await.slot.len(), 1, "the live parent declares exactly its one member");

        let archive = PluginApp::document_archive(&app).await.expect("the composed document archives its closure");
        let document = app.stores.store.envelope().id.clone();
        assert_eq!(
            archive.members.iter().map(|entry| (entry.owner.parent.artifact_id.as_str(), entry.owner.slot.as_str(), entry.owner.child_id.as_str(), entry.reference.artifact_id.as_str())).collect::<Vec<_>>(),
            vec![(document.as_str(), "slot", "child-1", "child-1")],
            "the archive carries the one member, owner-stamped to the document"
        );

        let mut reloaded = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let status = drive_test_archive_load(&mut reloaded, 21, archive.clone()).await;
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
        assert_eq!(reloaded.test_snapshot().await.slot.len(), 1, "the reloaded parent carries its own declaration through the pack");
        let child = reloaded.child_store("slot", "child-1").await.expect("child restored");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the reloaded child lost its own edit history");
        assert_eq!(child.owner_ref().map(|owner| (owner.parent.artifact_id, owner.slot, owner.child_id)), Some((document.clone(), "slot".to_string(), "child-1".to_string())), "the reloaded child is owned by the reloaded document");

        let mut halved = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let mut parent_alone = archive;
        parent_alone.members.clear();
        let refused = drive_test_archive_load(&mut halved, 22, parent_alone).await;
        let refusal = format!("{:?}", semio_framework_diagnostic::decode_fault_bytes(&refused.fault));
        assert_eq!(refused.state, protocol::DocumentArchiveLoadState::Fault, "a parent that declares a member never loads without it: {refusal}");
        assert!(refusal.contains("closure-rejected"), "the ownership closure names the refusal: {refusal}");
        assert!(halved.test_snapshot().await.slot.is_empty() && halved.child_store("slot", "child-1").await.is_none(), "a refused load leaves the open document as it was");

        drain_and_close_composed_fixture(&mut halved, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut reloaded, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ↩️ Store-level undo bypasses `ArtifactApp::handle` entirely — this is exactly where the
    /// `thread_local!` child caches this view replaces used to go stale.
    #[semio_framework_async_macros::async_test]
    async fn the_child_content_view_never_goes_stale_across_undo_and_redo() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        assert_eq!(reads_child_count(&app).await, 7);

        app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("undo");
        assert_eq!(reads_child_count(&app).await, 0, "the view must reflect the child's undone state");
        app.dispatch_action("redo", None, &meta(), &mut identity,cx).await.expect("redo");
        assert_eq!(reads_child_count(&app).await, 7, "the view must reflect the child's redone state");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪢️ `child-b` is registered but NEVER targeted by the composite gesture below — its
    /// `tail_group_id()` stays `None`, the textbook "foreign tail" `GroupUndoReport` must
    /// skip rather than abort the whole group over.
    ///
    /// The parent reverted...
    ///
    /// ...child-a (the real group member) reverted too...
    ///
    /// ...and child-b — a genuine foreign tail, never touched by this group — is reported as
    /// SKIPPED, not silently dropped nor allowed to abort the rest of the group.
    #[semio_framework_async_macros::async_test]
    async fn group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-a").await;
        register_test_child(&mut app, "child-b").await;

        let before = app.stores.test_snapshot().await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-a".into(), child_value: 5 }, &meta(), &mut identity,cx).await.expect("composite edit");
        assert_eq!(app.stores.test_snapshot().await.label, "composite");

        let result = app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("group undo");

        assert_eq!(app.stores.test_snapshot().await, before);
        let TestMembers::Child(child_a_store) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("child-a").member;
        assert_eq!(child_a_store.snapshot().expect("child-a snapshot").count, 0);
        assert!(!result.diagnostics.is_empty(), "child-b's foreign tail must surface a diagnostic, not vanish silently");
        assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");

    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }


    /// ⏪️ Revert-to-command walks the DOCUMENT lane alone (design §20.13, L4): "leave the TARGET row applied, undo everything
    /// after it" — reverting to the first increment undoes the second and leaves the config the interleaved selects wrote.
    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_walks_the_document_lane_and_leaves_config_alone() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta(), &mut identity,cx).await.expect("select a");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta(), &mut identity,cx).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment again");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 2, "two document rows and no config row");
        let first = history.commands.iter().min_by_key(|entry| entry.seq).expect("the first increment row");
        assert!(first.revertible, "a document row is revertible");
        reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first.seq }))), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1, "reverting to the first increment undoes the second");
        assert_eq!(app.stores.test_config().await.selected, Some("b".to_string()), "the config lane is never walked");
        let after = app.render_ctx().test_history().await;
        assert_eq!(after.commands.len(), 3, "the revert appends one History-kind row");
        assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_actor_ingress_admits_undeclared_window_kind() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation, ViewWindowInstance};
        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let before = app.stores.test_store().await.applied_edit_ids().len();
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_instances: vec![ViewWindowInstance { id: "world-1".into(), window_kind_id: "world".into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "missing-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        assert!(semio_framework_plugin_reactor_app::addressed_action_view(&view, &invocation.address).is_err(), "strict window projection rejects a missing instance");
        let admitted = semio_framework_plugin_reactor_app::admit_addressed_action_view(&view, &invocation.address).expect("reserved undo is admitted without window ownership");
        let meta = ActionMeta { view_state: Some(admitted), ..meta() };
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;let admitted=artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta,&mut identity,cx).await.expect("actor reserved undo");
        assert!(admitted.mutations.is_empty());
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)), "first turn must admit spawn-job");
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first turn must not commit history");
        let result = settle_reserved(&mut app, admitted).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.stores.test_snapshot().await.count, 0, "history order must regress through actor ingress");
        assert!(app.stores.test_store().await.applied_edit_ids().len() < before);
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 Browser #37 mixed stack: document example under chrome-top Resize Window.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_shell_then_falls_through_to_document_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("document example");
        let _ = app.render_ctx().test_history().await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
         &mut identity)
        .await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let before = app.render_ctx().test_history().await;
        assert!(before.commands.iter().any(|entry| label_in(&entry.label, Locale::En).contains("Set Active Example")));
        assert_eq!(before.commands.first().map(|entry| label_in(&entry.label, Locale::En)), Some("Resize Window"));

        let first = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(
            first.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first undo must pop the chrome-top shell, not the document example");
        assert!(first.history_patch.as_ref().is_some_and(|patch| patch.can_undo && !patch.upserts.is_empty()));

        let second = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(second.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })));
        assert_eq!(app.stores.test_snapshot().await.count, 0, "second undo must apply the document inverse");
        assert!(second.history_patch.as_ref().is_some_and(|patch| patch.can_redo && !patch.upserts.is_empty()));

        reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1, "redo must restore the document example before the shell");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 §3.2's user-visible half: clicking a window between two edits noted `shell.windowActivate`,
    /// which the chrome branch then popped instead of the user's own edit — undo looked like it did
    /// nothing and the host replayed the chrome id into the guest. A note with no declared inverse is
    /// not an undo target, so the DOCUMENT edit is what `undo` pops and no replay leaves the guest.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_steps_over_undeclared_chrome_and_pops_the_document_edit() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("document edit");
        let _ = app.render_ctx().test_history().await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({ "commandId": "shell.windowActivate", "label": "Activate Window", "detail": { "windowId": "trinity-jack-graph" } }))),
         &mut identity)
        .await;
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.first().map(|entry| label_in(&entry.label, Locale::En)), Some("Activate Window"), "the window activation is the newest row");
        assert!(before.commands.iter().find(|entry| entry.action_id == "shell.windowActivate").is_some_and(|entry| !entry.revertible), "a note that declared no inverse is not an undo target");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let undone = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(
            undone.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })),
            "undo must not replay a window activation into the guest"
        );
        assert_eq!(app.stores.test_snapshot().await.count, 0, "undo must pop the document edit, not the window activation");
        let after = app.render_ctx().test_history().await;
        assert!(after.commands.iter().any(|entry| entry.action_id == "shell.windowActivate"), "the chrome row stays in the append-only log");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ `commands` is newest-first — take the MINIMUM seq among "increment" entries to target inc1, not inc2.
    ///
    /// 🧾️ `commands` is newest-first — the just-appended revert entry is the FIRST element, not the last.
    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_restores_the_snapshot_and_appends_one_entry() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc1");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc2");
        assert_eq!(app.stores.test_snapshot().await.count, 2);
        let first_increment_seq = app.render_ctx().test_history().await.commands.iter().filter(|entry| entry.action_id == "increment").map(|entry| entry.seq).min().expect("first increment entry");
        let before_len = app.render_ctx().test_history().await.commands.len();

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first_increment_seq }))), &mut identity).await;
        assert!(result.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1, "revert leaves the target edit applied, undoing only what came after it");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), before_len + 1, "exactly one entry appended for the revert itself");
        assert_eq!(history.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn consecutive_config_only_view_dispatches_add_no_history_rows() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for id in ["node-1", "node-2", "node-3"] {
            app.dispatch_typed(TestCommand::Select { id: Some(id.into()) }, &meta(), &mut identity,cx).await.expect("select");
        }
        assert!(app.render_ctx().test_history().await.commands.is_empty(), "three config-only Views add no row");
        assert_eq!(app.stores.test_config().await.selected, Some("node-3".to_string()));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn note_shell_command_is_intercepted_before_the_app_and_records_each_repeat() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let args = dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "detail": "dark" }));
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &mut identity).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty(), "interception must happen before the app ever sees noteShellCommand");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "os.setThemeId");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(label_in(&entry.label, Locale::En).contains("dark"));

        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &mut identity).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty());
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 2);
        assert!(history.commands.iter().all(|entry| entry.count == 1));
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📨️ LAW: a utility switch a dispatch's view shows is exactly one `UtilityChanged` for its window, delivered before the
    /// dispatch; the first view of a window, an unchanged utility and a window the view names no utility for deliver none.
    #[semio_framework_async_macros::async_test]
    async fn a_utility_switch_in_a_dispatch_view_delivers_one_host_event_to_its_window() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        take_test_host_events();
        let under = |view: ViewModel| ActionMeta { view_state: Some(view), ..meta() };
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("select"))), &mut identity,cx).await.expect("the first view");
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("select"))), &mut identity,cx).await.expect("the same utility");
        assert_eq!(take_test_host_events(), Vec::new(), "neither the first view of a window nor an unchanged utility is a switch");
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("brush"))), &mut identity,cx).await.expect("the switch");
        assert_eq!(take_test_host_events(), vec![HostEvent::UtilityChanged { window_id: "main-1".into(), from: Some("select".into()), to: Some("brush".into()) }]);
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-2", None)), &mut identity,cx).await.expect("another window");
        assert_eq!(take_test_host_events(), Vec::new(), "main-2 is seen first, and this view names no utility for main-1");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📨️ LAW: a forwarded `hostEvent{windowId, kind}` reaches the app as its typed event for that window, the app's typed
    /// answer dispatches without a history row of its own, and an unknown kind is refused.
    #[semio_framework_async_macros::async_test]
    async fn a_forwarded_host_event_reaches_the_app_and_its_answer_is_never_a_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        take_test_host_events();
        let under = ActionMeta { view_state: Some(host_event_view("main-1", None)), ..meta() };
        for (kind, event) in [
            ("blur", HostEvent::WindowBlurred { window_id: "main-2".into() }),
            ("captureLost", HostEvent::PointerCaptureLost { window_id: "main-2".into() }),
            ("retiring", HostEvent::Retiring { window_id: "main-2".into() }),
        ] {
            app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(dv(json!({ "windowId": "main-2", "kind": kind }))), &under, &mut identity).await.unwrap_or_else(|fault| panic!("{kind}: {fault:?}"));
            assert_eq!(take_test_host_events(), vec![event], "{kind}");
        }
        let history = app.render_ctx().test_history().await;
        assert!(history.commands.is_empty(), "the answer to a blur is never a history row: {:?}", history.commands.iter().map(|entry| &entry.action_id).collect::<Vec<_>>());
        let refused = app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(dv(json!({ "windowId": "main-1", "kind": "wobble" }))), &under, &mut identity).await;
        assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new("hostEvent.invalid")));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`) acceptance: an Activate intent on a test app
    /// produces the mutation's document change AND a command-log entry within the SAME
    /// `handle_intent_frame` call — the reactor's own `poll` never needs a second turn to see it,
    /// since `plugin_dispatch_intents` (which calls this) runs before the SAME turn's
    /// `dirty_render` pass.
    #[semio_framework_async_macros::async_test]
    async fn activate_intent_dispatches_through_the_typed_command_path_same_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("counter").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::try_v1("app", "incrementViaCommand").expect("bounded fixture"),
            args: None,
            input: None,
            seq: 1,
        };
        let result = handle_intent_settled(&mut app, &intent, &meta(), &mut identity,cx).await.expect("an Activate intent on a Mutation-kind action must dispatch");
        assert_eq!(app.stores.test_snapshot().await.count, 1, "the mutation must have applied");
        assert_eq!(result.mutations.len(), 1, "the returned invocation carries the one mutation the intent caused");
        let history = app.render_ctx().test_history().await;
        assert!(history.commands.iter().any(|entry| entry.action_id == "incrementViaCommand"), "a command-log entry must have been recorded for the intent");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎯️ M1 acceptance: a `View`-kind action arriving as an intent and returning artifact ops
    /// still hard-faults — kind discipline survives the new intent path unchanged, because
    /// `handle_intent_frame` routes through the SAME `dispatch_typed_command_inner` every other
    /// command path uses.
    #[semio_framework_async_macros::async_test]
    async fn view_kind_intent_returning_operations_hard_faults() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("bad").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::try_v1("app", "badView").expect("bounded fixture"),
            args: None,
            input: None,
            seq: 2,
        };
        let error = handle_intent_settled(&mut app, &intent, &meta(), &mut identity,cx).await.expect_err("a View-kind intent emitting operations must be rejected");
        assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default(), "kind discipline must block the mutation");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪢️ Regression guard for `result_from_last_edit`'s `tail_offset` slicing: even though the
    /// transaction's open edit accumulates every tick's operations (3 after this loop), each dispatch's
    /// `InvocationResult` must report only the operation IT just added — never re-serializing the whole
    /// growing edit into every `KernelMutation`/`UndoGroup` on every single dispatch.
    ///
    /// The narrowed per-dispatch reporting must not affect the one-edit/undo semantics of the transaction.
    #[semio_framework_async_macros::async_test]
    async fn a_streamed_tick_reports_only_its_own_new_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::StreamLabel { value: "a".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel a");
        app.dispatch_typed(TestCommand::StreamLabel { value: "ab".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel ab");
        let result = app.dispatch_typed(TestCommand::StreamLabel { value: "abc".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel abc");
        assert_eq!(result.mutations.len(), 1, "must report only this dispatch's new operation, not the whole open edit");
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "abc".into() })).unwrap());
        assert_eq!(
            result.mutations[0].inverse.inverse_diff.payload,
            protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "ab".into() })).unwrap()]),
            "the new operation's own inverse undoes back to the pre-dispatch label, not the whole gesture"
        );
        assert_eq!(result.inverse_group.mutations.len(), 1);
        assert_eq!(result.inverse_group.inverse_mutations.len(), 1);
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        app.dispatch_typed(TestCommand::StreamLabel { value: "abcd".into(), commit: true }, &meta(), &mut identity,cx).await.expect("streamLabel commit");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🐁️ Empty targets clears the channel (see `next_hover`'s "empty batch clears" law).
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_is_ephemeral_and_never_touches_the_persisted_interaction_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        let edits_after_select = app.stores.interaction_store.envelope().vcs.edits.len();

        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;

        assert_eq!(app.stores.interaction_store.envelope().vcs.edits.len(), edits_after_select, "hover must never mint a persisted interaction_store edit");
        assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.clone()), Some(vec!["item-1".to_string()]));

        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&dv(json!({ "domainId": "items", "channel": "pointer", "targets": "[]" }))), &mut identity).await;
        assert!(app.interaction_state().await.hover.get("items").is_none(), "an emptied hover channel is removed, not left as an empty entry");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.21 — `interactionSelect` leftover `Invocation.output` carries InteractionView selected ids + lock.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_job_completion_publishes_interaction_view_on_leftover() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let locked = view.get("locked").expect("lock state included on leftover InteractionView");
        assert_eq!(locked.get("item-1").and_then(semio_framework_value::DslValue::as_bool), Some(false));
        let gumball = view.get("gumball").expect("gumball leftover");
        assert_eq!(gumball.get("active").and_then(semio_framework_value::DslValue::as_bool), Some(false), "leftover selection must not arm the world gumball — the guest selectionJson lane owns utility-aware gumballActive");
        assert_eq!(gumball.get("anchorId").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 Empty-target `interactionSelect` with `replace` clears the domain (background deselect) even when an object is hovered.
    #[semio_framework_async_macros::async_test]
    async fn emitted_empty_replace_write_clears_selection_while_hover_remains() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let clear = Emit { interaction_writes: vec![InteractionWrite::replace("items", "object", std::iter::empty::<String>())], ..Default::default() };
        app.test_dispatch_emit("canvasPointerDown", clear, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("explicit selection clear lands");
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.ids.is_empty()));
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.anchor_id.is_none()));
        assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.as_slice()), Some(["item-1".to_string()].as_slice()));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_target_interaction_select_clears_selection_while_hover_remains() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_empty_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }))), &mut identity).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.is_empty(), "empty-target replace interactionSelect must clear selectedIds, got {ids:?}");
        let hover = view.get("hoverTarget").expect("hoverTarget");
        assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — a guest-emitted `interactionSelect` is applied
    /// in the SAME turn as the app command that carried it: the effect never reaches the host, the
    /// selection snapshot already moved when the carrier's result leaves, the carrier's `ui_scope` is
    /// widened by the verb's app-declared interaction refresh scope, and the leftover `InteractionView`
    /// rides the carrier's own `output` (the lane a host-dispatched verb publishes on). Every other
    /// effect still reaches the host untouched, in order. Fails-before: the effect was handed to the
    /// host and the pick landed one guest round trip later, behind any pointer input issued meanwhile.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_interaction_select_is_folded_into_the_carrying_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let emit = Emit {
            effects: vec![Effect::Navigate { uri: "semio://home".into() }, inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))],
            ui_scope: UiDirtyScope::None,
            ..Default::default()
        };
        let result = app.test_dispatch_emit("canvasPointerUp", emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the carrying command lands");
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }], "the interaction verb never reaches the host; every other effect does, in order");
        assert!(result.diagnostics.is_empty(), "a well-formed verb applies without diagnostics: {:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert_eq!(snapshot.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the selection snapshot moved inside the carrying turn");
        let declared = <TestApp<false> as ArtifactApp>::interaction_scope(InteractionVerb::Select, &["items"]).expect("the fixture declares its select scope");
        assert_eq!(result.ui_scope, declared, "a `None` carrier scope widens to exactly the verb's declared interaction refresh scope");
        let view = result.output.get("interactionView").expect("the leftover InteractionView rides the carrier's output");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let history = app.render_ctx().test_history().await;
        assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ A second carrier whose folded `interactionSelect` names no target with `replace` clears the
    /// selection (the background-click deselect), and its scope is the UNION of the carrier's own
    /// partial scope and the verb's declared partial scope — widest wins, nothing the app asked to
    /// refresh is dropped.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_empty_replace_select_clears_and_unions_the_carrier_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let pick = Emit { effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))], ui_scope: UiDirtyScope::None, ..Default::default() };
        app.test_dispatch_emit("canvasPointerUp", pick, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("pick lands");
        assert_eq!(app.interaction.test_interaction_selection_snapshot(&app.stores).selection.get("items").map(|selection| selection.ids.len()), Some(1));

        let carrier_scope = UiDirtyScope::Partial { window_bodies: vec!["some.window".into()], panel_bodies: Vec::new(), utilities: true, tools: false, engagements: false, measures: false, labels: false };
        let clear = Emit { effects: vec![inline_interaction_select(interaction_empty_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" })))], ui_scope: carrier_scope, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerDown", clear, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("clear lands");
        assert!(result.requested_effects.is_empty(), "nothing is left for the host to replay: {:?}", result.requested_effects);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "an empty-target replace clears the domain inline: {:?}", snapshot.selection);
        let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, .. } = &result.ui_scope else { panic!("two partial scopes union into a partial scope, got {:?}", result.ui_scope) };
        assert!(window_bodies.iter().any(|body| body == "some.window"), "the carrier's own window body survives the merge: {window_bodies:?}");
        assert!(window_bodies.iter().any(|body| body == TEST_APP_WINDOW_BODY_KEY), "the verb's declared window body joins it: {window_bodies:?}");
        assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "the verb's declared panel body joins it: {panel_bodies:?}");
        assert!(*utilities, "the carrier's own section flag survives the merge");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The second emission shape for the same intent — `Effect::DispatchAction { action ∈
    /// INTERACTION_ACTION_IDS }`, what `📏️layout`'s `layout_select_effect` and `💡️reasoning`'s
    /// `wires_select_effect` build — folds identically to the `ReplayShellCommand` shape: the
    /// effect never reaches the host, the selection snapshot moved inside the carrying turn, the
    /// carrier's scope widens to the verb's declared interaction refresh scope, the leftover
    /// `InteractionView` rides the carrier's `output`, and the surrounding effects keep their order.
    /// The placeholder `req` an app literal carries is awaited by nothing, so dropping it is silent.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_dispatch_action_interaction_select_is_folded_into_the_carrying_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let verb = Effect::DispatchAction { req: RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: Some(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), delay_ms: 0 };
        let emit = Emit { effects: vec![Effect::Navigate { uri: "semio://home".into() }, verb], ui_scope: UiDirtyScope::None, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerDown", emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the carrying command lands");
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }], "the DispatchAction interaction verb never reaches the host; every other effect does, in order");
        assert!(result.diagnostics.is_empty(), "a well-formed verb applies without diagnostics: {:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert_eq!(snapshot.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the selection snapshot moved inside the carrying turn");
        let declared = <TestApp<false> as ArtifactApp>::interaction_scope(InteractionVerb::Select, &["items"]).expect("the fixture declares its select scope");
        assert_eq!(result.ui_scope, declared, "a `None` carrier scope widens to exactly the verb's declared interaction refresh scope");
        let view = result.output.get("interactionView").expect("the leftover InteractionView rides the carrier's output");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let history = app.render_ctx().test_history().await;
        assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    /// 🧪 Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-dedupe-2026-09-12.md`): the leftover
    /// `selectedIds` publication is a SET of topology ids — `Select` is event-sourced, so it is
    /// idempotent per id whatever the merge, and however many mirror domains (`vortex`) the flatten
    /// walks. Fixture-driven off `🧫️fixtures/🕹️selection-set.json`, the same file the language-neutral
    /// Node twin (`interactionSelectionSetOracle`) re-derives, so neither side can drift alone.
    /// Fails-before: one plain pick published the id twice, once per mirroring domain.
    #[semio_framework_async_macros::async_test]
    async fn leftover_interaction_view_selected_ids_are_a_set_for_every_merge() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🕹️selection-set.json")).expect("language-neutral selection-set fixture");
        let domain = fixture["domain"].as_str().expect("fixture domain");
        let granularity = fixture["granularity"].as_str().expect("fixture granularity");
        let strings = |value: &Value| -> Vec<String> { value.as_array().expect("fixture id list").iter().map(|id| id.as_str().expect("fixture id").to_string()).collect() };
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert!(!cases.is_empty(), "the selection-set fixture must carry at least one case");
        for case in cases {
            let case_id = case["id"].as_str().expect("case id");
            let seed = strings(&case["seed"]);
            let targets = strings(&case["targets"]);
            let expected = strings(&case["selectedIds"]);
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            if !seed.is_empty() {
                reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": domain, "merge": "replace", "method": "pick" }), granularity, &seed)), &mut identity).await;
            }
            let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": domain, "merge": case["merge"].as_str().expect("case merge"), "method": "pick" }), granularity, &targets)), &mut identity).await;
            let view = settled.output.get("interactionView").expect("leftover InteractionView");
            let published: Vec<String> = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds").iter().map(|id| id.as_str().expect("selected id").to_string()).collect();
            assert_eq!(published, expected, "{case_id}: {}", case["why"].as_str().unwrap_or_default());
            let mut distinct = published.clone();
            distinct.sort();
            distinct.dedup();
            assert_eq!(distinct.len(), published.len(), "{case_id}: selectedIds must name every id at most once, got {published:?}");
            let semio_framework_value::DslValue::Object(locked) = view.get("locked").expect("lock state") else { panic!("{case_id}: locked must be an object") };
            assert_eq!(locked.len(), published.len(), "{case_id}: locked has exactly one key per published id, got {locked:?}");
            let gumball = view.get("gumball").expect("gumball leftover");
            assert_eq!(gumball.get("anchorId").and_then(semio_framework_value::DslValue::as_str), published.first().map(String::as_str), "{case_id}: the gumball anchor is the first published id");
            close_reserved_app(&mut app, mounted_policy,cx);
        }
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    /// 🎯️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-merge-vocabulary-2026-09-12.md`): the
    /// DOMAIN path — the reserved `interactionSelect` job a domain-bound world scene dispatches into —
    /// accepts exactly the five words of `🕹️interaction`'s own `MergeMode` schema enum, and NOTHING
    /// else. Fixture-driven off `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`, the file the Node twin
    /// (`selectionMergeVocabularyOracle`) and the reducer law (`semio-framework`'s
    /// `next_selection_obeys_the_merge_vocabulary_fixture_for_every_mode`) read too.
    ///
    /// Fails-before: the renderer host translated the resolved mode into `add`/`remove`/`toggle`, so a
    /// shift-click / ctrl-click on the generation3d preview faulted `interactionSelect: unknown merge
    /// 'add'` and only the unmodified `replace` pick worked.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_speaks_exactly_the_schema_merge_vocabulary() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🕹️interaction/🧪️tests/🧫️fixtures/🎯️merge-modes.json")).expect("language-neutral merge-modes fixture");
        let words = |value: &Value| -> Vec<String> { value.as_array().expect("fixture word list").iter().map(|word| word.as_str().expect("fixture word").to_string()).collect() };
        let vocabulary = words(&fixture["vocabulary"]);
        let deleted = words(&fixture["deletedWords"]);
        assert!(!vocabulary.is_empty() && !deleted.is_empty(), "the vocabulary fixture must carry both the accepted and the deleted words");
        for word in &vocabulary {
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": "items", "merge": word, "method": "pick" }), "item", &["item-1".to_string()])), &mut identity).await;
            assert!(settled.output.get("interactionView").is_some(), "the declared merge '{word}' must settle a leftover InteractionView, not a fault");
            close_reserved_app(&mut app, mounted_policy,cx);
        }
        for word in &deleted {
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            let admitted = app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(interaction_targets_args(json!({ "domainId": "items", "merge": word, "method": "pick" }), "item", &["item-1".to_string()])), &meta(), &mut identity).await.expect("the reserved job is admitted before its args are decoded");
            let fault = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted).await.expect_err("a merge word outside the schema vocabulary must fault the reserved job, never fall back to a replace");
            assert!(format!("{fault:?}").contains(&format!("unknown merge '{word}'")), "the deleted word '{word}' must be rejected outright — no adapter, no silent replace; got {fault:?}");
            close_reserved_app(&mut app, mounted_policy,cx);
        }
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📸️ A task's `restart` command survives into the checkpoint pack (not the task itself —
    /// design-abi.md §4) and `restore_now` queues it as an ordinary `Command` resume, ready for
    /// `plugin_resume_task` on the very next turn — exercised end to end against a fresh
    /// `TestApp` instance, exactly like a live task's own resume.
    ///
    /// 🚫️ The in-flight task (and its parked request) belong to the OLD actor incarnation —
    /// never resumed as though the host round-trip were still live (design-abi.md §4).
    /// `restore_now` below is what re-arms it, as a fresh Command resume, not a revival.
    /// The order is the close ladder's own and the completion oracle's
    /// `await-request-retirement-before-observation`: the registry retires its slots FIRST
    /// (`ReactorCloseState::requests_complete` precedes `tasks_complete`), so a task still live
    /// at that moment can only ever observe its request as retired — never as a successful host
    /// response — and `cancel_instance_tasks` then DROPS whatever is left without polling it.
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume() {
        let completion: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let instance = 507;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, instance, "local");
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let restart_command = <TestCommand as ::protocol::OpBinary>::encode_op(&TestCommand::ApplyCountFromTask { value: 7 }).expect("must encode");
        let observed_completion = std::sync::Arc::new(std::sync::Mutex::new(None));

        let task = test_task("checkpointed", {
            let observed_completion = observed_completion.clone();
            move |ctx: TaskCtx| async move {
                let result = ctx.host.storage_read("never-resolved-either").await;
                *observed_completion.lock().unwrap() = Some(result);
                Ok(TaskResolution::Done)
            }
        })
        .restartable(restart_command.clone());
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).expect("must spawn");
        assert!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await);
        let observed_parked_requests = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await;

        let packed = semio_framework_plugin_reactor_core::checkpoint_now(&runtime).await.expect("checkpoint must succeed while the task is in flight");

        assert!(completion["execution"].as_array().unwrap().iter().any(|row| row == "await-request-retirement-before-observation"), "the completion oracle declares this exact order");
        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::cancel_instance_registry_requests(instance).await;
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::poll_instance_tasks_once(instance).await, 1, "the live task observes its retired request on its next poll and finishes");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::cancel_instance_tasks(instance);
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await as u64, completion["checkpoint"]["tasksAfterCancel"].as_u64().unwrap());
        assert_eq!(observed_parked_requests as u64, completion["checkpoint"]["parkedRequests"].as_u64().unwrap(), "checkpoint must observe an actually polled and parked task");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await as u64, completion["checkpoint"]["requestsAfterCancel"].as_u64().unwrap());
        let fault = observed_completion.lock().unwrap().take().expect("the original retained task completed").expect_err("retired request must not become a successful host response");
        assert_eq!(fault.code.0, completion["checkpoint"]["completionFaultCode"].as_str().unwrap());
        assert_eq!(fault.message, completion["checkpoint"]["completionFaultMessage"].as_str().unwrap());

        semio_framework_plugin_reactor_core::restore_now(&runtime, &packed).await.expect("restore must succeed");
        let (resumed_instance, resumed_meta, resumed_input) = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.expect("restore must have queued exactly one resume for the restartable task");
        assert_eq!(resumed_instance, instance);
        assert_eq!(resumed_meta.instance_id, instance);
        let semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(bytes) = resumed_input.expect("a restart resume is always Command, never Fault") else { panic!("expected Command") };
        assert_eq!(bytes, restart_command, "restore_now must requeue the EXACT restart bytes the task declared via .restartable(..)");

        let command = <TestCommand as ::protocol::OpBinary>::decode_op(&bytes).expect("the actual restored command must decode");
        assert_eq!(command, TestCommand::ApplyCountFromTask { value: completion["checkpoint"]["restartValue"].as_i64().unwrap() as i32 });
        test_restart_publish_and_close(command, &resumed_meta, &completion).await;
    }