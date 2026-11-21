//! 🧬️ Layout snapshot schema — artifact-lane fields only.

#[cfg(test)]
#[cfg(test)] use crate::Frame;
use crate::{CharacterStyle, GridSettings, ImageLink, LayoutDrawingMember, Page, ParagraphStyle, ParentPage, Spread, TextStory};
#[cfg(test)] #[cfg(test)] use crate::LAYOUT_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persists Layout document fields and genuine Drawing member references.
/// Current child heads belong to their independent member stores; authored initial payloads remain in separate Drawing births.
/// Derived child/link slot tables and text/pack codecs share the same reference-only schema.
/// `ArtifactSchema` emits slot tables; text and pack are the
/// derived spec-driven encodings of the one `dsl::DslRecord` spec, composed child and link slot included.
/// Current child heads belong to their member stores.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive, semio_framework_value::SnapshotProjection)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields, retire_with="crate::snapshot::retire", by_value_projection)]
#[dsl(extension = "layout")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    #[value(rename = "paragraphStyles")]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    #[value(rename = "characterStyles")]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    #[value(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    #[child(kind = "s.draw.drawing")]
    pub pages: Vec<Page>,
    #[state(artifact)]
    #[value(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[value(rename = "dataFields", default, skip_serializing_if = "Option::is_none")]
    pub data_fields: Option<semio_framework_value_dictionary::QuestionDictionary>,
    #[state(artifact)]
    #[child(kind = "s.draw.drawing")]
    #[value(rename = "backgroundDrawing", default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingMember>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[value(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

