//! ⚖️ Layout artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpText`/`protocol::OpBinary for LayoutOperation` would normally come for free from
//! `#[derive(dsl::DslOps)]`, but its `Pages`/`Stories`/`Links` variants wrap a foreign generic type
//! (`protocol::CollectionOperation<K,V,P>`, orphan rule: can't gain a `dsl::DslField`/`dsl::DslVariants`
//! binding here) and `FramePatch.fill`/`.stroke` (`Option<Option<[f32;4]>>`) has the same "no direct
//! binding" issue one level down. `LayoutOperationDsl` is a local, DSL-only mirror that flattens each
//! collection wrapper into its own keyworded variants and `FramePatchDsl`/`ColorPatch` fix the doubly-
//! optional color fields, mirroring `process_3d::Process3dOperationDsl`'s identical fix for the same
//! foreign-type problem. Both hand-written impls below convert at the `OpText`/`OpBinary` boundary only;
//! `LayoutOperation` itself (and every consumer matching on it) is untouched.
//!
//! The app's typed `LayoutCommand` enum — which used to share the old `📡️protocol` crate with this codec
//! — is an APP concern, not an artifact one: it now lives in `🎛️apps/📏️layout/🦀️component.rs`, assembled
//! from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::layout::op::LayoutOperation;
use crate::artifacts::layout::{Frame, FramePatch, ImageLink, ImageLinkPatch, Page, PagePatch, TextStory, TextStoryPatch};
use protocol::{CollectionOperation, OpBinary, OpText};
use store::TextError;

/// 📦️ Encodes a `LayoutOperation` to its binary state-patch form.
pub fn encode_op(operation: &LayoutOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutOperation, protocol::ProtocolError> {
    LayoutOperation::decode_op(bytes)
}

//#region 🔖️DslMirror
/// 🎨️ 3-state tag standing in for `FramePatch.fill`/`.stroke`'s `Option<Option<[f32;4]>>` — the DSL
/// engine's plain `Option<T>` can only express "untouched vs present"; `Clear` carries the doubly-
/// optional field's "explicitly cleared to none" state that a single `Option` can't.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ColorPatch {
    Clear,
    Set { color: [f32; 4] },
}

/// 🩹️ DSL-only mirror of `FramePatch` — only `fill`/`stroke` differ from the real type (see
/// `ColorPatch`); every other field passes through unchanged. Never fixture-visible (only ever
/// appears inside a `patchFrame` op line), so its own shape has no compatibility obligation beyond
/// its own round trip.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct FramePatchDsl {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    #[dsl(statements, block)]
    fill: Option<ColorPatch>,
    #[dsl(statements, block)]
    stroke: Option<ColorPatch>,
    wrap_mode: Option<String>,
    columns: Option<u32>,
}

fn frame_patch_to_dsl(patch: &FramePatch) -> FramePatchDsl {
    FramePatchDsl {
        x: patch.x,
        y: patch.y,
        width: patch.width,
        height: patch.height,
        fill: match patch.fill {
            None => None,
            Some(None) => Some(ColorPatch::Clear),
            Some(Some(color)) => Some(ColorPatch::Set { color }),
        },
        stroke: match patch.stroke {
            None => None,
            Some(None) => Some(ColorPatch::Clear),
            Some(Some(color)) => Some(ColorPatch::Set { color }),
        },
        wrap_mode: patch.wrap_mode.clone(),
        columns: patch.columns,
    }
}

fn frame_patch_from_dsl(patch: FramePatchDsl) -> FramePatch {
    FramePatch {
        x: patch.x,
        y: patch.y,
        width: patch.width,
        height: patch.height,
        fill: match patch.fill {
            None => None,
            Some(ColorPatch::Clear) => Some(None),
            Some(ColorPatch::Set { color }) => Some(Some(color)),
        },
        stroke: match patch.stroke {
            None => None,
            Some(ColorPatch::Clear) => Some(None),
            Some(ColorPatch::Set { color }) => Some(Some(color)),
        },
        wrap_mode: patch.wrap_mode,
        columns: patch.columns,
    }
}

/// ⚡️ DSL-only mirror of `LayoutOperation` — see this module's opening doc comment.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum LayoutOperationDsl {
    PagesAdd {
        index: usize,
        #[dsl(block)]
        item: Page,
    },
    PagesRemove {
        id: String,
    },
    PagesMove {
        id: String,
        to_index: usize,
    },
    PagesPatch {
        id: String,
        #[dsl(block)]
        patch: PagePatch,
    },
    StoriesAdd {
        index: usize,
        #[dsl(block)]
        item: TextStory,
    },
    StoriesRemove {
        id: String,
    },
    StoriesMove {
        id: String,
        to_index: usize,
    },
    StoriesPatch {
        id: String,
        #[dsl(block)]
        patch: TextStoryPatch,
    },
    LinksAdd {
        index: usize,
        #[dsl(block)]
        item: ImageLink,
    },
    LinksRemove {
        id: String,
    },
    LinksMove {
        id: String,
        to_index: usize,
    },
    LinksPatch {
        id: String,
        #[dsl(block)]
        patch: ImageLinkPatch,
    },
    AddFrame {
        page_id: String,
        index: usize,
        #[dsl(statements)]
        frame: Box<Frame>,
        layer_id: Option<String>,
    },
    RemoveFrame {
        page_id: String,
        frame_id: String,
    },
    PatchFrame {
        page_id: String,
        frame_id: String,
        #[dsl(block)]
        patch: FramePatchDsl,
    },
    #[dsl(key = "data-fields")]
    SetDataFields {
        json: Option<String>,
    },
}

