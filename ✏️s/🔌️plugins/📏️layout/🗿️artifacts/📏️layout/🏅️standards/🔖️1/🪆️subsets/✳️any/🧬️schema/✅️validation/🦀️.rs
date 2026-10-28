//! ✅️ Canonical Layout graph invariants shared by semantic publication paths.

use std::collections::{HashMap, HashSet};
use crate::{snapshot::LayoutSnapshot, Frame, Layer, LayoutBounds, LayoutRect, Page, ParentPage, TextStory, TextStyleRun};

type Validation = Result<(), &'static str>;

fn require(condition: bool, message: &'static str) -> Validation {
    if condition { Ok(()) } else { Err(message) }
}

fn unique_ids<'a>(ids: impl IntoIterator<Item = &'a str>) -> Validation {
    let mut seen = HashSet::new();
    require(ids.into_iter().all(|id| !id.trim().is_empty() && seen.insert(id)), "Ids must be non-empty and unique in their collection.")
}

fn positive(value: f64) -> bool { value.is_finite() && value > 0.0 }
fn nonnegative(value: f64) -> bool { value.is_finite() && value >= 0.0 }
fn bounds_valid(bounds: &LayoutBounds, allow_zero: bool) -> bool {
    [bounds.x, bounds.y, bounds.rotation].iter().all(|value| value.is_finite()) && if allow_zero { nonnegative(bounds.width) && nonnegative(bounds.height) } else { positive(bounds.width) && positive(bounds.height) }
}
fn rect_valid(rect: &LayoutRect) -> bool {
    rect.x.is_finite() && rect.y.is_finite() && nonnegative(rect.width) && nonnegative(rect.height)
}
fn color_valid(color: &Option<[f32; 4]>) -> bool {
    color.as_ref().is_none_or(|channels| channels.iter().all(|channel| channel.is_finite()))
}

fn validate_content(document: &LayoutSnapshot, layers: &[Layer], frames: &[Frame]) -> Validation {
    unique_ids(layers.iter().map(|layer| layer.id.as_str()))?;
    unique_ids(frames.iter().map(Frame::id))?;
    let layer_ids: HashSet<_> = layers.iter().map(|layer| layer.id.as_str()).collect();
    let stories: HashSet<_> = document.stories.iter().map(|story| story.id.as_str()).collect();
    let links: HashSet<_> = document.links.iter().map(|link| link.id.as_str()).collect();
    for frame in frames {
        require(layer_ids.contains(frame.layer_id()), "Every frame must belong to an existing owning layer.")?;
        validate_frame_bounds(frame.bounds())?;
        match frame {
            Frame::Rect { fill, stroke, .. } => require(color_valid(fill) && color_valid(stroke), "Frame colors must be finite.")?,
            Frame::Text { story_id, columns, inset, .. } => {
                require(stories.contains(story_id.as_str()), "Text frames must name an existing story.")?;
                require(*columns > 0 && rect_valid(inset), "Text columns must be positive and inset sizes finite and non-negative.")?;
            }
            Frame::Image { link_id, .. } => require(links.contains(link_id.as_str()), "Image frames must name an existing link.")?,
        }
    }
    Ok(())
}

fn validate_threads<'a>(frames: impl IntoIterator<Item = &'a Frame>) -> Validation {
    let frames: HashMap<_, _> = frames.into_iter().map(|frame| (frame.id(), frame)).collect();
    let mut predecessors = HashSet::new();
    for frame in frames.values() {
        if let Frame::Text { story_id, thread_next: Some(next), .. } = frame {
            let Some(Frame::Text { story_id: target_story, .. }) = frames.get(next.as_str()) else { return Err("Thread targets must be existing text frames in the same content graph.") };
            require(story_id == target_story, "Thread edges must connect frames of the same story.")?;
            require(predecessors.insert(next.as_str()), "Text frames must have at most one predecessor.")?;
        }
    }
    let mut complete = HashSet::new();
    for id in frames.keys().copied() {
        let mut active = HashSet::new();
        let mut cursor = Some(id);
        while let Some(current) = cursor {
            if complete.contains(current) { break; }
            require(active.insert(current), "Text threads must not contain cycles.")?;
            cursor = match frames.get(current) {
                Some(Frame::Text { thread_next: Some(next), .. }) => Some(next.as_str()),
                _ => None,
            };
        }
        complete.extend(active);
    }
    Ok(())
}

fn validate_membership(layer_ids: &[String], layers: &[Layer], frames: &[Frame]) -> Validation {
    require(layer_ids.iter().map(String::as_str).eq(layers.iter().map(|layer| layer.id.as_str())), "Layer membership must match the owning layer order.")?;
    for layer in layers {
        require(layer.object_ids.iter().map(String::as_str).eq(frames.iter().filter(|frame| frame.layer_id() == layer.id).map(Frame::id)), "Layer object membership must match its frames in paint order.")?;
    }
    Ok(())
}


