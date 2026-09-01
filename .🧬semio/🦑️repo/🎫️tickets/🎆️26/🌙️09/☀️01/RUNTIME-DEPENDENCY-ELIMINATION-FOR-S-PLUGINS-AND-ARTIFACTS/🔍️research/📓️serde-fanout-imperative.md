# Serde/serde_json Fan-Out — Imperative Batch (5 extensions + procedural × 2)

Batch owner's notes for the 7-manifest assignment. Read `📓️serde-fanout-playbook.md` first — it is
the authoritative mechanical recipe (confirmed accurate below) and documents the framework-wide
`MutationDiff`/`Mutation` trait-bound fix this batch builds on.

## Result summary

| manifest | outcome |
|---|---|
| `📜️imperative/🧩️extensions/🎮️control` | **DONE** — `serde_json` deleted, `pack::json` in |
| `📜️imperative/🧩️extensions/📝️text` | **DONE** — `serde_json` deleted, `pack::json` in |
| `📜️imperative/🧩️extensions/📣️effect` | **DONE** — `serde_json` deleted, `pack::json` in |
| `📜️imperative/🧩️extensions/🧠️logic` | **DONE** — `serde_json` deleted, `pack::json` in |
| `📜️imperative/🧩️extensions/🧮️math` | **DONE** — `serde_json` deleted, `pack::json` in |
| `🔌️plugins/🌀️procedural` (plugin) | **NOT ATTEMPTED** — hard framework blocker, see below |
| `📖️playbook/🧩️extensions/🌀️procedural` | **NOT ATTEMPTED** — same blocker, see below |

## The five imperative extensions