fn layout_operation_to_dsl(operation: &LayoutOperation) -> LayoutOperationDsl {
    match operation {
        LayoutOperation::Pages(CollectionOperation::Add { index: at, item }) => LayoutOperationDsl::PagesAdd { index: *at, item: item.clone() },
        LayoutOperation::Pages(CollectionOperation::Remove { id }) => LayoutOperationDsl::PagesRemove { id: id.clone() },
        LayoutOperation::Pages(CollectionOperation::Move { id, to_index: to }) => LayoutOperationDsl::PagesMove { id: id.clone(), to_index: *to },
        LayoutOperation::Pages(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::PagesPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::Stories(CollectionOperation::Add { index: at, item }) => LayoutOperationDsl::StoriesAdd { index: *at, item: item.clone() },
        LayoutOperation::Stories(CollectionOperation::Remove { id }) => LayoutOperationDsl::StoriesRemove { id: id.clone() },
        LayoutOperation::Stories(CollectionOperation::Move { id, to_index: to }) => LayoutOperationDsl::StoriesMove { id: id.clone(), to_index: *to },
        LayoutOperation::Stories(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::StoriesPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::Links(CollectionOperation::Add { index: at, item }) => LayoutOperationDsl::LinksAdd { index: *at, item: item.clone() },
        LayoutOperation::Links(CollectionOperation::Remove { id }) => LayoutOperationDsl::LinksRemove { id: id.clone() },
        LayoutOperation::Links(CollectionOperation::Move { id, to_index: to }) => LayoutOperationDsl::LinksMove { id: id.clone(), to_index: *to },
        LayoutOperation::Links(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::LinksPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::AddFrame { page_id, index, frame, layer_id } => LayoutOperationDsl::AddFrame { page_id: page_id.clone(), index: *index, frame: Box::new(frame.clone()), layer_id: layer_id.clone() },
        LayoutOperation::RemoveFrame { page_id, frame_id } => LayoutOperationDsl::RemoveFrame { page_id: page_id.clone(), frame_id: frame_id.clone() },
        LayoutOperation::PatchFrame { page_id, frame_id, patch } => LayoutOperationDsl::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: frame_patch_to_dsl(patch) },
        LayoutOperation::SetDataFields { json } => LayoutOperationDsl::SetDataFields { json: json.clone() },
    }
}

fn layout_operation_from_dsl(operation: LayoutOperationDsl) -> LayoutOperation {
    match operation {
        LayoutOperationDsl::PagesAdd { index, item } => LayoutOperation::Pages(CollectionOperation::Add { index: index, item }),
        LayoutOperationDsl::PagesRemove { id } => LayoutOperation::Pages(CollectionOperation::Remove { id }),
        LayoutOperationDsl::PagesMove { id, to_index } => LayoutOperation::Pages(CollectionOperation::Move { id, to_index: to_index }),
        LayoutOperationDsl::PagesPatch { id, patch } => LayoutOperation::Pages(CollectionOperation::Patch { id, patch }),
        LayoutOperationDsl::StoriesAdd { index, item } => LayoutOperation::Stories(CollectionOperation::Add { index: index, item }),
        LayoutOperationDsl::StoriesRemove { id } => LayoutOperation::Stories(CollectionOperation::Remove { id }),
        LayoutOperationDsl::StoriesMove { id, to_index } => LayoutOperation::Stories(CollectionOperation::Move { id, to_index: to_index }),
        LayoutOperationDsl::StoriesPatch { id, patch } => LayoutOperation::Stories(CollectionOperation::Patch { id, patch }),
        LayoutOperationDsl::LinksAdd { index, item } => LayoutOperation::Links(CollectionOperation::Add { index: index, item }),
        LayoutOperationDsl::LinksRemove { id } => LayoutOperation::Links(CollectionOperation::Remove { id }),
        LayoutOperationDsl::LinksMove { id, to_index } => LayoutOperation::Links(CollectionOperation::Move { id, to_index: to_index }),
        LayoutOperationDsl::LinksPatch { id, patch } => LayoutOperation::Links(CollectionOperation::Patch { id, patch }),
        LayoutOperationDsl::AddFrame { page_id, index, frame, layer_id } => LayoutOperation::AddFrame { page_id, index, frame: *frame, layer_id },
        LayoutOperationDsl::RemoveFrame { page_id, frame_id } => LayoutOperation::RemoveFrame { page_id, frame_id },
        LayoutOperationDsl::PatchFrame { page_id, frame_id, patch } => LayoutOperation::PatchFrame { page_id, frame_id, patch: frame_patch_from_dsl(patch) },
        LayoutOperationDsl::SetDataFields { json } => LayoutOperation::SetDataFields { json },
    }
}

impl OpText for LayoutOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        Ok(layout_operation_from_dsl(<LayoutOperationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <LayoutOperationDsl as OpText>::print_op(&layout_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `LayoutOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl OpBinary for LayoutOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        layout_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(layout_operation_from_dsl(LayoutOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️DslMirror

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::layout::LayoutDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = crate::artifacts::layout::engine::default_document();
        let page_id = document.pages[0].id.clone();
        let operation = LayoutOperation::Pages(CollectionOperation::Patch { id: page_id, patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_applied_operations() {
        use crate::artifacts::layout::LAYOUT_FIXTURE_SCHEMA;

        let initial = crate::artifacts::layout::engine::default_document();
        let envelope = store::create_document_envelope(LAYOUT_FIXTURE_SCHEMA, "layout-doc-text-test", initial, None);
        let mut doc_store: store::DocumentStore<LayoutDocument, LayoutOperation> = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { width: Some(640.0), ..Default::default() } })],
                description: Some("resize page".into()),
            })
            .expect("apply patch page width");
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } })],
                description: Some("rename page".into()),
            })
            .expect("apply patch page");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
        store::test_support::assert_live_equals_replay(&doc_store);
    }

    #[test]
    fn op_text_round_trips_every_layout_operation_variant() {
        let doc = crate::artifacts::layout::engine::default_document();

        let mut page_2 = doc.pages[0].clone();
        page_2.id = "page-3".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Add { index: 1, item: page_2 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Remove { id: "page-1".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Move { id: "page-1".into(), to_index: 1 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() } }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch::default() }));

        let mut story_2 = doc.stories[0].clone();
        story_2.id = "story-2".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Add { index: 1, item: story_2 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Remove { id: "story-1".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Move { id: "story-1".into(), to_index: 0 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: None } }));

        let mut link_2 = doc.links[0].clone();
        link_2.id = "link-2".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Add { index: 1, item: link_2 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Remove { id: "link-missing".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Move { id: "link-missing".into(), to_index: 0 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: None } }));

        let rect_frame = Frame::Rect {
            id: "frame-new".into(),
            layer_id: "layer-1".into(),
            bounds: crate::artifacts::layout::LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 },
            locked: None,
            visible: Some(true),
            fill: Some([0.1, 0.2, 0.3, 1.0]),
            stroke: None,
        };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: rect_frame, layer_id: Some("layer-1".into()) });
        let image_frame =
            Frame::Image { id: "frame-img".into(), layer_id: "layer-1".into(), bounds: crate::artifacts::layout::LayoutBounds { x: 1.0, y: 2.0, width: 3.0, height: 4.0, rotation: 5.0 }, locked: Some(false), visible: None, link_id: "link-missing".into() };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: image_frame, layer_id: None });
        store::test_support::assert_op_line_round_trip(&LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-text-1".into() });
        store::test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame {
            page_id: "page-1".into(),
            frame_id: "frame-text-1".into(),
            patch: FramePatch { x: Some(10.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), stroke: Some(None), ..Default::default() },
        });
        store::test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-text-1".into(), patch: FramePatch::default() });
    }

    #[test]
    fn op_text_round_trips_full_page_and_frame_patch_fields() {
        let full_page_patch =
            PagePatch { name: Some("Renamed".into()), width: Some(300.0), height: Some(400.0), margin_top: Some(1.0), margin_right: Some(2.0), margin_bottom: Some(3.0), margin_left: Some(4.0), columns_count: Some(5), columns_gutter: Some(6.0) };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: full_page_patch }));

        let full_frame_patch = FramePatch { x: Some(1.0), y: Some(2.0), width: Some(3.0), height: Some(4.0), fill: Some(Some([0.1, 0.2, 0.3, 0.4])), stroke: Some(None), wrap_mode: Some("column".into()), columns: Some(3) };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: full_frame_patch });

        let clearing_frame_patch = FramePatch { fill: Some(None), stroke: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: clearing_frame_patch });
    }

    #[test]
    fn parse_op_reports_engine_parser_errors() {
        assert!(LayoutOperation::parse_op("patch-frame page-id=page-1 frame-id=frame-1 patch=1,2,3").is_err(), "patch must be a block, not a bare tuple attribute");
    }

    #[test]
    fn op_text_rejects_unknown_operation_name() {
        assert!(LayoutOperation::parse_op("bogusOp id=x").is_err(), "unknown op keyword must fail");
    }
}
//#endregion 🧪️Tests
