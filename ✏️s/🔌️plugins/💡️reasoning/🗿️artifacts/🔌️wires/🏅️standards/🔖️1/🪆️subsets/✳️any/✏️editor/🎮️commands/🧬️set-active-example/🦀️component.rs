//! 🧬️ 🧬️ Wires play app commands command — `set-active-example`.

use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::empty_wires_snapshot;
use crate::artifacts::wires::schema::metabolism_wires_example_snapshot;
use crate::artifacts::wires::op::WiresMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use serde::{Deserialize, Serialize};

/// 🧬️ Manifest `.example` id for the metabolism fixture — shared by `SetActiveExample`'s payload check
/// and `crate::editor::wires::create_wires_app`'s `.example(...)` registration.
pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned
/// outright — see `📓️taxonomy.md`'s forbidden vocabulary), so loading a named example builds
/// `editor::wires::reset_wires_document_effect` (a `Effect::LoadDocument`, outside undo history)
/// instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, crate::artifacts::wires::WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let next = if payload.example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
        metabolism_wires_example_snapshot().map_err(|error| {
            let message = if error.target.is_empty() {
                error.message.clone()
            } else {
                format!("{} at {}", error.message, error.target.join("."))
            };
            Fault::new(
                FaultOrigin::App,
                FaultCode::new(error.code.clone()),
                message,
            )
        })?
    } else {
        empty_wires_snapshot()
    };
    Ok(Emit {
        effects: vec![crate::editor::wires::reset_wires_document_effect(&next)],
        config_mutations: vec![WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }],
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{dispatch, metabolism_app, new_app};
    use crate::editor::wires::WiresCommand;
    use crate::artifacts::wires::schema::fixture_nodes;

    /// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned
    /// outright), so `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the
    /// example document's pack bytes rather than an `artifact_mutations` entry — `dispatch`'s
    /// in-process harness never applies `effects` to its own store, so this asserts on the emitted
    /// effect directly (mirrors `🎮️commands/📚️example`-style facets elsewhere in this ticket).
    #[test]
    fn set_active_example_metabolism_loads_seven_nodes() {
        use semio_framework_plugin::Effect;
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: WIRES_PLAY_EXAMPLE_METABOLISM_ID.into() }));
        assert!(result.mutations.is_empty(), "setActiveExample replaces the whole document via an effect, not in-history mutations");
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let document = <crate::artifacts::wires::WiresSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(fixture_nodes(&crate::artifacts::wires::wires_working_board(&document)).len(), 7);
    }

    #[test]
    fn set_active_example_unknown_id_loads_empty_document() {
        use semio_framework_plugin::Effect;
        let mut app = metabolism_app();
        let result = dispatch(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: "nope".into() }));
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let document = <crate::artifacts::wires::WiresSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(fixture_nodes(&crate::artifacts::wires::wires_working_board(&document)).is_empty());
    }
}
//#endregion 🧪️Tests
