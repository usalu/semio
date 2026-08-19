//! 🧾 `outline` — one named inference: this document's own outline. Writer documents are plain
//! text with no structured fields, so the outline is derived straight from the `text` field:
//! markdown-style `#`/`##`/… headings become `sectionOutline`, plus real `wordCount`/`lineCount`
//! stats over the whole document.

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Writer` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterOutline {
    pub section_outline: Vec<String>,
    pub word_count: u32,
    pub line_count: u32,
}

impl WriterOutline {
    pub async fn compute(snapshot: &WriterSnapshot) -> Self {
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
mod tests {
    use super::*;

    async fn snapshot_with_text(text: &str) -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "outline-test", "plaintext", "writer://outline-test", text)
    }

    #[test]
    async fn outline_extracts_markdown_headings_in_order() {
        let snapshot = snapshot_with_text("# Title\nsome body text here\n## Section Two\nmore words follow");
        let outline = WriterOutline::compute(&snapshot);
        assert_eq!(outline.section_outline, vec!["Title".to_string(), "Section Two".to_string()]);
    }

    #[test]
    async fn outline_counts_words_and_lines() {
        let snapshot = snapshot_with_text("one two three\nfour five");
        let outline = WriterOutline::compute(&snapshot);
        assert_eq!(outline.word_count, 5);
        assert_eq!(outline.line_count, 2);
    }

    #[test]
    async fn empty_text_produces_an_empty_outline() {
        let outline = WriterOutline::compute(&WriterSnapshot::default());
        assert!(outline.section_outline.is_empty());
        assert_eq!(outline.word_count, 0);
        assert_eq!(outline.line_count, 0);
    }

    #[test]
    async fn outline_is_deterministic() {
        let snapshot = snapshot_with_text("# Only heading\nbody");
        assert_eq!(WriterOutline::compute(&snapshot), WriterOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
