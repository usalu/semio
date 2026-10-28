//! 🎨 `update-character-style` — replaces one character style.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::{CharacterStylePatch, LayoutCharacterStylesDelta};
use semio_s_artifact_layout_layout_diff::LayoutCharacterStylesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateCharacterStyle where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_character_style::UpdateCharacterStyle> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "character-style", kind: "update-character-style", record: "UpdatedCharacterStyle" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update character style \"{}\"", self.id), &format!("Zeichenformat \"{}\" aktualisieren", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for UpdateCharacterStyle where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_character_style::UpdateCharacterStyle> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_character_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_update_character_style(self, base)? 
    })
}
}

pub fn diff_update_character_style(payload: &UpdateCharacterStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(style) = base.character_styles.iter().find(|style| style.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Character style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.font_size.is_some_and(|size| !size.is_finite() || size <= 0.0) || payload.font_weight.is_some_and(|weight| weight == 0) || payload.tracking.is_some_and(|tracking| !tracking.is_finite()) || payload.color.as_ref().is_some_and(|color| color.iter().any(|value| !value.is_finite())) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A character style size must be positive, its weight non-zero, and its tracking finite.", std::iter::empty::<String>());
    }
    if style.name == payload.name && style.font_family == payload.font_family && semio_s_artifact_layout_layout_model::equality::equal_optional_f64(style.font_size, payload.font_size) && style.font_weight == payload.font_weight && style.italic == payload.italic && semio_s_artifact_layout_layout_model::equality::equal_color(&style.color, &payload.color) && semio_s_artifact_layout_layout_model::equality::equal_optional_f64(style.tracking, payload.tracking) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Character style is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        character_styles: Some(LayoutCharacterStylesDelta {
            modified: vec![LayoutCharacterStylesModification {
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

pub fn inverse_update_character_style<Op: From<crate::update_character_style::UpdateCharacterStyle>>(payload: &UpdateCharacterStyle, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_update_character_style(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    Ok((|| {
    let Some(style) = base.character_styles.iter().find(|style| style.id == payload.id) else { return Vec::new() };
    vec![Op::from(UpdateCharacterStyle { id: style.id.clone(), name: style.name.clone(), font_family: style.font_family.clone(), font_size: style.font_size, font_weight: style.font_weight, italic: style.italic, color: style.color, tracking: style.tracking })]

    })())
}


impl PartialEq for UpdateCharacterStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.font_family == other.font_family
            && semio_s_artifact_layout_layout_model::equality::equal_optional_f64(self.font_size, other.font_size)
            && self.font_weight == other.font_weight
            && self.italic == other.italic
            && semio_s_artifact_layout_layout_model::equality::equal_color(&self.color, &other.color)
            && semio_s_artifact_layout_layout_model::equality::equal_optional_f64(self.tracking, other.tracking)
    }
}
