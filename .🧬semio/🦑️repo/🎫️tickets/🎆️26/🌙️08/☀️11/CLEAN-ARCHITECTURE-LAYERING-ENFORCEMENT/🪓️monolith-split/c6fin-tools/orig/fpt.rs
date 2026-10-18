//! ⚖️ App laws and the test modules of the app vocabulary.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
/// 🪪️ Runs one test body with a local native authoring identity: `with_authoring_identity!(|identity| body)` binds `identity` as a mutable
/// [`EntityIdentityAuthority`](semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority) for `body`, an expression that may await.
#[cfg(any(test, feature = "artifact-app-testing"))]
#[macro_export]
macro_rules! with_authoring_identity {
    (|$identity:ident| $body:expr) => {{
        let mut __observer = |_: $crate::__value::native_encoding::NativeEncodeProgress| true;
        let __observer: &mut $crate::__kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut __observer;
        let mut $identity = $crate::__kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, __observer).expect("test authoring identity admission");
        $body
    }};
}
#[doc(hidden)]
pub use semio_framework_os_kernel as __kernel;
#[doc(hidden)]
pub use semio_framework_value as __value;

pub use semio_framework_plugin_testing_caller::OriginalFixtureCaller;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_framework_plugin_window_transient::window_transient::{WindowTransientOwner, WindowTransientOwnerRegistry};
use semio_framework_value::DslValue;
use protocol::{OpBinary, OpText};
use semio_framework::kernel::UiDirtyScope;
use semio_framework::manifest::ActionInvocation as ManifestActionInvocation;
use semio_framework::kernel::{ActorId, Effect, InvocationResult};
use semio_framework::{ActionKind, AppIo, ConfigSpec, Fault, FaultCode, FaultOrigin, MediaPortSpec, ViewModel, ViewWindowInstance, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_runtime::ComponentTree;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
use pack::snapshot::ArtifactPack;
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::ArtifactCommand;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, NoMembers, SpaceMember};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuRequest;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowMeasure;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::{Media, MediaError, MediaFingerprint};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
/// 🙋️ Explicit actor identity for standalone local births and fixtures.
use protocol::LOCAL_ACTOR_ID;
use store::ArtifactStoreInitializationAuthority;
use semio_framework::AppRole;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_app_builder::App;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_kits::AppActionRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_reactor_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::Emit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::AppRenderOperationContext;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::ArtifactView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::ConfigView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::DraftView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::InteractionHoverState;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::InteractionView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoDraft;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoPresence;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoPresenceMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoTransient;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoTransientMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::PresenceView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_transient::window_transient::TransientView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::VcsArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::reserved_jobs::is_framework_reserved_action_id;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_registry::artifact_registration::surface_dependency_breaches;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::child_view::ChildContentView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::history_view::HistoryView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_interaction::interaction_topology::empty_draft_references;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::PeerPresenceRoot;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_plugin_program::Plugin;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ArtifactEditor;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ArtifactViewer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ViewerApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::TransactionProposalDraft;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_interaction::UiAssemblyResult;















#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../⚖️authoring-law/🦀️.rs"]
pub mod authoring_law;

#[cfg(any(test, feature = "artifact-app-testing"))]
pub mod artifact_app_laws {
    //! 🧪️ Generic test-harness helpers for `ArtifactApp` implementors. Factors out the ~24x duplicated
    //! `meta()`/`new_app()`/`new_app_with_registry()`/`paired_apps()` boilerplate plus the repeated
    //! undo-redo / two-instance-convergence / ingest-idempotency test *bodies* (parameterized by closures
    //! for the app-specific action names/snapshot shape, so only the control flow is shared). Not
    //! Enabled only by this crate's tests or the explicit `artifact-app-testing` feature; see
    //! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
    //! themselves inline.

