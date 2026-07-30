//! ⚡ Block 2D app — operation enum + laws (constitutional: op).

use block_2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use block_shared::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

pub type Block2dEnvelope = store::DocumentEnvelope<Block2dDefinition, Block2dOperation>;
pub type Block2dStore = store::DocumentStore<Block2dDefinition, Block2dOperation>;

// #region 🔖Collections
trait Block2dHasId {
    fn id(&self) -> &str;
}
impl Block2dHasId for Block2dHandleKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for Block2dHandleTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩 Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block2dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block2d_collection_diff<T: Block2dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

fn block2d_index_of<T: Block2dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🩹 Sparse block-2d diff over the four id-keyed tables plus the scalar node_kind/presentation/
/// camera2d/meta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dDiff {
    /// 🌍 Whole-document replacement (example load, reset); wins over every field below.
    pub document: Option<Block2dDefinition>,
    pub node_kind: Option<BlockKindIdentity>,
    pub presentation: Option<Block2dPresentation>,
    pub handle_kinds: Block2dCollectionDiff<Block2dHandleKind>,
    pub handles: Block2dCollectionDiff<Block2dHandleTemplate>,
    pub compatibility: Block2dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block2dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera2d: Option<BlockCamera2d>,
    pub meta: Option<BlockMeta>,
}

