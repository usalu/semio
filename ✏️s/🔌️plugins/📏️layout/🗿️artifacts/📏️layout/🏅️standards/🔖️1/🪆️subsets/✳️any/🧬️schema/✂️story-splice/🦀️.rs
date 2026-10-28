//! ✂️ Unicode scalar splices preserve authored UTF-8 style intervals and overlapping run order.

use crate::{TextStory, TextStyleRun};

/// 📍️ Resolves a scalar-character splice into UTF-8 byte boundaries.
pub fn byte_range(content: &str, offset: usize, delete: usize) -> Option<(usize, usize)> {
    let end = offset.checked_add(delete)?;
    let length = content.chars().count();
    if end > length { return None; }
    Some((content.char_indices().nth(offset).map_or(content.len(), |(at, _)| at), content.char_indices().nth(end).map_or(content.len(), |(at, _)| at)))
}

/// 🧵️ Retains each run's surviving pieces and right-biased insertion style, with left bias at EOF.
pub fn splice(story: &TextStory, offset: usize, delete: usize, insert: &str) -> Option<TextStory> {
    let (start, end) = byte_range(&story.content, offset, delete)?;
    let start = u64::try_from(start).ok()?;
    let end = u64::try_from(end).ok()?;
    let inserted = u64::try_from(insert.len()).ok()?;
    let insertion_end = start.checked_add(inserted)?;
    let old_length = u64::try_from(story.content.len()).ok()?;
    old_length.checked_sub(end - start)?.checked_add(inserted)?;
    let shift = |at: u64| at.checked_sub(end - start)?.checked_add(inserted);
    let mut style_runs: Vec<TextStyleRun> = Vec::with_capacity(story.style_runs.len());
    for run in &story.style_runs {
        let mut next = run.clone();
        if run.start == run.end {
            let at = if run.start < start { run.start } else if run.start >= end { shift(run.start)? } else { insertion_end };
            next.start = at;
            next.end = at;
            style_runs.push(next);
            continue;
        }
        let prefix = (run.start < start).then_some((run.start, run.end.min(start)));
        let inherits = run.start <= start && start < run.end || start == old_length && run.start < start && run.end == start;
        let middle = (inherits && inserted > 0).then_some((start, insertion_end));
        let suffix = if run.end > end { Some((shift(run.start.max(end))?, shift(run.end)?)) } else { None };
        let mut pieces = [prefix, middle, suffix].into_iter().flatten().filter(|(a, b)| a < b);
        if let Some((a, b)) = pieces.next() {
            next.start = a;
            next.end = pieces.last().map_or(b, |(_, end)| end);
            style_runs.push(next);
        }
    }
    let (start, end) = (usize::try_from(start).ok()?, usize::try_from(end).ok()?);
    Some(TextStory { id: story.id.clone(), content: format!("{}{}{}", &story.content[..start], insert, &story.content[end..]), style_runs })
}