/// 🆔️ Checks admission of an authored collection id.
pub fn validate_id(id: &str) -> Validation {
    require(!id.trim().is_empty(), "Ids must be non-empty.")
}

/// ✒️ Checks UTF-8 run boundaries and style references against the authored story.
pub fn validate_style_runs(document: &LayoutSnapshot, content: &str, runs: &[TextStyleRun]) -> Validation {
    let paragraph_ids: HashSet<_> = document.paragraph_styles.iter().map(|style| style.id.as_str()).collect();
    let character_ids: HashSet<_> = document.character_styles.iter().map(|style| style.id.as_str()).collect();
    validate_runs(content, runs, &paragraph_ids, &character_ids)
}

fn validate_runs(content: &str, runs: &[TextStyleRun], paragraph_ids: &HashSet<&str>, character_ids: &HashSet<&str>) -> Validation {
    for run in runs {
        require(run.start <= run.end && usize::try_from(run.start).is_ok_and(|at| content.is_char_boundary(at)) && usize::try_from(run.end).is_ok_and(|at| content.is_char_boundary(at)), "Style runs must stay inside the story on UTF-8 character boundaries.")?;
        require(run.paragraph_style_id.as_deref().is_none_or(|id| paragraph_ids.contains(id)) && run.character_style_id.as_deref().is_none_or(|id| character_ids.contains(id)), "Style runs must name existing styles.")?;
    }
    Ok(())
}

/// 📖️ Validates a candidate story without revisiting unrelated graph content.
pub fn validate_story(document: &LayoutSnapshot, story: &TextStory) -> Validation {
    validate_id(&story.id)?;
    validate_style_runs(document, &story.content, &story.style_runs)
}

/// 🔗️ Admits the proposed story and outgoing edge while preserving incoming story identity and unique flow.
pub fn validate_thread_edge(document: &LayoutSnapshot, source: &str, story: &str, next: Option<&str>) -> Validation {
    require(document.stories.iter().any(|held| held.id == story), "Text frames must name an existing story.")?;
    let frames: HashMap<_, _> = document.pages.iter().flat_map(|page| page.frames.iter()).map(|frame| (frame.id(), frame)).collect();
    let mut source_predecessors = 0;
    for frame in frames.values() {
        if let Frame::Text { story_id, thread_next: Some(target), .. } = frame {
            if frame.id() != source && target == source {
                require(story_id == story, "Thread edges must connect frames of the same story.")?;
                source_predecessors += 1;
                require(source_predecessors <= 1, "Text frames must have at most one predecessor.")?;
            }
            require(frame.id() == source || next.is_none_or(|next| target != next), "Text frames must have at most one predecessor.")?;
        }
    }
    let mut active = HashSet::new();
    let mut cursor = next;
    while let Some(id) = cursor {
        require(id != source && active.insert(id), "Text threads must not contain cycles.")?;
        let Some(Frame::Text { story_id, thread_next, .. }) = frames.get(id) else { return Err("Thread targets must be existing text frames in the same content graph.") };
        require(story_id == story, "Thread edges must connect frames of the same story.")?;
        cursor = thread_next.as_deref();
    }
    Ok(())
}

/// ➖️ Refuses removing targets still referenced by surviving local frames.
pub fn validate_frame_removal<'a>(document: &LayoutSnapshot, ids: impl IntoIterator<Item = &'a str>) -> Validation {
    let ids: HashSet<_> = ids.into_iter().collect();
    require(!document.pages.iter().flat_map(|page| page.frames.iter()).any(|frame| match frame {
        Frame::Text { id, thread_next: Some(next), .. } => !ids.contains(id.as_str()) && ids.contains(next.as_str()),
        _ => false,
    }), "A surviving text frame still points to a removed frame.")
}

/// ➕️ Checks a single frame admission against its owning page and the local thread graph.
pub fn validate_frame_creation(document: &LayoutSnapshot, page: &Page, frame: &Frame) -> Validation {
    validate_id(frame.id())?;
    require(!document.pages.iter().flat_map(|page| page.frames.iter()).any(|held| held.id() == frame.id()), "Local frame ids must be globally unique.")?;
    require(!page.parent_page_id.as_deref().and_then(|id| document.parent_pages.iter().find(|parent| parent.id == id)).is_some_and(|parent| parent.frames.iter().any(|held| held.id() == frame.id())), "Inherited and local frame ids must not collide on a page.")?;
    validate_content(document, &page.layers, std::slice::from_ref(frame))?;
    if let Frame::Text { story_id, thread_next, .. } = frame { validate_thread_edge(document, frame.id(), story_id, thread_next.as_deref())?; }
    Ok(())
}

