//! ⚡️ Layout app — operation enum + laws (constitutional: op).

use layout::{Frame, FramePatch, ImageLink, ImageLinkPatch, LayoutDocument, Page, PagePatch, TextStory, TextStoryPatch};
use protocol::{apply_collection_operation, invert_collection_operation, CollectionOperation, OpBinary, OpText, Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::TextError;

//#region 🔖️Patches
/// 🩹️ Applies a {@link FramePatch} in place and returns the patch that undoes it.
pub fn apply_frame_patch(frame: &mut Frame, patch: &FramePatch) -> FramePatch {
    let mut inverse = FramePatch::default();
    {
        let bounds = match frame {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        };
        if patch.x.is_some() {
            inverse.x = Some(bounds.x);
        }
        if patch.y.is_some() {
            inverse.y = Some(bounds.y);
        }
        if patch.width.is_some() {
            inverse.width = Some(bounds.width);
        }
        if patch.height.is_some() {
            inverse.height = Some(bounds.height);
        }
        if let Some(value) = patch.x {
            bounds.x = value;
        }
        if let Some(value) = patch.y {
            bounds.y = value;
        }
        if let Some(value) = patch.width {
            bounds.width = value;
        }
        if let Some(value) = patch.height {
            bounds.height = value;
        }
    }
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            if patch.fill.is_some() {
                inverse.fill = Some(*fill);
            }
            if patch.stroke.is_some() {
                inverse.stroke = Some(*stroke);
            }
            if let Some(new) = patch.fill {
                *fill = new;
            }
            if let Some(new) = patch.stroke {
                *stroke = new;
            }
        }
        Frame::Text { wrap_mode, columns, .. } => {
            if patch.wrap_mode.is_some() {
                inverse.wrap_mode = Some(wrap_mode.clone());
            }
            if patch.columns.is_some() {
                inverse.columns = Some(*columns);
            }
            if let Some(new) = &patch.wrap_mode {
                *wrap_mode = new.clone();
            }
            if let Some(new) = patch.columns {
                *columns = new;
            }
        }
        Frame::Image { .. } => {}
    }
    inverse
}
//#endregion 🔖️Patches

//#region 🔖️Operation
/// 🧺️ The typed layout document operation. Pages/stories/links are flat id-keyed collections; frames
/// are nested per-page so they get bespoke add/remove/patch variants. Camera pose is ephemeral
/// per-surface view state owned by the layout-ui app's runtime, never a document operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum LayoutOperation {
    Pages(CollectionOperation<String, Page, PagePatch>),
    Stories(CollectionOperation<String, TextStory, TextStoryPatch>),
    Links(CollectionOperation<String, ImageLink, ImageLinkPatch>),
    AddFrame { page_id: String, index: usize, frame: Frame, layer_id: Option<String> },
    RemoveFrame { page_id: String, frame_id: String },
    PatchFrame { page_id: String, frame_id: String, patch: FramePatch },
    /// 🔠️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: whole-field replace for
    /// `LayoutDocument::data_fields_json` — the `fields:in` workflow port's real, undoable write (see
    /// `layout_ui::LayoutPlayApp::import_media`).
    SetDataFields { json: Option<String> }
}

fn apply_layout_operation(doc: &mut LayoutDocument, operation: &LayoutOperation) {
    match operation {
        LayoutOperation::Pages(cop) => apply_collection_operation(&mut doc.pages, cop),
        LayoutOperation::Stories(cop) => apply_collection_operation(&mut doc.stories, cop),
        LayoutOperation::Links(cop) => apply_collection_operation(&mut doc.links, cop),
        LayoutOperation::AddFrame { page_id, index, frame, layer_id } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                let at = (*index).min(page.frames.len());
                page.frames.insert(at, frame.clone());
                if let Some(layer_id) = layer_id {
                    if let Some(layer) = page.layers.iter_mut().find(|layer| layer.id == *layer_id) {
                        layer.object_ids.push(frame.id().to_string());
                    }
                }
            }
        }
        LayoutOperation::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                page.frames.retain(|frame| frame.id() != frame_id);
                for layer in &mut page.layers {
                    layer.object_ids.retain(|id| id != frame_id);
                }
            }
        }
        LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter_mut().find(|frame| frame.id() == frame_id) {
                    apply_frame_patch(frame, patch);
                }
            }
        }
        LayoutOperation::SetDataFields { json } => {
            doc.data_fields_json = json.clone();
        }
    }
}

