# W1b — `📏️layout` (`semio-s-plugin-layout`) `.artifact()` migration

## Clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. `📏️layout`
appears under SMO's own "RELEASED — Wave C / late Wave M lanes complete" table (its mutation-facet lane is
done, workspace-compiling as of SMO's last check) and is **not** listed under any HELD section. Not held for
APA purposes either — proceeded per the ticket's own clearance rule.

## What changed

### `✏️s/🔌️plugins/📏️layout/🦀️component.rs` (plugin root)
- `.setup(crate::artifacts::layout::engine::register)` → `.artifact(crate::artifacts::layout::engine::declaration())`.
- Added `.setup(crate::apps::layout::config::schema::register_app_schema)` — this call used to happen
  *inside* the old `engine::register()` body; it is now hoisted to the plugin root's own `.setup()`,
  exactly mirroring `🗒️note`'s pattern.
- Plugin root region is otherwise unchanged; still only `Plugin::builder(...)` wiring.

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `🔖️Register` region (was lines 42–112): `pub fn register()` and `pub fn register_pilot_languages()`
  replaced with `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` and a private
  `fn pilot_languages() -> &'static [dsl::LanguageSpec]`. Field mapping:
  - `.schema(...)` ← `register_artifact_schema()`'s body (`layout_artifact_schema_descriptor()`).
  - `.inferences([...])` ← `register_artifact_inference()`'s body (`layout_artifact_inference_descriptor()`).
  - `.composers(...)` ← `crate::artifacts::layout::standards::v1::engine::io_registry::entries()`
    (the `pub mod io_registry` at the bottom of this same file, region `🚪️DerivedIoRegistry`).
  - `.languages(pilot_languages())` ← body of the old `register_pilot_languages()`, unchanged, moved
    into a private helper.
  - `.document_codec::<crate::apps::layout::LayoutPlayApp>()` ← the old
    `register_document_codec_for_app::<LayoutPlayApp>(LAYOUT_DOCUMENT_SCHEMA)` call.
  - `crate::apps::layout::config::schema::register_app_schema()` — moved out, now called from the plugin
    root's own `.setup()` (see above), matching note's exemplar exactly (app-scope config/presence schema,
    the one field `ArtifactDeclaration` deliberately omits).
- **Deleted, not migrated**: the call `crate::artifacts::layout::io_registry::register()` (was line 49 of
  the old `register()`). This is a **duplicate-IO-registration finding**, same shape as lowpoly's 7/15:
  `artifacts::layout::io_registry::register()` (the top-level module at the bottom of
  `🗿️artifacts/📏️layout/🦀️component.rs`, region `🚪️DerivedIoRegistry`) does nothing but
  `register_composer_entries(v1::entries())` where `v1` = `standards::v1::engine::io_registry` — the exact
  same `entries()` slice now passed to `.composers(...)` above. Registering it twice was harmless (same
  entries, idempotent registry) but redundant. Verified via
  `grep -rn "layout::io_registry" --include="*.rs" .` (repo-wide) that this top-level module's `register()`
  had no other caller, so the call is deleted rather than migrated. The module itself
  (`artifacts/layout/component.rs`'s `io_registry` region, lines ~639–663) is left in place as inert dead
  code — confirmed `🗒️note`'s own sibling module (`artifacts/note/component.rs`'s `io_registry` region) is
  in the identical unreferenced state after its own migration, so this matches the exemplar rather than
  diverging from it.
- Deleted the now-dead `🔖️SchemaRegistry` region: `pub fn register_artifact_schema()` and
  `pub fn register_artifact_inference()` (were lines 764–776) — both had zero callers left once their
  bodies were inlined into `declaration()`. No external caller existed before the edit either
  (`grep -rn "register_artifact_schema()\|register_artifact_inference()"` → only the deleted `register()`
  call sites).
