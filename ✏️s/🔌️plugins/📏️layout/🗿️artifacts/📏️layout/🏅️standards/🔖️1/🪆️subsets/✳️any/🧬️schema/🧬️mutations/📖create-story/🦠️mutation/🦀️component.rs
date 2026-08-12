//! 📖 `create-story` — brings a new {@link TextStory} into existence in the id-keyed `stories`
//! collection.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, TextStory};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 📖CreateStory
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStory {
    pub story: TextStory,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateStory {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "story", kind: "create-story", record: "CreatedStory" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_create_story(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_create_story(self, base)
    }
    fn label(&self) -> String {
        format!("Create story \"{}\"", self.story.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.story.id.clone()]
    }
}
//#endregion 📖CreateStory