fn backwards_layout_operation(doc: &LayoutDocument, operation: &LayoutOperation) -> Vec<LayoutOperation> {
    match operation {
        LayoutOperation::Pages(cop) => vec![LayoutOperation::Pages(invert_collection_operation(&doc.pages, cop))],
        LayoutOperation::Stories(cop) => vec![LayoutOperation::Stories(invert_collection_operation(&doc.stories, cop))],
        LayoutOperation::Links(cop) => vec![LayoutOperation::Links(invert_collection_operation(&doc.links, cop))],
        LayoutOperation::AddFrame { page_id, frame, .. } => {
            vec![LayoutOperation::RemoveFrame { page_id: page_id.clone(), frame_id: frame.id().to_string() }]
        }
        LayoutOperation::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(index) = page.frames.iter().position(|frame| frame.id() == frame_id) {
                    let frame = page.frames[index].clone();
                    let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == frame_id)).map(|layer| layer.id.clone());
                    return vec![LayoutOperation::AddFrame { page_id: page_id.clone(), index, frame, layer_id }];
                }
            }
            Vec::new()
        }
        LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) {
                    let mut clone = frame.clone();
                    let inverse = apply_frame_patch(&mut clone, patch);
                    return vec![LayoutOperation::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: inverse }];
                }
            }
            Vec::new()
        }
        LayoutOperation::SetDataFields { .. } => vec![LayoutOperation::SetDataFields { json: doc.data_fields_json.clone() }],
    }
}

/// 📦️ Operation-list diff: layout operations fold sequentially over a cloned projection. `absorb`
/// concatenates — sequential edits replay forwards in order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LayoutDiff {
    pub operations: Vec<LayoutOperation>,
}

