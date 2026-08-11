# W3 — Sourcing (`✏️s/🔌️plugins/🪵️sourcing/` + its 3 extensions)

Assignment: Step A (app-schema self-registration) + Step B (open `TopicContribution` producer,
`sourcing.module` specifically) for `✏️s/🔌️plugins/🪵️sourcing/` and `🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}`.

## Extensions found
`ls 🧩️extensions/` → `🧱️slabs`, `🪟️windows`, `🪵️beams` (3, as briefed).

## Step A — App-schema self-registration

Sourcing has exactly one app: `sourcing-curate`, catalog id `"s.sourcing.curate"` (found at
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:1004-1020`, closed catalog — untouched). The same file's
parked `catalog-integration` region already names the exact target function path at line 1507:
`semio_s_plugin_sourcing::apps::curate::config::schema::register_app_schema()` — matched precisely.

**File:** `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎚️config/🧬️schema/🦀️component.rs`
Added a `//#region 🔖️AppSchemaRegistration` block with:
```rust
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.sourcing.curate",
        config: ::schema::FacetLeaves { rust: include_str!("🦀️component.rs"), typescript: include_str!("🟦️component.ts"), graphql: include_str!("🔗️component.graphql"), json_schema: include_str!("🔣️component.json"), proto: include_str!("🛰️component.proto") },
        presence: ::schema::FacetLeaves { rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"), typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"), graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"), json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"), proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto") },
    });
}
```
Transplanted verbatim from the closed catalog's `s.sourcing.curate` descriptor block, `include_str!`
paths shortened to be relative to this file's own new home (self-reference for the config facet's own
`.rs`, `../../👥️presence/🧬️schema/…` for the presence facet — one level up out of `🎚️config`, one more
up out of `🗂️curate`, back down into the sibling `👥️presence/🧬️schema` dir). Used `::schema::…`
(leading `::`, crate-root-relative) to match this plugin's own existing convention for the same crate
alias — see `🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:30`'s
`::schema::register_artifact_schema_descriptor(...)`. The `schema` name resolves via
`extern crate semio_framework_schema as schema;` in this plugin's own `📦️glue.rs:21` — confirmed
`AppSchemaDescriptor`/`FacetLeaves`/`register_app_schema_descriptor` are all `pub` and re-exported at
that crate's root (`📦️glue.rs: pub use component::*;`).

**File:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
Added one call, `crate::apps::curate::config::schema::register_app_schema();`, inside the existing
`pub fn register()` (the `🔖️Register` region), alongside the existing `register_artifact_schema()` call
— this is the function the plugin root's `Plugin::builder("sourcing").setup(...)` already wires up
(`✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs:10`), so no new setup wiring was needed, only the one extra
call inside the established `register()`.

No apps in this plugin were skipped — `curate` is sourcing's only app.

## Step B — Open `TopicContribution` producer (`sourcing.module`)

Grepped `Contribution::` across the whole assigned subtree. Found:
- **Real manifest producers** (3, one per extension, all identical shape):
  `🧩️extensions/🪵️beams/🦀️component.rs:15`, `🧩️extensions/🧱️slabs/🦀️component.rs:15`,
  `🧩️extensions/🪟️windows/🦀️component.rs:15` — each an `ExtensionBundle::new(...).extends("sourcing")
  .contributes(Contribution::SourcingModule { app_id, module_id, label, icon_id, typology_json,
  kinds_json })`.
