//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::viewer::gismap::modes::view::windows::map::config::mutations::*;
use crate::viewer::gismap::modes::view::windows::map::config::{GisMapViewerCamera, GisMapViewerWindowConfig};


impl protocol::OpText for GisMapViewerWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}
}

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::viewer::gismap::modes::view::windows::map::config::mutations::*;
use crate::viewer::gismap::modes::view::windows::map::config::{GisMapViewerCamera, GisMapViewerWindowConfig};


/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `GisMapViewerWindowConfig`.
///
/// @see gis_map_mutation_report_json — the same report for the map snapshot.
pub fn gis_map_viewer_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    use semio_framework_value::{FromValue as _, ToValue as _};
    let decode_snapshot = |text: &str| -> Result<GisMapViewerWindowConfig, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisMapViewerWindowConfigMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let forward = <GisMapViewerWindowConfigMutation as protocol::ColdMutation<GisMapViewerWindowConfig>>::diff(&mutation, &base);
    let applied = protocol::apply_diff_cold(forward.diff(), &base).map_err(|error| error.to_string())?;
    let inverse = <GisMapViewerWindowConfigMutation as protocol::ColdMutation<GisMapViewerWindowConfig>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let outcome = <GisMapViewerWindowConfigMutation as protocol::ColdMutation<GisMapViewerWindowConfig>>::diff(step, &undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
        undone = protocol::apply_diff_cold(outcome.diff(), &undone).map_err(|error| error.to_string())?;
    }
    let json = |value: semio_framework_value::DslValue| semio_framework_pack_json::from_dsl_value(&value);
    let report = semio_framework_pack_json::object([
        ("base".to_string(), json(base.to_value())),
        ("expectedSnapshot".to_string(), json(expected.to_value())),
        ("snapshot".to_string(), json(applied.to_value())),
        ("diff".to_string(), json(forward.diff().to_value())),
        ("messages".to_string(), json(forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), json(inverse.to_value())),
        ("inverseSnapshot".to_string(), json(undone.to_value())),
        ("inverseMessages".to_string(), json(inverse_messages.to_value())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_wire_codec::*;