fn validate_page_content(document: &LayoutSnapshot, page: &Page) -> Validation {
    if let Some(child)=&page.background_drawing{validate_page_drawing_child(child)?;}
    require(positive(page.width) && positive(page.height), "Pages need positive finite dimensions.")?;
    require([page.margins.top, page.margins.right, page.margins.bottom, page.margins.left].iter().all(|value| value.is_finite()) && page.columns.count > 0 && nonnegative(page.columns.gutter) && page.guides.iter().all(rect_valid), "Page margins must be finite, columns positive, and gutter and guide sizes finite and non-negative.")?;
    validate_content(document, &page.layers, &page.frames)?;
    validate_membership(&page.layer_ids, &page.layers, &page.frames)?;
    validate_parent_assignment(document, page, page.parent_page_id.as_deref())?;
    Ok(())
}


/// 📐️ Checks authored geometry shared by frame admission and transformed output publication.
pub fn validate_frame_bounds(bounds: &LayoutBounds) -> Validation {
    require(bounds_valid(bounds, false), "Frame bounds must be finite and have a positive size.")
}

/// 🗄️ Checks only the inherited identity and override consequences of assigning a parent.
pub fn validate_parent_assignment(document: &LayoutSnapshot, page: &Page, parent_id: Option<&str>) -> Validation {
    let parent = match parent_id {
        Some(id) => Some(document.parent_pages.iter().find(|parent| parent.id == id).ok_or("Page parents must exist.")?),
        None => None,
    };
    unique_ids(page.overrides.iter().map(|entry| entry.object_id.as_str()))?;
    for entry in &page.overrides {
        require(parent.is_some_and(|parent| parent.frames.iter().any(|frame| frame.id() == entry.object_id)) && entry.bounds.as_ref().is_none_or(|bounds| bounds_valid(bounds, true)), "Overrides must name inherited frames and have finite non-negative bounds.")?;
    }
    if let Some(parent) = parent {
        let local_ids: HashSet<_> = page.frames.iter().map(Frame::id).collect();
        require(parent.frames.iter().all(|frame| !local_ids.contains(frame.id())), "Inherited and local frame ids must not collide on a page.")?;
    }
    Ok(())
}

/// 🌱️ Validates a new page and its candidate thread graph before atomic spread admission.
pub fn validate_page_creation(document: &LayoutSnapshot, page: &Page) -> Validation {
    validate_id(&page.id)?;
    require(document.spreads.iter().any(|spread| spread.id == page.spread_id), "The owning spread must exist.")?;
    validate_page_content(document, page)?;
    if let Some(child)=&page.background_drawing{validate_page_drawing_identity(document,&page.id,child)?;}
    unique_ids(document.pages.iter().flat_map(|held| held.frames.iter()).chain(page.frames.iter()).map(Frame::id))?;
    validate_threads(document.pages.iter().flat_map(|held| held.frames.iter()).chain(page.frames.iter()))
}

/// 🧩️ Validates parent-owned content and derives its redundant memberships for both creation and replacement.
pub fn normalize_parent_content(document: &LayoutSnapshot, page: &mut ParentPage) -> Validation {
    require(!page.id.trim().is_empty() && !page.name.trim().is_empty() && positive(page.width) && positive(page.height), "Parent pages need an id, a name, and positive finite dimensions.")?;
    validate_content(document, &page.layers, &page.frames)?;
    validate_threads(page.frames.iter())?;
    for held in document.pages.iter().filter(|held| held.parent_page_id.as_deref() == Some(page.id.as_str())) {
        let ids: HashSet<_> = held.frames.iter().map(Frame::id).collect();
        require(page.frames.iter().all(|frame| !ids.contains(frame.id())), "Inherited and local frame ids must not collide on a page.")?;
    }
    page.layer_ids = page.layers.iter().map(|layer| layer.id.clone()).collect();
    for layer in &mut page.layers {
        layer.object_ids = page.frames.iter().filter(|frame| frame.layer_id() == layer.id).map(|frame| frame.id().to_string()).collect();
    }
    Ok(())
}

