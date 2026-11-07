//! 🪧️ Generator pane smoke: the bundled hexagonal mushroom column loads through the example command, one `AddGeneration` applies, undo restores the loaded snapshot and the app closes through its exact retained close.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[global_allocator]
static HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
use app_fixture as context;
use semio_s_artifact_procedural_3d_editor::*;
use semio_s_artifact_procedural_3d_editor_commands::{add_generation, set_active_example};

/// 🌀️ LAW: the Generator pane loads its bundled example, adds a generation, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn generator_pane_loads_applies_undoes_and_closes() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let mut app = context::app().await;
    let example = Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() });
    let command = Generation3dCommand::AddGeneration(add_generation::AddGeneration {});
    let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, example, command, |app| { let read = context::snapshot(app); (read.generation.generations.len(), read.host_snapshot.widgets.len()) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert!(loaded.1 > 0, "the loaded example carries its widgets");
    assert_eq!(applied.0, loaded.0 + 1);
}
