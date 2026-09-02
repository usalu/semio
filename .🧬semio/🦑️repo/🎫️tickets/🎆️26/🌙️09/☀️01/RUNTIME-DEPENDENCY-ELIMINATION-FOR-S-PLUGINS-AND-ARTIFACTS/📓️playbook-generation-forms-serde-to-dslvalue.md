# Playbook `mod generation_forms` — off `serde_json` onto `DslValue`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`, `pub mod generation_forms` (was lines 379–933).

## Pre-flight check (per task instructions)

`git diff --numstat -- <file>` was empty before editing (file was `git status` `A` — newly staged,
1276/0 lines, no commit history, no working-tree divergence from the index). No in-progress markers.
Safe to edit, confirmed before any change was made.

## What changed

Inside `mod generation_forms` only (the `mod builder_kit` region and everything above `mod
generation_forms` in the same file were left untouched — out of scope):

- `use serde_json::{json, Map, Value};` removed. Added `eval_playbook_expr`, `DslValue`,
  `PlaybookValues` to the `use super::{...}` import (all already `pub` in the parent module).
- `FormGeneration.values: Map<String, Value>` → `PlaybookValues` (the file's own pre-existing
  `pub type PlaybookValues = HashMap<String, DslValue>`, defined at line 233 in the *Runtime* region —
  reused rather than inventing a second alias).
- `initial_generation_values` return type + body: no longer round-trips through
  `super::dsl_value_to_json`; `default_value_for_block` already returns `DslValue` natively, so the
  JSON bridge was pure overhead once the field itself is `DslValue`-typed.
- `update_generation_values(..., value: Value)` → `value: DslValue`.
- `handle_generation_action(action, args: Option<&Value>, ...)` → `Option<&DslValue>`. Body unchanged
  — `DslValue::get`/`as_str` are call-compatible with the `serde_json::Value` methods used here.
- `GenerationMutation::UpdateValues { value: Value }` → `value: DslValue`.
- `generation_operations(action, args: Option<&Value>, ...)` → `Option<&DslValue>`. `arg_str` closure's
  `.and_then(Value::as_str)` → `.and_then(DslValue::as_str)`.
- `invert_generation_operation`: `Value::Null` → `DslValue::Null`.
- `generation_action(controller_id, action, args: Option<Value>) -> ActionDescriptor`: since
  `ActionDescriptor.args` is *already* `Option<DslValue>` (checked at
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:21`), this dropped the
  `dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null)` bridge entirely — the param is now
  `Option<DslValue>` and gets passed straight through.
- Every `json!({ ... })` call site building action args (tree remove/rename/select, per-field
  `on_change`, per-vector-field `on_change`) → `DslValue::object([(String, DslValue), ...])` literals.
  `"fieldIndex": index` (a `usize`) became `DslValue::uint(index as u64)` — kept as an integer per the
  contract (`DslValue::Number` has distinct `UInt`/`Int`/`Float` arms; using `float` there would have
  silently changed wire shape).
- `render_question_field`/`render_generation_form_body` signature: `values: &Map<String, Value>` →
  `&PlaybookValues`.
- **`is_block_visible` bridge removed from this module.** `super::is_block_visible` takes
  `&serde_json::Map<String, serde_json::Value>` (it's outside `mod generation_forms`, out of scope,
  untouched) and was being called with the now-`PlaybookValues`-typed `values` — a type mismatch once
  the field changed. Rather than JSON-round-tripping just to reach it, added a small
  `PlaybookValues`-native sibling inside the module:
  ```rust
  fn question_visible(question: &PlaybookBlock, values: &PlaybookValues) -> bool {
      question.condition.as_ref().map(|expr| eval_playbook_expr(expr, values).as_bool().unwrap_or(false)).unwrap_or(true)
  }
  ```
  This is exactly `is_block_visible`'s own definition, minus the `playbook_values_from_json` bridge —
  `eval_playbook_expr(expr: &PlaybookExpr, values: &PlaybookValues) -> DslValue` was already
  `DslValue`-native (Runtime region, line 243), so this needed no new machinery.
- `"vector"` field-value arm: `value.as_array().cloned()` doesn't compile for `DslValue` (`as_array()`
  returns `Option<&[DslValue]>`, an unsized slice ref — `Option<&[T]>` has no `.cloned()` because `[T]`
  isn't `Clone`; `Vec<Value>` did have it). Changed to `.map(|slice| slice.to_vec())`.
- Default/fallback field-value arm (extension block kinds): `value.to_string()` (serde_json's `Display`
  impl, which JSON-stringifies) has no `DslValue` equivalent (`DslValue` implements no `Display`).
  Replaced with `dsl::os_pack::json::to_json_string(&value)` — `DslValue: ToValue` (confirmed at
  `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:294`), and `dsl::os_pack::json` resolves because `dsl` is
  the crate-wide `extern crate semio_framework_os_kernel as dsl;` alias declared at the *owning*
  crate's root (see below) — same alias already used elsewhere in this exact file
  (`dsl::parse`, `dsl::DslValue::Null` in the code this replaced).
- Test `json!(4.0)` → `DslValue::float(4.0)` (had to change — the arg type of
  `update_generation_values` it feeds is no longer `Value`).
- **Left alone, deliberately**: the other test, `render_generations_tree_contains_add_action`, still
  calls `serde_json::to_string(&render_generations_tree(...))` fully-qualified. That's serializing a
  `UiNode` (a `Serialize`-deriving type unrelated to this migration), not `FormGeneration`/`GenerationMutation`
  — kept as-is per "keep `#[cfg(test)]`/test-domain serde_json alone."
