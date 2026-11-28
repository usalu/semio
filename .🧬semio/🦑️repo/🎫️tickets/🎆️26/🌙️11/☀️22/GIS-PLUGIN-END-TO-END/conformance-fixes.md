# GIS conformance fixes

Abbreviations as in `conformance-audit.md` (PL = `✏️s/🔌️plugins/🌍️gis`, GM/GT/HUB).

## Items

1. Unknown body fallback localized (en "Unknown body: …", de "Unbekannter Body: …") via `LocalizedLabel::native(..).resolve(view_state.terminology, view_state.locale)`.
   Files: GM `👁️viewer/🦀️.rs`, GM `✏️editor/🧭️play-app-editor/🦀️.rs`, GT `👁️viewer/🦀️.rs`, GT `✏️editor/🦀️.rs`.
2. GT overlay faults localized with the app fault-notice pattern (stable code `gis.terrain.overlay.<name>`, English message in the Fault, en/de notice table registered in `Gis3dPlayApp::fault_notices`), as contours/geojson already do. 23 faults: all 19 in the overlays commands plus the 4 in `command_from_action` (missing id, invalid label, missing vertices, missing/invalid longitude/latitude/elevation).
   Files: GT `✏️editor/🎮️commands/🗺️overlays/🦀️.rs` (new `OverlayFault`, `fault_notices`), GT `✏️editor/🦀️.rs`.
   Left as is (developer-only protocol faults, not UI text): `gis3d-command-payload-too-large` code and the "not a framework-reserved action" message.
3. Removed `eprintln!("[TRACE] …")` in GM `…/🔬️window/🦀️.rs` (the following `outcome.expect` already reports the error).
4. `🗺️tiles/🌍️gis-policy` no longer uses serde_json at runtime: overlays are built as `DslValue` and encoded with `semio_framework_pack_json::to_json_string`. The policy descriptor (serde-only framework type) is now encoded by a new framework function `map_policy_descriptor_json` (next to `map_lod_scale_json`). `gis_map_overlays_value` now returns `DslValue`; consumers updated.
   Cargo.toml: `serde`/`serde_json` removed from `[dependencies]` (kept in dev-dependencies for tests), `semio-framework-value` added.
   Files: `🗺️tiles/🌍️gis-policy/🦀️.rs`, its `📦️packages/🦀️rust/Cargo.toml`, its `🧪️tests/🔬️unit/🦀️.rs` (overlays test uses DslValue), GM `✏️editor/🗺️maphost/🦀️.rs`, framework `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🎛️map-policy/🦀️.rs` and `…/🗺️tiled-map/🦀️.rs` (re-export).
   Not done: `maphost` still parses the camera with serde_json (out of scope).
5. HUB label "GIS": NOT changed. The plugin manifest label is a plain `String` (`PluginBuilder::label`, `Plugin::new`) for every plugin; there is no bilingual API for it, and adding one is a framework-wide manifest change. "GIS" is identical in en/de. Mapzen attribution: kept as a proper noun (identical in both languages); the scene payload is locale-free, so a docstring on `TerrainAttribution` now states this (GT `🧬️schema/🧭️terrain-descriptor/🎞️scene/🦀️.rs`).
6. Language-agnostic tests (shared JSON fixture + command JSON schema + Rust test + TS test with Ajv and a further third-party/platform library):
   - GT `📂️local-file`: `🔣️.json` schema, `🧪️tests/🧫️fixtures/🔣️.json`, `🧪️tests/🦀️.rs`, `🧪️tests/🟦️.ts` (Ajv). Module hooked in `🦀️.rs`.
   - GM `🌐️shell`: `🔣️.json`, `🧪️tests/🧫️fixtures/🔣️.json`, `🧪️tests/🦀️.rs`, `🧪️tests/🟦️.ts` (Ajv + WHATWG URL). Module hooked in `🦀️.rs`.
   - GT `🏔️exaggeration`: `🔣️.json`, `🧪️tests/🧫️fixtures/🔣️.json`, `🧪️tests/🦀️.rs`, `🧪️tests/🟦️.ts` (Ajv + mathjs). Module hooked in `🦀️.rs`.

