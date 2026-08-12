apa-status: complete

# W1b — `semio-s-plugin-sourcing` (`🪵️sourcing`) — `register()` → `declaration()` conversion

## Clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` first. `🪵️sourcing` appears only under **RELEASED** (`🗂️curate` facet, SMO's own mutation-triad work, already finished) and nowhere under **HELD**. Per that file's own stated default, absence from HELD = free. Proceeded.

## What changed

### 1. `register()` → `declaration()` — one artifact, one standard, one subset, one `register()`

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (mounted as `crate::artifacts::curate::standards::v1::engine`, re-exported as `crate::artifacts::curate::engine` via the plugin's own pre-migration shim in `📦️glue.rs:306-308`). This artifact has exactly one standard (`1`) and one subset (`any`), so **one `declaration()` fn**, no folding needed.

- `register()` (old, imperative, called `crate::artifacts::curate::io_registry::register()`, `register_artifact_schema()`, `register_artifact_inference()`, `crate::apps::curate::config::schema::register_app_schema()`, `register_pilot_languages()`, and `register_document_codec_for_app::<SourcingCurateApp>(...)` directly) → `declaration() -> ArtifactDeclaration`:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.curate")
          .schema(crate::artifacts::curate::schema::curate_artifact_schema_descriptor())
          .inferences([crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::curate_artifact_inference_descriptor()])
          .composers(crate::artifacts::curate::standards::v1::engine::io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::curate::SourcingCurateApp>()
          .build()
  }
  ```
- `register_pilot_languages()` (imperative, called `dsl::register_language` 5×) → private `pilot_languages() -> &'static [dsl::LanguageSpec]`, `OnceLock`-backed, same 5 entries as data (`sourcing.curate`, `sourcing.curate.op`, `sourcing.curate.diff`, `curate.pack`, `curate.spr`) — byte-identical grammar/protocol wiring, just returned instead of registered.
- `register_artifact_schema()` and `register_artifact_inference()` deleted outright — their bodies are now exactly `.schema(...)` and `.inferences([...])`, and neither had any other call site (confirmed by grep across the plugin before deleting).
- `crate::artifacts::curate::io_registry::register()` (the artifact-root-level wrapper that called `register_composer_entries(v1::entries())`) — **deleted its call site, not ported**, per Step 4's "prefer deleting a call that merely duplicates an existing composer entry": `.composers(v1::engine::io_registry::entries())` in the declaration now performs the identical `register_composer_entries` call inside `ArtifactDeclaration::register_all`. The `io_registry` module itself (`entries()`/`compose()`, at `🗿️artifacts/🗂️curate/🦀️component.rs:154-176`) is left in place — it is now orphaned (0 remaining callers repo-wide, grep-confirmed) but removing it is unrelated cleanup outside this wave, matching the note exemplar's own precedent for its analogous orphaned `io_registry`.

Both edits followed the note exemplar's exact shape (`.schema()` → `.inferences()` → `.composers()` → `.languages()` → `.document_codec::<A>()` → `.build()`, in that order) — no deviation.

`kind` string: `"s.curate"` (matches note's own `"s.note"` — 2-segment pre-migration form; the composer table's `Dialect.artifact_kind` for this artifact is already `"s.curate"`, verified against `CURATE_DIALECT` in the same engine file).

### 2. Plugin root — `✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs`

```rust
.setup(crate::apps::curate::config::schema::register_app_schema)
.artifact(crate::artifacts::curate::engine::declaration())
```
replacing the old single `.setup(crate::artifacts::curate::engine::register)`.

### 3. Stale doc-comment references updated (no behavior change)
- `🎛️apps/🗂️curate/🎚️config/🧬️schema/🦀️component.rs:20-21` — was "Called from `crate::artifacts::curate::engine::register()`", now describes the narrowed `.setup()` call and points at `ArtifactDeclaration`'s own doc for why app-scope schema has no field.
- `🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:1-2` — was "called once from ⚙️engine::register", now "declared once by ⚙️engine::declaration, walked by `PluginBuilder::build()`".

## Does `.setup()` survive, and exactly why

Yes, narrowed to exactly one call: `crate::apps::curate::config::schema::register_app_schema`. This registers `SourcingCurateConfig`'s `AppSchemaDescriptor` (config/presence facets) into `schema::AppSchemaRegistry` — app-scope, not artifact-scope. `ArtifactDeclaration` has no field for `register_app_schema_descriptor` by design (per the W1 mechanism report's exhaustive field↔registrar mapping: 7 of 9 artifact-scoped §6 registrars have a field; `register_app_schema_descriptor` and `register_linked_flow_extension_installer` are the two documented exceptions). This is identical to note's own exemplar treatment. No other reason for `.setup()` to survive was found — no `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*` calls exist anywhere in this plugin (grepped, zero hits).

