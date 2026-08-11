# Wave 3 — Batch A: animate, architect, block, dag, demonstrator

Status: DONE

Applied shared recipe (Step A schema self-registration + Step B open contribution producer
conversion) to each of the 5 assigned plugins independently, using w3-cad / w3-procedural /
w3-stdio as templates for the exact pattern (same `::schema::register_app_schema_descriptor`
call shape, same `//region 📎 App-schema self-registration` block, `::schema` extern-prelude
alias already established in every plugin's `📦️glue.rs`). None of the 5 have a `🧩️extensions/`
subdirectory (confirmed via `find -maxdepth 1 -iname "*extension*"` in each).

---

## 🎞️animate

### Step A — Schema self-registration
One app: `s.animate.present` (`🎛️apps/🎬️present/`). Found descriptor block at
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:494-514`; expected fn path from the parked
catalog-integration region confirmed by grep:
`semio_s_plugin_animate::apps::present::config::schema::register_app_schema()`.

Added `register_app_schema()` to
`🎛️apps/🎬️present/🎚️config/🧬️schema/🦀️component.rs` (end of file, new
`//region 📎 App-schema self-registration` block), transplanting the same `AppSchemaDescriptor`
construction (`config` leaves now same-dir `include_str!`, `presence` leaves
`../../👥️presence/🧬️schema/...`). Verified the module path against `📦️glue.rs:421-444`
(`apps::present::config::schema`).

Wired the call into `register()` in
`🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, alongside the existing
`register_artifact_schema()` / `register_document_codec_for_app` calls (established pattern:
`Plugin::builder("animate").setup(crate::artifacts::present::engine::register)`).

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🎞️animate/` → no matches. No `[package.metadata.semio]`
`contributes` entries either. **Skipped — no producer sites.**

### Verification
`cargo check -p semio-s-plugin-animate` — blocked by the known concurrent "document"-panel
churn, not my bug:
```
✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs:485:13: error: couldn't read
`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/./././../../🎛️apps/🎬️present/📌️panels/📄️document/🦀️component.rs`:
No such file or directory (os error 2)
```
`📦️glue.rs` wires `pub mod document;` for the `present` app's `📌️panels/` region, pointing at a
file that doesn't exist on disk yet — the same concurrent "document" concept refactor flagged in
the briefing. `📦️glue.rs` not touched by me. Only error in the run (single `error:` line).

### Files touched
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

---

## 🏛️architect

### Step A — Schema self-registration
One app: `s.architect.architect` (`🎛️apps/🏛️architect/`). Descriptor block at
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:579-593`. Expected fn path:
`semio_s_plugin_architect::apps::architect::config::schema::register_app_schema()` — confirmed
against `📦️glue.rs:960-981`.

Added `register_app_schema()` to
`🎛️apps/🏛️architect/🎚️config/🧬️schema/🦀️component.rs` (same region-block shape as animate).

Wired the call into `register_architect_exports`'s inner `register()` in
`🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, alongside the existing
`register_artifact_schema()` call (established pattern:
`Plugin::builder("architect").setup(crate::register_architect_exports)`, which just calls
`register()`).

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🏛️architect/` → no matches. **Skipped — no producer
sites.**

### Verification
`cargo check -p semio-s-plugin-architect` — same class of blocker, not my bug:
```
✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs:1043:13: error: couldn't read
`✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/./././../../🎛️apps/🏛️architect/📌️panels/📄️document/🦀️component.rs`:
No such file or directory (os error 2)
```
Only error in the run.

### Files touched
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

---

## 🧱️block

### Step A — Schema self-registration
Three apps: `s.block.2d` (`🎛️apps/◻2d/`), `s.block.5d` (`🎛️apps/🖐️5d/`), `s.block.3d`
(`🎛️apps/🧊️3d/`). Descriptor blocks at
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:919-968`. Expected fn paths confirmed against
`📦️glue.rs:1260-1441`: `apps::block2d::config::schema`, `apps::block3d::config::schema`,
`apps::block5d::config::schema`.

Added `register_app_schema()` to all three apps' `🎚️config/🧬️schema/🦀️component.rs` files
(same region-block shape, `id` matching each app's descriptor id).

Wired all three calls into `register_block_exports()` in `📦️packages/🦀️rust/📦️glue.rs`
(block's plugin-root setup fn is `register_block_exports`, already itself living in `glue.rs`
rather than a separate engine file — this is block's own established location for the
plugin-setup aggregator, unlike animate/architect/dag which route setup through an artifact
engine file), alongside the existing three `crate::artifacts::block*d::engine::register()`
calls.

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🧱️block/` → no matches. No
`[package.metadata.semio]` `contributes` entries either. **Skipped — no producer sites.**

### Verification
`cargo check -p semio-s-plugin-block` — same class of blocker, not my bug:
```
✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs:1323:13: error: couldn't read
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/./././../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs`:
No such file or directory (os error 2)
```
Only error in the run.

### Files touched
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`

---

## 🕸️dag

### Step A — Schema self-registration
One app: `s.dag.dag` (`🎛️apps/🕸️dag/`). Descriptor block at
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:800-820`. Expected fn path:
`semio_s_plugin_dag::apps::dag::config::schema::register_app_schema()` — confirmed against
`📦️glue.rs:346-360`.

Added `register_app_schema()` to `🎛️apps/🕸️dag/🎚️config/🧬️schema/🦀️component.rs`.

Wired the call into `register()` in
`🗿️artifacts/🕸️dag/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, alongside the existing
`register_artifact_schema()` call (established pattern:
`Plugin::builder("dag").setup(crate::artifacts::dag::engine::register)`).

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🕸️dag/` → no matches. **Skipped — no producer sites.**

### Verification
`cargo check -p semio-s-plugin-dag` — same class of blocker, not my bug:
```
✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs:406:13: error: couldn't read
`✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/./././../../🎛️apps/🕸️dag/📌️panels/📄️document/🦀️component.rs`:
No such file or directory (os error 2)
```
Only error in the run.

### Files touched
- `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

---

## 🎪️demonstrator

### Step A — Schema self-registration
No apps. `🎛️apps/🦀️component.rs` is a one-line stub comment ("Apps facet for `🎪️demonstrator` —
document app factories registered via `.register_document_app`"), no app subdirectories exist
under `🎛️apps/`, and `plugin()` in `🦀️component.rs` calls no `.register_document_app(...)` — it's
a pure pane-bundle plugin (`crate::artifacts::playground::engine::register(); crate::panes::bundle()`).
Framework's closed catalog also has no `s.demonstrator.*` app-schema descriptor (grep confirmed —
only `semio_s_plugin_demonstrator::artifacts::playground::engine::register_artifact_schema()`
appears, an artifact-schema call, already pre-existing/unrelated to app-schema Step A). **Step A
skipped — no apps needing schema registration.**

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🎪️demonstrator/` → no matches. `Cargo.toml`'s
`[package.metadata.semio]` only has `consumes = ["forms.questionKind", "flow.extension",
"process.machines"]`, no `contributes` — consistent with demonstrator being a pure consumer
(multi-pane bundle over other plugins' contributions), never a producer. **Skipped — no
producer sites.**

### Verification
`cargo check -p semio-s-plugin-demonstrator` — demonstrator's own crate produced no error of its
own (no `error:` line mentioning `semio-s-plugin-demonstrator` itself), but the overall check
failed because 6 of demonstrator's own dependency plugin crates (`process`, `gis`, `sourcing`,
`cad`, `procedural`, `puzzle` — all outside my assigned scope, not touched by me) each hit the
same "document"-panel-module-doesn't-exist-yet error as the other 4 plugins in this batch. Not my
bug, no files of mine involved:
```
✏️s/🔌️plugins/🏭️process/.../🎛️apps/🧊️3d/🎮️commands/📄️document/🦀️component.rs: No such file or directory
✏️s/🔌️plugins/🌍️gis/.../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs: No such file or directory
✏️s/🔌️plugins/🪵️sourcing/.../🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs: No such file or directory
✏️s/🔌️plugins/📐️cad/.../🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs: No such file or directory
✏️s/🔌️plugins/🌀️procedural/.../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs: No such file or directory
✏️s/🔌️plugins/🧩️puzzle/.../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs: No such file or directory
```

### Files touched
None — Step A inapplicable (no apps), Step B inapplicable (no producers).

---

## Summary table

| Plugin       | Step A apps registered          | Step B producers converted | cargo check |
|--------------|----------------------------------|-----------------------------|-------------|
| animate      | 1 (`s.animate.present`)          | 0 (none found)               | blocked, unrelated "document" churn |
| architect    | 1 (`s.architect.architect`)      | 0 (none found)               | blocked, unrelated "document" churn |
| block        | 3 (`s.block.2d/3d/5d`)           | 0 (none found)               | blocked, unrelated "document" churn |
| dag          | 1 (`s.dag.dag`)                  | 0 (none found)               | blocked, unrelated "document" churn |
| demonstrator | 0 (no apps — inapplicable)       | 0 (none found)               | own crate clean; blocked transitively by 6 unrelated dependency plugins' "document" churn |

All 5 plugins: no `🧩️extensions/` subtree found, so subtree ownership was exactly the plugin
directory itself in each case. Every blocker encountered is the same pre-existing/concurrent
"document" concept panel-module refactor flagged in the briefing (a `pub mod document;` wired in
each plugin's `📦️glue.rs` pointing at a `📌️panels/📄️document/🦀️component.rs` or
`🎮️commands/📄️document/🦀️component.rs` file that does not yet exist on disk) — none of my edits
touch `📦️glue.rs`'s panels/commands wiring (except block's `register_block_exports()` addition,
which is a separate, unrelated function in the same file) or create/rename anything under
`📌️panels/` or `🎮️commands/`. None of my two-line-per-plugin additions were reached by the
compiler before it hit the pre-existing error, so — consistent with the same caveat noted in
w3-procedural — I could not get a compiler-verified clean pass for animate/architect/block/dag;
visual re-check against the cad/procedural template shows no divergence in field shapes or the
`::schema::` alias usage.
