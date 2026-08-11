# W3 — Playbook (`✏️s/🔌️plugins/📖️playbook/` incl. `🧩️extensions/🌀️procedural`)

Scope: `📖️playbook` plugin subtree + its one extension (`🌀️procedural`, the module-block extension —
a separate crate from the top-level `procedural` plugin owned by another agent).

## Step B — Open contribution producer conversion (EXTRA TASK, done first per instructions)

`grep -rn "Contribution::" ✏️s/🔌️plugins/📖️playbook/` found 4 sites:
- `🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs:44` — **consumer** (destructures
  `entry.contribution` while iterating parsed `contributions_json`). Not touched, per instructions (consumers
  are a later wave's job).
- `🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs:88` — inside `#[cfg(test)]`, builds
  a `ProgramContributionEntry{ contribution: Contribution::PlaybookBlockKind{..} }` purely as JSON test
  fixture data for the consumer test above (`render_builder_palette_includes_contributed_block_kinds`). Not a
  manifest producer — no `PluginManifest`/`ExtensionManifest` is being built here, just a config string.
  Judged out of scope for "producer" (nothing to push a matching `TopicContribution` into) and left alone.
- `🧩️extensions/🌀️procedural/🦀️component.rs:902` — **consumer** (destructures `manifest.contributions[0]`
  in a test assertion). Not touched.
- `🧩️extensions/🌀️procedural/🦀️component.rs:846` — **the real producer.** `module_extension_bundle()` builds
  an `ExtensionBundle` via `.contributes(Contribution::PlaybookBlockKind { app_id, block_kind: "buildingComponent", label, icon_id, default_value_json, params_body_key, preview_body_key })`.

### Blocker on the one real producer — reported, not worked around
`ExtensionBundle`'s manifest type is `ExtensionManifest`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6911-6923`, the WIT extension-world payload
type) — fields: `extension_id, label, version, extends, capabilities, contributions`. **It has no
`topic_contributions` field.** Only `PluginManifest` (the plain-plugin manifest, a different struct) got the
additive `topic_contributions: Vec<TopicContribution>` field in the prior wave (`📓️w2-open-contribution.md`).
Confirmed by re-reading the current file: `PluginManifest`'s two struct-literal sites (`🔌️plugin/🦀️component.rs:5884`
and `:6120`) already carry `topic_contributions: Vec::new()`/`vec![]` (fixed by an intervening wave), but
`ExtensionManifest` was never given the field at all — this isn't a stale literal, the field itself doesn't exist
on the type.

The task's instruction was to "push into the SAME manifest's `topic_contributions` vec" — that vec does not
exist on `ExtensionManifest`. Adding it requires editing
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, which is the OS product tree — explicitly
out of my ownership for this wave (framework/os product tree forbidden; only the playbook plugin subtree is
mine). I did **not** edit that file and did **not** invent a workaround (e.g. stuffing the topic elsewhere) —
doing so would fork the mechanism from what the rest of the codebase expects `TopicContribution` to mean.

