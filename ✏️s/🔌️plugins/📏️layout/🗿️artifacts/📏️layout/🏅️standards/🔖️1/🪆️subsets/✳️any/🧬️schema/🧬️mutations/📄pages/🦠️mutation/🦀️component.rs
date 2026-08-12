//! 📄 Retired. `pages`'s generic `Pages(CollectionMutation<String, Page, PagePatch>)` variant was
//! replaced by the semantic `create-page`/`delete-page`/`rename-page`/`change-page-width`/
//! `change-page-height`/`update-page-margins`/`update-page-columns`/`reorder-pages` triads — see
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`. This directory has
//! no active payload structs and no `Pages` variant remains in `LayoutMutation`.
//!
//! The three empty leaf files under this directory (this one, `🔺️diff`, `↩️inverse`) are kept only
//! because `📦️glue.rs` — outside this facet's package boundary — still `#[path]`-wires them;
//! deleting the directory requires editing that shared file. Tracked as a `sharedFileRequests` item
//! in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️wave2-reports/layout-layout-1-any-report.md`.
