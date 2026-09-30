//! 🗑️ `delete-character-style` — removes a character style that no story run uses.

use crate::mutations::{create_character_style, update_character_style, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::LayoutCharacterStylesDelta;
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteCharacterStyle {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteCharacterStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "character-style", kind: "delete-character-style", record: "DeletedCharacterStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_delete_character_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_delete_character_style(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Delete character style \"{}\"", self.id), &format!("Zeichenformat \"{}\" löschen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_delete_character_style(payload: &DeleteCharacterStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.character_styles.iter().any(|style| style.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Character style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.stories.iter().any(|story| story.style_runs.iter().any(|run| run.character_style_id.as_deref() == Some(payload.id.as_str()))) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Character style \"{}\" is used by a story.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { character_styles: Some(LayoutCharacterStylesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}

pub fn inverse_delete_character_style(payload: &DeleteCharacterStyle, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(style) = base.character_styles.iter().find(|style| style.id == payload.id) else { return Vec::new() };
    vec![
        LayoutMutation::CreateCharacterStyle(create_character_style::CreateCharacterStyle { id: style.id.clone(), name: style.name.clone() }),
        LayoutMutation::UpdateCharacterStyle(update_character_style::UpdateCharacterStyle { id: style.id.clone(), name: style.name.clone(), font_family: style.font_family.clone(), font_size: style.font_size, font_weight: style.font_weight, italic: style.italic, color: style.color, tracking: style.tracking }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
