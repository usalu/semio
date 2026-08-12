//! 🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — `LowpolyMutation::SetSnapshot` was replaced
//! by nothing — whole-document replace is banned outright, see taxonomy.md; `store::ArtifactStore::reset` is the sanctioned non-history path (see `📓️taxonomy.md`/`📓️derivation-rules.md`). This file stays present only
//! because `📦️glue.rs` (plugin-shared, outside this facet's boundary) still `#[path]`-wires it;
//! see this ticket's wave2 report `sharedFileRequests` for the glue.rs cleanup this orphaning
//! needs (delete this directory's `pub mod` block entirely).
