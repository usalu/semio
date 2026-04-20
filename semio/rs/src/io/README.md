# Persistence (JSON / SQLite / ZIP)

Inherent methods on `Kit` (`from_json_str`, `to_json_pretty`, SQLite import/export, ZIP workflows) currently live in `src/lib.rs` (`mod sqlite_import_export`, `mod zip_import_export`, `mod kit_workflow`). They are candidates for relocation into this directory as `json.rs`, `sqlite.rs`, `zip.rs` without changing public signatures.
