# Wave-R r2c — architect/program (standards/1/subsets/any) — 4 `CollectionMutation`-referencing triad leaves

Facet: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-architect`

## Live-vs-orphan determination

Evidence gathered by reading the dispatch enum (`🧬️mutations/🦀️component.rs`) and grepping the
package's `📦️glue.rs` mount table before touching anything.

### Dispatch enum evidence (`🧬️mutations/🦀️component.rs:297-300`)

```
ConnectAdjacency(super::set_adjacency::mutation::ConnectAdjacency),
DisconnectAdjacency(super::clear_adjacency::mutation::DisconnectAdjacency),
ConnectTrace(super::traces::mutation::ConnectTrace),
DisconnectTrace(super::traces::mutation::DisconnectTrace),
```

- `ConnectAdjacency`/`DisconnectAdjacency` payloads resolve to `super::set_adjacency::…` /
  `super::clear_adjacency::…` — i.e. the pre-existing `🗺️set-adjacency`/`🧹clear-adjacency`
  directories, **not** `🔀adjacencies`. A repo-wide grep of the dispatch file for `adjacencies`
  turns up zero `super::adjacencies::…` references — only prose comments and snapshot-field
  accesses (`snapshot.adjacencies.len()`, an unrelated `Vec` field on `ProgramSnapshot` itself).
- `ConnectTrace`/`DisconnectTrace` payloads resolve to `super::traces::mutation::…` — i.e. the
  `🧵traces` directory itself. Live.

### `📦️glue.rs` mount evidence (read-only grep, file not touched)

```
519:  pub mod adjacencies {
520:      #[path = ".../🔀adjacencies/🦠️mutation/🦀️component.rs"]
522:      #[path = ".../🔀adjacencies/🔺️diff/🦀️component.rs"]
524:      #[path = ".../🔀adjacencies/↩️inverse/🦀️component.rs"]
753:  pub mod traces {
754:      #[path = ".../🧵traces/🦠️mutation/🦀️component.rs"]
756:      #[path = ".../🧵traces/🔺️diff/🦀️component.rs"]
758:      #[path = ".../🧵traces/↩️inverse/🦀️component.rs"]
```

Both directories are `#[path]`-mounted as `pub mod adjacencies` / `pub mod traces`. Combined with
the dispatch-enum grep: `adjacencies` mod is mounted but never referenced (**orphan**, kept alive
only by the glue mount, matching the wave-2 report's account of the `🔀adjacencies` →
`🗺️set-adjacency`/`🧹clear-adjacency` supersession) — `traces` mod is mounted **and** referenced
(**live**).

## Determination per file

| File | Outcome | Reason |
|---|---|---|
| `🔀adjacencies/🦠️mutation/🦀️component.rs` | (b) orphan stub | no dispatch-enum reference |
| `🔀adjacencies/🔺️diff/🦀️component.rs` | (b) orphan stub | no dispatch-enum reference |
| `🔀adjacencies/↩️inverse/🦀️component.rs` | (b) orphan stub | no dispatch-enum reference |
| `🧵traces/🦠️mutation/🦀️component.rs` | (a) live | `ConnectTrace`/`DisconnectTrace` reachable from dispatch |

## What I found on first read, and what I actually changed

All 4 files were **already structurally in their target end-state** before this wave started —
the wave-2 pass (`📓️wave2-reports/architect-program-1-any-report.md`) had already reduced the
`🔀adjacencies` triad to empty doc-only stubs and already rewrote `🧵traces/🦠️mutation` into real
`ConnectTrace`/`DisconnectTrace` `MutationKind` payload structs delegating to sibling
`🔺️diff`/`↩️inverse` leaves (verified those two sibling files too — `diff_connect`/`diff_disconnect`
build `ProgramDiff` sparsely from `(payload, base)`, `inverse_connect`/`inverse_disconnect`
reconstruct from `base` only and return `Vec::new()` when absent — exactly the target pattern, no
`protocol::CollectionMutation` anywhere in the actual code).

The only remaining defect, in all 4 files, was that the **doc comments** (not the code) still
contained the literal string `CollectionMutation` while narrating what each leaf used to be /
supersede (e.g. `` `Adjacencies(CollectionMutation<EntityId, Adjacency, AdjacencyPatch>)` `` and
`` `Traces(CollectionMutation<EntityId, TraceLink, TraceLinkPatch>)` ``). Per
`📓️remaining-work-map.md`'s policy-state section, the vocabulary rule regex-tests raw file content
**including comments**, so these 4 files were exactly the "doc-comment mentions in retired stubs"
category it calls out. This is also why `remaining-work-map.md`'s census (compiled after the
wave-2 pass) still listed all 4 as "CollectionMutation debt" despite the underlying code already
being clean.

Fix applied to all 4: reworded the doc-comment line(s) that quoted the old generic-wrapper type
signature to describe it in prose ("the old generic per-collection add/remove/patch wrapper")
instead of spelling out the banned type name, and (for the 3 `🔀adjacencies` files) also removed an
incidental mention of `` `🖼️set-snapshot` ``/`` `🫙no-mutation` `` sibling-orphan slugs that added no
information toward the banned-token check but were replaced with a plainer phrase for clarity. No
code, imports, or logic changed in any of the 4 files — there were no `use` statements to drop (the
3 orphan stubs are comment-only files; `🧵traces/🦠️mutation` already imports only what it uses).
Docstrings already led with a fitting emoji (🪦 for the orphan stubs, 🦠️ for the live mutation leaf)
and needed no change there.

Post-edit verification: `grep -nE "CollectionMutation|SetSnapshot|NoMutation"` against all 4 files
returns zero matches (confirmed via exit code 1 on each).

## Gate: `cargo check -p semio-s-plugin-architect`

**Crate is unbuildable for reasons that are NOT mine.** Captured full error lists before my first
edit and after my last edit; both runs produced **254 errors**, and a `diff` of the two
`--message-format=short` transcripts shows only non-deterministic build-log ordering (one
`Checking semio-s-plugin-stdio …` progress line present in one run and not the other; two error
lines swapped order due to parallel-codegen scheduling) — the *set* of 254 error messages is
identical byte-for-byte between runs. My edits added zero errors and fixed zero errors, as
expected for comment-only changes.

Root cause (confirmed by direct grep, matching the wave-2 report's independent diagnosis):
`✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs:938` still reads

```rust
pub mod registers { pub use crate::artifacts::program::standards::v1::subsets::any::io::registers::*; }
```

(should be `...schema::registers::*`, matching every sibling alias on adjacent lines). This is on
the ticket's explicit DO-NOT-TOUCH list (`📦️glue.rs`, owned by a later wave) and is the same
external typo the wave-2 architect report already diagnosed as introduced by a concurrent session.
It alone accounts for the ~132 register-resolution errors plus knock-on `🎛️apps/🏛️architect/**`
failures; a separate, also-foreign, single error in `💡️inferences/🦀️component.rs`
(`ProgramInference::infer` not in scope) belongs to the concurrent
`INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` ticket per the wave-2 report.
Neither was touched.

Before/after error counts:

| | count |
|---|---|
| before my edits | 254 |
| after my edits | 254 |
| diff | 0 (identical error set; only log-line ordering differs) |

Full transcripts kept in the ticket scratch folder: `scratch-r2c-before-cargo.txt`,
`scratch-r2c-after-cargo.txt`.

## Wave-C carryovers: directories to delete when glue is rewired

- `🔀adjacencies/` (all 3 leaves: `🦠️mutation`, `🔺️diff`, `↩️inverse`) — orphan stub, superseded by
  `🗺️set-adjacency`/`🧹clear-adjacency`; delete once `📦️glue.rs`'s `pub mod adjacencies { … }`
  mount (lines ~519-525) is removed. This item was already flagged by the wave-2 report; restating
  it here as this wave's own carryover for completeness.

(`🧵traces` is NOT a carryover — it is live and stays.)

## `allowlistKeysToRemove`

Repo-relative paths now free of `CollectionMutation`/`SetSnapshot`/`NoMutation` (verified by grep,
exit code 1 on each):

```
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀adjacencies/🦠️mutation/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀adjacencies/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀adjacencies/↩️inverse/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵traces/🦠️mutation/🦀️component.rs
```