/// 🧷️ Real "empty" constructor used as the parse/decode starting point (mirrors cad's
/// `empty_cad_snapshot`) — `default_document()` at `crate::schema` seeds a full
/// demo document instead, so this can't reuse a `Default` impl (this type has none).
pub fn empty_layout_snapshot() -> LayoutSnapshot {
    LayoutSnapshot {
        schema: String::new(),
        name: String::new(),
        grid: GridSettings { baseline_grid: 0.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields: None,
        background_drawing: None,
        referenced_model: None,
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs


//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests

//#endregion 🧪️Tests




/// 🧹️ Releases the original dictionary before the plain snapshot and member reference fields.
pub fn retire(mut value: LayoutSnapshot) {
    if let Some(dictionary) = value.data_fields.take() { <semio_framework_value_dictionary::QuestionDictionary as semio_framework_value::FromValue>::retire_decoded(dictionary); }
    drop(value);
}

/// 🎬️ Projects cold snapshot content to the common vector-scene representation.
pub mod page_projection {
    use super::LayoutSnapshot;
    use crate::Frame;
    use semio_framework_io_ports::{VectorCanvas, VectorLayer, VectorNode, VectorPoint2, VectorPoint3, VectorRgba, VectorScene, VectorSegment, VectorStyle, VectorTransform};
    use semio_os_kernel as store;

    /// 📐️ A closed axis-aligned rectangle as `MoveTo` + three `LineTo`s + `Close`, the "rects-as-paths" primitive the page and frame rects build on.
    pub fn rect_path_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<VectorSegment> {
        vec![
            VectorSegment::MoveTo { to: VectorPoint2 { x, y } },
            VectorSegment::LineTo { to: VectorPoint2 { x: x + width, y } },
            VectorSegment::LineTo { to: VectorPoint2 { x: x + width, y: y + height } },
            VectorSegment::LineTo { to: VectorPoint2 { x, y: y + height } },
            VectorSegment::Close,
        ]
    }

    fn rgba_from_channels(channels: [f32; 4]) -> VectorRgba {
        VectorRgba { r: channels[0], g: channels[1], b: channels[2], a: channels[3] }
    }

    /// 🖍️ Projects printable page content, text and images from a cold snapshot.
    pub fn layout_snapshot_to_scene(doc: &LayoutSnapshot) -> Result<VectorScene, String> {
        if doc.background_drawing.is_some() || doc.pages.iter().any(|page| page.background_drawing.is_some()) {
            return Err("layout.export.member-source-required".into());
        }
        const PAGE_GAP: f64 = 24.0;
        let mut styles = Vec::new();
        let mut layers = Vec::new();
        let mut x_offset = 0.0f64;
        let mut canvas_width = 0.0f64;
        let mut canvas_height = 0.0f64;
        for page in &doc.pages {
            let mut children = Vec::new();
            let parent = page.parent_page_id.as_ref().and_then(|id| doc.parent_pages.iter().find(|parent| &parent.id == id));
            let frames = parent.into_iter().flat_map(|parent| &parent.frames).chain(&page.frames);
            for frame in frames {
                let inherited = parent.is_some_and(|parent| parent.frames.iter().any(|candidate| std::ptr::eq(candidate, frame)));
                let override_value = inherited.then(|| page.overrides.iter().find(|value| value.object_id == frame.id())).flatten();
                if !override_value.and_then(|value| value.visible).unwrap_or_else(|| frame.visible()) { continue; }
                let visible_layer = page.layers.iter().chain(parent.into_iter().flat_map(|parent| &parent.layers)).find(|layer| layer.id == frame.layer_id()).is_none_or(|layer| layer.visible);
                if !visible_layer { continue; }
                let bounds = override_value.and_then(|value| value.bounds.as_ref()).unwrap_or_else(|| frame.bounds());
                let style_name = format!("frame-{}-{}", page.id, frame.id());
                let mut nodes = Vec::new();
                match frame {
                    Frame::Rect { fill, stroke, .. } => {
                        styles.push(VectorStyle { name: style_name.clone(), fill: fill.map(rgba_from_channels), stroke: stroke.map(rgba_from_channels), stroke_width: stroke.map(|_| 1.0), opacity: None });
                        nodes.push(VectorNode::Path { segments: rect_path_segments(-bounds.width * 0.5, -bounds.height * 0.5, bounds.width, bounds.height), style: Some(style_name) });
                    },
                    Frame::Text { story_id, inset, .. } => {
                        let story = doc.stories.iter().find(|story| &story.id == story_id).ok_or_else(|| format!("layout export: missing story {story_id}"))?;
                        let paragraph = story.style_runs.first().and_then(|run| run.paragraph_style_id.as_ref()).and_then(|id| doc.paragraph_styles.iter().find(|style| &style.id == id));
                        let size = paragraph.map_or(12.0, |style| style.font_size);
                        let leading = paragraph.map_or(size * 1.2, |style| style.leading);
                        let color = story.style_runs.first().and_then(|run| run.character_style_id.as_ref()).and_then(|id| doc.character_styles.iter().find(|style| &style.id == id)).and_then(|style| style.color).unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        styles.push(VectorStyle { name: style_name.clone(), fill: Some(rgba_from_channels(color)), stroke: None, stroke_width: None, opacity: None });
                        for (index, value) in story.content.split('\n').enumerate() {
                            nodes.push(VectorNode::Text { value: value.into(), at: VectorPoint2 { x: -bounds.width * 0.5 + inset.x, y: -bounds.height * 0.5 + inset.y + size + index as f64 * leading }, style: Some(style_name.clone()) });
                        }
                    },
                    Frame::Image { link_id, .. } => {
                        let link = doc.links.iter().find(|link| &link.id == link_id).ok_or_else(|| format!("layout export: missing link {link_id}"))?;
                        let data = link.proxy_data_url.as_deref().ok_or_else(|| format!("layout export: unresolved image {link_id}"))?;
                        let (header, data) = data.split_once(",").ok_or("layout export: image data URL")?;
                        let mime = header.strip_prefix("data:").and_then(|header| header.strip_suffix(";base64")).filter(|mime| matches!(*mime, "image/png" | "image/jpeg" | "image/gif" | "image/svg+xml")).ok_or("layout export: unsupported image data URL")?;
                        let bytes = store::pack_rt::pack_value_from_base64(data).map_err(|error| error.to_string())?;
                        nodes.push(VectorNode::Image { at: VectorPoint2 { x: -bounds.width * 0.5, y: -bounds.height * 0.5 }, width: bounds.width, height: bounds.height, mime: mime.into(), bytes });
                    },
                }
                let half = bounds.rotation * 0.5;
                children.push(VectorNode::Group { transform: VectorTransform { translation: VectorPoint3 { x: bounds.x + bounds.width * 0.5, y: bounds.y + bounds.height * 0.5, z: 0.0 }, rotation: semio_framework_io_ports::VectorQuaternion { x: 0.0, y: 0.0, z: half.sin(), w: half.cos() }, scale: VectorPoint3 { x: 1.0, y: 1.0, z: 1.0 } }, children: nodes });
            }
            layers.push(VectorLayer { id: page.id.clone(), name: page.name.clone(), visible: true, root: VectorNode::Group { transform: VectorTransform { translation: VectorPoint3 { x: x_offset, y: 0.0, z: 0.0 }, ..VectorTransform::identity() }, children } });
            canvas_width = (x_offset + page.width).max(canvas_width);
            canvas_height = page.height.max(canvas_height);
            x_offset += page.width + PAGE_GAP;
        }
        Ok(VectorScene { schema: String::new(), canvas: VectorCanvas { width: canvas_width.max(1.0), height: canvas_height.max(1.0), background: Some(VectorRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) }, styles, layers })
    }
}
