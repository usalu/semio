//! 🧬️ layout artifact — document mutation dispatch.


use crate::artifacts::layout::schema::diff::text::{
    diff_set_snapshot, links_delta_from_collection_mutation, pages_delta_from_collection_mutation,
    pages_replace_delta, stories_delta_from_collection_mutation,
};
use crate::artifacts::layout::schema::diff::LayoutDiff;
use crate::artifacts::layout::{Frame, FramePatch, ImageLink, ImageLinkPatch, LayoutSnapshot, Page, PagePatch, TextStory, TextStoryPatch};
use protocol::{apply_collection_mutation, inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

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

//#region 🔖️Mutations
/// 🧺️ The typed layout document operation. Pages/stories/links are flat id-keyed collections; frames
/// are nested per-page so they get bespoke add/remove/patch variants. Camera pose is ephemeral
/// per-surface view state owned by the layout app's config, never a document operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum LayoutMutation {
    Pages(CollectionMutation<String, Page, PagePatch>),
    Stories(CollectionMutation<String, TextStory, TextStoryPatch>),
    Links(CollectionMutation<String, ImageLink, ImageLinkPatch>),
    AddFrame {
        page_id: String,
        index: usize,
        frame: Frame,
        layer_id: Option<String>,
    },
    RemoveFrame {
        page_id: String,
        frame_id: String,
    },
    PatchFrame {
        page_id: String,
        frame_id: String,
        patch: FramePatch,
    },
    /// 🔠️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: whole-field replace for
    /// `LayoutSnapshot::data_fields_json` — the `fields:in` workflow port's real, undoable write (see
    /// `crate::apps::layout::commands::author::import_media`).
    SetDataFields {
        json: Option<String>,
    },
}

/// 🧮️ Applies `operation` onto `doc` in place — the sole forward-transform used both by
/// `LayoutDiff::apply` and (indirectly, via `Mutation::diff`) by every op's own `backwards`.
pub fn apply_layout_mutation(doc: &mut LayoutSnapshot, operation: &LayoutMutation) {
    match operation {
        LayoutMutation::Pages(cop) => apply_collection_mutation(&mut doc.pages, cop),
        LayoutMutation::Stories(cop) => apply_collection_mutation(&mut doc.stories, cop),
        LayoutMutation::Links(cop) => apply_collection_mutation(&mut doc.links, cop),
        LayoutMutation::AddFrame { page_id, index, frame, layer_id } => {
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
        LayoutMutation::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                page.frames.retain(|frame| frame.id() != frame_id);
                for layer in &mut page.layers {
                    layer.object_ids.retain(|id| id != frame_id);
                }
            }
        }
        LayoutMutation::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter_mut().find(|frame| frame.id() == frame_id) {
                    apply_frame_patch(frame, patch);
                }
            }
        }
        LayoutMutation::SetDataFields { json } => {
            doc.data_fields_json = json.clone();
        }
    }
}

pub fn inverse_layout_mutation(doc: &LayoutSnapshot, operation: &LayoutMutation) -> Vec<LayoutMutation> {
    match operation {
        LayoutMutation::Pages(cop) => vec![LayoutMutation::Pages(inverse_collection_mutation(&doc.pages, cop))],
        LayoutMutation::Stories(cop) => vec![LayoutMutation::Stories(inverse_collection_mutation(&doc.stories, cop))],
        LayoutMutation::Links(cop) => vec![LayoutMutation::Links(inverse_collection_mutation(&doc.links, cop))],
        LayoutMutation::AddFrame { page_id, frame, .. } => {
            vec![LayoutMutation::RemoveFrame { page_id: page_id.clone(), frame_id: frame.id().to_string() }]
        }
        LayoutMutation::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(index) = page.frames.iter().position(|frame| frame.id() == frame_id) {
                    let frame = page.frames[index].clone();
                    let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == frame_id)).map(|layer| layer.id.clone());
                    return vec![LayoutMutation::AddFrame { page_id: page_id.clone(), index, frame, layer_id }];
                }
            }
            Vec::new()
        }
        LayoutMutation::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) {
                    let mut clone = frame.clone();
                    let inverse = apply_frame_patch(&mut clone, patch);
                    return vec![LayoutMutation::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: inverse }];
                }
            }
            Vec::new()
        }
        LayoutMutation::SetDataFields { .. } => vec![LayoutMutation::SetDataFields { json: doc.data_fields_json.clone() }],
    }
}

