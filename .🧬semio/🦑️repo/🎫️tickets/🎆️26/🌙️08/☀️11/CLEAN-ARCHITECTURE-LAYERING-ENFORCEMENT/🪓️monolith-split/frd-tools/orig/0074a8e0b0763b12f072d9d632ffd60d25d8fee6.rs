//! 🔬️ Part 6 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedFirstStepProof;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationContract;
use semio_framework_plugin_decl_app::tool_contract::BOUNDED_FIRST_STEP_FACTORY;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_decl_app::MEDIA_SCHEMA_MISMATCH_CODE;
use semio_framework::manifest::ActionInvocation as ManifestActionInvocation;
use semio_framework_plugin_decl_app::MediaConsumption;
use semio_framework::MediaWireFormat;
use semio_framework_plugin_decl_views::NoTransient;
use semio_framework_plugin_decl_views::NoTransientMutation;
use semio_framework_plugin_job::reserved_jobs::QualifiedBoundedFirstStepProof;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_job::TYPED_OPERATION_RESULT_PAGE_BYTES;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework_plugin_interaction::UiAssemblyResult;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_plugin_program::decode_wire_serialized;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::drive_framework_reserved_spawn_like_host;
use semio_framework_plugin_plugin_program::encode_wire_serialized;
use semio_framework_plugin_extension::artifact_inference::inference_child_dependency;
use semio_framework_plugin_decl_app::media_artifact_fault;
use semio_framework_plugin_job::reserved_jobs::plugin_job_yield_once;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_plugin_job::reserved_jobs;
use semio_framework_os_kernel as store;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
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
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{INGRESS_TURNS_PER_COMMAND, RENDER_CONTEXT_PROBE, SingleDocumentApp, SingleDocumentAppUnderTest, SpawnCountTask, TEST_APP_TOOLS_NONE, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_SCHEMA, TEST_RETAINED_COMMAND_TOOL, TestApp, TestCommand, TestRetainedCommandFactory, __semio_plugin_bundle, app_maintenance_turn, archive_fault_text, close_member_admission_app, close_reserved_app, command_page_authority_budget, contract_app, contract_app_raw, contract_app_under_test, contract_composed_app_raw, contract_registry, declare_test_app_verbs, drain_and_close_composed_fixture, drain_and_close_fixture, drive_single_document_archive, dv, interaction_target_args, meta, reactor_native_lifecycle_ack, reactor_native_lifecycle_budget, reactor_native_lifecycle_finish, reactor_native_lifecycle_open, reactor_native_lifecycle_poll, register_test_child, reserved_action, test_app_surface_id, test_child_dialect};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_job::lifted_full_operation_job::TypedCommandFullOperationJobFactory;
use semio_framework_plugin_job::{test_retained_factory_proof_join, test_unregistered_tool_job_admission_rejected};
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::NOTE_SHELL_COMMAND_ACTION_ID;
use semio_framework_job::InteractiveJob as _;
use semio_framework_ui_contract::{SurfaceId, UiRevision};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use semio_framework_2d::compute::EngineHandles;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
include!("../🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs");


    struct OtherTestRetainedCommandFactory(TestRetainedCommandFactory);

    impl ToolJobFactory for OtherTestRetainedCommandFactory {
        type Payload = semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>;
        fn keys(&self) -> &[ToolFactoryKey] {
            &self.0.keys
        }
        fn payload_schema_id(&self) -> &str {
            TEST_RETAINED_COMMAND_SCHEMA
        }
        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolJobFactory::execution_contract(&self.0)
        }
        fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            ToolJobFactory::create_job(&mut self.0, operation, payload)
        }
    }

    impl ArtifactOwnedToolJobFactory for OtherTestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TestRetainedCommandFactory::PUBLICATION_CONTRACTS;
    }

    /// 🧬️ A hostile structural copy of Draw's public controller/schema/tool constants. Its
    /// concrete Rust type is deliberately not the audited Draw owner type.
    #[derive(Default)]
    struct CopyDrawApp;

    impl ArtifactApp for CopyDrawApp {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.draw.draw", standard: StandardId("1"), subset: SubsetId::ANY };
        const APP_ID: &'static str = "s.draw.draw@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "draw.document";
        type Snapshot = TestSnapshot;
        type Mutation = TestMutation;
        type Config = TestConfig;
        type ConfigMutation = TestConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = NoTransient;
        type TransientMutation = NoTransientMutation;
        type Command = TestCommand;

        async fn initial_snapshot() -> TestSnapshot {
            TestSnapshot::default()
        }

        async fn command_id(_command: &TestCommand) -> &'static str {
            "canvasPointerDown"
        }

        async fn handle(
            command: &TestCommand,
            doc: &ArtifactView<'_, TestSnapshot>,
            cfg: &ConfigView<'_, TestConfig>,
            interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            draft: &DraftView<'_, NoDraft>,
            engines: &EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            TestApp::<false>::handle(command, doc, cfg, interaction, _view_state, draft, engines).await
        }

        async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            TestApp::<false>::render(body_key, doc, cfg, _view_state).await
        }
    }

    /// 🧵️ Source text of the mounted artifact instance: its module tree, the parent file followed by every staged impl slice in declaration order, read at run time from the manifest folder of the hosting crate.
    #[doc(hidden)]
