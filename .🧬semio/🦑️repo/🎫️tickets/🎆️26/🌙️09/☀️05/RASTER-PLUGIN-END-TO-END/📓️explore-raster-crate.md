# Raster Plugin Rust-Side Audit

Scope: `✏️s/🔌️plugins/🖨️raster/` — crate root `📦️packages/🦀️rust/Cargo.toml` + `🦀️.rs`, plugin body `✏️s/🔌️plugins/🖨️raster/🦀️.rs`, `🗿️artifacts/🖨️raster/**` (incl. `🏅️standards`, `🧪️fixtures`), `🎮️commands`.

Read-only audit. No files edited, no builds run. HEAD at audit time: `3a6a9d6bfc` (2026-09-05 22:02:04 +0200).

---

## 1. Crate identity, component metadata, dependencies

- Crate name: `semio-s-plugin-raster` (`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:2`).
- `package.metadata.component.package = "semio:raster"` (Cargo.toml:12).
- `Plugin::<RasterApps>::builder("raster")` … `.package_id("semio:raster")` (`✏️s/🔌️plugins/🖨️raster/🦀️.rs:31,34`).
- **Match: yes.** Builder id `"raster"`, `package_id("semio:raster")`, and Cargo component package `"semio:raster"` are all consistent (compare to the known-bad pattern in memory note "Plugin Id Drift Builder vs Component" — raster does not exhibit that drift).

`[dependencies]` (Cargo.toml:26-39), all unconditional — **none are `target_arch`-gated**; there is no `[target.'cfg(...)'.dependencies]` section anywhere in this Cargo.toml:
- `semio-framework-value-derive`, `semio-s-plugin-stdio` (features `["full-artifact-catalog"]`, `default-features = false`), `semio-framework-os-kernel`, `semio-framework-os`, `semio-framework`, `semio-framework-plugin` (feature `component-guest`), `semio-framework-dispatch-macros`, `semio-framework-schema`, `semio-framework-job` (workspace), `framework_hash` (= `semio-framework-hash`), `base64_codec` (= `semio-framework-io-base64`).
- `[dev-dependencies]` (Cargo.toml:41-44): `serde`, `serde_json`, `semio-framework-async-macros`. `serde`/`serde_json` are dev-only; the one production `use serde::{Deserialize, Serialize}` in the tree (`🧬️schema/🦀️.rs:6`) is behind `#[cfg_attr(test, derive(Serialize, Deserialize))]` (line 11) — no production serde leakage.
- `[lib] crate-type = ["cdylib", "rlib"]` — both native (rlib, used by tests/host-native paths) and wasm component (cdylib) builds share the identical dependency set; nothing here special-cases wasm32.

---

## 2. Module tree — `#[path]` / `include_str!` resolution

