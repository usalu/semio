//! 🏗️ Cursor construction and owner pages of the layout export JSON: the cursor constructors, string resolution with escapes, output and node accounting, the advance step, the owner page and the frame string fields and bounds. See [`super`].

use super::*;

impl TypedJsonCursor {
    pub fn document() -> Self {
        Self { stack: vec![TypedJsonNode::Document], emitted_nodes: 0, fragment_bytes: None, drawing: None }
    }

    pub fn preflight() -> Self {
        Self { stack: vec![TypedJsonNode::MissingLinks { index: 0, opened: false, emitted: false }], emitted_nodes: 0, fragment_bytes: None, drawing: None }
    }

    pub fn manifest() -> Self {
        Self { stack: vec![TypedJsonNode::Manifest], emitted_nodes: 0, fragment_bytes: None, drawing: None }
    }

    pub fn validating(node: TypedJsonNode) -> Self {
        Self { stack: Self::fragment(node).into_iter().rev().collect(), emitted_nodes: 0, fragment_bytes: None, drawing: None }
    }

    pub(super) fn static_node(bytes: &'static [u8]) -> TypedJsonNode {
        TypedJsonNode::Static { bytes, cursor: 0 }
    }

    pub(super) fn scalar<T: JsonScalar + ?Sized>(value: &T) -> Result<TypedJsonNode, String> {
        let bytes = value.json_bytes()?;
        if bytes.len() > 256 {
            return Err("layout-export-json-scalar-limit".into());
        }
        Ok(TypedJsonNode::Scalar { bytes, cursor: 0 })
    }

    pub(super) fn string(source: StringSource) -> TypedJsonNode {
        TypedJsonNode::String { source, cursor: JsonStringWriteCursor::default() }
    }

    pub fn owned_string(value: String) -> TypedJsonNode {
        TypedJsonNode::OwnedString { value, cursor: JsonStringWriteCursor::default() }
    }

    pub(super) fn push_sequence(&mut self, nodes: Vec<TypedJsonNode>) {
        self.stack.extend(nodes.into_iter().rev());
    }

    fn fragment(node: TypedJsonNode) -> Vec<TypedJsonNode> {
        vec![TypedJsonNode::FragmentStart, node, TypedJsonNode::FragmentEnd]
    }

