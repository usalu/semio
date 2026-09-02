//! 💾️ Writer mutation binary identity registry.

pub const BINARY_TAG_REGISTRY: &[(&str, u8)] =
    &[("rename-writer", super::rename_writer::binary::BINARY_TAG), ("change-uri", super::change_uri::binary::BINARY_TAG), ("change-language", super::change_language::binary::BINARY_TAG), ("edit-text", super::edit_text::binary::BINARY_TAG)];
