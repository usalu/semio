//! 💬️ Lowpoly play app commands — the engagement text input (`engagementInput`) and its typed-token
//! resolution into a real mesh-edit command (`engagementSubmit`).

use crate::editor::lowpoly::commands::mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, triangulate};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::editor::lowpoly::LowpolyCommand;
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{engagement_token_matches, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub async fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![LowpolyConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub value: Option<String>,
    }

    pub async fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        const ENGAGEMENT_COMMANDS: &[&str] = &["extrude", "inset", "bevel", "loopCut", "subdivide", "triangulate", "mirror", "decimate", "flipFaces", "merge", "dissolve", "snap"];
        let Some(typed) = payload.value.as_deref().map(str::trim).filter(|value| !value.is_empty()) else {
            return Ok(Emit::default());
        };
        let Some(&resolved) = ENGAGEMENT_COMMANDS.iter().find(|candidate| engagement_token_matches(typed, candidate)) else {
            return Ok(Emit::default());
        };
        let resolved_command = match resolved {
            "extrude" => LowpolyCommand::Extrude(extrude::Extrude { extrude_distance: None }),
            "inset" => LowpolyCommand::Inset(inset::Inset { inset_amount: None }),
            "bevel" => LowpolyCommand::Bevel(bevel::Bevel { bevel_amount: None, bevel_segments: None }),
            "loopCut" => LowpolyCommand::LoopCut(loop_cut::LoopCut { loop_cuts: None }),
            "subdivide" => LowpolyCommand::Subdivide(subdivide::Subdivide {}),
            "triangulate" => LowpolyCommand::Triangulate(triangulate::Triangulate {}),
            "mirror" => LowpolyCommand::Mirror(mirror::Mirror { axis: None }),
            "decimate" => LowpolyCommand::Decimate(decimate::Decimate { decimate_ratio: None }),
            "flipFaces" => LowpolyCommand::FlipFaces(flip_faces::FlipFaces { face_ids: Vec::new() }),
            "merge" => LowpolyCommand::Merge(merge::Merge {}),
            "dissolve" => LowpolyCommand::Dissolve(dissolve::Dissolve {}),
            "snap" => LowpolyCommand::Snap(snap::Snap {}),
            _ => return Ok(Emit::default()),
        };
        resolved_command.dispatch(doc, cfg, ctx)
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app_with_registry, dispatch, select_face};
    use crate::editor::lowpoly::LowpolyCommand;

    #[semio_framework_async_macros::async_test]
    async fn engagement_submit_resolves_a_typed_token_into_a_real_command() {
        use semio_framework_plugin::PluginApp;
        let mut a = app_with_registry();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        select_face(&mut a, &object_id, 0);
        let before = a.snapshot().expect("projection").objects[0].mesh.clone();
        dispatch(&mut a, LowpolyCommand::EngagementSubmit(super::engagement_submit::EngagementSubmit { value: Some("extrude".into()) }));
        assert_ne!(a.snapshot().expect("projection").objects[0].mesh, before, "typed 'extrude' must run the extrude command");
    }

    #[semio_framework_async_macros::async_test]
    async fn engagement_submit_ignores_unresolvable_input() {
        let mut a = app_with_registry();
        let result = dispatch(&mut a, LowpolyCommand::EngagementSubmit(super::engagement_submit::EngagementSubmit { value: Some("bogus".into()) }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
