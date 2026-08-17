# Raster / Reasoning-Mindmap / Procedural / Trinity-Jack (shell+lsp) / Draw-Fsm (+macros) — Report

Scope: 7 workspace members assigned by the coordinator. All verified via individual
`cargo check -p <crate> --message-format=short` runs (foreground, synchronous — no
`run_in_background`/Monitor tool used, per the hazard note about subagents never receiving
background-task notifications). All 7 are now at 0 warnings / 0 errors on `(lib)` (or, for
`procedural`, 0 warnings outside one explicitly-scoped, deliberately-untouched subsystem — see
below). No `(lib test)` targets were touched or investigated (out of scope per the ticket's
standing methodology — the workspace-wide `Mutation::apply`/`::diff` migration blocks most test
targets and is explicitly another session's in-progress work).

## 1. `semio-s-plugin-raster` (`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust`)
**6 → 0 warnings, 0 errors.**
- `🗿️artifacts/🖨️raster/🦀️component.rs`: moved a `thread_local! { ... }` block's doc comment
  from *before* the macro invocation to *inside* the block (directly above the `static`) —
  fixed an `unused_doc_comments` warning; `thread_local!` only forwards attributes/doc comments
  placed inside the block, not ones preceding the macro call itself. No content change.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`: `ComposeSource` → `ComposeSource<'_>`
  in the `compose` signature (hidden-lifetime lint, same recurring shape as ~10 other plugins
  this session); deleted unused `use semio_framework_plugin::ArtifactAnalyzer as _;` (confirmed
  zero other references crate-wide).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`: deleted dead
  `const SEMIO_RASTER_EXAMPLE_TEXT: &str = crate::artifacts::raster::dsl::SEMIO_RASTER_EXAMPLE_TEXT;`
  and its doc comment — confirmed zero references anywhere in the crate (including tests); its
  own doc comment claimed `semio_example_document`/`semio_example_json` were "the only ways it
  should be consumed" but neither function actually reads it (they call `semio_fixture_snapshot()`
  instead) — stale doc describing code that had already diverged.
- `🚪️io/📥️import/🧩️deserializers/.../🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` and the paired
  `📤️export/🧵️serializers/.../🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`: deleted the dead
  hand-rolled `json_value_to_serde`/`serde_to_json_value` converters (confirmed zero callers —
  `deserialize`/`serialize` actually go through `JsonSnapshot::to_serde_value`/`from_value`
  directly), plus their now-unused imports (`JsonValue`, `JsonMember`, `std::str::FromStr`).
  Same "hand-rolled JSON bridge superseded by the real snapshot bridge method" shape already
  seen this session in fem/gis/playbook/shooting/stdio. Lightly reworded the two module doc
  comments (they described the now-deleted converters) to describe the real code path instead.

## 2. `semio-s-plugin-reasoning-mindmap` (`✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust`)
**Already 0 warnings, 0 errors — no changes made.**
The coordinator's briefing flagged this path as possibly a *different* crate from
`semio-s-plugin-reasoning-mindmap` (already fixed in wave 3 per `📓️progress.md`). Verified this
is **not** the case: this path's `Cargo.toml` package name is literally
`semio-s-plugin-reasoning-mindmap` (description: "one crate for the wires artifact ... and the
wires play app"), and a repo-wide grep for `reasoning-mindmap` in any `Cargo.toml` turns up only
this one path. It is the same crate wave 3 already drove to 0 (the `pub use` private-extern-crate-
alias fix documented in progress.md). Confirmed via two independent full `cargo check -p
semio-s-plugin-reasoning-mindmap` runs (no `semio-s-plugin-reasoning-mindmap (lib) generated ...`
line in either, i.e. 0 warnings; `Finished` with exit 0, no errors).

## 3. `semio-s-plugin-procedural` (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust`)
**355 → 164 warnings, 0 errors** (164 all in one deliberately-untouched subsystem — see below).
First confirmed this is a genuinely different crate from the playbook extension the coordinator
flagged (`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust`, package name
`semio-s-plugin-playbook-procedural`) — different package name, different workspace member,
independently investigated. This crate (`semio-s-plugin-procedural`) had not been touched by any
prior wave.
- `cargo fix --lib -p semio-s-plugin-procedural --allow-dirty --allow-staged`: 355 → 173
  (182 mechanical fixes: unused imports, unnecessary qualifications, etc. — matches the crate's
  own reported "182 suggestions" count from the initial check).