    use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolFactoryRegistry, TypedOperationResultLane};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_app::VcsArtifactApp;
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};

    /// 🎟️ Drives the actual borrowed action endpoint and pays remaining original fixture sources under its authored close loan.
    pub async fn dispatch_fixture_action<P:PluginApp>(app:&mut P,action:&str,args:Option<semio_framework_value::DslValue>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>{
        let mut original_args=args;
        let mut result=Ok(None);
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            result=app.handle_action(action,&mut original_args,meta,identity,cx).await;
            if !matches!(result,Ok(None)){break}
            if let Err(failure)=app.maintenance_step(cx,close_loan){result=Err(failure.into());break}
        }
        close_fixture_action_inputs(original_args,None,cx,close_loan);
        result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original action fixture dispatch exceeded its authored turn bound").into())
    }

    /// 📨️ Exercises the real continuation receiver with one original argument and fault pair.
    pub async fn dispatch_fixture_response_action<P:PluginApp>(app:&mut P,action:&str,args:Option<semio_framework_value::DslValue>,fault:Option<semio_framework::Fault>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>{
        let(mut original_args,mut original_fault)=(args,fault);let mut result=Ok(None);
        for _ in 0..OriginalFixtureCaller::maximum_turns(){result=app.handle_response_action(action,&mut original_args,&mut original_fault,meta,identity,cx).await;if !matches!(result,Ok(None)){break}if let Err(failure)=app.maintenance_step(cx,close_loan){result=Err(failure.into());break}}
        close_fixture_action_inputs(original_args,original_fault,cx,close_loan);
        result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original continuation fixture dispatch exceeded its authored turn bound").into())
    }

    fn close_fixture_action_inputs(args:Option<semio_framework_value::DslValue>,fault:Option<semio_framework::Fault>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant){
        if args.is_some()||fault.is_some(){
            let mut receiving=Some(semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource{action:None,args,prepared:None,assembled:None,fault});
            let mut original_caller=None;
            let mut owner=semio_framework_plugin_instance_stores::action_ingress::ActionIngressRecord::receive(&mut receiving,&mut original_caller,cx).expect("original fixture action receiving receipt").expect("authored caller must fund original unused argument receiving");
            owner.finish_parser();
            for _ in 0..OriginalFixtureCaller::maximum_turns(){
                if owner.terminal_is_empty(){break}
                cx.with_retained_loan(close_loan,|same|owner.close_original_step(same)).expect("original fixture action unused argument close");
            }
            assert!(owner.terminal_is_empty(),"original fixture action unused source did not complete its paid close");
        }
    }

    /// 🔬️ Borrows one real app registry publication for a synchronous integration-law closure.
    pub async fn with_document_view<A, M, R>(app: &mut VcsArtifactApp<A, M>, inspect: impl for<'a> FnOnce(&semio_framework_plugin_decl_views::ArtifactView<'a, A::Snapshot>) -> R) -> Result<R, semio_framework::Fault>
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        app.render_ctx().refresh_cache().await?;
        let read = app.stores.store.snapshot_read().map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
        let (generation, revision) = (app.stores.store.generation_now(), app.stores.store.content_revision());
        if !read.commit_authority_matches(generation, revision) {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("framework.artifact-law.publication-invariant"), "law parent snapshot publication changed before document view capture").with_param("refusalKind", semio_framework_value::ValueRefusalKind::InvariantViolated.as_str()));
        }
        let (_, snapshot, _, history) = app.history.cache.as_ref().ok_or_else(|| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("law document view cache was not prepared"))?;
        if !std::ptr::eq(snapshot.as_ref(), read.get()) {
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("framework.artifact-law.publication-invariant"), "law history cache belongs to a different parent snapshot publication").with_param("refusalKind", semio_framework_value::ValueRefusalKind::InvariantViolated.as_str()));
        }
        let view = semio_framework_plugin_decl_views::ArtifactView::with_children(read.get(), history.as_ref(), semio_framework_plugin_decl_views::child_view::ChildContentView::clone(&app.child_composition.child_content_root));
        Ok(inspect(&view))
    }

    /// 🎟️ The explicit mounted owner policy every testkit fixture instance runs under.
    pub fn fixture_mounted_policy() -> semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
    }

    /// 🎟️ One valid original retained turn input for a fixture `Budget`.
    pub fn fixture_retained_turn() -> semio_framework::kernel::RetainedTurnInput {
        semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: fixture_mounted_policy().maintenance }
    }

    /// 🧮️ One fresh unfunded-by-default step context carrying exactly the operation, generation and grant of [`fixture_retained_turn`] (a kernel turn refuses a context its retained input does not name); its sequence and progress cells are leaked, which costs a few bytes per call.
    pub fn fixture_step_context() -> semio_framework_job::StepContext<'static> {
        fixture_step_context_for(fixture_retained_turn())
    }

    /// 🧮️ [`fixture_step_context`] for any retained turn input: the context names exactly that input's operation, generation and grant, so a fixture that funds a turn with a narrower grant (or none) hands the kernel a context that agrees.
    pub fn fixture_step_context_for(turn: semio_framework::kernel::RetainedTurnInput) -> semio_framework_job::StepContext<'static> {
        let sequence: &'static mut u64 = Box::leak(Box::new(0));
        let progress: &'static mut semio_framework_value::RetainedCloneProgress = Box::leak(Box::default());
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(turn.operation), semio_framework_job::Generation(turn.generation), semio_framework_job::StepBudget::new(1_000_000, u64::MAX, turn.grant), semio_framework_job::root_cancel_token(), || Some(0), sequence, progress)
    }

    /// 📒️ One leaked retained-progress cell for a hand-built `StepContext`.
    pub fn fixture_progress() -> &'static mut semio_framework_value::RetainedCloneProgress {
        Box::leak(Box::default())
    }

    /// 🤝️ Resolves a payload-free fixture job outcome (yield, cancelled or empty completion) from its descriptor.
    pub fn fixture_job_outcome<'a>(descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fixture job lends no payload outcome")),
        }
    }

    /// 📬️ Everything a fixture test reads from one lent job outcome; payload bytes are copied out, the payload itself stays in the job's custody.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum FixtureStepOutcome {
        Idle,
        Yield,
        Preview(Vec<u8>),
        Checkpoint { applied_progress: u64, state: Vec<u8> },
        Complete { state: Vec<u8>, output: Vec<u8> },
        Cancelled,
        Fault(Vec<u8>),
    }

    /// 📖️ Concatenates every page of one retained payload.
    pub fn fixture_payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        (0..payload.page_count()).filter_map(|index| payload.page(index)).flatten().copied().collect()
    }

    /// ▶️ Drives one job turn and summarises the lent outcome so the borrow ends before the test touches the job again.
    pub fn fixture_step<J: semio_framework_job::InteractiveJob + ?Sized>(job: &mut J, cx: &mut semio_framework_job::StepContext<'_>) -> FixtureStepOutcome {
        use semio_framework_job::JobOutcomeBorrow as B;
        match job.step(cx).expect("fixture job step") {
            None => FixtureStepOutcome::Idle,
            Some(B::Yield { .. }) => FixtureStepOutcome::Yield,
            Some(B::PreviewReady { payload, .. }) => FixtureStepOutcome::Preview(fixture_payload_bytes(payload)),
            Some(B::CheckpointReady { state, applied_progress, .. }) => FixtureStepOutcome::Checkpoint { applied_progress, state: fixture_payload_bytes(state) },
            Some(B::Complete { state, output, .. }) => FixtureStepOutcome::Complete { state: state.map(fixture_payload_bytes).unwrap_or_default(), output: output.map(fixture_payload_bytes).unwrap_or_default() },
            Some(B::Cancelled { .. }) => FixtureStepOutcome::Cancelled,
            Some(B::Fault { detail, .. }) => FixtureStepOutcome::Fault(fixture_payload_bytes(detail)),
        }
    }

    /// 🪪️ One local native authoring identity for a fixture call: `&mut fixture_identity()` lives for the whole statement, awaits included.
    /// Its no-op observer is a zero-sized closure, so leaking it allocates nothing.
    pub fn fixture_identity() -> semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'static> {
        let observer: &'static mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'static> = Box::leak(Box::new(|_: semio_framework_value::native_encoding::NativeEncodeProgress| true));
        semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("fixture authoring identity admission")
    }

    /// 🛬️ The archive operation a law's whole-document load runs under: bit 61 alone, below the restore (62) and cold-pair
    /// (63) namespaces.
    pub const LAW_DOCUMENT_LOAD_OPERATION: u64 = 1 << 61;

    /// 🛬️ Loads `files` into `app` the only way a whole document loads (`📓️api-stepped-document-load.md` §4): the pair is
    /// stamped with the live store's identity — exactly what the runtime does to an `Effect::LoadDocument` — admitted as a
    /// whole-document archive load (`members: []`), polled to its terminal state over real polls and acknowledged. A load
    /// that does not land answers its own fault.
    pub async fn load_document<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::os_store::serialization::ArtifactPackFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let archive = PluginApp::document_load_archive(app, files)?;
        PluginApp::begin_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION, archive)?;
        let mut polls = 0usize;
        let status = loop {
            if cx.should_yield(){return Err(semio_framework::Fault::from("document archive requires its next original source turn").into())}cx.consume_fuel(1);let grant=cx.retained_grant();PluginApp::maintenance_step(app,cx,grant)?;
            let status = PluginApp::poll_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION, identity,cx).await?;
            if !law_load_running(&status, &mut polls)? {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION)?;
        law_load_outcome(status).map_err(Into::into)
    }

    /// 🏗️ Drives one live instance's metered store constructor through the runtime exchange the reactor uses, so a fixture that
    /// loads a document straight after opening the instance finds every store owner catalog installed.
    pub async fn plugin_settle_store_constructor<PA: PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while semio_framework_plugin_runtime_ops::ops_document::plugin_store_constructor_pending(runtime, instance_id).await? {
            if std::time::Instant::now() >= deadline { return Err(semio_framework::Fault::from("the instance store constructor did not settle").into()); }
            if cx.should_yield(){return Err(semio_framework::Fault::from("store constructor requires its next original source turn").into())}cx.consume_fuel(1);
            semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime, instance_id, None, identity, cx).await?;
        }
        Ok(())
    }

    /// 🛬️ [`load_document`] for one instance of a live runtime: the archive comes from the instance's own store stamp and goes
    /// through the runtime's archive-load entry points — admit, poll to terminal, acknowledge — the path a checkpoint restore
    /// takes.
    pub async fn plugin_load_document<PA: PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, files: &store::os_store::serialization::ArtifactPackFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        plugin_settle_store_constructor(runtime, instance_id, identity,cx).await?;
        let archive = semio_framework_plugin_runtime_ops::ops_document::plugin_document_load_archive(runtime, instance_id, files).await?;
        semio_framework_plugin_runtime_ops::ops_document::plugin_begin_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION, archive).await?;
        let mut polls = 0usize;
        let status = loop {
            if cx.should_yield(){return Err(semio_framework::Fault::from("runtime archive requires its next original source turn").into())}cx.consume_fuel(1);semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime,instance_id,None,identity,cx).await?;
            let status = semio_framework_plugin_runtime_ops::ops_document::plugin_poll_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION, identity,cx).await?;
            if !law_load_running(&status, &mut polls)? {
                break status;
            }
        };
        semio_framework_plugin_runtime_ops::ops_document::plugin_acknowledge_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION).await?;
        law_load_outcome(status).map_err(Into::into)
    }

    /// 📜️ [`plugin_load_document`] for a text pair typed by `S`/`Mutation`: parsed and printed to the pack and SPR the archive
    /// load takes, like [`load_document_text`].
    pub async fn plugin_load_document_text<PA, S, Mutation>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, files: &store::os_store::text_format::ArtifactTextFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
    where
        PA: PluginApp,
        S: Clone + pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: store::OpText + store::OpBinary + store::Mutation<S> + 'static,
    {
        let parsed: store::os_store::text_format::ParsedDocumentText<S, Mutation> = store::os_store::text_format::parse_document_text(&files.dsl, &files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
        let envelope = parsed.into_envelope();
        let pack = store::os_store::text_format::print_document_pack(&envelope).await;
        envelope.retire_unadopted();
        plugin_load_document(runtime, instance_id, &pack.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?, identity,cx).await
    }

    /// ⏳️ Whether a polled law load still runs; a load that never reaches its terminal state within a million polls is a fault.
    fn law_load_running(status: &protocol::DocumentArchiveLoadStatus, polls: &mut usize) -> Result<bool, semio_framework::Fault> {
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            return Ok(false);
        }
        *polls += 1;
        if *polls >= 1_000_000 {
            return Err(semio_framework::Fault::from("the document archive load never reached a terminal state"));
        }
        Ok(true)
    }

    /// 🏁️ A law load's acknowledged terminal: `Ready` loaded, `Cancelled` and `Fault` answer a fault (the archive's own on `Fault`).
    fn law_load_outcome(status: protocol::DocumentArchiveLoadStatus) -> Result<(), semio_framework::Fault> {
        match status.state {
            protocol::DocumentArchiveLoadState::Ready => Ok(()),
            protocol::DocumentArchiveLoadState::Cancelled => Err(semio_framework::Fault::from("the document archive load was cancelled")),
            _ => Err(semio_framework_diagnostic::decode_fault_bytes(&status.fault)),
        }
    }

    /// 📜️ [`load_document`] for a text pair: the DSL and op log parse into the envelope whose pack and SPR the archive load
    /// takes, through the same printers `document_pack` uses; the parsed envelope is retired through its exact owners, never
    /// dropped.
    pub async fn load_document_text<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::os_store::text_format::ArtifactTextFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let parsed: store::os_store::text_format::ParsedDocumentText<A::Snapshot, A::Mutation> = store::os_store::text_format::parse_document_text(&files.dsl, &files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
        let envelope = parsed.into_envelope();
        let pack = store::os_store::text_format::print_document_pack(&envelope).await;
        envelope.retire_unadopted();
        load_document(app, &pack.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?, identity,cx).await
    }

    /// ⌨️ One long typing run (fixture `🧫️fixtures/⌨️typing-run/🔣️.json`: ≥ 1000 typed characters with pauses, caret moves back
    /// into the run and Backspace/Delete corrections) as the full texts a text-editor host delivers, one per changed key.
    pub struct TypingRun {
        pub initial: String,
        pub texts: Vec<String>,
        pub expected: String,
    }

    /// ⌨️ Replays the typing-run fixture under the caret-relative text-input model (typing inserts at the caret,
    /// Backspace/Delete remove one character, arrows move one character, Home/End reach the line edges, Enter inserts a line
    /// break) — the model the editor's own text-input law and Chromium's native textarea answer identically.
    pub fn typing_run() -> TypingRun {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/⌨️typing-run/🔣️.json")).expect("typing-run fixture");
        let initial = fixture["initial"].as_str().expect("typing-run initial").to_string();
        let mut text: Vec<char> = initial.chars().collect();
        let mut caret = text.len();
        let mut texts = Vec::new();
        for key in fixture["keys"].as_array().expect("typing-run keys") {
            let before = text.clone();
            match key.as_str().expect("typing-run key") {
                "pause" => {}
                "Backspace" => {
                    if caret > 0 {
                        caret -= 1;
                        text.remove(caret);
                    }
                }
                "Delete" => {
                    if caret < text.len() {
                        text.remove(caret);
                    }
                }
                "ArrowLeft" => caret = caret.saturating_sub(1),
                "ArrowRight" => caret = (caret + 1).min(text.len()),
                "Home" => caret = text[..caret].iter().rposition(|ch| *ch == '\n').map_or(0, |index| index + 1),
                "End" => caret = text[caret..].iter().position(|ch| *ch == '\n').map_or(text.len(), |index| caret + index),
                "Enter" => {
                    text.insert(caret, '\n');
                    caret += 1;
                }
                typed => {
                    for ch in typed.chars() {
                        text.insert(caret, ch);
                        caret += 1;
                    }
                }
            }
            if text != before {
                texts.push(text.iter().collect());
            }
        }
        let expected = fixture["expect"]["text"].as_str().expect("typing-run expected text").to_string();
        assert_eq!(texts.last(), Some(&expected), "the typing-run model reaches the fixture's expected text");
        TypingRun { initial, texts, expected }
    }

    /// 🎟️ The fixture's original policy admits each quoted demand without creating new authority.
    fn fixture_lifecycle_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
        let grant = fixture_mounted_policy().maintenance;
        assert!(grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth, "fixture demand exceeds its original mounted policy");
        grant
    }

    /// 🌡️ Spends the maintenance the plugin runtime spends on a pressured instance: while a displaced-owner queue of the
    /// document or a config-lane store sits at or above `store::os_store::snapshot_read::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`, or a child
    /// retirement registry at or above `MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`, the runtime drains it in
    /// one-item steps of the turn's own retained grant within the same turn. A law that commits faster
    /// than the fair rotation retires (a typing run: every keystroke amends one edit and displaces its envelope, snapshot
    /// and dag — and a content-addressed child's root and member) calls this after each settled command, as the live host does.
    pub fn drain_maintenance_pressure<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) {
        let mut idle_stages = 0;
        while app.maintenance_under_pressure() {
            let grant = fixture_lifecycle_grant(PluginApp::maintenance_retirement_demands(app, fixture_mounted_policy().maintenance.maximum_copy_bytes).expect("quoted maintenance demand"));
            if cx.should_yield(){return}
            let demand=PluginApp::maintenance_retirement_demands(app,grant.maximum_copy_bytes).expect("original maintenance quote");
            match semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,loan|Ok((PluginApp::maintenance_step(app,same,loan)?,PluginApp::maintenance_terminal_is_empty(app)))).expect("original bounded maintenance") {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress != semio_framework_value::retained_clone::RetainedCloneProgress::default() => idle_stages = 0,
                _ => idle_stages += 1,
            }
            assert!(idle_stages <= usize::from(semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES), "a pressured displaced-owner queue made no maintenance progress over a whole stage rotation");
        }
    }

    /// 🧪️ Replays `seed_genesis_children`'s roster lookup over a surface's initial document without
    /// constructing the app, so a bundle proves at test time what `with_registry_on_bus` would only
    /// discover by panicking inside a live guest. The probe snapshot is never dropped: retiring one
    /// needs the owner factories a live `ArtifactStore` installs, and this law runs once per surface in
    /// a test binary, so it hands the value to `ManuallyDrop` rather than trip the retirement assert.
    fn genesis_children_declared<S, M>(snapshot: S, derives: impl Fn(&S, &str, &str) -> Result<bool, semio_framework_value::ValueError>) -> Result<usize, semio_framework::Fault>
    where
        S: semio_framework_schema_composition::ArtifactCompositionFields,
        M: store::MemberFactory,
    {
        let snapshot = std::mem::ManuallyDrop::new(snapshot);
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&*snapshot).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(format!("initial snapshot child projection failed: {error}")))?;
        let mut derived = 0;
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            if !derives(&snapshot as &S, slot, fields.child_id).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(error.into_message()))? {
                continue;
            }
            let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema::<M>(&dialect)?;
            derived += 1;
        }
        Ok(derived)
    }

    /// ✏️🌱️ Proves this editor's OWN roster (`E::Members`) declares every dialect its initial document
    /// derives a genesis child for, and answers how many it checked — a caller pins that count so a
    /// surface that stops deriving its children cannot turn the assertion vacuous. Any bundle
    /// registering `E` instantiates that same roster, so one assertion covers every bundle.
    pub fn assert_editor_genesis_children_declared<E: semio_framework_plugin_decl_surface::ArtifactEditor>() -> Result<usize, semio_framework::Fault> {
        genesis_children_declared::<E::Snapshot, E::Members>(E::initial_snapshot(), |snapshot, slot, child_id| E::genesis_child_pack(snapshot, slot, child_id).map(|value| value.is_some()))
    }

    /// 👁️🌱️ Viewer twin of `assert_editor_genesis_children_declared`.
    pub fn assert_viewer_genesis_children_declared<V: semio_framework_plugin_decl_surface::ArtifactViewer>() -> Result<usize, semio_framework::Fault> {
        genesis_children_declared::<V::Snapshot, V::Members>(V::initial_snapshot(), |snapshot, slot, child_id| V::genesis_child_pack(snapshot, slot, child_id).map(|value| value.is_some()))
    }

    pub const FIXTURE_TREE_MAX_DEPTH: usize = 64;
    pub const FIXTURE_TREE_MAX_NODES: usize = semio_framework_ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS;
    pub const FIXTURE_TREE_RETIRE_STEPS: usize = FIXTURE_TREE_MAX_NODES * (semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX + 1);

    /// 🔗️ The action bindings are part of the surface a host receives — a tree item's
    /// `activate` route lives in `BuiltNode.bindings`, never inside its `Component`, so a
    /// projection that omitted them showed an inert row and every "this window offers action
    /// X" assertion read against a tree that had no actions at all
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ♿️ …and so is the accessibility spec, for exactly the same reason: a `Component::Surface`
    /// paints into a canvas with no accessible children, so the record's own name, description,
    /// liveness and `aria-keyshortcuts` are ALL an assistive technology ever learns about it. A
    /// projection that omitted them made every "this canvas is named / announces / advertises its
    /// chord" assertion read against a tree that carried no accessibility at all
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn project_fixture_node(node: &semio_framework_ui_contract::BuiltNode, remaining: &mut usize, depth: usize) -> Result<serde_json::Value, &'static str> {
        if depth >= FIXTURE_TREE_MAX_DEPTH || *remaining == 0 {
            return Err("tree-limit");
        }
        *remaining -= 1;
        if !node.rejected_children.is_empty() {
            return Err("rejected-children");
        }
        let component = serde_json::to_value(&node.component).map_err(|_| "component-json")?;
        if component["type"].as_str().is_none() {
            return Err("component-type");
        }
        let mut children = Vec::with_capacity(node.children.len());
        for (index, child) in node.children.iter().enumerate() {
            if node.children.iter().take(index).any(|previous| previous.key == child.key) {
                return Err("duplicate-key");
            }
            children.push(project_fixture_node(child, remaining, depth + 1)?);
        }
        let bindings = serde_json::to_value(&node.bindings).map_err(|_| "bindings-json")?;
        let accessibility = serde_json::to_value(&node.accessibility).map_err(|_| "accessibility-json")?;
        Ok(serde_json::json!({ "key": node.key.as_str(), "component": component, "bindings": bindings, "disabled": node.disabled, "accessibility": accessibility, "children": children }))
    }

    /// 🖼️ Observes fixture keys/components/children and retires that exact tree before any outcome; not a wire codec.
    pub fn project_and_retire_fixture_tree(tree: semio_framework_ui_runtime::ComponentTree) -> Result<String, &'static str> {
        observe_and_retire_fixture_tree(tree, |root| {
            let mut remaining = FIXTURE_TREE_MAX_NODES;
            project_fixture_node(root, &mut remaining, 0).and_then(|value| serde_json::to_string(&value).map_err(|_| "projection-json"))
        })
    }

    /// ⌨️ The input commit-mode lint (audit F-4, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every field of a
    /// projected body that would publish one document edit per keystroke — an `Input` of any kind but `number` and
    /// `color` (continuous controls, the framework scrub, in both hosts) and `file`, bound through `change` to a verb
    /// `is_document_verb` names, without the blur commit policy. Answers `"<node key> → <verb>"` per offender, sorted.
    /// Fixture `🔌️plugin/🧫️fixtures/🧫️input-commit-lint/🔣️.json`.
    pub fn document_input_commit_findings(projection: &serde_json::Value, is_document_verb: &dyn Fn(&str) -> bool) -> Vec<String> {
        let mut findings = Vec::new();
        let mut stack = vec![projection];
        while let Some(node) = stack.pop() {
            let component = &node["component"];
            if component["type"] == "input" && !matches!(component["kind"].as_str(), Some("number" | "color" | "file")) && component["commit"].as_str() != Some("blur") {
                for binding in node["bindings"].as_array().into_iter().flatten().filter(|binding| binding["trigger"] == "change") {
                    if let Some(verb) = binding["action"]["name"].as_str().filter(|verb| is_document_verb(verb)) {
                        findings.push(format!("{} → {verb}", node["key"].as_str().unwrap_or_default()));
                    }
                }
            }
            stack.extend(node["children"].as_array().into_iter().flatten());
        }
        findings.sort();
        findings
    }

    /// ⌨️ [`document_input_commit_findings`] of `body_key` rendered under `view`; a document verb is one declared with the
    /// `Mutation` kind.
    pub async fn document_input_commit_lint<A: ArtifactApp + Default, M: store::SpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, body_key: &str, view: &semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<String>, semio_framework_job::OriginalTurnFailure> {
        let projected = render_registered_fixture_app(app, body_key, view,cx).await?;
        let projection: serde_json::Value = serde_json::from_str(&projected).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(error.to_string()))?;
        Ok(document_input_commit_findings(&projection, &|verb| app.tools.registry.get(verb).is_some_and(|definition| definition.kind == semio_framework::ActionKind::Mutation)))
    }

    /// 🎬️ The projected fixture node carrying `T`'s scene — the root itself when a body IS a bare
    /// surface, otherwise the one surface of that schema anywhere beneath it. A window body is free
    /// to compose its scene alongside semantic siblings (generation3d's Flow window renders the
    /// graph's node/port/wire rows beside the GPU canvas), and a scene assertion may not care which
    /// of the two shapes the body under test happens to be.
    pub fn fixture_scene_node<T: semio_framework_ui_scene::SceneDoc>(tree: &serde_json::Value) -> Option<&serde_json::Value> {
        let component = &tree["component"];
        if component["type"].as_str() == Some("surface") && component["docSchema"].as_str() == Some(T::SCHEMA) {
            return Some(tree);
        }
        tree["children"].as_array().and_then(|children| children.iter().find_map(fixture_scene_node::<T>))
    }

    /// 🎬️ Decodes the exact typed scene a projected fixture carries.
    pub fn decode_fixture_scene<T: semio_framework_ui_scene::SceneDoc>(projection: &str) -> Result<T, &'static str> {
        let tree: serde_json::Value = serde_json::from_str(projection).map_err(|_| "fixture-scene-json")?;
        let component = &fixture_scene_node::<T>(&tree).ok_or("fixture-scene-schema")?["component"];
        let bytes = component["doc"]["bytes"].as_array().ok_or("fixture-scene-bytes")?.iter().map(|value| value.as_u64().and_then(|value| u8::try_from(value).ok()).ok_or("fixture-scene-byte")).collect::<Result<Vec<_>, _>>()?;
        T::decode_pack(&bytes).map_err(|_| "fixture-scene-pack")
    }

    /// 🚚️ The ASSEMBLED scene a render host reads, taken straight off ONE built surface node: the
    /// spine decoded out of the fixed-capacity `SurfaceProps.doc`, with every out-of-doc payload
    /// lane hanging off that same node merged back in. `semio_framework_ui_scene::decode` alone
    /// returns the SPINE — since ticket 26/09/02 wave P a world-3d scene's `instancesJson`/
    /// `meshesJson`/… do not ride inside the doc at all, so a test that reads one off a bare
    /// decode reads an empty field.
    pub fn built_surface_scene<T: semio_framework_ui_scene::SceneDoc>(node: &semio_framework_ui_contract::BuiltNode) -> Result<T, &'static str> {
        let semio_framework_ui_contract::Component::Surface(props) = &node.component else { return Err("built-surface-component") };
        let mut scene: T = semio_framework_ui_scene::decode(props).map_err(|_| "built-surface-decode")?;
        for child in node.children.iter() {
            scene.merge_lane(child.key.as_str(), built_carrier_text(child));
        }
        Ok(scene)
    }

    /// 🚚️ Depth-first concatenation of every `Component::Text` leaf under one built lane carrier.
    pub fn built_carrier_text(node: &semio_framework_ui_contract::BuiltNode) -> String {
        let mut payload = String::new();
        let mut order = Vec::new();
        let mut frontier = vec![node];
        while let Some(current) = frontier.pop() {
            order.push(current);
            frontier.extend(current.children.iter().rev());
        }
        for current in order {
            if let semio_framework_ui_contract::Component::Text(text) = &current.component {
                payload.push_str(&text.packed_payload());
            }
        }
        payload
    }

    /// 🚚️ Depth-first concatenation of every `text` leaf under a projected fixture node — the exact
    /// inverse of [`crate::app::paged_text_carrier`], and the Rust twin of the React Interpreter's
    /// own `surfaceSceneLaneText`.
    pub fn fixture_carrier_text(node: &serde_json::Value) -> String {
        let mut payload = String::new();
        let mut frontier = vec![node];
        let mut order = Vec::new();
        while let Some(current) = frontier.pop() {
            order.push(current);
            if let Some(children) = current["children"].as_array() {
                for child in children.iter().rev() {
                    frontier.push(child);
                }
            }
        }
        for current in order {
            if current["component"]["type"].as_str() == Some("text") {
                let value = current["component"]["value"].as_str().unwrap_or("");
                let mut attributes = Vec::new();
                if let Some(map) = current["component"]["dataAttributes"].as_object() {
                    let mut keys: Vec<&String> = map.keys().collect();
                    keys.sort();
                    for key in keys {
                        if let Some(slice) = map[key].as_str() {
                            attributes.push(slice);
                        }
                    }
                }
                payload.push_str(&semio_framework_ui_contract::packed_text_leaf(value, attributes));
            }
        }
        payload
    }

    /// 🚚️ Decodes a projected surface's typed scene AND re-attaches every out-of-doc payload lane
    /// hanging off it — the Rust twin of the React Interpreter's `world3dSurfaceLaneTexts` +
    /// `world3dSceneFromLanes`. A child whose key is not a lane carrier is ignored.
    pub fn decode_fixture_scene_with_lanes<T: semio_framework_ui_scene::SceneDoc>(projection: &str) -> Result<T, &'static str> {
        let tree: serde_json::Value = serde_json::from_str(projection).map_err(|_| "fixture-scene-json")?;
        let mut scene = decode_fixture_scene::<T>(projection)?;
        for child in fixture_scene_node::<T>(&tree).ok_or("fixture-scene-schema")?["children"].as_array().ok_or("fixture-scene-children")? {
            let key = child["key"].as_str().ok_or("fixture-scene-lane-key")?;
            scene.merge_lane(key, fixture_carrier_text(child));
        }
        Ok(scene)
    }

    #[doc(hidden)]
