//! 📝️ `update-paragraph-style` — replaces one paragraph style's typography.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::{LayoutParagraphStylesDelta, ParagraphStylePatch};
use crate::standards::v1::subsets::any::schema::diff::LayoutParagraphStylesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
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

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateParagraphStyle where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_paragraph_style::UpdateParagraphStyle> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "paragraph-style", kind: "update-paragraph-style", record: "UpdatedParagraphStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_paragraph_style(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_update_paragraph_style(self, base)?
    
    })
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Paragraph style is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        paragraph_styles: Some(LayoutParagraphStylesDelta {
            modified: vec![LayoutParagraphStylesModification {
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

pub fn inverse_update_paragraph_style<Op: From<crate::update_paragraph_style::UpdateParagraphStyle>>(payload: &UpdateParagraphStyle, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(style) = base.paragraph_styles.iter().find(|style| style.id == payload.id) else { return Vec::new() };
    vec![Op::from(UpdateParagraphStyle {
        id: style.id.clone(),
        name: style.name.clone(),
        font_family: style.font_family.clone(),
        font_size: style.font_size,
        font_weight: style.font_weight,
        leading: style.leading,
        tracking: style.tracking,
        alignment: style.alignment.clone(),
    })]

    })())
}