impl OperationDiff<LayoutDocument> for LayoutDiff {
    fn apply(&self, projection: &LayoutDocument) -> LayoutDocument {
        let mut next = projection.clone();
        for operation in &self.operations {
            apply_layout_operation(&mut next, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.operations.extend(other.operations);
    }
}

impl Operation<LayoutDocument> for LayoutOperation {
    type Diff = LayoutDiff;

    fn diff(&self, _projection: &LayoutDocument) -> LayoutDiff {
        LayoutDiff { operations: vec![self.clone()] }
    }

    fn backwards(&self, projection: &LayoutDocument) -> Vec<Self> {
        backwards_layout_operation(projection, self)
    }
}
//#endregion 🔖️Operation

//#region 🔖️OpText
// 🔤️ `LayoutOperation`'s `store::OpText`/`protocol::OpBinary` would normally come for free from
// `#[derive(dsl::DslOps)]`, but its `Pages`/`Stories`/`Links` variants wrap a foreign generic type
// (`protocol::CollectionOperation<K,V,P>`, orphan rule: can't gain a `dsl::DslField`/`dsl::DslVariants`
// binding here) and `FramePatch.fill`/`.stroke` (`Option<Option<[f32;4]>>`) has the same "no direct
// binding" issue one level down. `LayoutOperationDsl` is a local, DSL-only mirror that flattens each
// collection wrapper into its own keyworded variants and `FramePatchDsl`/`ColorPatch` fix the doubly-
// optional color fields, mirroring `process_3d::Process3dOperationDsl`'s identical fix for the same
// foreign-type problem. Both hand-written impls below convert at the `OpText`/`OpBinary` boundary only;
// `LayoutOperation` itself (and every consumer matching on it, e.g. `layout/plugin`) is untouched.

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

/// ⚡️ DSL-only mirror of `LayoutOperation` — see this region's opening doc comment.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum LayoutOperationDsl {
    PagesAdd {
        index: usize,
        #[dsl(block)]
        item: Page,
    },
    PagesRemove { id: String },
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
    StoriesRemove { id: String },
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
    LinksRemove { id: String },
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
    SetDataFields { json: Option<String> }
}

fn layout_operation_to_dsl(operation: &LayoutOperation) -> LayoutOperationDsl {
    match operation {
        LayoutOperation::Pages(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::PagesAdd { index: *at, item: item.clone() },
        LayoutOperation::Pages(CollectionOperation::Remove { id }) => LayoutOperationDsl::PagesRemove { id: id.clone() },
        LayoutOperation::Pages(CollectionOperation::Move { id, to }) => LayoutOperationDsl::PagesMove { id: id.clone(), to_index: *to },
        LayoutOperation::Pages(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::PagesPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::Stories(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::StoriesAdd { index: *at, item: item.clone() },
        LayoutOperation::Stories(CollectionOperation::Remove { id }) => LayoutOperationDsl::StoriesRemove { id: id.clone() },
        LayoutOperation::Stories(CollectionOperation::Move { id, to }) => LayoutOperationDsl::StoriesMove { id: id.clone(), to_index: *to },
        LayoutOperation::Stories(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::StoriesPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::Links(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::LinksAdd { index: *at, item: item.clone() },
        LayoutOperation::Links(CollectionOperation::Remove { id }) => LayoutOperationDsl::LinksRemove { id: id.clone() },
        LayoutOperation::Links(CollectionOperation::Move { id, to }) => LayoutOperationDsl::LinksMove { id: id.clone(), to_index: *to },
        LayoutOperation::Links(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::LinksPatch { id: id.clone(), patch: patch.clone() },
        LayoutOperation::AddFrame { page_id, index, frame, layer_id } => {
            LayoutOperationDsl::AddFrame { page_id: page_id.clone(), index: *index, frame: Box::new(frame.clone()), layer_id: layer_id.clone() }
        }
        LayoutOperation::RemoveFrame { page_id, frame_id } => LayoutOperationDsl::RemoveFrame { page_id: page_id.clone(), frame_id: frame_id.clone() },
        LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
            LayoutOperationDsl::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: frame_patch_to_dsl(patch) }
        }
        LayoutOperation::SetDataFields { json } => LayoutOperationDsl::SetDataFields { json: json.clone() },
    }
}

fn layout_operation_from_dsl(operation: LayoutOperationDsl) -> LayoutOperation {
    match operation {
        LayoutOperationDsl::PagesAdd { index, item } => LayoutOperation::Pages(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        LayoutOperationDsl::PagesRemove { id } => LayoutOperation::Pages(CollectionOperation::Remove { id }),
        LayoutOperationDsl::PagesMove { id, to_index } => LayoutOperation::Pages(CollectionOperation::Move { id, to: to_index }),
        LayoutOperationDsl::PagesPatch { id, patch } => LayoutOperation::Pages(CollectionOperation::Patch { id, patch }),
        LayoutOperationDsl::StoriesAdd { index, item } => LayoutOperation::Stories(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        LayoutOperationDsl::StoriesRemove { id } => LayoutOperation::Stories(CollectionOperation::Remove { id }),
        LayoutOperationDsl::StoriesMove { id, to_index } => LayoutOperation::Stories(CollectionOperation::Move { id, to: to_index }),
        LayoutOperationDsl::StoriesPatch { id, patch } => LayoutOperation::Stories(CollectionOperation::Patch { id, patch }),
        LayoutOperationDsl::LinksAdd { index, item } => LayoutOperation::Links(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        LayoutOperationDsl::LinksRemove { id } => LayoutOperation::Links(CollectionOperation::Remove { id }),
        LayoutOperationDsl::LinksMove { id, to_index } => LayoutOperation::Links(CollectionOperation::Move { id, to: to_index }),
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
//#endregion 🔖️OpText

//#region 🔖️ConfigOperations
/// @emoji 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: `layout_engine::LayoutConfig`'s
/// operation enum — mirrors `shooting_op::ShootingConfigOperation`'s shape exactly: one variant per
/// settled interaction (was a `LayoutPlayRuntime` field write pre-B1), plus a generic `Snapshot` every
/// variant's `backwards()` returns. `Operation::Diff` is the WHOLE `LayoutConfig` (not a granular patch
/// type): `diff()` returns "the full config after this op", and `OperationDiff<LayoutConfig>::apply`
/// for `LayoutConfig` itself (below) just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum LayoutConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: layout_engine::LayoutConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-page")]
    SetActivePage { page_id: String },
    #[dsl(key = "hover")]
    SetHover { id: Option<String> },
    #[dsl(key = "drop-preview")]
    SetDropPreview {
        #[dsl(block)]
        preview: layout_engine::LayoutDropPreviewState,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: layout::LayoutCamera,
    },
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: layout::LayoutCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<layout_engine::LayoutConfig> for LayoutConfigOperation {
    type Diff = layout_engine::LayoutConfig;

    fn diff(&self, base: &layout_engine::LayoutConfig) -> layout_engine::LayoutConfig {
        let mut next = base.clone();
        match self {
            LayoutConfigOperation::Snapshot { config } => return config.clone(),
            LayoutConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            LayoutConfigOperation::SetActivePage { page_id } => next.active_page_id = page_id.clone(),
            LayoutConfigOperation::SetHover { id } => next.hovered_id = id.clone(),
            LayoutConfigOperation::SetDropPreview { preview } => next.drop_preview = preview.clone(),
            LayoutConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            LayoutConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            LayoutConfigOperation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            LayoutConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &layout_engine::LayoutConfig) -> Vec<Self> {
        vec![LayoutConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use layout::LayoutBounds;

    const SAMPLE: &str = r#"{"schema":"layout.fixture","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[{"id":"story-1","content":"Hello","styleRuns":[]}],"links":[{"id":"link-1","path":"a.png","hash":"h","width":10,"height":10,"dpi":300}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}],"printTarget":null}"#;

    fn sample_doc() -> LayoutDocument {
        layout_engine::parse_layout_document(SAMPLE).expect("sample doc")
    }

    fn new_rect(id: &str) -> Frame {
        Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 }, locked: None, visible: None, fill: Some([0.1, 0.2, 0.3, 1.0]), stroke: None }
    }

    fn round_trip(doc: &LayoutDocument, operation: &LayoutOperation) -> LayoutDocument {
        let forward = vcs::apply_operation(doc, operation);
        let backs = operation.backwards(doc);
        let mut restored = forward.clone();
        for back in &backs {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, doc, "backwards must restore the pre-operation document");
        forward
    }

    #[test]
    fn pages_add_and_patch_round_trip() {
        let doc = sample_doc();
        let mut page_2 = doc.pages[0].clone();
        page_2.id = "page-2".into();
        let add = LayoutOperation::Pages(CollectionOperation::Add { id: page_2.id.clone(), item: page_2, at: 1 });
        let with_page = round_trip(&doc, &add);
        assert_eq!(with_page.pages.len(), 2);

        let patch = LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() } });
        let patched = round_trip(&doc, &patch);
        let page = patched.pages.iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.name, "Renamed");
        assert_eq!(page.width, 300.0);
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn frame_add_remove_patch_round_trip() {
        let doc = sample_doc();
        let add = LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: new_rect("frame-2"), layer_id: Some("layer-1".into()) };
        let added = round_trip(&doc, &add);
        assert_eq!(added.pages[0].frames.len(), 2);
        assert!(added.pages[0].layers[0].object_ids.iter().any(|id| id == "frame-2"));

        let remove = LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-1".into() };
        let removed = round_trip(&doc, &remove);
        assert!(removed.pages[0].frames.iter().all(|frame| frame.id() != "frame-1"));

        let patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(99.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() } };
        let patched = round_trip(&doc, &patch);
        let frame = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
        assert_eq!(frame.bounds().x, 99.0);
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect") };
        assert_eq!(fill.unwrap(), [0.5, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn story_and_link_patch_round_trip() {
        let doc = sample_doc();
        let story = LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } });
        let edited = round_trip(&doc, &story);
        assert_eq!(edited.stories[0].content, "Edited");

        let link = LayoutOperation::Links(CollectionOperation::Patch { id: "link-1".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } });
        let relinked = round_trip(&doc, &link);
        assert_eq!(relinked.links[0].path, "b.png");
    }

    fn new_text(id: &str) -> Frame {
        Frame::Text { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 }, locked: None, visible: None, story_id: "story-1".into(), thread_next: None, columns: 1, inset: layout::LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, wrap_mode: "box".into() }
    }

    #[test]
    fn patch_frame_updates_text_fields_and_ignores_fill_on_image_frames() {
        let doc = sample_doc();
        let with_text = vcs::apply_operation(&doc, &LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_text("frame-text"), layer_id: None });
        let patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-text".into(), patch: FramePatch { wrap_mode: Some("column".into()), columns: Some(2), ..Default::default() } };
        let patched = round_trip(&with_text, &patch);
        let Frame::Text { wrap_mode, columns, .. } = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-text").unwrap() else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "column");
        assert_eq!(*columns, 2);

        let image_frame = Frame::Image { id: "frame-img".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 5.0, height: 5.0, rotation: 0.0 }, locked: None, visible: None, link_id: "link-1".into() };
        let with_image = vcs::apply_operation(&doc, &LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: image_frame, layer_id: None });
        let image_patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-img".into(), patch: FramePatch { x: Some(3.0), fill: Some(Some([1.0, 0.0, 0.0, 1.0])), ..Default::default() } };
        let patched_image = round_trip(&with_image, &image_patch);
        let patched_frame = patched_image.pages[0].frames.iter().find(|frame| frame.id() == "frame-img").unwrap();
        assert_eq!(patched_frame.bounds().x, 3.0, "bounds still patch on an image frame");
    }

    #[test]
    fn add_remove_patch_frame_are_no_ops_when_target_missing() {
        let doc = sample_doc();

        let missing_page_add = LayoutOperation::AddFrame { page_id: "no-page".into(), index: 0, frame: new_rect("frame-x"), layer_id: None };
        assert_eq!(vcs::apply_operation(&doc, &missing_page_add), doc, "adding to a missing page must be a no-op");

        let unmatched_layer = LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_rect("frame-y"), layer_id: Some("no-layer".into()) };
        let result = vcs::apply_operation(&doc, &unmatched_layer);
        assert!(result.pages[0].frames.iter().any(|frame| frame.id() == "frame-y"));
        assert!(result.pages[0].layers[0].object_ids.iter().all(|id| id != "frame-y"), "unmatched layer id must not be populated");

        let missing_page_remove = LayoutOperation::RemoveFrame { page_id: "no-page".into(), frame_id: "frame-1".into() };
        assert_eq!(vcs::apply_operation(&doc, &missing_page_remove), doc);
        assert!(missing_page_remove.backwards(&doc).is_empty());

        let missing_frame_remove = LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "no-frame".into() };
        assert_eq!(vcs::apply_operation(&doc, &missing_frame_remove), doc);
        assert!(missing_frame_remove.backwards(&doc).is_empty());