pub fn observe_and_retire_fixture_tree<R>(tree: semio_framework_ui_runtime::ComponentTree, project: impl FnOnce(&semio_framework_ui_contract::BuiltNode) -> R) -> R {
        let projected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| project(&tree.root)));
        let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(tree.root);
        while !retirement.terminal_is_empty() {
            let step = retirement.close_step(1, 4096).expect("fixture's exact built-tree retirement remains valid");
            if !step.progressed {
                std::thread::yield_now();
            }
        }
        match projected {
            Ok(result) => result,
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }

    /// 🧪️ Mounts the production shared reserved-action factories on an isolated harness bus.
    pub fn register_framework_reserved_action_factories<A: ArtifactApp>(bus: &semio_framework::ActionBus, controller_id: &str) -> Result<(), semio_framework::Fault> {
        let mut registry = ArtifactToolFactoryRegistry::<A>::new(bus, controller_id);
        register_framework_reserved_factories(&mut registry)
    }

    /// 🪪️ A local-actor `ActionMeta` for test dispatch (`instance_id: 1`).
    // 🚫️async: E1 pure constructor, called pervasively as `&meta("...")` — see R9.
    pub fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1, view_state: None }
    }

    // 🏛️ 👁️✏️ W3 dissolution (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the two-
    // generations-obsolete `assert_constitutional_crates`/`assert_taxonomy_components` gate (ticket
    // 26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES) hunted for a literal
    // `🎛️apps/` directory next to a plugin's manifest dir and hard-`panic!`ked when none was found.
    // Repo-wide grep confirmed zero callers (not even `Plugin::builder`, despite its own docstring's
    // stale claim) — dead code whose ONE reachable branch would now panic unconditionally, since
    // `🎛️apps` can never exist again. Deleted rather than retargeted: the shape it enforced (an
    // app-root ⚙️engine, `constitutional-crate` slots) is itself retired by the ENGINELESS ticket and
    // by this ticket's surface dissolution, and the same completeness duty is covered live by the
    // TS-side `validateTaxonomyTree` (📇️registry/📜️script.ts) and `policyTaxonomyDirsBreaches` (root
    // 📜️script.ts), both already surface-aware.

    /// 🧬️ A registry-less wrapper (`VcsArtifactApp::new`) for store-only tests. UI and typed
    /// command dispatch fail closed until a real registry and exact factory are supplied.
    pub async fn new_app<A: ArtifactApp + Default>(actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A> {
        let mut app = VcsArtifactApp::new(A::default(), actor, mounted_policy, identity).await;
        settle_store_constructor(&mut app, mounted_policy);
        app
    }

    /// 🧬️ A registry-backed wrapper carrying `manifest`'s real `AppActionRegistry` — needed whenever a
    /// test must exercise declared-arg defaults/required-arg enforcement or View/Shell kind discipline.
    pub async fn new_app_with_registry<A: ArtifactApp + Default>(manifest: fn() -> App, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A> {
        new_app_with_registry_and_members::<A, store::NoMembers>(manifest, actor, mounted_policy, identity).await
    }

    /// 🧬️ `new_app_with_registry` over an explicit member roster `M` — for an app whose snapshot
    /// composes children it derives (`ArtifactApp::genesis_child_pack`), which `NoMembers` can never open.
    pub async fn new_app_with_registry_and_members<A: ArtifactApp + Default, M: store::SpaceMember + store::MemberFactory + Send + 'static>(manifest: fn() -> App, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A, M> {
        let definition = manifest().definition;
        let registry = AppActionRegistry::from_definition(&definition);
        drop(definition);
        let mut app = VcsArtifactApp::with_registry(A::default(), registry, actor, mounted_policy, identity).await;
        settle_store_constructor(&mut app, mounted_policy);
        app
    }

    /// 🧬️ A registry-backed wrapper whose manifest is authored asynchronously by the concrete fixture,
    /// over the default (empty) member roster. The body lives in
    /// [`new_registered_app_with_members`]; this is the `NoMembers` spelling of it.
    pub async fn new_registered_app<A, Manifest>(manifest: Manifest, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A>
    where
        A: ArtifactApp + Default,
        Manifest: std::future::Future<Output = App>,
    {
        new_registered_app_with_members::<A, store::NoMembers, Manifest>(manifest, actor, mounted_policy, identity).await
    }

    /// 🧬️ [`new_registered_app`] over an explicit member roster `M`. An app whose
    /// `ArtifactApp::genesis_child_pack` mints a composed child cannot be constructed on the default
    /// roster at all — its own construction fails `derived child dialect '…' is not declared by this
    /// app's member roster` — so every registered law was unreachable for a composing app
    /// (mathematical's equation and imperative's procedure, ticket
    /// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, knowledge). Rust has no default type parameter on
    /// a function, so the roster-carrying generic is the twin and the plain name delegates to it.
    ///
    /// 🪪️ A REGISTERED fixture app is by definition mounted: `dispatch_typed_command_inner`
    /// refuses `interactive-job.live-instance` for any command whose `ActionMeta.instance_id`
    /// is not the app's bound live runtime instance, and a fresh app has none. `meta` stamps 1,
    /// so every caller of this helper gets the instance its own `meta(...)` addresses; a caller
    /// that binds a different id afterwards simply overwrites this one.
    pub async fn new_registered_app_with_members<A, M, Manifest>(manifest: Manifest, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A, M>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        Manifest: std::future::Future<Output = App>,
    {
        let definition = manifest.await.definition;
        let mut app = VcsArtifactApp::with_registry(A::default(), AppActionRegistry::from_definition(&definition), actor, mounted_policy, identity).await;
        app.bind_instance_id(meta("local").instance_id).await;
        settle_store_constructor(&mut app, mounted_policy);
        app
    }

    /// 🏗️ A `VcsArtifactApp` is born constructing: its store owner catalogs are installed by metered constructor turns
    /// the reactor drives before it admits any command. A fixture that reaches the stores directly settles that
    /// constructor first, claiming and validating every phase receipt exactly as the reactor's exchange does.
    pub fn settle_store_constructor<A: ArtifactApp, M: store::SpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> Vec<TypedOperationFixtureOwnershipReceipt> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut receipts = Vec::new();
        loop {
            retain_original_ownership_receipts(app, mounted_policy, &mut receipts).expect("original fixture constructor receipts");
            if !app.store_constructor_is_pending() { return receipts; }
            assert!(std::time::Instant::now() < deadline, "registered fixture store constructor did not settle");
            app.stores.drive_store_constructor_unit(&mut app.constructor, mounted_policy.preparation).expect("registered fixture store constructor turn");
        }
    }

    /// 🚰️ Drains a standalone Store's displaced-owner maintenance queue to terminal emptiness under its own quoted grant — the caller no scheduler funds — so a fixture that edits a long history frees the fixed 1 024-slot retirement authority between batches.
    pub fn drain_store_displaced_retirements<P, Mutation>(store: &mut store::ArtifactStore<P, Mutation>)
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + store::OpBinary + store::OpText + Send + 'static,
    {
        for _ in 0..1_000_000 {
            if store.maintenance_retirements_terminal_is_empty() { return; }
            let demand = store.maintenance_retirements_demands(4096).expect("original displaced-owner quote");
            store.maintenance_retirements_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(4096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }).expect("original displaced-owner step");
        }
        panic!("the displaced-owner queue did not drain under its quoted grants");
    }

    /// 🧹 The close-side twin of [`settle_store_constructor`]: drives the constructor's own close phase to terminal emptiness, claiming every phase receipt, so the close quote that follows names the next application owner instead of the constructor.
    pub fn settle_store_constructor_close<A: ArtifactApp, M: store::SpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> Vec<TypedOperationFixtureOwnershipReceipt> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut receipts = Vec::new();
        loop {
            retain_original_ownership_receipts(app, mounted_policy, &mut receipts).expect("original fixture constructor close receipts");
            if app.constructor.constructor_terminal_is_empty() { return receipts; }
            assert!(std::time::Instant::now() < deadline, "registered fixture store constructor close did not settle");
            app.stores.close_store_constructor_unit(&mut app.constructor, mounted_policy.close).expect("registered fixture store constructor close turn");
        }
    }

    /// 📬️ Host-visible output collected while one registered fixture operation reaches quiescence.
    /// 🧾️ Fixtures retain the original producer receipt under the caller's declared phase policy.
    pub enum TypedOperationFixtureOwnershipReceipt {
        Constructor { kind: semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind, phase: semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        Preparation { operation: semio_framework_job::OperationId, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        Retirement { operation: semio_framework_job::OperationId, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        History { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        ToolRun { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        LocalInteraction { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
        Maintenance { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    }

    pub struct TypedOperationFixtureReceipt {
        pub ownership_receipts: Vec<TypedOperationFixtureOwnershipReceipt>,
        pub lanes: Vec<TypedOperationResultLane>,
        pub effects: Vec<semio_framework::kernel::Effect>,
        pub events: Vec<semio_framework::kernel::AppEvent>,
        pub ui_scope: Option<semio_framework::kernel::UiDirtyScope>,
        pub completions: usize,
        pub completion_operations: Vec<u64>,
        pub revisions: Vec<u64>,
        /// 🧩️ One entry per composed gesture this settle observed — see
        /// [`crate::app::ComposedGestureResult`].
        pub composed: Vec<semio_framework_plugin_composition_publication::ComposedGestureResult>,
    }

    /// 🎚️ Projects current config from the concrete window registry, including its applied event history.
    pub async fn capture_fixture_window_config<O: semio_framework_plugin_window_config::WindowConfigOwner, A: ArtifactApp, M: store::SpaceMember + store::MemberFactory>(app: &mut VcsArtifactApp<A, M>, view: &semio_framework::ViewModel) -> Result<Option<O::State>, semio_framework::Fault> {
        Ok(app.stores.window_config_store.capture(Some(view)).await?.and_then(|authority| authority.snapshot.get::<O>().cloned()))
    }

    /// 🔁️ Drives the same bounded continuation and exact ACK protocol as the plugin host.
    ///
    /// 📄️ EVERY presented page, not one per turn: `has_pending_typed_operations` counts the
    /// effect/event/ui/completion outboxes and the mounted operations, but NOT a presented
    /// page waiting for its ACK — so a turn that leaves a second page queued can be the turn
    /// the loop exits on, and that page is never seen. A command that publishes an artifact,
    /// a config and an app transient reported two of the three
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🧩️ The composed lane rides the same bounded outbox and is counted by
    /// `has_pending_typed_operations`, so leaving it undrained would spin this loop to its
    /// 30 s deadline exactly the way the terminal witness once did.
    ///
    /// 🧹️ The terminal witness lands in its OWN outbox (`typed_completion_outbox`), which
    /// `has_pending_typed_operations` counts — leaving it undrained spun this loop until
    /// its 30 s deadline for every operation that produced no lane page at all
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 📜️ A pending history patch (a time-travel stage change) rides a UI-progress frame the host takes every turn;
    /// left untaken it kept `has_pending_typed_operations` true until the deadline for the first operation after a
    /// history edit (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, forms frozen-press law).
    pub async fn settle_registered_typed_operation<P: PluginApp>(app: &mut P, receiver: u32, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut turns=0;let maximum_turns=OriginalFixtureCaller::maximum_turns();
        let mut receipt = TypedOperationFixtureReceipt { ownership_receipts: Vec::new(), lanes: Vec::new(), effects: Vec::new(), events: Vec::new(), ui_scope: None, completions: 0, completion_operations: Vec::new(), revisions: Vec::new(), composed: Vec::new() };
        retain_registered_ownership_receipts(app, mounted_policy, &mut receipt)?;
        while app.has_pending_typed_operations() {
            if cx.should_yield()||turns>=maximum_turns{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture settlement exhausted its caller opportunity").into())}turns+=1;
            let grant = mounted_policy.maintenance;
            let demand = app.maintenance_retirement_demands(grant.maximum_copy_bytes)?;
            let maintenance = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant| {
                let step = app.maintenance_step(same,grant)?;
                Ok((step, app.maintenance_terminal_is_empty()))
            })?;
            if let Some(progress) = maintenance.progress() { receipt.ownership_receipts.push(TypedOperationFixtureOwnershipReceipt::Maintenance { grant, progress }); }
            app.advance_typed_operation_publication(identity,cx).await?;
            retain_registered_ownership_receipts(app, mounted_policy, &mut receipt)?;
            while let Some(page) = app.take_typed_operation_result_page(receiver) {
                let lane = page.lane;
                let fault = (lane == TypedOperationResultLane::Fault).then(|| semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page(page.bytes()));
                if !app.acknowledge_typed_operation_result(page.token)? {
                    return Err(semio_framework::Fault::from("registered fixture typed operation rejected its exact result ACK").into());
                }
                receipt.lanes.push(lane);
                if let Some(fault) = fault { return Err(fault.into()); }
            }
            while let Some(effect) = app.take_typed_operation_effect() {
                receipt.effects.push(effect);
            }
            while let Some(event) = app.take_typed_operation_event() {
                receipt.events.push(event);
            }
            while let Some(scope) = app.take_typed_operation_ui_scope() {
                receipt.ui_scope = Some(scope);
            }
            while let Some(progress) = app.take_typed_operation_ui_progress() {
                receipt.ui_scope = receipt.ui_scope.or(Some(progress.ui_scope));
            }
            while let Some(composed) = app.take_typed_operation_composed_result() {
                receipt.composed.push(composed);
            }
            while let Some(completion) = app.take_typed_operation_completion().await? {
                receipt.completions += 1;
                receipt.completion_operations.push(completion.operation);
                receipt.revisions.push(completion.revision);
            }
            while let Some(reply) = app.take_local_interaction_query_reply() {
                if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                    let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                    if !app.acknowledge_local_interaction_query(&token) {
                        return Err(semio_framework::Fault::from("registered fixture typed operation rejected its exact local-interaction ACK").into());
                    }
                }
            }
            std::thread::yield_now();
        }
        Ok(receipt)
    }

    fn retain_registered_ownership_receipts<P: PluginApp>(app: &mut P, policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, recipient: &mut TypedOperationFixtureReceipt) -> Result<(), semio_framework::Fault> {
        retain_original_ownership_receipts(app, policy, &mut recipient.ownership_receipts)
    }

    /// 🧾️ Collects original producer receipts once before any subsequent semantic fixture callback.
    #[doc(hidden)]
pub fn retain_original_ownership_receipts<P: PluginApp>(app: &mut P, policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, recipient: &mut Vec<TypedOperationFixtureOwnershipReceipt>) -> Result<(), semio_framework::Fault> {
        if let Some((kind, phase, grant, progress)) = app.take_constructor_receipt() {
            let original = match phase { semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Preparation => policy.preparation, semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Close => fixture_close_grant(policy) };
            if grant != original || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture constructor receipt differs from its original caller authority")); }
            recipient.push(TypedOperationFixtureOwnershipReceipt::Constructor { kind, phase, grant, progress });
        }
        if let Some((operation, grant, progress)) = app.take_typed_operation_preparation_receipt() {
            if grant != policy.preparation || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture preparation receipt differs from its original caller authority")); }
            recipient.push(TypedOperationFixtureOwnershipReceipt::Preparation { operation, grant, progress });
        }
        if let Some((operation, grant, progress)) = app.take_typed_operation_retirement_receipt() {
            if grant != policy.close || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture Close receipt differs from its original caller authority")); }
            recipient.push(TypedOperationFixtureOwnershipReceipt::Retirement { operation, grant, progress });
        }
        if let Some((grant, progress)) = app.take_history_command_receipt() {
            if grant != policy.maintenance || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture history receipt differs from its original caller authority")); }
            recipient.push(TypedOperationFixtureOwnershipReceipt::History { grant, progress });
        }
        if let Some((grant, progress)) = app.take_tool_run_receipt() {
            if grant != policy.maintenance || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture tool run receipt differs from its original caller authority")); }
            recipient.push(TypedOperationFixtureOwnershipReceipt::ToolRun { grant, progress });
        }
        if let Some((grant,progress))=app.take_local_interaction_query_receipt(){
            if grant!=policy.preparation||!progress.fits(grant){return Err(semio_framework::Fault::from("registered fixture interaction receipt differs from its original caller authority"))}
            recipient.push(TypedOperationFixtureOwnershipReceipt::LocalInteraction{grant,progress});
        }
        Ok(())
    }

    /// 🔁️ One minted extension invocation as a fixture observes it — the `Effect::InvokeExtension`
    /// `reactor::poll` would have handed the shell.
    pub struct PendingExtensionInvocation {
        pub extension_id: String,
        pub capability: String,
        pub request_json: String,
    }

    /// 🔁️ The HOST half of the extension round trip, for a fixture that drives a `PluginApp`
    /// directly and therefore has no `reactor::poll` to carry it.
    ///
    /// An app declares its request through `Emit::extension_invocations`; the SDK mints the
    /// `req`, parks a continuation and queues one `Effect::InvokeExtension` on the actor's
    /// outbound queue. In production the shell runs the capability and answers with
    /// `Event::Completed`, which `poll` routes back into `plugin_dispatch_response_action`. This
    /// helper performs exactly those three steps: it drains the queued invocations, asks
    /// `serve` for each capability's bytes, and dispatches the recorded `response_action` — with
    /// the ORIGINAL request's own correlation fields plus `ok`/`outputJson` — back into `app`
    /// through the same `PluginApp::handle_command` seam, settling each redispatch. Returns how
    /// many invocations were answered, so a caller can loop until the chain quiesces
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🔁️ What one settled round of extension answers produced: how many invocations were
    /// answered, and every `Effect` the redispatched response actions emitted in turn — a
    /// continuation that re-arms its own chain (e.g. `flowEvalResolve` → `flowEvalTick`) is only
    /// observable to the caller through these (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub struct SettledExtensionInvocations {
        pub ownership_receipts: Vec<TypedOperationFixtureOwnershipReceipt>,
        pub answered: usize,
        pub effects: Vec<semio_framework::kernel::Effect>,
    }

    /// 📦️ Exactly what `runCapturedExtensionEffect` (`🏛️ShellHost/🟦️.tsx`) does with the
    /// capability's own JSON answer before it crosses the ABI:
    /// `encodePackValue(JSON.parse(outputJson))`. A fixture that handed the raw JSON
    /// bytes over instead was green against a shape the shell never sends
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn settle_extension_invocations<P: PluginApp>(
        app: &mut P,
        receiver: u32,
        action_meta: &ActionMeta,
        serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, semio_framework::Fault>,
        mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
        identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<SettledExtensionInvocations,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut settled = SettledExtensionInvocations { ownership_receipts: Vec::new(), answered: 0, effects: Vec::new() };
        for effect in semio_framework_plugin_reactor_tasks::tasks_extension_continuation::drain_queued_effects(receiver) {
            let semio_framework::kernel::Effect::InvokeExtension { req, extension_id, capability, request_json, .. } = effect else { continue };
            let pending = PendingExtensionInvocation { extension_id, capability, request_json };
            let outcome = serve(&pending).and_then(|json| {
                let text = String::from_utf8(json).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.answer-not-json"), error.to_string()))?;
                let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.answer-not-json"), format!("the served capability answer is not JSON: {error}")))?;
                Ok(store::pack_rt::encode_wire_value(&value))
            });
            let Ok((instance, action, args, original_fault)) = semio_framework_plugin_reactor_tasks::tasks_extension_continuation::take_extension_response(req, outcome) else { continue };
            let meta = ActionMeta { instance_id: instance, ..action_meta.clone() };
            dispatch_fixture_response_action(app,&action,Some(args),original_fault,&meta,identity,cx,mounted_policy.close).await?;
            let receipt = settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await?;
            settled.ownership_receipts.extend(receipt.ownership_receipts);
            settled.effects.extend(receipt.effects);
            settled.answered += 1;
        }
        Ok(settled)
    }

    /// 🪪️ The close grant every fixture turn is driven under: exactly the caller's close policy, as a mounted runtime drives it.
    #[doc(hidden)]
pub fn fixture_close_grant(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> RetainedCloneGrant {
        mounted_policy.close
    }

    /// 🧹️ Closes one registered fixture through the exact retained app close state machine.
    pub fn close_registered_fixture_app<A: PluginApp>(app: &mut A, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<TypedOperationFixtureOwnershipReceipt>,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut pending_authority = None;
        let mut last_quote = None;
        let mut close_turn = 0usize;let maximum_turns=OriginalFixtureCaller::maximum_turns();
        let mut ownership_receipts = Vec::new();
        retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts)?;
        while close_turn<maximum_turns {
            if app.close_terminal_is_empty() { return Ok(ownership_receipts); }
            if cx.should_yield(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture close exhausted its caller opportunity").into())}
            let grant = fixture_close_grant(mounted_policy);
            let demand = app.close_retirement_demands(grant.maximum_copy_bytes)?;
            last_quote = Some((demand, grant));
            let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant| {
                Ok((app.close_step(same,grant)?, app.close_terminal_is_empty()))
            })?;
            retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts)?;
            match step {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => {
                    assert!(progress.fits(grant), "registered fixture close turn {close_turn} exceeded its exact physical grant");
                    if matches!(step, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)) { break; }
                    if progress == Default::default() { std::thread::yield_now(); }
                }
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason } | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason } => {
                    pending_authority = Some(reason);
                    std::thread::yield_now();
                }
            }
            close_turn = close_turn.saturating_add(1);
        }
        assert!(app.close_terminal_is_empty(), "registered fixture did not reach its exact terminal-empty witness{}{}", pending_authority.map(|reason| format!(", last pending close authority: {reason}")).unwrap_or_default(), last_quote.map(|(demand, grant)| format!(", last close quote {demand:?} against grant {grant:?} on rung {}", app.close_diagnostic())).unwrap_or_default());
        Ok(ownership_receipts)
    }

    /// 🔗️ Two registry-less store-only instances joined by an in-memory backbone on `channel`.
    pub async fn paired_apps<A: ArtifactApp + Default>(channel: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A>, VcsArtifactApp<A>) {
        let mut a = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        let mut b = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel).await;
        a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
        b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");
        (a, b)
    }

    /// 🧬️ Two explicitly registered fixture instances joined by an in-memory backbone, over the
    /// default (empty) member roster — the `NoMembers` spelling of
    /// [`paired_registered_apps_with_members`].
    pub async fn paired_registered_apps<A, Manifest, Build>(channel: &str, manifest: Build, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A>, VcsArtifactApp<A>)
    where
        A: ArtifactApp + Default,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        paired_registered_apps_with_members::<A, store::NoMembers, Manifest, Build>(channel, manifest, mounted_policy, identity).await
    }

    /// 🧬️ [`paired_registered_apps`] over an explicit member roster `M` — see
    /// [`new_registered_app_with_members`] for why a composing app needs it.
    pub async fn paired_registered_apps_with_members<A, M, Manifest, Build>(channel: &str, manifest: Build, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A, M>, VcsArtifactApp<A, M>)
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        let mut a = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        let mut b = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        a.bind_instance_id(meta("local").instance_id).await;
        b.bind_instance_id(meta("local").instance_id).await;
        let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel).await;
        a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
        b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");
        (a, b)
    }

    /// ⏪️ Drives ONE framework-reserved history verb the whole way a shell does: `handle_action`
    /// only ADMITS `"undo"`/`"redo"` — they are `framework_reserved_job!` routes
    /// (`FrameworkUndoJob`/`FrameworkRedoJob`), so the admission comes back as an
    /// `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }` the caller has to run before the
    /// store ever sees `ArtifactCommand::Undo`. Dropping that receipt and pumping only the typed
    /// publication left every retained app's undo a no-op, which read as a flaky
    /// `undo did not revert to the expected snapshot` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// A non-retained app admits no spawn job and
    /// [`super::settle_framework_reserved_admission`] hands the same result straight back.
    pub async fn settle_history_verb<A, M>(app: &mut VcsArtifactApp<A, M>, action: &str, receiver: u32, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let admitted = dispatch_fixture_action(app,action,None,&meta("local"),identity,cx,mounted_policy.close).await.unwrap_or_else(|fault| panic!("history verb {action} admission: {fault:?}"));
        semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await.unwrap_or_else(|fault| panic!("history verb {action} reserved-job commit: {fault:?}"));
        settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await.unwrap_or_else(|fault| panic!("history verb {action} publication: {fault:?}"));
    }

    /// 🧪️ Runs `command` once via the typed channel, asserts `probe(app)` matches `after`, undoes (still
    /// the framework-reserved `"undo"` action) and asserts `before`, redoes and asserts `after` again — the
    /// repeated undo/redo round-trip test body. B1: takes a typed `A::Command` value (`dispatch_typed`) —
    /// `ArtifactApp::handle_action`'s stringly-typed dispatch no longer exists.
    ///
    /// 🔁️ Every step SETTLES: a retained tool's `dispatch_typed` only returns an admission
    /// receipt, so probing straight after it observes the pre-command document and the
    /// round trip passes or fails against the wrong state entirely. A non-retained app has
    /// nothing pending and settles in zero turns (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn assert_undo_redo_round_trip<A, M, P>(app: &mut VcsArtifactApp<A, M>, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, before: P, after: P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        P: PartialEq + std::fmt::Debug,
    {
        let receiver = meta("local").instance_id;
        app.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");
        settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await.expect("apply command publication");
        assert_eq!(probe(app), after, "command did not produce the expected snapshot");
        settle_history_verb(app, "undo", receiver, mounted_policy, identity,cx).await;
        assert_eq!(probe(app), before, "undo did not revert to the expected snapshot");
        settle_history_verb(app, "redo", receiver, mounted_policy, identity,cx).await;
        assert_eq!(probe(app), after, "redo did not reapply the expected snapshot");
    }

    pub use semio_framework_plugin_declared_verdict::{
        DeclaredArgumentProbe, DeclaredVerbEffect, DeclaredVerbFinding, DeclaredVerbOutcome, DeclaredVerbProbe, DeclaredVerbWindowProbe, declared_argument_alternatives, declared_bridge_arguments, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_findings, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document
    };

    /// 🧪️ Every declared app action bridges through `command_from_action` and round-trips `command_id`
    /// with its declared defaults and schema-validated representatives for required inputs without defaults.
    ///
    /// 🧱️ Window-KIT catalog rows are framework-owned surface verbs, not app actions: the shared
    /// `window_kind_definition` scaffold below mints them (and stamps them `Migrated`) for the
    /// editable variants of `TextWindowKit`/`TableWindowKit`/`TreeWindowKit`, so every app that
    /// composes one of those kits inherits an id it never authored a command row for. They belong
    /// in this skip list for exactly the same reason the framework-reserved verbs above do.
    pub async fn assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() -> App) {
        let definition = manifest().definition;
        let _app = A::default();
        let skip = [
            "undo",
            "redo",
            "commitCheckpoint",
            "createAlternative",
            "switchAlternative",
            "checkoutCheckpoint",
            "copy",
            "cut",
            "paste",
            "revertToCommand",
            "setHistoryCommandFilter",
            "noteShellCommand",
            "recordTutorial",
            "startIntroduction",
            "startTutorial",
            "setActiveUtility",
            "setActiveTool",
            semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
            semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
            "interactionSelect",
            "interactionHover",
            "clearSelection",
            "selectAll",
            "setSelectionMode",
            "setInteractionGranularity",
        ];
        for action in definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)) {
            if skip.contains(&action.id.as_str())
                || semio_framework_plugin_job::is_tool_run_action_id(&action.id)
                || semio_framework_plugin_decl_app::is_time_travel_action_id(&action.id)
                || action.id == semio_framework_plugin_operation::CANCEL_TYPED_OPERATION_ACTION_ID
                || action.id == semio_framework::HOST_EVENT_ACTION_ID
            {
                continue;
            }
            if action.id == semio_framework_plugin_window_kits::text_kit::TextWindowKit::EDIT_ACTION_ID || matches!(action.id.as_str(), "set-cell" | "set-node") {
                continue;
            }
            let staged = declared_bridge_arguments(&action.args).unwrap_or_else(|error| panic!("action {} could not stage required arguments: {error}", action.id));
            let mut original_args=Some(staged.clone());let mut original_fault=None;
            let command = A::command_from_action(&action.id, &mut original_args, &mut original_fault).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
            assert_eq!(A::command_id(&command).await, action.id.as_str(), "command_id mismatch for action {}", action.id);
        }
    }

    /// 🏷️ The stable lower-case name of one published result lane.
    fn declared_verb_lane_name(lane: TypedOperationResultLane) -> &'static str {
        match lane {
            TypedOperationResultLane::Artifact => "artifact",
            TypedOperationResultLane::Config => "config",
            TypedOperationResultLane::Draft => "draft",
            TypedOperationResultLane::Presence => "presence",
            TypedOperationResultLane::Transient => "transient",
            TypedOperationResultLane::WindowConfig => "windowConfig",
            TypedOperationResultLane::WindowTransient => "windowTransient",
            TypedOperationResultLane::Child => "child",
            TypedOperationResultLane::Interaction => "interaction",
            TypedOperationResultLane::Effect => "effect",
            TypedOperationResultLane::Event => "event",
            TypedOperationResultLane::Ui => "ui",
            TypedOperationResultLane::Download => "download",
            TypedOperationResultLane::Terminal => "terminal",
            TypedOperationResultLane::Fault => "fault",
        }
    }

    /// #️⃣ FNV-1a over every observed part of one settled dispatch.
    fn declared_verb_fingerprint<'a>(parts: impl IntoIterator<Item = &'a [u8]>) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for part in parts {
            for byte in part.iter().chain(&[0xff]) {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x0100_0000_01b3);
            }
        }
        hash
    }

    /// 🚪️ Infrastructure refusals (the verb never reached its handler) versus a handler's own refusal.
    /// `interactive-job.app-owned-output` is the code a retained job's fault page carries when the
    /// handler's own `Fault` crossed the publication boundary, so it is the handler's refusal.
    fn declared_verb_refusal(failure:impl Into<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>)->DeclaredVerbOutcome{
        let failure=failure.into();let(code,detail)=match &failure.primary{semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Value(error)=>("interactive-job.owner-refused".to_string(),format!("{error:?}")),semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Domain(domain)=>(domain.code().to_owned(),domain.message().to_owned())};
        if(code.starts_with("interactive-job.")&&code!="interactive-job.app-owned-output")||code=="app.command.unsupported"{DeclaredVerbOutcome::Unreachable{code,detail,failure:Some(failure)}}else{DeclaredVerbOutcome::Refused{code,detail,failure:Some(failure)}}
    }

    /// 🎬️ How a fresh probe fixture boots the app's own example: its `setActiveExample` invocation (the
    /// staged declared defaults overlaid by the surface's example arguments) from the window that
    /// presents it — replayed on every fixture, so an example that lands as operations, as owned
    /// children or as a host `LoadDocument` boots the same way the shell boots it.
    pub struct DeclaredVerbBoot {
        pub args: semio_framework::DslValue,
        pub view: semio_framework::ViewModel,
    }

    /// 🎬️ The arguments the shell's example picker boots with: the staged declared defaults, and for every
    /// closed choice still without a value its first option — the example the navbar opens first.
    fn declared_verb_boot_args(action: &semio_framework::ActionDefinition, staged: semio_framework::DslValue) -> semio_framework::DslValue {
        use semio_framework::{ArgSchema, DslValue};
        let DslValue::Object(mut entries) = staged else { return staged };
        for argument in &action.args {
            let ArgSchema::String { options, .. } = &argument.schema else { continue };
            let Some(first) = options.first() else { continue };
            if !entries.iter().any(|(key, value)| key == &argument.id && !matches!(value, DslValue::Null) && value.as_str() != Some("")) {
                entries.retain(|(key, _)| key != &argument.id);
                entries.push((argument.id.clone(), DslValue::String(first.value.clone())));
            }
        }
        DslValue::Object(entries)
    }

    /// 🧫️ The per-probe fixture: the registered app, bound as instance 1, booted with the app's own
    /// example when it declares one.
    async fn declared_verb_fixture_app<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> VcsArtifactApp<A, M>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let mut app = VcsArtifactApp::<A, M>::with_registry(A::default(), AppActionRegistry::from_definition(definition), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        app.bind_instance_id(meta("local").instance_id).await;
        settle_store_constructor(&mut app, mounted_policy);
        if let Some(boot) = boot {
            let action_meta = ActionMeta { view_state: Some(boot.view.clone()), ..meta("local") };
            let result = dispatch_fixture_action(&mut app,"setActiveExample",Some(boot.args.clone()),&action_meta,identity,cx,mounted_policy.close).await.unwrap_or_else(|fault| panic!("the app's own boot example must be admitted: {fault:?}"));
            let receipt = settle_registered_typed_operation(&mut app, action_meta.instance_id, mounted_policy, identity,cx).await.unwrap_or_else(|fault| panic!("the app's own boot example must settle: {fault:?}"));
            for effect in result.requested_effects.into_iter().chain(receipt.effects) {
                if let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect {
                    load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, identity,cx).await.unwrap_or_else(|fault| panic!("the app's own boot example must load: {fault:?}"));
                }
            }
            app.child_composition.follow_derivable_children(&mut app.stores, identity).await.unwrap_or_else(|fault| panic!("the boot example's derivable children must follow its coordinates: {fault:?}"));
        }
        app
    }

    /// 📦️ The document (with every owned child's envelope) and config packs the stores hold right now.
    async fn declared_verb_state<A, M>(app: &mut VcsArtifactApp<A, M>) -> (Vec<u8>, Vec<u8>)
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let mut document = app.snapshot().map(|snapshot| pack::snapshot::ArtifactPack::encode_pack(&snapshot)).unwrap_or_default();
        for child in PluginApp::child_packs(app).await.unwrap_or_default() {
            document.extend_from_slice(child.slot.as_bytes());
            document.extend_from_slice(child.child_id.as_bytes());
            document.extend_from_slice(&child.envelope_pack);
        }
        let config = match app.render_ctx().refresh_cache().await {
            Ok(()) => app.history.cache.as_ref().map(|(_, _, config, _)| pack::snapshot::ArtifactPack::encode_pack(config.as_ref())).unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        (document, config)
    }

    /// 🪆️ `slot/child_id` of every composed child the parent names right now that no child store holds.
    async fn declared_verb_unheld_children<A, M>(app: &VcsArtifactApp<A, M>) -> Vec<String>
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let Ok(snapshot) = app.snapshot() else { return Vec::new() };
        let declared: Vec<(String, String)> = match semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot) {
            Ok(projection) => (0..projection.len()).filter_map(|index| projection.get(index)).map(|(slot, fields)| (slot.to_string(), fields.child_id.to_string())).collect(),
            Err(error) => vec![(String::from("projection"), error.to_string())],
        };
        let mut unheld = Vec::new();
        for (slot, child_id) in declared {
            if app.child_store(&slot, &child_id).await.is_none() {
                unheld.push(format!("{slot}/{child_id}"));
            }
        }
        unheld
    }

    /// 🎯️ Dispatches one declared verb exactly as the shell does and reads back what it did.
    async fn declared_verb_dispatch<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, verb: &str, args: &semio_framework::DslValue, view: &semio_framework::ViewModel, body_key: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> DeclaredVerbOutcome
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
        let mut original_context=original_caller.context();let cx=&mut original_context;
        let result=async{
        let mut app = declared_verb_fixture_app::<A, M>(definition, boot, mounted_policy, identity,cx).await;
        let (document_before, config_before) = declared_verb_state(&mut app).await;
        let unheld_before = declared_verb_unheld_children(&app).await;
        let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("local") };
        let receiver = action_meta.instance_id;
        let outcome = match dispatch_fixture_action(&mut app,verb,Some(args.clone()),&action_meta,identity,cx,mounted_policy.close).await {
            Err(fault) => declared_verb_refusal(fault),
            Ok(result) => match settle_registered_typed_operation(&mut app, receiver, mounted_policy, identity,cx).await {
                Err(fault) => declared_verb_refusal(fault),
                Ok(receipt) => {
                    let effects = result.requested_effects.iter().chain(receipt.effects.iter()).map(|effect| format!("{effect:?}")).collect::<Vec<_>>();
                    let events = result.events.iter().chain(receipt.events.iter()).map(|event| format!("{event:?}")).collect::<Vec<_>>();
                    let replaced = result.requested_effects.iter().chain(receipt.effects.iter()).any(|effect| matches!(effect, semio_framework::kernel::Effect::LoadDocument { .. }));
                    let downloaded = result
                        .requested_effects
                        .iter()
                        .chain(receipt.effects.iter())
                        .any(|effect| matches!(effect, semio_framework::kernel::Effect::DownloadMediaExport { .. } | semio_framework::kernel::Effect::IconRenderExport { .. } | semio_framework::kernel::Effect::VideoRenderExport { .. }));
                    let lanes = receipt.lanes.iter().map(|lane| declared_verb_lane_name(*lane)).collect::<std::collections::BTreeSet<_>>();
                    let (document_after, config_after) = declared_verb_state(&mut app).await;
                    let orphaned_children = declared_verb_unheld_children(&app).await.into_iter().filter(|child| !unheld_before.contains(child)).collect();
                    let rendered = match render_registered_fixture_app(&mut app,body_key,view,cx).await {
                        Ok(projected) => projected,
                        Err(failure)=>{
                            close_registered_fixture_app(&mut app,mounted_policy,cx).expect("original failed render fixture closes under its existing caller");
                            return declared_verb_refusal(failure);
                        }
                    };
                    let lane_text = lanes.iter().copied().collect::<Vec<_>>().join(",");
                    let fingerprint = declared_verb_fingerprint(
                        [document_after.as_slice(), config_after.as_slice(), lane_text.as_bytes(), rendered.as_bytes()].into_iter().chain(effects.iter().map(|effect| effect.as_bytes())).chain(events.iter().map(|event| event.as_bytes())),
                    );
                    DeclaredVerbOutcome::Settled(DeclaredVerbEffect {
                        user_path_written: downloaded || lanes.contains("download"),
                        lanes,
                        document_changed: document_after != document_before,
                        document_replaced: replaced,
                        config_changed: config_after != config_before,
                        host_effects: effects.len() + events.len(),
                        fingerprint,
                        orphaned_children,
                    })
                }
            },
        };
        close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
        outcome
    
        }.await;
        let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
        result
    }

    /// 🎬️ Projects the same stationary render source without donating its native resources to the fixture.
    #[doc(hidden)]
