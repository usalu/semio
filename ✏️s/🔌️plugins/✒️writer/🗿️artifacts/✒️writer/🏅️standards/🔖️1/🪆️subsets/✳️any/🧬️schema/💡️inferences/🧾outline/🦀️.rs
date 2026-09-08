//! 🧾 `outline` — one named inference: this document's own outline. Writer documents are plain
//! text with no structured fields, so the outline is derived straight from the `text` field:
//! markdown-style `#`/`##`/… headings become `sectionOutline`, plus real `wordCount`/`lineCount`
//! stats over the whole document.

use crate::{writer_text, WriterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Writer` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::ToValue, dsl::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WriterOutline {
    pub section_outline: Vec<String>,
    pub word_count: u32,
    pub line_count: u32,
}

impl WriterOutline {
    pub fn compute(snapshot: &WriterSnapshot) -> Self {
        let text = writer_text(snapshot);
        let section_outline: Vec<String> = text
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with('#').then(|| trimmed.trim_start_matches('#').trim().to_string())
            })
            .collect();
        let word_count = text.split_whitespace().count() as u32;
        let line_count = if text.is_empty() { 0 } else { text.lines().count() as u32 };
        Self { section_outline, word_count, line_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