Scripted check (Python, walked every `.rs` file under the plugin root, resolved each `#[path = "..."]` relative to its containing file's directory):

- **Total `#[path]` attributes: 232**
- Resolved (real file/dir target, not `"."`): **111**
- Resolved (`"."` self-directory markers, used throughout for the grouping-module trick documented at `📦️packages/🦀️rust/🦀️.rs:1-11`): **109**
- **Unresolved: 12**

All 12 unresolved mounts are in `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/🦀️.rs`, and all 12 are `#[cfg(test)] mod tests_*` declarations under `artifacts::raster::standards::v1::subsets::any::schema::mutations::*`. Each references a full descriptive test-directory slug that does not exist on disk; the real directory on disk uses a short hash-suffixed slug instead:

| mutation | referenced path (does not exist) | real directory on disk |
|---|---|---|
| create-layer (🦀️.rs:101) | `🧪️tests/🖋️creates-an-ink-layer-inside-the-artwork-group/` | `🧪️tests/🖋️creates-an-ink-8a1bf9/` |
| delete-layer (🦀️.rs:114) | `🧪️tests/🚫️deletes-the-frame-group-and-its-nested-children/` | `🧪️tests/🚫️deletes-the-frame-2d257c/` |
| reorder-layers (🦀️.rs:127) | `🧪️tests/⤴️lifts-the-caption-layer-out-of-the-frame-group/` | `🧪️tests/⤴️lifts-the-caption-fe529f/` |
| rename-layer (🦀️.rs:140) | `🧪️tests/✏️renames-the-sketch-layer-to-final-linework/` | `🧪️tests/✏️renames-the-sketch-73921a/` |
| change-layer-visible (🦀️.rs:153) | `🧪️tests/🙈️hides-the-overlay-layer/` | `🧪️tests/🙈️hides-the-d0ca7b/` |
| change-layer-opacity (🦀️.rs:166) | `🧪️tests/🌫️fades-the-highlight-layer-to-a-quarter/` | `🧪️tests/🌫️fades-the-c4cbe8/` |
| change-layer-blend-mode (🦀️.rs:179) | `🧪️tests/💡️switches-the-glow-layer-to-screen/` | `🧪️tests/💡️switches-the-a17d90/` |
| move-layer (🦀️.rs:192) | `🧪️tests/📍️slides-the-stamp-layer-off-the-origin/` | `🧪️tests/📍️slides-the-stamp-b7bdca/` |
| resize-layer (🦀️.rs:205) | `🧪️tests/📐️resizes-the-canvas-layer-to-256-by-128/` | `🧪️tests/📐️resizes-the-canvas-41d97f/` |
| change-layer-adjustment-kind (🦀️.rs:218) | `🧪️tests/📈️switches-the-tone-layer-from-levels-to-curves/` | `🧪️tests/t040/` |
| add-layer-asset (🦀️.rs:231) | `🧪️tests/🖼️declines-to-reattach-an-asset-already-on-the-document/` | `🧪️tests/🖼️declines-to-4af870/` |
| remove-layer-asset (🦀️.rs:244) | `🧪️tests/🖼️rejects-removing-an-asset-the-document-never-attached/` | `🧪️tests/🖼️rejects-removing-1c84a7/` |

Because every one of these `mod` items is `#[cfg(test)]`-gated, a plain `cargo build`/wasm component build is unaffected, but **`cargo test`/`cargo nextest` for this crate will fail to compile** (rustc "file not found for module" for all 12). `git log -1 --date=iso` shows the mutation-tests directories were last touched by `3a6a9d6bfc` (2026-09-05 22:02:04, the current HEAD), one commit after the mount file itself (`b0dfa0f09b`, 2026-09-05 19:04:38) — consistent with a peer's in-flight rename of the fixture directories (hash-suffixing) that has not yet been reflected back into `packages/🦀️rust/🦀️.rs`.

Every other `#[path]`/`include_str!` mount (artifacts root, schema/snapshot/diff/mutations non-test modules, io import/export leaves for all 9 formats, editor/viewer component tree, commands, modes/windows/panels, presence/config schema, wasm bridge, examples) resolves cleanly.

---

## 3. Apps, commands, interactive-job classification

Raster defines exactly two apps in one closed fleet (`✏️s/🔌️plugins/🖨️raster/🦀️.rs:10-15`):
```
RasterApps::RasterEditor(VcsArtifactApp<EditorApp<crate::editor::raster::RasterPlayApp>>)
RasterApps::RasterViewer(VcsArtifactApp<ViewerApp<crate::viewer::raster::RasterViewer>>)
```
No `🪆️subsets` split beyond the single `✳️any` subset under standard `🔖️1`.

### RasterPlayApp (editor) — `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

`RasterCommand` (`app_commands!`, lines 174-200) declares 16 rows: `addLayer, dropLayerKind, setLayerVisible, toggleLayerVisible, deleteLayer, duplicateLayer, patchLayer, patchLayers, moveLayer, setBrushSize, setBrushOpacity, setCompositeViewport, setCamera, setCameraZoom, setActiveUtility, setLocale`. `create_raster_app()` (lines 424-512) additionally declares 15 manifest actions (14 `.action_with(raster_internal_action(...))` rows + the auto-injected `setActiveUtility`/interaction-domain actions) plus one `.mutation("addLayer", ...)`.

**Critical finding: zero `.action_interactive_job(...)` calls anywhere in the raster tree** (`grep -rn 'action_interactive_job' ✏️s/🔌️plugins/🖨️raster` → no matches, vs. 4 matches each in `🧱️block` and `🧩️puzzle`). Every action/command therefore keeps the framework default `InteractiveJobClassification::Unclassified` (default confirmed at `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:837-839`, enum definition, and the builder path that leaves it unset). Per `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:949-969` (`validate_interactive_job_classification`), **any `Unclassified` action/command in the catalog is a release-blocking validation error**, and per the doc comment there, "UI dispatch separately rejects dispositions that are not `Migrated`" — i.e. even bypassing that gate, none of raster's 15 actions / 16 commands are UI-dispatchable today.

Compare to the oracle pattern in `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: `Block2dPlayApp::classification()` returns `Migrated` (line 234), the manifest explicitly calls `.action_interactive_job("patchNodeKind", InteractiveJobClassification::Migrated)` etc. for exactly the 9 UI-dispatchable ids (lines 627-635), and a test (line 715) asserts every `bounded_first_step_tool_proofs()` entry is classified `Migrated`. `🧩️puzzle` mirrors this identically. **Raster has none of this.**

**Also missing: `bounded_first_step_tool_proofs!` / `factory_type` / retained command job factory.** `grep -rn 'bounded_first_step_tool_proofs\|factory_type'` over the raster tree → no matches (vs. present in all three block subsets `🧊️3d`/`🖐️5d`/`◻️2d` and all three puzzle subsets). `ArtifactEditor::bounded_first_step_tool_proofs()` has a default empty-`Vec` body (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26646-26649`) that `RasterPlayApp` never overrides — so there is no owned reducer factory registered at all, matching the "Bare Bounded Factory Means Every Action Dead" failure mode from prior audits (missing `factory_type` → `interactive-job.missing-owned-reducer`), compounding finding above: raster's entire editor action surface is dispatch-dead at runtime even assuming a clean compile.

