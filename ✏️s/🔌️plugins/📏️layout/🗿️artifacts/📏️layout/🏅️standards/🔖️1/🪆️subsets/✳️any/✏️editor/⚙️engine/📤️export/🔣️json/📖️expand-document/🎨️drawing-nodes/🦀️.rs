//! 🎨️ Drawing nodes: the drawing canvas, styles, layers, nodes, children, segments, points and byte streams of the layout export JSON expansion. See [`super`].

use super::*;

impl TypedJsonCursor {
    /// 🧩 Expands the `DrawStyle` node into its output fragments.
    pub(super) fn expand_draw_style(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let scene = self.drawing_scene(snapshot)?;
        let value = scene.styles.get(index).ok_or("layout-export-json-index")?;
        let mut nodes = vec![Self::static_node(b"{\"name\":"), Self::string(StringSource::DrawStyleName(index))];
        if let Some(fill) = value.fill {
            nodes.extend([Self::static_node(b",\"fill\":"), TypedJsonNode::DrawRgba(fill)]);
        }
        if let Some(stroke) = value.stroke {
            nodes.extend([Self::static_node(b",\"stroke\":"), TypedJsonNode::DrawRgba(stroke)]);
        }
        if let Some(width) = value.stroke_width {
            nodes.extend([Self::static_node(b",\"strokeWidth\":"), Self::scalar(&width)?]);
        }
        if let Some(opacity) = value.opacity {
            nodes.extend([Self::static_node(b",\"opacity\":"), Self::scalar(&opacity)?]);
        }
        nodes.push(Self::static_node(b"}"));
        self.push_sequence(nodes);
        Ok(())
    }

