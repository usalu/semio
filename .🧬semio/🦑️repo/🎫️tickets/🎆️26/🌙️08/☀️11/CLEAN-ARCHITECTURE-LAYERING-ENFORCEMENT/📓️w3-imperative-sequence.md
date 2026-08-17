# W3 — Imperative + Sequence (Step A schema self-registration, Step B open contribution, C3 cleanup)

Assignment: `✏️s/🔌️plugins/📜️imperative/` (+ `🧩️extensions/{control,effect,logic,math,text}`),
`✏️s/🔌️plugins/🎬️sequence/`, and `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/`.

## Precedent found

Before writing anything, grepped for existing `fn register_app_schema` implementations in `✏️s/` to see
if a prior wave had already landed the pattern for some other plugin (avoid inventing a new shape).
Found `✏️s/🔌️plugins/🌀️procedural/🎛️apps/{🧊️3d,◻2d}/🎚️config/🧬️schema/🦀️component.rs` already does exactly
this (`register_app_schema()` calling `::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor
{...})`, called from a `register_exports()` fn wired into `Plugin::builder(...).setup(register_exports)`).
Mirrored that pattern exactly for both plugins assigned here, for consistency across the codebase.

Confirmed the `::schema` alias resolves via each plugin's own `📦️glue.rs`:
`extern crate semio_framework_schema as schema;` (present in both imperative's and sequence's glue.rs).

## Step A — Schema self-registration

### Imperative (`s.imperative.imperative`, single app)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🦀️component.rs` — added
  `pub fn register_app_schema()` in a new `//region 📎 App-schema self-registration` /
  `//endregion` block (same emoji/region-name convention procedural used for the identical
  feature), transplanting the descriptor construction from framework schema's closed catalog
  (`register_all_app_schema_descriptors()` lines ~732–748) with `include_str!` paths now relative to
  this file (config facet: sibling files; presence facet: `../../👥️presence/🧬️schema/*`).
- `✏️s/🔌️plugins/📜️imperative/🦀️component.rs` — added a `register_exports()` fn that calls
  `crate::artifacts::imperative::engine::register()` (previously the bare `.setup(...)` target) plus the
  new `crate::apps::imperative::config::schema::register_app_schema()`; `.setup(register_exports)`
  replaces `.setup(crate::artifacts::imperative::engine::register)`.

