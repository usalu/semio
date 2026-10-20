//! 🗑️ `delete-character-style` — removes a character style that no story run uses.

use crate::{create_character_style, update_character_style};
#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutCharacterStylesDelta;
use semio_s_artifact_layout_layout_diff::LayoutCharacterStyleRemoval;
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
pub struct DeleteCharacterStyle {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeleteCharacterStyle where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_character_style::CreateCharacterStyle> + From<crate::update_character_style::UpdateCharacterStyle> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "character-style", kind: "delete-character-style", record: "DeletedCharacterStyle" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_delete_character_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_delete_character_style(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete character style \"{}\"", self.id), &format!("Zeichenformat \"{}\" löschen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_delete_character_style(payload: &DeleteCharacterStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.character_styles.iter().position(|style| style.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Character style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if base.stories.iter().any(|story| story.style_runs.iter().any(|run| run.character_style_id.as_deref() == Some(payload.id.as_str()))) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Character style \"{}\" is used by a story.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { character_styles: Some(LayoutCharacterStylesDelta { removed: vec![LayoutCharacterStyleRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_delete_character_style<Op: From<crate::create_character_style::CreateCharacterStyle> + From<crate::update_character_style::UpdateCharacterStyle>>(payload: &DeleteCharacterStyle, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(at) = base.character_styles.iter().position(|style| style.id == payload.id) else { return Vec::new() };
    let style = &base.character_styles[at];
    vec![
        Op::from(update_character_style::UpdateCharacterStyle { id: style.id.clone(), name: style.name.clone(), font_family: style.font_family.clone(), font_size: style.font_size, font_weight: style.font_weight, italic: style.italic, color: style.color, tracking: style.tracking }),
        Op::from(create_character_style::CreateCharacterStyle { id: style.id.clone(), name: style.name.clone(), index: Some(at) }),
    ]

    })())
}


