# 🏁️ architect / norm / draw — driven to 0 production serde refs + manifest lines removed

Measured 2026-09-02. Recipe (comments stripped, `#[cfg(test)] mod …` brace-stripped, `serde`-family
grep, minus `_serde::`/`Error::(Serialize|Deserialize)`/`VcsError::`/`cfg_attr(test`):

    find <plugin> -name '*.rs' -not -path '*🧪*' -not -path '*🏭*' -not -path '*🔬*' \
      | while read -r f; do awk -f /tmp/striptests.awk "$f"; done \
      | grep -E 'use[[:space:]]+serde|serde::|serde_json|#\[serde\(|derive\([^)]*\b(Serialize|Deserialize)\b' \
      | grep -vE '_serde::|Error::(Serialize|Deserialize)|VcsError::|cfg_attr\(test'

## Scoreboard

| plugin | refs before → after | manifest `[dependencies]` | manifest `[dev-dependencies]` | cargo check errors before → after |
|---|---|---|---|---|
| 🏛️architect | 1 → **0** | serde/serde_json removed | serde/serde_json added | 2457 → 2457 (0 ours, all peer churn) |
| 📕️norm | 2 → **0** | serde/serde_json removed | serde/serde_json added | 606 → 606 (0 ours, all peer churn) |
| 🖍️draw | 2 → **0** | serde/serde_json removed | serde/serde_json added | 60 → 60 (0 ours, all peer churn) |

`cargo check -p <crate> --message-format short`, `CARGO_TARGET_DIR` pinned to an isolated dir,
`RUSTC_WRAPPER=""`, error counts via `grep -cE ': error(\[|:)'` (never `^error`, which undercounts).
Each plugin's error count is byte-identical before and after the manifest edit — moving serde/
serde_json to `[dev-dependencies]` cost nothing, because production code no longer touched it.

## What each plugin actually needed

### 🏛️architect — 1 ref
File `🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: a stray
`use serde_json::Value;` backing `command_from_action`'s `Option<&Value>` bridge (JSON args →
typed `ArchitectCommand`), plus three `Value::to_string()` calls building `patch_json`/`value_json`/
`operations_json` string fields.
- `use serde_json::Value;` → `use dsl::DslValue as Value;` — matches the sibling `🗂️catalog/🦀️.rs`
  in the same plugin, which already had this exact alias (`dsl` = `extern crate
  semio_framework_os_kernel as dsl;`, declared in this crate's real root
  `📦️packages/🦀️rust/🦀️.rs` — NOT the decoy `🦀️.rs` at the plugin's top level, which is a
  different, unrelated dispatch-registration file. `find … -iname Cargo.toml` and read its `[lib]
  path` to find the real root before assuming a crate has no `dsl` alias.)
- `Value::to_string()` (three call sites) → `dsl::json::to_json_string` (works because
  `impl ToValue for DslValue` exists in `🌱️value/🔁️codec/🦀️.rs:294` — `to_json_string<T: ToValue>`
  accepts `&DslValue` directly, no bridging through `serde_json::Value`).
- Aliasing `Value` to `DslValue` also fixed a latent type mismatch: `command_from_action` was
  calling `catalog::parse_register_id`/`parse_entity_id(_from_args)` — which take
  `Option<&dsl::DslValue>` — while passing `Option<&serde_json::Value>`. That would never have
  type-checked; now both files agree on `Value = DslValue`.

### 📕️norm — 2 refs (both on one function)
File `🖥️app-surface/🦀️.rs`, `pub fn selected_check_index_arg(args: Option<&serde_json::Value>) ->
Option<u32>` — doc'd as "Builds the args-side of an app's `command_from_action` bridge", called by
no in-tree app yet (future/sibling wiring) but public API.
- Signature → `Option<&dsl::DslValue>`; body → `dsl::DslValue::as_u64` (same `Option<T>` shape as
  `serde_json::Value::as_u64`, drop-in).
- `dsl` alias already declared at norm's crate root (`extern crate semio_framework_os_kernel as
  dsl;`).
- The one test exercising it built its input with `serde_json::json!(...)` — kept, but wrapped:
  `dsl::DslValue::from(&serde_json::json!({ "index": 3 }))`. `impl From<&serde_json::Value> for
  DslValue` lives in `🌱️value/🦀️.rs:247` — this is the sanctioned oracle-bridge pattern (see
  `🪵️sourcing`'s `🗂️curate/✏️editor/🦀️.rs` test module for the same idiom), not a production
  dependency: serde_json here is dev-only, gated by the test itself needing `.workspace = true` in
  `[dev-dependencies]`.

### 🖍️draw — 2 refs, both were dead/misfiled
1. `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` had `use serde_json::Value;`
   at **file scope** (uncounted by the `#[cfg(test)] mod` brace-strip because it's a single
   `#[cfg(test)]`-gated `use` line, not a `mod` block) — but grepping the whole file for bare
   `Value` usage outside the awk-visible test region showed exactly one real use, at line 1465,
   *inside* `mod tests { use super::*; … }`. Fix: delete the file-scope import, spell the one
   call site as `serde_json::Value` explicitly. Zero production impact — this import was never
   reachable outside `#[cfg(test)]` in the first place.
