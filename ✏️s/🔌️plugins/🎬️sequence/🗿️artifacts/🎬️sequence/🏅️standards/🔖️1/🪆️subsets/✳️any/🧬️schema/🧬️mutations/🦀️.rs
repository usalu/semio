//! 🎬️ Sequence parent mutation vocabulary — empty by design (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12,
//! §20.15): a sequence's steps and edges live in its composed `content` child (`s.stdio.semio@v1/flow`), so every content
//! edit is a child-lane leaf in that child's store (`drag-nodes`, `insert-node`, `set-node-param`, …) and the parent owns no
//! leaf that could read the child. Editors publish those child leaves (`crate::editor::sequence::edit_rules::SceneEdit`).

use crate::diff::SequenceDiff;
use crate::SequenceSnapshot;

pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🕳️ The uninhabited parent vocabulary of a document whose whole content is its composed child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
pub enum SequenceMutation {}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for SequenceMutation {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        match *self {}
    }
}

impl protocol::Mutation<SequenceSnapshot> for SequenceMutation {
    type Diff = SequenceDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }
    fn diff(&self, _base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        match *self {}
    }
    fn inverse(&self, _base: &SequenceSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match *self {}
    
    })())
}
}

/// 🏷️ No parent kind exists, so no parent operation is ever labelled; child leaves label their own rows.
impl protocol::SemanticMutation<SequenceSnapshot> for SequenceMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}





/// 🧊️ Nothing to retire: the vocabulary is uninhabited.
impl neural_engine::ColdRetire for SequenceMutation {
    fn retire_cold(self) {
        match self {}
    }
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