**Default boot snapshot:** `RasterPlayApp::initial_snapshot()` (editor, line 258-260) and `RasterViewer::initial_snapshot()` (viewer) both return `crate::artifacts::raster::schema::empty_raster_document()`, which is **not** a bare `::default()` — it seeds `id = "empty"` and one real `Pixel` layer named "Background" at 512×512 (`🧬️schema/🦀️.rs:381-386`).

### RasterViewer (viewer) — `👁️viewer/🦀️.rs`

Read-only by construction: single inert `RasterViewCommand::Noop` variant, `handle()` always returns `ViewEmit::default()`. No classification needed (viewer commands are exempt — it never emits mutations), and none is declared. `surface_tests` in the plugin root (`🦀️.rs:47-62`) exercises `assert_viewer_never_mutates::<RasterViewer>()` and `assert_editor_and_viewer_share_dialect::<RasterPlayApp, RasterViewer>()` — both real framework assertions, not stand-ins.

---

## 4. Examples

- `📚️examples/🎬️demo/🦀️.rs`: a real `ExampleSource` (`ID = "demo"`, `PRIMARY_TEXT = include_str!("🖼️assets/🗣️.dsl.semio")`), mounted at `packages/🦀️rust/🦀️.rs:712-718` (`pub mod examples { pub mod art_raster_demo; ... }`).
- `✏️editor/📚️examples/🎬️demo-session/🦀️.rs` (+ `🖼️assets/🎮️.cmd.semio`): a second example artifact, mounted at `packages/🦀️rust/🦀️.rs:532-540` (`artifacts::raster::examples::demo`) and again at the plugin-root shim (`packages/🦀️rust/🦀️.rs:714-715`, `app_raster_demo_session`).
- **Neither is reachable from the running app.** `create_raster_app()`'s builder chain (`✏️editor/🦀️.rs:424-512`) never calls `.example(...)`; the doc comment at lines 419-423 explicitly flags this as a known SDK gap: `EditorBuilder::.editor::<E>(def: AppDefinition)` takes a bare `AppDefinition` with "no place left on this builder for the old `.example(...)`/`.workflow(...)` calls," and states both are "dropped here, not silently ported."
- **No `set-active-example`/`setActiveExample` handler exists at all** — it was deliberately deleted (`✏️editor/🦀️.rs:454-456`, `🎮️commands/📃️document/🦀️.rs:2-6`): "Whole-document replace (`setSnapshot`, `setActiveExample`) is gone — file-open/load-example go through the `.example(...)` registration … entirely outside `RasterMutation` history." Since that registration is never actually called, the demo/demo-session examples are orphaned data with no code path that surfaces them to a user or test.