impl Mutation<LayoutSnapshot> for LayoutMutation {
    type Diff = LayoutDiff;

    fn diff(&self, snapshot: &LayoutSnapshot) -> LayoutDiff {
        match self {
            LayoutMutation::Pages(op) => LayoutDiff {
                pages: Some(pages_delta_from_collection_mutation(&snapshot.pages, op)),
                ..Default::default()
            },
            LayoutMutation::Stories(op) => LayoutDiff {
                stories: Some(stories_delta_from_collection_mutation(&snapshot.stories, op)),
                ..Default::default()
            },
            LayoutMutation::Links(op) => LayoutDiff {
                links: Some(links_delta_from_collection_mutation(&snapshot.links, op)),
                ..Default::default()
            },
            LayoutMutation::SetDataFields { json } => LayoutDiff {
                data_fields_json: Some(json.clone()),
                ..Default::default()
            },
            LayoutMutation::AddFrame { .. } | LayoutMutation::RemoveFrame { .. } | LayoutMutation::PatchFrame { .. } => {
                let mut next = snapshot.clone();
                apply_layout_mutation(&mut next, self);
                LayoutDiff {
                    pages: Some(pages_replace_delta(&snapshot.pages, &next.pages)),
                    ..Default::default()
                }
            }
        }
    }