Not touched: "serde behind pack interface in ~25 Cargo.toml" (broad item; only the gis-policy manifest was changed).

## Verification

Nothing is verified by a green build yet. Honest status from logs in `🗑️generated/conformance-*.log`:

- `semio-framework-surface --tests`: lib compiled; the only error is in another lane's test `tiled_map_mercator_oracle` (unresolved `serde_json`, not my change). Log `conformance-surface.log` (deleted when I re-ran with new args).
- GM workspace (`gis2d`, `gismap`, `gismap-viewer`): cargo cannot refresh the stale `Cargo.lock` because of a cyclic package dependency in the stdio pdf plugin (`semio-s-artifact-stdio-pdf-tests-2` ↔ `page-editor`), edited by another lane at 15:51. Logs `conformance-gis2d.log`, `-gismap.log`, `-gismap-viewer.log`. The gismap lock needs a refresh anyway because of my Cargo.toml change.
- GT editor: the run was stopped by the host-memory admission ("a unit over its ceiling"), not by a compiler error (`conformance-gt-editor.log`). A batched retry (`-p …-gisterrain-editor -p …-gisterrain`) was queued behind the other lanes' compile.lock (log `conformance-gt.log`); see the status line below.
- The TS tests (Ajv/mathjs/URL) and Rust tests were not run.
- Per the coordinator, `repo test` needs the task first and flags after; not yet used.

Need to re-run once the pdf cycle is fixed: `check --tests` for gis2d, gismap, gismap-viewer; then tests `shell`, `the_overlays_value_carries_every_document_feature`, GT `exaggeration`, `local_file`; and `repo test-file` for the three `🟦️.ts` files.

Helper scripts in the ticket: `conformance-run.sh` (lock + cargo + release), `conformance-all.sh`.

## Follow-up from the coordinator (second message)

### (a) TypeScript tests via `repo test-file <file> --lang=en`: all pass
- GT `🏔️exaggeration/🧪️tests/🟦️.ts`: 3 pass, 0 fail (23 expects).
- GT `📂️local-file/🧪️tests/🟦️.ts`: 2 pass, 0 fail (16 expects).
- GM `🌐️shell/🧪️tests/🟦️.ts`: 3 pass, 0 fail (23 expects).
Logs: `🗑️generated/conformance-ts-*.log`.

### (b) Bilingual plugin manifest label (item 5): NOT applied, inventory and plan only
Applying it now is a repo-wide breaking change on the live replicated worktree: every write reaches 50+ machines within a second, so a half-applied schema/type change breaks everyone's builds, and the compile/test cycles needed to finish it are blocked by the compile lock and the pdf cycle. I stopped before the first edit; nothing for (b) is changed.

Inventory:
- Type: `PluginManifest.label: String` in `🧰️framework/🔨️modules/🛂️manifest/🧾️plugin-manifest/🦀️.rs`; schema `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` (label at lines ~60/73/106); also generated TS `PackageDescriptor`/`PluginManifest` (`🧬️schema/📽️projection/🎟️rows-h-t`).
- Builder: `PluginBuilder::label(impl Into<String>)` and `Plugin::new(id, label: impl Into<String>, version)` in `🔌️plugin/🏗️builder`.
- Call sites: about 254 `.label("…")` in Rust (stdio 80+14, os framework 35, norm 23+5, block 3, process 3, flow 3, space 2, native-openable-provider 2, hub compositions) plus non-literal ones (`format!`, variables); mixed with non-plugin `.label(` builders, so the rewrite must be anchored on `PluginBuilder::…::new("id")`.
- Readers of `manifest.label` (about 21 files): `🔌️plugin/🛂️describe/🦀️.rs` (`= extension.label`), `🖨️describe/🛂️descriptor-emission`, `🌉️mcp/🗂️catalog/📚️catalog-compile` (plugin_label in CapabilityOwner), wgpu shell settings/dock persistence (two files), ProgramBridge native-extension-store, ShellHost TS (`settings-and-marketplace-hosts`, `artifact-opening` open-entries and catalog), `🌐️browser-bundle/🛂️descriptor`, `🏪️store/📥️installation`, `📇️registry` catalog-complete test, hub tests (`trusted-stdio-gis-bundle-check`, demonstrator manifest, bootstrap rotation), playbook procedural test.
- Data carrying the wire shape: per-composition `🌎️hub/🧩️compositions/*/🧩️component/🔣️.json`, trusted-catalog and package-descriptor fixtures, build-graph fixtures, and signed/hashed hub catalog bundles (changing the label shape changes content hashes and signatures; regeneration is required, not a text edit).