- `ArtifactDeclaration::builder("s.layout")` — canonical-grammar mismatch is pre-existing and intentional,
  matching `🗒️note`'s own `.builder("s.note")` despite `artifact_kind().id == "2d.note"`: the declaration's
  `kind` string must match what the composer entries' `Dialect.artifact_kind` actually use, which here is
  `"s.layout"` (see `LAYOUT_DIALECT` in the `io_registry` region 20 lines below), not the manifest-facing
  `artifact_kind().id == "2d.layout"`. `ArtifactDeclaration::register_all`'s ownership assertions
  (writes/reads must touch `"s.layout"`) pass for all three registered composer entries (the native
  `LayoutAnyComposer` entry writes `s.layout`; the two export entries write foreign `s.stdio.*` dialects but
  read `s.layout`).

## `.setup()` survives — why

Exactly one call, on the plugin root: `.setup(crate::apps::layout::config::schema::register_app_schema)`.
That function (`🎛️apps/📏️layout/🎚️config/🧬️schema/🦀️component.rs:24`) registers the `s.layout.layout`
`AppSchemaDescriptor` (config + presence facet leaves) via `::schema::register_app_schema_descriptor(...)`
— app-scope config/presence, not an artifact concern, and `ArtifactDeclaration` has no field for it by
design (see that struct's own doc, `🔌️plugin/🦀️component.rs:930-941`). Identical shape to `🗒️note`'s
surviving `.setup()` call. No other `.setup()` call remains anywhere in the plugin.

## Plugin-specific note — `🛂️manifest.json` and `LAYOUT_EMIT_DEMO_DSL`

- **`🛂️manifest.json`**: already correctly located at
  `🗿️artifacts/📏️layout/📚️examples/🛂️manifest.json` (fixture data, describing the sample layout document —
  `"fixture": "📏️sample.layout"`). No root-level `🛂️manifest.json` exists; the plugin root already contains
  only `🎛️apps`, `🗿️artifacts`, `📦️packages`, `🦀️component.rs`, `AGENTS.md` — nothing to relocate for Step 3.
  Noted in passing: this file has zero `include_str!`/reference from any `.rs` in the plugin
  (`grep -rln "🛂️manifest.json"` under the plugin finds only itself) — pre-existing, orphaned, out of this
  ticket's scope.
- **`LAYOUT_EMIT_DEMO_DSL`**: `std::env::var("LAYOUT_EMIT_DEMO_DSL")` at
  `🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs:99`, inside
  `#[test] fn demo_dsl_snapshot()`, itself inside `#[cfg(test)] mod tests { ... }` (module opens at line 27
  of that file, `#[cfg(test)]` at line 27, `mod tests {` at line 28). **Fully inside `#[cfg(test)]`** —
  clear of Step 5's runtime-`std::env` inventory.

## Step 3 — plugin root closure

Already closed: `ls ✏️s/🔌️plugins/📏️layout` → `🎛️apps`, `🦀️component.rs`, `🗿️artifacts`, `AGENTS.md`,
`📦️packages`. No `🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/` dirs, no stray `#[path]` mounts outside those
five. Nothing to delete or relocate for this step.

## Step 4 — escape hatches / deps

- No `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*` calls
  anywhere in the plugin (`grep` empty).
- `semio_framework_os` stays in `Cargo.toml`: `grep -rn "semio_framework_os::"` finds real, non-registration
  usage — `semio_framework_os::{DwgDrawing, DwgGeometry, DwgEntity, DwgColor, DwgLayer}` (domain types for
  DWG import in both the top-level and engine `component.rs` files' DWG-import regions). Not purged.
- The one duplicate composer registration (`artifacts::layout::io_registry::register()`) is documented
  above — deleted, not an escape-hatch call to a foreign kind (both slices are layout's own).

## Step 5 — inventory (not touched, reported only)

- `thread_local!`: none in the plugin.
- Interior-mutable app/derived-cache state: none found beyond the two `OnceLock<Vec<ComposerEntry>>`
  patterns (`artifacts/layout/component.rs:645` top-level `io_registry::ENTRIES`,
  `⚙️engine/component.rs:784` `io_registry::ENTRIES`) and the new `pilot_languages()`'s
  `OnceLock<Vec<dsl::LanguageSpec>>` — all three are lazily-built, process-static **data** tables (leaked
  once, never mutated after `get_or_init`), not user-gesture state or a derived cache; same shape as
  `🗒️note`'s own `pilot_languages`/`io_registry::ENTRIES`.
- No `static` holding a host/engine handle (no `OnceLock<...Host>` or similar) anywhere in the plugin.
- `std::fs`/`std::process`/`Command::new`: none. `std::env`: only `LAYOUT_EMIT_DEMO_DSL`, `#[cfg(test)]`-only
  (see above).

## Step 6 — verification

1. **`#[path]` resolution** (`📦️packages/🦀️rust/📦️glue.rs`, scripted, every non-`"."` `#[path]` target
   resolved against the real filesystem relative to the glue file's own directory):
   **129 targets, 0 missing.**
2. **`include_str!`/`include_bytes!` resolution** (scripted across every `.rs` file under the plugin,
   resolved relative to each *including* file's own directory, never pattern-substituted):
   **50 targets, 0 missing.**
3. `cargo metadata --no-deps --format-version 1 >/dev/null && echo OK` → **`OK`**.
4. `RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" cargo check -p
   semio-s-plugin-layout --all-targets` — **did not reach `semio-s-plugin-layout`.** Real pasted output
   (tail of the run, full log at
   `.🦑️repo/🎫️tickets/.../ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-layout-cargo-check.txt`):
   ```
      Checking semio-s-plugin-stdio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
   error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./././././././././../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
      --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:6028:37
   error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
   ```
   This is a **pre-existing, out-of-scope breakage in `semio-s-plugin-stdio`** (a `semio-s-plugin-layout`
   dependency), not caused by this change: the `📄set-snapshot` triad dir under stdio's
   `🧿semio/v1/drawing` subset was deleted (or is mid-rename) while `stdio`'s own `📦️glue.rs` still mounts
   `#[path]`s into it (confirmed: `grep -n "set_snapshot" 📦️glue.rs` in stdio shows dozens of live
   `set_snapshot` module mounts across every stdio artifact kind — `💾binary`, `📄txt`, `🔣json`, the
   `🧿semio/drawing` subset among them). `🗄️stdio` is explicitly **not SMO's/this ticket's to touch** — per
   `plugin-release-status.md`: *"🗄️stdio — claimed by UCAS (#2548) for the 🧿️semio subset roster
   restructure."* `semio-s-plugin-layout` itself was never reached by the check (log shows `Checking
   semio-s-plugin-stdio` immediately followed by the error; `semio-s-plugin-layout` does not appear in the
   log at all — confirmed via `grep -n "Checking\|semio-s-plugin-layout"` on the full log). Per the "no
   `cargo` spam, poll rather than chase" guidance I did not re-run the check; **I am not claiming a green
   compile for `semio-s-plugin-layout` — this could not be obtained this session** because a shared,
   out-of-scope dependency crate is mid-refactor. Everything actually verifiable without a full workspace
   compile (steps 1–3 above, plus manual line-by-line review against the `🗒️note` exemplar) is clean.

## sharedFileRequests

None from my own plugin's changes. Flagging for awareness only (no action taken, out of scope per
`STAY IN YOUR PLUGIN`): `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:6028` currently `#[path]`s into a
deleted directory (`🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`),
which blocks any `cargo check`/`cargo test` that transitively depends on `semio-s-plugin-stdio` (every
plugin that depends on stdio, `📏️layout` included, is currently unable to get a green workspace-check run
until this resolves). This is UCAS's (#2548) `🧿️semio` subset roster restructure in flight, not a finding
against this ticket — surfacing it here only so a future report doesn't waste time re-diagnosing it.

## apa-status

`📏️layout` migration to `.artifact()`/`ArtifactDeclaration` is **code-complete**: plugin root wired,
`declaration()` written with every facet (`schema`, `inferences`, `composers`, `languages`,
`document_codec`), duplicate IO registration deleted, dead schema-registry functions deleted, `.setup()`
reduced to the one sanctioned app-schema exception, plugin root already closed to the mandated file set.
**Compile confirmation is blocked**, not failing — `cargo metadata` OK, `#[path]`/`include!` resolution
100%, but a full `cargo check --all-targets -p semio-s-plugin-layout` could not run to completion because
`semio-s-plugin-stdio` (a real dependency, owned by UCAS #2548) is currently broken by unrelated in-flight
work. Recommend re-running step 6.4 once stdio's `🧿️semio/v1/drawing` restructure lands.
