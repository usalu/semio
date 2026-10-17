//! 🧱️ The authored SQLite DDL of the Architect snapshot, embedded once in a module of its own so the codec that reads it carries none of its bytes.

/// 🗄️ Authored relational schema (`CREATE TABLE` statements) the Architect SQLite snapshot codec projects into and restores from.
pub const SCHEMA: &str = include_str!("../🗄️.sql");