    fn inverse(&self, projection: &LayoutSnapshot) -> Vec<Self> {
        inverse_layout_mutation(projection, self)
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::layout::LayoutBounds;
    use protocol::MutationDiff;

    const SAMPLE: &str = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[{"id":"story-1","content":"Hello","styleRuns":[]}],"links":[{"id":"link-1","path":"a.png","hash":"h","width":10,"height":10,"dpi":300}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}],"printTarget":null}"#;

    fn sample_doc() -> LayoutSnapshot {
        serde_json::from_str(SAMPLE).expect("sample doc")
    }

    fn new_rect(id: &str) -> Frame {
        Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 }, locked: None, visible: None, fill: Some([0.1, 0.2, 0.3, 1.0]), stroke: None }
    }

    fn round_trip(doc: &LayoutSnapshot, operation: &LayoutMutation) -> LayoutSnapshot {
        let forward = operation.diff(doc).apply(doc);
        let backs = operation.inverse(doc);
        let mut restored = forward.clone();
        for back in &backs {
            restored = back.diff(&restored).apply(&restored);
        }
        assert_eq!(&restored, doc, "backwards must restore the pre-operation document");
        forward
    }

    #[test]
    fn pages_add_and_patch_round_trip() {
        let doc = sample_doc();
        let mut page_2 = doc.pages[0].clone();
        page_2.id = "page-2".into();
        let add = LayoutMutation::Pages(CollectionMutation::Add { index: 1, item: page_2 });
        let with_page = round_trip(&doc, &add);
        assert_eq!(with_page.pages.len(), 2);

        let patch = LayoutMutation::Pages(CollectionMutation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() } });
        let patched = round_trip(&doc, &patch);
        let page = patched.pages.iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.name, "Renamed");
        assert_eq!(page.width, 300.0);
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn frame_add_remove_patch_round_trip() {
        let doc = sample_doc();
        let add = LayoutMutation::AddFrame { page_id: "page-1".into(), index: 1, frame: new_rect("frame-2"), layer_id: Some("layer-1".into()) };
        let added = round_trip(&doc, &add);
        assert_eq!(added.pages[0].frames.len(), 2);
        assert!(added.pages[0].layers[0].object_ids.iter().any(|id| id == "frame-2"));

        let remove = LayoutMutation::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-1".into() };
        let removed = round_trip(&doc, &remove);
        assert!(removed.pages[0].frames.iter().all(|frame| frame.id() != "frame-1"));

        let patch = LayoutMutation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(99.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() } };
        let patched = round_trip(&doc, &patch);
        let frame = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
        assert_eq!(frame.bounds().x, 99.0);
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect") };
        assert_eq!(fill.unwrap(), [0.5, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn story_and_link_patch_round_trip() {
        let doc = sample_doc();
        let story = LayoutMutation::Stories(CollectionMutation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } });
        let edited = round_trip(&doc, &story);
        assert_eq!(edited.stories[0].content, "Edited");

        let link = LayoutMutation::Links(CollectionMutation::Patch { id: "link-1".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } });
        let relinked = round_trip(&doc, &link);
        assert_eq!(relinked.links[0].path, "b.png");
    }

    fn new_text(id: &str) -> Frame {
        Frame::Text {
            id: id.into(),
            layer_id: "layer-1".into(),
            bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 },
            locked: None,
            visible: None,
            story_id: "story-1".into(),
            thread_next: None,
            columns: 1,
            inset: crate::artifacts::layout::LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            wrap_mode: "box".into(),
        }
    }

    #[test]
    fn patch_frame_updates_text_fields_and_ignores_fill_on_image_frames() {
        let doc = sample_doc();
        let add = LayoutMutation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_text("frame-text"), layer_id: None };
        let with_text = add.diff(&doc).apply(&doc);
        let patch = LayoutMutation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-text".into(), patch: FramePatch { wrap_mode: Some("column".into()), columns: Some(2), ..Default::default() } };
        let patched = round_trip(&with_text, &patch);
        let Frame::Text { wrap_mode, columns, .. } = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-text").unwrap() else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "column");
        assert_eq!(*columns, 2);

        let image_frame = Frame::Image { id: "frame-img".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 5.0, height: 5.0, rotation: 0.0 }, locked: None, visible: None, link_id: "link-1".into() };
        let add_image = LayoutMutation::AddFrame { page_id: "page-1".into(), index: 0, frame: image_frame, layer_id: None };
        let with_image = add_image.diff(&doc).apply(&doc);
        let image_patch = LayoutMutation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-img".into(), patch: FramePatch { x: Some(3.0), fill: Some(Some([1.0, 0.0, 0.0, 1.0])), ..Default::default() } };
        let patched_image = round_trip(&with_image, &image_patch);
        let patched_frame = patched_image.pages[0].frames.iter().find(|frame| frame.id() == "frame-img").unwrap();
        assert_eq!(patched_frame.bounds().x, 3.0, "bounds still patch on an image frame");
    }

    #[test]
    fn add_remove_patch_frame_are_no_ops_when_target_missing() {
        let doc = sample_doc();
        let apply = |operation: &LayoutMutation| operation.diff(&doc).apply(&doc);

        let missing_page_add = LayoutMutation::AddFrame { page_id: "no-page".into(), index: 0, frame: new_rect("frame-x"), layer_id: None };
        assert_eq!(apply(&missing_page_add), doc, "adding to a missing page must be a no-op");

        let unmatched_layer = LayoutMutation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_rect("frame-y"), layer_id: Some("no-layer".into()) };
        let result = apply(&unmatched_layer);
        assert!(result.pages[0].frames.iter().any(|frame| frame.id() == "frame-y"));
        assert!(result.pages[0].layers[0].object_ids.iter().all(|id| id != "frame-y"), "unmatched layer id must not be populated");

        let missing_page_remove = LayoutMutation::RemoveFrame { page_id: "no-page".into(), frame_id: "frame-1".into() };
        assert_eq!(apply(&missing_page_remove), doc);
        assert!(missing_page_remove.inverse(&doc).is_empty());

        let missing_frame_remove = LayoutMutation::RemoveFrame { page_id: "page-1".into(), frame_id: "no-frame".into() };
        assert_eq!(apply(&missing_frame_remove), doc);
        assert!(missing_frame_remove.inverse(&doc).is_empty());

        let missing_page_patch = LayoutMutation::PatchFrame { page_id: "no-page".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
        assert_eq!(apply(&missing_page_patch), doc);
        assert!(missing_page_patch.inverse(&doc).is_empty());

        let missing_frame_patch = LayoutMutation::PatchFrame { page_id: "page-1".into(), frame_id: "no-frame".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
        assert_eq!(apply(&missing_frame_patch), doc);
        assert!(missing_frame_patch.inverse(&doc).is_empty());
    }

    #[test]
    fn set_data_fields_round_trips_and_restores_previous_value() {
        let doc = sample_doc();
        let set = LayoutMutation::SetDataFields { json: Some(r#"{"key":"value"}"#.into()) };
        let with_fields = round_trip(&doc, &set);
        assert_eq!(with_fields.data_fields_json.as_deref(), Some(r#"{"key":"value"}"#));

        let clear = LayoutMutation::SetDataFields { json: None };
        let cleared = round_trip(&with_fields, &clear);
        assert!(cleared.data_fields_json.is_none());
    }
}
//#endregion 🧪️Tests
