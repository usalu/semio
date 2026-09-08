
use super::*;
use semio_framework_plugin::testkit::{meta, new_app};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Fem2dApp = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub fn fem2d_app() -> Fem2dApp {
    semio_framework_plugin::resolve_ready(new_app::<EditorApp<Fem2dPlayApp>>())
}

pub async fn dispatch(app: &mut Fem2dApp, command: Fem2dCommand) -> InvocationResult {
    let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
    for effect in &result.requested_effects {
        if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
            let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
            app.load_document_pack(&files).await.expect("test host applies load-document effect");
        }
    }
    result
}

pub fn render(app: &mut Fem2dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render")).expect("fixture projection")
}
