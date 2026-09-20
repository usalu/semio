//! 🎬️ 🎬️ Equation play app commands command — `set-active-example`.

use crate::editor::equation::reset_equation_document_effect;
use crate::op::EquationMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ The whole-document replacement this verb publishes. It has no `EquationMutation`
/// representative — the taxonomy forbids a whole-document replace variant — so it lands as an
/// `Effect::LoadDocument` outside undo history, the `🕸️dag`/`🏛️architect` shape. The document is
/// parsed from this subset's own committed example asset, so what loads is exactly what
/// `📚️examples/🎬️demo` declares rather than a second, hand-built copy of it. An id this app does not
/// publish is an empty emit rather than a fault: the playground navbar dispatches whatever its
/// combobox holds, including ids belonging to other apps.
///
/// 🪨️ Free of any `ArtifactView`, because the retained work's `setActiveExample` branch runs BEFORE
/// a scene owner exists — resolving one is precisely what this verb is there to make possible.
pub fn emit(example_id: &str) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    if !example_id.is_empty() && example_id != crate::examples::demo::ID {
        return Ok(Emit::default());
    }
    let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT)
        .map_err(|error| Fault::from(format!("equation example {} is not parsable: {error:?}", crate::examples::demo::ID)))?;
    Ok(Emit { effects: vec![reset_equation_document_effect(&document)], ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, crate::EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    emit(&payload.example_id)
}
