//! ⚡ Note app — operation enum + laws (constitutional: op).

use note::{NoteBlockNode, NoteCamera, NoteDocument, NoteImageAsset};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖Types
/// 📐 Typed content mutation for a `NoteDocument`. Every content change flows through one of these so
/// the `DocumentStore` records a true inverse (`backwards`). Scalar setters carry the field's own
/// `Option` shape (backwards is a plain prior-value read); block edits use a whole-tree `SetBlocks`
/// snapshot (the recursive reid/clone tree makes per-node operations far messier than a snapshot); asset and
/// full-document loads have dedicated variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum NoteOperation {
    SetCamera {
        #[dsl(block)]
        camera: NoteCamera,
    },
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
    SetDocument {
        #[dsl(block)]
        document: NoteDocument,
    },
}

/// 🧩 Snapshot diff wrapping the forward `NoteOperation` — `apply` replays it, `absorb` keeps the latest
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
        NoteOperation::SetCamera { camera } => next.camera = camera.clone(),
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
            NoteOperation::SetCamera { .. } => vec![NoteOperation::SetCamera { camera: projection.camera.clone() }],
            NoteOperation::SetGridVisible { .. } => vec![NoteOperation::SetGridVisible { visible: projection.grid_visible }],
            NoteOperation::SetGridSpacing { .. } => vec![NoteOperation::SetGridSpacing { spacing: projection.grid_spacing }],
            NoteOperation::SetGridSubdivisions { .. } => vec![NoteOperation::SetGridSubdivisions { value: projection.grid_subdivisions }],
            NoteOperation::SetGridOpacity { .. } => vec![NoteOperation::SetGridOpacity { opacity: projection.grid_opacity }],
            NoteOperation::SetSnapEnabled { .. } => vec![NoteOperation::SetSnapEnabled { enabled: projection.snap_enabled }],
            NoteOperation::SetSnapGridSpacing { .. } => vec![NoteOperation::SetSnapGridSpacing { spacing: projection.snap_grid_spacing }],
            NoteOperation::SetPencilWidth { .. } => vec![NoteOperation::SetPencilWidth { width: projection.pencil_width }],
            NoteOperation::SetEraserRadius { .. } => vec![NoteOperation::SetEraserRadius { radius: projection.eraser_radius }],
            NoteOperation::SetBlocks { .. } => vec![NoteOperation::SetBlocks { blocks: projection.blocks.clone() }],
            NoteOperation::PutAsset { .. } => vec![NoteOperation::SetDocument { document: projection.clone() }],
            NoteOperation::SetDocument { .. } => vec![NoteOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖Types

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_text_round_trips_every_variant() {
        use note::{NoteBlockNode, NoteCamera, NoteImageAsset};

        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetCamera { camera: NoteCamera { x: 1.0, y: -2.5, zoom: 2.0 } });
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

        store::test_support::assert_op_text_binary_equivalence(&NoteOperation::SetDocument { document: note_engine::empty_note_document() });
    }

    #[test]
    fn operation_backwards_restores_pre_state() {
        let pre = note_engine::empty_note_document();
        store::test_support::assert_operation_round_trip(&pre, NoteOperation::SetGridSpacing { spacing: Some(48.0) });
    }
}
//#endregion 🧪Tests
