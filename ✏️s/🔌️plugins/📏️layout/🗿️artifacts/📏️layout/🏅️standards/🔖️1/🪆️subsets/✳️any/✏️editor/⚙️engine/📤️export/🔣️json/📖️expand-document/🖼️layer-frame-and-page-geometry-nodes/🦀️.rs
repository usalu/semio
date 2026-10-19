//! 🖼️ Layer, frame and page geometry nodes: the layers, frames, bounds, rectangles, insets, margins, guides and overrides of the layout export JSON expansion. See [`super`].

use super::*;

impl TypedJsonCursor {
    /// 📉 Expands the `Layer` node into its output fragments.
    pub(super) fn expand_layer(&mut self, owner: RecordOwner, layer: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = Self::owner_page(snapshot, owner)?.layers().get(layer).ok_or("layout-export-json-index")?;
        self.push_sequence(vec![
            Self::static_node(b"{\"id\":"),
            Self::string(StringSource::LayerId { owner, layer }),
            Self::static_node(b",\"name\":"),
            Self::string(StringSource::LayerName { owner, layer }),
            Self::static_node(b",\"visible\":"),
            Self::scalar(&value.visible)?,
            Self::static_node(b",\"locked\":"),
            Self::scalar(&value.locked)?,
            Self::static_node(b",\"objectIds\":"),
            TypedJsonNode::StringArray { source: StringArraySource::LayerObject { owner, layer }, index: 0, opened: false },
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🔖 Expands the `Frame` node into its output fragments.
    pub(super) fn expand_frame(&mut self, owner: RecordOwner, frame: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")?;
        let mut nodes = vec![
            Self::static_node(b"{\"kind\":"),
            Self::static_node(match value {
                Frame::Rect { .. } => b"\"rect\"",
                Frame::Text { .. } => b"\"text\"",
                Frame::Image { .. } => b"\"image\"",
            }),
            Self::static_node(b",\"id\":"),
            Self::string(StringSource::FrameId { owner, frame }),
            Self::static_node(b",\"layerId\":"),
            Self::string(StringSource::FrameLayerId { owner, frame }),
            Self::static_node(b",\"bounds\":"),
            TypedJsonNode::Bounds { owner, frame },
        ];
        match value {
            Frame::Rect { locked, visible, fill, stroke, .. } => nodes.extend([
                Self::static_node(b",\"locked\":"),
                Self::scalar(locked)?,
                Self::static_node(b",\"visible\":"),
                Self::scalar(visible)?,
                Self::static_node(b",\"fill\":"),
                Self::scalar(fill)?,
                Self::static_node(b",\"stroke\":"),
                Self::scalar(stroke)?,
            ]),
            Frame::Text { locked, visible, thread_next, columns, .. } => nodes.extend([
                Self::static_node(b",\"locked\":"),
                Self::scalar(locked)?,
                Self::static_node(b",\"visible\":"),
                Self::scalar(visible)?,
                Self::static_node(b",\"storyId\":"),
                Self::string(StringSource::FrameStoryId { owner, frame }),
                Self::static_node(b",\"threadNext\":"),
                Self::optional_string(thread_next.is_some(), StringSource::FrameThreadNext { owner, frame }),
                Self::static_node(b",\"columns\":"),
                Self::scalar(columns)?,
                Self::static_node(b",\"inset\":"),
                TypedJsonNode::TextInset { owner, frame },
                Self::static_node(b",\"wrapMode\":"),
                Self::string(StringSource::FrameWrapMode { owner, frame }),
            ]),
            Frame::Image { locked, visible, .. } => nodes.extend([
                Self::static_node(b",\"locked\":"),
                Self::scalar(locked)?,
                Self::static_node(b",\"visible\":"),
                Self::scalar(visible)?,
                Self::static_node(b",\"linkId\":"),
                Self::string(StringSource::FrameLinkId { owner, frame }),
            ]),
        }
        nodes.push(Self::static_node(b"}"));
        self.push_sequence(nodes);
        Ok(())
    }

    /// 🏷 Expands the `Bounds` node into its output fragments.
    pub(super) fn expand_bounds(&mut self, owner: RecordOwner, frame: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = frame_bounds(Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")?);
        self.push_sequence(vec![
            Self::static_node(b"{\"x\":"),
            Self::scalar(&value.x)?,
            Self::static_node(b",\"y\":"),
            Self::scalar(&value.y)?,
            Self::static_node(b",\"w\":"),
            Self::scalar(&value.width)?,
            Self::static_node(b",\"h\":"),
            Self::scalar(&value.height)?,
            Self::static_node(b",\"rotation\":"),
            Self::scalar(&value.rotation)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🔗 Expands the `Rect` node into its output fragments.
    pub(super) fn expand_rect(&mut self, page: usize, guide: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = snapshot.pages.get(page).and_then(|page| page.guides.get(guide)).ok_or("layout-export-json-index")?;
        self.push_sequence(vec![
            Self::static_node(b"{\"x\":"),
            Self::scalar(&value.x)?,
            Self::static_node(b",\"y\":"),
            Self::scalar(&value.y)?,
            Self::static_node(b",\"w\":"),
            Self::scalar(&value.width)?,
            Self::static_node(b",\"h\":"),
            Self::scalar(&value.height)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🧷 Expands the `TextInset` node into its output fragments.
    pub(super) fn expand_text_inset(&mut self, owner: RecordOwner, frame: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = match Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")? {
            Frame::Text { inset, .. } => inset,
            _ => return Err("layout-export-json-frame".into()),
        };
        self.push_sequence(vec![
            Self::static_node(b"{\"x\":"),
            Self::scalar(&value.x)?,
            Self::static_node(b",\"y\":"),
            Self::scalar(&value.y)?,
            Self::static_node(b",\"w\":"),
            Self::scalar(&value.width)?,
            Self::static_node(b",\"h\":"),
            Self::scalar(&value.height)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 📌 Expands the `Margins` node into its output fragments.
    pub(super) fn expand_margins(&mut self, page: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = &snapshot.pages.get(page).ok_or("layout-export-json-index")?.margins;
        self.push_sequence(vec![
            Self::static_node(b"{\"top\":"),
            Self::scalar(&value.top)?,
            Self::static_node(b",\"right\":"),
            Self::scalar(&value.right)?,
            Self::static_node(b",\"bottom\":"),
            Self::scalar(&value.bottom)?,
            Self::static_node(b",\"left\":"),
            Self::scalar(&value.left)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 📍 Expands the `Override` node into its output fragments.
    pub(super) fn expand_override(&mut self, page: usize, value: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let override_value = snapshot.pages.get(page).and_then(|page| page.overrides.get(value)).ok_or("layout-export-json-index")?;
        self.push_sequence(vec![
            Self::static_node(b"{\"objectId\":"),
            Self::string(StringSource::OverrideObjectId { page, value }),
            Self::static_node(b",\"bounds\":"),
            if override_value.bounds.is_some() { TypedJsonNode::OverrideBounds { page, value } } else { Self::static_node(b"null") },
            Self::static_node(b",\"visible\":"),
            Self::scalar(&override_value.visible)?,
            Self::static_node(b",\"locked\":"),
            Self::scalar(&override_value.locked)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🧭 Expands the `OverrideBounds` node into its output fragments.
    pub(super) fn expand_override_bounds(&mut self, page: usize, value: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let bounds = snapshot.pages.get(page).and_then(|page| page.overrides.get(value)).and_then(|value| value.bounds.as_ref()).ok_or("layout-export-json-index")?;
        self.push_sequence(vec![
            Self::static_node(b"{\"x\":"),
            Self::scalar(&bounds.x)?,
            Self::static_node(b",\"y\":"),
            Self::scalar(&bounds.y)?,
            Self::static_node(b",\"w\":"),
            Self::scalar(&bounds.width)?,
            Self::static_node(b",\"h\":"),
            Self::scalar(&bounds.height)?,
            Self::static_node(b",\"rotation\":"),
            Self::scalar(&bounds.rotation)?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }
}
