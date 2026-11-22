//! 📦️ Saves the registered Puzzle 3D editor's actual portable World3D scene carriers.

use semio_plugin::{artifact_app_laws as laws, AppActionRegistry, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_puzzle_3d_editor::{create_puzzle3d_app, Puzzle3dPlayApp};
use semio_ui_contract::Component;
use semio_ui_locale::{Locale, Terminology};
use std::path::PathBuf;

fn main() {
    std::thread::Builder::new().stack_size(134_217_728).spawn(produce).unwrap().join().unwrap();
}

fn produce() {
    let output = PathBuf::from(std::env::args_os().nth(1).expect("scene output directory"));
    semio_framework_async::block_on(async {
        let mut scope = laws::OriginalFixtureScope::admit();
        let registry = AppActionRegistry::from_definition(&create_puzzle3d_app());
        let policy = laws::fixture_mounted_policy();
        let mut app: VcsArtifactApp<EditorApp<Puzzle3dPlayApp>> =
            semio_plugin::with_authoring_identity!(|identity| VcsArtifactApp::with_registry(EditorApp::<Puzzle3dPlayApp>::default(), registry, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity)
                .await
                .expect("registered Puzzle snapshot"));
        app.bind_instance_id(931).await;
        laws::settle_store_constructor(&mut app, policy);
        let body = "puzzle3d.play.composite:puzzle3d-main-perspective";
        let window = "puzzle3d-main-perspective";
        let mut view = ViewModel::new(Locale::En, Terminology::Native);
        view.window_instances.push(ViewWindowInstance { id: window.into(), window_kind_id: "puzzle3d-main".into() });
        let view = view.for_window_instance(window).expect("registered perspective window");
        let scene = laws::observe_registered_fixture_app(&mut app, body, &view, scope.cx(), |root| {
            let mut pending = vec![root];
            while let Some(node) = pending.pop() {
                if let Component::Surface(surface) = &node.component {
                    if surface.doc_schema.as_str() == "world-3d@1" {
                        let lanes: Vec<_> = node.children.iter().map(|child| (child.key.as_str().to_owned(), laws::built_carrier_text(child))).collect();
                        return Some((surface.doc.bytes.as_slice().to_vec(), lanes));
                    }
                }
                pending.extend(node.children.iter());
            }
            None
        })
        .await
        .expect("registered producer rendered its body")
        .expect("body has a world carrier");
        std::fs::write(output.join("spine.pack"), &scene.0).unwrap();
        std::fs::write(output.join("lanes.json"), serde_json::to_vec(&scene.1).unwrap()).unwrap();
        println!("[DEBUG] Producer wrote {} spine bytes and {} exact lanes", scene.0.len(), scene.1.len());
        laws::close_registered_fixture_app(&mut app, policy, scope.cx()).unwrap();
    });
}
