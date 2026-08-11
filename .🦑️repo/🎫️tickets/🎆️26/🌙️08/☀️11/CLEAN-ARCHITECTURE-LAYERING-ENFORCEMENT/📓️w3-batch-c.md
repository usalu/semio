# Wave 3 — Batch C: lowpoly, mathematical, norm, note, raster

Status: IN PROGRESS

## Plan
Apply shared recipe (Step A schema self-registration + Step B open contribution producer
conversion) to each of the 5 assigned plugins independently. Using w3-cad / w3-procedural
as templates for the exact pattern (register_app_schema() fn transplanted from framework's
closed catalog, called from plugin's existing setup/register path).

Resolved directory names via `ls`:
- 💠️lowpoly
- ➗️mathematical
- 📕️norm
- 🗒️note
- 🖨️raster

Pre-read: w2-schema-api.md (open registry API, all pub), w2-open-contribution.md
(TopicContribution + PluginManifest.topic_contributions field — NOT on ExtensionManifest,
per w3-cad finding: Step B is BLOCKED for any producer going through
ExtensionBundle/ExtensionManifest since that struct has no topic_contributions field and
is out of scope (framework/products/os tree) to add it). Also read w3-cad.md and
w3-procedural.md as the exact working template for Step A's `register_app_schema()` shape
and call-site wiring.

None of the 5 assigned plugins have a `🧩️extensions/` subdirectory (confirmed via `ls` at
assignment time and re-confirmed while walking each plugin's tree during this wave).

## Framework schema descriptor ids/paths (from `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`,
`register_all_app_schema_descriptors()`) and parked catalog-integration expected fn paths
(same file, `register_all_plugin_app_schema_descriptors()`, `#[cfg(feature = "catalog-integration")]`):

- `s.lowpoly.lowpoly` (lines 614-628) → `semio_s_plugin_lowpoly::apps::lowpoly::config::schema::register_app_schema()`
- `s.mathematical.mathematical` (lines 376-390) → `semio_s_plugin_mathematical::apps::mathematical::config::schema::register_app_schema()`
- `s.norm.norm` (lines 698-714) → `semio_s_plugin_norm::config::schema::register_app_schema()` — **note: no `apps::norm::` segment**, this app's config/presence schema lives directly under the plugin root (`📕️norm/🎚️config/`, `📕️norm/👥️presence/`), not under `🎛️apps/`. Norm's `🎛️apps/` dir instead holds 14 unrelated standards sub-apps (din4108, en1990..en1999, iso16757, vdi3805, din16798, din18599) — confirmed correct via the framework descriptor's own `include_str!` paths, which point at plugin-root `🎚️config`/`👥️presence`, and via `grep -n "pub mod config"` in norm's `glue.rs` (module path matches `crate::config::schema` exactly).
- `s.raster.raster` (lines 834-849) → `semio_s_plugin_raster::apps::raster::config::schema::register_app_schema()`
- `s.note.note` (lines 851-867) → `semio_s_plugin_note::apps::note::config::schema::register_app_schema()`

## Step A — Schema self-registration (all 5 plugins, one app each)

Each app's `🎚️config/🧬️schema/🦀️component.rs` already existed with its `*Config` struct(s);
appended a `//region 📎 App-schema self-registration` block (or `//#region 🔖️Registration` in
mathematical to match that file's own existing `//#region 🔖️Config` convention) at file end,
transplanting the exact `AppSchemaDescriptor`/`FacetLeaves` construction from the framework's
closed catalog block above but with `include_str!` paths relativized to the app's own file
location (config leaves become same-directory siblings; presence leaves become
`../../👥️presence/🧬️schema/...`), calling `::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor{..})`
— confirmed the `::schema` alias (`extern crate semio_framework_schema as schema;`) is
already established in every one of these 5 crates' own `📦️glue.rs` (matches the sibling
artifact-schema registration convention `::schema::register_artifact_schema_descriptor(...)`
already present in each plugin's own `⚙️engine/🦀️component.rs`).

Wired the new `register_app_schema()` fn into each plugin's existing setup/register call chain:

- **lowpoly**: added `crate::apps::lowpoly::config::schema::register_app_schema();` to
  `register_lowpoly_exports()` in `🔧️setup/🦀️component.rs` (the fn already wired via
  `.setup(crate::register_lowpoly_exports)` in the plugin root), alongside the existing
  `crate::artifacts::lowpoly::engine::register()` call.
- **mathematical**: added `crate::apps::mathematical::config::schema::register_app_schema();`
  to `register()` in `🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
  (the plugin root's `.setup(...)` points directly at this `engine::register` fn), alongside
  the existing `register_artifact_schema()` call.
- **norm**: added `crate::config::schema::register_app_schema();` as the first line of
  `register_norm_exports()` in `🔧️setup/🦀️component.rs` (wired via `.setup(crate::register_norm_exports)`
  in the plugin root) — norm has no single "norm artifact" engine `register()` (it's 14
  independent standards apps, each its own `register_document_app`), so the plugin-wide
  setup fn is the correct single call site, matching how norm's own artifact-schema
  registrations for all 14 standards are already centralized there.
- **note**: added `crate::apps::note::config::schema::register_app_schema();` to `register()`
  in `🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, alongside the existing
  `register_artifact_schema()` call.
- **raster**: added `crate::apps::raster::config::schema::register_app_schema();` to `register()`
  in `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, alongside the existing
  `register_artifact_schema()` call.

Framework's closed catalog (`register_all_app_schema_descriptors()` and the parked
`catalog-integration` regions) — not touched, per instructions.

## Step B — Open contribution producer conversion (all 5 plugins)

`grep -rn "Contribution::" ✏️s/🔌️plugins/{💠️lowpoly,➗️mathematical,📕️norm,🗒️note,🖨️raster}/` →
**zero hits in all 5 plugins.** None of these plugins construct (or even consume/pattern-match)
any `Contribution::<Variant>(...)` value anywhere in their trees. **Step B skipped for all 5
plugins — no producer sites exist, confirmed by grep, nothing to convert.**

## Verification

Crate names resolved from each `Cargo.toml` `[package] name`: `semio-s-plugin-lowpoly`,
`semio-s-plugin-mathematical`, `semio-s-plugin-norm`, `semio-s-plugin-note`,
`semio-s-plugin-raster`.

All 5 `cargo check -p <crate>` runs are **BLOCKED by the same unrelated concurrent "document"
churn** flagged in the briefing (another session threading a `document` concept through
plugins/AppDefinition/OsAppRegistration) — identical pattern to what `w3-cad.md` and
`w3-procedural.md` independently hit in their own plugin subtrees. Each crate's own `📦️glue.rs`
(not touched by me) declares `pub mod document;` pointing at a `📌️panels/📄️document/🦀️component.rs`
(or, for norm, a plugin-root-level `📄️document/🦀️component.rs`) that does not exist on disk yet:

- `semio-s-plugin-lowpoly`: `couldn't read .../🎛️apps/💠️lowpoly/📌️panels/📄️document/🦀️component.rs` (glue.rs:576)
- `semio-s-plugin-mathematical`: `couldn't read .../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs` (glue.rs:314)
- `semio-s-plugin-norm`: `couldn't read .../📕️norm/📄️document/🦀️component.rs` (glue.rs:35, plugin-root level, no `🎛️apps/` prefix — consistent with norm's flatter layout)
- `semio-s-plugin-note`: `couldn't read .../🎛️apps/🗒️note/📌️panels/📄️document/🦀️component.rs` (glue.rs:526)
- `semio-s-plugin-raster`: `couldn't read .../🎛️apps/🖨️raster/📌️panels/📄️document/🦀️component.rs` (glue.rs:510)

Each is a single `error: couldn't read ...` (file-level `mod`-path resolution failure, not a
type error) — the compiler never reaches any Rust source my two edits touched in each crate
(`🎚️config/🧬️schema/🦀️component.rs` and one call-site line in `🔧️setup` or `⚙️engine`), so this
gives no signal either way on whether my additions themselves compile. None of my edits touch
`glue.rs`, `📌️panels/`, or anything named `document` in any of the 5 plugins — confirmed by
re-diffing exactly which lines I changed in each file. Per instructions, did not fix this
(out of scope, another session's in-progress refactor), noting and moving on. Manually
re-checked each new `register_app_schema()` block against the framework's
`AppSchemaDescriptor`/`FacetLeaves` field shapes and the sibling artifact-schema registration's
`::schema::` alias usage — no divergence found.

## Files touched

- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs` (added call line)
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call line)
- `✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/📕️norm/🔧️setup/🦀️component.rs` (added call line)
- `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call line)
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call line)

No other files edited. No Step B changes made anywhere (no producer sites found).

Status: DONE
