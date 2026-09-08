
use super::*;
use semio_framework_plugin::ActionMeta;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
use store::ArtifactEnvelope;

/// ✏️ `VcsPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
/// — `EditorApp<VcsPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
/// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<VcsPlayApp>` builds it.
pub type VcsApp = VcsArtifactApp<EditorApp<VcsPlayApp>>;

/// ✏️ Adapts `create_vcs_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::new_app_with_registry` still expects — framework testkit gap, not
/// modifiable here (`🧰️framework/**` is outside this packet's lease).
fn vcs_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_vcs_app(), examples: Vec::new() }
}

/// 🧪️ A bare, pre-seeded app instance — no `AppActionRegistry`, so undeclared internal commands
/// dispatch freely. Seeded via `seed_vcs_demo_history` (see its own doc comment for why this
/// replaced `ArtifactApp::seed`).
pub async fn app() -> VcsApp {
    let mut instance = new_app::<EditorApp<VcsPlayApp>>().await;
    seed_vcs_demo_history(&mut instance).await;
    instance
}

/// 🧪️ A pre-seeded app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn app_with_registry() -> VcsApp {
    let mut instance = new_app_with_registry::<EditorApp<VcsPlayApp>>(vcs_app_manifest_for_testkit).await;
    seed_vcs_demo_history(&mut instance).await;
    instance
}

/// 🧾️ Builds one flat, string-valued action argument object — the `DslValue` shape
/// `handle_action`/`command_from_action` take now that the action wire is the DSL value, not JSON.
pub fn action_args(entries: impl IntoIterator<Item = (&'static str, String)>) -> dsl::DslValue {
    dsl::DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), dsl::DslValue::String(value))))
}

/// 🕳️ The empty action argument object.
pub fn no_args() -> dsl::DslValue {
    dsl::DslValue::Object(Vec::new())
}