pub async fn render_registered_fixture_app<A,M>(app:&mut VcsArtifactApp<A,M>,body_key:&str,view:&semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<String,semio_framework_job::OriginalTurnFailure>
    where A:ArtifactApp+Default,M:store::SpaceMember+store::MemberFactory+Send+'static{
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            if cx.should_yield(){break}
            if app.render(body_key,None,view,cx).await?.is_some(){
                let source=app.original_render_tree_source(cx)?;
                let tree=source.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original fixture render readiness lost its stationary tree"))?;
                let mut remaining=FIXTURE_TREE_MAX_NODES;
                return project_fixture_node(&tree.root,&mut remaining,0).and_then(|value|serde_json::to_string(&value).map_err(|_|"projection-json")).map_err(|source|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,source).into())
            }
        }
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original registered fixture render remained pending under its authored caller").into())
    }

    /// 🫴️ Deferred addressed fixture dispatch keeps its actual input slots and initiating context over every step.
    pub async fn dispatch_addressed_fixture_on_original<PA:semio_framework_plugin_reactor_app::PluginApp>(app:&mut PA,address:&semio_framework::manifest::ActionAddress,args:&mut Option<semio_framework_value::DslValue>,fault:&mut Option<semio_framework::Fault>,active_mode:Option<&str>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework::InvocationResult,semio_framework_job::OriginalTurnFailure>{
        let maximum_turns=OriginalFixtureCaller::maximum_turns();
        for _ in 0..maximum_turns{
            if cx.should_yield(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original addressed fixture exhausted its initiating work").into())}
            if let Some(result)=app.handle_action_invocation(address,args,fault,active_mode,meta,identity,cx).await?{return Ok(result)}
            let grant=cx.retained_grant();app.maintenance_step(cx,grant)?;
        }
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original addressed fixture reached its finite step limit").into())
    }

    /// 🪜️ Keeps one admitted preview source under its initiating fixture context through pending phases.
    async fn preview_fixture_on_original<A,M>(app:&mut VcsArtifactApp<A,M>,invocation:&semio_framework::manifest::ActionInvocation,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework::InvocationResult,semio_framework_job::OriginalTurnFailure>
    where A:ArtifactApp,M:store::SpaceMember+store::MemberFactory+Send+'static{
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            if cx.should_yield(){break}
            if let Some(result)=app.command_ctx().preview_addressed_action(invocation,meta,cx).await?{return Ok(result)}
        }
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture preview did not settle under its supplied context").into())
    }

    /// 🤖️ Previews `verb` with `args` exactly as the MCP gateway's `action_prepare` does — the agent lane's typed command
    /// frame, addressed to the caller's declared window instance with explicit language and terminology — and answers how many document ops
    /// the preview would commit (ticket 26/09/23, G12 session 14c).
    pub async fn agent_preview<A, M>(app: &mut VcsArtifactApp<A, M>, definition: &semio_framework::AppDefinition, verb: &str, args: &semio_framework::DslValue, view: &semio_framework::ViewModel, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<usize, semio_framework_job::OriginalTurnFailure>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let window_kind_id = definition.window_kinds.iter().find(|window| semio_framework::window_kind_actions(definition, window).iter().any(|action| action.id == verb)).map_or_else(|| "*".to_string(), |window| window.id.clone());
        let window_instance_id = view.window_instances.iter().find(|window| window.window_kind_id == window_kind_id).ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("fixture.agent-window"), "agent fixture requires its caller's declared window instance"))?.id.clone();
        let arguments = match args {
            semio_framework::DslValue::Object(entries) => entries.iter().cloned().collect(),
            _ => std::collections::BTreeMap::new(),
        };
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: String::new(),
                app_id: app.app.instance_id().await.to_string(),
                mode_id: definition.default_mode_id.clone(),
                window_kind_id,
                window_instance_id,
                action_id: verb.to_string(),
            },
            arguments,
        };
        let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("agent") };
        let result = preview_fixture_on_original(app,&invocation,&action_meta,cx).await?;
        Ok(result.output.get("documentOps").and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0))
    }

    /// 🤖️ Dispatches one declared verb exactly as an agent does — the owner-qualified invocation the
    /// MCP gateway sends as a typed command frame (`dispatch_command_frame`, its `action_prepare`
    /// phase) — and reads back what that phase reports it would do.
    async fn declared_verb_agent_dispatch<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, verb: &str, args: &semio_framework::DslValue, window_kind_id: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> DeclaredVerbOutcome
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
        let mut original_context=original_caller.context();let cx=&mut original_context;
        let result=async{
        let mut app = declared_verb_fixture_app::<A, M>(definition, boot, mounted_policy, identity,cx).await;
        let app_id = app.app.instance_id().await.to_string();
        let arguments = match args {
            semio_framework::DslValue::Object(entries) => entries.iter().cloned().collect(),
            _ => std::collections::BTreeMap::new(),
        };
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: String::new(),
                app_id,
                mode_id: definition.default_mode_id.clone(),
                window_kind_id: window_kind_id.to_string(),
                window_instance_id: "declared-verb-agent".to_string(),
                action_id: verb.to_string(),
            },
            arguments,
        };
        let view = semio_framework::ViewModel { active_mode_id: Some(definition.default_mode_id.clone()), active_window_kind_id: Some(window_kind_id.to_string()), window_id: Some("declared-verb-agent".to_string()), window_instances: vec![semio_framework::ViewWindowInstance { id: "declared-verb-agent".to_string(), window_kind_id: window_kind_id.to_string() }], ..semio_framework::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let action_meta = ActionMeta { view_state: Some(view), ..meta("agent") };
        let outcome = match preview_fixture_on_original(&mut app,&invocation,&action_meta,cx).await {
            Err(fault) => declared_verb_refusal(fault),
            Ok(result) => {
                let count = |key: &str| result.output.get(key).and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
                let (document, config, draft, children) = (count("documentOps"), count("configOps"), count("draftOps"), count("childGroups"));
                let lanes = [("artifact", document), ("config", config), ("draft", draft), ("child", children)].into_iter().filter(|(_, ops)| *ops != 0).map(|(lane, _)| lane).collect::<std::collections::BTreeSet<_>>();
                let host_effects = result.requested_effects.len() + result.events.len();
                let fingerprint = declared_verb_fingerprint([format!("{:?}", result.output).as_bytes()]);
                DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint, orphaned_children: Vec::new() })
            }
        };
        close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
        outcome
    
        }.await;
        let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
        result
    }

    /// 🧪️ Probes every verb `definition` declares, on the app's own boot example: each window kind's
    /// resolved actions (framework-reserved, tool-run and host-owned utility/tool verbs excluded) are
    /// dispatched from an instance of that window kind with the staged declared defaults — overlaid by
    /// the surface's example arguments, `examples` = `{"verbs": {"<verb>": {"<arg>": <value>}}}`, the
    /// language-agnostic `🧫️fixtures/⚖️declared-verb-examples.json` of a surface whose verbs need
    /// document ids to act — then with two distinct values per perturbable declared argument. Every
    /// probe runs on a fresh registered app.
    pub async fn probe_declared_verbs<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        probe_declared_verbs_where::<A, M>(definition, examples, None, mounted_policy, identity).await
    }

    /// 🤖️ The agent lane beside the shell lane for exactly the named `verbs`, on the app's own boot example: each is
    /// dispatched with its staged declared defaults (overlaid by `examples`) from every window kind that presents it,
    /// and once through the agent lane at the address the capability catalog publishes — without argument
    /// perturbation, so a surface whose boot is expensive pins what an agent gets for the verbs it names.
    /// [`declared_verb_agent_divergences`] over the answer lists the named verbs whose two lanes disagree.
    pub async fn probe_agent_lane<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, verbs: &[&str], mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let probes = probe_declared_verbs_where::<A, M>(definition, examples, Some(verbs), mounted_policy, identity).await;
        let missing = verbs.iter().filter(|verb| !probes.iter().any(|probe| probe.verb == **verb)).collect::<Vec<_>>();
        assert!(missing.is_empty(), "the surface declares no verb {missing:?}");
        probes
    }

    /// 🧪️ [`probe_declared_verbs`] over every declared verb, or — `only` — the agent-lane probe over the named verbs.
    async fn probe_declared_verbs_where<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, only: Option<&[&str]>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        use semio_framework::{effective_action_args, resolve_audience, window_kind_actions, DslValue};
        let definition = definition();
        let instances = definition.window_kinds.iter().enumerate().map(|(index, window)| semio_framework::ViewWindowInstance { id: format!("declared-verb-probe-{index}"), window_kind_id: window.id.clone() }).collect::<Vec<_>>();
        let view_of = |index: usize| semio_framework::ViewModel {
            active_mode_id: Some(definition.default_mode_id.clone()),
            active_window_kind_id: Some(definition.window_kinds[index].id.clone()),
            window_id: Some(instances[index].id.clone()),
            window_instances: instances.clone(),
            ..semio_framework::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let examples: serde_json::Value = examples.map(|text| serde_json::from_str(text).expect("declared-verb examples are JSON")).unwrap_or(serde_json::Value::Null);
        let staged_of = |action: &semio_framework::ActionDefinition| {
            let example = examples["verbs"].get(&action.id).map(DslValue::from).unwrap_or(DslValue::Object(Vec::new()));
            effective_action_args(&action.args, &example, None)
        };
        let boot = definition.window_kinds.iter().enumerate().find_map(|(index, window)| {
            window_kind_actions(&definition, window).into_iter().find(|action| action.id == "setActiveExample").map(|action| DeclaredVerbBoot { args: declared_verb_boot_args(action, staged_of(action)), view: view_of(index) })
        });
        let mut order: Vec<(semio_framework::ActionDefinition, Vec<usize>)> = Vec::new();
        for (index, window) in definition.window_kinds.iter().enumerate() {
            for action in window_kind_actions(&definition, window) {
                if semio_framework_plugin_job::reserved_jobs::is_framework_reserved_action_id(&action.id) || matches!(action.id.as_str(), semio_framework::SET_ACTIVE_TOOL_ACTION_ID | semio_framework::SET_ACTIVE_UTILITY_ACTION_ID) {
                    continue;
                }
                match order.iter_mut().find(|(known, _)| known.id == action.id) {
                    Some((_, windows)) => windows.push(index),
                    None => order.push((action.clone(), vec![index])),
                }
            }
        }
        let mut probes = Vec::new();
        for (action, windows) in &order {
            if only.is_some_and(|only| !only.contains(&action.id.as_str())) {
                continue;
            }
            let boot = if action.id == "setActiveExample" { None } else { boot.as_ref() };
            let audience = resolve_audience(action);
            let staged = staged_of(action);
            let mut original_args=Some(staged.clone());let mut original_fault=None;
            let bridge = A::command_from_action(&action.id, &mut original_args, &mut original_fault).await.map(drop).map_err(|fault| format!("{}: {}", fault.code.0, fault.message));
            let mut window_probes = Vec::new();
            for index in windows {
                let view = view_of(*index);
                let window = &definition.window_kinds[*index];
                let staged_outcome = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &staged, &view, &window.body_key, mounted_policy, identity).await;
                let mut arguments = Vec::new();
                if only.is_none() && audience != semio_framework::CapabilityAudience::Input && !matches!(staged_outcome, DeclaredVerbOutcome::Unreachable { .. }) {
                    for argument in &action.args {
                        let Some((first, second)) = declared_argument_alternatives(argument) else { continue };
                        let with = |value: DslValue| {
                            let mut args = staged.clone();
                            if let DslValue::Object(entries) = &mut args {
                                entries.retain(|(key, _)| key != &argument.id);
                                entries.push((argument.id.clone(), value));
                            }
                            args
                        };
                        let others_specified = action.args.iter().filter(|other| other.id != argument.id).all(|other| staged.get(&other.id).is_some_and(|value| !matches!(value, DslValue::Null)));
                        let first = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &with(first), &view, &window.body_key, mounted_policy, identity).await;
                        let second = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &with(second), &view, &window.body_key, mounted_policy, identity).await;
                        arguments.push(DeclaredArgumentProbe { argument: argument.id.clone(), others_specified, first, second });
                    }
                }
                window_probes.push(DeclaredVerbWindowProbe { window: window.id.clone(), staged: staged_outcome, arguments });
            }
            let agent = match audience {
                semio_framework::CapabilityAudience::Agent => {
                    let claimed = definition.window_kinds.iter().find(|window| window.actions.iter().any(|claimed| claimed.id == action.id)).map_or("*", |window| window.id.as_str());
                    Some(declared_verb_agent_dispatch::<A, M>(&definition, boot, &action.id, &staged, claimed, mounted_policy, identity).await)
                }
                _ => None,
            };
            probes.push(DeclaredVerbProbe { verb: action.id.clone(), kind: action.kind, audience, destructive: action.semantics.effects.destructive, bridge, windows: window_probes, agent });
        }
        probes
    }

    /// ⚖️ Every verb `definition` declares honours its declaration ([`declared_verb_findings`] over
    /// [`probe_declared_verbs`], with the surface's optional example arguments). Answers the probes, so a
    /// caller pins their count and the outcome of the verbs its examples make act — a surface that stops
    /// declaring its verbs or whose examples stop acting cannot turn the law vacuous.
    pub async fn assert_declared_verbs_honour_their_declarations<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
    {
        let probes = probe_declared_verbs::<A, M>(definition, examples, mounted_policy, identity).await;
        let findings = probes.iter().flat_map(declared_verb_findings).filter(|finding| !matches!(finding, DeclaredVerbFinding::AgentLaneDiverges { .. })).map(|finding| finding.to_string()).collect::<Vec<_>>();
        assert!(findings.is_empty(), "{} finding(s) over {} declared verb(s):\n{}", findings.len(), probes.len(), findings.join("\n"));
        probes
    }



    /// 🧪️ `command_a`/`command_b` are applied to two `paired_apps` instances; each side then folds the
    /// other's events, one side commits a checkpoint (a structural history event) and the other folds
    /// it too — `probe` must agree after every exchange. The repeated two-instance-convergence test body
    /// (see `playbook-plugin`'s `two_instances_converge_disjoint_edits_via_backbone` for the original,
    /// app-specific version).
    pub async fn assert_two_instances_converge<A, P>(channel: &str, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        P: PartialEq + std::fmt::Debug,
    {
        let mut original_caller=OriginalFixtureCaller::admit().unwrap();let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let (mut instance_a, mut instance_b) = paired_apps::<A>(channel, mounted_policy, identity).await;
        let genesis = probe(&instance_a);
        instance_a.dispatch_typed(command_a, &meta("actor-a"), identity,cx).await.expect("a applies its edit");
        instance_b.dispatch_typed(command_b, &meta("actor-b"), identity,cx).await.expect("b applies its edit");
        exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
        assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
    
        close_registered_fixture_app(&mut instance_a,mounted_policy,cx).expect("original first app close");close_registered_fixture_app(&mut instance_b,mounted_policy,cx).expect("original second app close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();result
    }

    /// 🧬️ Registered twin of [`assert_two_instances_converge`]: every typed command is settled until its
    /// retained publication retires, so both edits really land before the replicas exchange events.
    /// The `NoMembers` spelling of [`assert_two_registered_instances_converge_with_members`].
    pub async fn assert_two_registered_instances_converge<A, P, Manifest, Build>(channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        P: PartialEq + std::fmt::Debug,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        assert_two_registered_instances_converge_with_members::<A, store::NoMembers, P, Manifest, Build>(channel, manifest, command_a, command_b, probe, mounted_policy, identity).await;
    }

    /// 🧬️ [`assert_two_registered_instances_converge`] over an explicit member roster `M` — see
    /// [`new_registered_app_with_members`] for why a composing app needs it.
    pub async fn assert_two_registered_instances_converge_with_members<A, M, P, Manifest, Build>(channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        P: PartialEq + std::fmt::Debug,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
        let mut original_context=original_caller.context();let cx=&mut original_context;
        let result=async{
        let (mut instance_a, mut instance_b) = paired_registered_apps_with_members::<A, M, Manifest, Build>(channel, manifest, mounted_policy, identity).await;
        let genesis = probe(&instance_a);
        let receiver = meta("actor-a").instance_id;
        instance_a.dispatch_typed(command_a, &meta("actor-a"), identity,cx).await.expect("a applies its edit");
        settle_registered_typed_operation(&mut instance_a, receiver, mounted_policy, identity,cx).await.expect("a's edit publishes");
        instance_b.dispatch_typed(command_b, &meta("actor-b"), identity,cx).await.expect("b applies its edit");
        settle_registered_typed_operation(&mut instance_b, receiver, mounted_policy, identity,cx).await.expect("b's edit publishes");
        exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
        assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
        instance_a.detach_backbone().await.expect("a releases its backbone");
        instance_b.detach_backbone().await.expect("b releases its backbone");
        close_registered_fixture_app(&mut instance_a, mounted_policy,cx).expect("registered fixture original close");
        close_registered_fixture_app(&mut instance_b, mounted_policy,cx).expect("registered fixture original close");
    
        }.await;
        let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
        result
    }

    /// 🔀️ Both replicas fold each other's events, `a` commits a checkpoint over everything it holds, `b`
    /// folds that commit — `probe` must agree after each exchange: edits and structural history alike
    /// converge by projection of the shared event log.
    async fn exchange_and_assert_convergence<A, M, P>(instance_a: &mut VcsArtifactApp<A, M>, instance_b: &mut VcsArtifactApp<A, M>, actor_a: &str, probe: &impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
    where
        A: ArtifactApp,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        P: PartialEq + std::fmt::Debug,
    {
        instance_a.tick_backbone(identity,cx).await.expect("a folds b's events");
        instance_b.tick_backbone(identity,cx).await.expect("b folds a's events");
        assert_eq!(probe(instance_a), probe(instance_b), "both instances must converge on the same snapshot");
        let receiver = meta(actor_a).instance_id;
        let admitted = dispatch_fixture_action(instance_a,"commitCheckpoint",None,&meta(actor_a),identity,cx,mounted_policy.close).await.expect("a commits a checkpoint");
        semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(instance_a, admitted).await.expect("a's checkpoint commit settles");
        settle_registered_typed_operation(instance_a, receiver, mounted_policy, identity,cx).await.expect("a's checkpoint publication settles");
        instance_b.tick_backbone(identity,cx).await.expect("b folds a's checkpoint");
        assert_eq!(probe(instance_a), probe(instance_b), "a replicated checkpoint keeps both instances converged");
    }

    // 🪦️ `assert_graph_merge_preserves_referential_integrity` DELETED
    // (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` M1): zero callers
    // repo-wide (confirmed via grep) — a generic testkit helper offered for app tests to probe
    // `(snapshot, Vec<..>)`-of-free-form-diagnostics-shaped reconcile results, but no app's own test suite ever
    // called it; the one real exercise of this exact scenario is
    // `🖥️host/🦀️.rs`'s `concurrent_delete_and_wire_reconciles_without_a_dangling_edge`,
    // written directly against `OsWorkflowStore`/`reconcile_workflow_snapshot` rather than through
    // this helper. Dead code deleted rather than migrated to `Vec<protocol::MutationMessage>`.

    /// 🧪️ Applies `command` on a sender attached to a backbone, replays the resulting envelopes onto a
    /// fresh receiver twice, and asserts `probe` sees the same result both times — feeding the same operation
    /// twice must not double-apply.
    pub async fn assert_ingest_idempotent<A, P>(command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        P: PartialEq + std::fmt::Debug,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original ingestion fixture caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mut sender = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
        sender.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        receiver.ingest_operations(&operations, identity,cx).await.expect("ingest once");
        let once = probe(&receiver);
        receiver.ingest_operations(&operations, identity,cx).await.expect("ingest twice");
        assert_eq!(probe(&receiver), once, "feeding the same operation twice must not double-apply");
        sender.detach_backbone().await.expect("original sender detaches");close_registered_fixture_app(&mut sender,mounted_policy,cx).expect("original sender closes");close_registered_fixture_app(&mut receiver,mounted_policy,cx).expect("original receiver closes");
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingestion caller closes");result
    }

    /// 🧬️ Registered twin of [`assert_ingest_idempotent`] — the `NoMembers` spelling of
    /// [`assert_registered_ingest_idempotent_with_members`].
    pub async fn assert_registered_ingest_idempotent<A, P, Manifest, Build>(manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        P: PartialEq + std::fmt::Debug,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        assert_registered_ingest_idempotent_with_members::<A, store::NoMembers, P, Manifest, Build>(manifest, command, probe, mounted_policy, identity).await;
    }

    /// 🧬️ [`assert_registered_ingest_idempotent`] over an explicit member roster `M` — see
    /// [`new_registered_app_with_members`] for why a composing app needs it.
    pub async fn assert_registered_ingest_idempotent_with_members<A, M, P, Manifest, Build>(manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        A: ArtifactApp + Default,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        P: PartialEq + std::fmt::Debug,
        Manifest: std::future::Future<Output = App>,
        Build: Fn() -> Manifest,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
        let mut original_context=original_caller.context();let cx=&mut original_context;
        let result=async{
        let mut sender = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        sender.bind_instance_id(meta("local").instance_id).await;
        let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
        sender.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");
        settle_registered_typed_operation(&mut sender, meta("local").instance_id, mounted_policy, identity,cx).await.expect("the edit publishes");
        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        assert!(!envelopes.is_empty(), "the applied edit must reach the channel as events");
        let operations = protocol::encode_envelopes(&envelopes);
        let mut receiver = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        receiver.bind_instance_id(meta("local").instance_id).await;
        let genesis = probe(&receiver);
        receiver.ingest_operations(&operations, identity,cx).await.expect("ingest once");
        let once = probe(&receiver);
        assert_ne!(once, genesis, "ingesting the events must materialize the edit");
        receiver.ingest_operations(&operations, identity,cx).await.expect("ingest twice");
        assert_eq!(probe(&receiver), once, "feeding the same operation twice must not double-apply");
        sender.detach_backbone().await.expect("sender releases its backbone");
        close_registered_fixture_app(&mut sender, mounted_policy,cx).expect("registered fixture original close");
        close_registered_fixture_app(&mut receiver, mounted_policy,cx).expect("registered fixture original close");
    
        }.await;
        let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
        result
    }

    /// 🔀️ Dispatches `command` and asserts it produced a `TransactionProposalDraft` instead of
    /// applying anything (contract §5.1) — the guest-side half of "propose instead of apply".
    /// Returns the draft so a test can inspect `local_ops`/`foreign`.
    pub async fn assert_proposes_transaction<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, command: A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin_job::TransactionProposalDraft {
        let generation_before = app.stores.store.generation();
        let edits_before = app.stores.store.envelope().vcs.edits.len();
        app.dispatch_typed(command, &meta("local"), identity,cx).await.expect("dispatch a command whose mutations carry foreign steps");
        settle_registered_typed_operation(app, meta("local").instance_id, mounted_policy, identity,cx).await.expect("a proposing dispatch settles its own operation");
        assert_eq!(app.stores.store.generation(), generation_before, "a proposed transaction must not bump the document store's generation");
        assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before, "a proposed transaction must not add an Edit");
        app.take_pending_transaction_proposal().await.expect("dispatch_emit must stash a TransactionProposalDraft when foreign steps are present")
    }

    /// 🔀️ Scripts a whole pre-planned-form transaction against ONE in-process member without a
    /// host: prepares `ops` under `txn_id`/`origin`, asserts the prepare was accepted,
    /// commits, and asserts exactly one resulting `Edit` whose `group_id` is `txn_id` and whose
    /// every `MutationMeta.origin` is `origin`. Returns the committed edit id.
    pub async fn assert_transaction_commits_as_one_edit<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, txn_id: &str, ops: Vec<A::Mutation>, origin: protocol::MutationOrigin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> String {
        let edits_before = app.stores.store.envelope().vcs.edits.len();
        let mut prepared_ops: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
        for op in ops.iter() {
            prepared_ops.push(::protocol::OpBinary::encode_op(op).expect("encode prepared op"));
        }
        let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], Some(origin.clone())).await;
        assert!(outcome.rejection.is_none(), "prepare unexpectedly rejected: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
        let edit_id = app.transaction_commit(txn_id, &meta("local"), identity).await.expect("commit prepared transaction");
        assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before + 1, "commit must produce exactly one new Edit");
        let edit = app.stores.store.envelope().vcs.edits.last().expect("committed edit must exist");
        assert_eq!(edit.id, edit_id, "transaction_commit must return the tail edit's own id");
        assert!(!edit.mutation_meta.is_empty(), "the committed edit must carry at least one MutationMeta entry");
        for op_meta in &edit.mutation_meta {
            assert_eq!(op_meta.group_id.as_deref(), Some(txn_id), "every op in the committed edit must carry the transaction's group_id");
            assert_eq!(op_meta.origin, origin, "every op in the committed edit must carry the prepared origin");
        }
        edit_id
    }

    /// 🔀️ Prepares then rolls back a transaction, asserting the store's generation and edit
    /// count are completely untouched (contract §5.5: rollback discards pending state without
    /// ever having applied anything).
    pub async fn assert_transaction_rollback_leaves_state_untouched<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, txn_id: &str, ops: Vec<A::Mutation>) {
        let generation_before = app.stores.store.generation();
        let edits_before = app.stores.store.envelope().vcs.edits.len();
        let mut prepared_ops: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
        for op in ops.iter() {
            prepared_ops.push(::protocol::OpBinary::encode_op(op).expect("encode prepared op"));
        }
        let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "prepare unexpectedly rejected");
        app.transaction_rollback(txn_id).await.expect("rollback the pending transaction");
        assert_eq!(app.stores.store.generation(), generation_before, "rollback must not change the store generation");
        assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before, "rollback must not add any edit");
    }


    // 🎫️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — lane 0-F. New
    // subregion, appended after the peer `🧫️fixtures` subregion above without touching it.
    use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer, ViewerApp};

    #[doc(hidden)]