- `mod builder_kit` (same file, different region) still uses `serde_json::{Value}` and
  `serde_json::to_string` — **out of scope**, not touched.

## Important correction to the task's own framing — verify crate is wrong

The task said to verify with `cargo check -p semio-framework-os-kernel`, describing it as the crate
that owns this file. **That is not accurate.** Grepped the *entire* repo for every `#[path = "...
📖️playbook/🦀️.rs"]` mount:

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs:35:
    #[path = "../../../📖️playbook/🦀️.rs"]
    pub mod playbook;
```

This is the file's **only** mount point in the whole tree. It compiles as part of crate
`semio-framework-os-flow` (package name from that crate's `Cargo.toml`), which does
`extern crate semio_framework_os_kernel as dsl;` / `as store;` and
`pub use semio_framework_os_kernel::os_pack;` at its own crate root — that's how `dsl::...` and
`dsl::os_pack::json::...` resolve from inside this file. `semio-framework-os-kernel` itself never
mounts this file anywhere (checked its whole crate-root glue file and grepped the entire kernel
package directory) — `cargo check -p semio-framework-os-kernel` type-checks **zero** lines of this
file. The "checks CLEAN at exit 0 in ~1m45s, precise baseline" description in the task is true of that
crate but is not evidence about this file at all; before-and-after counts against it would both read 0
regardless of what changed here.

Also note: the crate-root re-export the task cited ("`DslValue`, `ToValue`, `FromValue` are re-exported
by `semio-framework-os-kernel` (🦀️.rs:337, :347)") is real, but those line numbers are in the **kernel
crate root** file (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`), not in this playbook file —
a same-named-file (`🦀️.rs`) ambiguity worth flagging for whoever wrote that briefing.

Ran `cargo check -p semio-framework-os-kernel` anyway (as instructed) — baseline and after both exit 0,
0 errors, but per the above this only proves the kernel crate itself still compiles, not that this
edit is sound.

**Real verification target**: `cargo check -p semio-framework-os-flow`. Attempted this in the
task-specified isolated `CARGO_TARGET_DIR`
(`.../8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target`) but it was not actually
isolated in practice — many other concurrent sibling sessions in this same ticket wave are pointed at
the identical path (`ps aux` showed a dozen+ concurrent `cargo check -p semio-s-plugin-*`/`-p
semio-framework-os-kernel`/`-p semio-framework-os-flow` processes all using this exact
`CARGO_TARGET_DIR`), so my `-p semio-framework-os-flow` check queued on cargo's build-dir file lock for
the whole session with no CPU progress. Deliberately did **not** spin up a second, privately-isolated
`CARGO_TARGET_DIR` to route around this — that would cold-rebuild the entire kernel+flow dependency
graph from scratch on an already very heavily loaded machine (dozens of concurrent rustc processes
observed), risking the swap/OOM failure mode this repo's own build notes warn about. Left the queued
check running in the background; will report its real exit code/error count once it returns, rather
than guessing.

