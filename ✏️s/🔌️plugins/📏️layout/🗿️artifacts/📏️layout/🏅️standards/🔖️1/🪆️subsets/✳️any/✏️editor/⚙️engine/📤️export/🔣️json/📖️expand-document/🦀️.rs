//! 📖️ Expand document of the layout export JSON: the bounded expansion of one typed node into output bytes for the document, page, frame, story, style, layer and package fragments. See [`super`].

use super::*;

#[path = "📄️document-and-page-nodes/🦀️.rs"]
mod document_and_page_nodes;

#[path = "🖼️layer-frame-and-page-geometry-nodes/🦀️.rs"]
mod layer_frame_and_page_geometry_nodes;

#[path = "📦️package-and-manifest-nodes/🦀️.rs"]
mod package_and_manifest_nodes;

impl TypedJsonCursor {
    pub(super) fn expand(&mut self, node: TypedJsonNode, snapshot: &LayoutSnapshot) -> Result<(), String> {
        match node {
            TypedJsonNode::Document => self.expand_document(snapshot)?,
            TypedJsonNode::Grid => self.push_sequence(vec![
                Self::static_node(b"{\"baselineGrid\":"),
                Self::scalar(&snapshot.grid.baseline_grid)?,
                Self::static_node(b",\"baselineOffset\":"),
                Self::scalar(&snapshot.grid.baseline_offset)?,
                Self::static_node(b",\"snapToBaseline\":"),
                Self::scalar(&snapshot.grid.snap_to_baseline)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::TopArray { kind, index, opened } => self.expand_top_array(kind, index, opened, snapshot)?,
            TypedJsonNode::Paragraph(index) => self.expand_paragraph(index, snapshot)?,
            TypedJsonNode::Character(index) => self.expand_character(index, snapshot)?,
            TypedJsonNode::Story(index) => self.push_sequence(vec![
                Self::static_node(b"{\"id\":"),
                Self::string(StringSource::StoryId(index)),
                Self::static_node(b",\"content\":"),
                Self::string(StringSource::StoryContent(index)),
                Self::static_node(b",\"styleRuns\":"),
                TypedJsonNode::StoryRuns { story: index, index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::StoryRuns { story, index, opened } => {
                let runs = &snapshot.stories.get(story).ok_or("layout-export-json-index")?.style_runs;
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::StoryRuns { story, index, opened: true }]);
                } else if index < runs.len() {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::StoryRun { story, run: index }, TypedJsonNode::StoryRuns { story, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::StoryRun { story, run } => self.expand_story_run(story, run, snapshot)?,
            TypedJsonNode::Link(index) => self.expand_link(index, snapshot)?,
            TypedJsonNode::Parent(index) => self.expand_parent(index, snapshot)?,
            TypedJsonNode::Spread(index) => self.push_sequence(vec![
                Self::static_node(b"{\"id\":"),
                Self::string(StringSource::SpreadId(index)),
                Self::static_node(b",\"name\":"),
                Self::string(StringSource::SpreadName(index)),
                Self::static_node(b",\"pageIds\":"),
                TypedJsonNode::StringArray { source: StringArraySource::SpreadPage(index), index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Page(index) => self.expand_page(index, snapshot)?,
            TypedJsonNode::StringArray { source, index, opened } => self.expand_string_array(source, index, opened, snapshot)?,
            TypedJsonNode::Layers { owner, index, opened } => {
                let len = Self::owner_page(snapshot, owner)?.layers().len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Layers { owner, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Layer { owner, layer: index }, TypedJsonNode::Layers { owner, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Layer { owner, layer } => self.expand_layer(owner, layer, snapshot)?,
            TypedJsonNode::Frames { owner, index, opened } => {
                let len = Self::owner_page(snapshot, owner)?.frames().len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Frames { owner, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Frame { owner, frame: index }, TypedJsonNode::Frames { owner, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Frame { owner, frame } => self.expand_frame(owner, frame, snapshot)?,
            TypedJsonNode::Bounds { owner, frame } => self.expand_bounds(owner, frame, snapshot)?,
            TypedJsonNode::Rect { page, guide } => self.expand_rect(page, guide, snapshot)?,
            TypedJsonNode::TextInset { owner, frame } => self.expand_text_inset(owner, frame, snapshot)?,
            TypedJsonNode::Margins(page) => self.expand_margins(page, snapshot)?,
            TypedJsonNode::Columns(page) => {
                let value = &snapshot.pages.get(page).ok_or("layout-export-json-index")?.columns;
                self.push_sequence(vec![Self::static_node(b"{\"count\":"), Self::scalar(&value.count)?, Self::static_node(b",\"gutter\":"), Self::scalar(&value.gutter)?, Self::static_node(b"}")]);
            }
            TypedJsonNode::Guides { page, index, opened } => {
                let len = snapshot.pages.get(page).ok_or("layout-export-json-index")?.guides.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Guides { page, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Rect { page, guide: index }, TypedJsonNode::Guides { page, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Overrides { page, index, opened } => {
                let len = snapshot.pages.get(page).ok_or("layout-export-json-index")?.overrides.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Overrides { page, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Override { page, value: index }, TypedJsonNode::Overrides { page, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Override { page, value } => self.expand_override(page, value, snapshot)?,
            TypedJsonNode::OverrideBounds { page, value } => self.expand_override_bounds(page, value, snapshot)?,
            TypedJsonNode::BackgroundDrawing => self.stack.push(TypedJsonNode::Child(ChildJsonCursor::new(semio_s_artifact_layout_layout_model::DrawingSourceAddress::Document))),
            TypedJsonNode::ChildHandle => {
                self.push_sequence(vec![Self::static_node(b"{\"childId\":"), Self::string(StringSource::ChildId), Self::static_node(b",\"target\":"), TypedJsonNode::ArtifactRef { referenced: false }, Self::static_node(b"}")]);
            }
            TypedJsonNode::ArtifactRef { referenced } => self.push_sequence(vec![
                Self::static_node(b"{\"artifactId\":"),
                Self::string(if referenced { StringSource::ReferencedTargetArtifactId } else { StringSource::ChildTargetArtifactId }),
                Self::static_node(b",\"dialect\":"),
                TypedJsonNode::Dialect { referenced },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Dialect { referenced } => self.push_sequence(vec![
                Self::static_node(b"{\"artifactKind\":"),
                Self::string(if referenced { StringSource::ReferencedTargetArtifactKind } else { StringSource::ChildTargetArtifactKind }),
                Self::static_node(b",\"standard\":"),
                Self::string(if referenced { StringSource::ReferencedTargetStandard } else { StringSource::ChildTargetStandard }),
                Self::static_node(b",\"subset\":"),
                Self::string(if referenced { StringSource::ReferencedTargetSubset } else { StringSource::ChildTargetSubset }),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::ReferencedModel => self.push_sequence(vec![
                Self::static_node(b"{\"target\":"),
                TypedJsonNode::ArtifactRef { referenced: true },
                Self::static_node(b",\"pin\":"),
                TypedJsonNode::LinkPin,
                Self::static_node(b",\"role\":"),
                Self::string(StringSource::ReferencedRole),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::LinkPin => {
                let pin = &snapshot.referenced_model.as_ref().ok_or("layout-export-json-reference")?.pin;
                match pin {
                    store::LinkPin::Head => self.stack.push(Self::static_node(b"{\"kind\":\"head\"}")),
                    store::LinkPin::Checkpoint { .. } => self.push_sequence(vec![Self::static_node(b"{\"kind\":\"checkpoint\",\"id\":"), Self::string(StringSource::ReferencedCheckpointId), Self::static_node(b"}")]),
                    store::LinkPin::Snapshot { .. } => self.push_sequence(vec![Self::static_node(b"{\"kind\":\"snapshot\",\"blob\":"), TypedJsonNode::BlobRef, Self::static_node(b"}")]),
                }
            }
            TypedJsonNode::BlobRef => self.expand_blob_ref(snapshot)?,
            TypedJsonNode::MissingLinks { index, opened, emitted } => self.expand_missing_links(index, opened, emitted, snapshot)?,
            TypedJsonNode::MissingLink(index) => {
                self.push_sequence(vec![Self::static_node(b"{\"kind\":\"missing-link\",\"linkId\":"), Self::string(StringSource::LinkId(index)), Self::static_node(b",\"path\":"), Self::string(StringSource::LinkPath(index)), Self::static_node(b"}")]);
            }
            TypedJsonNode::Manifest => self.push_sequence(vec![
                Self::static_node(b"{\"schema\":\"layout.package-manifest/v1\",\"document\":\"document.json\",\"preflight\":\"preflight-report.json\",\"links\":"),
                TypedJsonNode::ManifestLinks { index: 0, opened: false },
                Self::static_node(b",\"generatedAt\":\"2026-07-02T00:00:00Z\"}"),
            ]),
            TypedJsonNode::ManifestLinks { index, opened } => {
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::ManifestLinks { index, opened: true }]);
                } else if index < snapshot.links.len() {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::ManifestLink(index), TypedJsonNode::ManifestLinks { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::ManifestLink(index) => self.expand_manifest_link(index, snapshot)?,
            TypedJsonNode::Child(_) | TypedJsonNode::Dictionary(_) | TypedJsonNode::FragmentStart | TypedJsonNode::FragmentEnd | TypedJsonNode::Static { .. } | TypedJsonNode::Scalar { .. } | TypedJsonNode::String { .. } | TypedJsonNode::OwnedString { .. } => {
                unreachable!("leaf nodes are handled before expansion")
            }
        }
        Ok(())
    }
}