**Left `🧩️extensions/🌀️procedural/🦀️component.rs:846` unconverted.** The topic string this producer would use,
confirmed correct against this crate's own `Cargo.toml` metadata
(`🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:16`, `contributes = ["playbook.blockKind"]` — matches
the topic given in my assignment verbatim), is `"playbook.blockKind"`, with a payload transplanting the same
7 fields the `Contribution::PlaybookBlockKind` variant carries:
```rust
.contributes(Contribution::PlaybookBlockKind { app_id: "playbook-play".into(), block_kind: "buildingComponent".into(), label: "Building Component".into(), icon_id: "building".into(), default_value_json: r#"{"height":6,"radius":0.5,"sides":6}"#.into(), params_body_key: BODY_PARAMS.into(), preview_body_key: BODY_PREVIEW.into() })
// would also need, once ExtensionManifest gains the field:
.topic_contributes(TopicContribution::new("playbook.blockKind", serde_json::json!({
    "appId": "playbook-play", "blockKind": "buildingComponent", "label": "Building Component",
    "iconId": "building", "defaultValueJson": r#"{"height":6,"radius":0.5,"sides":6}"#,
    "paramsBodyKey": BODY_PARAMS, "previewBodyKey": BODY_PREVIEW,
})))
```
(`.topic_contributes` doesn't exist yet either — `ExtensionBundle` would need a builder method mirroring
`Plugin::builder`'s pattern, once the field lands.) **Flagging for whichever wave/agent owns
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` next** — same "found it, can't touch it, here's
the exact fix" handoff style `w2-open-contribution.md` used for its own out-of-ownership `PluginManifest`
literal sites.

## Step A — Schema self-registration

Playbook has exactly one app under `🎛️apps/📖️playbook/`: `playbook-play` (id `s.playbook.playbook` in
framework's closed catalog, `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:715-731`). Read that block plus the
parked `catalog-integration` call site at line 1490
(`semio_s_plugin_playbook::apps::playbook::config::schema::register_app_schema();`) to get the exact expected
fn path, and the module wiring in `📦️glue.rs:381-388` to confirm `apps::playbook::config::schema` is the real
module path for `🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs`.

### Added `register_app_schema()`
File: `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs` — added a
`//#region 🔖️Register` block, transplanting the same `AppSchemaDescriptor{ id: "s.playbook.playbook", config, presence }`
construction framework's closed catalog had, with `include_str!` paths now relative to this file's own location
(config facet leaves are same-directory siblings; presence facet leaves are `../../👥️presence/🧬️schema/*`,
two levels up to `🎛️apps/📖️playbook/` then into the sibling `👥️presence/🧬️schema/` dir). Calls
`schema::register_app_schema_descriptor(...)` — `schema` here is the crate-root `extern crate
semio_framework_schema as schema;` alias already established in this plugin's `📦️glue.rs:25` and already used
the same way by the artifact-schema self-registration precedent at
`🗿️artifacts/📖️playbook/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:199` (`::schema::register_artifact_schema_descriptor(...)`).

### Wired the call into the real setup path
File: `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — added
`crate::apps::playbook::config::schema::register_app_schema();` inside `register()`, alongside the existing
`register_artifact_schema();` call. `register()` is this plugin's real `.setup(...)` entry point (wired via
`🎛️apps/📖️playbook/🦀️component.rs:175` → `🦀️component.rs:10` `Plugin::builder("playbook-play")...setup(...)`),
**not** the parked `#[cfg(feature = "catalog-integration")]` block in framework's schema module (that block
stays untouched — framework's closed catalog fan-out list is not mine to edit — but the fn path now matches
what it already expects for playbook).

### Procedural extension — Step A skipped, explicit
`🧩️extensions/🌀️procedural` has one `App` (`ModuleApp`, id `playbook-module-procedural`) but its
`type Config = semio_framework_plugin::NoConfig` — the generic framework no-op config type, not a
plugin-specific schema struct. Confirmed no entry for `playbook-module-procedural`/`playbook.module` exists
anywhere in framework's closed catalog (`grep` empty). Nothing to self-register here — skipped, not silently
missed.

## Verification
Ran `cargo check -p semio-s-plugin-playbook` and `cargo check -p semio-s-plugin-playbook-procedural` (exact
crate names from each subtree's `Cargo.toml` `[package] name`), plus `--tests` variants for both.

- **`semio-s-plugin-playbook-procedural`**: clean — 4 warnings (all pre-existing: unused `extern crate flow`/`vcs`,
  unused `ViewModel` import, dead `mesh_from_tessellation_json`), 0 errors, `--tests` also 0 errors. I made no
  code changes to this crate (Step B blocked, Step A n/a) — this run just confirms I didn't need to and nothing
  regressed.
- **`semio-s-plugin-playbook`**: 3 pre-existing `E0308` errors, **unrelated to my changes** — all three are in
  `🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📤️export,📥️import}/.../🔣️json/🔖️rfc8259/.../🦀️component.rs`
  (`JsonValue` vs `serde_json::Value` mismatch on `JsonSnapshot.value` / `STDIO_JSON_DOCUMENT_SCHEMA` codec
  glue), files I never touched. `git status` confirms these files carry no local modification — this is
  concurrent churn already present in the shared tree before I started (not the "document" pattern called out
  in my briefing, but the same category: pre-existing breakage from another in-progress session, not mine to
  fix). Confirmed by `--tests` too (same 3 errors, no new ones). My two edited files
  (`🎚️config/🧬️schema/🦀️component.rs`, `⚙️engine/🦀️component.rs`) introduce zero new diagnostics of any kind.

## Files touched
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added the
  `register_app_schema()` call inside `register()`)

No other files edited. Nothing in `🧩️extensions/🌀️procedural` was edited (Step A n/a, Step B blocked — see
above). No framework or OS-product files touched.
