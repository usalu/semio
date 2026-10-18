//! 📖️ Expand document of the layout export JSON: the bounded expansion of one typed node into output bytes for the document, page, frame, story, style, layer and package fragments. See [`super`].

use super::*;

impl TypedJsonCursor {
    pub(super) fn expand(&mut self, node: TypedJsonNode, snapshot: &LayoutSnapshot) -> Result<(), String> {
        match node {
            TypedJsonNode::Document => {
                let mut nodes = vec![
                    Self::static_node(b"{\"schema\":"),
                    Self::string(StringSource::DocumentSchema),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::DocumentName),
                    Self::static_node(b",\"grid\":"),
                    TypedJsonNode::Grid,
                    Self::static_node(b",\"paragraphStyles\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::ParagraphStyles, index: 0, opened: false },
                    Self::static_node(b",\"characterStyles\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::CharacterStyles, index: 0, opened: false },
                    Self::static_node(b",\"stories\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Stories, index: 0, opened: false },
                    Self::static_node(b",\"links\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Links, index: 0, opened: false },
                    Self::static_node(b",\"parentPages\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::ParentPages, index: 0, opened: false },
                    Self::static_node(b",\"spreads\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Spreads, index: 0, opened: false },
                    Self::static_node(b",\"pages\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Pages, index: 0, opened: false },
                    Self::static_node(b",\"printTarget\":"),
                    Self::optional_string(snapshot.print_target.is_some(), StringSource::PrintTarget),
                ];
                if snapshot.data_fields.is_some() {
                    nodes.extend([Self::static_node(b",\"dataFields\":"), TypedJsonNode::Dictionary(DictionaryJsonCursor::new())]);
                }
                if snapshot.background_drawing.is_some() {
                    nodes.extend([Self::static_node(b",\"backgroundDrawing\":"), TypedJsonNode::BackgroundDrawing]);
                }
                if snapshot.referenced_model.is_some() {
                    nodes.extend([Self::static_node(b",\"referencedModel\":"), TypedJsonNode::ReferencedModel]);
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::Grid => self.push_sequence(vec![
                Self::static_node(b"{\"baselineGrid\":"),
                Self::scalar(&snapshot.grid.baseline_grid)?,
                Self::static_node(b",\"baselineOffset\":"),
                Self::scalar(&snapshot.grid.baseline_offset)?,
                Self::static_node(b",\"snapToBaseline\":"),
                Self::scalar(&snapshot.grid.snap_to_baseline)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::TopArray { kind, index, opened } => {
                let len = match kind {
                    TopArrayKind::ParagraphStyles => snapshot.paragraph_styles.len(),
                    TopArrayKind::CharacterStyles => snapshot.character_styles.len(),
                    TopArrayKind::Stories => snapshot.stories.len(),
                    TopArrayKind::Links => snapshot.links.len(),
                    TopArrayKind::ParentPages => snapshot.parent_pages.len(),
                    TopArrayKind::Spreads => snapshot.spreads.len(),
                    TopArrayKind::Pages => snapshot.pages.len(),
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::TopArray { kind, index, opened: true }]);
                } else if index < len {
                    let item = match kind {
                        TopArrayKind::ParagraphStyles => TypedJsonNode::Paragraph(index),
                        TopArrayKind::CharacterStyles => TypedJsonNode::Character(index),
                        TopArrayKind::Stories => TypedJsonNode::Story(index),
                        TopArrayKind::Links => TypedJsonNode::Link(index),
                        TopArrayKind::ParentPages => TypedJsonNode::Parent(index),
                        TopArrayKind::Spreads => TypedJsonNode::Spread(index),
                        TopArrayKind::Pages => TypedJsonNode::Page(index),
                    };
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, item, TypedJsonNode::TopArray { kind, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Paragraph(index) => {
                let value = snapshot.paragraph_styles.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::ParagraphId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::ParagraphName(index)),
                    Self::static_node(b",\"fontFamily\":"),
                    Self::string(StringSource::ParagraphFontFamily(index)),
                    Self::static_node(b",\"fontSize\":"),
                    Self::scalar(&value.font_size)?,
                    Self::static_node(b",\"fontWeight\":"),
                    Self::scalar(&value.font_weight)?,
                    Self::static_node(b",\"leading\":"),
                    Self::scalar(&value.leading)?,
                    Self::static_node(b",\"tracking\":"),
                    Self::scalar(&value.tracking)?,
                    Self::static_node(b",\"alignment\":"),
                    Self::string(StringSource::ParagraphAlignment(index)),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Character(index) => {
                let value = snapshot.character_styles.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::CharacterId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::optional_string(value.name.is_some(), StringSource::CharacterName(index)),
                    Self::static_node(b",\"fontFamily\":"),
                    Self::optional_string(value.font_family.is_some(), StringSource::CharacterFontFamily(index)),
                    Self::static_node(b",\"fontSize\":"),
                    Self::scalar(&value.font_size)?,
                    Self::static_node(b",\"fontWeight\":"),
                    Self::scalar(&value.font_weight)?,
                    Self::static_node(b",\"italic\":"),
                    Self::scalar(&value.italic)?,
                    Self::static_node(b",\"color\":"),
                    Self::scalar(&value.color)?,
                    Self::static_node(b",\"tracking\":"),
                    Self::scalar(&value.tracking)?,
                    Self::static_node(b"}"),
                ]);
            }
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
            TypedJsonNode::StoryRun { story, run } => {
                let value = snapshot.stories.get(story).and_then(|story| story.style_runs.get(run)).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"start\":"),
                    Self::scalar(&value.start)?,
                    Self::static_node(b",\"end\":"),
                    Self::scalar(&value.end)?,
                    Self::static_node(b",\"paragraphStyleId\":"),
                    Self::optional_string(value.paragraph_style_id.is_some(), StringSource::StoryRunParagraph { story, run }),
                    Self::static_node(b",\"characterStyleId\":"),
                    Self::optional_string(value.character_style_id.is_some(), StringSource::StoryRunCharacter { story, run }),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Link(index) => {
                let value = snapshot.links.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::LinkId(index)),
                    Self::static_node(b",\"path\":"),
                    Self::string(StringSource::LinkPath(index)),
                    Self::static_node(b",\"hash\":"),
                    Self::string(StringSource::LinkHash(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"dpi\":"),
                    Self::scalar(&value.dpi)?,
                    Self::static_node(b",\"colorProfile\":"),
                    Self::optional_string(value.color_profile.is_some(), StringSource::LinkColorProfile(index)),
                    Self::static_node(b",\"state\":"),
                    Self::optional_string(value.state.is_some(), StringSource::LinkState(index)),
                    Self::static_node(b",\"proxyDataUrl\":"),
                    Self::optional_string(value.proxy_data_url.is_some(), StringSource::LinkProxy(index)),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Parent(index) => {
                let value = snapshot.parent_pages.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::ParentId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::ParentName(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"layerIds\":"),
                    TypedJsonNode::StringArray { source: StringArraySource::ParentLayer(index), index: 0, opened: false },
                    Self::static_node(b",\"layers\":"),
                    TypedJsonNode::Layers { owner: RecordOwner::Parent(index), index: 0, opened: false },
                    Self::static_node(b",\"frames\":"),
                    TypedJsonNode::Frames { owner: RecordOwner::Parent(index), index: 0, opened: false },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Spread(index) => self.push_sequence(vec![
                Self::static_node(b"{\"id\":"),
                Self::string(StringSource::SpreadId(index)),
                Self::static_node(b",\"name\":"),
                Self::string(StringSource::SpreadName(index)),
                Self::static_node(b",\"pageIds\":"),
                TypedJsonNode::StringArray { source: StringArraySource::SpreadPage(index), index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Page(index) => {
                let value = snapshot.pages.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::PageId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::PageName(index)),
                    Self::static_node(b",\"spreadId\":"),
                    Self::string(StringSource::PageSpreadId(index)),
                    Self::static_node(b",\"parentPageId\":"),
                    Self::optional_string(value.parent_page_id.is_some(), StringSource::PageParentId(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"margins\":"),
                    TypedJsonNode::Margins(index),
                    Self::static_node(b",\"columns\":"),
                    TypedJsonNode::Columns(index),
                    Self::static_node(b",\"guides\":"),
                    TypedJsonNode::Guides { page: index, index: 0, opened: false },
                    Self::static_node(b",\"layerIds\":"),
                    TypedJsonNode::StringArray { source: StringArraySource::PageLayer(index), index: 0, opened: false },
                    Self::static_node(b",\"layers\":"),
                    TypedJsonNode::Layers { owner: RecordOwner::Page(index), index: 0, opened: false },
                    Self::static_node(b",\"frames\":"),
                    TypedJsonNode::Frames { owner: RecordOwner::Page(index), index: 0, opened: false },
                    Self::static_node(b",\"overrides\":"),
                    TypedJsonNode::Overrides { page: index, index: 0, opened: false },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::StringArray { source, index, opened } => {
                let values = match source {
                    StringArraySource::SpreadPage(owner) => &snapshot.spreads.get(owner).ok_or("layout-export-json-index")?.page_ids,
                    StringArraySource::PageLayer(owner) => &snapshot.pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::ParentLayer(owner) => &snapshot.parent_pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::LayerObject { owner, layer } => &Self::owner_page(snapshot, owner)?.layers().get(layer).ok_or("layout-export-json-index")?.object_ids,
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::StringArray { source, index, opened: true }]);
                } else if values.get(index).is_some() {
                    self.push_sequence(vec![
                        if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") },
                        Self::string(StringSource::StringArrayElement { source, index }),
                        TypedJsonNode::StringArray { source, index: index + 1, opened: true },
                    ]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
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
            TypedJsonNode::Layer { owner, layer } => {
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
            }
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
            TypedJsonNode::Frame { owner, frame } => {
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
            }
            TypedJsonNode::Bounds { owner, frame } => {
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
            }
            TypedJsonNode::Rect { page, guide } => {
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
            }
            TypedJsonNode::TextInset { owner, frame } => {
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
            }
            TypedJsonNode::Margins(page) => {
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
            }
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
            TypedJsonNode::Override { page, value } => {
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
            }
            TypedJsonNode::OverrideBounds { page, value } => {
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
            }
            TypedJsonNode::BackgroundDrawing => self.push_sequence(vec![Self::static_node(b"{\"handle\":"), TypedJsonNode::ChildHandle, Self::static_node(b",\"content\":"), TypedJsonNode::Drawing, Self::static_node(b"}")]),
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
            TypedJsonNode::Drawing => self.push_sequence(vec![
                Self::static_node(b"{\"schema\":"),
                Self::string(StringSource::DrawingSchema),
                Self::static_node(b",\"canvas\":"),
                TypedJsonNode::DrawCanvas,
                Self::static_node(b",\"styles\":"),
                TypedJsonNode::DrawStyles { index: 0, opened: false },
                Self::static_node(b",\"layers\":"),
                TypedJsonNode::DrawLayers { index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawCanvas => {
                let scene = self.drawing_scene(snapshot)?;
                let value = &scene.canvas;
                let mut nodes = vec![Self::static_node(b"{\"width\":"), Self::scalar(&value.width)?, Self::static_node(b",\"height\":"), Self::scalar(&value.height)?];
                if let Some(background) = value.background {
                    nodes.extend([Self::static_node(b",\"background\":"), TypedJsonNode::DrawRgba(background)]);
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::DrawRgba(value) => self.push_sequence(vec![
                Self::static_node(b"{\"r\":"),
                Self::scalar(&value.r)?,
                Self::static_node(b",\"g\":"),
                Self::scalar(&value.g)?,
                Self::static_node(b",\"b\":"),
                Self::scalar(&value.b)?,
                Self::static_node(b",\"a\":"),
                Self::scalar(&value.a)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawStyles { index, opened } => {
                let len = self.drawing_scene(snapshot)?.styles.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawStyles { index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawStyle(index), TypedJsonNode::DrawStyles { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawStyle(index) => {
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
            }
            TypedJsonNode::DrawLayers { index, opened } => {
                let len = self.drawing_scene(snapshot)?.layers.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawLayers { index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawLayer(index), TypedJsonNode::DrawLayers { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawLayer(index) => {
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
            }
            TypedJsonNode::DrawNode { layer, path } => {
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
            }
            TypedJsonNode::DrawChildren { layer, path, index, opened } => {
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
            }
            TypedJsonNode::DrawSegments { layer, path, index, opened } => {
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
            }
            TypedJsonNode::DrawSegment { layer, path, segment } => {
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
            }
            TypedJsonNode::DrawPoint2(value) => self.push_sequence(vec![Self::static_node(b"{\"x\":"), Self::scalar(&value.x)?, Self::static_node(b",\"y\":"), Self::scalar(&value.y)?, Self::static_node(b"}")]),
            TypedJsonNode::DrawPoint3(value) => {
                self.push_sequence(vec![Self::static_node(b"{\"x\":"), Self::scalar(&value.x)?, Self::static_node(b",\"y\":"), Self::scalar(&value.y)?, Self::static_node(b",\"z\":"), Self::scalar(&value.z)?, Self::static_node(b"}")]);
            }
            TypedJsonNode::DrawQuaternion(value) => self.push_sequence(vec![
                Self::static_node(b"{\"x\":"),
                Self::scalar(&value.x)?,
                Self::static_node(b",\"y\":"),
                Self::scalar(&value.y)?,
                Self::static_node(b",\"z\":"),
                Self::scalar(&value.z)?,
                Self::static_node(b",\"w\":"),
                Self::scalar(&value.w)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawTransform(value) => self.push_sequence(vec![
                Self::static_node(b"{\"translation\":"),
                TypedJsonNode::DrawPoint3(value.translation),
                Self::static_node(b",\"rotation\":"),
                TypedJsonNode::DrawQuaternion(value.rotation),
                Self::static_node(b",\"scale\":"),
                TypedJsonNode::DrawPoint3(value.scale),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawBytes { layer, path, index, opened } => {
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
            }
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
            TypedJsonNode::BlobRef => {
                let blob = match &snapshot.referenced_model.as_ref().ok_or("layout-export-json-reference")?.pin {
                    store::LinkPin::Snapshot { blob } => blob,
                    _ => return Err("layout-export-json-reference".into()),
                };
                self.push_sequence(vec![
                    Self::static_node(b"{\"hash\":"),
                    Self::string(StringSource::ReferencedBlobHash),
                    Self::static_node(b",\"size\":"),
                    Self::scalar(&blob.size)?,
                    Self::static_node(b",\"mediaType\":"),
                    Self::string(StringSource::ReferencedBlobMediaType),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::MissingLinks { index, opened, emitted } => {
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::MissingLinks { index, opened: true, emitted }]);
                } else if let Some(link) = snapshot.links.get(index) {
                    if link.state.as_deref() == Some("missing") {
                        self.push_sequence(vec![if emitted { Self::static_node(b",") } else { Self::static_node(b"") }, TypedJsonNode::MissingLink(index), TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted: true }]);
                    } else {
                        self.stack.push(TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted });
                    }
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
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
            TypedJsonNode::ManifestLink(index) => {
                let link = snapshot.links.get(index).ok_or("layout-export-json-index")?;
                let hash = if link.hash.is_empty() { Self::owned_string(format!("sha256:{}", semio_framework_hash::sha256_hex(link.path.as_bytes()))) } else { Self::string(StringSource::LinkHash(index)) };
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::LinkId(index)),
                    Self::static_node(b",\"path\":"),
                    Self::string(StringSource::LinkPath(index)),
                    Self::static_node(b",\"hash\":"),
                    hash,
                    Self::static_node(b",\"state\":"),
                    Self::optional_string(link.state.is_some(), StringSource::LinkState(index)),
                    Self::static_node(b",\"missing\":"),
                    Self::scalar(&(link.state.as_deref() == Some("missing")))?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Dictionary(_) | TypedJsonNode::FragmentStart | TypedJsonNode::FragmentEnd | TypedJsonNode::Static { .. } | TypedJsonNode::Scalar { .. } | TypedJsonNode::String { .. } | TypedJsonNode::OwnedString { .. } => {
                unreachable!("leaf nodes are handled before expansion")
            }
        }
        Ok(())
    }
}
