//! 🧪️ Cross-layer mutation laws: graph-editor replay, touched regions, gesture leaves with their codec witnesses, store-backed mutation behavior and document restoration.

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
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧭️graph-editor/🦀️.rs"]
mod graph_editor_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🗺️touched-regions/🦀️.rs"]
mod touched_regions_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutation_unit_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs"]
mod gesture_leaves_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/📄️document-restoration/🦀️.rs"]
mod document_restoration_tests;
