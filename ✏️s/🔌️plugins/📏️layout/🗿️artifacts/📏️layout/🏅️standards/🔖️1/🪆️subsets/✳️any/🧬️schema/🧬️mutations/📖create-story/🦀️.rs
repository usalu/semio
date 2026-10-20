//! 📖 `create-story` — brings a new {@link TextStory} into existence in the id-keyed `stories`
//! collection.

use crate::delete_story;
use semio_s_artifact_layout_layout_diff::LayoutStoriesDelta;
use semio_s_artifact_layout_layout_diff::LayoutStoryInsertion;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::TextStory;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📖CreateStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateStory {
    pub story: TextStory,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreateStory where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::delete_story::DeleteStory> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "story", kind: "create-story", record: "CreatedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_story(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create story \"{}\"", self.story.id), &format!("Textfluss \"{}\" erstellen", self.story.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.story.id.clone()]
    }
}
//#endregion 📖CreateStory

//#region 📖CreateStory
pub fn diff_create_story(payload: &CreateStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.stories.iter().any(|story| story.id == payload.story.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A story with id \"{}\" already exists.", payload.story.id), [payload.story.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.stories.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.stories.len()), [&payload.story.id.to_string()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { inserted: vec![LayoutStoryInsertion { index: payload.index.unwrap_or(base.stories.len()), row: payload.story.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 📖CreateStory

//#region 📖CreateStory
pub fn inverse_create_story<Op: From<crate::delete_story::DeleteStory>>(payload: &CreateStory, _base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_story::DeleteStory { id: payload.story.id.clone() })]

    })())
}
//#endregion 📖CreateStory

