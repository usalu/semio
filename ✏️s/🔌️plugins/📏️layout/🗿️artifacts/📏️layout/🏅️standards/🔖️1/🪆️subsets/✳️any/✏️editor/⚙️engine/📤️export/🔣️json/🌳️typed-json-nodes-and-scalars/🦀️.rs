//! 🌳️ Typed JSON nodes and scalars of the layout export JSON: the typed node tree, the typed cursor record, the owned byte accounting, the scalar trait with its bool, float, option and vector impls and the finite number writer. See [`super`].

use super::*;

#[derive(Clone, Debug)]
pub enum TypedJsonNode {
    FragmentStart,
    FragmentEnd,
    Static { bytes: &'static [u8], cursor: usize },
    Scalar { bytes: Vec<u8>, cursor: usize },
    String { source: StringSource, cursor: JsonStringWriteCursor },
    OwnedString { value: String, cursor: JsonStringWriteCursor },
    Document,
    Dictionary(DictionaryJsonCursor),
    Grid,
    TopArray { kind: TopArrayKind, index: usize, opened: bool },
    Paragraph(usize),
    Character(usize),
    Story(usize),
    StoryRuns { story: usize, index: usize, opened: bool },
    StoryRun { story: usize, run: usize },
    Link(usize),
    Parent(usize),
    Spread(usize),
    Page(usize),
    StringArray { source: StringArraySource, index: usize, opened: bool },
    Layers { owner: RecordOwner, index: usize, opened: bool },
    Layer { owner: RecordOwner, layer: usize },
    Frames { owner: RecordOwner, index: usize, opened: bool },
    Frame { owner: RecordOwner, frame: usize },
    Bounds { owner: RecordOwner, frame: usize },
    Rect { page: usize, guide: usize },
    TextInset { owner: RecordOwner, frame: usize },
    Margins(usize),
    Columns(usize),
    Guides { page: usize, index: usize, opened: bool },
    Overrides { page: usize, index: usize, opened: bool },
    Override { page: usize, value: usize },
    OverrideBounds { page: usize, value: usize },
    BackgroundDrawing,
    ChildHandle,
    ArtifactRef { referenced: bool },
    Dialect { referenced: bool },
    Drawing,
    DrawCanvas,
    DrawRgba(VectorRgba),
    DrawStyles { index: usize, opened: bool },
    DrawStyle(usize),
    DrawLayers { index: usize, opened: bool },
    DrawLayer(usize),
    DrawNode { layer: usize, path: Vec<usize> },
    DrawChildren { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    DrawSegments { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    DrawSegment { layer: usize, path: Vec<usize>, segment: usize },
    DrawPoint2(VectorPoint2),
    DrawPoint3(VectorPoint3),
    DrawQuaternion(VectorQuaternion),
    DrawTransform(VectorTransform),
    DrawBytes { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    ReferencedModel,
    LinkPin,
    BlobRef,
    MissingLinks { index: usize, opened: bool, emitted: bool },
    MissingLink(usize),
    Manifest,
    ManifestLinks { index: usize, opened: bool },
    ManifestLink(usize),
}

#[derive(Clone, Debug, Default)]
pub struct TypedJsonCursor {
    pub stack: Vec<TypedJsonNode>,
    pub emitted_nodes: usize,
    pub fragment_bytes: Option<usize>,
    pub(super) drawing: Option<Arc<VectorScene>>,
}

fn typed_string_source_owned_bytes(source: &StringSource) -> usize {
    match source {
        StringSource::DrawNodeValue { path, .. } | StringSource::DrawNodeStyle { path, .. } | StringSource::DrawNodeMime { path, .. } => path.len().saturating_mul(size_of::<usize>()),
        _ => 0,
    }
}

pub fn typed_json_node_owned_bytes(node: &TypedJsonNode) -> usize {
    match node {
        TypedJsonNode::Scalar { bytes, .. } => bytes.len(),
        TypedJsonNode::Dictionary(cursor) => cursor.owned_bytes(),
        TypedJsonNode::String { source, .. } => typed_string_source_owned_bytes(source),
        TypedJsonNode::OwnedString { value, .. } => value.len(),
        TypedJsonNode::DrawNode { path, .. } | TypedJsonNode::DrawChildren { path, .. } | TypedJsonNode::DrawSegments { path, .. } | TypedJsonNode::DrawSegment { path, .. } | TypedJsonNode::DrawBytes { path, .. } => {
            path.len().saturating_mul(size_of::<usize>())
        }
        _ => 0,
    }
}

pub trait JsonScalar {
    fn json_bytes(&self) -> Result<Vec<u8>, String>;
}

pub fn finite_json_number(value: f64) -> Result<Vec<u8>, String> {
    if !value.is_finite() {
        return Err("layout-export-json-number".into());
    }
    let mut text = value.to_string();
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        text.push_str(".0");
    }
    Ok(text.into_bytes())
}

impl JsonScalar for bool {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(if *self { b"true".to_vec() } else { b"false".to_vec() })
    }
}

impl JsonScalar for f64 {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        finite_json_number(*self)
    }
}

impl JsonScalar for f32 {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        if !self.is_finite() {
            return Err("layout-export-json-number".into());
        }
        let mut text = self.to_string();
        if !text.contains('.') && !text.contains('e') && !text.contains('E') {
            text.push_str(".0");
        }
        Ok(text.into_bytes())
    }
}

impl<T: JsonScalar> JsonScalar for Option<T> {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        self.as_ref().map_or_else(|| Ok(b"null".to_vec()), JsonScalar::json_bytes)
    }
}

impl JsonScalar for [f32; 4] {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::with_capacity(64);
        bytes.push(b'[');
        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                bytes.push(b',');
            }
            bytes.extend_from_slice(&value.json_bytes()?);
        }
        bytes.push(b']');
        Ok(bytes)
    }
}