    /// 🧱 Expands the `DrawLayer` node into its output fragments.
    pub(super) fn expand_draw_layer(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let scene = self.drawing_scene(snapshot)?;
        let value = scene.layers.get(index).ok_or("layout-export-json-index")?;
        self.push_sequence(vec![
            Self::static_node(b"{\"id\":"),
            Self::string(StringSource::DrawLayerId(index)),
            Self::static_node(b",\"name\":"),
            Self::string(StringSource::DrawLayerName(index)),
            Self::static_node(b",\"visible\":"),
            Self::scalar(&value.visible)?,
            Self::static_node(b",\"root\":"),
            TypedJsonNode::DrawNode { layer: index, path: Vec::new() },
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🧲 Expands the `DrawNode` node into its output fragments.
    pub(super) fn expand_draw_node(&mut self, layer: usize, path: Vec<usize>, snapshot: &LayoutSnapshot) -> Result<(), String> {
        if path.len() > 64 {
            return Err("layout-export-json-depth-limit".into());
        }
        let scene = self.drawing_scene(snapshot)?;
        let value = Self::draw_node(&scene, layer, &path)?;
        match value {
            VectorNode::Path { style, .. } => self.push_sequence(vec![
                Self::static_node(b"{\"kind\":\"path\",\"segments\":"),
                TypedJsonNode::DrawSegments { layer, path: path.clone(), index: 0, opened: false },
                if style.is_some() { Self::static_node(b",\"style\":") } else { Self::static_node(b"") },
                if style.is_some() { Self::string(StringSource::DrawNodeStyle { layer, path }) } else { Self::static_node(b"") },
                Self::static_node(b"}"),
            ]),
            VectorNode::Text { at, style, .. } => self.push_sequence(vec![
                Self::static_node(b"{\"kind\":\"text\",\"value\":"),
                Self::string(StringSource::DrawNodeValue { layer, path: path.clone() }),
                Self::static_node(b",\"at\":"),
                TypedJsonNode::DrawPoint2(*at),
                if style.is_some() { Self::static_node(b",\"style\":") } else { Self::static_node(b"") },
                if style.is_some() { Self::string(StringSource::DrawNodeStyle { layer, path }) } else { Self::static_node(b"") },
                Self::static_node(b"}"),
            ]),
            VectorNode::Group { transform, .. } => self.push_sequence(vec![
                Self::static_node(b"{\"kind\":\"group\",\"transform\":"),
                TypedJsonNode::DrawTransform(*transform),
                Self::static_node(b",\"children\":"),
                TypedJsonNode::DrawChildren { layer, path, index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            VectorNode::Image { at, width, height, .. } => self.push_sequence(vec![
                Self::static_node(b"{\"kind\":\"image\",\"at\":"),
                TypedJsonNode::DrawPoint2(*at),
                Self::static_node(b",\"width\":"),
                Self::scalar(width)?,
                Self::static_node(b",\"height\":"),
                Self::scalar(height)?,
                Self::static_node(b",\"mime\":"),
                Self::string(StringSource::DrawNodeMime { layer, path: path.clone() }),
                Self::static_node(b",\"bytes\":"),
                TypedJsonNode::DrawBytes { layer, path, index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
        }
        Ok(())
    }

    /// 🧪 Expands the `DrawChildren` node into its output fragments.
    pub(super) fn expand_draw_children(&mut self, layer: usize, path: Vec<usize>, index: usize, opened: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
        if path.len() > 64 {
            return Err("layout-export-json-depth-limit".into());
        }
        let scene = self.drawing_scene(snapshot)?;
        let len = match Self::draw_node(&scene, layer, &path)? {
            VectorNode::Group { children, .. } => children.len(),
            _ => return Err("layout-export-json-draw-path".into()),
        };
        if !opened {
            self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawChildren { layer, path, index, opened: true }]);
        } else if index < len {
            let mut child = path.clone();
            child.push(index);
            self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawNode { layer, path: child }, TypedJsonNode::DrawChildren { layer, path, index: index + 1, opened: true }]);
        } else {
            self.stack.push(Self::static_node(b"]"));
        }
        Ok(())
    }

    /// 🧰 Expands the `DrawSegments` node into its output fragments.
    pub(super) fn expand_draw_segments(&mut self, layer: usize, path: Vec<usize>, index: usize, opened: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let scene = self.drawing_scene(snapshot)?;
        let len = match Self::draw_node(&scene, layer, &path)? {
            VectorNode::Path { segments, .. } => segments.len(),
            _ => return Err("layout-export-json-draw-path".into()),
        };
        if !opened {
            self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawSegments { layer, path, index, opened: true }]);
        } else if index < len {
            self.push_sequence(vec![
                if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") },
                TypedJsonNode::DrawSegment { layer, path: path.clone(), segment: index },
                TypedJsonNode::DrawSegments { layer, path, index: index + 1, opened: true },
            ]);
        } else {
            self.stack.push(Self::static_node(b"]"));
        }
        Ok(())
    }

    /// 🧶 Expands the `DrawSegment` node into its output fragments.
    pub(super) fn expand_draw_segment(&mut self, layer: usize, path: Vec<usize>, segment: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let scene = self.drawing_scene(snapshot)?;
        let value = match Self::draw_node(&scene, layer, &path)? {
            VectorNode::Path { segments, .. } => segments.get(segment).ok_or("layout-export-json-index")?,
            _ => return Err("layout-export-json-draw-path".into()),
        };
        let nodes = match value {
            VectorSegment::MoveTo { to } => vec![Self::static_node(b"{\"kind\":\"moveTo\",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
            VectorSegment::LineTo { to } => vec![Self::static_node(b"{\"kind\":\"lineTo\",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
            VectorSegment::CubicTo { c1, c2, to } => vec![
                Self::static_node(b"{\"kind\":\"cubicTo\",\"c1\":"),
                TypedJsonNode::DrawPoint2(*c1),
                Self::static_node(b",\"c2\":"),
                TypedJsonNode::DrawPoint2(*c2),
                Self::static_node(b",\"to\":"),
                TypedJsonNode::DrawPoint2(*to),
                Self::static_node(b"}"),
            ],
            VectorSegment::QuadTo { c, to } => vec![Self::static_node(b"{\"kind\":\"quadTo\",\"c\":"), TypedJsonNode::DrawPoint2(*c), Self::static_node(b",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
            VectorSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => vec![
                Self::static_node(b"{\"kind\":\"arcTo\",\"rx\":"),
                Self::scalar(rx)?,
                Self::static_node(b",\"ry\":"),
                Self::scalar(ry)?,
                Self::static_node(b",\"xRotation\":"),
                Self::scalar(x_rotation)?,
                Self::static_node(b",\"largeArc\":"),
                Self::scalar(large_arc)?,
                Self::static_node(b",\"sweep\":"),
                Self::scalar(sweep)?,
                Self::static_node(b",\"to\":"),
                TypedJsonNode::DrawPoint2(*to),
                Self::static_node(b"}"),
            ],
            VectorSegment::Close => vec![Self::static_node(b"{\"kind\":\"close\"}")],
        };
        self.push_sequence(nodes);
        Ok(())
    }

    /// 🧵 Expands the `DrawBytes` node into its output fragments.
    pub(super) fn expand_draw_bytes(&mut self, layer: usize, path: Vec<usize>, index: usize, opened: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let scene = self.drawing_scene(snapshot)?;
        let bytes = match Self::draw_node(&scene, layer, &path)? {
            VectorNode::Image { bytes, .. } => bytes,
            _ => return Err("layout-export-json-draw-path".into()),
        };
        if !opened {
            self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawBytes { layer, path, index, opened: true }]);
        } else if let Some(value) = bytes.get(index) {
            self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, Self::scalar(value)?, TypedJsonNode::DrawBytes { layer, path, index: index + 1, opened: true }]);
        } else {
            self.stack.push(Self::static_node(b"]"));
        }
        Ok(())
    }
}