pub async fn dispatch(instance: &mut VcsApp, command: VcsCommand) -> InvocationResult {
    instance.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(instance: &mut VcsApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(instance.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

/// 📦️ Parses `document_pack()` (the full envelope) for tests that need to inspect raw
/// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
/// edit in the log is still applied.
pub async fn seeded_envelope(instance: &VcsApp) -> ArtifactEnvelope<VcsSnapshot, VcsDemoMutation> {
    let files = instance.document_pack().await.expect("document pack");
    store::parse_document_pack::<VcsSnapshot, VcsDemoMutation>(&files.pack, &files.spr).await.expect("parse document pack").envelope
}

/// 🌱️ Seeds a rich, forked checkpoint/alternative history through `VcsApp`'s own public dispatch
/// surface (`dispatch_typed`/`handle_action`) — this app's whole point is exercising the history UI
/// (swimlane graph, checkpoints, alternatives, undo/redo), so every test instance starts as a
/// populated history, not a bare projection. Replaces the old direct-`ArtifactStore`-touch
/// `seed_vcs_demo_history(&mut ArtifactStore)` dispatched via the now-removed `ArtifactApp::seed`
/// hook (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4). Field edits go through
/// `VcsCommand::TextEdit` (whole-projection diff, matching `patch::text_edit_operations`) so one
/// call can bundle several field changes into one undo-log entry, mirroring the original narrative's
/// grouping. Per-checkpoint authorship is lost here: `handle_action`'s `"commitCheckpoint"` arm
/// hardcodes `authors: Vec::new()` with no wire path for real authors (framework-owned, out of this
/// plugin's boundary) — no test asserts on authorship, so this is a silent, documented fidelity
/// loss, not a functional gap.
async fn seed_edit(app: &mut VcsApp, local: &ActionMeta, mutate: fn(&mut VcsSnapshot)) {
    let mut next = app.snapshot().expect("materialize snapshot");
    mutate(&mut next);
    let text = serde_json::to_string(&next).expect("serialize snapshot");
    let _ = app.dispatch_typed(VcsCommand::TextEdit(text_edit::TextEdit { text }), local).await;
}

async fn seed_commit(app: &mut VcsApp, local: &ActionMeta, message: &str) {
    let _ = app.handle_action("commitCheckpoint", Some(&action_args([("message", message.to_string())])), local).await;
}

async fn seed_checkout(app: &mut VcsApp, local: &ActionMeta, checkpoint_id: &str) {
    let _ = app.handle_action("checkoutCheckpoint", Some(&action_args([("checkpointId", checkpoint_id.to_string())])), local).await;
}

async fn seed_create_alternative(app: &mut VcsApp, local: &ActionMeta, name: &str) -> String {
    let _ = app.handle_action("createAlternative", Some(&action_args([("name", name.to_string())])), local).await;
    seeded_envelope(app).await.active_alternative_id.clone().expect("alternative id")
}

async fn seed_switch_alternative(app: &mut VcsApp, local: &ActionMeta, alternative_id: &str) {
    let _ = app.handle_action("switchAlternative", Some(&action_args([("alternativeId", alternative_id.to_string())])), local).await;
}

async fn seed_last_checkpoint_id(app: &VcsApp) -> String {
    seeded_envelope(app).await.vcs.checkpoints.last().expect("checkpoint just committed").id.clone()
}

pub async fn seed_vcs_demo_history(app: &mut VcsApp) {
    let local = meta("local");

    seed_edit(app, &local, |s| {
        s.counter = 1;
        s.title = "VCS Demo".into();
    })
    .await;
    seed_commit(app, &local, "Bootstrap").await;
    let c1 = seed_last_checkpoint_id(app).await;

    seed_edit(app, &local, |s| {
        s.notes = "main line".into();
        s.status = "draft".into();
    })
    .await;
    seed_commit(app, &local, "Annotate main draft").await;
    let c2 = seed_last_checkpoint_id(app).await;

    seed_edit(app, &local, |s| {
        s.counter = 2;
    })
    .await;
    seed_commit(app, &local, "Main milestone").await;
    let c3 = seed_last_checkpoint_id(app).await;

    seed_checkout(app, &local, &c3).await;
    let feature_a_id = seed_create_alternative(app, &local, "feature-a").await;
    seed_edit(app, &local, |s| {
        s.title = "Feature A".into();
        s.tags.push("feature-a".into());
    })
    .await;
    seed_commit(app, &local, "Start feature A").await;
    let c4 = seed_last_checkpoint_id(app).await;

    seed_edit(app, &local, |s| {
        s.counter = 10;
    })
    .await;
    seed_commit(app, &local, "Feature A progress").await;

    seed_checkout(app, &local, &c3).await;
    let feature_b_id = seed_create_alternative(app, &local, "feature-b").await;
    seed_edit(app, &local, |s| {
        s.title = "Feature B".into();
        s.notes = "branch b".into();
    })
    .await;
    seed_commit(app, &local, "Start feature B").await;

    seed_edit(app, &local, |s| {
        s.counter = 20;
    })
    .await;
    seed_commit(app, &local, "Feature B try").await;

    seed_checkout(app, &local, &c3).await;
    seed_edit(app, &local, |s| {
        s.status = "active".into();
    })
    .await;
    seed_commit(app, &local, "Resume main").await;
    let c8 = seed_last_checkpoint_id(app).await;

    seed_switch_alternative(app, &local, &feature_a_id).await;
    seed_edit(app, &local, |s| {
        s.counter = 11;
        s.tags.push("wip".into());
    })
    .await;
    seed_commit(app, &local, "Feature A sprint").await;

    seed_checkout(app, &local, &c4).await;
    let _ = seed_create_alternative(app, &local, "feature-a-hotfix").await;
    seed_edit(app, &local, |s| {
        s.status = "hotfix".into();
    })
    .await;
    seed_commit(app, &local, "Hotfix off feature A").await;

    seed_switch_alternative(app, &local, &feature_b_id).await;
    seed_edit(app, &local, |s| {
        s.tags.push("review".into());
    })
    .await;
    seed_commit(app, &local, "Feature B review").await;

    seed_checkout(app, &local, &c8).await;
    seed_edit(app, &local, |s| {
        s.counter = 3;
        s.notes = "main polish".into();
        s.tags.push("release".into());
    })
    .await;
    seed_commit(app, &local, "Main batch polish").await;

    seed_edit(app, &local, |s| {
        s.status = "done".into();
    })
    .await;
    seed_commit(app, &local, "Main release").await;

    seed_checkout(app, &local, &c2).await;
    let _ = seed_create_alternative(app, &local, "docs").await;
    seed_edit(app, &local, |s| {
        s.notes = "documentation pass".into();
    })
    .await;
    seed_commit(app, &local, "Docs branch").await;

    seed_checkout(app, &local, &c1).await;
    let _ = seed_create_alternative(app, &local, "spike").await;
    seed_edit(app, &local, |s| {
        s.title = "Spike prototype".into();
    })
    .await;
    seed_commit(app, &local, "Spike experiment").await;
}
