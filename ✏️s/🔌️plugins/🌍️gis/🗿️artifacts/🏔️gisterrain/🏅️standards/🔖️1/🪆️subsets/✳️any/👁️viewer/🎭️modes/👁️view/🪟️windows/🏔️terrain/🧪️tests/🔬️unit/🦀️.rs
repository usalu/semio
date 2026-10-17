use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use crate::schema::terrain_descriptor::TerrainDescriptorJson;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::build_terrain_scene_json;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::fixture_descriptor;
#[allow(unused_imports)]
use crate::GisTerrainSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)]
use semio_framework_plugin::scene_surface;
#[allow(unused_imports)]
use semio_framework_plugin::world3d_selection_json;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_plugin::World3dScene;
#[allow(unused_imports)]
use semio_framework_surface::terrain::projection;
#[allow(unused_imports)]
use serde_json::{json, Value};
use crate::viewer::gisterrain::modes::view::windows::terrain::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_world3d_terrain_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::schema::default_terrain_document();
    let _node = render(&document);
}
