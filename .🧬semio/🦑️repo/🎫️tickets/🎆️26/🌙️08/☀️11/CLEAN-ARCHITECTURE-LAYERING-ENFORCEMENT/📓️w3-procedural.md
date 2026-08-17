# W3 — Procedural plugin (`✏️s/🔌️plugins/🌀️procedural/`)

Crate: `semio-s-plugin-procedural` (`📦️packages/🦀️rust/Cargo.toml`)

Scope note: left the flow-extension dependency wiring / `register_linked_flow_extension_installer`
/ `ensure_linked_flow_extensions` call in `🦀️component.rs` and the 7 flow-extension crate deps in
`Cargo.toml` untouched, per instructions (audit finding C2, parked for a later wave).

## Step A — App-schema self-registration

Two apps in this plugin: `procedural2d` (dir `🎛️apps/◻2d`) and `procedural3d` (dir `🎛️apps/🧊️3d`),
app-schema ids `s.procedural.2d` / `s.procedural.3d`. Found their descriptor blocks in
`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` (lines ~392–425, inside
`register_all_app_schema_descriptors()`) and the exact expected fn path in the parked
`catalog-integration` region (lines 1471–1472):
`semio_s_plugin_procedural::apps::procedural2d::config::schema::register_app_schema()` /
`...::apps::procedural3d::config::schema::register_app_schema()`.

`📦️glue.rs` already wires `pub mod schema;` at exactly those module paths, pointing at each app's
`🎚️config/🧬️schema/🦀️component.rs` — so no glue.rs changes were needed, only additions inside those
two files.

Added a `//region 📎 App-schema self-registration` / `//endregion` block to each:
- `🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`

Each defines `pub fn register_app_schema()` that transplants the same `AppSchemaDescriptor`
construction from the framework's closed catalog (same `id`, same five-leaf `config`/`presence`
`FacetLeaves`), but with `include_str!` paths now relative to the app's own file location (config
leaves are now same-directory siblings; presence leaves are `../../👥️presence/🧬️schema/...`), calling
`::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor { .. })`. Used the `::schema`
extern-prelude alias already established by this crate's `📦️glue.rs`
(`extern crate semio_framework_schema as schema;`) — the same alias the sibling artifact-schema
registration (`🗿️artifacts/.../⚙️engine/🦀️component.rs::register_artifact_schema()`, pre-existing) uses
via `::schema::register_artifact_schema_descriptor(...)`, so this matches established convention.

Wired both new fns into the plugin's existing `register_exports()` in the plugin-root
`🦀️component.rs` (the fn already called via `.setup(register_exports)` in `plugin()`), alongside the
existing `engine::register()` calls:
```rust
fn register_exports() {
    crate::artifacts::procedural2d::engine::register();
    crate::artifacts::procedural3d::engine::register();
    crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();
    crate::apps::procedural2d::config::schema::register_app_schema();
    crate::apps::procedural3d::config::schema::register_app_schema();
}
```

Framework's closed catalog (`register_all_app_schema_descriptors()`, its ~390 `include_str!` roster,
and the parked `catalog-integration` regions) — not touched, per instructions.

## Step B — Open contribution producer conversion

`grep -rn "Contribution::" ✏️s/🔌️plugins/🌀️procedural/` → exactly one hit:
`🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:50`:
```rust
if let Contribution::FlowExtension { manifest_json, .. } = entry.contribution {
```
This is a **consumer** (pattern-match on a `Contribution` value read from `entry.contribution`, a
deserialized `ProgramContributionEntry`), not a producer — procedural never constructs a
`Contribution::<Variant>(...)` value anywhere. Consistent with the task's own prediction ("likely
none, since it mainly consumes flow extensions rather than producing contributions itself"),
confirmed by grep. **Step B skipped for this plugin — no producer sites exist.** Did not touch the
consumer site (out of scope per instructions — consumers are a later wave's job).

## Verification

`cargo check -p semio-s-plugin-procedural`:

```
error: couldn't read `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/./././../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs:901:13
    |
901 |             pub mod document;
```

**Blocked — not my bug, did not fix, per explicit briefing instruction to leave "document"
module/field errors alone.** `📦️glue.rs` (committed, unmodified by me — last touched by commit
`5b22e9f4ab`, unrelated to this session) declares `pub mod document;` for both apps' `📌️panels`
region (lines ~901 and ~1002), pointing at `📌️panels/📄️document/🦀️component.rs` in each app, but on
disk both `🎛️apps/◻2d/📌️panels/` and `🎛️apps/🧊️3d/📌️panels/` currently only contain `📄️artifact/`,
`🔍️inspection/`, `🛍️catalogue/` — no `📄️document/` directory exists yet. This is the "document" concept
being threaded through plugins by another concurrent session mentioned in my briefing — I did not
create, rename, or touch anything under `📌️panels/` or `📦️glue.rs`. Also observed already-staged
(index, not working-tree) changes to this plugin's own `Cargo.toml` and
`🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` from a source other than this
session, corroborating live concurrent churn in this exact plugin directory.

**Could not get a clean `cargo check` for `semio-s-plugin-procedural`** — blocked entirely by the
above pre-existing/concurrent `📄️document` panel-module error, which occurs before the compiler ever
reaches my Step A additions (a `couldn't read` file-level `include`/`mod`-path error, not a type
error, so it gives no signal either way on whether my two `register_app_schema()` additions
themselves are correct). Could not independently re-verify my additions compile in isolation without
either fixing (out of scope) or reverting the panels wiring; visual re-check of both new
`register_app_schema()` blocks against the framework's `AppSchemaDescriptor`/`FacetLeaves` field
shapes and the sibling artifact-schema registration's `::schema::` alias usage shows no divergence.

## Files touched

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`

No other files edited. Did not touch `Cargo.toml` flow-extension deps, `📦️glue.rs`, or anything under
`📌️panels/`.
