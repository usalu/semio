//! ⚡️ Note app — operation enum + laws (constitutional: op).

use note::{NoteBlockNode, NoteCamera, NoteDocument, NoteImageAsset};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// 📐️ Typed content mutation for a `NoteDocument`. Every content change flows through one of these so
/// the `DocumentStore` records a true inverse (`backwards`). Scalar setters carry the field's own
/// `Option` shape (backwards is a plain prior-value read); block edits use a whole-tree `SetBlocks`
/// snapshot (the recursive reid/clone tree makes per-node operations far messier than a snapshot); asset and
/// full-document loads have dedicated variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum NoteOperation {
    SetGridVisible { visible: Option<bool> },
    SetGridSpacing { spacing: Option<f64> },
    SetGridSubdivisions { value: Option<f64> },
    SetGridOpacity { opacity: Option<f64> },
    SetSnapEnabled { enabled: Option<bool> },
    SetSnapGridSpacing { spacing: Option<f64> },
    SetPencilWidth { width: Option<f64> },
    SetEraserRadius { radius: Option<f64> },
    SetBlocks {
        #[dsl(statements, block)]
        blocks: Vec<NoteBlockNode>,
    },
    PutAsset {
        key: String,
        #[dsl(block)]
        asset: NoteImageAsset,
    },
    /// 🗑️ True composed inverse of a `PutAsset` that introduced the key — the whole-document
    /// `SetDocument` snapshot this used to invert to would have undone every other pending change too.
    RemoveAsset { key: String },
    SetDocument {
        #[dsl(block)]
        document: NoteDocument,
    }
}

/// 🧩️ Snapshot diff wrapping the forward `NoteOperation` — `apply` replays it, `absorb` keeps the latest
/// (coalescing a whole gesture's `SetBlocks` stream into one edit).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NoteDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<NoteOperation>,
}

impl OperationDiff<NoteDocument> for NoteDiff {
    fn apply(&self, projection: &NoteDocument) -> NoteDocument {
        match &self.operation {
            Some(operation) => apply_note_operation(projection, operation),
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.operation.is_some() {
            self.operation = other.operation;
        }
    }
}

fn apply_note_operation(projection: &NoteDocument, operation: &NoteOperation) -> NoteDocument {
    let mut next = projection.clone();
    match operation {
        NoteOperation::SetGridVisible { visible } => next.grid_visible = *visible,
        NoteOperation::SetGridSpacing { spacing } => next.grid_spacing = *spacing,
        NoteOperation::SetGridSubdivisions { value } => next.grid_subdivisions = *value,
        NoteOperation::SetGridOpacity { opacity } => next.grid_opacity = *opacity,
        NoteOperation::SetSnapEnabled { enabled } => next.snap_enabled = *enabled,
        NoteOperation::SetSnapGridSpacing { spacing } => next.snap_grid_spacing = *spacing,
        NoteOperation::SetPencilWidth { width } => next.pencil_width = *width,
        NoteOperation::SetEraserRadius { radius } => next.eraser_radius = *radius,
        NoteOperation::SetBlocks { blocks } => next.blocks = blocks.clone(),
        NoteOperation::PutAsset { key, asset } => {
            next.assets.insert(key.clone(), asset.clone());
        }
        NoteOperation::RemoveAsset { key } => {
            next.assets.remove(key);
        }
        NoteOperation::SetDocument { document } => next = document.clone(),
    }
    next
}

impl Operation<NoteDocument> for NoteOperation {
    type Diff = NoteDiff;

    fn diff(&self, _projection: &NoteDocument) -> NoteDiff {
        NoteDiff { operation: Some(self.clone()) }
    }

