//! 📄️ Document and page nodes: the document root, the top level arrays, the styles, stories, links, parents, spreads, pages and string arrays of the layout export JSON expansion. See [`super`].

use super::*;

impl TypedJsonCursor {
    /// 🧾 Expands the `Document` node into its output fragments.
    pub(super) fn expand_document(&mut self, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🧮 Expands the `TopArray` node into its output fragments.
    pub(super) fn expand_top_array(&mut self, kind: TopArrayKind, index: usize, opened: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🗂 Expands the `Paragraph` node into its output fragments.
    pub(super) fn expand_paragraph(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🗃 Expands the `Character` node into its output fragments.
    pub(super) fn expand_character(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🗄 Expands the `StoryRun` node into its output fragments.
    pub(super) fn expand_story_run(&mut self, story: usize, run: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🗒 Expands the `Link` node into its output fragments.
    pub(super) fn expand_link(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 🗓 Expands the `Parent` node into its output fragments.
    pub(super) fn expand_parent(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }

    /// 📊 Expands the `Page` node into its output fragments.
    pub(super) fn expand_page(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let value = snapshot.pages.get(index).ok_or("layout-export-json-index")?;
        let mut nodes=vec![
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
        ];
        nodes.pop();
        if value.background_drawing.is_some(){nodes.extend([Self::static_node(b",\"backgroundDrawing\":"),TypedJsonNode::Child(ChildJsonCursor::new(semio_s_artifact_layout_layout_model::DrawingSourceAddress::Page(index)))]);}
        nodes.push(Self::static_node(b"}"));self.push_sequence(nodes);
        Ok(())
    }

    /// 📈 Expands the `StringArray` node into its output fragments.
    pub(super) fn expand_string_array(&mut self, source: StringArraySource, index: usize, opened: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
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
        Ok(())
    }
}
