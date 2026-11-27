# 🏷️ Bilingual Plugin Manifest Label — Status Report

Source plan: `2026/11/22/GIS-PLUGIN-END-TO-END` `conformance-fixes.md` item 5b. That inventory was incomplete: the extension manifest, the `.sxt` package manifest, the native extension store record, the stdio composition owners and the registry descriptor-verification schema all carry the same label and had to change too (a `LocalizedLabel::data(string)` bridge would have been a compat adapter).

## Done (source edits, all applied)

### Schema and projections
- `🧰️framework/🔨️modules/🧬️schema/📽️projection/🎟️rows-h-t/🦀️.rs`: `PluginManifest` v2 → v3, `label: LocalizedLabel` (real type, not `unknown`); renderer header imports `LocalizedLabel` from `../🎚️ui-axes/🟦️.ts` (`📽️projection/🦀️.rs`).
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`: the same text by hand, so `exports_typescript_bindings` stays equal (not yet re-run).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json`: `manifest.label` is `$defs/RegistryLocalizedLabel` (native/reuse × en/de, every cell 1..4096 chars).
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/{🔣️.json,🧬️schema/🔣️.json,🟦️.ts}`: owner `label` is `{en, de}`; the generator emits `semio_plugin::LocalizedLabel::native(en, de)`. This was needed because the generator had already overwritten the script's stdio rewrite.

### Types and builders (Rust)
- `PluginManifest.label: LocalizedLabel` (`🛂️manifest/🧾️plugin-manifest/🦀️.rs`).
- `PluginBuilder::label(LocalizedLabel)`, `Plugin::new(id, LocalizedLabel, version)`, `ExtensionBundle::new(id, LocalizedLabel, version)`. None of them has an `Into<String>` overload.
- `ExtensionManifest.label`, `ExtensionPackageManifest.label` (`semio-os-extension`; it gains `semio-ui-locale` and `semio-framework-value`, and the edge law against `semio-framework` still holds) and `NativeExtensionRecord.label` (it drops `Eq`) are now `LocalizedLabel`.
- Re-exports: `semio_plugin_builder::LocalizedLabel`, `semio_plugin_plugin_program::LocalizedLabel`, `semio_plugin_extension::extension_runtime_lift::LocalizedLabel` and `semio_os_extension::LocalizedLabel`. `semio_plugin::LocalizedLabel` already existed.
- Retirement: `MetadataOwner for LocalizedLabel`, and a new `LocalizedLabel::owned_bytes()` (`🖱️ui/🌐️locale/🏷️label/🦀️.rs`).
- Cargo: `semio-ui-locale` added to builder, program (moved from dev-deps) and extension. The lock file has not been refreshed.

### Call sites (scripts in this ticket)
- `rewrite-call-sites.py` + `labels.py`: 222 `PluginBuilder…label(…)` and 6 `Plugin::new` sites in 170 files. Each one is `LocalizedLabel::native(en, de)` with a German label: proper nouns and acronyms stay identical, descriptive names are translated.
- `rewrite-extension-sites.py`: 46 `ExtensionBundle::new` sites in 33 files. The `EXTENSION_LABEL` constants keep their English and get the German inline.
- Fixed by hand: the demonstrator (`["Designing with Existing Stock", "Entwerfen mit Bestand"]`), the declaration-registration laws (`native(plugin_id, plugin_id)`) and the oversized retirement test (`LocalizedLabel::data`).

### Readers
- Rust:
  - The MCP catalog resolves the label per request `(terminology, locale)`.
  - Both wgpu shell readers resolve with `prefs.active_terminology()/active_locale()`. `default_app_rows` now takes both.
  - The assembly-failed stub carries `LocalizedLabel::data(fault.message)`; the emitter refusal prints the Native/En cell.
  - Tests updated: freshness, part-7 builder, playbook procedural, extension unit, ProgramBridge store, host install laws.
- TypeScript:
  - New `parseLocalizedLabelV1` (`@semio-tech/framework-manifest/localized-labels`, new export). It is tested against the shared `🖱️ui/🌐️locale/🧪️tests/🧫️fixtures/🏷️localized-label` rows and refusals, which the AJV oracle already checks.
  - Store installation types and parsing, packaging, wgpu host-io store record and browser-bundle descriptor are updated.
  - ShellHost marketplace (now gets `uiTerminology`), artifact-opening catalog and open-entries resolve the label through `resolveManifestLabel`.
  - The registry catalog-complete test and the hub trusted rotation (a bilingual label with a nonce) are updated, along with its source-shape check.
- `🧪️installation.json`: the fixture label is now the matrix.

## Not done / not verified
1. **Nothing has compiled yet.** `repo cargo check -p semio-plugin --tests` ran four times at 4, 2, 1 and 1 jobs. Each time the host memory warden killed cargo while it was rebuilding shared `🌱️value`/`📡️replication`/`🧵️job` units, before it reached any crate changed here. fseventsd held about 3 GB and other agents' bun processes held 1+ GB. Expect compile errors to fix, most likely:
   - `Terminology`/`Locale` not in scope in the wgpu `🧲️plugins` file;
   - crates lacking `semio_plugin` where the script chose the `semio_plugin::LocalizedLabel` qualifier;
   - `size_of` in extension retirement;
   - the `Cargo.lock` refresh.
   - Other workspaces also need checking: `semio-framework-manifest-plugin`, `semio-os-extension`, the os host, the hub compositions and the `✏️s` plugin crates.
2. TypeScript typecheck and tests have not run.
3. **No generators have run:**
   - `@semio-tech/os-plugin-describe-rs:describe` (per-composition `🧩️component/{🔣️.json,🛂️.descriptor.semio}`, which builds every wasm component);
   - `@semio-tech/framework:generate` (it should reproduce the hand-written TS projection);
   - the stdio composition preparation (it re-emits the stdio `.label(...)` lines from the new `{en, de}` owners);
   - `os-hub:trusted-catalog-bootstrap` (signed catalogs), then the catalog verification and preflight.
4. Out of scope, left as it was: `flow_extension_topic_contribution(…, EXTENSION_LABEL, …)` topic payload labels are still English-only strings. That is a follow-up.