    fn backwards(&self, projection: &NoteDocument) -> Vec<Self> {
        match self {
            NoteOperation::SetGridVisible { .. } => vec![NoteOperation::SetGridVisible { visible: projection.grid_visible }],
            NoteOperation::SetGridSpacing { .. } => vec![NoteOperation::SetGridSpacing { spacing: projection.grid_spacing }],
            NoteOperation::SetGridSubdivisions { .. } => vec![NoteOperation::SetGridSubdivisions { value: projection.grid_subdivisions }],
            NoteOperation::SetGridOpacity { .. } => vec![NoteOperation::SetGridOpacity { opacity: projection.grid_opacity }],
            NoteOperation::SetSnapEnabled { .. } => vec![NoteOperation::SetSnapEnabled { enabled: projection.snap_enabled }],
            NoteOperation::SetSnapGridSpacing { .. } => vec![NoteOperation::SetSnapGridSpacing { spacing: projection.snap_grid_spacing }],
            NoteOperation::SetPencilWidth { .. } => vec![NoteOperation::SetPencilWidth { width: projection.pencil_width }],
            NoteOperation::SetEraserRadius { .. } => vec![NoteOperation::SetEraserRadius { radius: projection.eraser_radius }],
            NoteOperation::SetBlocks { .. } => vec![NoteOperation::SetBlocks { blocks: projection.blocks.clone() }],
            // 🗑️ Composed, not a snapshot: restores the prior value at `key` if one existed, else removes
            // the key that didn't exist before this `PutAsset` — every other asset/field is untouched.
            NoteOperation::PutAsset { key, .. } => match projection.assets.get(key) {
                Some(prior) => vec![NoteOperation::PutAsset { key: key.clone(), asset: prior.clone() }],
                None => vec![NoteOperation::RemoveAsset { key: key.clone() }],
            },
            NoteOperation::RemoveAsset { key } => match projection.assets.get(key) {
                Some(prior) => vec![NoteOperation::PutAsset { key: key.clone(), asset: prior.clone() }],
                // Removing a key that was already absent is a no-op — nothing to restore.
                None => Vec::new(),
            },
            NoteOperation::SetDocument { .. } => vec![NoteOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `note_engine::NoteConfig`'s operation enum — mirrors `shooting_op::ShootingConfigOperation`'s
/// pilot shape exactly: one variant per settled interaction (the pre-migration `NotePlayRuntime` field
/// writes), plus a generic `Snapshot` every variant's `backwards()` returns — since a config-only "View"
/// dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct, real config edit, and
/// "undo this tick" is exactly "restore the whole-config snapshot from just before it".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NoteConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: note_engine::NoteConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { block_ids: Vec<String> },
    #[dsl(key = "hovered-block")]
    SetHoveredBlock { block_id: Option<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: NoteCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<note_engine::NoteConfig> for NoteConfigOperation {
    type Diff = note_engine::NoteConfig;

    fn diff(&self, base: &note_engine::NoteConfig) -> note_engine::NoteConfig {
        let mut next = base.clone();
        match self {
            NoteConfigOperation::Snapshot { config } => return config.clone(),
            NoteConfigOperation::SetSelection { block_ids } => next.selected_block_ids = block_ids.clone(),
            NoteConfigOperation::SetHoveredBlock { block_id } => next.hovered_block_id = block_id.clone(),
            NoteConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            NoteConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            NoteConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            NoteConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &note_engine::NoteConfig) -> Vec<Self> {
        vec![NoteConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_text_round_trips_every_variant() {
        use note::{NoteBlockNode, NoteImageAsset};

        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridVisible { visible: Some(true) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridVisible { visible: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridSpacing { spacing: Some(16.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridSpacing { spacing: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridSubdivisions { value: Some(8.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetGridOpacity { opacity: Some(0.6) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetSnapEnabled { enabled: Some(false) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetSnapGridSpacing { spacing: Some(4.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetPencilWidth { width: Some(5.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetEraserRadius { radius: Some(20.0) });

        let stroke_with_points = NoteBlockNode::Ink {
            id: "stroke-1".into(),
            name: "Ink".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: vec![[0.0, 0.0], [1.5, 2.5], [3.0, -1.0]],
            stroke_width: 3.0,
            color: [0.0, 0.0, 0.0, 1.0],
        };
        let text_block = NoteBlockNode::Text {
            id: "text-1".into(),
            name: "Text".into(),
            x: 5.0,
            y: 5.0,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: Vec::new(),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetBlocks { blocks: vec![text_block, stroke_with_points] });

        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::PutAsset {
            key: "asset-2".into(),
            asset: NoteImageAsset { mime: "image/jpeg".into(), data: "data:image/jpeg;base64,xyz".into(), width: None, height: None },
        });
        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::RemoveAsset { key: "asset-2".into() });

        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetDocument { document: note_engine::empty_note_document() });
    }

    #[test]
    fn operation_backwards_restores_pre_state() {
        let pre = note_engine::empty_note_document();
        store::test_support::assert_operation_round_trip(&pre, NoteOperation::SetGridSpacing { spacing: Some(48.0) });
    }

    /// 🗑️ `PutAsset`'s inverse must be composed (touch only the one key), not a whole-document
    /// `SetDocument` snapshot that would also clobber every other pending change.
    #[test]
    fn put_asset_backwards_is_composed_not_a_whole_document_snapshot() {
        use note::NoteImageAsset;

        let mut pre = note_engine::empty_note_document();
        pre.grid_spacing = Some(99.0); // an unrelated field that a snapshot inverse would wrongly revert too
        let asset = NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: None, height: None };

        // Introducing a brand-new key inverts to removing it.
        let put_new = NoteOperation::PutAsset { key: "asset-1".into(), asset: asset.clone() };
        let inverse = put_new.backwards(&pre);
        assert_eq!(inverse, vec![NoteOperation::RemoveAsset { key: "asset-1".into() }]);
        let mut after_put = apply_note_operation(&pre, &put_new);
        assert_eq!(after_put.assets.get("asset-1"), Some(&asset));
        for op in &inverse {
            after_put = apply_note_operation(&after_put, op);
        }
        assert_eq!(after_put, pre, "undoing a fresh PutAsset must restore exactly the pre-state, unrelated fields included");

        // Replacing an existing key inverts to restoring the prior asset at that key.
        pre.assets.insert("asset-1".into(), asset.clone());
        let replacement = NoteImageAsset { mime: "image/jpeg".into(), data: "data:image/jpeg;base64,def".into(), width: None, height: None };
        let put_replace = NoteOperation::PutAsset { key: "asset-1".into(), asset: replacement.clone() };
        let inverse = put_replace.backwards(&pre);
        assert_eq!(inverse, vec![NoteOperation::PutAsset { key: "asset-1".into(), asset: asset.clone() }]);
        let mut after_replace = apply_note_operation(&pre, &put_replace);
        assert_eq!(after_replace.assets.get("asset-1"), Some(&replacement));
        for op in &inverse {
            after_replace = apply_note_operation(&after_replace, op);
        }
        assert_eq!(after_replace, pre);

        // Removing a key inverts to restoring it.
        let remove = NoteOperation::RemoveAsset { key: "asset-1".into() };
        let inverse = remove.backwards(&pre);
        assert_eq!(inverse, vec![NoteOperation::PutAsset { key: "asset-1".into(), asset }]);

        // Removing an already-absent key is a no-op with a no-op (empty) inverse.
        let remove_missing = NoteOperation::RemoveAsset { key: "never-existed".into() };
        assert_eq!(remove_missing.backwards(&pre), Vec::new());
    }

    /// 🧮️ B1 ConfigOperation OpText/OpBinary round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE),
    /// one assertion per `NoteConfigOperation` variant.
    #[test]
    fn note_config_operation_text_and_binary_round_trip_every_variant() {
        let config = note_engine::NoteConfig {
            selected_block_ids: vec!["text-1".into()],
            hovered_block_id: Some("image-2".into()),
            engagement_input: "Renaming…".into(),
            camera: NoteCamera { x: 3.0, y: -1.5, zoom: 1.75 },
            active_utility_id: "pencil".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::Snapshot { config });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetSelection { block_ids: vec!["text-1".into(), "table-2".into()] });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetHoveredBlock { block_id: Some("image-2".into()) });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetHoveredBlock { block_id: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetEngagementInput { value: "Renaming…".into() });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetActiveUtility { utility_id: "eraserStroke".into() });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// 🧮️ Every `NoteConfigOperation`'s `backwards()` is the whole-config snapshot from just before it —
    /// mirrors `shooting_op`'s analogous coverage.
    #[test]
    fn note_config_operation_backwards_is_always_a_snapshot_of_the_prior_config() {
        let base = note_engine::NoteConfig::default();
        let operation = NoteConfigOperation::SetActiveUtility { utility_id: "pencil".into() };
        assert_eq!(operation.backwards(&base), vec![NoteConfigOperation::Snapshot { config: base.clone() }]);
        let next = operation.diff(&base);
        assert_eq!(next.active_utility_id, "pencil");
        let restored = NoteConfigOperation::Snapshot { config: base.clone() }.diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
