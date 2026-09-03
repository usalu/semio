# Fix flow schema mutations — semio-s-plugin-flow

Scope: exactly three files under
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:
- `🦀️.rs` (the `FlowMutation` dispatch enum + framework bridge)
- `👯️duplicate-widget/🧩️plan/🦀️.rs`
- `➕️create-widget/🦀️.rs`

No `cargo` was run. All reasoning below is static: module declarations were traced through the
crate's `#[path]`-based module tree in `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs` (the crate
root/lib.rs), and every referenced struct/enum was located and read at its definition site.

## 1. `DuplicateWidget` path (E0425, 3 occurrences in `🦀️.rs`, lines 35/197/213 after edit)

Evidence: the crate root `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs` declares, inside
`pub mod mutations { ... }`:

```rust
#[path = "."]
pub mod duplicate_widget {
    #[path = ".../🧬️mutations/👯️duplicate-widget/🦀️.rs"]
    pub mod mutation;
    #[path = ".../🧬️mutations/👯️duplicate-widget/🧩️plan/🦀️.rs"]
    pub mod plan;
    ...
}
```

Unlike every other sibling leaf mutation (`create_widget`, `connect_widgets`, etc.), which flattens
via `mod component; pub use component::*;`, `duplicate_widget` was **never collapsed** — it keeps
`pub mod mutation;` deliberately, wired to the composite comment above it ("owns 🦠️mutation +
🧩️plan only"). So `DuplicateWidget` (the struct in `👯️duplicate-widget/🦀️.rs`) genuinely lives at
`duplicate_widget::mutation::DuplicateWidget`, not `duplicate_widget::DuplicateWidget`.

Confirming evidence: two sibling files *outside* my scope —
`🧬️mutations/💾️binary/🦀️.rs:66` and `🧬️mutations/📝️text/🦀️.rs:74` — already reference
`crate::artifacts::flow::schema::mutations::duplicate_widget::mutation::DuplicateWidget` and are
**not** reported as broken anywhere in the error log, i.e. that path already resolves correctly
today. `👯️duplicate-widget/🧩️plan/🦀️.rs:11` (`use super::mutation::DuplicateWidget;`, a sibling
reference from inside the `plan` module) was likewise already correct and untouched.

Fix: changed all 3 occurrences in `🦀️.rs` from `super::duplicate_widget::DuplicateWidget` to
`super::duplicate_widget::mutation::DuplicateWidget`.

## 2. Stale `::mutation::` segments (E0432) — `👯️duplicate-widget/🧩️plan/🦀️.rs`

Lines 4–5 imported `connect_widgets::mutation::ConnectWidgets` and
`create_widget::mutation::CreateWidget`. Evidence these ARE collapsed (unlike `duplicate_widget`):
lib.rs declares both as `mod component; pub use component::*;` (no `mutation` submodule). Fixed to
`connect_widgets::ConnectWidgets` / `create_widget::CreateWidget`. Grepped this file (including its
`#[cfg(test)] mod tests`) for any other `::mutation::` occurrence — none found.

Also grepped `🦀️.rs` and `➕️create-widget/🦀️.rs` (test modules included) for the same stale
pattern — none found; `🦀️.rs`'s own test module already imports every sibling mutation type without
`::mutation::`, so no `#[cfg(test)]`-only instances were hiding there.

## 3. `➕️create-widget/🦀️.rs` — missing `Identified` import (E0599 × 2, lines 30/33)

`self.widget.id()` needs `Identified` in scope (`Widget: Identified<String>`, defined in
`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1050`, re-exported as `protocol::Identified`).
rustc's auto-suggestion proposed `crate::dsl::Identified`, but every sibling file in this same
mutations tree (`🔗️connect-widgets/🔺️diff/🦀️.rs`, `🔀️🪟️reorder-widgets/🔺️diff/🦀️.rs`,
`➕️create-widget/🔺️diff/🦀️.rs`, `👯️duplicate-widget/🧩️plan/🦀️.rs`, etc.) uses
`use protocol::Identified;` — followed that established convention instead. Added `Identified` to
the existing `use protocol::{MutationKind, SemanticDescriptor};` import.

## 4. NOT one of the ticket's 3 named error families, but required to compile `🦀️.rs`: the file's
   reported error count (18) did not match families 1–3 alone (11 + 3 + 4 = 18 is the CRATE-WIDE
   total across all families; family 3, `DuplicateWidgetStep` serde, turned out to live in a
   different, out-of-scope file — see §5). Grepping the log for
   `🧬️schema/🧬️mutations/🦀️.rs:` specifically found exactly 18 error sites: 3 are the
   `DuplicateWidget` path (§1), and the other **15** are `E0599`/`E0631` errors saying
   `flow::FlowMutation` has no variants `Widgets`/`Synapses`/`SetLayout`/`SetFixture`, plus a
   `filter_map` type mismatch on `flow::flow_fixture_operations(...)`.

   Root cause (verified by reading the definition): the framework crate
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs` was restructured
   by a concurrent, out-of-scope change (doc comment there cites
   "RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass"). Its
   `FlowMutation` enum changed from 4 generic `CollectionMutation<...>`-wrapping variants
   (`Widgets`/`Synapses`/`SetLayout`/`SetFixture`) to 10 concrete per-verb leaf variants
   (`AddWidget`, `RemoveWidget`, `MoveWidget`, `ChangeWidget`, `AddSynapse`, `RemoveSynapse`,
   `MoveSynapse`, `ChangeSynapse`, `ChangeLayout`, `ReplaceFlowFixture`), each read directly from
   its own leaf file under that same framework directory. Also,
   `flow::flow_fixture_operations(...)` now returns `MutationApplyResult<Vec<FlowMutation>>`
   (`= Result<..., MutationApplyError>`) instead of a bare `Vec<FlowMutation>`.

   Since `🦀️.rs` is unambiguously in my file scope and this is what actually keeps it from
   compiling, I rewrote `snapshot_operations`, `from_framework_mutation`, and `to_framework_mutation`
   (the "🌉️FrameworkBridge" region) against the new leaf shape:
   - `snapshot_operations` now calls `.unwrap_or_default()` on the `Result` before `.into_iter()`
     (keeps its own signature `-> Vec<FlowMutation>` unchanged — its two callers,
     `✏️editor/🎮️commands/🪟️patch-flow-widgets/🦀️.rs:43` and
     `✏️editor/🎮️commands/🪟️rename-flow-widget/🦀️.rs:63`, are both outside my scope, so the
     signature could not be changed to `Result` without touching those).
   - `from_framework_mutation`/`to_framework_mutation` now match/construct the 10 new leaf variants
     one-to-one against the existing plugin-local leaf types (verified field-for-field by reading
     each: `🔗️connect-widgets/🦀️.rs`, `🔄️update-synapse-endpoints/🦀️.rs`, `🔀️🪟️reorder-widgets/🦀️.rs`,
     `🔀️reorder-synapses/🦀️.rs`, `🗑️delete-widget/🦀️.rs`, `✂️disconnect-widgets/🦀️.rs`,
     `🔁️replace-widget/🦀️.rs`, `📍️move-widgets/🦀️.rs`).
   - The new framework leaves use `index`/`to_index: u32` ("wire" index) where the plugin-local
     types use `usize` ("native" index — confirmed via `protocol::CollectionMutation`'s old
     `Add { index: usize, .. }` and via the framework's own private `flow_wire_index`/
     `flow_native_index` helpers, which formalize this u32-wire/usize-native split but are not
     `pub`, so my bridge does plain `as u32`/`as usize` casts rather than reusing them.
     `u32 -> usize` is always lossless; `usize -> u32` truncates only past 4 billion widgets, never
     realistic here).
   - `flow::SynapseSpec`'s public fields (`id`, `from`, `from_port`, `to`, `to_port`) are unchanged
     by the framework refactor (confirmed at
     `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs:195`), so the
     Connect/UpdateSynapseEndpoints construction logic is otherwise unchanged.
   - Removed the now-unused `use protocol::CollectionMutation` from the top-level import (grepped
     the whole file afterward — no remaining reference).
   - Updated the two doc comments in that region, and the two `"set-fixture has no semantic
     mutation representation..."` error-message string literals, from the old `SetFixture` naming
     to `ReplaceFlowFixture` for accuracy (CLAUDE.md: docstrings/messages must not go stale).

## 5. Explicitly OUT of scope, found but not touched

- **`DuplicateWidgetStep: serde::Serialize`/`Deserialize` (4 errors)** — the ticket named this as
  "my" family 3, but all 4 occurrences are in
  `✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs` (lines 33/174/178/189/220), which is under
  `✏️editor/`, not `🧬️schema/🧬️mutations/`. Per the explicit "touch nothing outside
  `…/🧬️schema/🧬️mutations/`" instruction I left this file alone. Whoever owns `✏️editor/` should
  apply the same `ToValue`/`FromValue` migration already used by `DuplicateWidget` itself
  (`👯️duplicate-widget/🦀️.rs`) — derive `value_derive::ToValue, value_derive::FromValue` and make
  `serde::Serialize`/`Deserialize` `#[cfg_attr(test, derive(...))]`-only, matching the sibling
  composite type.
- `🧬️mutations/💾️binary/🦀️.rs` and `🧬️mutations/📝️text/🦀️.rs` — under `🧬️mutations/` but not in my
  3-file list; not reported as broken in the log (they already spell
  `duplicate_widget::mutation::DuplicateWidget` correctly), so left untouched.
- `📍️move-widgets/🦀️.rs`'s own doc comment (line 4) still says `SetLayout` (now `ChangeLayout`) —
  stale terminology, but that file is outside my 3-file scope.
- Numerous other errors in `✏️editor/🦀️.rs`, `✏️editor/📌️panels/*`, `👁️viewer/🦀️.rs`,
  `✏️editor/🎚️config/🦀️.rs`, `✏️editor/👥️presence/🦀️.rs` (missing `Arc` import, `Label` type
  mismatches, `Menu` future-not-awaited, `render` return-type mismatch, missing `MutationKind`
  trait items, etc.) — all outside `🧬️schema/🧬️mutations/`, left untouched.

## What I could NOT verify without compiling

Everything above was checked by reading the actual definition sites (framework leaf mutation
structs, `protocol::CollectionMutation`, `protocol::Identified`'s impl for `Widget`,
`protocol::MutationApplyResult`'s type alias, lib.rs's `#[path]` module tree) rather than by
inference, so I'm confident in the reasoning. That said, I did **not** run `cargo check` — I cannot
confirm:
- That there are no further, currently-unreported errors in these 3 files that only surface once
  the 18+2+2 reported ones are fixed (rustc does not always report every error in one pass,
  especially past a type mismatch).
- That the `#[derive(protocol::Mutations)]` macro on `FlowMutation` (line 20) accepts a composite
  variant (`DuplicateWidget`) pointing at `duplicate_widget::mutation::DuplicateWidget` without
  further macro-level requirements I can't see from the derive call site alone (I did not read the
  `protocol::Mutations` proc-macro implementation itself).
- Whether `MutationApplyError` (the error type inside `flow::flow_fixture_operations`'s
  `MutationApplyResult`) can realistically be produced by real inputs today, i.e. whether
  `.unwrap_or_default()` will ever actually discard a real error in practice, versus only the
  theoretical u32-overflow case its private wire-index helpers guard against.
- The overall crate-wide build result — other agents are concurrently editing sibling files
  (`✏️editor/`, the framework `🌊️flow` module itself) whose errors are outside my scope and were
  still present as of the error log snapshot I worked from.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️duplicate-widget/🧩️plan/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🦀️.rs`