- Two **test-fixture constructions** inside
  `🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (`available_modules_tracks_contributed_
  modules`, `sync_sourcing_module_contributions_adds_hot_installed_modules`) that build a bare
  `ProgramContributionEntry { contribution: Contribution::SourcingModule { .. } }` directly to feed the
  *consumer* (`sync_sourcing_module_contributions`) under test — not pushed into any `PluginManifest`/
  `ExtensionManifest`, so not "producers" in the sense the task means. Left untouched (also: touching a
  consumer's test fixture is explicitly out of scope — "do not touch any CONSUMER of `Contribution`").

Cargo.toml confirms the topic string to reuse: every one of the 3 extension crates already declares
`contributes = ["sourcing.module"]` in `[package.metadata.semio]` — used verbatim as the
`TopicContribution` topic, per instructions.

### Blocker — could not complete Step B, reported not silently skipped

The real producer sites all construct their `Contribution::SourcingModule` value inside an
`ExtensionBundle::new(..).contributes(...)` chain, i.e. they push into `ExtensionManifest.contributions`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6906-6923`), **not**
`PluginManifest.contributions`. Checked what the prior `w2-open-contribution` wave actually added the
new `topic_contributions: Vec<TopicContribution>` field to — confirmed by grep
(`grep -n "topic_contributions" 🧰️framework/…/🔌️plugin/🦀️component.rs 🧰️framework/…/🛂️manifest/🦀️component.rs`):
only `PluginManifest` got the new field (`🛂️manifest/🦀️component.rs:2844`, plus the 2 literal-site fixups
in `🔌️plugin/🦀️component.rs:5884` and `:6128`, both `PluginManifest` construction sites). `ExtensionManifest`
(same file, lines 6911-6923) has no `topic_contributions` field and no `ExtensionBundle` builder method to
set one — there is currently no open-contribution surface at all for extension bundles, only for the
plugin root.

Since sourcing's `Contribution::SourcingModule` producers are 100% extension-bundle sites (the plugin
root itself, `✏️s/🔌️plugins/🪵️sourcing/🦀️component.rs`, contributes nothing), Step B is **not
implementable for sourcing without first extending `ExtensionManifest`/`ExtensionBundle`** in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — a framework file outside my assigned
ownership (`✏️s/🔌️plugins/🪵️sourcing/` only). Did not touch it. Flagging this as a gap in the prior W2 wave
(`topic_contributions` needs the identical additive treatment on `ExtensionManifest` + a
`.topic_contributes(TopicContribution)` builder method on `ExtensionBundle`, mirroring `.contributes()`)
for whoever owns that framework file next — likely blocks every other plugin whose `Contribution`
producers are also extension bundles, not just sourcing's.

**Step B outcome: 0 of 3 producer sites converted.** No `TopicContribution` code written anywhere in the
sourcing subtree — nothing to half-apply without the missing framework surface.

## Verification

`cargo check -p semio-s-plugin-sourcing` (also transitively exercises the 3 extension crates, each
depends on it): **blocked by a pre-existing, unrelated error, not caused by my edits**:
```
error: couldn't read `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/./././../../🎛️apps/🗂️curate/🎮️commands/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs:351:13
```
Confirmed via `git status --porcelain` that this is NOT something I touched (only my 2 files show as
modified) and confirmed via `ls` that the on-disk directory is actually named `🎛️apps/🗂️curate/🎮️commands/
📄️artifact/` (containing the same `🦀️component.rs`), i.e. `📦️glue.rs`'s `pub mod document;` `#[path]`
attribute still points at the old `📄️document` directory name post-rename. This matches the briefed
concurrent "document"-concept refactor exactly (renaming `document`→something, here `artifact`, across
plugins) — per instructions, not my bug, did not touch it, moving on.

Reran also with the 3 extension crates explicitly (`cargo check -p semio-s-plugin-sourcing-beams -p
semio-s-plugin-sourcing-slabs -p semio-s-plugin-sourcing-windows`): same single blocking error,
transitively from their `sourcing_curate` path dependency — no independent errors of their own reached.

Given the crate cannot currently build for a reason unrelated to my change, verified my two edits by
manual review instead (module-path and `include_str!`-relative-path correctness against the existing,
already-compiling `register_artifact_schema()` sibling in the same file/crate, and against the exact
target function path the framework's own parked `catalog-integration` region names at line 1507)
rather than by a clean `cargo check` run. Both edits are additive-only, touch no existing signature, and
add exactly one new call site each.

## Files touched
- `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎚️config/🧬️schema/🦀️component.rs` (Step A: new `register_app_schema()`)
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (Step A: wired the call into `register()`)

No files created. No files deleted. Step B produced no file changes (blocked, see above).
