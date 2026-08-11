# w4b — store: `s.space.history` → `os.space.history`

## Scope
File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`

Rename the schema-id literal `s.space.history` to `os.space.history` everywhere in this
file (layering fix: this is a generic os-shell "history" abstraction, not an `s.`-product
concept). `S_SPACE_HISTORY_SCHEMA` const identifier left unchanged (only its string value
changes) — not asked to rename the identifier itself. `s.stdio.gif` (lines ~4653/4656) is
explicitly out of scope for this wave and was left untouched.

## Changes
- Line 3578: `pub const S_SPACE_HISTORY_SCHEMA: &str = "s.space.history";` → `"os.space.history"`.
- Line 3573: doc-comment literal mention `` `"s.space.history"` `` → `` `"os.space.history"` ``.
- Line 3580: doc-comment literal mention (on `SpaceHistorySnapshot`) → `"os.space.history"`.
- Line 3770: doc-comment literal mention (on `SpaceHost`) → `"os.space.history"`.
- Fixed the now-contradictory "Renamed from `os.space.history`" clause in the const's
  docstring (lines 3573-3578 originally) — that note documented a *prior* rename in the
  opposite direction (`os.` → `s.`); since this wave restores the id back to `os.space.history`,
  keeping that clause would have made the docstring self-referential/false. Replaced it with
  prose explaining the id lives under the generic `os.` schema lattice, distinct from the
  `s.` product lattice (`space::S_SPACE_SCHEMA`/`space::S_COLLECTION_SCHEMA`, both out of scope
  for this wave, still `s.`-prefixed) — this is the actual reason for the rename, not a
  migration-history footnote. No dual-support/alias left behind, per repo convention.
- Lines 5239/5265/5284/5311/5328/5351 (tests) reference the const `S_SPACE_HISTORY_SCHEMA`
  rather than the literal string, so they picked up the new value automatically — no edit
  needed there.

## Verify
- `cargo check -p semio-framework-os-kernel --features sync` → **succeeded** (`Finished
  \`dev\` profile`, only pre-existing warnings — ambiguous glob re-exports in glue.rs, unused
  vars/dead code elsewhere in the crate, none related to this change).
- Crate located via `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`, which mounts
  `🏪️store/🦀️component.rs` at `#[path = "../../🔨️modules/🏪️store/🦀️component.rs"]`; crate name
  `semio-framework-os-kernel` per its `Cargo.toml`.
- `cargo test -p semio-framework-os-kernel --features sync history` → **fails to compile**,
  but the errors are unrelated to this rename: `DemoMutation`/`DemoSnapshot` (in
  `🏪️store/🔄️sync/🦀️component.rs`, a sibling file I did not touch) don't satisfy
  `ArtifactPack`/`OpBinary`/`OpText` trait bounds at lines ~2058/2684 of that file. Grepped
  the full error output for `S_SPACE_HISTORY_SCHEMA` / `s.space.history` / `os.space.history`
  — zero matches, confirming this rename isn't implicated. Consistent with the "concurrent
  cargo workspace churn" pattern (another session's in-progress work in a shared file) — not
  fixed here, out of scope for this file-scoped assignment.
- Confirmed `s.stdio.gif` (lines 4653, 4656) still reads `"s.stdio.gif"` — untouched.

## Grep confirmation (post-edit)
```
3573: `"os.space.history"` (doc)
3578: pub const S_SPACE_HISTORY_SCHEMA: &str = "os.space.history";
3580: `"os.space.history"` (doc)
3770: `"os.space.history"` (doc)
4653/4656: "s.stdio.gif" (untouched, out of scope)
5239/5265/5284/5311/5328/5351: use S_SPACE_HISTORY_SCHEMA const (auto-updated)
```
No remaining occurrences of the literal `"s.space.history"` in this file.
