# W3 — CAD plugin subtree (📐️cad + extensions: spatial-shape, aec-building, aec-building-energy, aec-building-structure)

## Scope
`✏️s/🔌️plugins/📐️cad/` including `🧩️extensions/{🏛️aec-building-structure,🏢️aec-building,📐️spatial-shape,🔥️aec-building-energy}`.

## Pre-read
- `📓️w2-schema-api.md` (open app-schema registry API, all 6 fns confirmed `pub` already).
- `📓️w2-open-contribution.md` (open `TopicContribution` type + `topic_contributions: Vec<TopicContribution>` field added to `PluginManifest` only — NOT to `ExtensionManifest`, see Step B finding below).
- Confirmed via repo-wide grep: no plugin crate anywhere yet actually calls `TopicContribution::new(...)` — this wave is the first producer attempt anywhere.
- Found an existing Step-A template already landed by a parallel wave: `✏️s/🔌️plugins/🌀️procedural/🎛️apps/{🧊️3d,◻2d}/🎚️config/🧬️schema/🦀️component.rs` (`register_app_schema()` fn using `::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor{...})`), called from `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`'s setup path. Used as the exact template for cad's app.

## Step B — Open contribution producer conversion (EXTRA TASK)

`grep -rn "Contribution::" ✏️s/🔌️plugins/📐️cad/` found 4 producer sites, all inside `🧩️extensions/*`, all constructing `Contribution::CadComputer { .. }` via `ExtensionBundle::new(..).contributes(Contribution::CadComputer{..})`:
- `🧩️extensions/🏢️aec-building/🦀️component.rs:87`
- `🧩️extensions/📐️spatial-shape/🦀️component.rs:37`
- `🧩️extensions/🏛️aec-building-structure/🦀️component.rs:86`
- `🧩️extensions/🔥️aec-building-energy/🦀️component.rs:71`
(plus matching test-only consumer/pattern-match sites in the same 4 files, and one real consumer at `🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:704` — consumers out of scope for Step B per instructions.)

### BLOCKED — reported, not worked around
All 4 producers use `semio_framework_plugin::ExtensionBundle` / `ExtensionManifest`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6906-6966`), **not** `PluginManifest`.
Checked the struct: `ExtensionManifest` has fields `extension_id, label, version, extends, capabilities,
contributions: Vec<Contribution>` — **no `topic_contributions` field at all**, and `ExtensionBundle` has
no `.topic_contributes(...)` builder method. Only `PluginManifest` (a sibling, different struct) got the
additive `topic_contributions: Vec<TopicContribution>` field in the prior `w2-open-contribution` wave.

Grepped the whole repo (`grep -rn "topic_contributions\|TopicContribution"`) — confirmed this gap is real
and not something I'm missing: every existing `topic_contributions: vec![]` site is a `PluginManifest`
literal; zero `ExtensionManifest` sites exist because the field isn't declared there.

Adding the missing field + builder method would require editing
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, which is the **framework/products/os
tree — outside my assigned scope** (`✏️s/🔌️plugins/📐️cad/` only). Per the operational rules I must not
touch files outside my assigned plugin subtree, so I did **not** make this change.

**Result: Step B is blocked for all 4 cad extension producers, not completed.** This needs a follow-up
wave that either (a) owns the framework plugin-host file and adds `topic_contributions` +
`.topic_contributes()` to `ExtensionManifest`/`ExtensionBundle` to mirror `PluginManifest`, or (b)
decides extensions intentionally stay on the closed `Contribution` enum only. Flagging for the
orchestrator rather than guessing which.

No files edited for Step B.

## Step A — Schema self-registration

Apps in scope: exactly one, `s.cad.cad` (`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/`). Confirmed via
`grep -n "cad" 🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` — only one `AppSchemaDescriptor` block
(lines 681-697) with id `"s.cad.cad"`, config+presence facets both pointing at
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/{🎚️config,👥️presence}/🧬️schema/`. Parked catalog-integration region
(line 1488) already names the expected fn path: `semio_s_plugin_cad::apps::cad::config::schema::register_app_schema()`.

Extensions have no apps of their own (all 4 are `extends("cad")` pure contribution/library crates,
`semio_plugin_extension!`-style, no `App`/`register_document_app` calls) — **Step A skipped for all 4
extensions**, correctly inapplicable per recipe point 4.

### Change made
Added to `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs` (end of file), mirroring the
procedural2d/3d template exactly, transplanting the descriptor construction from framework's closed
catalog (lines 681-697) with `include_str!` paths made relative to this file's own location:

```rust
//region 📎 App-schema self-registration
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.cad.cad",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    });
}
//endregion 📎 App-schema self-registration
```

Called it from the app's own init/setup path — cad's established pattern is `Plugin::builder("cad")
.setup(crate::artifacts::cad::engine::register)` in `✏️s/🔌️plugins/📐️cad/🦀️component.rs`, and that
`register()` fn (`🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:719`) already calls
`register_artifact_schema()` for the artifact side. Added the app-schema call alongside it:

```rust
pub fn register() {
    crate::artifacts::cad::composer::register();

    register_artifact_schema();
    crate::apps::cad::config::schema::register_app_schema();
    register_pilot_languages();
    ...
}
```

## Verification

Crate names resolved from each `Cargo.toml` `[package] name`:
`semio-s-plugin-cad`, `semio-s-plugin-cad-aec-building-structure`, `semio-s-plugin-cad-aec-building`,
`semio-s-plugin-cad-spatial-shape`, `semio-s-plugin-cad-aec-building-energy`.

- `cargo check -p semio-s-plugin-cad-aec-building-structure` — **clean**, 0 errors (only pre-existing
  unrelated warnings from `semio-framework-plugin`'s `VcsArtifactApp` destructuring, not touched by me).
- `cargo check -p semio-s-plugin-cad-aec-building` — **clean**, 0 errors, same pre-existing warnings only.
- `cargo check -p semio-s-plugin-cad-spatial-shape` — **clean**, 0 errors, same pre-existing warnings only.
- `cargo check -p semio-s-plugin-cad-aec-building-energy` — **clean**, 0 errors, same pre-existing warnings only.
- `cargo check -p semio-s-plugin-cad` — **BLOCKED by unrelated concurrent churn, not my bug**:
  ```
  error: couldn't read `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/./././../../🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
     --> ✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs:567:13
  error: could not compile `semio-s-plugin-cad` (lib) due to 1 previous error
  ```
  `📦️glue.rs:566-567` wires `pub mod document;` at `#[path = "../../🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs"]`
  but that file does not exist on disk (`ls` confirms the `📌️panels/` dir has no `📄️document/` child;
  only `🛍️catalogue` and `🔍️inspection` exist alongside it). This is exactly the flagged concurrent
  "document" concept refactor from the briefing (threading a document concept through
  plugins/AppDefinition/OsAppRegistration) — a module named `document` under an app's `panels/` tree that
  another session is mid-adding. Not caused by either of my two edits (schema `component.rs` — new fn at
  end of file; artifact engine `component.rs` — one added call line inside `register()`), neither of
  which touches `📌️panels/` or `glue.rs`. Confirmed by re-running `cargo check -p semio-s-plugin-cad`
  filtered to `^error` lines only — exactly this one error, nothing else. Per instructions, did not fix
  it, noting and moving on.

## Files touched
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`, Step A)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added one call line to
  `register()` invoking the above, Step A)

No other files edited. Step B not completed (blocked, see above — needs framework-side
`ExtensionManifest`/`ExtensionBundle` change outside my assigned scope).