## Manual type-check walkthrough (done in lieu of a completed compile, not a substitute for one)

Traced every call site touched by the signature changes by hand against the real definitions:
- `DslValue::get(&self, key: &str) -> Option<&DslValue>` — object-key-only, matches every `.get(...)`
  use in this module (all object-shaped args).
- `DslValue::as_str/as_f64/as_bool/as_array` — all `&self -> Option<...>`, drop-in for the
  `serde_json::Value` equivalents used here.
- `DslValue::object(impl IntoIterator<Item = (String, DslValue)>)` — array literals of
  `(String, DslValue)` tuples satisfy this directly.
- `PlaybookValues::new()` resolves through the type alias to `HashMap::<String, DslValue>::new()`.
- `HashMap<String, DslValue>` satisfies `Clone + Debug + PartialEq + Serialize + Deserialize` (the
  derives on `FormGeneration`) because `DslValue` itself implements all of those
  (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:99` for the first three, `:281`/`:288` for `Serialize`/
  `Deserialize`).
- Duplicate `DslValue` import in `generation_forms_tests` (`use super::super::{DslValue, ...}` plus the
  new `use super::{..., DslValue, ...}` reaching it via `use super::*;`) is not an error — both paths
  resolve to the exact same item; Rust only rejects same-name imports of *different* items, and an
  explicit import always wins over a glob silently.

This is not a substitute for the real `-p semio-framework-os-flow` compile — flagged above as still
pending — but every line touched was checked against the actual trait/type definitions in this repo,
not assumed.

## Consumer call sites the signature change requires updating (list only — not edited, per scope)

All under `✏️s/🔌️plugins/`, none touched. Every one currently imports `serde_json::{json, Value}` and
threads `Option<&serde_json::Value>` through a local `handle_generation`-style dispatcher into
`generation_operations`/`handle_generation_action`, and/or destructures
`GenerationMutation::UpdateValues { value, .. }` expecting `serde_json::Value`, and/or reads
`FormGeneration.values`/`GenerationPlayState` expecting `serde_json::Map`.

**🌊️flow plugin** (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/`):
- `🎮️commands/🧬️add-generation/🦀️.rs`
- `🎮️commands/🧬️remove-generation/🦀️.rs`
- `🎮️commands/🧬️rename-generation/🦀️.rs`
- `🎮️commands/🧬️select-generation/🦀️.rs`
- `🎮️commands/🧬️update-generation-values/🦀️.rs`
- `🪟️windows/📝️form/🦀️.rs` (renders via `render_generation_form_body`/`FormGeneration.values`)

