//! 📝️ `update-paragraph-style` — replaces one paragraph style's typography.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutParagraphStylePatchEntry, LayoutParagraphStylesDelta, ParagraphStylePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateParagraphStyle {
    pub id: String,
    pub name: String,
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: u32,
    pub leading: f64,
    pub tracking: f64,
    pub alignment: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateParagraphStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "paragraph-style", kind: "update-paragraph-style", record: "UpdatedParagraphStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_paragraph_style(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_update_paragraph_style(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update paragraph style \"{}\"", self.name), &format!("Absatzformat \"{}\" aktualisieren", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

fn alignment_ok(value: &str) -> bool {
    matches!(value, "left" | "center" | "right" | "justify")
}

pub fn diff_update_paragraph_style(payload: &UpdateParagraphStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(style) = base.paragraph_styles.iter().find(|style| style.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paragraph style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.font_size.is_finite() || payload.font_size <= 0.0 || !payload.leading.is_finite() || payload.leading <= 0.0 || !payload.tracking.is_finite() || payload.font_weight == 0 || !alignment_ok(&payload.alignment) || payload.name.trim().is_empty() || payload.font_family.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A paragraph style needs a name, a font family, a positive size and leading, a finite tracking, a non-zero weight, and alignment left, center, right, or justify.", std::iter::empty::<String>());
    }
    if style.name == payload.name && style.font_family == payload.font_family && style.font_size == payload.font_size && style.font_weight == payload.font_weight && style.leading == payload.leading && style.tracking == payload.tracking && style.alignment == payload.alignment {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Paragraph style is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        paragraph_styles: Some(LayoutParagraphStylesDelta {
            patched: vec![LayoutParagraphStylePatchEntry {
                id: payload.id.clone(),
                patch: ParagraphStylePatch {
                    name: Some(payload.name.clone()),
                    font_family: Some(payload.font_family.clone()),
                    font_size: Some(payload.font_size),
                    font_weight: Some(payload.font_weight),
                    leading: Some(payload.leading),
                    tracking: Some(payload.tracking),
                    alignment: Some(payload.alignment.clone()),
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn inverse_update_paragraph_style(payload: &UpdateParagraphStyle, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(style) = base.paragraph_styles.iter().find(|style| style.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::UpdateParagraphStyle(UpdateParagraphStyle {
        id: style.id.clone(),
        name: style.name.clone(),
        font_family: style.font_family.clone(),
        font_size: style.font_size,
        font_weight: style.font_weight,
        leading: style.leading,
        tracking: style.tracking,
        alignment: style.alignment.clone(),
    })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
