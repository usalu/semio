//! 🚪️ IO s.en1991 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `En1991Snapshot::default()` on import (an honesty bug, not a
//! real codec). En1991Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.en1991` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] { &[] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &[] }
