
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type LowpolyApp = VcsArtifactApp<EditorApp<LowpolyPlayApp>>;

/// 🧪️ `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` (framework testkit,
/// unchanged for this ticket) still take `fn() -> App` — `create_lowpoly_app` now returns
/// `AppDefinition` (contract §2.4). This tiny local wrapper is the documented bridge (pilot report
/// `📓️w2-cad-report.md` recipe step 7), not a framework fix owed by this packet.
fn lowpoly_manifest_for_testkit() -> App {
    App { definition: create_lowpoly_app(), examples: Vec::new() }
}

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn app() -> LowpolyApp {
    new_app::<EditorApp<LowpolyPlayApp>>().await
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn app_with_registry() -> LowpolyApp {
    new_app_with_registry::<EditorApp<LowpolyPlayApp>>(lowpoly_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut LowpolyApp, body_key: &str) -> String {
    serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
/// injected `interactionSelect` verb, dispatched against the "mesh" domain declared on this app —
/// requires `app_with_registry().await` (a bare `app().await` has no declared interaction domains to select
/// against). `object_id`/`face_id` address the same row id the Document panel tree renders (see
/// `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region).
pub async fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32) {
    let target_id = crate::editor::lowpoly::view::document_target_row_id(object_id, 0, "face", face_id);
    let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "face", "id": target_id }])).expect("targets json");
    app.handle_action("interactionSelect", Some(&protocol::DslValue::from(&serde_json::json!({ "domainId": MESH_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" }))), &meta("test")).await.expect("interactionSelect");
}
