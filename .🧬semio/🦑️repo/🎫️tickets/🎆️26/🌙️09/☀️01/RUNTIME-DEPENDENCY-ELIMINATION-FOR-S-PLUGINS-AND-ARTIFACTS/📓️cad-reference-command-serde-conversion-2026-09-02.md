# CAD reference command: serde_json → protocol::DslValue conversion

File: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️reference/🦀️.rs`

## Context

A sibling agent retyped `resolve_number_edit` and `command_value_json` in the parent
`✏️editor/🦀️.rs` to take/return `protocol::DslValue` instead of `serde_json::Value`. This
`🖼️reference` command module is their only consumer (`patch_cad_play_reference::handle`) and
still imported `serde_json::{json, Value}` and called `Value::as_bool` against their results,
so it could not compile.

## Changes made

- Removed `use serde_json::{json, Value};` (now unused).
- `delta_json` construction: `payload.delta.map(|entry| json!(entry))` →
  `payload.delta.map(protocol::DslValue::float)` (payload.delta is `Option<f64>`; `DslValue::float`
  is the direct constructor, mirroring the pattern in `command_value_json` in the parent file).
- Both `Value::as_bool` call sites (for the `"hidden"` and `"locked"` fields of
  `PatchCadPlayReference`) → `protocol::DslValue::as_bool`, which returns `Option<bool>` just like
  the serde_json original.
- `protocol::DslValue` is referenced with its full path, no `use` needed — same convention already
  used bare in the parent `✏️editor/🦀️.rs` (crate name resolves via the 2018+ extern prelude, no
  explicit `use protocol;` required).
- No other `serde_json`/`json!`/`Value` usage remained in this file after the edit — confirmed by a
  full read of all 107 (now 105) lines before and after.
- `resolve_number_edit(current, value_json.as_ref(), delta_json.as_ref())` call sites were already
  compatible in shape (still `Option<&DslValue>`, `Option<&DslValue>`) — no change needed there
  beyond the type of `value_json`/`delta_json` themselves now flowing as `DslValue`.
- No integer-fidelity concern here: every value handled in this file (`widthWorld`, `origin.x/y/z`)
  is genuinely `f64`; nothing that should stay a `u64`/`i64` was routed through `DslValue::float`.

## Verification

`cargo check -p semio-s-plugin-cad --message-format short` was run with an isolated
`CARGO_TARGET_DIR` and `RUSTC_WRAPPER=""` per the task's instructions (first run auto-backgrounded
by the harness after 10 minutes, but it completed shortly after and its output was inspected —
exit code 0, meaning cargo itself ran to completion and reported diagnostics normally).

The serde_json → DslValue conversion above compiles clean: none of the 10 real compiler errors in
that first run's output touch the lines this ticket's conversion changed (the `delta_json`
construction or either `as_bool` call site).

However, this SAME file had 4 additional, unrelated pre-existing errors, all on the untouched
import lines 3-6:
```
error[E0432]: unresolved import `...::change_reference_hidden::mutation`: could not find `mutation` in `change_reference_hidden`
error[E0432]: unresolved import `...::change_reference_locked::mutation`: could not find `mutation` in `change_reference_locked`
error[E0432]: unresolved import `...::change_reference_width::mutation`: could not find `mutation` in `change_reference_width`
error[E0432]: unresolved import `...::move_reference::mutation`: could not find `mutation` in `move_reference`
```
Root cause confirmed by reading the actual mutation modules directly: a concurrent, in-flight,
repo-wide "mutations" module flattening refactor by another agent has removed the nested `mutation`
submodule from each mutation dir — the struct (e.g. `ChangeReferenceHidden`) now lives directly at
`change_reference_hidden::ChangeReferenceHidden`, not `change_reference_hidden::mutation::
ChangeReferenceHidden`. `git status` shows the four sibling mutation dirs
(`👁change-reference-hidden`, `🔒change-reference-locked`, `📏change-reference-width`,
`📍move-reference`) with uncommitted renames/modifications from that other work, and the same
`::mutation::`-suffixed import pattern is broken identically in two OTHER command files in this
same package (`🎮️commands/🗺️model-definition/🦀️.rs`, `🎮️commands/🕸️node/🦀️.rs`) — confirming
this is unrelated, wider churn, not something introduced by (or in scope of) the serde_json/
DslValue conversion task.

This was a trivial, single-file, verified fix (confirmed the flattened structure by reading all 4
target mutation files directly), so it was applied here too, confined to this one file: the 4
import lines now read `...::change_reference_hidden::ChangeReferenceHidden` etc. (dropped the
`::mutation` segment). A second `cargo check` run was kicked off to confirm; it auto-backgrounded
again before finishing in this turn — its result was not observed by this agent before reporting.

**Net status**: the serde_json→DslValue conversion (this ticket's actual task) is confirmed
compiling correctly. The unrelated `::mutation::` import fix is applied and believed correct (root
cause verified by direct inspection of target modules) but not yet re-confirmed by a green
`cargo check` in this agent's own session.

Errors NOT in this file, seen in the same build (not this ticket's scope, not touched):
- `✏️editor/👥️presence/🦀️.rs:103` — E0046 missing `DESCRIPTORS`/`descriptor` trait items
- `✏️editor/🎚️config/🦀️.rs:320` — E0046 missing `DESCRIPTORS`/`descriptor` trait items
- `✏️editor/🦀️.rs:1846:108` — E0308 mismatched types, `&DslValue` vs `&Value` (a DIFFERENT call
  site in the parent file than the two functions this ticket covers — likely another consumer of a
  serde/DslValue-retyped helper still passing a `&serde_json::Value`)
- `🎮️commands/🗺️model-definition/🦀️.rs`, `🎮️commands/🕸️node/🦀️.rs` — same `::mutation::` import
  churn as above, in files this agent does not own

## Re-verification (confirmed)

Second `cargo check -p semio-s-plugin-cad --message-format short` run completed (exit code 0,
meaning cargo ran to completion; diagnostics reported normally). Result:

- Total package errors dropped from 10 (first run) to 5 (second run, 2 distinct source locations).
- **This file (`🎮️commands/🖼️reference/🦀️.rs`) has ZERO mentions anywhere in the second run's
  output** — no errors, no warnings. Both the serde_json→DslValue conversion and the `::mutation::`
  import fix are confirmed compiling clean.
- Remaining errors, NOT in this file, NOT this ticket's scope:
  - `✏️editor/👥️presence/🦀️.rs:103` — E0046 missing `DESCRIPTORS`/`descriptor`
  - `✏️editor/🎚️config/🦀️.rs:320` — E0046 missing `DESCRIPTORS`/`descriptor`
  - (The earlier `✏️editor/🦀️.rs:1846` `&DslValue`/`&Value` mismatch and the two other
    `::mutation::`-import files, `🗺️model-definition` and `🕸️node`, no longer appear in this run's
    error list — resolved by other concurrent work between the two runs.)

**Status: this ticket's task is complete and verified.** No further action needed on
`🎮️commands/🖼️reference/🦀️.rs`.
