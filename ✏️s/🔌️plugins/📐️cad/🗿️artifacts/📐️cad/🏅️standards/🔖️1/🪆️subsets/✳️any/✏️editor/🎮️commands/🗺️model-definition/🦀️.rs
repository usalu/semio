//! 🗺️ CAD play app commands — which model definition the document is focused on, and which bundled example is loaded.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{preview_transition_snapshot_of, reset_document_effect, CadPlayRuntime};
use crate::op::CadMutation;
use crate::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_EXAMPLE_FOREST_LEFT};
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
        } else if example_id == crate::examples::demo::ID {
            <CadSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| Fault::from(format!("cad.example.invalid: registered example `{example_id}` does not parse: {error:?}")))?
        } else {
            return Err(Fault::from(format!("cad.example.unknown: `{example_id}` is not an example this app ships")));
        };
        let runtime = CadPlayRuntime {
            active_example_id: (!example_id.is_empty()).then(|| if example_id == "forest-left" { CAD_EXAMPLE_FOREST_LEFT.to_string() } else { example_id.to_string() }),
            ..CadPlayRuntime::default()
        };
        let mut emit = Emit { effects: vec![reset_document_effect(&scene)], ..Default::default() };
        emit.config_mutations = vec![preview_transition_snapshot_of(&runtime, cfg.snapshot, ctx)?];
        Ok(emit)
    }
}
//#endregion 🔖️SetActiveExample