Plan (ordered, each step compile-checked under the lock, one batched invocation per workspace, 120 s pause between lock takes):
1. Schema first: change `label` in the manifest schema to the existing `LocalizedLabel` definition; regenerate the Rust and TS projections.
2. `PluginManifest.label: LocalizedLabel`; `Plugin::new` and `PluginBuilder::label` take `LocalizedLabel` only (no `Into<String>` overload).
3. Rewrite call sites by script anchored on `PluginBuilder`, to `LocalizedLabel::native("X", "X")` (proper nouns identical; real translations for descriptive names such as "Stdio Fixture"), then fix the non-literal sites by hand.
4. Readers resolve with the active locale and terminology (shells already hold both); describe/`extension.label` and the MCP catalog resolve per request locale.
5. Regenerate component descriptors, fixtures and signed catalogs through the repo tooling; update tests asserting `label.as_str()`.
Needs an owner and a freeze of the other lanes touching `🔌️plugin`, `🛂️manifest` and `🌎️hub` while steps 1-5 land.

### Status after the coordinator's third message
- Item 5b is split off by the coordinator; left out here.
- `pdf-cargo-cycle-fix.md` has not appeared yet, and `compile.lock` has been held by `gismap-lane` for over 45 minutes (since 16:25). My batched GT check (`conformance-run.sh gt check … -p …-gisterrain-editor -p …-gisterrain --tests`, process queued since about 15:55) is still waiting for the lock and will write `🗑️generated/conformance-gt.log`.
- Still to run once the cycle is fixed and the lock frees: GM `check --tests` (gis2d, gismap, gismap-viewer), then tests `shell`, `the_overlays_value_carries_every_document_feature`, GT `exaggeration`, `local_file`; fix what fails.

## Takeover (gismap2-lane): production code no longer uses test support
- Cause of `E0433 semio_os_store_test_support` in `semio-s-artifact-gis-gismap-config`: `map_window_config_mutation_report_json` (editor map config) and `gis_map_viewer_window_config_mutation_report_json` (viewer map config) called `semio_os_store_test_support::mutation_report_json` from non-test code while that crate is only a dev-dependency.
- Fix: both bridges now build the report with the production API (`ColdMutation::diff`/`inverse`, `protocol::apply_diff_cold`, `semio_framework_pack_json`), the same way `gis_map_mutation_report_json` already does. Files: GM `✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs`, GM `👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs`.
- Checked every other GIS crate: remaining `semio_os_store_test_support` uses are in `🧪️tests` files and `[dev-dependencies]` only (gismap, gisterrain, gisterrain-editor).
- Verification runs: see `gismap-lane-results.md` (runner `gismap2-run.sh`, logs `🗑️generated/gismap2-*.log`).

### Verification attempts with lock6 and the fleet prefix (-j 6), after the pdf fix
- GT (`-p …-gisterrain-editor -p …-gisterrain --tests`): attempt 1 failed with ENOSPC on the compiler image (retry was also full); attempt 2 failed with an infrastructure `EPERM` in the repo tool (`fstatat .reset.json` under `.cache/b/l`). No compiler error in any conformance-fix code was reported. Not green yet.
- GM (`-p …-gis2d -p …-gismap -p …-gismap-gismap --tests`): attempt 1 failed with ENOSPC (syn, io-base64); attempt 2 has been running for about 50 minutes, stuck behind `cargo:build-entry … semio.compiler.host-capacity/v1` (log `🗑️generated/conformance-gm.log`, runner `conformance-run6.sh` under `compile.lock6`). Lock-free cargo errors: none seen.
- Rust tests (`shell`, overlays value, GT `exaggeration`, `local_file`): not run, since they need a successful check first.
- Result so far: PASS for the three TypeScript tests only; Rust check and tests UNVERIFIED (blocked by ENOSPC, EPERM and capacity waits, not by errors in the changed code).
