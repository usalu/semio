# W3 Batch D — reasoning-mindmap, remodel, s/space, shooting

Scope: apply Step A (schema self-registration) + Step B (open contribution producer conversion) to four plugins. No `🧩️extensions/` subdirectory found for any of them.

## Plugin resolution

| requested name | resolved directory | crate name |
|---|---|---|
| reasoning-mindmap | `✏️s/🔌️plugins/💡️reasoning` | `semio-s-plugin-reasoning-mindmap` |
| remodel | `✏️s/🔌️plugins/📸️remodel` | `semio-s-plugin-remodel` |
| s / space | `✏️s/🔌️plugins/🪐️space` | `semio-s-plugin-space` |
| shooting | `✏️s/🔌️plugins/🎥️shooting` | `semio-s-plugin-shooting` |

---

## 💡️reasoning (reasoning-mindmap)

**Step A**: one app, `s.reasoning.wires`. Transplanted the descriptor from framework schema's closed catalog (component.rs:630-646) into
`✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎚️config/🧬️schema/🦀️component.rs`, new `pub fn register_app_schema()` in a `//region 📎 App-schema self-registration` block, calling `::schema::register_app_schema_descriptor(...)` (crate root `📦️glue.rs` already does `extern crate semio_framework_schema as schema;`).
Wired the call from `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` (the function the plugin root's `.setup(crate::artifacts::wires::engine::register)` invokes) — added `crate::apps::wires::config::schema::register_app_schema();` alongside the existing `register_artifact_schema()`/`register_pilot_languages()` calls. This matches the exact expected call-site path already referenced by the framework's parked `catalog-integration` test block (`semio_s_plugin_reasoning_mindmap::apps::wires::config::schema::register_app_schema();`).

**Step B**: `grep -rn "Contribution::" ✏️s/🔌️plugins/💡️reasoning/` — no matches. No producer sites. Skipped, nothing to convert.

**cargo check -p semio-s-plugin-reasoning-mindmap**: BLOCKED, not by my change.
```
error: couldn't read `.../💡️reasoning/📦️packages/🦀️rust/./././../../🎛️apps/🔌️wires/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> .../💡️reasoning/📦️packages/🦀️rust/📦️glue.rs:429:13
```
`glue.rs` (untouched by me, matches HEAD per `git status`) declares `pub mod document;` under `📌️panels`, but the directory only has `📄️artifact`, `🔍️inspection`, `🛍️catalogue` — no `📄️document`. This is the repo-wide "document" concept refactor the task brief warned about (`pub mod document;` under `📌️panels` is present in glue.rs for ~26 plugins repo-wide, not just mine — grepped and confirmed). Not my bug; did not touch it.

---

## 📸️remodel

**Step A**: one app, `s.remodel.remodel.remodelworldcamera` (yes — that's the literal id used in the framework's closed-catalog entry at component.rs:749-765, kept verbatim even though it reads oddly for an app id). Transplanted into
`✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎚️config/🧬️schema/🦀️component.rs::register_app_schema()`.
Wired from `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` — added `crate::apps::remodel::config::schema::register_app_schema();`.

**Step B**: `grep -rn "Contribution::" ✏️s/🔌️plugins/📸️remodel/` — no matches. Skipped.

**cargo check -p semio-s-plugin-remodel**: BLOCKED, same class of pre-existing issue, not by my change.
```
error: couldn't read `.../📸️remodel/📦️packages/🦀️rust/./././../../🎛️apps/📸️remodel/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> .../📸️remodel/📦️packages/🦀️rust/📦️glue.rs:687:13
```

---

## 🪐️space (s / pluginId "s")

Two apps: `s.space.home` and `s.space.space`. No dedicated `.setup(...)` hook and no `🗿️artifacts/🪐️space` engine `register()` function exists for the `space` app (only `🗿️artifacts/🏠️home` has an artifact tree); the plugin's own `register_s_exports()` in `📦️glue.rs` (called from `plugin()` in the plugin-root `🦀️component.rs`) is the established hook for plugin-wide registration side effects (it already registers the document codecs for both apps), so I added both calls there.

**Step A**:
- `s.space.home` — transplanted into `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🎚️config/🧬️schema/🦀️component.rs::register_app_schema()` (framework catalog component.rs:970-986).
- `s.space.space` — transplanted into `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎚️config/🧬️schema/🦀️component.rs::register_app_schema()` (framework catalog component.rs:987-1003).
- Wired both from `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs::register_s_exports()` — added `apps::home::config::schema::register_app_schema();` and `apps::space::config::schema::register_app_schema();` ahead of the existing codec registrations.

**Step B**: `grep -rn "Contribution::" ✏️s/🔌️plugins/🪐️space/` — no matches. Skipped.

**cargo check -p semio-s-plugin-space**: BLOCKED, not by my change — this one hits the *other* documented concurrent hazard directly (the in-progress "document" field threading through `AppDefinition`/`OsAppRegistration`), in framework/product/os, a dependency of this plugin:
```
error[E0560]: struct `OsAppRegistration` has no field named `document`
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:4387:13
error[E0609]: no field `document` on type `&AppDefinition`
   --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:4387:27
```
Exactly the pattern the task brief pre-flagged ("another session is actively mid-refactor threading a 'document' concept through several plugins/AppDefinition/OsAppRegistration"). Not touched.

---

## 🎥️shooting

**Step A**: one app, `s.shooting.shooting`. Transplanted into
`✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎚️config/🧬️schema/🦀️component.rs::register_app_schema()` (framework catalog component.rs:511-527).
Wired from `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` — added `crate::apps::shooting::config::schema::register_app_schema();`.

**Step B**: `grep -rn "Contribution::" ✏️s/🔌️plugins/🎥️shooting/` — no matches. Skipped.

**cargo check -p semio-s-plugin-shooting**: BLOCKED, same pre-existing panels/document issue as reasoning-mindmap/remodel:
```
error: couldn't read `.../🎥️shooting/📦️packages/🦀️rust/./././../../🎛️apps/🎥️shooting/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> .../🎥️shooting/📦️packages/🦀️rust/📦️glue.rs:613:13
```
(Note: `semio-framework-os` itself compiled clean in this run for the shooting check — the E0560/E0609 `document`-field churn seen under `space` wasn't present at this moment, consistent with it being a live in-progress concurrent edit rather than a stable failure.)

---

## Summary

- Step A: 5 apps registered across 4 plugins (wires, remodel, space.home, space.space, shooting). All transplanted descriptors match the framework closed-catalog entries verbatim (including the odd `remodelworldcamera`-suffixed id for remodel).
- Step B: no `Contribution::` producer sites in any of the four plugins — no-op for all, confirmed by direct grep of each plugin subtree.
- `cargo check` could not get past pre-existing/concurrent-churn errors for any of the four crates (the repo-wide `📌️panels::document` module-path mismatch affecting ~26 plugins, and — for `space` specifically — the live `AppDefinition`/`OsAppRegistration.document` field refactor called out in the task brief). Neither issue originates from my edits; `git status` before I started editing showed only my own two touched files as modified for `💡️reasoning`, and the `document` panel mismatch is baked into the currently-committed `glue.rs` for every affected plugin, not something I introduced.
- My additions themselves are structurally identical to the already-landed sibling examples (`🌊️flow`'s `apps::flow::config::schema::register_app_schema()`), including the exact fully-qualified call-site paths the framework's parked `catalog-integration` test block expects (verified by grep against `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` lines 1467-1506) — but this could not be confirmed to compile end-to-end due to the blockers above.

## Files touched

- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call in `register()`)
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call in `register()`)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` (added both calls in `register_s_exports()`)
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added call in `register()`)