---

## 5. IO — registered formats, real vs. stub

`io_registry::entries()` (`🚪️io/🦀️.rs:566-583`) registers 9 export composer entries (gif, svg, pdf, jpg, png, json, dwg, bmp, tiff) plus the native `RasterAnyComposer` import entry. Checked every leaf `deserialize_bytes`/`serialize_bytes`:

| format | import (`deserialize_bytes`) | export (`serialize_bytes`) |
|---|---|---|
| gif | **STUB** — ignores `bytes`, returns `empty_raster_snapshot()` titled "Imported gif" | **STUB** — `require_empty_output_shell()` then prints the artifact's own DSL text |
| svg | **STUB** (same pattern) | **STUB** (same pattern) |
| pdf | **STUB** | **STUB** |
| jpg | **STUB** | **STUB** |
| png | **STUB** | **STUB** |
| tiff | **STUB** | **STUB** |
| bmp | **STUB** | **STUB** |
| dwg | **REAL** — `semio_s_plugin_stdio::artifacts::dwg::{decode_dwg, dwg_from_bytes}` then `raster_document_json_from_dwg` (real geometry→layer conversion) | **STUB** (same pattern as above) |
| json | **REAL** — `parse_json_text` + `JsonSnapshot`/`RasterSnapshot::from_value` (stdio's real JSON bridge) | **REAL** — `write_json_pretty` via stdio's real `JsonSnapshot` |

The 7 "STUB" export functions (gif/svg/pdf/jpg/png/tiff/bmp) plus dwg-export are byte-identical:
```rust
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    snapshot.require_empty_output_shell().map_err(str::to_owned)?;
    Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
```
`require_empty_output_shell` (`🧬️schema/📸️snapshot/🦀️.rs:47-53`) returns `Err(RASTER_POPULATED_OUTPUT_ERROR)` **unless `layers.is_empty() && assets.is_empty()`**. Since every real document (including the app's own `initial_snapshot()`) has at least one layer, **exporting any non-trivial raster document to gif/svg/pdf/jpg/png/tiff/bmp/dwg always fails** — these 8 of 9 registered export composer entries are non-functional placeholders, not real encoders (they don't even encode the correct byte format on the one case they don't error on — they print raster's own DSL text, mislabeled as the target format).

`RasterAnalyzer`/`RasterComposerComposition::reads()` (`🗿️artifacts/🖨️raster/🦀️.rs:335-341`) lists all 9 as read dialects for import composition, but 7 of those import paths are the same no-op stubs above.

Separately, `artifact_kind()` (`🗿️artifacts/🖨️raster/🦀️.rs:649-650`) declares `export_formats: vec![]` / `import_formats: vec![]` (empty) at the `ArtifactKindSpec` level, even though `export_stdio_kinds`/`import_stdio_kinds` (line 651-652) list all 9 `stdio.*` kinds and `io_registry::entries()` separately registers all 9 composer entries — two different declaration surfaces disagree on whether formats exist; worth checking against another plugin (e.g. `🧱️block`/`🧩️puzzle`) for which of the two is the intended source of truth.

---

## 6. Framework-API drift check

Compared raster's trait impls against the current framework trait definitions:

- **`ArtifactDsl`** — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4897-4906`: `parse_dsl`/`print_dsl` are **sync**. Raster's impls (`RasterSnapshot` at `🧬️schema/📸️snapshot/🦀️.rs:523`, `RasterConfig` at `✏️editor/🎚️config/🦀️.rs:44`, `RasterPresence` at `✏️editor/👥️presence/🦀️.rs:37`) are all sync — **no drift**.
- **`ArtifactPack`** — same file, line 9332-9344: `encode_pack_with`/`decode_pack_with` **sync**. Raster's impls (`RasterSnapshot` at `📸️snapshot/🦀️.rs:543`, `RasterConfig` at `🎚️config/🦀️.rs:65`, `RasterPresence` at `👥️presence/🦀️.rs:60`) are sync — **no drift**.
- **`MutationKind::diff`/`inverse`** — actual definition lives at `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:215-227` (not at the `📡️replication/🎮️mutation` guess path verbatim as one file per the task prompt, but that IS the real path — confirmed via grep), both **sync** (`fn diff(&self, base: &P) -> MutationOutcome<...>`, `fn inverse(&self, base: &P) -> Vec<Op>`). All 12 raster mutation kinds (create-layer, delete-layer, reorder-layers, rename-layer, change-layer-visible, change-layer-opacity, change-layer-blend-mode, move-layer, resize-layer, change-layer-adjustment-kind, add-layer-asset, remove-layer-asset) implement `protocol::MutationKind<RasterSnapshot, RasterMutation>` with sync `diff`/`inverse` (e.g. `🧬️mutations/🌱create-layer/🦀️.rs:19-26`) — **no drift**.
- **`OpBinary::encode_op`/`decode_op`** — `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1266-1271`, both **sync**. `RasterMutation`/`RasterCommand`/`RasterViewCommand` implement this sync (e.g. `👁️viewer/🦀️.rs:23-29`). However, `🧬️schema/🧬️mutations/💾️binary/🦀️.rs:13-19` defines free-function **`pub async fn encode_op`/`pub async fn decode_op`** that just call the sync trait methods with no actual suspension inside — non-suspending async wrappers, consistent with the known repo-wide "Semio Async Convention Debt" (per project memory) rather than a raster-specific new bug; flagged for completeness since the audit asked for an `async fn` count.
- **`ArtifactEditor::render`** returns `UiAssemblyResult<ComponentTree>` (framework trait, confirmed via the same file as `bounded_first_step_tool_proofs` around line 266xx). `RasterPlayApp::render` (`✏️editor/🦀️.rs:320`) matches this signature exactly — **no drift**. `ArtifactViewer::render` returns bare `UiNode` (different trait, viewers don't build a full `ComponentTree`); `RasterViewer::render` (`👁️viewer/🦀️.rs`) matches — no drift.
- **`Label`/`ToValue`/`FromValue` vs serde**: no production `#[derive(Serialize/Deserialize)]` outside the one `#[cfg_attr(test, ...)]` in `🧬️schema/🦀️.rs:11`; `ToValue`/`FromValue` (framework derive macros) are used throughout for real (e.g. `dsl::ToValue::to_value(document)` in `✏️editor/🦀️.rs:59`) — no drift.

**`async fn` count**: 170 total in the raster tree; 168 are `#[semio_framework_async_macros::async_test]`-tagged test functions. The only 2 non-test `async fn` are the `encode_op`/`decode_op` wrappers above (`🧬️mutations/💾️binary/🦀️.rs:14,19`) — both non-suspending. No `E0053`/`E0046`/`E0433`-shaped drift spots were found in the trait impls actually inspected (ArtifactEditor, ArtifactDsl, ArtifactPack, MutationKind, OpBinary all match current signatures); the concrete compile risk in this crate is the test-module path drift in §2, not a trait-signature mismatch.

---

## 7. Plugin-root descriptor files

- `✏️s/🔌️plugins/🖨️raster/🛂️.descriptor.semio` — present, 52,156 bytes.
- `✏️s/🔌️plugins/🖨️raster/🔣️.json` — present, 222,832 bytes (binary/compiled-looking catalog dump, not human JSON text).
- Both files' last real content change per `git log -1 --date=iso`: commit `21fbcd3538` @ **2026-09-02 12:19:02 +0200**.
- The plugin tree's overall latest commit: `3a6a9d6bfc` @ **2026-09-05 22:02:04 +0200** (same commit that touched the mutation-test fixture directories, see §2).
- **Stale**: the descriptor/catalog snapshot at the plugin root predates ~3 days and at least the test-directory rename by three intervening days of raster-tree commits (`fe7c8a8f8b` 09-05 03:53, `b0dfa0f09b` 09-05 19:04, `3a6a9d6bfc` 09-05 22:02) — it was not regenerated after those changes. Cannot decode the binary `🔣️.json` payload to prove which specific fields drifted without the generator tool, but the mtime/commit gap alone establishes staleness.
- `🧪️oracle/🔣️.json` (plugin root) is a hand-authored `noOracleDecision` manifest (schemaVersion 2), not a compiled catalog — it records that raster's own artifact has no third-party reference implementation and points `oracleHostPackages` at `semio-s-plugin-stdio-test-oracle`. Not obviously stale, no strong drift signal there.

---

## 8. Prioritized defect list

1. **(Runtime-blocking, highest priority)** Zero `.action_interactive_job(...)` calls in `create_raster_app()` — all 15 manifest actions and all 16 `RasterCommand` rows sit at the default `InteractiveJobClassification::Unclassified`. This fails the release-blocking `validate_interactive_job_classification` gate (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:952-969`) and, per that function's own doc comment, is separately rejected by UI dispatch regardless. **Nothing in the raster editor is currently dispatchable.** Fix: classify every currently-dispatched action/command as `Migrated` (mirroring `🧱️block`'s `◻️2d` editor, `✏️editor/🦀️.rs` lines 627-635) or an appropriate other classification.
2. **(Runtime-blocking, tied to #1)** No `bounded_first_step_tool_proofs!` macro invocation, no `factory_type`, no retained command job factory anywhere in raster — `RasterPlayApp` relies on the trait's empty default. Without an owned reducer factory registered, even correctly `Migrated`-classified actions will hit `interactive-job.missing-owned-reducer` at runtime. Fix: add the macro block (mirror `🧱️block`'s `◻️2d`/`🖐️5d`/`🧊️3d` editors) with a real `Raster...RetainedCommandJobFactory` and `factory_type`.
3. **(Test-suite-blocking)** 12 unresolved `#[cfg(test)] mod tests_*` paths in `📦️packages/🦀️rust/🦀️.rs` (lines 101,114,127,140,153,166,179,192,205,218,231,244) reference full-slug test directories that don't exist; real directories use short hash-suffixed slugs (table in §2). `cargo test`/`cargo nextest` for this crate will not compile until these 12 `#[path]` strings are updated to match the actual on-disk directory names (or the directories are renamed back).
4. **(Feature gap, silent)** 8 of 9 registered export composer formats (gif/svg/pdf/jpg/png/tiff/bmp/dwg) are non-functional stubs that error on any non-empty document (`require_empty_output_shell`) and, on the one no-op path they don't error on, print the wrong byte format entirely (raster's own DSL text mislabeled as the target format). 7 of 9 import deserializers ignore their input bytes and fabricate an empty document. Only `json` (both directions) and `dwg` (import only) are real. If these composer entries are meant to be functional export/import paths (they are registered and advertised via `export_stdio_kinds`/`import_stdio_kinds`), they need real encoders/decoders; if they are deliberately scaffolded placeholders, that should be documented at the registration site so callers don't assume raster can round-trip gif/svg/pdf/jpg/png/tiff/bmp today.
5. **(Feature gap, acknowledged in-repo)** `demo` and `demo-session` examples exist as real data (ExampleSource + DSL/cmd fixtures) but are never wired into `create_raster_app()` — no `.example(...)` call, and the `setActiveExample` action was deliberately removed with no replacement dispatch path. Comment at `✏️editor/🦀️.rs:419-423` calls this a known SDK gap shared with `cad`. Needs either an `EditorBuilder` API that accepts examples, or an explicit decision that these examples are dead weight to be deleted.
6. **(Consistency, low severity)** `artifact_kind()`'s `export_formats`/`import_formats` fields are both `vec![]` while `io_registry::entries()` registers 9 composer entries and `export_stdio_kinds`/`import_stdio_kinds` list all 9 `stdio.*` kinds — worth reconciling against the block/puzzle convention for which field is authoritative.
7. **(Housekeeping, low severity)** Plugin-root `🛂️.descriptor.semio` / `🔣️.json` are stale relative to the latest 3 commits on the raster tree (including the test-dir rename in #3) — regenerate once #3 is fixed.
8. **(Cosmetic, repo-wide debt, not raster-specific)** Two non-suspending `async fn` (`encode_op`/`decode_op` in `🧬️mutations/💾️binary/🦀️.rs:14,19`) wrap already-sync `OpBinary` trait methods — harmless but part of the documented repo-wide async-convention debt; not blocking.

Nothing in `[dependencies]`/`[target.*]` is `target_arch`-gated (§1) — not a defect, just noted since the task asked for it explicitly; no native-vs-wasm dependency split exists in this crate's Cargo.toml at all.
