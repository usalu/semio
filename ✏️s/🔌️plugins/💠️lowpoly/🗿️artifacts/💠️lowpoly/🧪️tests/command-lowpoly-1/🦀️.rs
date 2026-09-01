//! 🦀️ Lowpoly command catalog case — Rust subject adapter. Ticket
//! `26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`.
//!
//! Recorded no-oracle decision `lowpoly-command-catalog-shape`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`) — see this case's own feature file
//! for the full "reduced scope, stated honestly" account of why: dispatching a command and asserting
//! its produced mutation needs `ArtifactView`/`ConfigView` (from `semio_framework_plugin`) and
//! `Mutation::diff`/`apply` (from `protocol`), neither of which any generated Rust test host in this
//! repository links today. What this case asserts is the narrower claim that survives without either
//! crate: `LowpolyCommand`'s `TOOL_JOB_IDS`/`command_id()` are plain INHERENT items on the enum the
//! macro generates, and every payload struct is a public, directly constructible type.

use semio_repo_test_host::{Adapter, Context, Outcome};

#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_lowpoly::editor::lowpoly::commands::{add_primitive, camera, chrome, engagement, fixture, mesh_edit, patch_object, paint, selection, sun, transform, utility, uv};
    use semio_s_plugin_lowpoly::editor::lowpoly::LowpolyCommand;

    /// 🧾️ One representative value per group, mirroring the crate's own `every_command()` test
    /// helper's example payloads exactly (`✏️editor/🦀️component.rs`'s `#[cfg(test)] mod tests`), so a
    /// drift between the two is a drift this case would also have to be told about by hand.
    fn representative(group: &str) -> Option<LowpolyCommand> {
        match group {
            "patch-object" => Some(LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some("\"Renamed\"".into()) })),
            "add-primitive" => Some(LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) })),
            "sun" => Some(LowpolyCommand::SetSunAzimuth(sun::set_sun_azimuth::SetSunAzimuth { value: 45.0 })),
            "camera" => Some(LowpolyCommand::SetCamera(camera::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })),
            "chrome" => Some(LowpolyCommand::ToggleShowEdges(chrome::toggle_show_edges::ToggleShowEdges {})),
            "engagement" => Some(LowpolyCommand::EngagementInput(engagement::engagement_input::EngagementInput { value: "ext".into() })),
            "fixture" => Some(LowpolyCommand::SetFixtureJson(fixture::set_fixture_json::SetFixtureJson { json: "{}".into() })),
            "mesh-edit" => Some(LowpolyCommand::ToggleSmooth(mesh_edit::toggle_smooth::ToggleSmooth {})),
            "paint" => Some(LowpolyCommand::AddPaintLayer(paint::add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) })),
            "selection" => Some(LowpolyCommand::SetActiveObject(selection::set_active_object::SetActiveObject { object_id: "obj-1".into() })),
            "utility" => Some(LowpolyCommand::SetUtilityParam(utility::set_utility_param::SetUtilityParam { key: "brushSize".into(), value_json: "20".into() })),
            "transform" => Some(LowpolyCommand::TransformEnd(transform::transform_end::TransformEnd {})),
            "uv" => Some(LowpolyCommand::UnwrapActive(uv::unwrap_active::UnwrapActive {})),
            _ => None,
        }
    }

    pub fn command_shape(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let group = spec.str("group");
        let expected_id = spec.str("commandId");
        let command = representative(&group).ok_or_else(|| format!("no representative payload registered for group {group:?}"))?;
        let actual_id = command.command_id();
        if actual_id != expected_id {
            return Err(format!("group {group:?}: command_id() returned {actual_id:?}, the feature declared {expected_id:?}"));
        }
        let matches = LowpolyCommand::TOOL_JOB_IDS.iter().filter(|id| **id == expected_id).count();
        if matches != 1 {
            return Err(format!("group {group:?}: LowpolyCommand::TOOL_JOB_IDS contains {expected_id:?} {matches} time(s), expected exactly once"));
        }
        let projection = semio_repo_test_host::parse_json(&format!("{{\"group\":\"{group}\",\"commandId\":\"{actual_id}\"}}"))?;
        Ok(Outcome::with_raw(actual_id.as_bytes().to_vec(), projection))
    }

    pub fn catalog_size(_ctx: &Context) -> Result<Outcome, String> {
        let ids = LowpolyCommand::TOOL_JOB_IDS;
        if ids.len() != 47 {
            return Err(format!("LowpolyCommand::TOOL_JOB_IDS has {} entries, expected exactly 47", ids.len()));
        }
        let mut sorted: Vec<&&str> = ids.iter().collect();
        sorted.sort_unstable();
        sorted.dedup();
        if sorted.len() != ids.len() {
            return Err(format!("LowpolyCommand::TOOL_JOB_IDS has {} entries but only {} are unique", ids.len(), sorted.len()));
        }
        let projection = semio_repo_test_host::parse_json(&format!("{{\"count\":{}}}", ids.len()))?;
        Ok(Outcome::with_raw(format!("{}", ids.len()).into_bytes(), projection))
    }
}

//#region 🔖️Registration
/// 🧭️ Registration is by full expanded scenario id (`command-<group>`, from the feature's own
/// `Examples` table's `id` column) plus the standalone `catalog-size` scenario.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for group in ["patch-object", "add-primitive", "sun", "camera", "chrome", "engagement", "fixture", "mesh-edit", "paint", "selection", "utility", "transform", "uv"] {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("command-{group}"), subject::command_shape);
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("catalog-size", subject::catalog_size);
    }
    built
}
//#endregion 🔖️Registration
