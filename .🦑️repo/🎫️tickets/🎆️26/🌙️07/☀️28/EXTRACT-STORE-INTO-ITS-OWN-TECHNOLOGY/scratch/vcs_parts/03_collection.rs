//#region 🔖️CollectionDiff
/// @emoji 🧩️ Sparse collection patch entry (mirrors compose `XModified`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩️ Sparse collection diff (mirrors compose `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self {
            removed: Vec::new(),
            modified: Vec::new(),
            added: Vec::new(),
        }
    }
}
//#endregion 🔖️CollectionDiff

//#region 🔖️CollectionOperation
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// @emoji 🩹️ Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch> {
    fn apply_patch(&mut self, patch: &TPatch) -> TPatch;
}

/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOperation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionOperation` to a `Vec` in place.
pub fn apply_collection_operation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionOperation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionOperation::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionOperation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionOperation::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionOperation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionOperation` from the pre-state `items`. Panics if `operation` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub fn invert_collection_operation<TId, TItem, TPatch>(
    items: &[TItem],
    operation: &CollectionOperation<TId, TItem, TPatch>,
) -> CollectionOperation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionOperation::Add { item, .. } => CollectionOperation::Remove { id: item.id().clone() },
        CollectionOperation::Remove { id } => {
            let index = items
                .iter()
                .position(|item| item.id() == id)
                .expect("remove target must exist in pre-state");
            CollectionOperation::Add {
                index,
                item: items[index].clone(),
            }
        }
        CollectionOperation::Move { id, .. } => {
            let index = items
                .iter()
                .position(|item| item.id() == id)
                .expect("move target must exist in pre-state");
            CollectionOperation::Move {
                id: id.clone(),
                to_index: index,
            }
        }
        CollectionOperation::Patch { id, patch } => {
            let mut prior = items
                .iter()
                .find(|item| item.id() == id)
                .cloned()
                .expect("patch target must exist in pre-state");
            let inverse_patch = prior.apply_patch(patch);
            CollectionOperation::Patch {
                id: id.clone(),
                patch: inverse_patch,
            }
        }
    }
}

/// @emoji 🧮️ Projects a `CollectionOperation` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Operation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub fn collection_diff_from_operation<TId, TItem, TPatch>(
    items: &[TItem],
    operation: &CollectionOperation<TId, TItem, TPatch>,
) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionOperation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionOperation::Remove { id } => diff.removed.push(id.clone()),
        CollectionOperation::Patch { id, patch } => diff.modified.push(ItemPatch {
            id: id.clone(),
            patch: patch.clone(),
        }),
        CollectionOperation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖️CollectionOperation
