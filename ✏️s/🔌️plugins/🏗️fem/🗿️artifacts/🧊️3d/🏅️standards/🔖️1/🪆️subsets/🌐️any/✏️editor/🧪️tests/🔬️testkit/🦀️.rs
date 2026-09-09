use super::*;
use semio_framework_plugin::testkit::{meta, new_app};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Fem3dApp = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
/// `EditorApp<Fem3dPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
/// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<Fem3dPlayApp>` builds it.
pub fn fem3d_app() -> Fem3dApp {
    semio_framework_plugin::resolve_ready(new_app::<EditorApp<Fem3dPlayApp>>())
}

pub async fn dispatch(app: &mut Fem3dApp, command: Fem3dCommand) -> InvocationResult {
    let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
    for effect in &result.requested_effects {
        if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
            let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
            app.load_document_pack(&files).await.expect("test host applies load-document effect");
        }
    }
    result
}

pub fn render(app: &mut Fem3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render")).expect("fixture projection")
}

/// 🧪️ An app reset to the empty document. `Fem3dPlayApp::initial_snapshot` now boots the bundled
/// `default` example (so the `World3d` Model window paints real geometry on first paint), which means
/// a test reasoning about "the first material" or "no load cases yet" has to say so explicitly —
/// `setActiveExample` with any id other than `examples::demo::ID` is exactly that reset, and `dispatch` above
/// already applies its `Effect::LoadDocument` the way the real host does.
pub async fn fem3d_empty_app() -> Fem3dApp {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "empty".into() })).await;
    app
}
