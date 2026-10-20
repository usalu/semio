//! 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`.

use crate::create_story;
use semio_s_artifact_layout_layout_diff::LayoutStoriesDelta;
use semio_s_artifact_layout_layout_diff::LayoutStoryRemoval;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeleteStory
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteStory {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeleteStory where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_story::CreateStory> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "story", kind: "delete-story", record: "DeletedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_story(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete story \"{}\"", self.id), &format!("Textfluss \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStory

//#region 🗑️DeleteStory
pub fn diff_delete_story(payload: &DeleteStory, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.stories.iter().position(|story| story.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff { stories: Some(LayoutStoriesDelta { removed: vec![LayoutStoryRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteStory

//#region 🗑️DeleteStory
pub fn inverse_delete_story<Op: From<crate::create_story::CreateStory>>(payload: &DeleteStory, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.stories.iter().position(|story| story.id == payload.id) {
        Some(index) => vec![Op::from(create_story::CreateStory { story: base.stories[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🗑️DeleteStory