## Step 3 — plugin root closure

Root already contained only `🦀️component.rs`, `AGENTS.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages`, `🧩️extensions` (no `🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/`, no stray root data files). No `README.md` present, none required. Nothing to close.

`🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}` — all three carry `Cargo.toml` (`role = "extension"`, `extends = "sourcing"`) → **inventory only, not moved**, per the Cargo.toml-first rule and the plugin-specific note.

## Step 4 — escape hatches and deps

- `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*`: **zero hits** anywhere in the plugin (grepped).
- `crate::artifacts::curate::io_registry::register()` duplicate call: **deleted** (see §1 above), not ported — it duplicated exactly what `.composers()` now does.
- `semio-framework-os` purge: **not applicable** — `📦️packages/🦀️rust/Cargo.toml` never depended on `semio-framework-os` at all (only `semio-framework-os-kernel`, a distinct package aliased as `dsl`/`store`/`protocol`/`pack`/`vcs` in `📦️glue.rs`, which the crate does use `dsl::`/`store::`/`protocol::` extensively). Nothing to purge, nothing to file.

## Step 5 — inventory

- `CONTRIBUTED_SOURCING_MODULES: Mutex<Vec<ContributedSourcingModule>>` and `LAST_SOURCING_CONTRIBUTIONS_JSON: Mutex<String>` (⚙️engine/🦀️component.rs:591-592) — a **derived cache**, not draft/gesture state: refreshed only from host-pushed `sourcing.module` topic contributions via `sync_sourcing_module_contributions`, deduplicated by JSON-equality, never written by a user gesture directly. Flagged per the "derived cache belongs in an inference, not the draft lane" instruction — currently a plain process-local static, not routed through the inference mechanism. Pre-existing, not touched.
- `LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>` (new, this wave, inside `pilot_languages()`) and the two `ENTRIES: OnceLock<Vec<...ComposerEntry>>` statics (engine's own `io_registry` and the artifact-root `io_registry` wrapper) — inert `&'static`-slice memoization caches, identical in shape and purpose to note's own sanctioned exemplar pattern (`LANGUAGES` in note's engine file). Not a violation class — no host/engine handle involved.
- No `OnceLock<HostHandle>`-style engine/host handle found anywhere in the plugin.
- No `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]` anywhere in the plugin (grepped, zero hits).

## Real, pre-existing bugs found and fixed (in-plugin, required to make the final `cargo check --all-targets` actually pass)

Both fully inside `🪵️sourcing`'s own files, both predate this session (confirmed via `git log`/`stat` mtimes well before this session started, zero uncommitted diff before I touched them), neither related to the `ArtifactDeclaration` mechanism itself:

1. **JSON IO leaves used `serde_json::Value` where `stdio`'s `JsonSnapshot::value` is actually its own `JsonValue` type** (E0308 × 3, in `🚪️io/📥️import/…/🔣️json/…/🦀️component.rs` and the paired `📤️export` leaf). This is a known, already-underway "stdio_gap/foreign-lag fix" pattern — I found the identical fix already landed in `🗒️note` and `💠️lowpoly`'s sibling json leaves (searched before writing anything). Fixed by switching both leaves to `stdio`'s own RFC8259 text codec (`parse_json_text`/`write_json_text`/`write_json_pretty`, round-tripping via `serde_json::to_string`/`from_str`), matching `💠️lowpoly`'s json leaves verbatim — the simplest of the sanctioned variants (no per-leaf structural `Value↔JsonValue` converter needed).
2. **`🧬️mutations/🦀️component.rs` had a self-collision (E0252 × 3)**: bare `use super::create_curated_item;`/`delete_curated_item`/`change_curated_item_count` aliases, combined with `pub use create_curated_item::mutation::{create_curated_item, CreateCuratedItem};` through that same alias, put the leaf's free function and its own module alias in conflict. Compared against `🕸️dag`'s identical (compiling, released) mutations file: dag never creates the bare module alias at all — it goes straight to `pub use super::create_node::mutation::{create_node, CreateNode};` and uses the bare struct names in the enum body. Applied the identical pattern: dropped the three `use super::X;` aliases, prefixed the three `pub use` lines with `super::`, and switched the enum body + 9 test-site references from `X::mutation::Y { .. }` to bare `Y { .. }` — exactly what the compiler's own "unnecessary qualification" suggestions (seen on the very first check run) already pointed at.

Both fixes are local, mechanical, and match an already-established sibling-plugin pattern; neither touches `🗄️stdio`, `🧬️mutations/**` under any *other* plugin, or any file outside `🪵️sourcing`.

## Verification

1. **`📦️glue.rs` `#[path]` resolution**: 59 non-`.` entries, **0 missing** (scripted, resolved against the real filesystem).
2. **`include_str!`/`include_bytes!` resolution**: 51 occurrences across the plugin, **0 missing** (scripted, resolved per-call-site against its own directory, not pattern-substituted).
3. **`cargo metadata --no-deps --format-version 1`**: `OK`.
4. **`cargo check -p semio-s-plugin-sourcing --all-targets`**, `RUSTC_WRAPPER=""`, real output (6th attempt — see note below on why 5 preceded it):
   ```
   warning: `semio-s-plugin-sourcing` (lib) generated 14 warnings (10 duplicates) (run `cargo fix --lib -p semio-s-plugin-sourcing` to apply 2 suggestions)
   warning: `semio-s-plugin-sourcing` (lib test) generated 15 warnings (3 duplicates) (run `cargo fix --lib -p semio-s-plugin-sourcing --tests` to apply 11 suggestions)
       Finished `dev` profile [unoptimized] target(s) in 2m 22s
   ```
   Exit code **0**. **0 errors.** Remaining warnings are pre-existing/unrelated (unused imports, an unnecessary qualification, a dead-code pair on `SourcingEngine`'s two fields, hidden-lifetime lints inherited from the framework trait signatures) — none newly introduced by this wave's edits.

   **Why 6 attempts**: attempt 1 (before any fix) surfaced the two real pre-existing bugs above. Attempts 2–5, after fixing the JSON leaves, hit `semio-s-plugin-stdio` failing to compile with **three different, mutually inconsistent errors across the four retries** (a missing `#[path]` target mid-delete, then four `E0080` const-eval panics on `SemioDrawingMutation` kebab-case checks) — conclusive evidence of another session actively mid-editing `🗄️stdio`'s `🧬️mutations/**` (SMO/UCAS territory, explicitly off-limits per this ticket's rules) in real time, not a bug in this plugin. Followed the documented "poll rather than chase" protocol; attempt 6 landed on a moment where stdio itself compiled clean, at which point `semio-s-plugin-sourcing`'s only remaining errors were its own E0252s, fixed as described above, giving the clean run pasted.

## sharedFileRequests

None. No file outside `🪵️sourcing` was edited. `🗄️stdio`'s repeated transient failures during verification were never touched — flagged above only as evidence, not as something I fixed or need fixed on my behalf (it self-resolved by the final retry).

## Files touched

- `✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs` — `.setup()` narrowed + `.artifact(declaration())` added.
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_artifact_schema()`/`register_artifact_inference()`/`register_pilot_languages()` → `declaration()` + `pilot_languages()`.
- `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎚️config/🧬️schema/🦀️component.rs` — doc-comment update only (no code change).
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — doc-comment update only (no code change).
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — pre-existing `JsonValue`/`serde_json::Value` bug fixed (stdio_gap pattern, matching lowpoly).
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — same, export side.
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — pre-existing E0252 self-collision fixed (dropped 3 module aliases, `super::`-qualified the 3 `pub use` lines, 12 call sites switched to bare struct names).

Nothing created, nothing deleted at the file level. Scratch/verification logs: `scratch-w1b-sourcing-cargo-check-{1..6}.txt` in this ticket folder.
