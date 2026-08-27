//! 📝️ Writer mutation text identity registry.

pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] =
    &[("rename-writer", super::rename_writer::text::TEXT_OPCODE), ("change-uri", super::change_uri::text::TEXT_OPCODE), ("change-language", super::change_language::text::TEXT_OPCODE), ("edit-text", super::edit_text::text::TEXT_OPCODE)];