### Sequence (`s.sequence.sequence`, single app)
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🎚️config/🧬️schema/🦀️component.rs` — same treatment,
  descriptor transplanted from framework schema's closed catalog (lines ~529–542).
- `✏️s/🔌️plugins/🎬️sequence/🦀️component.rs` — same `register_exports()` wiring
  (`crate::artifacts::sequence::engine::register()` + `crate::apps::sequence::config::schema::register_app_schema()`).

### Extensions (control/effect/logic/math/text) + extension_sdk
No apps in any of the 5 imperative extension crates (each is `🦀️component.rs` + `📦️packages` only, no
`🎛️apps/`) — pure library/extension-bundle crates. Step A skipped for all 5, explicitly, per the shared
recipe's "no apps → skip and say so" clause. `extension_sdk` is also app-less (shared SDK). Step A: N/A.

## Step B — Open contribution producer conversion

`grep -rn "Contribution::"` across the assignment found exactly one literal-construction (producer) site:
`✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs:99`, inside
`imperative_module_contribution(...)`, building `Contribution::ImperativeModule { app_id, module_id,
label, icon_id, manifest_json }`. (The other two `Contribution::ImperativeModule` matches, in
`🧩️extensions/📣️effect/🦀️component.rs:162` and `🔨️modules/📜️imperative/📇️registry/🦀️component.rs:96`, are
`let Contribution::ImperativeModule { .. } = ... else` destructures — consumers, not producers; left
untouched per instructions.) Sequence has zero `Contribution::` producer sites — Step B skipped for
sequence entirely.

### Shape mismatch with the generic recipe (documented deviation, additive-safe)
The recipe's literal instruction is "push `TopicContribution::new(topic, payload)` into the SAME
manifest's `topic_contributions` vec, alongside the `Contribution::<Variant>` push into `contributions`".
That literal shape assumes a `PluginManifest { contributions: vec![...], .. }` struct-literal exists at
the producer site. It doesn't, here: `grep -rn "PluginManifest\s*{"` across the whole assignment returns
nothing. The actual producer returns a standalone `ProgramContributionEntry { plugin_id, contribution }`
(no `topic_contributions` field on that type — only `PluginManifest` got one in the w2 wave), consumed
downstream via JSON serialization (`contributions_json_from_entries`) for hot-swap module sync, not via a
hand-written manifest literal anywhere in this subtree.

Adapted the recipe to fit: added a sibling, purely-additive function
`imperative_module_topic_contribution(...) -> TopicContribution` in `extension_sdk/🦀️component.rs`
(topic `"imperative.module"`, matching every extension's own `Cargo.toml`
`[package.metadata.semio] contributes = ["imperative.module"]`), building the same fields as a
`serde_json::json!({...})` payload (`appId`/`moduleId`/`label`/`iconId`/`manifestJson`, mirroring
`Contribution::ImperativeModule`'s field ts-rename shape). Then added a matching sibling
`imperative_module_topic_contribution()` wrapper in each of the 5 extension crates
(control/effect/logic/math/text), next to their existing `imperative_module_contribution()`, calling
the SDK twin with the same literal args each extension already passes to the closed-enum version.
Nothing that reads `Contribution`/`ProgramContributionEntry` was touched — purely additive new functions,
unused by any existing call site (future waves that build a real `PluginManifest`/aggregate topic list for
imperative can call these).

## C3 — sequence's dev-dependency on 4 imperative extension crates

Investigated before touching anything (validated the finding's premise rather than executing blind).
`✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/Cargo.toml` `[dev-dependencies]` has exactly the 4 named
crates (imperative-math, -text, -effect, -control — **not** -logic, confirming the audit's own count).

Traced every use of `semio_s_plugin_imperative_{math,text,effect,control}::` in the sequence crate: all
four are used **only** inside `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`,
in one `#[cfg(test)]` bootstrap fn `ensure_imperative_modules_for_tests()` (registers the four crates'
native modules + syncs their `imperative_module_contribution()` JSON into the global imperative module
registry), called from `SequenceHost::from_snapshot` under `#[cfg(test)]`.

**Finding: the premise "these are duplicate tests of the extension crates that can be relocated" does not
hold.** Every test that exercises this bootstrap is a `SequenceHost`/DAG-building test (`disconnect_steps_*`,
`build_path_*`, `step_to_dag_node_*`, `run_executes_default_snapshot_and_records_scope`,
`compile_text_renders_default_snapshot_steps`, `catalogue_json_reports_imperative_catalogue_schema`, etc.)
— none of them assert anything about the math/text/effect/control extensions' own behavior; they use real
step kinds (`math.add`, `text.concat`, `control.if`, `wait.delay`, `log.print`) as realistic fixture data to
exercise `SequenceHost`'s own port-mapping/DSL-compile/execution logic. There is no test body anywhere in
sequence's file that could be moved into an extension crate's test module — there is nothing
extension-specific being asserted. Two of the four (`run_executes_default_snapshot_and_records_scope`,
`compile_text_renders_default_snapshot_steps`) go further: they call `host.run()` / `host.compile_text()`,
which require the **real** `Executor`/operator dispatch behavior of these extensions (not just metadata) —
`math.add`'s function-kind port shapes are also asserted directly
(`function_steps_use_data_ports_without_visible_execution_pins`), which reads real `registry.operator_info`
channel data only the real math extension provides.

Given that, removing the dev-dependencies and deleting/relocating this bootstrap would **not** consolidate
duplicate tests — it would delete or falsify real `SequenceHost` integration coverage, directly
contradicting this same task's own verification requirement ("verify `cargo test -p semio-s-plugin-sequence`
... still pass"). A synthetic in-crate `Registry`/`OperatorInfo` fixture replicating the real extensions'
exact channel/execution semantics was considered and rejected: it would duplicate real extension internals
inside sequence's test module (exactly the kind of hand-maintained duplication CLAUDE.md's "no
compatibility layers / no duplication" stance argues against), for a coupling (`[dev-dependencies]`, used
only by `#[cfg(test)]` code) that is otherwise idiomatic, correct Rust — this is precisely what
dev-dependencies exist for.