        let missing_page_patch = LayoutOperation::PatchFrame { page_id: "no-page".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
        assert_eq!(vcs::apply_operation(&doc, &missing_page_patch), doc);
        assert!(missing_page_patch.backwards(&doc).is_empty());

        let missing_frame_patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "no-frame".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
        assert_eq!(vcs::apply_operation(&doc, &missing_frame_patch), doc);
        assert!(missing_frame_patch.backwards(&doc).is_empty());
    }

    #[test]
    fn op_text_round_trips_every_layout_operation_variant() {
        let doc = sample_doc();

        let mut page_2 = doc.pages[0].clone();
        page_2.id = "page-3".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Add { id: page_2.id.clone(), item: page_2, at: 1 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Remove { id: "page-1".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Move { id: "page-1".into(), to: 1 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch {
            id: "page-1".into(),
            patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() },
        }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch::default() }));

        let mut story_2 = doc.stories[0].clone();
        story_2.id = "story-2".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Add { id: story_2.id.clone(), item: story_2, at: 1 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Remove { id: "story-1".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Move { id: "story-1".into(), to: 0 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: None } }));

        let mut link_2 = doc.links[0].clone();
        link_2.id = "link-2".into();
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Add { id: link_2.id.clone(), item: link_2, at: 1 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Remove { id: "link-missing".into() }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Move { id: "link-missing".into(), to: 0 }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } }));
        store::test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: None } }));

        let rect_frame = Frame::Rect {
            id: "frame-new".into(),
            layer_id: "layer-1".into(),
            bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 },
            locked: None,
            visible: Some(true),
            fill: Some([0.1, 0.2, 0.3, 1.0]),
            stroke: None,
        };
        store::test_support::assert_op_line_round_trip(&LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: rect_frame, layer_id: Some("layer-1".into()) });
        let image_frame = Frame::Image {
            id: "frame-img".into(),
            layer_id: "layer-1".into(),
            bounds: LayoutBounds { x: 1.0, y: 2.0, width: 3.0, height: 4.0, rotation: 5.0 },
            locked: Some(false),
            visible: None,
            link_id: "link-missing".into(),
        };
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
        let full_page_patch = PagePatch { name: Some("Renamed".into()), width: Some(300.0), height: Some(400.0), margin_top: Some(1.0), margin_right: Some(2.0), margin_bottom: Some(3.0), margin_left: Some(4.0), columns_count: Some(5), columns_gutter: Some(6.0) };
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

    #[test]
    fn set_data_fields_round_trips_and_restores_previous_value() {
        let doc = sample_doc();
        let set = LayoutOperation::SetDataFields { json: Some(r#"{"key":"value"}"#.into()) };
        let with_fields = round_trip(&doc, &set);
        assert_eq!(with_fields.data_fields_json.as_deref(), Some(r#"{"key":"value"}"#));

        let clear = LayoutOperation::SetDataFields { json: None };
        let cleared = round_trip(&with_fields, &clear);
        assert!(cleared.data_fields_json.is_none());

        store::test_support::assert_op_line_round_trip(&LayoutOperation::SetDataFields { json: Some("{}".into()) });
        store::test_support::assert_op_line_round_trip(&LayoutOperation::SetDataFields { json: None });
    }

    //#region 🧪️ConfigOperations
    fn sample_config() -> layout_engine::LayoutConfig {
        layout_engine::LayoutConfig {
            active_page_id: "page-2".into(),
            selected_ids: vec!["frame-1".into()],
            hovered_id: Some("frame-2".into()),
            drop_preview: layout_engine::LayoutDropPreviewState { kind: "rect".into(), x: 1.0, y: 2.0 },
            engagement_input: "export png".into(),
            camera: layout::LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 },
            preview_camera: layout::LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 },
            locale: "de-DE".into(),
        }
    }

    fn config_round_trip(base: &layout_engine::LayoutConfig, operation: &LayoutConfigOperation) -> layout_engine::LayoutConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_apply_and_restore_every_field() {
        let base = layout_engine::LayoutConfig::default();
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetSelection { ids: vec!["a".into()] }).selected_ids, vec!["a".to_string()]);
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetActivePage { page_id: "page-9".into() }).active_page_id, "page-9");
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetHover { id: Some("frame-9".into()) }).hovered_id, Some("frame-9".to_string()));
        let previewed = config_round_trip(&base, &LayoutConfigOperation::SetDropPreview { preview: layout_engine::LayoutDropPreviewState { kind: "rect".into(), x: 5.0, y: 6.0 } });
        assert_eq!(previewed.drop_preview.kind, "rect");
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetEngagementInput { value: "undo".into() }).engagement_input, "undo");
        let cam = config_round_trip(&base, &LayoutConfigOperation::SetCamera { camera: layout::LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(cam.camera, layout::LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 });
        let preview_cam = config_round_trip(&base, &LayoutConfigOperation::SetPreviewCamera { camera: layout::LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 } });
        assert_eq!(preview_cam.preview_camera, layout::LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 });
        assert_eq!(config_round_trip(&base, &LayoutConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
    }

    #[test]
    fn config_snapshot_op_text_round_trips() {
        let config = sample_config();
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetHover { id: None });
        store::test_support::assert_op_line_round_trip(&LayoutConfigOperation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🧪️ConfigOperations
}
//#endregion 🧪️Tests
