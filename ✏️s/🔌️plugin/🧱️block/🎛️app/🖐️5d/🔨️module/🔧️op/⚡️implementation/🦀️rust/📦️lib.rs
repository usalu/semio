//! ⚡️ Block 5D app — operation enum + laws (constitutional: op).

use block_5d::{Block5dDefinition, Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
use block_shared::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

pub type Block5dEnvelope = store::DocumentEnvelope<Block5dDefinition, Block5dOperation>;
pub type Block5dStore = store::DocumentStore<Block5dDefinition, Block5dOperation>;

// #region 🔖️Collections
trait Block5dHasId {
    fn id(&self) -> &str;
}
impl Block5dHasId for BlockRepresentation {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for Block5dGripKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for Block5dGripTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block5dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩️ Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block5dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block5d_collection_diff<T: Block5dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

fn block5d_index_of<T: Block5dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Operations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block5dDiff {
    pub document: Option<Block5dDefinition>,
    pub part_kind: Option<BlockKindIdentity>,
    pub part_2d: Option<Block5dPart2d>,
    pub part_3d: Option<Block5dPart3d>,
    pub representations: Block5dCollectionDiff<BlockRepresentation>,
    pub grip_kinds: Block5dCollectionDiff<Block5dGripKind>,
    pub grips: Block5dCollectionDiff<Block5dGripTemplate>,
    pub compatibility: Block5dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block5dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera2d: Option<BlockCamera2d>,
    pub camera3d: Option<BlockCamera3d>,
    pub meta: Option<BlockMeta>,
}

fn block5d_diff_absorb(diff: &mut Block5dDiff, other: Block5dDiff) {
    if other.document.is_some() {
        *diff = Block5dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.part_kind.is_some() {
        diff.part_kind = other.part_kind;
    }
    if other.part_2d.is_some() {
        diff.part_2d = other.part_2d;
    }
    if other.part_3d.is_some() {
        diff.part_3d = other.part_3d;
    }
    diff.representations.removed.extend(other.representations.removed);
    diff.representations.set.extend(other.representations.set);
    diff.grip_kinds.removed.extend(other.grip_kinds.removed);
    diff.grip_kinds.set.extend(other.grip_kinds.set);
    diff.grips.removed.extend(other.grips.removed);
    diff.grips.set.extend(other.grips.set);
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
    if other.camera3d.is_some() {
        diff.camera3d = other.camera3d;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Block5dDefinition> for Block5dDiff {
    fn apply(&self, projection: &Block5dDefinition) -> Block5dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(part_kind) = &self.part_kind {
            next.part_kind = part_kind.clone();
        }
        if let Some(part_2d) = &self.part_2d {
            next.part_2d = part_2d.clone();
        }
        if let Some(part_3d) = &self.part_3d {
            next.part_3d = part_3d.clone();
        }
        apply_block5d_collection_diff(&mut next.representations, &self.representations.removed, &self.representations.set);
        apply_block5d_collection_diff(&mut next.grip_kinds, &self.grip_kinds.removed, &self.grip_kinds.set);
        apply_block5d_collection_diff(&mut next.grips, &self.grips.removed, &self.grips.set);
        apply_block5d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block5d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
        }
        if let Some(camera2d) = &self.camera2d {
            next.camera2d = camera2d.clone();
        }
        if let Some(camera3d) = &self.camera3d {
            next.camera3d = camera3d.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        block5d_diff_absorb(self, other);
    }
}

/// 🧮️ Block-5d operation: id-keyed table edits plus scalar part_kind/part_2d/part_3d/camera2d/
/// camera3d/meta, each with a true inverse computed from the pre-operation projection, and a
/// whole-document replace for example loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Block5dOperation {
    #[dsl(key = "setPartKind")]
    SetPartKind { #[dsl(block)] part_kind: BlockKindIdentity },
    #[dsl(key = "setPart2d")]
    SetPart2d { #[dsl(block)] part_2d: Block5dPart2d },
    #[dsl(key = "setPart3d")]
    SetPart3d { #[dsl(block)] part_3d: Block5dPart3d },
    #[dsl(key = "setRepresentation")]
    SetRepresentation { index: usize, #[dsl(block)] representation: BlockRepresentation },
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "setGripKind")]
    SetGripKind { index: usize, #[dsl(block)] grip_kind: Block5dGripKind },
    #[dsl(key = "removeGripKind")]
    RemoveGripKind { id: String },
    #[dsl(key = "setGrip")]
    SetGrip { index: usize, #[dsl(block)] grip: Block5dGripTemplate },
    #[dsl(key = "removeGrip")]
    RemoveGrip { id: String },
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
    #[dsl(key = "setCamera3d")]
    SetCamera3d { #[dsl(block)] camera3d: BlockCamera3d },
    #[dsl(key = "setMeta")]
    SetMeta { #[dsl(block)] meta: BlockMeta },
    #[dsl(key = "setDocument")]
    SetDocument { #[dsl(block)] document: Block5dDefinition },
}

fn block5d_operation_diff(operation: &Block5dOperation) -> Block5dDiff {
    let mut diff = Block5dDiff::default();
    match operation {
        Block5dOperation::SetPartKind { part_kind } => diff.part_kind = Some(part_kind.clone()),
        Block5dOperation::SetPart2d { part_2d } => diff.part_2d = Some(part_2d.clone()),
        Block5dOperation::SetPart3d { part_3d } => diff.part_3d = Some(part_3d.clone()),
        Block5dOperation::SetRepresentation { index, representation } => diff.representations.set.push((*index, representation.clone())),
        Block5dOperation::RemoveRepresentation { id } => diff.representations.removed.push(id.clone()),
        Block5dOperation::SetGripKind { index, grip_kind } => diff.grip_kinds.set.push((*index, grip_kind.clone())),
        Block5dOperation::RemoveGripKind { id } => diff.grip_kinds.removed.push(id.clone()),
        Block5dOperation::SetGrip { index, grip } => diff.grips.set.push((*index, grip.clone())),
        Block5dOperation::RemoveGrip { id } => diff.grips.removed.push(id.clone()),
        Block5dOperation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block5dOperation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block5dOperation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block5dOperation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block5dOperation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block5dOperation::SetCamera2d { camera2d } => diff.camera2d = Some(camera2d.clone()),
        Block5dOperation::SetCamera3d { camera3d } => diff.camera3d = Some(camera3d.clone()),
        Block5dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block5dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Block5dDefinition> for Block5dOperation {
    type Diff = Block5dDiff;

    fn diff(&self, _projection: &Block5dDefinition) -> Block5dDiff {
        block5d_operation_diff(self)
    }

    fn backwards(&self, projection: &Block5dDefinition) -> Vec<Self> {
        match self {
            Block5dOperation::SetPartKind { .. } => vec![Block5dOperation::SetPartKind { part_kind: projection.part_kind.clone() }],
            Block5dOperation::SetPart2d { .. } => vec![Block5dOperation::SetPart2d { part_2d: projection.part_2d.clone() }],
            Block5dOperation::SetPart3d { .. } => vec![Block5dOperation::SetPart3d { part_3d: projection.part_3d.clone() }],
            Block5dOperation::SetRepresentation { representation, .. } => match block5d_index_of(&projection.representations, &representation.id) {
                Some(index) => vec![Block5dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }],
                None => vec![Block5dOperation::RemoveRepresentation { id: representation.id.clone() }],
            },
            Block5dOperation::RemoveRepresentation { id } => block5d_index_of(&projection.representations, id).map(|index| vec![Block5dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetGripKind { grip_kind, .. } => match block5d_index_of(&projection.grip_kinds, &grip_kind.id) {
                Some(index) => vec![Block5dOperation::SetGripKind { index, grip_kind: projection.grip_kinds[index].clone() }],
                None => vec![Block5dOperation::RemoveGripKind { id: grip_kind.id.clone() }],
            },
            Block5dOperation::RemoveGripKind { id } => block5d_index_of(&projection.grip_kinds, id).map(|index| vec![Block5dOperation::SetGripKind { index, grip_kind: projection.grip_kinds[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetGrip { grip, .. } => match block5d_index_of(&projection.grips, &grip.id) {
                Some(index) => vec![Block5dOperation::SetGrip { index, grip: projection.grips[index].clone() }],
                None => vec![Block5dOperation::RemoveGrip { id: grip.id.clone() }],
            },
            Block5dOperation::RemoveGrip { id } => block5d_index_of(&projection.grips, id).map(|index| vec![Block5dOperation::SetGrip { index, grip: projection.grips[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetCompatibilityRule { rule, .. } => match block5d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block5dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block5dOperation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block5dOperation::RemoveCompatibilityRule { id } => block5d_index_of(&projection.compatibility, id).map(|index| vec![Block5dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetAttribute { attribute, .. } => match block5d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block5dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block5dOperation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block5dOperation::RemoveAttribute { key } => block5d_index_of(&projection.attributes, key).map(|index| vec![Block5dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetAuthors { .. } => vec![Block5dOperation::SetAuthors { authors: projection.authors.clone() }],
            Block5dOperation::SetCamera2d { .. } => vec![Block5dOperation::SetCamera2d { camera2d: projection.camera2d.clone() }],
            Block5dOperation::SetCamera3d { .. } => vec![Block5dOperation::SetCamera3d { camera3d: projection.camera3d.clone() }],
            Block5dOperation::SetMeta { .. } => vec![Block5dOperation::SetMeta { meta: projection.meta.clone() }],
            Block5dOperation::SetDocument { .. } => vec![Block5dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖️Operations

//#region 🔖️ConfigOperations
/// 🧮️ `block_5d_engine::Block5dConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `Block5dPlayApp` `RefCell` field write), plus a generic `Snapshot` every
/// variant's `backwards()` returns — same "whole-config-snapshot inverse" shape
/// `shooting_op::ShootingConfigOperation` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block5dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot { #[dsl(block)] config: block_5d_engine::Block5dConfig },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<block_5d_engine::Block5dConfig> for Block5dConfigOperation {
    type Diff = block_5d_engine::Block5dConfig;

    fn diff(&self, base: &block_5d_engine::Block5dConfig) -> block_5d_engine::Block5dConfig {
        let mut next = base.clone();
        match self {
            Block5dConfigOperation::Snapshot { config } => return config.clone(),
            Block5dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block5dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &block_5d_engine::Block5dConfig) -> Vec<Self> {
        vec![Block5dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_grip_then_remove_round_trips_through_true_inverse() {
        let mut projection = block_5d_engine::empty_block5d_definition();
        let set = Block5dOperation::SetGrip { index: 0, grip: Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -0.1, radius_2d: 3.0, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 } };
        let inverse = set.backwards(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.grips.len(), 1);
        assert_eq!(inverse, vec![Block5dOperation::RemoveGrip { id: "g0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, block_5d_engine::empty_block5d_definition());
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = block_5d_engine::Block5dConfig::default();
        let operation = Block5dConfigOperation::SetSelection { ids: vec!["g0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["g0".to_string()]);
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Block5dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(inverse[0].diff(&next), base);
    }
}
//#endregion 🧪️Tests