/// 🌐️ Validates a complete typed Layout graph before whole-document publication without changing its payload.
pub fn validate_document(document: &LayoutSnapshot) -> Validation {
    unique_ids(drawing_children(document).map(|child|child.handle.child_id.as_str()))?;
    unique_ids(drawing_children(document).map(|child|child.handle.target.artifact_id.as_str()))?;
    if let Some(child)=&document.background_drawing{validate_drawing_child(child)?;}
    require(document.schema == crate::LAYOUT_DOCUMENT_SCHEMA, "The document schema is unsupported.")?;
    require(positive(document.grid.baseline_grid) && document.grid.baseline_offset.is_finite(), "The baseline grid must be positive and finite and its offset finite.")?;
    unique_ids(document.paragraph_styles.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.character_styles.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.stories.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.links.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.parent_pages.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.spreads.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.pages.iter().map(|row| row.id.as_str()))?;
    unique_ids(document.pages.iter().flat_map(|page| page.frames.iter()).map(Frame::id))?;
    for style in &document.paragraph_styles {
        require(!style.name.trim().is_empty() && !style.font_family.trim().is_empty() && positive(style.font_size) && positive(style.leading) && style.tracking.is_finite() && style.font_weight > 0 && matches!(style.alignment.as_str(), "left" | "center" | "right" | "justify"), "Paragraph typography must have valid names, sizes, weight, tracking, and alignment.")?;
    }
    for style in &document.character_styles {
        require(style.font_size.is_none_or(positive) && style.font_weight.is_none_or(|weight| weight > 0) && style.tracking.is_none_or(f64::is_finite) && color_valid(&style.color), "Character typography must have positive sizes and weight and finite tracking and colors.")?;
    }
    let paragraph_ids: HashSet<_> = document.paragraph_styles.iter().map(|style| style.id.as_str()).collect();
    let character_ids: HashSet<_> = document.character_styles.iter().map(|style| style.id.as_str()).collect();
    for story in &document.stories { validate_runs(&story.content, &story.style_runs, &paragraph_ids, &character_ids)?; }
    for parent in &document.parent_pages {
        require(!parent.name.trim().is_empty() && positive(parent.width) && positive(parent.height), "Parent pages need a name and positive finite dimensions.")?;
        validate_content(document, &parent.layers, &parent.frames)?;
        validate_threads(parent.frames.iter())?;
        validate_membership(&parent.layer_ids, &parent.layers, &parent.frames)?;
    }
    let spreads: HashMap<_, _> = document.spreads.iter().map(|spread| (spread.id.as_str(), spread)).collect();
    let pages: HashMap<_, _> = document.pages.iter().map(|page| (page.id.as_str(), page)).collect();
    for spread in &document.spreads {
        require(!spread.name.trim().is_empty(), "Spreads need a name.")?;
        unique_ids(spread.page_ids.iter().map(String::as_str))?;
        require(spread.page_ids.iter().all(|id| pages.get(id.as_str()).is_some_and(|page| page.spread_id == spread.id)), "Spread members must exist and belong to that spread.")?;
    }
    for page in &document.pages {
        require(spreads.get(page.spread_id.as_str()).is_some_and(|spread| spread.page_ids.iter().any(|id| id == &page.id)), "Every page must be a member of its existing spread.")?;
        validate_page_content(document, page)?;
    }
    validate_threads(document.pages.iter().flat_map(|page| page.frames.iter()))
}

/// 🧩️ Admits new Drawing content locally through its canonical composed owner.
pub fn validate_drawing_child(child:&crate::LayoutDrawingMember)->Validation{let target=&child.handle.target;require(!child.handle.child_id.trim().is_empty()&&!target.artifact_id.trim().is_empty()&&target.dialect.artifact_kind=="s.draw.drawing"&&target.dialect.standard=="1"&&target.dialect.subset=="*","Drawing member references must have independent identities and the genuine Draw dialect.")}

/// 🖼️ Page content preserves a finite positive authored Drawing viewport.
pub fn validate_page_drawing_child(child:&crate::LayoutDrawingMember)->Validation{validate_drawing_child(child)}

fn drawing_children(document:&LayoutSnapshot)->impl Iterator<Item=&crate::LayoutDrawingMember>{document.background_drawing.iter().chain(document.pages.iter().filter_map(|page|page.background_drawing.as_ref()))}
fn validate_page_drawing_identity(document:&LayoutSnapshot,page_id:&str,child:&crate::LayoutDrawingMember)->Validation{let aliases=|held:&crate::LayoutDrawingMember|held.handle.child_id==child.handle.child_id||held.handle.target.artifact_id==child.handle.target.artifact_id;require(document.background_drawing.as_ref().is_none_or(|held|!aliases(held))&&document.pages.iter().filter(|page|page.id!=page_id).all(|page|page.background_drawing.as_ref().is_none_or(|held|!aliases(held))),"Page Drawing children must have independent local and target identities.")}
/// 🪪️ Checks only the new page child and its identity among the existing owned child slots.
pub fn validate_page_drawing_assignment(document:&LayoutSnapshot,page_id:&str,child:&crate::LayoutDrawingMember)->Validation{validate_page_drawing_child(child)?;validate_page_drawing_identity(document,page_id,child)}

#[path="🧳️retained/🦀️.rs"]
pub mod retained;

#[path="👁️graph/🦀️.rs"]pub mod graph;