pub fn instance_source() -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧵️instance/🦀️.rs")).expect("the instance module tree reads")
    }

    /// 🧭️ The bijection has exactly two named residues, and both are asserted as exact sets so a
    /// drift in either direction fails on the clause that names its cause.
    ///
    /// 1. Framework-reserved SURFACE verbs the framework registers a factory for unconditionally
    ///    (`register_framework_reserved_tool_factories`) while this fixture's manifest declares
    ///    none of them — it has no interaction topology and no window kit to mint them. A verb
    ///    here is registered but undeclared, so `require_ui_safe_declaration` refuses it by name
    ///    before its factory is ever reached.
    /// 2. Migrated verbs the framework serves without a tool factory, so a missing registration
    ///    is not the dead-action defect it would be for any other migrated verb: `setActiveUtility`
    ///    (injected whenever the app declares utilities; `handle_action_invocation` routes it
    ///    DIRECTLY to `dispatch_emit`) and `cancelTypedOperation` (every app's operation-progress
    ///    cancel; the head of `dispatch_action` routes it DIRECTLY to `dispatch_operation_cancellation`).
    ///
    /// 🚦️ `not-ui-safe`, not `missing-factory`: `contract_registry` declares every verb
    /// `BatchOnlyPendingRewrite`, and the UI-safety backstop is read from the manifest declaration
    /// at the HEAD of `admit_command_wire`, before any proof is resolved. A non-migrated verb has
    /// no factory precisely BECAUSE it is not migrated, so `missing-factory` was the consequence,
    /// not the cause — the clause above still proves there is no factory to find.
    #[semio_framework_async_macros::async_test]
    async fn activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let declared = registry.test_migrated_tool_ids();
        let controller_id = registry.test_controller_id().to_string();
        let mut app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry_on_bus(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), registry.clone(), platform.action_bus.clone(), protocol::ActorId(LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        let registered: std::collections::BTreeSet<String> = app.tools.test_registered_tool_keys()
            .into_iter()
            .map(|key| {
                assert_eq!(key.0, controller_id);
                key.1
            })
            .collect();
        let framework_registered_without_declaration: std::collections::BTreeSet<String> =
            ["clearSelection", "configuration-binary", "import-media", "interactionHover", "interactionSelect", "selectAll", "setInteractionGranularity", "setSelectionMode"].into_iter().map(String::from).collect();
        let framework_directly_routed_migrated: std::collections::BTreeSet<String> = ["cancelTypedOperation", "setActiveUtility", semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, semio_framework::HOST_EVENT_ACTION_ID]
            .into_iter()
            .chain(semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS)
            .map(String::from)
            .collect();
        assert_eq!(registered.difference(&declared).cloned().collect::<std::collections::BTreeSet<_>>(), framework_registered_without_declaration, "an activated factory without a manifest declaration must be one of the framework's own reserved surface verbs");
        assert_eq!(declared.difference(&registered).cloned().collect::<std::collections::BTreeSet<_>>(), framework_directly_routed_migrated, "a migrated declaration with no activated factory is a dead action unless the framework routes it directly");
        assert!(
            instance_source().contains("} else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {"),
            "the only reason `setActiveUtility` may carry no factory is its direct `dispatch_emit` arm in `handle_action_invocation`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifaction==CANCEL_TYPED_OPERATION_ACTION_ID{returnself.dispatch_operation_cancellation(args,meta).await;}"),
            "the only reason `cancelTypedOperation` may carry no factory is its direct `dispatch_operation_cancellation` arm at the head of `dispatch_action`"
        );
        let runtime_source: String = instance_source().split_whitespace().collect();
        assert!(
            runtime_source.contains("ifInstanceExtensions::claims_action(action){returnBox::pin(InstanceExtensions::dispatch_action(self,action,args,meta,identity,cx)).await"),
            "the only reason the history-edit verbs may carry no factory is their boxed instance-extension route at the head of `dispatch_action`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifaction==semio_framework::HOST_EVENT_ACTION_ID{returnBox::pin(self.dispatch_host_event_action(args,meta,identity)).await;}"),
            "the only reason `hostEvent` may carry no factory is its host-driven, boxed arm at the head of `dispatch_action`"
        );
        assert!(
            instance_source().split_whitespace().collect::<String>().contains("ifmatches!(action,semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID|semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID){returnErr(Fault::new(FaultOrigin::Framework,FaultCode::new(\"framework.document-transfer.shell-owned\")"),
            "the only reason the document transfer verbs may carry no factory is their shell-owned refusal at the head of `dispatch_action`"
        );
        let platform_visible = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(platform_visible, registered, "every activated factory key is joined on the platform bus under this controller, and nothing else is");
        let before = platform.action_bus.dispatch_count();
        let verb = <TestApp<false, TEST_APP_TOOLS_NONE> as ArtifactApp>::command_id(&TestCommand::IncrementViaCommand).await;
        assert!(!registered.contains(verb), "the unproved verb has no activated factory of its own");
        assert!(!declared.contains(verb), "and this registry does not declare it migrated either");
        let error = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity).await.expect_err("an unproved typed command must remain fail closed");
        assert_eq!(error.code.0, "interactive-job.not-ui-safe");
        assert_eq!(platform.action_bus.dispatch_count(), before);
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    //#region 🧪️SharedFrameworkActionRouteTests
    /// 🪪️ Every framework-reserved factory is parameterized by its OWNING app type, so the
    /// identity this law joins must name the exact owner the wrapper above was built on — a bare
    /// `TestApp` is `TestApp<false, TEST_APP_TOOLS_FULL>`, a DIFFERENT owner and a different TypeId.
    ///
    /// 📏️ The last column is `ToolExecutionContract::max_raw_wire_bytes`, i.e. the `$raw` column of
    /// this verb's own `framework_reserved_job!(..)` line — NOT its output cap. `copy`/`cut` carry
    /// an 8 KiB raw wire and a 1 MiB OUTPUT; recording the output cap here made the law demand a
    /// 1 MiB wire admission that the bus has never granted them.
    #[semio_framework_async_macros::async_test]
    async fn shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let controller_id = registry.test_controller_id().to_string();
        let app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry_on_bus(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), registry, platform.action_bus.clone(), protocol::ActorId(LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        type UnprovedFrameworkOwner = TestApp<false, TEST_APP_TOOLS_NONE>;
        let expected: [(&str, &str, std::any::TypeId, &'static str, usize); 12] = [
            ("copy", "framework.reserved.copy.v1", std::any::TypeId::of::<FrameworkCopyJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkCopyJobFactory<UnprovedFrameworkOwner>>(), 8_192),
            ("cut", "framework.reserved.cut.v1", std::any::TypeId::of::<FrameworkCutJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkCutJobFactory<UnprovedFrameworkOwner>>(), 8_192),
            ("paste", "framework.reserved.paste.v1", std::any::TypeId::of::<FrameworkPasteJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkPasteJobFactory<UnprovedFrameworkOwner>>(), 1_048_576),
            ("noteShellCommand", "framework.reserved.noteShellCommand.v1", std::any::TypeId::of::<FrameworkNoteShellCommandJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkNoteShellCommandJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            (
                "setHistoryCommandFilter",
                "framework.reserved.setHistoryCommandFilter.v1",
                std::any::TypeId::of::<FrameworkSetHistoryCommandFilterJobFactory<UnprovedFrameworkOwner>>(),
                std::any::type_name::<FrameworkSetHistoryCommandFilterJobFactory<UnprovedFrameworkOwner>>(),
                4_096,
            ),
            ("recordTutorial", "framework.reserved.recordTutorial.v1", std::any::TypeId::of::<FrameworkRecordTutorialJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkRecordTutorialJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            ("clearSelection", "framework.reserved.clearSelection.v1", std::any::TypeId::of::<FrameworkClearSelectionJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkClearSelectionJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            ("interactionHover", "framework.reserved.interactionHover.v1", std::any::TypeId::of::<FrameworkInteractionHoverJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkInteractionHoverJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            ("interactionSelect", "framework.reserved.interactionSelect.v1", std::any::TypeId::of::<FrameworkInteractionSelectJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkInteractionSelectJobFactory<UnprovedFrameworkOwner>>(), 65_536),
            ("selectAll", "framework.reserved.selectAll.v1", std::any::TypeId::of::<FrameworkSelectAllJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkSelectAllJobFactory<UnprovedFrameworkOwner>>(), 4_096),
            (
                "setInteractionGranularity",
                "framework.reserved.setInteractionGranularity.v1",
                std::any::TypeId::of::<FrameworkSetInteractionGranularityJobFactory<UnprovedFrameworkOwner>>(),
                std::any::type_name::<FrameworkSetInteractionGranularityJobFactory<UnprovedFrameworkOwner>>(),
                16_384,
            ),
            ("setSelectionMode", "framework.reserved.setSelectionMode.v1", std::any::TypeId::of::<FrameworkSetSelectionModeJobFactory<UnprovedFrameworkOwner>>(), std::any::type_name::<FrameworkSetSelectionModeJobFactory<UnprovedFrameworkOwner>>(), 16_384),
        ];
        let joined = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        let registrations = app.tools.test_framework_tool_registrations();
        for (tool_id, schema_id, factory_type_id, factory_type_name, maximum) in expected {
            let (key, registered_schema_id, registered_factory_type_id, registered_factory_type_name, contract) = registrations.get(tool_id).expect("shared framework registration");
            assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
            assert_eq!(registered_schema_id, schema_id);
            assert_eq!(*registered_factory_type_id, factory_type_id);
            assert_eq!(*registered_factory_type_name, factory_type_name);
            assert_eq!(contract.max_raw_wire_bytes, maximum);
            assert!(joined.contains(tool_id));
            let exact = vec![0_u8; maximum];
            let admission = platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &exact).expect("exact maximum admission");
            assert_eq!(admission.factory_type_id, factory_type_id);
            assert_eq!(admission.factory_type_name, factory_type_name);
            assert!(platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &vec![0_u8; maximum + 1]).is_err());
        }
        let mut app = app;
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    //#endregion 🧪️SharedFrameworkActionRouteTests

    #[semio_framework_async_macros::async_test]
    async fn retained_factory_proof_requires_the_exact_registered_runtime_authority() {
        test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn copied_app_type_cannot_inherit_same_controller_schema_and_id_proof() {
        let mut owner = contract_registry().await;
        let mut declaration = owner.actions.get("noopMutation").expect("fixture action").clone();
        declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        owner.actions.insert("canvasPointerDown".into(), declaration);
        owner.controller_id = "s.draw.draw@1/*#editor".into();
        assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));

        let audited_draw_row =
            ArtifactBoundedFirstStepProof::new::<TestApp>("owner.rs", "s.draw.draw@1/*#editor", BOUNDED_FIRST_STEP_FACTORY, "canvasPointerDown", "draw.document", ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
        let audited_draw_proof = QualifiedBoundedFirstStepProof { owner: audited_draw_row.owner, row: audited_draw_row };
        assert!(TypedCommandFullOperationJobFactory::<CopyDrawApp>::from_proof(audited_draw_proof).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        struct ReturnedDecoder {
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<TestSnapshot, TestMutation> for ReturnedDecoder {
            fn accept_field_token(
                &mut self,
                _field_id: u16,
                _token: store::os_store::owned_schema_decode::OwnedSchemaToken,
                _terminal: bool,
                _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn maximum_close_byte_demand(&self) -> usize {
                0
            }

            fn maximum_retained_close_bytes(&self) -> usize {
                0
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if grant.maximum_items == 0 {
                    return Ok(store::RetainedCloneStep::Progress(Default::default()));
                }
                self.terminal = true;
                Ok(store::RetainedCloneStep::Complete(Default::default()))
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for ReturnedDecoder {
            fn drop(&mut self) {
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let lease = app.ingest.envelope_field_decoders.try_admit(Box::new(ReturnedDecoder { terminal: false, drops: drops.clone() })).unwrap_or_else(|_| panic!("app decoder return registry admits one exact owner"));
        let ticket = lease.ticket();
        drop(lease);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);

        for _ in 0..4 {
            app.maintenance.maintenance_stage = 12;
            let _ = app_maintenance_turn(&mut app,cx).expect("production maintenance pumps one returned decoder owner");
        }
        assert!(app.ingest.envelope_field_decoders.ticket_reclaimed(ticket));
        assert!(app.ingest.envelope_field_decoder_retirements.is_empty());
        assert!(app.ingest.envelope_field_decoders.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.ladder_ctx().drive_envelope_field_decoder_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close hierarchy observes exact terminal state"), PluginLifecycleStep::Complete(Default::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔢️ Swept over every declared stage instead of a frozen stage index: which ordinal owns the
    /// completed-record lane is an implementation detail that moves whenever a stage is added, while
    /// the law — ordinary maintenance never retires an unconsumed completed output — is not.
    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        struct CompletedRecordSentinel {
            remaining: usize,
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation> for CompletedRecordSentinel {
            fn try_publish_to(&mut self, _target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<TestSnapshot, TestMutation>) -> bool {
                false
            }

            fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, semio_framework_value::ValueError> {
                if grant.maximum_items == 0 {
                    return Ok(store::RetainedCloneStep::Progress(Default::default()));
                }
                if self.remaining != 0 {
                    self.remaining -= 1;
                    return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                self.terminal = true;
                Ok(store::RetainedCloneStep::Complete(Default::default()))
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for CompletedRecordSentinel {
            fn drop(&mut self) {
                assert!(self.terminal);
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let owner: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation>> = Box::new(CompletedRecordSentinel { remaining: 2, terminal: false, drops: drops.clone() });
        let ticket = match app.ingest.envelope_completed_records.try_admit(owner) {
            Ok(ticket) => ticket,
            Err((_fault, mut owner)) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(store::RetainedCloneGrant { maximum_items: 1, ..Default::default() }).expect("rejected completed owner bounded close");
                }
                panic!("app completed-record registry must admit one exact owner")
            }
        };
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        for stage in 0..MAINTENANCE_STAGES {
            app.maintenance.maintenance_stage = stage;
            let _ = app_maintenance_turn(&mut app,cx).expect("ordinary maintenance preserves unconsumed completed output");
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        app.ingest.envelope_completed_records.try_request_close(ticket).expect("cancelled consumer hands exact completed output to maintenance");
        for _ in 0..6 {
            for stage in 0..MAINTENANCE_STAGES {
                app.maintenance.maintenance_stage = stage;
                let _ = app_maintenance_turn(&mut app,cx).expect("production maintenance pumps one completed-record owner");
            }
        }
        match app.ingest.envelope_completed_records.try_detach(ticket) {
            Err(fault) => assert_eq!(fault, store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Stale),
            Ok(mut owner) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(store::RetainedCloneGrant { maximum_items: 1, ..Default::default() }).expect("unexpected completed owner bounded close");
                }
                panic!("maintenance did not reclaim the exact completed record")
            }
        }
        assert!(app.ingest.envelope_completed_record_retirements.is_empty());
        assert!(app.ingest.envelope_completed_records.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.ladder_ctx().drive_envelope_completed_record_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close observes exact completed terminal state"), PluginLifecycleStep::Complete(Default::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    
    /// 🧱️ Page counts the page-set law drives. The host splits at `COMMAND_PAGE_MAXIMUM_BYTES`, so an
    /// 8-page command is the shape a `registerBrushMesh`-scale payload reaches — and 67 is the
    /// 272 089-char scoped contributions pack the procedural plugin pushed on 2026-09-14, which a
    /// fixed 64-page ceiling refused outright (`command ingress exceeds 64 pages`), and 80 is past it
    /// by a margin no constant explains.
    const INGRESS_PAGE_SETS: [usize; 6] = [1, 2, 4, 8, 67, 80];
    
    /// ⏱️ Turns a page set may spend beyond one per page. The reactor wire admits exactly ONE page per
    /// turn, so `pages` is the protocol's own floor; the slack covers a single instance-authority
    /// contention retry (`plugin_exchange` hands the owner back when `try_lock` would block).
    const INGRESS_TURN_SLACK: usize = 1;
    
    /// 📤️ The production host owner for one multi-page command — the `CommandBatchDriver` a shard keeps
    /// per command, which hands out its next page only once the guest's status has been observed.
    async fn command_page_authority_driver(instance: u32, seq: u64, command: &protocol::AppCommand) -> semio_framework::kernel::CommandBatchDriver {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("fixed command batch authority");
        envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit fixture command: {fault:?}"));
        let batch = semio_framework::kernel::CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit fixture batch: {fault:?}"));
        semio_framework::kernel::CommandBatchDriver::new(seq, batch)
    }
    
    /// 🚚️ Drives a whole page set to its terminal ingress status exactly as a shard does: hand the owner's
    /// next page (the wire carries at most one per turn), observe the status it produced, repeat. Returns
    /// whether it completed, the turns it spent and the pages the owner declared.
    async fn drive_one_page_set(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, driver: &mut semio_framework::kernel::CommandBatchDriver,cx:&mut semio_framework_job::StepContext<'_>) -> (bool, usize, usize) {
        let mut turns = 0;
        let mut declared = 0;
        let mut carried = driver.next_page().expect("the host owner produces its first page");
        loop {
            declared = declared.max(carried.as_ref().map_or(0, |(cursor, _)| cursor.page_count as usize));
            let result = semio_framework_plugin_reactor_turn::poll_kernel(runtime, &mut None, carried.take(), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            turns += 1;
            match result.command_ingress {
                semio_framework::kernel::CommandIngressStatus::CommandComplete(_) => return (true, turns, declared),
                semio_framework::kernel::CommandIngressStatus::Fault { .. } => return (false, turns, declared),
                ref pending if turns >= INGRESS_TURNS_PER_COMMAND => panic!("a page set did not reach a terminal ingress status in {turns} turns: {pending:?}"),
                ref accepted @ (semio_framework::kernel::CommandIngressStatus::PageAccepted(_) | semio_framework::kernel::CommandIngressStatus::Backpressure(_)) => {
                    driver.observe(accepted, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).expect("the host owner accepts the status its own page produced");
                    carried = driver.next_page().expect("the host owner produces its pages");
                }
                _ => {}
            }
        }
    }
    
    /// ⚖️ LAW: a command's page set reaches its terminal ingress status in ONE TURN PER PAGE — the cost is
    /// the wire's own page count, never the number of internal moves the ingress owner makes.
    ///
    /// 🐢️ Before ticket 26/09/02 wave B24 `plugin_exchange` answered `Pending` after EVERY move of
    /// `PluginCommandIngress::step` — `Encoded → Decoding`, the header, one read per decoded field, then
    /// `Decoded → Ready` — and each `Pending` costs the host a whole turn round trip. A one-page
    /// `CommandText` measured 4.006 turns over 320 commands and an `AppCommand::Command` five, so the real
    /// cost was `pages + k` with `k` set by the command's decode shape rather than by the wire. This law
    /// fails for every page count under that pacing, which is what makes it discriminating.
    #[semio_framework_async_macros::async_test]
    async fn a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_022;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-set".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        let mut census = Vec::new();
        for (index, declared) in INGRESS_PAGE_SETS.into_iter().enumerate() {
            let seq = 512 + index as u64;
            let line = "p".repeat((declared - 1) * semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
            let mut driver = command_page_authority_driver(instance, seq, &protocol::AppCommand::CommandText { seq, line }).await;
            let (_, turns, pages) = drive_one_page_set(&runtime, &mut driver,cx).await;
            assert_eq!(pages, declared, "the fixture payload must produce exactly {declared} host pages");
            census.push(format!("{declared}→{turns}"));
            assert!(
                turns <= declared + INGRESS_TURN_SLACK,
                "a {declared}-page command spent {turns} turns reaching its terminal ingress status (ceiling {}) — the ingress is stepping a per-move ladder the host has to drive from outside",
                declared + INGRESS_TURN_SLACK
            );
            assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "a {declared}-page command must leave no retained ingress owner");
        }
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }
    
    
    
    /// 📥️ The cross-language paged command-ingress contract — the rows the kernel's own reassembly law
    /// and the TypeScript host writer's twin read.
    #[derive(serde::Deserialize)]
    struct CommandIngressDisorderFixture {
        disorder: Vec<CommandIngressDisorderRow>,
    }
    
    #[derive(serde::Deserialize)]
    struct CommandIngressDisorderRow {
        pub name: String,
        pub pages: usize,
        pub deliver: Vec<usize>,
        #[serde(rename = "rejectedDelivery")]
        rejected_delivery: usize,
    }
    
    /// 📤️ Every page of one command with the cursor the host stamps on it — the bytes come from the real
    /// `encode_app_command`, cut at the page extent exactly as `CommandBatchDriver` cuts them, so a law
    /// may deliver them in an order no production owner would.
    async fn command_ingress_pages_of(instance: u32, seq: u64, command: &protocol::AppCommand) -> Vec<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)> {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let byte_len = encoded.byte_len();
        let mut reader = semio_framework::kernel::PagedCommandReader::new(encoded);
        let mut body = Vec::with_capacity(byte_len);
        for _ in 0..byte_len {
            body.push(reader.read_byte().expect("the encoded command reads back"));
        }
        let page_count = body.len().div_ceil(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        body.chunks(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)
            .enumerate()
            .map(|(index, chunk)| {
                let cursor = semio_framework::kernel::CommandPageCursor {
                    owner: u64::from(instance),
                    generation: 1,
                    command_index: 0,
                    command_count: 1,
                    instance,
                    seq,
                    kind: body[0],
                    page_index: index as u32,
                    page_count: page_count as u32,
                    item_count: 0,
                    metadata: 0,
                };
                (cursor, semio_framework::kernel::FixedCommandPage::try_copy_from(chunk).expect("a cut page"))
            })
            .collect()
    }
    
    /// ⚖️ LAW: the guest reassembles a multi-page command in ARRIVAL ORDER and refuses every page that is
    /// not the next one — a duplicate, a skip and a swap all answer `plugin.command-page-order`.
    ///
    /// 🧨️ A stream with no page ceiling is only safe if the reassembly is exact: the ingress admits one
    /// page per turn over an unbounded number of turns, so a retried or reordered turn must not be able
    /// to splice a command together out of the wrong bytes.
    #[semio_framework_async_macros::async_test]
    async fn a_command_page_that_is_not_the_next_one_is_refused_by_its_order() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let fixture: CommandIngressDisorderFixture =
            serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/📄️pages/🔣️.json")).expect("📥️command-ingress-pages fixture parses");
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_031;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-order".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        for (index, row) in fixture.disorder.iter().enumerate() {
            let seq = 1_024 + index as u64;
            let line = "p".repeat((row.pages - 1) * semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
            let pages = command_ingress_pages_of(instance, seq, &protocol::AppCommand::CommandText { seq, line }).await;
            assert_eq!(pages.len(), row.pages, "row {} must cut into {} pages", row.name, row.pages);
            for (delivery, page_index) in row.deliver.iter().copied().enumerate() {
                let (cursor, page) = pages[page_index].clone();
                let result = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, Some((cursor, page)), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
                match (&result.command_ingress, delivery == row.rejected_delivery) {
                    (semio_framework::kernel::CommandIngressStatus::Fault { fault, .. }, true) => {
                        assert_eq!(fault.as_slice(), b"plugin.command-page-order", "row {} delivery {delivery} must be refused by its order", row.name);
                    }
                    (semio_framework::kernel::CommandIngressStatus::PageAccepted(_), false) => {}
                    (observed, _) => panic!("row {} delivery {delivery} (page {page_index}) answered {observed:?}", row.name),
                }
                if delivery == row.rejected_delivery {
                    break;
                }
            }
            assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "row {} must leave no retained ingress owner behind", row.name);
        }
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }

    /// ⚖️ LAW: an ingress owner the host has stopped driving never answers for the command it IS
    /// driving, and never pins the ingress lane against it.
    ///
    /// 🧭️ Nothing on this wire tells the guest that a command was abandoned — the host's
    /// `CommandBatchDriver` simply stops existing when its slice loop gives up, its wall budget runs
    /// out or its own cleanup tears the command down. The one piece of evidence the guest can read is
    /// a page naming a DIFFERENT command on the same instance, because the host drives one command
    /// per instance at a time. So that page must supersede whatever is retained, and from then on
    /// every status the new command's own driver observes must be one it can acknowledge — including
    /// on the turns the host carries no page at all, which is exactly when a stale owner used to
    /// speak.
    ///
    /// 🐛️ Before the retiring lane the superseded owner stayed in the single used ingress slot: every
    /// page of the new command answered `Backpressure` forever, and every page-less turn answered
    /// `CommandPending` with the STALE cursor, which `validate_cursor` correctly refused as
    /// `plugin.command-cursor-mismatch: … owner is 1, the retained owner's is 554` — the fault that
    /// held every semio-MCP mutation verb (ticket 26/09/18,
    /// `📓️a2-mcp-plugin-host-instance-open.md` §6.1, `📓️r2-reactor-retained-command-owner.md`).
    #[semio_framework_async_macros::async_test]
    async fn an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_024;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "abandoned-ingress-owner".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;

        let abandoned_seq = 4_096;
        let line = "p".repeat(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
        let mut abandoned = command_page_authority_driver(instance, abandoned_seq, &protocol::AppCommand::CommandText { seq: abandoned_seq, line }).await;
        let first = abandoned.next_page().expect("the abandoned owner produces its first page").expect("a two-page command has a first page");
        let opened = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, Some(first), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
        assert!(
            matches!(opened.command_ingress, semio_framework::kernel::CommandIngressStatus::PageAccepted(_)),
            "the guest accepts the first page of a two-page command: {:?}",
            opened.command_ingress
        );
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 1, "a half-assembled command is retained");
        drop(abandoned);

        let seq = 4_097;
        let mut driver = command_page_authority_driver(instance, seq, &protocol::AppCommand::CommandText { seq, line: "after the abandoned owner".into() }).await;
        let mut completed = false;
        for turn in 0..INGRESS_TURNS_PER_COMMAND {
            let carried = driver.next_page().expect("the host owner produces its pages");
            let result = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, carried, None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            let progress = driver.observe(&result.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).unwrap_or_else(|fault| {
                panic!("turn {turn}: the guest answered {:?} for the command this host owns as seq {seq}: {}: {}", result.command_ingress, fault.code.0, fault.message)
            });
            match progress {
                semio_framework::kernel::CommandBatchProgress::Complete => {
                    completed = true;
                    break;
                }
                semio_framework::kernel::CommandBatchProgress::Faulted => panic!("turn {turn}: the command the host drives faulted: {:?}", result.command_ingress),
                _ => {}
            }
        }
        assert!(completed, "the command the host drives never completed in {INGRESS_TURNS_PER_COMMAND} turns — the abandoned owner still holds the ingress lane");
        for _ in 0..INGRESS_TURNS_PER_COMMAND {
            if semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy() == 0 {
                break;
            }
            semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, None, None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
        }
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "the superseded owner retires on its own budget instead of pinning the authority");
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let before_snapshot = app.stores.test_snapshot().await;
        let before_edits = app.stores.test_store().await.applied_edit_ids().len();

        let set = semio_framework_plugin_runtime_instances::inst_wire_frames::set_merge_policy_frames(&mut app, 7, protocol::MergePolicy::Vigilant.as_u8()).await.expect("known policy accepts");
        assert_eq!(set, vec![protocol::AppFrame::Done { in_reply_to: 7 }]);
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant);
        assert_eq!(app.stores.test_snapshot().await, before_snapshot, "local policy changes no document state");
        assert_eq!(app.stores.test_store().await.applied_edit_ids().len(), before_edits, "local policy changes no shared history");

        let invalid_policy = semio_framework_plugin_runtime_instances::inst_wire_frames::set_merge_policy_frames(&mut app, 8, 3).await.expect_err("unknown policy ordinal rejects");
        assert_eq!(invalid_policy.code.0, "merge.invalid-policy");
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant, "invalid policy never changes the active policy");

        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity).await.expect("seed one durable edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the seeded durable edit reaches its publication");
        let (mut envelope, applied_edit_ids) = {
            let store = app.stores.test_store().await;
            let files = store::os_store::text_format::print_document_pack(store.envelope()).await.expect("print retained test envelope");
            (store::os_store::text_format::parse_document_pack::<TestSnapshot, TestMutation>(&files.pack, &files.spr).await.expect("parse retained test envelope").envelope, store.applied_edit_ids().to_vec())
        };
        let kind = protocol::ConflictKind::Degraded { edit_ids: applied_edit_ids.clone() };
        let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
        let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
        let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
        envelope.conflicts.push(protocol::Conflict {
            id: conflict_id.clone(),
            kind,
            status: protocol::ConflictStatus::Open,
            messages: vec![protocol::MutationMessage::warning("mutation.partial", "test conflict").at_op(0)],
            actors: vec![ActorId("local".into())],
            timestamp,
        });
        app.stores.test_store_mut().await.reset(envelope).await.expect("seed valid open degraded conflict");

        let read = semio_framework_plugin_runtime_instances::inst_wire_frames::read_conflicts_frames(&app, 9).await;
        let [protocol::AppFrame::Conflicts { in_reply_to: Some(9), conflicts }] = read.as_slice() else { panic!("read must yield exactly one correlated conflict projection") };
        let open: Vec<protocol::Conflict> = semio_framework_plugin_plugin_program::decode_wire_serialized(conflicts).await.expect("conflict payload decodes canonically");
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, conflict_id);
        assert_eq!(open[0].status, protocol::ConflictStatus::Open);

        let invalid_resolution = semio_framework_plugin_runtime_instances::inst_wire_frames::resolve_conflict_frames(&mut app, 10, conflict_id.0.clone(), 2).await.expect_err("unknown resolution ordinal rejects");
        assert_eq!(invalid_resolution.code.0, "merge.invalid-resolution");
        assert_eq!(app.open_conflicts().await.len(), 1, "invalid resolution never changes conflict state");

        let resolved = semio_framework_plugin_runtime_instances::inst_wire_frames::resolve_conflict_frames(&mut app, 11, conflict_id.0.clone(), 0).await.expect("accept resolves the open degraded conflict");
        let [protocol::AppFrame::MergeReport { in_reply_to: Some(11), report }, protocol::AppFrame::Conflicts { in_reply_to: Some(11), conflicts }] = resolved.as_slice() else {
            panic!("resolve must emit correlated merge report then open-conflict projection")
        };
        let report: protocol::MergeReport = semio_framework_plugin_plugin_program::decode_wire_serialized(report).await.expect("merge report payload decodes canonically");
        assert!(report.accepted);
        assert_eq!(report.conflict.as_ref(), Some(&conflict_id));
        let open: Vec<protocol::Conflict> = semio_framework_plugin_plugin_program::decode_wire_serialized(conflicts).await.expect("post-resolution conflict payload decodes canonically");
        assert!(open.is_empty(), "accepted conflict leaves the open projection");
        assert!(app.open_conflicts().await.is_empty());
        drain_and_close_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "x".into() }, &meta(), &mut identity,cx).await.expect("set label");
        assert_eq!(app.stores.presence_store.generation().await, 0);
        assert_eq!(app.stores.transient_store.generation().await, 0);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪆️ LAW (design §20.15, W-b): `child_head_packs` — what `AppCommand::ReadChildHeads` answers and the MCP gateway injects as
    /// the `child:<slot>/<childId>` dependencies of an inference request — is every owned child's CURRENT head snapshot pack with
    /// its slot, id and dialect: a reader that composes on read decodes the child's live value from it, no history fold.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_documents_child_heads_answer_each_owned_childs_current_head_snapshot() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the composite gesture settles");

        let heads = PluginApp::child_head_packs(&app).await.expect("child heads");
        assert_eq!(heads.iter().map(|entry| (entry.slot.as_str(), entry.child_id.as_str(), entry.dialect.clone())).collect::<Vec<_>>(), vec![("slot", "child-1", test_child_dialect().await.to_coordinate())]);
        let head: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&heads[0].head_pack).expect("the head pack is the child's snapshot pack");
        assert_eq!(head.count, 7, "the head pack carries the child's current value");
        assert_eq!(semio_framework_plugin_extension::artifact_inference::inference_child_dependency(&heads[0].slot, &heads[0].child_id), "child:slot/child-1");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎞️ LAW (W2A-6 P5, `📓️api-stepped-document-load.md` §4/§9): consuming a whole document of the app's own schema never folds
    /// inside the call — `consume_media` answers the carrier's archive (the source document verbatim) and leaves the document untouched;
    /// only the stepped archive load (admit → polls → acknowledge) makes it the consumer's document. The carrier is `pk:` base64 text, so
    /// a text-only host edge carries it losslessly. A foreign document schema is refused under the localized framework code.
    #[semio_framework_async_macros::async_test]
    async fn a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut source = contract_composed_app_raw(mounted_policy, &mut identity).await;
        source
            .test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 41 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("the source edits its document");
        let media_fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️media/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let media_grant=serde_json::from_value(media_fixture["grant"].clone()).unwrap();let mut media_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let original=identity.pause().expect("same original media admission receipt");assert_eq!(original.maximum_bytes(),media_fixture["originalNativeMaximumBytes"].as_u64().unwrap()as usize);let original=match original.with_retirement_recipient(&mut media_recipient){Ok(original)=>original,Err((error,_original))=>panic!("{error}")};let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume_retirement(original,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let artifact = identity.encode(|native| { let mut owner=semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner::new(native,media_grant); ::semio_framework_async::poll::resolve_ready(PluginApp::produce_media(&mut source, "artifact:out", &mut owner)) }).expect("the source hands out its whole document through its original authority");
        let mut consumer = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let before = consumer.test_snapshot().await;
        let source_id = PluginApp::document_identity(&source).expect("the source's identity");
        assert!(std::str::from_utf8(&artifact.data).is_ok_and(|text| text.starts_with("pk:")), "the whole-document carrier is pk: base64 text");

        let semio_framework_plugin_decl_app::MediaConsumption::DocumentLoad(archive) = PluginApp::consume_media(&mut consumer, "artifact:in", artifact.clone()).await.expect("a whole document of the own schema is consumed") else {
            panic!("a whole document is answered as a document archive load");
        };
        assert_eq!(consumer.test_snapshot().await, before, "consuming folded nothing: the document is unchanged until the load runs");
        assert!(archive.members.is_empty() && !archive.parent_pack.is_empty() && !archive.parent_spr.is_empty());
        PluginApp::begin_document_archive_load(&mut consumer, 7, archive).expect("the runtime admits the archive");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(&mut consumer, 7, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(&mut consumer, 7).expect("acknowledge");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready);
        assert_eq!(consumer.test_snapshot().await, source.test_snapshot().await, "the stepped load lands the source's head");
        assert_eq!(PluginApp::document_identity(&consumer).as_deref(), Some(source_id.as_str()), "a whole-document transfer loads the source document verbatim");

        let mut foreign = artifact;
        foreign.descriptor.wire = semio_framework::MediaWireFormat::Document { schema: "foreign.document".into() };
        let refusal = PluginApp::consume_media(&mut consumer, "artifact:in", foreign).await.expect_err("a foreign document schema is refused");
        let fault = semio_framework_plugin_decl_app::media_artifact_fault(refusal);
        assert_eq!(fault.code.0, semio_framework_plugin_decl_app::MEDIA_SCHEMA_MISMATCH_CODE);
        let notice = semio_framework::kernel::fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).expect("the refusal earns its localized notice");
        assert!(notice.text.contains("foreign.document") && notice.text.contains(<TestApp as ArtifactApp>::DOCUMENT_SCHEMA), "{}", notice.text);
        drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut source, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 👁️ A view command never advances the document.
    #[semio_framework_async_macros::async_test]
    async fn view_action_emits_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_action_emits_host_effect_without_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Navigate, &meta(), &mut identity,cx).await.expect("navigate");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }]);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 #38 host-drive contract: spawn-admit undo reaches Done in 2 Isolated steps with the browser budget.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_reaches_done_within_host_drive_contract() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
         &mut identity)
        .await;
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
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        let (settled, steps) = semio_framework_plugin_instance_app::plugin_app_impl::drive_framework_reserved_spawn_like_host(&mut app, admitted).await.expect("host-like Isolated drive");
        assert!(steps > 1, "the reserved dummy job is a multi-turn bounded job; every turn is one funded step ({steps})");
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert!(settled.history_patch.is_some_and(|patch| patch.can_undo && patch.can_redo && !patch.upserts.is_empty()));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪️ The addressed borrowed receiver retains one original invocation and caller through pending admission.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_addressed_receiver_admits_isolated_spawn_job() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::ViewWindowInstance;
        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        app.bind_instance_id(1).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.expect("host-wire bundle"));
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_push_instance(&runtime, semio_framework_plugin_plugin_program::AppInstance { id: 1, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, 1, "local");
        let action_json = format!(
            r#"{{"address":{{"pluginId":"test","appId":"{}","modeId":"edit","windowKindId":"puzzle3d-main","windowInstanceId":"puzzle3d-main-perspective","actionId":"undo"}},"arguments":{{"windowId":"puzzle3d-main-perspective"}}}}"#,
            TestApp::<false>::APP_ID,
        );
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_id: Some("puzzle3d-main-perspective".into()),
            window_instances: vec![ViewWindowInstance { id: "puzzle3d-main-perspective".into(), window_kind_id: "puzzle3d-main".into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let mut invocation:ManifestActionInvocation=semio_framework_pack_json::from_json_str(&action_json,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed original addressed invocation");
        let meta=ActionMeta{actor:"local".into(),instance_id:1,view_state:Some(view)};
        let cell=runtime.instances.borrow().get(1).cloned().expect("original mounted receiver");
        let mut original_args=Some(DslValue::Object(std::mem::take(&mut invocation.arguments).into_iter().collect()));let mut original_fault=None;
        let mut admitted=None;
        for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){
            let mut instance=cell.host_instance().expect("original exclusive receiver");
            admitted=instance.app.handle_action_invocation(&invocation.address,&mut original_args,&mut original_fault,Some("edit"),&meta,&mut identity,cx).await.expect("actual original addressed receiver");
            if admitted.is_some(){break}
            instance.app.maintenance_step(cx,cx.retained_grant()).expect("original receiving maintenance");
        }
        let admitted=admitted.expect("finite original addressed admission reached acceptance");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "export first turn must admit Isolated framework.reserved.tool, got {:?}",
            admitted.requested_effects,
        );
        assert!(admitted.output.get("operationId").and_then(semio_framework_value::DslValue::as_str).is_some(), "first-turn output names the spawn-job");
        let cell = runtime.instances.borrow().get(1).cloned().expect("live export instance");
        let mut instance = cell.instance.lock().expect("export instance");
        artifact_app_laws::close_registered_fixture_app(&mut instance.app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_config_only_view_action_is_not_a_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert!(app.render_ctx().test_history().await.commands.is_empty(), "a config-only View is never a row (design §20.13, L4)");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "while the config lane holds its edit");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_does_not_widen_a_none_ui_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::ViewNoScope, &meta(), &mut identity,cx).await.expect("viewNoScope");
        assert_eq!(result.ui_scope, UiDirtyScope::None);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_preserves_partial_ui_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::ViewPartialScope, &meta(), &mut identity,cx).await.expect("viewPartialScope");
        let UiDirtyScope::Partial { window_bodies, panel_bodies, .. } = result.ui_scope else { panic!("expected a Partial scope") };
        assert_eq!(window_bodies, vec!["some.window".to_string()]);
        assert!(panel_bodies.is_empty());
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn scope_upgrade_full_stays_full() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert_eq!(result.ui_scope, UiDirtyScope::Full);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn view_action_emitting_ops_is_rejected() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let error = app.dispatch_typed(TestCommand::BadView, &meta(), &mut identity,cx).await.expect_err("a View command emitting operations must be rejected");
        assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }


    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `⚛️reactor::poll`'s real
    // event routing (`Event::Completed`, `run_until_idle`, `drain_task_resumes`) is
    // wasm32-wasip2-only (`wit_bridge`) and cannot run under a native `cargo test`. These tests
    // exercise the SAME underlying primitives `poll` itself drives — `⚛️reactor::spawn_task`,
    // `⚛️reactor::reactor_driver::{run_until_idle, resolve_request, pop_task_resume, ...}`, and
    // `semio_framework_plugin::plugin_runtime::plugin_resume_task` — directly, standing in for the wasm-only
    // glue: `reactor_driver::resolve_request` IS what `Event::Completed` routes to
    // (`RequestRegistry::resolve`), and `pop_task_resume`/`plugin_resume_task` together ARE
    // `drain_task_resumes` minus the frame-routing tail (`route_app_frame`, itself untouched by
    // this packet). An honest, named gap: the wasm-only tail is not exercised natively.

    /// 🌱️ A command spawns a task; the task awaits a REAL `host::storage_read` round trip
    /// (genuinely parked on `RequestRegistry`, not a synchronously-ready future); an injected
    /// completion (`reactor_driver::resolve_request`, standing in for `Event::Completed`)
    /// resolves it; the follow-up `TaskResolution::Command` mutates the store, stamped with
    /// the task's CLONED originating `ActionMeta`.
    ///
    /// 🪪️ This law dispatches under its OWN instance, not the fixture's default: a typed command
    /// is refused with `interactive-job.live-instance` unless the wrapper's mounted live instance
    /// is exactly `meta.instance_id`, and `spawn_task` is keyed off that same id.
    ///
    /// 🪜️ A migrated verb's `dispatch_typed` only ADMITS: the emit is owned by the mounted
    /// operation, and every lane of it — the task lane included — is spent by the bounded
    /// publication ladder, one lane per unit. Asserting the spawn before settling would assert
    /// that a migrated command never spawns.
    ///
    /// 🪪️ `spawn_task` is keyed off `meta.instance_id` (the publication ladder's own captured
    /// `mounted.meta`, i.e. `spawn_meta` above, which shares `instance`'s value by construction).
    ///
    /// ▶️ First poll: the task runs up to its `.await` on `host.storage_read(..)` and parks —
    /// genuinely pending, not synchronously resolved.
    ///
    /// ✅️ Inject the "completion" (`⚛️reactor::poll`'s `Event::Completed` arm calls the exact
    /// same `RequestRegistry::resolve`) — request id 1 is `storage_read`'s, the first (and
    /// only) request this test has allocated.
    ///
    /// 🔀️ Re-enter through the SAME typed-command path a live command would — this IS
    /// `plugin_runtime::plugin_resume_task`'s `TaskResumeInput::Command` arm, exercised
    /// directly rather than through `crate::reactor::drain_task_resumes` (wasm-only).
    ///
    /// 🪜️ The follow-up re-enters the APPLYING lane (`PluginApp::resume_task_command` →
    /// `dispatch_typed`), so like every migrated dispatch it ADMITS here and the document
    /// advances when its publication ladder walks. Settle it through the same runtime cell the
    /// host's continuation drives, then read what it actually did.
    ///
    /// 🧾️ And it landed as the app's own verb in the command log, applied — a PREVIEW
    /// (`handle_command_frame`, the agent prepare lane this resume used to take) records
    /// nothing at all and applies nothing.
    #[semio_framework_async_macros::async_test]
    async fn a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let instance = 501;
        let spawn_meta = ActionMeta { actor: "alice".into(), instance_id: instance, view_state: None };
        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        semio_framework_plugin_reactor_app::PluginApp::bind_instance_id(&mut app, instance).await;

        let result = app.dispatch_typed(TestCommand::SpawnCountTask, &spawn_meta, &mut identity).await.expect("dispatching SpawnCountTask must succeed");
        assert!(result.mutations.is_empty(), "SpawnCountTask itself must emit no document mutation — only the LATER resume does");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app, instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the spawning operation's publication settles");
        assert_eq!(
            receipt.lanes,
            vec![TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal],
            "the task lane spends its own publication unit and mints NO host page of its own: the operation's pages are exactly the ones its emit's remaining lanes owe"
        );
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 1, "the publication ladder's task lane must have spawned exactly one task");

        let pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(pending, "the task must be parked on a real RequestRegistry await, not finished synchronously");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await, 1, "storage_read must have allocated exactly one RequestRegistry slot");
        assert!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.is_none(), "nothing can have resolved yet — the task is still parked");

        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::resolve_request(1, Ok(42i32.to_le_bytes().to_vec())).await;
        let pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(!pending, "the task must run to completion once its await resolves");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(spawn_meta.instance_id).await, 0, "TASK_RECORDS must be cleaned up the moment the task's future completes");

        let (resumed_instance, resumed_meta, resumed_input) = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.expect("the completed task must have queued exactly one resume");
        assert_eq!(resumed_instance, spawn_meta.instance_id);
        assert_eq!(resumed_meta.actor.as_str(), "alice", "the follow-up must carry the task's ORIGINATING actor, not whatever is 'current' at resume time");
        assert_eq!(resumed_meta.instance_id, spawn_meta.instance_id);
        let semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(command_bytes) = resumed_input.expect("the task resolved Ok, not with a Fault") else {
            panic!("SpawnCountTask's task resolves TaskResolution::Command, not ::Emit");
        };
        let decoded_command = <TestCommand as ::protocol::OpBinary>::decode_op(&command_bytes).expect("must decode back to a TestCommand");
        assert_eq!(decoded_command, TestCommand::ApplyCountFromTask { value: 42 }, "the resolved command must carry the value the injected completion delivered");

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, resumed_instance, "local");
        let output = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_resume_task(&runtime, resumed_instance, &resumed_meta, semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(command_bytes), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        assert_eq!(output.frames.len(), 1, "a successful resume must frame exactly one AppFrame::Emit");
        let frame = protocol::decode_app_frame(&output.frames[0]).await.expect("must decode back to an AppFrame");
        assert!(
            matches!(frame, protocol::AppFrame::Emit { .. }),
            "a resumed Command follow-up must frame as AppFrame::Emit — never a fault: {frame:?}"
        );

        let cell = semio_framework_plugin_runtime_ops::ops_wire_limits::runtime_instance_cell(&runtime, resumed_instance).expect("the resumed instance stays mounted");
        let mut active = cell.instance.try_lock().expect("no other owner holds the resumed instance");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut active.app, resumed_instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the resumed follow-up publishes");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Artifact), "the follow-up must have published on the ARTIFACT lane — it applies, it does not preview: {:?}", receipt.lanes);
        assert_eq!(receipt.completions, 1, "the follow-up is exactly one terminated operation");

        let history = semio_framework_plugin_reactor_app::PluginApp::history_snapshot(&mut active.app).await.expect("history after the resumed follow-up");
        let row = history.upserts.iter().find(|entry| entry.action_id == "applyCountFromTask").unwrap_or_else(|| panic!("the follow-up dispatch is recorded under its own verb: {:?}", history.upserts.iter().map(|entry| entry.action_id.clone()).collect::<Vec<_>>()));
        assert!(row.applied, "the follow-up is an APPLIED edit, not a staged preview");
        assert!(row.op_lines.iter().any(|line| line.contains("42")), "the applied edit carries the value the injected completion delivered: {:?}", row.op_lines);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }