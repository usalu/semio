//! 📖 Retired. `stories`'s generic `Stories(CollectionMutation<String, TextStory, TextStoryPatch>)`
//! variant was replaced by the semantic `create-story`/`delete-story`/`edit-story` triads — see
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`. This directory has
//! no active payload structs and no `Stories` variant remains in `LayoutMutation`.
//!
//! The three empty leaf files under this directory (this one, `🔺️diff`, `↩️inverse`) are kept only
//! because `📦️glue.rs` — outside this facet's package boundary — still `#[path]`-wires them;
//! deleting the directory requires editing that shared file. Tracked as a `sharedFileRequests` item
//! in `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️wave2-reports/layout-layout-1-any-report.md`.