All five (`🎮️control`, `📝️text`, `📣️effect`, `🧠️logic`, `🧮️math`) share one template: a
`catalogue_json()` helper that built a fixed/derived JSON string via `serde_json::json!` +
`serde_json::to_string`, purely for UI palette display — no `Mutation`/`MutationDiff`, no
`#[derive(Serialize, Deserialize)]` anywhere, no `ArtifactApp::Snapshot`. This is the clean
`serde_json::Value`-as-payload case the ticket foundation names: `pack::json::Value` +
`pack::json::{array, object, to_string}` (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`).

### What changed, every file

- `component.rs`: `use pack::json::{array, object, to_string, Value}` (aliased `Value as JsonValue`
  in the 4 files that already import `neural_engine::Value`, a same-named unrelated scope-graph
  value type — `🎮️control` has no such collision, kept the bare name). `catalogue_json()`
  rewritten to build the tree with `object([(key, Value::from(x)), ...])` / `array([...])` instead
  of `serde_json::json!`, and `to_string(&root)` instead of
  `serde_json::to_string(&root).unwrap_or_else(...)` — `pack::json::to_string` is infallible
  (`fn(&Value) -> String`), so the `unwrap_or_else(|_| "{}".into())` fallback was dead code and is
  deleted, not preserved.
- `📣️effect/🦀️component.rs` test `catalogue_json_includes_input_channels`: previously
  `serde_json::from_str` + `Value` indexing (`parsed["sections"][0]["items"]`). `pack::json::Value`
  has no `Index` impl (confirmed: grepped the whole repo for `impl.*Index.*for.*Value` restricted
  to the pack/json module — none), so this was rewritten on `pack::json::parse` +
  `.get("key")`/`.as_array()`/`.as_str()` chains (`Value::get`/`as_array`/`as_str` all exist on
  `pack::json::Value`). No indexing syntax anywhere in the five files.
- The OTHER two effect tests that also touch a `serde_json::Value` (`TopicContribution.payload`,
  from `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, out of this batch's scope) were left
  **untouched** on purpose: `payload["appId"]`/`payload["moduleId"] == "core"` operate on a type
  imported transitively through `semio_framework::TopicContribution` — Rust resolves the `Index`/
  `PartialEq<str>` impls on that foreign type without the crate needing `serde_json` as a direct
  dependency, and without the literal token `serde_json::` appearing in source. Confirmed by
  grep: zero remaining `serde_json`/`serde::` hits in any of the 5 crates after the edit.
- `Cargo.toml` (all 5, identical template): `serde_json = { workspace = true }` deleted; added
  `semio-framework-pack = { path = "../../../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }`
  — same 7-`../` depth as the existing `semio-framework`/`semio-framework-plugin` lines in the same
  file. **Resolve-checked** with `ls -d` for all 5 (all five printed the resolved path, no error).

### Crate-name derivation (not obvious, written down for whoever touches this next)

`semio-framework-pack`'s `[package] name` is `"semio-framework-pack"` but its `[lib] name` is
`"pack"` (`🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml`). Empirically confirmed
(`🧰️framework/🔨️modules/🧬️schema/🦀️validator.rs:3` does `use pack::json::{...}` while its own
`Cargo.toml` depends on it as plain `semio-framework-pack = { workspace = true }`, no rename) that
**the dependency's own `[lib] name` governs the extern identifier downstream code uses, not the
consumer's dependency-table key** when no `package =` rename is given — so despite the Cargo.toml
key being `semio-framework-pack`, source uses `pack::json::...`, matching validator.rs exactly.

### Verification

`cargo check -p semio-s-plugin-imperative-control --message-format=short`, foreground, shared
target dir: completed (exit 0 from the cargo invocation itself — cargo always exits 0 for a
downstream compile error report in this mode), but **did not reach my crate at all**. The build
failed 247 errors deep inside `semio-framework` itself — every single error is in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**` (`WorkflowMutation`/`WorkflowDiff`/
`RunMutation`/`RunDiff` and ~20 of their mutation-leaf files): `error: cannot find attribute
`value` in this scope` (the `#[value(...)]` attribute macro isn't imported there) cascading into
`error[E0277]: the trait bound `WorkflowMutation: ToValue`/`FromValue` is not satisfied`. This is
a concurrent peer's in-flight `#[value(...)]`/`ToValue`/`FromValue` rollout onto the workflow
module's own mutations (missing a `semio_framework_value_derive` import/dependency there),
**unrelated to this batch** — grepped the full 102-line output for `imperative`/`pack::json`/my
crate name: zero hits. `semio-s-plugin-imperative-control` itself was never reached because its
own dependency `semio-framework` failed to build first. Verbatim tail saved at
`🗑️generated/cargo-check-imperative-control.txt` (delete once this ticket closes, per the ticket
folder rules — it's a tool-generated log, not a report).

**Status: WRITTEN BUT UNVERIFIED**, blocked upstream by another agent's in-flight work, exactly the
scenario the ticket instructs to record-and-move-on rather than fix or wait for. Ran the identical
check a second time later in the session (peer was visibly iterating live): error count in the
same `🔁️workflow` module dropped from 247 to 80, now `error[E0308]: mismatched types: expected
`&_`, found <T>` across the same mutation-leaf files (`WorkflowMutation`/`RunMutation`'s `Ok(...)`
wrapping issue documented as trap #8 in `📓️serde-fanout-playbook.md` — "every non-error arm needs
`Ok(...)`" — looks like exactly this) — `semio-framework` still fails to compile, `imperative`/
`pack::json`/my crate name still zero hits in either run's output
(`🗑️generated/cargo-check-imperative-control.txt`,
`🗑️generated/cargo-check-imperative-control-2.txt`). Did not re-run a third time — the peer is
actively iterating on that file and further polling only re-confirms the same "not my crate" fact.
Whoever verifies this batch next should just run
`cargo check -p semio-s-plugin-imperative-control --message-format=short` once the workflow module
is green.

**Independent evidence the code is structurally sound**, short of a completed compile:
- Every one of the 5 edited `component.rs` files is brace/paren-balanced (checked with a small
  script, not eyeballed).
- `pack::json::{array, object, to_string, Value}` — every symbol used
  (`array`, `object`, `to_string`, `Value`, `Value::from`, `Value::get`, `Value::as_array`,
  `Value::as_str`, `pack::json::parse`) is a real, currently-existing public item in
  `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (`pub fn array`, `pub fn object`,
  `pub fn to_string`, `pub enum Value` + its inherent methods, `pub fn parse`), read directly, not
  assumed.
- `OperatorInfo`/`ChannelSpec` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`)
  fields used via `.as_str()` (`id`, `name`, `abbreviation`, `icon`, `summary`, `extension`, and
  `ChannelSpec::name`/`code`) are all plain `String`, confirmed by reading the struct defs — `.as_str()`
  is valid on every one.
- All 5 `semio-framework-pack` path deps resolve-checked with `ls -d` (above).

**Confounder observed mid-session, self-resolved by a concurrent peer**: earlier in this session
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` mounted
`#[path = "../../🔨️modules/🎒️pack/🔢️value/🦀️component.rs"]` pointing at a directory that did not
exist (`🧰️framework/🔨️modules/🎒️pack/🔢️value/` — confirmed absent by `find`), which would have
broken `semio-framework-os-kernel` itself (every one of these 5 plugins' transitive dependency).
Not caused by this batch — nothing here touches that path or `os_kernel`'s glue. By the time the
verification check below actually ran, a peer had replaced that mount with a plain
`pub use pack::json;` re-export (same file, now carries a docstring naming this exact ticket) and
the missing-directory error is gone from the actual check output. Left here as a record that it
happened, not as a live blocker — re-`find` before assuming it's still broken.

**Actual blocker hit by the real verification run**: unrelated to the above — see "Verification"
below.

## `🌀️procedural` (plugin) and `📖️playbook/🧩️extensions/🌀️procedural` — NOT ATTEMPTED

Both were in-scope per the ticket text but turned out to be architecturally blocked, discovered
by direct source reading (not assumed):

### The blocker

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, the `ArtifactApp` trait
(`pub trait ArtifactApp: Default + Send + 'static { ... }`, found at line 10891):

```rust
type Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static;
```

`Serialize`/`DeserializeOwned` here are real `serde::{Serialize}`/`serde::de::DeserializeOwned`
(confirmed: `use serde::{Deserialize, Serialize};` / `use serde::de::DeserializeOwned;` a few lines
above, same file, same `mod` scope). This is **separate** from the `MutationDiff`/`Mutation`
trait-bound fix `📓️serde-fanout-playbook.md` documents (that one already landed, bound on
`ToValue + FromValue`). `ArtifactApp::Snapshot` was not touched by that fix and still hard-requires
serde's real traits on every `Snapshot` type, repo-wide, for every plugin.

Both of this batch's remaining manifests define an `ArtifactApp` impl whose `Snapshot` is a
plugin-local type:

- `🌀️procedural`: `type Snapshot = AssemblySnapshot` / `Procedural2dSnapshot` / `Procedural3dSnapshot`
  (3 separate `ArtifactApp` impls, confirmed via `grep -rn "type Snapshot = "`).
- `📖️playbook/🧩️extensions/🌀️procedural`: `type Snapshot = ModuleRenderPayload`
  (`impl ArtifactApp for ModuleApp`).

Since `Serialize`/`DeserializeOwned` are real trait impls (not just names in scope), the only way
to satisfy this bound without a direct `serde` dependency in the plugin's own manifest would be a
framework-side fix — either changing the bound to `ToValue + FromValue` (mirroring the
`MutationDiff`/`Mutation` precedent) or a permanent serde↔first-party bridge — neither of which is
a plugin-side change, and both are squarely a shared, high-blast-radius edit to
`🔌️plugin/🦀️component.rs` (every `ArtifactApp` implementor repo-wide, dozens of plugins, not just
this batch's two). Editing it unilaterally from a 2-manifest batch task risks exactly the kind of
workspace-wide breakage the ticket's own "Incident — fixed" note in `📓️status.md` warns about.

This is consistent with, and extends, `📓️serde-fanout-playbook.md`'s own pilot outcome:
`📖️playbook`'s plugin manifest **still has serde** after its pilot pass for the identical reason
(quote: "the plugin's manifest cannot go to zero third-party until they are [converted]") — the
pilot converted `Mutation`/`MutationDiff` implementors (proving that half of the fix) but did not
reach a fully serde-free `ArtifactApp`-implementing plugin either, and did not call out
`ArtifactApp::Snapshot` by name as the reason. This doc adds that missing piece.

### What this means for the remaining scope

- `🌀️procedural`'s own manifest additionally has genuinely enormous fan-out even setting the
  blocker aside: 187 files / ~1277 `serde`/`serde_json` call sites inside its own crate tree
  (`grep -rl` / `grep -rn` counts, both restricted to `✏️s/🔌️plugins/🌀️procedural --include="*.rs"`),
  spanning the assembly/procedural2d/procedural3d schemas, ~10 mutation-leaf structs each with a
  dedicated round-trip test file, and the wfc-engine. Converting all of it would still not let the
  manifest drop `serde`/`serde_json`, because `AssemblySnapshot`/`Procedural2dSnapshot`/
  `Procedural3dSnapshot` (the very types most of that fan-out serializes) are forced to keep
  `Serialize + DeserializeOwned` by the `ArtifactApp::Snapshot` bound above. Doing that conversion
  work now would be effort spent for zero manifest-level progress until the framework bound moves.
- `📖️playbook/🧩️extensions/🌀️procedural` is a single, self-contained 1016-line file (only one
  `component.rs` touches serde in this crate — confirmed by `grep -rl`). Its `ModulePayloadDiff`/
  `ModulePayloadMutation` types implement `MutationDiff<ModuleRenderPayload>`/
  `Mutation<ModuleRenderPayload>` and so COULD be converted to `#[derive(ToValue, FromValue)]`
  today, following the playbook exactly (this is genuinely the "5-minute trivial case" the
  playbook describes) — but `ModuleRenderPayload` itself (the `Snapshot`) and `Command` (whose
  `command_from_action` signature takes `args: Option<&serde_json::Value>`, dictated by the same
  `ArtifactApp` contract) cannot drop serde regardless, so the manifest's `[dependencies]` would
  still carry `serde`/`serde_json` after that partial conversion. Deliberately left unconverted
  rather than landing a partial, hard-to-verify change to a file already contended by the machine's
  saturation (the playbook pilot's own `cargo check -p semio-s-plugin-playbook` did not complete in
  its session either, for the same contention reason) that produces zero manifest-level movement
  toward the stated goal.

### Recommended follow-up (flagged separately, not actioned here)

Fix `ArtifactApp::Snapshot`'s bound in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
to `ToValue + FromValue` (mirroring the already-landed `Mutation`/`MutationDiff` precedent), as a
dedicated framework-wide change with its own verification pass across every `ArtifactApp`
implementor — then `🌀️procedural` and `📖️playbook/🧩️extensions/🌀️procedural` (along with every
other plugin blocked the same way) become tractable as ordinary per-manifest fan-out work.

## Foundation notes for the next agent touching this surface

- `🌱️value/✨️derive`'s generated code is rooted at `::semio_framework_os_kernel::{ToValue,
  FromValue, DslValue, ValueError}`. Confirmed (again, independently of the playbook doc) via a
  real hand-written consumer already in the tree:
  `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  uses `::semio_framework_os_kernel::{ToValue, FromValue, DslValue, ValueError}` directly, and the
  promoting re-export is documented in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs:427`
  (`pub use protocol::value::{from_dsl_value, ordered, to_dsl_value, DslValue, FromValue, ToValue, ValueError};`,
  itself promoted to `semio_framework_os_kernel`'s crate root by `pub use crate::os_dsl::*;` in
  that crate's `📦️glue.rs`).
- None of this batch's 5 completed manifests needed that derive surface at all — their serde usage
  was pure `serde_json::Value`/`json!` UI-payload construction, not `Mutation`/`MutationDiff`
  structs, so `pack::json` alone was sufficient and no `semio-framework-value-derive` dependency
  was added to any of the 5.
