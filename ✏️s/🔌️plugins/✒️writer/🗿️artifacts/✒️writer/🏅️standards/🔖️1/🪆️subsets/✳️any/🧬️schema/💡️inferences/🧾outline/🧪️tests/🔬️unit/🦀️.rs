use super::*;

fn snapshot_with_text(text: &str) -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "outline-test", "plaintext", "writer://outline-test", text)
}

#[semio_framework_async_macros::async_test]
async fn outline_extracts_markdown_headings_in_order() {
    let snapshot = snapshot_with_text("# Title\nsome body text here\n## Section Two\nmore words follow");
    let outline = WriterOutline::compute(&snapshot);
    assert_eq!(outline.section_outline, vec!["Title".to_string(), "Section Two".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn outline_counts_words_and_lines() {
    let snapshot = snapshot_with_text("one two three\nfour five");
    let outline = WriterOutline::compute(&snapshot);
    assert_eq!(outline.word_count, 5);
    assert_eq!(outline.line_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn empty_text_produces_an_empty_outline() {
    let outline = WriterOutline::compute(&WriterSnapshot::default());
    assert!(outline.section_outline.is_empty());
    assert_eq!(outline.word_count, 0);
    assert_eq!(outline.line_count, 0);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = snapshot_with_text("# Only heading\nbody");
    assert_eq!(WriterOutline::compute(&snapshot), WriterOutline::compute(&snapshot));
}
