//! 🔮️ Independent PDF Unicode extraction stays behind a test-only string interface.
pub(super) fn pdf_text(bytes:&[u8])->Result<String,String>{
    let document=lopdf::Document::load_mem(bytes).map_err(|error|error.to_string())?;
    document.extract_text(&document.get_pages().keys().copied().collect::<Vec<_>>()).map_err(|error|error.to_string())
}