2. `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/✨️macros/🦀️.rs:625` — the `statechart!`
   proc-macro emitted `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
   into *generated* code for every FSM event enum. The sibling crate `semio-s-plugin-draw-fsm`
   (`fsm/📦️packages/🦀️rust/Cargo.toml`) declares `[features] default = ["macros"]` / `macros =
   ["dep:fsm_macros"]` / `testing = []` — **no `serde` feature exists anywhere in that table**, so
   `feature = "serde"` can never be true; the cfg_attr was permanently inert dead code. Deleted the
   line outright (not gated — genuinely unreachable, not a kept oracle).

## Manifest step (identical shape for all three)
`📦️packages/🦀️rust/Cargo.toml`: cut `serde.workspace = true` / `serde_json.workspace = true` from
`[dependencies]`, paste into `[dev-dependencies]` (both already existed with
`semio-framework-async-macros`; architect also keeps its `zip` dev-dep and its oracle-crate warning
comment untouched). Ran `cargo check -p <crate>` **twice** — once right after the source-level
conversion (step 2), once again after the manifest edit (step 4) — and diffed error counts byte-for-
byte before touching Cargo.toml, per the "never clear a line you haven't compiled" rule. All three
came back identical, confirming the manifest edit was safe.

## Peer churn observed (NOT ours — do not attribute)
All three plugins are drowning in unrelated, in-flight repo-wide migrations, none touching serde:
- **Async convention debt** (dominant in 🏛️architect, ~2400 of 2457 errors): helper fns like
  `catalog::parse_register_id`/`parse_entity_id(_from_args)` and dozens of `ArtifactEditor`/
  `EditorApp` trait methods went `async fn` while call sites still use the result synchronously —
  "expected `T`, found future" / "no method `X` found for opaque type `impl Future<...>`" everywhere.
- **Mutations-module import flattening** (🏛️architect): `error[E0433]: cannot find 'mutation' in
  create_program_element` etc. — a module path that used to resolve no longer does.
- **UiNode/const-eval refactor** (📕️norm, ~all 816 errors): `error[E0015]: cannot call non-const
  associated function EditionId::new in constants`.
- **`Mutation` trait gaining `DESCRIPTORS`** (🖍️draw, all 60 errors): `error[E0277]: the trait bound
  ...: MutationLeaf is not satisfied` for every mutation leaf type.
None of these error classes appear anywhere near the lines this ticket touched (verified by
grepping each full error log for the touched files/symbols: `selected_check_index_arg`, `🗂️catalog`,
`command_from_action`, the fsm macros file, `to_json_string`) — zero hits beyond the pre-existing
async-future noise already flagged above as not-ours.

## Replication recipe for the next plugin
1. Run the striptests-awk grep above; trust only that output, not a naive `grep -r serde`.
2. For each production ref: is it a bridge type (`serde_json::Value` used as `&Value` in a
   `command_from_action`-shaped fn)? → alias `use dsl::DslValue as Value;` (check the crate's real
   `[lib] path` root for an existing `extern crate semio_framework_os_kernel as dsl;`; add it there
   if missing — no new Cargo.toml dependency needed, `semio-framework-os-kernel` is almost always
   already a dependency). Replace `Value::to_string()` with `dsl::json::to_json_string(&value)`.
   Is it a `#[cfg_attr(feature = "X")]` in generated/macro code? → check whether that feature is
   ever declared in the consuming crate's `[features]` table; if not, it's dead, delete outright.
   Is it a file-scope `use` only reachable from `#[cfg(test)]`? → move/qualify inline, delete the
   import.
3. `cargo check -p <crate>` — record the error count (grep -cE, not `^error`). This is your true
   baseline; three concurrent peer migrations mean the repo does not compile clean regardless of
   your work.
4. Cut serde/serde_json from `[dependencies]`, paste into `[dev-dependencies]` (create the section
   if absent) — never delete outright while any test uses `serde_json` as a differential oracle.
5. `cargo check -p <crate>` again. The count from step 3 must reappear exactly. If it doesn't,
   grep the diff for your own touched symbols before concluding you broke something — most drift
   between runs on this repo right now is a peer landing/reverting a change mid-flight, not you.
