//! 🗺️ CAD play app commands — which model definition the document is focused on, and which bundled example is loaded.

use semio_s_artifact_cad_cad_config::{CadConfig, CadConfigMutation};
use semio_s_artifact_cad_cad_kit::CadDispatchCtx;
use semio_s_artifact_cad_cad_kit::{publish_engagement, reset_document_effect, snapshot_of, CadPlayRuntime};
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_inferences::{default_document, forest_play_scene, CAD_EXAMPLE_FOREST_LEFT};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🗃️ Loads the named example as ONE whole-document replacement: the empty id is the app's default
    /// document, the Concrete Forest id its bundled scene, and a registered example (the shell's
    /// navbar picker offers exactly `crate::examples::*`, and announces the first one at boot) the
    /// document its authored asset spells. An id the app does not ship is refused by name instead of
    /// silently leaving the previous document in place.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let example_id = payload.example_id.as_str();
        let scene = if example_id.is_empty() {
            default_document()
        } else if example_id == CAD_EXAMPLE_FOREST_LEFT || example_id == "forest-left" {
            forest_play_scene()
        } else if example_id == semio_s_artifact_cad_cad_examples::ID {
            <CadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_cad_cad_examples::PRIMARY_TEXT).map_err(|error| Fault::from(format!("cad.example.invalid: registered example `{example_id}` does not parse: {error:?}")))?
        } else {
            return Err(Fault::from(format!("cad.example.unknown: `{example_id}` is not an example this app ships")));
        };
        let runtime = CadPlayRuntime {
            active_example_id: (!example_id.is_empty()).then(|| if example_id == "forest-left" { CAD_EXAMPLE_FOREST_LEFT.to_string() } else { example_id.to_string() }),
            ..CadPlayRuntime::default()
        };
        let mut emit = Emit { effects: vec![reset_document_effect(&scene)], ..Default::default() };
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)?];
        publish_engagement(&runtime, ctx);
        Ok(emit)
    }
}
//#endregion 🔖️SetActiveExample