**Decision: left `[dev-dependencies]` and the bootstrap fn as-is, untouched.** No files changed for C3.
Flagging this back rather than force-executing a destructive "fix" whose premise didn't survive
inspection, per this ticket's own "validate assumptions, do not blindly execute" standing instruction.

## Verification

- `cargo check -p semio-s-imperative-extension-sdk -p semio-s-plugin-imperative-control
  -p semio-s-plugin-imperative-effect -p semio-s-plugin-imperative-logic
  -p semio-s-plugin-imperative-math -p semio-s-plugin-imperative-text` — **clean, 0 errors** (only
  pre-existing `dead_code: function 'bundle' is never used` warnings, unrelated to this change — `bundle()`
  is behind `#[cfg(target_arch = "wasm32")] semio_framework_plugin::extension_exports!(bundle);` so it's
  legitimately unused in a native `cargo check`). This covers every file I actually edited for Step B.
- `cargo check -p semio-s-plugin-imperative` and `cargo check -p semio-s-plugin-sequence` — **both blocked**,
  same root cause, unrelated to any edit in this session:
  ```
  error: couldn't read ".../🎛️apps/📜️imperative/📌️panels/📄️document/🦀️component.rs": No such file or directory
  error: couldn't read ".../🎛️apps/🎬️sequence/📌️panels/📄️document/🦀️component.rs": No such file or directory
  ```
  Both are `pub mod document;` lines inside each plugin's own `📦️glue.rs` (not edited by me), referencing a
  `📌️panels/📄️document/` directory that no longer exists on disk (only `📄️artifact`/`🔍️inspection`/`🛍️catalogue`
  remain under each plugin's `📌️panels/`) — this is exactly the concurrent cross-session "document" concept
  refactor flagged in my briefing ("if you hit compile errors mentioning 'document' fields/modules that you
  did not cause, that is NOT your bug"). Confirmed it's not something specific to my edits or even to this
  plugin pair: re-ran `cargo check -p semio-s-plugin-procedural` (the crate I copied the Step A pattern
  from) and it fails with the **identical** error shape at its own `📌️panels/📄️document/🦀️component.rs`
  glue.rs line — proof this is repo-wide, pre-existing, in-progress churn, not something introduced here.
  Did not touch any `📌️panels/📄️document/` reference, any glue.rs `document` module line, or investigate
  further, per instruction.
- Because of the above, `cargo check -p semio-s-plugin-imperative` / `-p semio-s-plugin-sequence` and
  `cargo test -p semio-s-plugin-sequence` could not be run to completion this session — the plugin crates'
  own lib never reaches the point of type-checking my `register_app_schema`/`register_exports` edits (glue.rs
  fails to even parse-include all its modules first). My edits in the two plugin-root `🦀️component.rs` files
  and the two `🎚️config/🧬️schema/🦀️component.rs` files are structurally identical to the already-compiling-in-this-repo
  `procedural` precedent (same field types, same `::schema::` alias, same `Plugin::builder(...).setup(...)`
  shape) — high confidence, not proven by a green compile. Flagging honestly rather than claiming a pass I
  didn't observe.

## Files changed

- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🦀️component.rs` (Step A)
- `✏️s/🔌️plugins/📜️imperative/🦀️component.rs` (Step A wiring)
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🎚️config/🧬️schema/🦀️component.rs` (Step A)
- `✏️s/🔌️plugins/🎬️sequence/🦀️component.rs` (Step A wiring)
- `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs` (Step B — new
  `imperative_module_topic_contribution` producer twin)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/🦀️component.rs` (Step B — sibling wrapper)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️component.rs` (Step B — sibling wrapper)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🦀️component.rs` (Step B — sibling wrapper)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🦀️component.rs` (Step B — sibling wrapper)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🦀️component.rs` (Step B — sibling wrapper)

No other files edited. C3: investigated, no changes made (see rationale above).