- Hand-fixed the 9 remaining non-wfc-engine warnings:
  - `🗿️artifacts/🌀️procedural2d/…/🚪️io/🦀️component.rs` and the `🧊️procedural3d` sibling:
    `ComposeSource` → `ComposeSource<'_>`, deleted unused `ArtifactAnalyzer as _` import (same
    recurring pattern as raster/other plugins, confirmed zero other references each).
  - `🗿️artifacts/🌀️procedural2d/…/🧬️schema/🧬️mutations/🦀️component.rs`: removed unused
    `MutationKind` from `use protocol::{Mutation, MutationKind};` (only `Mutation` is used;
    `MutationKind` appears solely in a doc comment).
  - Both procedural2d/procedural3d JSON import/export leaf pairs (four files total): deleted the
    dead `json_value_to_serde`/`serde_to_json_value` hand-rolled converters and their now-unused
    imports — identical shape and identical fix to raster's #1 above (confirmed zero callers in
    each file beyond the dead function's own recursion).
- **Left alone, deliberately, 164 warnings**: an entire Wave-Function-Collapse solver engine at
  `🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/`
  — ~34 files (`beam`, `bitset`, `chunk`, `constraint`, `constraints-card`, `constraints-conn`,
  `diag`, `domain`, `error`, `evolve`, `extract`, `flow`, `grid-2d`, `grid-3d`, `heuristics`,
  `ids`, `model`, `oracle`, `outcome`, `parallel`, `repair`, `sample`, `search`, `serial`, `soft`,
  `solver-graph`, `solver-grid-2d`, `solver-grid-3d`, `sparse-3d`, `symmetry`, `tiled`,
  `topology`, `trail`, `weights`). This is a comprehensive, fully-typed, well-structured
  constraint-solver subsystem (search/backjumping, union-find connectivity, max-flow, symmetry
  groups, checkpoint/resume serialization, parallel multi-start, beam search, entropy heuristics)
  with essentially zero external callers yet — i.e. real in-progress feature work for procedural
  assembly generation, not dead code. This is architecturally the *same shape* the ticket's
  hazard list explicitly calls out (large solver/engine subsystems found intact in `cad` and
  `stdio` this session, and the *other*, unrelated `wfc_engine` in the playbook-procedural
  extension) — confirmed by reading a representative sample of the files (not gutted, not
  investigated file-by-file beyond confirming the shape and zero-caller status via the two grep
  passes above, which never touched anything under `🧩️wfc-engine/`). Re-ran `cargo check` after
  the 9 fixes above and confirmed **every one of the 164 remaining warning lines** is under this
  one directory (verified via `grep ... | grep -v wfc-engine` returning empty) and **zero new
  errors**.

## 4. `semio-s-plugin-trinity-jack-shell` (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust`)
**Already 0 warnings, 0 errors — no changes made.** Verified via `cargo check -p
semio-s-plugin-trinity-jack-shell`.

## 5. `semio-s-plugin-trinity-jack-lsp` (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust`)
**Already 0 warnings, 0 errors — no changes made.** Verified via `cargo check -p
semio-s-plugin-trinity-jack-lsp`.

## 6. `semio-s-plugin-draw-fsm` (`✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust`)
**Already 0 warnings, 0 errors — no changes made.** Verified via `cargo check -p
semio-s-plugin-draw-fsm` (its `semio-s-plugin-draw-fsm-macros` dependency also compiled silently
in the same run).

## 7. `semio-s-plugin-draw-fsm-macros` (`✏️s/🔌️plugins/🖍️draw/🔄️fsm/✨️macros/📦️packages/🦀️rust`)
**Already 0 warnings, 0 errors — no changes made.** Verified independently via `cargo check -p
semio-s-plugin-draw-fsm-macros` (not just as a dependency of #6).

## Files touched (all edits, this report)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- Plus ~160+ files auto-touched by `cargo fix --lib -p semio-s-plugin-procedural
  --allow-dirty --allow-staged` (unused-import / unnecessary-qualification / elided-lifetime
  fixes across the crate) — not individually enumerated here; verify with the recorded warning
  count deltas (355 → 173 → 164) rather than a file list.

No git commands run. No `#[allow(...)]` added anywhere. No `(lib test)` targets touched. Ticket
itself not touched (no `ticket_close`/`ticket_open`/`ticket_reopen` calls).