**🌀️procedural2d** (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `✏️editor/🎮️commands/{🧬️select-generation,🧬️add-generation,🧬️remove-generation,🧬️rename-generation,🧬️update-generation-values}/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs`
- `✏️editor/🦀️.rs` (the `handle_generation` dispatcher shown as an example below)
- `🧬️schema/🧬️mutations/🦀️.rs`, `🧬️schema/🔺️diff/📝️text/🦀️.rs`, `🧬️schema/📸️snapshot/📝️text/🦀️.rs`,
  `🧬️schema/📸️snapshot/💾️binary/🦀️.rs`

**🧊️procedural3d** (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
same shape/count as procedural2d (select/add/remove/rename/update-generation commands, generate-mode
form/preview windows, `🧬️schema/🧬️mutations/🦀️.rs`, snapshot text/binary, diff text).

Representative pattern (read, not edited — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️select-generation/🦀️.rs`):
```rust
use serde_json::{json, Value};
fn handle_generation(action: &str, args: Option<&Value>, ...) -> Emit<...> {
    ...
    let Some(operations) = generation_operations(action, args, &state, &spec) else { ... };
    ...
}
pub fn handle(payload: &SelectGeneration, ...) -> Result<...> {
    Ok(handle_generation("selectGeneration", Some(&json!({ "id": payload.id })), doc, cfg, session))
}
```
Every file in the three lists above needs its local `args`/`Value` plumbing re-typed to `DslValue` and
its `json!({...})` call sites rebuilt as `DslValue::object([...])`, mirroring exactly the edits made in
this file. Mechanical once someone starts — same substitutions, same call shapes, three times over.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs` — only `mod generation_forms` edited.

## STOP — real verification surfaced a live concurrent edit + a missed consumer (2026-09-02 09:27 CEST)

`cargo check -p semio-framework-os-flow` (the real owning crate — see correction above) finally got
the build-dir lock and returned **18 errors, exit 1** (not the 0/0 the kernel-crate check showed).
Triaged all 18 by hand against the diff:

**1. Pre-existing, file-wide, unrelated to this ticket's `generation_forms` work (12 of 18):**
`error: cannot find attribute 'value' in this scope` / `cannot find derive macro 'FromValue' in this
scope` at lines 118, 184, 198, 205, 394, 403, 406, 409, 412, 524, 987, 988, plus the dependent
`BlockKindPayload: FromValue` at 1003. Proven pre-existing and NOT caused by any edit (mine or the
concurrent one below): the `mod builder_kit` occurrence at 987/988 (`BlockKindPayload`) is on a
`#[derive(FromValue)]` line **nobody touched** — `git diff` shows zero changes there — yet it fails
identically. So `#[value(...)]`/`#[derive(FromValue)]` do not resolve through this file's
`use dsl::{FromValue, ToValue};` (`dsl` = `extern crate semio_framework_os_kernel as dsl;`, declared
in the *owning* `semio-framework-os-flow` crate's root) at all, regardless of local imports — a
structural macro-resolution problem bigger than this file, needs its own investigation (does
`semio-framework-os-flow`'s `Cargo.toml` even pull in `semio_framework_value_derive` transitively with
the right feature? worth an agent dedicated to just that). Also unrelated, different file/module
entirely: `🌿️vcs/🦀️.rs:2771` `E0502` borrow error.

**2. A real gap in my own work — a consumer I didn't check (1 of 18):**
`📖️playbook/🧬️generation/🦀️.rs:97:58` `E0308`: `self.owners.push_front(JsonOwner::Object(generation.values.into_iter()))`
— an incremental/streaming JSON walker for `GenerationPlayRoot` (mounted by *this exact file*, line
16-17: `#[path="🧬️generation/🦀️.rs"] pub mod generation;`, i.e. same taxonomy family, a different
physical file) consumes `FormGeneration.values` assuming it is still `serde_json::Map`'s `IntoIter`.
Changing `.values` to `PlaybookValues` broke it. **Not fixed** — task scope says "stay inside
📖️playbook/🦀️.rs" (the one file), and `🧬️generation/🦀️.rs` is a different file; flagging rather than
silently expanding scope, especially given point 3 below. This needs a follow-up edit (rebuild that
`JsonOwner::Object` arm over `PlaybookValues`'s `IntoIter<String, DslValue>` instead).

**3. A live concurrent edit landed on this exact file during this session.** `git diff --numstat`
crept from my own 40/33 to 46/39 with zero further edits from me; `stat` shows the file was written
15 seconds before I checked. Diffing carefully: someone added `ToValue, FromValue` to the derive list
of `PlaybookBlockOption`, `PlaybookValidationError`, `PlaybookSpec`, `FormGeneration`,
`GenerationPlayState`, and `GenerationMutation`, and merged `FromValue, ToValue` into the `use
super::{...}` line I'd just edited inside `generation_forms`. This is worth flagging loudly:
**`PlaybookBlockOption` already has hand-written `impl ToValue for PlaybookBlockOption` /
`impl FromValue for PlaybookBlockOption` right below it** (lines 128-144, with a doc comment
explaining exactly why it's hand-written and not derived — the `#[serde(alias = "id")]` decode
fallback `#[derive(FromValue)]` doesn't support). Adding the derive back on top of that struct is a
near-certain future `E0119` conflicting-implementations error once the file-wide macro-resolution
blocker (point 1) is fixed and the derive actually starts expanding.

Per this ticket's own explicit instruction on this file ("if someone began editing it after I
checked... STOP, change nothing, and report that instead") and its cited history (this exact file, this
exact collision pattern, made things worse 6→23→25 errors before a revert, earlier in this same
ticket): **stopping here, no further edits.** My own `generation_forms` substitutions (documented
above) are complete and, in isolation, type-correct by hand-trace — but real verification is blocked
first by a pre-existing file-wide problem outside my scope, second by a live peer edit in progress on
this exact file that I should not race against.