    pub(super) fn owner_page<'a>(snapshot: &'a LayoutSnapshot, owner: RecordOwner) -> Result<OwnerPage<'a>, String> {
        match owner {
            RecordOwner::Page(index) => snapshot.pages.get(index).map(OwnerPage::Page),
            RecordOwner::Parent(index) => snapshot.parent_pages.get(index).map(OwnerPage::Parent),
        }
        .ok_or_else(|| "layout-export-json-index".into())
    }

    pub(super) fn drawing_scene(&mut self, snapshot: &LayoutSnapshot) -> Result<Arc<VectorScene>, String> {
        if let Some(scene) = &self.drawing {
            return Ok(scene.clone());
        }
        let scene = Arc::new(semio_s_artifact_layout_layout_model::background_drawing_scene(snapshot).map_err(|error| error.to_string())?.ok_or("layout-export-json-background")?);
        self.drawing = Some(scene.clone());
        Ok(scene)
    }

    pub(super) fn draw_node<'a>(drawing: &'a VectorScene, layer: usize, path: &[usize]) -> Result<&'a VectorNode, String> {
        let mut node = &drawing.layers.get(layer).ok_or("layout-export-json-draw-layer")?.root;
        for index in path {
            node = match node {
                VectorNode::Group { children, .. } => children.get(*index).ok_or("layout-export-json-draw-child")?,
                _ => return Err("layout-export-json-draw-path".into()),
            };
        }
        Ok(node)
    }

    fn resolve_string<'a>(source: &StringSource, snapshot: &'a LayoutSnapshot, drawing: Option<&'a VectorScene>) -> Result<&'a str, String> {
        let missing = || "layout-export-json-string-source".to_string();
        Ok(match source {
            StringSource::DocumentSchema => &snapshot.schema,
            StringSource::DocumentName => &snapshot.name,
            StringSource::PrintTarget => snapshot.print_target.as_deref().ok_or_else(missing)?,
            StringSource::StringArrayElement { source, index } => match source {
                StringArraySource::SpreadPage(owner) => snapshot.spreads.get(*owner).and_then(|value| value.page_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::PageLayer(owner) => snapshot.pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::ParentLayer(owner) => snapshot.parent_pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::LayerObject { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).and_then(|value| value.object_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
            },
            StringSource::ParagraphId(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.id,
            StringSource::ParagraphName(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.name,
            StringSource::ParagraphFontFamily(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.font_family,
            StringSource::ParagraphAlignment(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.alignment,
            StringSource::CharacterId(index) => &snapshot.character_styles.get(*index).ok_or_else(missing)?.id,
            StringSource::CharacterName(index) => snapshot.character_styles.get(*index).and_then(|value| value.name.as_deref()).ok_or_else(missing)?,
            StringSource::CharacterFontFamily(index) => snapshot.character_styles.get(*index).and_then(|value| value.font_family.as_deref()).ok_or_else(missing)?,
            StringSource::StoryId(index) => &snapshot.stories.get(*index).ok_or_else(missing)?.id,
            StringSource::StoryContent(index) => &snapshot.stories.get(*index).ok_or_else(missing)?.content,
            StringSource::StoryRunParagraph { story, run } => snapshot.stories.get(*story).and_then(|value| value.style_runs.get(*run)).and_then(|value| value.paragraph_style_id.as_deref()).ok_or_else(missing)?,
            StringSource::StoryRunCharacter { story, run } => snapshot.stories.get(*story).and_then(|value| value.style_runs.get(*run)).and_then(|value| value.character_style_id.as_deref()).ok_or_else(missing)?,
            StringSource::LinkId(index) => &snapshot.links.get(*index).ok_or_else(missing)?.id,
            StringSource::LinkPath(index) => &snapshot.links.get(*index).ok_or_else(missing)?.path,
            StringSource::LinkHash(index) => &snapshot.links.get(*index).ok_or_else(missing)?.hash,
            StringSource::LinkColorProfile(index) => snapshot.links.get(*index).and_then(|value| value.color_profile.as_deref()).ok_or_else(missing)?,
            StringSource::LinkState(index) => snapshot.links.get(*index).and_then(|value| value.state.as_deref()).ok_or_else(missing)?,
            StringSource::LinkProxy(index) => snapshot.links.get(*index).and_then(|value| value.proxy_data_url.as_deref()).ok_or_else(missing)?,
            StringSource::ParentId(index) => &snapshot.parent_pages.get(*index).ok_or_else(missing)?.id,
            StringSource::ParentName(index) => &snapshot.parent_pages.get(*index).ok_or_else(missing)?.name,
            StringSource::SpreadId(index) => &snapshot.spreads.get(*index).ok_or_else(missing)?.id,
            StringSource::SpreadName(index) => &snapshot.spreads.get(*index).ok_or_else(missing)?.name,
            StringSource::PageId(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.id,
            StringSource::PageName(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.name,
            StringSource::PageSpreadId(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.spread_id,
            StringSource::PageParentId(index) => snapshot.pages.get(*index).and_then(|value| value.parent_page_id.as_deref()).ok_or_else(missing)?,
            StringSource::LayerId { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).map(|value| value.id.as_str()).ok_or_else(missing)?,
            StringSource::LayerName { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).map(|value| value.name.as_str()).ok_or_else(missing)?,
            StringSource::FrameId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::Id).ok_or_else(missing)?,
            StringSource::FrameLayerId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::LayerId).ok_or_else(missing)?,
            StringSource::FrameStoryId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::StoryId).ok_or_else(missing)?,
            StringSource::FrameThreadNext { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::ThreadNext).ok_or_else(missing)?,
            StringSource::FrameWrapMode { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::WrapMode).ok_or_else(missing)?,
            StringSource::FrameLinkId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::LinkId).ok_or_else(missing)?,
            StringSource::OverrideObjectId { page, value } => &snapshot.pages.get(*page).and_then(|page| page.overrides.get(*value)).ok_or_else(missing)?.object_id,
            StringSource::ChildId => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.child_id,
            StringSource::ChildTargetArtifactId => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.artifact_id,
            StringSource::ChildTargetArtifactKind => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.artifact_kind,
            StringSource::ChildTargetStandard => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.standard,
            StringSource::ChildTargetSubset => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.subset,
            StringSource::DrawingSchema => &drawing.ok_or_else(missing)?.schema,
            StringSource::DrawStyleName(index) => &drawing.ok_or_else(missing)?.styles.get(*index).ok_or_else(missing)?.name,
            StringSource::DrawLayerId(index) => &drawing.ok_or_else(missing)?.layers.get(*index).ok_or_else(missing)?.id,
            StringSource::DrawLayerName(index) => &drawing.ok_or_else(missing)?.layers.get(*index).ok_or_else(missing)?.name,
            StringSource::DrawNodeValue { layer, path } => match Self::draw_node(drawing.ok_or_else(missing)?, *layer, path)? {
                VectorNode::Text { value, .. } => value,
                _ => return Err(missing()),
            },
            StringSource::DrawNodeStyle { layer, path } => match Self::draw_node(drawing.ok_or_else(missing)?, *layer, path)? {
                VectorNode::Path { style: Some(value), .. } | VectorNode::Text { style: Some(value), .. } => value,
                _ => return Err(missing()),
            },
            StringSource::DrawNodeMime { layer, path } => match Self::draw_node(drawing.ok_or_else(missing)?, *layer, path)? {
                VectorNode::Image { mime, .. } => mime,
                _ => return Err(missing()),
            },
            StringSource::ReferencedTargetArtifactId => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.artifact_id,
            StringSource::ReferencedTargetArtifactKind => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.artifact_kind,
            StringSource::ReferencedTargetStandard => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.standard,
            StringSource::ReferencedTargetSubset => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.subset,
            StringSource::ReferencedRole => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.role,
            StringSource::ReferencedCheckpointId => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Checkpoint { id } => id,
                _ => return Err(missing()),
            },
            StringSource::ReferencedBlobHash => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Snapshot { blob } => &blob.hash,
                _ => return Err(missing()),
            },
            StringSource::ReferencedBlobMediaType => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Snapshot { blob } => &blob.media_type,
                _ => return Err(missing()),
            },
        })
    }

    pub(super) fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> (Vec<u8>, bool) {
        let mut output = Vec::with_capacity(JSON_OUTPUT_BYTES_PER_UNIT);
        if !cursor.opened {
            output.push(b'"');
            cursor.opened = true;
        }
        let bytes = value.as_bytes();
        while cursor.byte_cursor < bytes.len() {
            let byte = bytes[cursor.byte_cursor];
            let escaped: &[u8] = match byte {
                b'"' => b"\\\"",
                b'\\' => b"\\\\",
                b'\x08' => b"\\b",
                b'\t' => b"\\t",
                b'\n' => b"\\n",
                b'\x0c' => b"\\f",
                b'\r' => b"\\r",
                0..=0x1f => &[],
                _ => std::slice::from_ref(&bytes[cursor.byte_cursor]),
            };
            let needed = if byte <= 0x1f && escaped.is_empty() { 6 } else { escaped.len() };
            if output.len() + needed + 1 > JSON_OUTPUT_BYTES_PER_UNIT {
                break;
            }
            if byte <= 0x1f && escaped.is_empty() {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                output.extend_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]]);
            } else {
                output.extend_from_slice(escaped);
            }
            cursor.byte_cursor += 1;
        }
        if cursor.byte_cursor == bytes.len() && !cursor.closed {
            output.push(b'"');
            cursor.closed = true;
        }
        (output, cursor.closed)
    }

    pub(super) fn optional_string(present: bool, source: StringSource) -> TypedJsonNode {
        if present {
            Self::string(source)
        } else {
            Self::static_node(b"null")
        }
    }

    fn account_output(&mut self, bytes: usize) -> Result<(), String> {
        if let Some(fragment_bytes) = &mut self.fragment_bytes {
            *fragment_bytes = fragment_bytes.checked_add(bytes).ok_or("layout-export-json-fragment-limit")?;
            if *fragment_bytes > MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES {
                return Err("layout-export-json-fragment-limit".into());
            }
        }
        Ok(())
    }

    fn account_node(&mut self) -> Result<(), String> {
        self.emitted_nodes = self.emitted_nodes.checked_add(1).ok_or("layout-export-json-node-limit")?;
        let maximum = if self.fragment_bytes.is_some() { MAX_LAYOUT_EXPORT_JSON_NODES } else { MAX_LAYOUT_EXPORT_DECODED_ITEMS };
        if self.emitted_nodes > maximum {
            return Err("layout-export-json-node-limit".into());
        }
        Ok(())
    }

    pub fn advance(&mut self, snapshot: &LayoutSnapshot) -> Result<(Vec<u8>, bool), String> {
        let Some(node) = self.stack.pop() else { return Ok((Vec::new(), true)) };
        let mut output = Vec::new();
        match node {
            TypedJsonNode::FragmentStart => {
                if self.fragment_bytes.replace(0).is_some() {
                    return Err("layout-export-json-fragment-nesting".into());
                }
                self.emitted_nodes = 0;
            }
            TypedJsonNode::FragmentEnd => {
                if self.fragment_bytes.take().is_none() {
                    return Err("layout-export-json-fragment-state".into());
                }
            }
            TypedJsonNode::Static { bytes, cursor } => {
                let end = cursor.saturating_add(JSON_OUTPUT_BYTES_PER_UNIT).min(bytes.len());
                output.extend_from_slice(&bytes[cursor..end]);
                if end < bytes.len() {
                    self.stack.push(TypedJsonNode::Static { bytes, cursor: end });
                }
            }
            TypedJsonNode::Scalar { bytes, cursor } => {
                self.account_node()?;
                let end = cursor.saturating_add(JSON_OUTPUT_BYTES_PER_UNIT).min(bytes.len());
                output.extend_from_slice(&bytes[cursor..end]);
                if end < bytes.len() {
                    self.stack.push(TypedJsonNode::Scalar { bytes, cursor: end });
                }
            }
            TypedJsonNode::String { source, mut cursor } => {
                self.account_node()?;
                let maximum = MAX_LAYOUT_EXPORT_STRING_BYTES;
                let drawing = if source.is_drawing() { Some(self.drawing_scene(snapshot)?) } else { None };
                let value = Self::resolve_string(&source, snapshot, drawing.as_deref())?;
                if value.len() > maximum {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(value, &mut cursor);
                output = bytes;
                if !done {
                    self.stack.push(TypedJsonNode::String { source, cursor });
                }
            }
            TypedJsonNode::Dictionary(mut cursor) => {
                let (bytes, done) = cursor.advance(snapshot)?;
                output = bytes;
                if !done { self.stack.push(TypedJsonNode::Dictionary(cursor)); }
            }
            TypedJsonNode::OwnedString { value, mut cursor } => {
                self.account_node()?;
                if value.len() > MAX_LAYOUT_EXPORT_STRING_BYTES {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(&value, &mut cursor);
                output = bytes;
                if !done {
                    self.stack.push(TypedJsonNode::OwnedString { value, cursor });
                }
            }
            node => {
                self.account_node()?;
                self.expand(node, snapshot)?;
            }
        }
        self.account_output(output.len())?;
        Ok((output, self.stack.is_empty()))
    }

}

enum OwnerPage<'a> {
    Page(&'a Page),
    Parent(&'a semio_s_artifact_layout_layout_model::ParentPage),
}

impl<'a> OwnerPage<'a> {
    pub(super) fn layers(&self) -> &'a [semio_s_artifact_layout_layout_model::Layer] {
        match self {
            Self::Page(value) => &value.layers,
            Self::Parent(value) => &value.layers,
        }
    }

    pub(super) fn frames(&self) -> &'a [Frame] {
        match self {
            Self::Page(value) => &value.frames,
            Self::Parent(value) => &value.frames,
        }
    }
}

#[derive(Clone, Copy)]
enum FrameStringField {
    Id,
    LayerId,
    StoryId,
    ThreadNext,
    WrapMode,
    LinkId,
}

fn frame_string(frame: &Frame, field: FrameStringField) -> Option<&str> {
    match (frame, field) {
        (Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. }, FrameStringField::Id) => Some(id),
        (Frame::Rect { layer_id, .. } | Frame::Text { layer_id, .. } | Frame::Image { layer_id, .. }, FrameStringField::LayerId) => Some(layer_id),
        (Frame::Text { story_id, .. }, FrameStringField::StoryId) => Some(story_id),
        (Frame::Text { thread_next: Some(value), .. }, FrameStringField::ThreadNext) => Some(value),
        (Frame::Text { wrap_mode, .. }, FrameStringField::WrapMode) => Some(wrap_mode),
        (Frame::Image { link_id, .. }, FrameStringField::LinkId) => Some(link_id),
        _ => None,
    }
}

pub fn frame_bounds(frame: &Frame) -> &LayoutBounds {
    match frame {
        Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
    }
}