fn block2d_diff_absorb(diff: &mut Block2dDiff, other: Block2dDiff) {
    if other.document.is_some() {
        *diff = Block2dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.node_kind.is_some() {
        diff.node_kind = other.node_kind;
    }
    if other.presentation.is_some() {
        diff.presentation = other.presentation;
    }
    diff.handle_kinds.removed.extend(other.handle_kinds.removed);
    diff.handle_kinds.set.extend(other.handle_kinds.set);
    diff.handles.removed.extend(other.handles.removed);
    diff.handles.set.extend(other.handles.set);
    diff.compatibility.removed.extend(other.compatibility.removed);
    diff.compatibility.set.extend(other.compatibility.set);
    diff.attributes.removed.extend(other.attributes.removed);
    diff.attributes.set.extend(other.attributes.set);
    if other.authors.is_some() {
        diff.authors = other.authors;
    }
    if other.camera2d.is_some() {
        diff.camera2d = other.camera2d;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Block2dDefinition> for Block2dDiff {
    fn apply(&self, projection: &Block2dDefinition) -> Block2dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(node_kind) = &self.node_kind {
            next.node_kind = node_kind.clone();
        }
        if let Some(presentation) = &self.presentation {
            next.presentation = presentation.clone();
        }
        apply_block2d_collection_diff(&mut next.handle_kinds, &self.handle_kinds.removed, &self.handle_kinds.set);
        apply_block2d_collection_diff(&mut next.handles, &self.handles.removed, &self.handles.set);
        apply_block2d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block2d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
        }
        if let Some(camera2d) = &self.camera2d {
            next.camera2d = camera2d.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        block2d_diff_absorb(self, other);
    }
}

/// 🧮 Block-2d operation: id-keyed table edits plus scalar node_kind/presentation/camera2d/meta, each
/// with a true inverse computed from the pre-operation projection, and a whole-document replace for
/// example loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Block2dOperation {
    #[dsl(key = "setNodeKind")]
    SetNodeKind { #[dsl(block)] node_kind: BlockKindIdentity },
    #[dsl(key = "setPresentation")]
    SetPresentation { #[dsl(block)] presentation: Block2dPresentation },
    #[dsl(key = "setHandleKind")]
    SetHandleKind { index: usize, #[dsl(block)] handle_kind: Block2dHandleKind },
    #[dsl(key = "removeHandleKind")]
    RemoveHandleKind { id: String },
    #[dsl(key = "setHandle")]
    SetHandle { index: usize, #[dsl(block)] handle: Block2dHandleTemplate },
    #[dsl(key = "removeHandle")]
    RemoveHandle { id: String },
    #[dsl(key = "setCompatibilityRule")]
    SetCompatibilityRule { index: usize, #[dsl(block)] rule: BlockCompatibilityRule },
    #[dsl(key = "removeCompatibilityRule")]
    RemoveCompatibilityRule { id: String },
    #[dsl(key = "setAttribute")]
    SetAttribute { index: usize, #[dsl(block)] attribute: BlockAttribute },
    #[dsl(key = "removeAttribute")]
    RemoveAttribute { key: String },
    #[dsl(key = "setAuthors")]
    SetAuthors { authors: Vec<BlockAuthor> },
    #[dsl(key = "setCamera2d")]
    SetCamera2d { #[dsl(block)] camera2d: BlockCamera2d },
    #[dsl(key = "setMeta")]
    SetMeta { #[dsl(block)] meta: BlockMeta },
    /// 🌍 Replaces the whole document (example import / reset).
    #[dsl(key = "setDocument")]
    SetDocument { #[dsl(block)] document: Block2dDefinition },
}

fn block2d_operation_diff(operation: &Block2dOperation) -> Block2dDiff {
    let mut diff = Block2dDiff::default();
    match operation {
        Block2dOperation::SetNodeKind { node_kind } => diff.node_kind = Some(node_kind.clone()),
        Block2dOperation::SetPresentation { presentation } => diff.presentation = Some(presentation.clone()),
        Block2dOperation::SetHandleKind { index, handle_kind } => diff.handle_kinds.set.push((*index, handle_kind.clone())),
        Block2dOperation::RemoveHandleKind { id } => diff.handle_kinds.removed.push(id.clone()),
        Block2dOperation::SetHandle { index, handle } => diff.handles.set.push((*index, handle.clone())),
        Block2dOperation::RemoveHandle { id } => diff.handles.removed.push(id.clone()),
        Block2dOperation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block2dOperation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block2dOperation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block2dOperation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block2dOperation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block2dOperation::SetCamera2d { camera2d } => diff.camera2d = Some(camera2d.clone()),
        Block2dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block2dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Block2dDefinition> for Block2dOperation {
    type Diff = Block2dDiff;

    fn diff(&self, _projection: &Block2dDefinition) -> Block2dDiff {
        block2d_operation_diff(self)
    }

    fn backwards(&self, projection: &Block2dDefinition) -> Vec<Self> {
        match self {
            Block2dOperation::SetNodeKind { .. } => vec![Block2dOperation::SetNodeKind { node_kind: projection.node_kind.clone() }],
            Block2dOperation::SetPresentation { .. } => vec![Block2dOperation::SetPresentation { presentation: projection.presentation.clone() }],
            Block2dOperation::SetHandleKind { handle_kind, .. } => match block2d_index_of(&projection.handle_kinds, &handle_kind.id) {
                Some(index) => vec![Block2dOperation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }],
                None => vec![Block2dOperation::RemoveHandleKind { id: handle_kind.id.clone() }],
            },
            Block2dOperation::RemoveHandleKind { id } => block2d_index_of(&projection.handle_kinds, id).map(|index| vec![Block2dOperation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetHandle { handle, .. } => match block2d_index_of(&projection.handles, &handle.id) {
                Some(index) => vec![Block2dOperation::SetHandle { index, handle: projection.handles[index].clone() }],
                None => vec![Block2dOperation::RemoveHandle { id: handle.id.clone() }],
            },
            Block2dOperation::RemoveHandle { id } => block2d_index_of(&projection.handles, id).map(|index| vec![Block2dOperation::SetHandle { index, handle: projection.handles[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetCompatibilityRule { rule, .. } => match block2d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block2dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block2dOperation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block2dOperation::RemoveCompatibilityRule { id } => block2d_index_of(&projection.compatibility, id).map(|index| vec![Block2dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetAttribute { attribute, .. } => match block2d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block2dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block2dOperation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block2dOperation::RemoveAttribute { key } => block2d_index_of(&projection.attributes, key).map(|index| vec![Block2dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetAuthors { .. } => vec![Block2dOperation::SetAuthors { authors: projection.authors.clone() }],
            Block2dOperation::SetCamera2d { .. } => vec![Block2dOperation::SetCamera2d { camera2d: projection.camera2d.clone() }],
            Block2dOperation::SetMeta { .. } => vec![Block2dOperation::SetMeta { meta: projection.meta.clone() }],
            Block2dOperation::SetDocument { .. } => vec![Block2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖Operations

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_handle_then_remove_round_trips_through_true_inverse() {
        let mut projection = block_2d_engine::empty_block2d_definition();
        let set = Block2dOperation::SetHandle { index: 0, handle: Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: 0.5, radius: 0.36 } };
        let inverse = set.backwards(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.handles.len(), 1);
        assert_eq!(inverse, vec![Block2dOperation::RemoveHandle { id: "h0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, block_2d_engine::empty_block2d_definition());
    }

    #[test]
    fn diff_absorb_collapses_to_latest_set_document() {
        let mut diff = Block2dDiff::default();
        diff.absorb(Block2dDiff { node_kind: Some(BlockKindIdentity::default()), ..Default::default() });
        diff.absorb(Block2dDiff { document: Some(block_2d_engine::empty_block2d_definition()), ..Default::default() });
        assert!(diff.document.is_some());
        assert!(diff.node_kind.is_none());
    }
}
//#endregion 🧪Tests
