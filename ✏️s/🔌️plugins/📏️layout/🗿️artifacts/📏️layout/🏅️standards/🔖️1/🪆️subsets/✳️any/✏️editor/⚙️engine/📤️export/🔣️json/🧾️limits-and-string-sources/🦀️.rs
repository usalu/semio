//! 🧾️ Limits and string sources of the layout export JSON: the node, string, item and fragment caps, the top array, record owner and string array kinds, the string source and the string write cursor. See [`super`].

pub const MAX_LAYOUT_EXPORT_JSON_NODES: usize = 2_048;

pub const MAX_LAYOUT_EXPORT_STRING_BYTES: usize = 8 << 10;

pub const MAX_LAYOUT_EXPORT_DECODED_ITEMS: usize = 131_072;

pub const MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES: usize = 64 << 10;

pub(super) const JSON_OUTPUT_BYTES_PER_UNIT: usize = 1_024;

#[derive(Clone, Copy, Debug)]
pub(super) enum TopArrayKind {
    ParagraphStyles,
    CharacterStyles,
    Stories,
    Links,
    ParentPages,
    Spreads,
    Pages,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RecordOwner {
    Page(usize),
    Parent(usize),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum StringArraySource {
    SpreadPage(usize),
    PageLayer(usize),
    ParentLayer(usize),
    LayerObject { owner: RecordOwner, layer: usize },
}

#[derive(Clone, Debug)]
pub enum StringSource {
    DocumentSchema,
    DocumentName,
    PrintTarget,
    StringArrayElement { source: StringArraySource, index: usize },
    ParagraphId(usize),
    ParagraphName(usize),
    ParagraphFontFamily(usize),
    ParagraphAlignment(usize),
    CharacterId(usize),
    CharacterName(usize),
    CharacterFontFamily(usize),
    StoryId(usize),
    StoryContent(usize),
    StoryRunParagraph { story: usize, run: usize },
    StoryRunCharacter { story: usize, run: usize },
    LinkId(usize),
    LinkPath(usize),
    LinkHash(usize),
    LinkColorProfile(usize),
    LinkState(usize),
    LinkProxy(usize),
    ParentId(usize),
    ParentName(usize),
    SpreadId(usize),
    SpreadName(usize),
    PageId(usize),
    PageName(usize),
    PageSpreadId(usize),
    PageParentId(usize),
    LayerId { owner: RecordOwner, layer: usize },
    LayerName { owner: RecordOwner, layer: usize },
    FrameId { owner: RecordOwner, frame: usize },
    FrameLayerId { owner: RecordOwner, frame: usize },
    FrameStoryId { owner: RecordOwner, frame: usize },
    FrameThreadNext { owner: RecordOwner, frame: usize },
    FrameWrapMode { owner: RecordOwner, frame: usize },
    FrameLinkId { owner: RecordOwner, frame: usize },
    OverrideObjectId { page: usize, value: usize },
    ChildId,
    ChildTargetArtifactId,
    ChildTargetArtifactKind,
    ChildTargetStandard,
    ChildTargetSubset,
    DrawingSchema,
    DrawStyleName(usize),
    DrawLayerId(usize),
    DrawLayerName(usize),
    DrawNodeValue { layer: usize, path: Vec<usize> },
    DrawNodeStyle { layer: usize, path: Vec<usize> },
    DrawNodeMime { layer: usize, path: Vec<usize> },
    ReferencedTargetArtifactId,
    ReferencedTargetArtifactKind,
    ReferencedTargetStandard,
    ReferencedTargetSubset,
    ReferencedRole,
    ReferencedCheckpointId,
    ReferencedBlobHash,
    ReferencedBlobMediaType,
}

impl StringSource {
    pub(super) fn is_drawing(&self) -> bool {
        matches!(self, Self::DrawingSchema | Self::DrawStyleName(_) | Self::DrawLayerId(_) | Self::DrawLayerName(_) | Self::DrawNodeValue { .. } | Self::DrawNodeStyle { .. } | Self::DrawNodeMime { .. })
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct JsonStringWriteCursor {
    pub(super) byte_cursor: usize,
    pub(super) opened: bool,
    pub(super) closed: bool,
}
