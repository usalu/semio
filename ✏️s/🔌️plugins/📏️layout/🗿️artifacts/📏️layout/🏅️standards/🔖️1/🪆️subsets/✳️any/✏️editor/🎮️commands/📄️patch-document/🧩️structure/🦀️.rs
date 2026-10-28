//! 🧩️ Inspector structure commands emit native layout mutations.

use super::*;
use semio_s_artifact_layout_layout_model::{Frame, ParagraphStyle, ParentPage, Spread, TextStory};
use semio_s_artifact_layout_layout_mutations_content::{create_story::CreateStory, delete_story::DeleteStory, edit_story::EditStory};
use semio_s_artifact_layout_layout_mutations_content::{create_paragraph_style::CreateParagraphStyle, delete_paragraph_style::DeleteParagraphStyle, create_parent_page::CreateParentPage, delete_parent_page::DeleteParentPage, create_spread::CreateSpread, delete_spread::DeleteSpread, set_parent_page_content::SetParentPageContent};

fn identity(prefix: &str, exists: impl Fn(&str) -> bool) -> String {
    let mut index = 1;
    loop {
        let id = format!("{prefix}-{index}");
        if !exists(&id) { return id }
        index += 1;
    }
}

pub(super) fn mutation(document: &LayoutSnapshot, field: &str, value: &str) -> Option<LayoutMutation> {
    if let Some(rest) = field.strip_prefix("story.") {
        let (id, key) = rest.rsplit_once('.')?;
        let story = document.stories.iter().find(|story| story.id == id)?;
        return match key {
            "content" => crate::patch_frame::story_splice(&story.content, value).map(|(offset, delete, insert)| LayoutMutation::EditStory(EditStory { id: id.into(), offset, delete, insert })),
            "delete" if !document.pages.iter().flat_map(|page| &page.frames).chain(document.parent_pages.iter().flat_map(|page| &page.frames)).any(|frame| matches!(frame, Frame::Text { story_id, .. } if story_id == id)) => Some(LayoutMutation::DeleteStory(DeleteStory { id: id.into() })),
            _ => None,
        };
    }
    match field {
        "addStory" => {
            let id = identity("story", |id| document.stories.iter().any(|row| row.id == id));
            return Some(LayoutMutation::CreateStory(CreateStory { story: TextStory { id, content: String::new(), style_runs: Vec::new() }, index: None }));
        }
        "addParagraphStyle" => {
            let id = identity("paragraph", |id| document.paragraph_styles.iter().any(|row| row.id == id));
            let style = ParagraphStyle { id, name: value.into(), font_family: "sans-serif".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() };
            return Some(LayoutMutation::CreateParagraphStyle(CreateParagraphStyle { style, index: None }));
        }
        "addParentPage" => {
            let id = identity("parent", |id| document.parent_pages.iter().any(|row| row.id == id));
            let source = document.pages.first();
            let page = ParentPage { id, name: value.into(), width: source.map_or(595.0, |page| page.width), height: source.map_or(842.0, |page| page.height), layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() };
            return Some(LayoutMutation::CreateParentPage(CreateParentPage { page, index: None }));
        }
        "addSpread" => {
            let id = identity("spread", |id| document.spreads.iter().any(|row| row.id == id));
            return Some(LayoutMutation::CreateSpread(CreateSpread { spread: Spread { id, name: value.into(), page_ids: Vec::new() }, index: None }));
        }
        _ => {}
    }
    let (id, key) = field.rsplit_once('.')?;
    match key {
        "delete" if document.paragraph_styles.iter().any(|style| style.id == id) => Some(LayoutMutation::DeleteParagraphStyle(DeleteParagraphStyle { id: id.into() })),
        "delete" if document.parent_pages.iter().any(|page| page.id == id) => Some(LayoutMutation::DeleteParentPage(DeleteParentPage { id: id.into() })),
        "delete" if document.spreads.iter().any(|spread| spread.id == id) => Some(LayoutMutation::DeleteSpread(DeleteSpread { id: id.into() })),
        "capture" if document.parent_pages.iter().any(|parent| parent.id == id) => {
            let page = document.pages.iter().find(|page| page.id == value)?;
            Some(LayoutMutation::SetParentPageContent(SetParentPageContent { id: id.into(), layers: page.layers.clone(), frames: page.frames.clone() }))
        }
        _ => None,
    }
}

/// 🧵️ Story span commits carry the exact content fingerprint of the rendered draft.
pub(super) fn guarded_text(document: &LayoutSnapshot, field: &str, value: &str) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let missing = || super::text_chunks::stale();
    let rest = field.strip_prefix("story.").ok_or_else(missing)?;
    let (id, span) = rest.rsplit_once(".content.chunk.").ok_or_else(missing)?;
    let story = document.stories.iter().find(|story| story.id == id).ok_or_else(missing)?;
    let stamp = super::text_chunks::fingerprint(story.content.as_bytes());
    let edit = super::text_chunks::splice(&story.content, &format!("chunk.{span}"), value, &stamp)?;
    Ok(Emit::mutations(edit.map(|(offset, delete, insert)| LayoutMutation::EditStory(EditStory { id: id.into(), offset, delete, insert })).into_iter().collect()))
}
