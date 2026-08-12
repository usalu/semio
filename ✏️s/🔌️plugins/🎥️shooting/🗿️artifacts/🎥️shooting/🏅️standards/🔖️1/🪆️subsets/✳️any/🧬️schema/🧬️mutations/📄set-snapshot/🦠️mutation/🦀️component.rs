//! 📄 Retired. `set-snapshot` (whole-document replacement) is BANNED per the semantic mutations
//! taxonomy (`📓️taxonomy.md`, "Forbidden vocabulary") — it has no replacement mutation; whole-doc
//! load/import/paste-over goes through `ArtifactStore::reset`, entirely outside the `Mutation`
//! enum. This directory has no active payload structs and no `📄set-snapshot` variant remains in
//! `ShootingMutation`.
//!
//! The three empty leaf files under this directory (this one, `🔺️diff`, `↩️inverse`) are kept only
//! because `📦️glue.rs` — outside this facet's package boundary — still `#[path]`-wires them;
//! deleting the directory requires editing that shared file. Tracked as a `sharedFileRequests` item
//! in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️wave2-reports/shooting-shooting-1-any-report.md`.
