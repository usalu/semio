# W6 — `semio-s-plugin-architect` Hover/Selection Migration

Crate was missed by the original W4 inventory (it has no hand-rolled hover/selection duplication of
its own selection UX beyond a single config field, so the earlier "17 crates with duplication"
inventory skipped it) but still needed the mechanical SDK-breaking-change fixes plus a genuine
selection migration for its one real field: `ArchitectConfig.selected_ids`/`ArchitectPresence.selected_ids`.

## Domain decision

One interaction domain, **`program`**, one granularity **`entity`**, `HierarchyProvider::Flat` (the
68 registers — `program.rs`'s `REGISTER_IDS`/`ProgramArtifact` — are flat `EntityId`-keyed lists, no
parent/child nesting), selection-only (no hover ever existed in this crate), `SelectionMode::Single`
+ `SelectionMethod::Pick` + `MergeMode::Replace` only — matches the actual pre-migration behavior:
the ONLY real caller of the old `SetSelection` command was the document panel's element-row click
(`architect_action("setSelection", Some(json!({ "ids": [element.header.id] })))`); no marquee/multi-
select ever existed in the UI.

`window_kind_interactions` is scoped to the graph window (`ARCHITECT_WINDOW_GRAPH`) — the app's
closest analog to a primary canvas/editing surface — mirroring `dag`'s main window's identical
declaration, even though `NodeGraphScene` (like `BlockListScene`) has no `interaction_domain` field
for the wrapper to stamp yet (documented gap, matches `dag`/`space`'s precedent).

The **adjacency matrix window** (`↔️adjacency`) deliberately declares an EMPTY `interactions` vec —
verified its cells cycle their `AdjacencyKind` directly on click (`setAdjacencyKind`), there is no
genuine select-then-act step to model as a domain/granularity today (unlike what the crate-specific
briefing hypothesized). Same for the **report window** (no selectable entities).

## What changed (real behavior, not just signatures)

- `🎚️config` + `👥️presence` + both facets' 5 schema leaves (rust/proto/json/ts/graphql): deleted
  `selected_ids`, proto fields renumbered contiguously (no back-compat/reserved gaps, matches the
  precedent already applied to `space`'s own proto in this ticket).
- Deleted the whole `🎮️commands/🗂️selection` dir (`SetSelection`) + its `declare_command_enum`
  row, `command_from_action` arm, and `.view_action("setSelection", …)` manifest declaration. Row
  removal shifted every following `app_commands!` ordinal by -1 — recomputed and verified (by
  running the tests, not by hand-trust) every pinned wire-byte assertion in
  `optional_field_rows_keep_their_pre_migration_bytes`.
- `📌️panels/📄️artifact` (document panel): rebuilt via the SDK's `PanelTreeBuilder` +
  `.interaction_domain("program")` instead of the local `tree_node` helper; element rows are now bare
  `tree_item_desc` (no `.action`) whose id IS the raw `EntityId` string (globally unique already, no
  row-id prefix/mapping needed, unlike `note`'s nested block ids); register rows keep their own
  `selectRegister` action untouched, coexisting in the same tree (mirrors `note`'s
  `action_rows`/`block_items` split). Proven end-to-end by a new test that dispatches the framework's
  real `interactionSelect` action and asserts the SAME element renders `"selected":true`.
- `🎮️commands/{🏗️element,📋️register,📤️exchange,🔍️search}`: dropped every `next.selected_ids = …`/
  `.retain(...)`/`.clear()` line (add-element/add-register-item/search no longer auto-select their
  result; remove-element/remove-register-item no longer prune a config field — the framework prunes
  the "program" domain's selection itself now); doc comments on each mirror `note`'s `add-block`
  precedent language.
- `🎨️chrome::tree_node`: simplified — its `selected_ids: Option<Vec<String>>` param was only ever
  `Some(...)` from the document panel (now migrated off it); every remaining caller always passed
  `None`, so the param was dead weight post-migration and was removed (4 call sites updated:
  catalogue/report/adjacency/trace).
- `📌️panels/🔍️inspection`, `🎭️modes/✏️edit/🪟️windows/🧭️trace`, `.../🕸️graph`, `.../📋️register`:
  `ArtifactApp::render` carries no `InteractionView` (framework-wide SDK limitation, not specific to
  this crate) — inspection degrades to a document-wide register-count summary (drops its 5 typed
  per-selected-entity inspector branches, matches `gis2d`'s inspection panel precedent exactly);
  trace degrades to a document-wide audit feed (drops trace-chain/impact, which structurally need a
  root id; audit trail has a natural "show all" degradation via `audit_trail(program, None)`); graph's
  `NodeGraphScene.selection` and register's `BlockListScene.selected_id` are left at their empty/None
  defaults (matches `dag`'s main window's and `space`'s workflow window's identical, already-flagged
  gap for those two scene kinds).
- `📦️packages/🦀️rust/📦️glue.rs`: removed the deleted command dir's module wiring.
- Test surface: extended in-file `mod tests` only (no new test files) — updated/replaced every test
  that referenced the deleted `selected_ids`/`SetSelection`, added a domain-declaration test, an
  end-to-end pick-and-stamp test, and updated the trace/inspection/document-panel tests for their new
  document-wide behavior.

## Explicitly scoped out (with evidence)

`🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,🔺️diff/*}`'s
`ProgramArtifact`/`ProgramArtifactDiff` types still carry a `#[state(presence)] selected_ids` field
(plus ANTLR/EBNF/ABNF/Kaitai/Spicy grammar mirrors under `🔺️diff/{📝️text,💾️binary}`). Verified via
`grep -rln "ProgramArtifact"` that this type is consumed by NOTHING outside its own schema directory
(no runtime dispatch path, no test) — it is a documentation/schema-registry-only mirror, disconnected
from the real `ArchitectConfig`/`ArchitectPresence` `ArtifactApp` actually uses (its own
`to_snapshot`/`from_snapshot`/`set_snapshot` deliberately never round-trip presence/config fields).
It is NOT among the 9 real compile errors. Left untouched rather than hand-editing ~15 interlocking
generated-looking multi-format files (several in exotic grammar formats with no local tooling to
validate a correct edit) for zero compile/test benefit — flagged here for a future pass instead of
risking silent corruption.

## Acceptance

```
cd /Users/ueli/Documents/semio && cargo check -p semio-s-plugin-architect
```
0 errors, 25 pre-existing warnings (all in files this task never touched — `🚪️io`/`💡️inferences`
unused-import/qualification lints). Real output: `w6-architect-cargo-check.txt`.

```
cd /Users/ueli/Documents/semio && cargo test -p semio-s-plugin-architect
```
**190 passed, 1 failed** (191 total). Real output: `w6-architect-cargo-test.txt`. The 1 failure —
`artifacts::program::standards::v1::subsets::any::schema::mutations::component::tests::create_stakeholder_obeys_the_inverse_and_absorb_laws`
— is PRE-EXISTING and unrelated to hover/selection:
- `git status --porcelain -- "🧬️mutations/🦀️component.rs"` (its file) → empty; this task never
  touched it.
- `git log --date=iso -1` on that file → last commit `bb3c03742b`, 2026-08-13 20:28:08, one day
  before this ticket (`26/08/14`) opened.
- It fails deterministically (reran isolated with `--test-threads=1`, same failure) — a genuine
  `CreateStakeholder` mutation diff/absorb-law bug (a renamed stakeholder's name is lost when
  `absorb`-ing two diffs, per the panic's own left/right dump), nothing to do with `EntityId`
  generation randomness or hover/selection.

## Files touched

- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎚️config/🧬️schema/{🦀️component.rs,🔣️component.json,🟦️component.ts,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/👥️presence/🧬️schema/{🦀️component.rs,🔣️component.json,🟦️component.ts,🔗️component.graphql,🛰️component.proto}`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎨️chrome/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/📌️panels/📄️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/📌️panels/📚️catalogue/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/{↔️adjacency,🕸️graph,📋️register,📄️report,🧭️trace}/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/{🏗️element,📋️register,📤️exchange,🔍️search}/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs`
- Deleted: `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🗂️selection/` (whole dir)
