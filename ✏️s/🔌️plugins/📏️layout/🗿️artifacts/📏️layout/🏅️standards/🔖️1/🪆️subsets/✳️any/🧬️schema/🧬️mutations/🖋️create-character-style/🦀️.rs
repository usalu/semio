//! 🖋 `create-character-style` — adds a character style.

use crate::mutations::{delete_character_style, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::LayoutCharacterStylesDelta;
use crate::{CharacterStyle, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateCharacterStyle {
    pub id: String,
    pub name: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateCharacterStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "character-style", kind: "create-character-style", record: "CreatedCharacterStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_character_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_create_character_style(self, base) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Create character style \"{}\"", self.id), &format!("Zeichenformat \"{}\" erstellen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_create_character_style(payload: &CreateCharacterStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if payload.id.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A character style needs an id.", std::iter::empty::<String>());
    }
    if base.character_styles.iter().any(|style| style.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Character style \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let style = CharacterStyle { id: payload.id.clone(), name: payload.name.clone(), font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None };
    protocol::MutationOutcome::new(LayoutDiff { character_styles: Some(LayoutCharacterStylesDelta { added: vec![style], ..Default::default() }), ..Default::default() })
}

pub fn inverse_create_character_style(payload: &CreateCharacterStyle, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteCharacterStyle(delete_character_style::DeleteCharacterStyle { id: payload.id.clone() })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
