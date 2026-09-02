# 🧬 vcs module: scope serde import to test code only

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs`

## What changed

1. Removed the module-level `use serde::{Deserialize, Serialize};` (was line 6) — this was
   the file's only production-path serde reference.
2. `Change`'s `#[cfg_attr(test, derive(Serialize, Deserialize))]` (line ~118) now spells the
   derive paths fully-qualified: `#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`.
   This attribute sits on a production struct (outside any `mod tests`), so scoping via a local
   `use` wasn't an option — fully-qualifying the derive paths keeps it test-only-active while
   requiring no bare `serde` name in scope anywhere in production code.
   `#[cfg_attr(test, serde(rename_all = "camelCase"))]` and
   `#[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]` needed no change — the
   `serde(...)` helper attribute is registered by the derive macro itself, not by the `use`.
3. Added `use serde::{Deserialize, Serialize};` inside `#[cfg(test)] mod tests { ... }`
   (~line 1319, alongside the existing `use super::*;`) so the two local test-fixture derives
   at `DemoItem`/`DemoItemPatch` (~line 1321, 1333) keep resolving `Serialize, Deserialize` bare.
4. `mod group_history_visibility_tests` (~256-327) needed no import at all — it only ever uses
   fully-qualified `serde_json::...` paths, which resolve via the 2018+ extern prelude without
   any `use` statement.
5. Left `VcsError::Serialize(String)` / `VcsError::Deserialize(String)` (~919-920, and their
   `Display` arms ~988-989) untouched — these are error-enum variant names, unrelated to the
   `serde` crate/trait.

Net effect: no production compilation path in this file names `serde` any more; the trait is
only in scope inside `#[cfg(test)]` code, either via the new `mod tests`-local `use` or via the
fully-qualified derive path on `Change`.

## Commands run (foreground) and results

- `cargo check -p semio-framework-os-kernel --message-format=short` — PASSED, exit 0.
  (Command initially exceeded the harness's 600s foreground window under heavy concurrent
  workspace load from other agents' builds and was auto-relocated to a background shell by the
  tool itself, not by me; I waited for it to finish rather than killing/retrying, per
  instructions. See output below.)
- `cargo test -p semio-framework-os-kernel --lib vcs::tests -- --nocapture` — PASSED / see below.

(Exact tail output and exit codes appended after each run — see conversation for the raw
command output captured at run time.)
