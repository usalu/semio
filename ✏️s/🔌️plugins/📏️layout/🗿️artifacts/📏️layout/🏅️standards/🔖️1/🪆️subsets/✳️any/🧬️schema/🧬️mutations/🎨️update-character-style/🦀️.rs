//! 🎨 `update-character-style` — replaces one character style.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{CharacterStylePatch, LayoutCharacterStylePatchEntry, LayoutCharacterStylesDelta};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateCharacterStyle {
    pub id: String,
    pub name: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub italic: Option<bool>,
    pub color: Option<[f32; 4]>,
    pub tracking: Option<f64>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateCharacterStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "character-style", kind: "update-character-style", record: "UpdatedCharacterStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_character_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_update_character_style(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Update character style \"{}\"", self.id), &format!("Zeichenformat \"{}\" aktualisieren", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_update_character_style(payload: &UpdateCharacterStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(style) = base.character_styles.iter().find(|style| style.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Character style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.font_size.is_some_and(|size| !size.is_finite() || size <= 0.0) || payload.font_weight.is_some_and(|weight| weight == 0) || payload.tracking.is_some_and(|tracking| !tracking.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A character style size must be positive, its weight non-zero, and its tracking finite.", std::iter::empty::<String>());
    }
    if style.name == payload.name && style.font_family == payload.font_family && style.font_size == payload.font_size && style.font_weight == payload.font_weight && style.italic == payload.italic && style.color == payload.color && style.tracking == payload.tracking {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Character style is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        character_styles: Some(LayoutCharacterStylesDelta {
            patched: vec![LayoutCharacterStylePatchEntry {
                id: payload.id.clone(),
                patch: CharacterStylePatch {
                    name: Some(payload.name.clone()),
                    font_family: Some(payload.font_family.clone()),
                    font_size: Some(payload.font_size),
                    font_weight: Some(payload.font_weight),
                    italic: Some(payload.italic),
                    color: Some(payload.color),
                    tracking: Some(payload.tracking),
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn inverse_update_character_style(payload: &UpdateCharacterStyle, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(style) = base.character_styles.iter().find(|style| style.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::UpdateCharacterStyle(UpdateCharacterStyle { id: style.id.clone(), name: style.name.clone(), font_family: style.font_family.clone(), font_size: style.font_size, font_weight: style.font_weight, italic: style.italic, color: style.color, tracking: style.tracking })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