pub struct BoundedViewerFixture<V: ArtifactViewer>(std::marker::PhantomData<V>);

    impl<V: ArtifactViewer> Default for BoundedViewerFixture<V> {
        fn default() -> Self {
            Self(std::marker::PhantomData)
        }
    }

    impl<V> ArtifactViewer for BoundedViewerFixture<V>
    where
        V: ArtifactViewer<Presence = semio_framework_plugin_decl_views::NoPresence, PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation, Transient = semio_framework_plugin_decl_views::NoTransient, TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation>,
    {
        fn document_store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{V::document_store_owners_source_demands()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
            V::build_document_store_owners(grant).or_else(|| Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Snapshot, Self::Mutation>(grant)))
        }

        fn config_store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{V::config_store_owners_source_demands()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
            V::build_config_store_owners(grant).or_else(|| Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Config, Self::ConfigMutation>(grant)))
        }

        fn build_document_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
            V::build_document_store_disposer().or_else(|| Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>()))
        }

        fn build_config_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
            V::build_config_store_disposer().or_else(|| Some(semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>()))
        }

        fn build_presence_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
            V::build_presence_store_disposer().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_store_disposer()))
        }

        fn build_transient_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
            V::build_transient_store_disposer().or_else(|| Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_store_disposer()))
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            V::build_presence_peer_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_peer_retirement_factory()))
        }

        const ROLE: semio_framework::AppRole = V::ROLE;
        const DIALECT: semio_framework_artifact_reference::Dialect = V::DIALECT;
        const DOCUMENT_SCHEMA: &'static str = V::DOCUMENT_SCHEMA;
        /// 🧩️ The viewer's own roster: the fixture forwards `genesis_child_pack`, and a derived
        /// child's dialect must resolve through the roster that declares it, or construction
        /// panics at `seed_genesis_children` (energy/gis viewers derive `s.stdio.semio` members).
        type Members = V::Members;
        type Snapshot = V::Snapshot;
        type Mutation = V::Mutation;
        type Config = V::Config;
        type ConfigMutation = V::ConfigMutation;
        type Presence = semio_framework_plugin_decl_views::NoPresence;
        type PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation;
        type Transient = semio_framework_plugin_decl_views::NoTransient;
        type TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation;
        type Command = V::Command;

        fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework::Fault> {
            V::child_restore_projection(snapshot)
        }
        fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
            V::genesis_child_pack(snapshot, slot, child_id)
        }

        fn mounted_job_maintenance_phase(instance_id: u32) -> Option<semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1> { V::mounted_job_maintenance_phase(instance_id) }
        fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { V::mounted_job_maintenance_demands(instance_id, body) }
        fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { V::mounted_job_close_demands(instance_id, body) }
        fn mounted_job_maintenance_step(instance_id: u32, cx: &mut semio_framework_job::StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            V::mounted_job_maintenance_step(instance_id, cx, grant)
        }

        fn mounted_job_close_step(instance_id: u32, cx: &mut semio_framework_job::StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            V::mounted_job_close_step(instance_id, cx, grant)
        }

        fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
            V::mounted_jobs_terminal_is_empty(instance_id)
        }

        fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin_decl_views::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
            V::mounted_job_prepare_snapshot_read(operation, snapshot)
        }

        fn register_window_transient_owners(registry: &mut semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry) -> Result<(), semio_framework::Fault> {
            V::register_window_transient_owners(registry)
        }

        fn register_window_config_owners(registry: &mut semio_framework_plugin_window_config::WindowConfigOwnerRegistry) -> Result<(), semio_framework::Fault> {
            V::register_window_config_owners(registry)
        }

        fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
            V::build_config_store_one_item_preparation_factory()
        }

        fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
            V::build_presence_store_one_item_preparation_factory()
        }

        fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
            V::build_transient_store_one_item_preparation_factory()
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            V::build_presence_local_root_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_local_root_retirement_factory()))
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            V::build_transient_local_root_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_local_root_retirement_factory()))
        }

        fn ephemeral(
            command: &Self::Command,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            presence: &semio_framework_plugin_decl_views::PresenceView<'_, Self::Presence>,
            transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
        ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
            V::ephemeral(command, doc, cfg, presence, transient)
        }

        fn config_schema() -> &'static str {
            V::config_schema()
        }

        fn initial_snapshot() -> Self::Snapshot {
            V::initial_snapshot()
        }

        fn initial_config() -> Self::Config {
            V::initial_config()
        }

        fn handle(
            command: &Self::Command,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
            view_state: Option<&semio_framework::ViewModel>,
            engines: &semio_framework_2d_compute::EngineHandles,
        ) -> Result<semio_framework_plugin_decl_emission::surface_contract::ViewEmit<Self::ConfigMutation>, semio_framework::Fault> {
            V::handle(command, doc, cfg, interaction, view_state, engines)
        }

        fn command_id(command: &Self::Command) -> &'static str {
            V::command_id(command)
        }

        fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, semio_framework::Fault> {
            V::command_from_action(action, args, original_fault)
        }

        fn config_spec() -> semio_framework::ConfigSpec {
            V::config_spec()
        }

        fn interaction_topology(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
            V::interaction_topology(doc, cfg)
        }

        fn render(body_key: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> semio_framework_plugin_interaction::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
            V::render(body_key, doc, cfg, view_state)
        }

        fn render_with_request_context(
            owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle,
            body_key: &str,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            view_state: &semio_framework::ViewModel,
            transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
            interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
            cx:&mut semio_framework_job::StepContext<'_>,
        ) -> semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<semio_framework_ui_runtime::ComponentTree> {
            V::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
        }

        fn pending_effects(owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view: Option<&semio_framework::ViewModel>) -> Vec<semio_framework::kernel::Effect> {
            V::pending_effects(owner, doc, cfg, view)
        }

        fn window_engagements(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, semio_framework_ui_layout::layout::WindowEngagement> {
            V::window_engagements(doc, cfg, view_state)
        }

        fn window_engagements_with_request_context(
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            view_state: &semio_framework::ViewModel,
            transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
            interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        ) -> std::collections::HashMap<String, semio_framework_ui_layout::layout::WindowEngagement> {
            V::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
        }

        fn window_measures(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
            V::window_measures(doc, cfg, view_state)
        }

        fn window_measures_with_request_context(
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            view_state: &semio_framework::ViewModel,
            interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        ) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
            V::window_measures_with_request_context(doc, cfg, view_state, interaction)
        }

        fn tool_measures(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
            V::tool_measures(doc, cfg, view_state)
        }

        fn app_catalogue_json() -> String {
            V::app_catalogue_json()
        }

        fn context_menu(
            request: &semio_framework_ui_layout::layout::ContextMenuRequest,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            view_state: &semio_framework::ViewModel,
            registry: &AppActionRegistry,
        ) -> Vec<semio_framework_ui_layout::layout::ContextMenuItemSpec> {
            V::context_menu(request, doc, cfg, view_state, registry)
        }

        fn context_menu_with_request_context(
            request: &semio_framework_ui_layout::layout::ContextMenuRequest,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
            view_state: &semio_framework::ViewModel,
            interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
            registry: &AppActionRegistry,
        ) -> Vec<semio_framework_ui_layout::layout::ContextMenuItemSpec> {
            V::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
        }

        fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
            V::app_schema()
        }

        fn io() -> Option<semio_framework::AppIo> {
            V::io()
        }

        async fn media_ports() -> Vec<semio_framework::MediaPortSpec> {
            V::media_ports().await
        }

        fn export_media(port: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>) -> Result<semio_framework::Media, semio_framework::MediaError> {
            V::export_media(port, doc)
        }

        fn export_media_with_request_context(
            owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle,
            port: &str,
            doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
            transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
        ) -> Result<semio_framework::Media, semio_framework::MediaError> {
            V::export_media_with_request_context(owner, port, doc, transient)
        }

        fn media_fingerprint(port: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>) -> Result<semio_framework::MediaFingerprint, semio_framework::MediaError> {
            V::media_fingerprint(port, doc)
        }
    }

    /// 🧪️ Invokes an editor's direct command reducer with empty local interaction and history.
    pub fn reduce_editor_command<E: ArtifactEditor>(command: &E::Command, snapshot: &E::Snapshot) -> Result<semio_framework_plugin_decl_emission::Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, semio_framework::Fault> {
        let history = semio_framework_plugin_decl_views::history_view::HistoryView::empty();
        let config = E::initial_config();
        let draft = E::initial_draft();
        let state = protocol::InteractionState::default();
        let hover = semio_framework_plugin_decl_views::InteractionHoverState::new();
        let peers = semio_framework_plugin_decl_views::PeerPresenceRoot::empty();
        E::handle(
            command,
            &semio_framework_plugin_decl_views::ArtifactView::new(snapshot, &history),
            &semio_framework_plugin_decl_views::ConfigView { snapshot: &config, window: None },
            &semio_framework_plugin_decl_views::InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: semio_framework_plugin_interaction::interaction_topology::empty_draft_references() },
            None,
            &semio_framework_plugin_decl_views::DraftView { snapshot: &draft },
            &semio_framework_2d_compute::EngineHandles::empty(),
        )
    }

    /// 👁️ Exercises the production ViewerApp adapter with a representative viewer command.
    /// The adapter must emit neither document nor draft mutations. A refusal emits nothing and so
    /// satisfies the law — a retained-route viewer refuses a direct `handle` by design. Command
    /// registration and admission are separate runtime contracts; a generic viewer fixture has no
    /// declared factory for its author-defined default command.
    ///
    /// 🧩️ Over the viewer's OWN roster (`V::Members`), never `new_viewer`'s `NoMembers`: the
    /// fixture forwards `genesis_child_pack`, so a viewer deriving composed children would
    /// otherwise refuse construction at `seed_genesis_children`.
    pub async fn assert_viewer_never_mutates<V>(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
    where
        V: ArtifactViewer<Presence = semio_framework_plugin_decl_views::NoPresence, PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation, Transient = semio_framework_plugin_decl_views::NoTransient, TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation>,
        V::Command: Default,
    {
        let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
        let mut original_context=original_caller.context();let cx=&mut original_context;
        let result=async{
        use super::{ArtifactView, ConfigView, DraftView, InteractionHoverState, InteractionView, NoDraft, PeerPresenceRoot};
        let mut app: VcsArtifactApp<ViewerApp<BoundedViewerFixture<V>>, V::Members> = VcsArtifactApp::new(ViewerApp::<BoundedViewerFixture<V>>::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
        app.render_ctx().refresh_cache().await.expect("viewer fixture cache");
        let result = {
            let (_, snapshot, config, history) = app.history.cache.as_ref().expect("viewer fixture cache populated");
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            let draft = NoDraft::default();
            let draft = DraftView { snapshot: &draft };
            let state = protocol::InteractionState::default();
            let hover = InteractionHoverState::new();
            let peers = PeerPresenceRoot::empty();
            let interaction = InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: semio_framework_plugin_interaction::interaction_topology::empty_draft_references() };
            ViewerApp::<V>::handle(&V::Command::default(), &doc, &cfg, &interaction, None, &draft, &semio_framework_2d_compute::EngineHandles::empty()).await.map(|emit| (emit.artifact_mutations.is_empty(), emit.draft_mutations.is_empty()))
        };
        close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
        if let Ok((artifact_is_empty, draft_is_empty)) = result {
            assert!(artifact_is_empty, "a viewer must never emit document mutations");
            assert!(draft_is_empty, "a viewer must never emit draft mutations");
        }
    
        }.await;
        let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
        result
    }

    /// ✏️👁️ Contract §2.5 helper 2/3 — an editor and its viewer over the same subset must resolve
    /// to one `Dialect` coordinate, or `surface_app_id` would mint two different canonical ids
    /// (`…#editor` / `…#viewer`) for what the contract treats as one subset's two surfaces.
    pub async fn assert_editor_and_viewer_share_dialect<E: ArtifactEditor, V: ArtifactViewer>() {
        assert_eq!(E::DIALECT, V::DIALECT, "an editor and its viewer over the same subset must share one Dialect coordinate");
    }

    /// 🔗️ Contract freeze §3 — every surface a plugin contributes for a FOREIGN artifact kind must
    /// be backed by a declared dependency on that kind's owner, or the host's `AppRouter` excludes
    /// the whole plugin (`surface.contribution-not-permitted`). Asserts over an already-assembled
    /// manifest so a bundle that borrows other plugins' apps proves the property from its own test,
    /// not only from the assembly gate (ticket 26/09/05/S-END-TO-END lane H).
    // 🚫️async: pure predicate over already-assembled manifest data, no suspension point — see R9.
    pub fn assert_surface_dependencies_declared(manifest: &semio_framework::PluginManifest) {
        let breaches = semio_framework_plugin_decl_registry::artifact_registration::surface_dependency_breaches(manifest);
        assert!(breaches.is_empty(), "plugin `{}` contributes surfaces without the matching dependency: {}", manifest.plugin_id, breaches.join("; "));
    }

    /// 👁️ Contract §2.5 helper 3/3 — the viewer twin of `new_app::<A: ArtifactApp>()`. `ViewerApp<V>`
    /// already satisfies the runtime `ArtifactApp` bound (the SDK adapter, contract §2.1), so this
    /// is a thin rename over the existing generic harness rather than new machinery.
    pub async fn new_viewer<V: ArtifactViewer>(actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<ViewerApp<V>> {
        new_app::<ViewerApp<V>>(actor, mounted_policy, identity).await
    }


    // 🌳️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C Task 4 — laws for the new
    // declaration tree (`super::declarations`). Every law commits through the REAL
    // `PluginBuilder::declare_artifact`/`try_build` path (or, for the pure id-derivation law, no
    // registration at all) rather than reaching into `declarations::commit_artifact_declarations`
    // directly, so a passing law is proof the whole public surface works end to end.
    /// 🌳️ Builds a plugin from `declaration` and asserts every subset ends up registered in the
    /// schema registry, the io registry (when it declares any `IoEntry` rows), and the app router
    /// (manifest apps) — "nothing is silently dropped" (Task 4, law 1/3).
    pub async fn assert_declaration_tree_registers_all<PA: PluginApp>(plugin_id: &str, declaration: semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let expected_schema_ids: Vec<&'static str> = declaration.standards.iter().flat_map(|standard| standard.subsets.iter().map(|subset| subset.schema.descriptor.id)).collect();
        let mut expected_io_pairs: Vec<(String, String)> = Vec::new();
        for entry in declaration.standards.iter().flat_map(|standard| standard.subsets.iter()).flat_map(|subset| subset.io.entries.iter()) {
            expected_io_pairs.push((semio_framework_artifact_reference::ArtifactDialect::from(entry.from).to_coordinate(), semio_framework_artifact_reference::ArtifactDialect::from(entry.into).to_coordinate()));
        }
        let expected_surface_ids: Vec<String> = declaration.standards.iter().flat_map(|standard| standard.subsets.iter()).flat_map(|subset| [subset.editor.definition.id.clone(), subset.viewer.definition.id.clone()]).collect();

        let plugin = semio_framework_plugin_builder::PluginBuilder::new(plugin_id).label(plugin_id).version("0.0.1").package_id(format!("semio:{plugin_id}")).declare_artifact(declaration).try_build().expect("a well-formed declaration tree must register");

        for id in &expected_schema_ids {
            assert!(::semio_framework_schema_registry::artifact_schema_descriptor_registered(id), "schema descriptor {id:?} must be registered");
        }
        let live_io_entries = semio_framework_os_kernel::io::io_mechanism::io_entries();
        for (from, into) in &expected_io_pairs {
            let mut found = false;
            for entry in live_io_entries.iter() {
                if &entry.from.to_coordinate() == from && &entry.into.to_coordinate() == into {
                    found = true;
                    break;
                }
            }
            assert!(found, "io entry {from} -> {into} must be registered");
        }
        let live_app_ids: Vec<&str> = plugin.manifest.apps.iter().map(|app| app.id.as_str()).collect();
        for id in &expected_surface_ids {
            assert!(live_app_ids.contains(&id.as_str()), "surface {id:?} must be registered on the app router");
        }
    }

    /// 🌳️ `invalid` must fail preflight by construction (e.g. two subsets whose schema
    /// descriptors share an id with different content) — asserts the schema AND io registries are
    /// byte-for-byte unchanged after the rejection: a preflight-failing declaration leaves ZERO
    /// rows behind (Task 4, law 2/3).
    pub async fn assert_declaration_registration_is_atomic<PA: PluginApp>(plugin_id: &str, invalid: semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
        let schema_count_before = ::semio_framework_schema_registry::with_artifact_schema_registry(|registry| registry.len());
        let io_count_before = semio_framework_os_kernel::io::io_mechanism::io_entries().len();
        let result = semio_framework_plugin_builder::PluginBuilder::new(plugin_id).label(plugin_id).version("0.0.1").package_id(format!("semio:{plugin_id}")).declare_artifact(invalid).try_build();
        assert!(result.is_err(), "a declaration that fails preflight must not build a plugin");
        assert_eq!(::semio_framework_schema_registry::with_artifact_schema_registry(|registry| registry.len()), schema_count_before, "schema registry must be unchanged after a rejected declaration");
        assert_eq!(semio_framework_os_kernel::io::io_mechanism::io_entries().len(), io_count_before, "io registry must be unchanged after a rejected declaration");
    }

    /// 🌳️ Every subset's editor/viewer surface id must equal `surface_app_id(&dialect, role)` —
    /// pure, no registration (Task 4, law 3/3).
    pub async fn assert_subset_declaration_ids_are_derived<PA: PluginApp>(declaration: &semio_framework_plugin_decl_registry::declarations::ArtifactDeclaration<PA>) {
        for standard in &declaration.standards {
            for subset in &standard.subsets {
                let dialect: semio_framework_artifact_reference::ArtifactDialect = subset.dialect.into();
                assert_eq!(subset.editor.definition.id, semio_framework::surface_app_id(&dialect, semio_framework::AppRole::Editor), "editor surface id must be derived via surface_app_id");
                assert_eq!(subset.viewer.definition.id, semio_framework::surface_app_id(&dialect, semio_framework::AppRole::Viewer), "viewer surface id must be derived via surface_app_id");
            }
        }
    }
}
