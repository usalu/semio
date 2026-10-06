//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::viewer::gismap::modes::view::windows::map::config::mutations::*;
use crate::viewer::gismap::modes::view::windows::map::config::{GisMapViewerCamera, GisMapViewerWindowConfig};
use set_camera::SetCamera;

impl protocol::OpText for GisMapViewerWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::viewer::gismap::modes::view::windows::map::config::mutations::*;
use crate::viewer::gismap::modes::view::windows::map::config::{GisMapViewerCamera, GisMapViewerWindowConfig};
use set_camera::SetCamera;

/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `GisMapViewerWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn gis_map_viewer_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<GisMapViewerWindowConfig, GisMapViewerWindowConfigMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_wire_codec::*;
