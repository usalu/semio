//! 🧪️ Editor route laws: the retained command, flow-eval, document-io and contributions routes, their work capacity and the fold contract, driven through the composed app.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
/// 🧩️ Aggregator-level geometry fixture binding.
mod test_geometry {
    /// 🧩️ Binds a fixture snapshot to the bundled catalogue and the native compute context.
    pub fn geometry_input(snapshot: &semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot) -> semio_s_artifact_procedural_3d_geometry::GeometryInput<'_> {
        semio_s_artifact_procedural_3d_geometry::GeometryInput::new(snapshot, semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue(), &semio_s_artifact_procedural_3d_io_geometry::NativeGeometryComputeContext)
    }
}
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs"]
mod editor_fold_contract;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
mod editor_unit_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔪️knife-mesh-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod editor_knife_selection_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs"]
mod editor_work_capacity;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod node_graph_edit_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs"]
mod flow_window_tests;
