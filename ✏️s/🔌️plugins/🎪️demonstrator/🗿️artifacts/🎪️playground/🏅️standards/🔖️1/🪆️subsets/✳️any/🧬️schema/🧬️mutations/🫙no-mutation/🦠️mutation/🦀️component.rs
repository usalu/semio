//! 🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — `NoMutation` is banned outright (a
//! mutation with nothing to undo returns `Vec::new()` from `MutationKind::inverse`, no sentinel
//! variant needed, see `📓️taxonomy.md`). This file stays present only because `📦️glue.rs`
//! (plugin-shared, outside this facet's boundary) still `#[path]`-wires it; see this ticket's
//! wave2 report `sharedFileRequests` for the glue.rs cleanup this orphaning needs (delete the
//! `no_mutation` module block entirely).
