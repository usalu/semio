# 📓️ Rename `📸️remodel` → `📸️remodeling`, and strip leaked emoji from `energy`/`architect` machine ids

## Task A — `📸️remodel` artifact renamed to `📸️remodeling`

### What changed

- **Directory move**: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel` → `.../🗿️artifacts/📸️remodeling` (plain `mv`, plugin directory `✏️s/🔌️plugins/📸️remodel` untouched).
- **Nested rename**: `.../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-remodel-1` → `.../mutate-remodeling-1` (only nested path that embedded `remodel`).
- **Bulk content rewrite** across all 660 files under the renamed artifact tree, via a token-aware Perl pass (never a blanket `s/remodel/remodeling/`):
  - `🗿️artifacts/📸️remodel/` → `🗿️artifacts/📸️remodeling/` (every `#[path = "..."]` string and the two TS barrel import paths).
  - `REMODEL_` → `REMODELING_` (SCREAMING_SNAKE consts: `REMODEL_DIALECT`, `REMODEL_DOCUMENT_SCHEMA`, `REMODEL_EMPTY_MESH_CHILD_ID`, `REMODEL_BOX_MESH_CHILD_ID`, `REMODEL_*_CONTENT_BYTES/CHUNKS`, `REMODEL_BOUNDED_MESH_*`, `REMODEL_MAX_STAGED_BLOBS`, `REMODEL_PRIVATE_*_STAGING`, `REMODEL_TEST_MESHES`, …).
  - `Remodel(?!Apps)` → `Remodeling` (every CamelCase type — `RemodelSnapshot`, `RemodelMutation`, `RemodelDiff`, `RemodelArtifact`, `RemodelPlayApp`, `RemodelViewer`, `RemodelAssetChild`, `RemodelMeshChild`, `RemodelDurableArtifact(Store)`, `RemodelStagingFault`, `RemodelAssetContentKind`, `RemodelAssetBlob`/`RemodelMeshBlob`, `RemodelAssetChunkSource`, plus every generated per-format type such as `RemodelIntoJson`/`JsonIntoRemodel`-style names — all become `Remodeling*`). `RemodelApps` was excluded by the negative lookahead — see below.
  - `remodel(?![a-z])` → `remodeling` (every snake_case/dotted/kebab token: fn names like `remodel_asset_chunk_source`, module paths `artifacts::remodel::`/`editor::remodel::`/`viewer::remodel::`, string ids `3d.remodel`, `remodel.scene`, `remodel.document/op/diff/pack/spr`, `s.remodel.*` capability/grammar rows, test `.feature` tags `@capability-remodel-1-mutate` etc., and JSON fixture ids).
- **Manual follow-up fixes** the safe regex correctly declined to touch because "remodel" was glued directly to a lowercase suffix (no delimiter) — these are genuine compound identifiers, fixed by hand in `✏️editor/🎚️config/🧬️schema/🦀️.rs` and `✏️editor/🎚️config/🦀️.rs`:
  - `s.remodel.remodel.remodelworldcamera` → `s.remodeling.remodeling.remodelingworldcamera`
  - `...remodellayervisibility` → `...remodelinglayervisibility`
  - `...remodelframecursor` → `...remodelingframecursor`
  - `extension = "remodelcfg"` → `extension = "remodelingcfg"`
- **`ArtifactKindSpec`** (`🗿️artifacts/📸️remodeling/🦀️.rs`) now reads:
  ```
  id: "3d.remodeling", name: "3D Remodeling", source_format: "remodeling.scene",
  component_kind: "remodeling", schema: "remodeling.scene"
  ```
  Note: the ticket's "currently"/"change to" table wrote the name as `3DRemodel`/`3DRemodeling` (no space); the actual code had `"3D Remodel"` (with a space). I kept the existing space convention → `"3D Remodeling"` rather than introduce a new no-space style — this is the only literal deviation from the ticket's exact text, and it only affects a display string.
  The file-extension for the document codec also moved from `remodel` to `remodeling` (`extension: Some("remodeling")`, codec descriptor `remodeling.scene:remodeling`) — a direct, intended consequence of the noun rename, not separately called out in the ticket but consistent with `source_format`/`schema` changing.
  German localization stayed `"Umbau"` (already the correct German noun, no emoji, nothing to change).
- **Plugin-identity lines intentionally left as `remodel`** (plugin directory/crate/label/id, per the ticket's explicit carve-out), fixed by hand after the bulk pass wrongly touched them:
  - `✏️s/🔌️plugins/📸️remodel/🦀️.rs`: `pub enum RemodelApps`, `Plugin::<RemodelApps>::builder("remodel")`, `.label("Remodel")`, doc comment "the remodel editor and viewer surfaces" — all restored/kept as `remodel`. The `RemodelApps` variants' generic type arguments (`crate::editor::remodeling::RemodelingPlayApp`, `crate::viewer::remodeling::RemodelingViewer`) and the `.artifact(...)`/`.activation(...)` calls into `crate::artifacts::remodeling::...` were updated. The two cross-surface test fns were renamed `remodeling_viewer_never_mutates` / `remodeling_editor_and_viewer_share_dialect`.
  - `📦️packages/🦀️rust/🦀️.rs` (wiring file): top module doc `"📸️ Remodel plugin — ..."` restored to `Remodel` (describes the plugin); every `#[path]`/module-nesting/const-alias below it (all artifact-scoped) is `remodeling`.
  - `📦️packages/🦀️rust/Cargo.toml`: package name `semio-s-plugin-remodel`, `package.metadata.component` `semio:remodel`, playground `variant = "remodel"` all untouched (plugin identity); only the free-text `description` line's two artifact mentions ("the remodel artifact" / "the remodel play app") became "remodeling".
  - `📦️packages/🟦️typescript/🟦️.ts`: rewritten to `remodeling_schema`/`remodeling_io` exports (artifact-scoped facets; no external importers found repo-wide).
  - `📦️packages/*/package.json`, `📋️project.json`, `📜️script.ts`, and the plugin-root `🎮️commands/📌️.empty.md` / `🧪️oracle/🔣️.json` reference only the plugin (`remodel-plugin`, `remodel-js`, `@semio-tech/remodel-plugin`, playground ports) — left untouched, correctly.
- **Not touched (generated/compiled, will regenerate on next build)**:
  - `✏️s/🔌️plugins/📸️remodel/🔣️.json` (670 KB, **untracked** — a plain-text dump of the compiled plugin descriptor) and `🛂️.descriptor.semio` (binary, also untracked) are both produced by `bun ./📜️script.ts describe` (confirmed by reading `📦️packages/🦀️rust/📜️script.ts`'s `DescribeScript`, and by the binary blob literally containing the old `"3d.remodel"`/`"3D Remodel"` strings verbatim as compiled UI-manifest text). Hand-editing a 670 KB generated JSON (which conflates plugin-level fields that must stay `remodel` with artifact-level fields that must become `remodeling`) was judged too error-prone versus just re-running `describe` after the crate rebuilds. **Follow-up**: run `bun ./📜️script.ts describe` for the remodel plugin once its wasm32-wasip2 build succeeds, to refresh these two files.
  - By contrast, the **subset policy** (`🏅️standards/🔖️1/🪆️subsets/🔣️.json`) and the **oracle decision** (`.../🧪️oracle/🔣️.json`) and the **mutation-catalog** file are hand-authored source (prose rationale, dated revision notes) and were updated by the bulk pass — verified correct (`"artifact": "s.remodeling.remodeling"`, `"id": "remodeling-mutation-semantics"`, `"id": "remodeling-1-any"`, `"capability": "remodeling-1-mutate"`).

### Verification performed

- Repo-wide corruption re-grep after the bulk pass: no `remodelinging`, `remodelinged`, `reremodel`, or stray `RemodelingApps`.
- Repo-wide re-grep confirms **zero** remaining bare `remodel`/`Remodel` tokens anywhere under the artifact tree (everything is `remodeling`/`Remodeling`, or intentionally `RemodelApps`).
- Repo-wide search for the literal old path `🗿️artifacts/📸️remodel/` outside this plugin: only hits are (a) historical ticket archives (`.🧬semio/.../🎫️tickets/26/08/**`, out of scope — dated records, not live source) and (b) one **pre-existing, already-stale** comment in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/.../🚪️io/🦀️.rs:66` referencing `⚙️engine` (removed from the taxonomy years before this ticket) — left untouched per the explicit "do not touch `🗄️stdio`" rule.
- `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`: **could not get a clean full run this session** — see "Verification gate — cargo check" below.

## Task B — emoji stripped from machine ids

### `energy` plugin (`data.🔋️model` → `data.model`)

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`: `id: "data.🔋️model".into()` → `id: "data.model".into()`; doc comment above `artifact_kind()` updated to match. Rest of `ArtifactKindSpec` (`name: "Energy Model"`, `source_format`/`schema: ENERGY_MODEL_DOCUMENT_SCHEMA` = `"energy.model"`, `component_kind: "energy"`) was already pure ASCII.
- `✏️s/🔌️plugins/🔋️energy/🔣️.json` (tracked, committed manifest): one occurrence `"kind": "data.🔋️model"` → `"data.model"`.
- `✏️s/🔌️plugins/🔋️energy/🛂️.descriptor.semio` (tracked binary compiled descriptor): confirmed via `strings`/`grep -a` to still contain the raw bytes `data.🔋️model`. This is a **build output** (same `describe` pipeline as remodel's) with length-prefixed string encoding — hand-patching would corrupt it since the ASCII replacement is a different byte length. **Follow-up**: regenerate via `bun ./📜️script.ts describe` for the energy plugin.
- Repo-wide grep for the old literal `"data.🔋️model"` otherwise only hits historical ticket JSON/report archives (Aug 2026 tickets) — left untouched.

### `architect` plugin (`data.🏛️program` → `data.program`)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs`: `id: "data.🏛️program".into()` → `id: "data.program".into()`; doc comment updated. Rest of `ArtifactKindSpec` (`name: "Architect Program"`, `source_format`/`schema: ARCHITECT_PROGRAM_SCHEMA` = `"architect.program"`, `component_kind: "architect"`) was already pure ASCII.
- `✏️s/🔌️plugins/🏛️architect/🔣️.json` (tracked, committed manifest): one occurrence fixed the same way.
- `🛂️.descriptor.semio`: same binary-regeneration follow-up as energy.
- **Bonus find**: `🧰️framework/🔨️modules/🚪️io/🦀️.rs`'s `artifact_kind_id_rejects_non_canonical_grammar` test used the *real* (pre-fix) `"data.🏛️program"` string as its live "contains emoji → must be rejected" example — direct proof this literal id failed `is_canonical_artifact_kind`/`ArtifactKindId::parse`. Now that architect's actual id is emoji-free, that string no longer corresponds to anything real, so I swapped it for a synthetic `"data.🧩widget"` placeholder (same negative-test intent, no longer misleadingly tied to a fixed plugin).

### Final sweep — all `✏️s/🔌️plugins/*/🗿️artifacts/*/🦀️.rs` (92 artifact-root files)

Grepped every `id:`/`schema:`/`source_format:`/`component_kind:` literal line (plus, for extra rigor, every same-file `const ... = "..."` a field might indirect through) across all 92 plugin artifact roots for non-ASCII bytes.

**Result: zero remaining non-ASCII findings.** The two fixed above (`energy`, `architect`) were the only two leaks; no other plugin's artifact-root file has emoji (or any other non-ASCII) inside these four fields.

## Verification gate — `cargo check --target wasm32-wasip2`

- **`semio-s-plugin-energy`**: ✅ clean, `[exited with code 0]`, only pre-existing unrelated warnings in `semio-framework`/`semio-framework-ui` (dead-code/unused-Result lints, nothing touched by this ticket).
- **`semio-s-plugin-architect`** and **`semio-s-plugin-remodel`**: ❌ could not complete cleanly this session — **not because of anything in this diff**. Repeated retries (12 total for remodel across the session, 2 for architect) all fail inside `🧰️framework/🔨️modules/🕸️graph`'s auto-generated `🦀️generated-value-bridge.rs` (`error[E0433]: cannot find 'rewrite_lhs'/'draw_layers' in 'generated'`, plus at times `error[E0432]: unresolved import 'super::dsl_core'` and one `#[derive(ToValue, FromValue)]` tuple-struct error). The error count moved 47 → 27 → 23 → 19 → 23 → 23 over successive retries: the `rewrite_lhs`/🔱️trinity half resolved and its rename landed in the auto-commit history (verified — `HEAD` now has `♻️rewriting`, no more `♻️rewrite`), but the `draw_layers`/🖍️draw half is still actively mid-flight as of the last retry. This is direct evidence of another live session's in-progress rename, not a stable state — I stopped polling per the "don't chase concurrent churn" convention rather than loop indefinitely. Root cause, confirmed directly:
  - Both `remodel` and `architect` depend on `semio-framework-graph` (`energy` does not — which is exactly why energy alone came back clean).
  - Mid-session, `git status` showed `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/...` staged for deletion — another live session executing this same ticket's Wave 1 rename (`🔱️trinity ♻️rewrite → ♻️rewriting`, per `📓️plan.md`), and `🖍️draw`'s renamed manifest paths appear in `graph`'s build-script `rerun-if-changed` list too. That trinity rename has since landed via this repo's auto-commit (`HEAD` at `96aa4f8c12` now has `♻️rewriting`, no more `♻️rewrite`), clearing the `rewrite_lhs` half of the errors; the `🖍️draw` half (`draw_layers`, `dsl_core`) is still mid-flight as of the last retry. `semio-framework-graph`'s generated registry (`generated::rewrite_lhs`, `generated::draw_layers`) trails whichever of these two renames hasn't finished yet, so it fails to build for **anyone** on the shared workspace right now — independent of this ticket's remodel/energy/architect changes, which are all separately confirmed correct (see final sweep above, and energy's clean run).
  - Nothing in this diff touches `🔱️trinity`, `🖍️draw`, or `🧰️framework/🔨️modules/🕸️graph`.
  - **Follow-up**: re-run `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2` and `-p semio-s-plugin-architect` once the concurrent trinity/draw rename lands and `graph`'s generated registry is refreshed.

## Files touched (Task A, `📸️remodel` → `📸️remodeling`)

- Moved: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/**` → `.../🗿️artifacts/📸️remodeling/**` (660 files), including the nested rename `.../🧪️tests/mutate-remodel-1` → `mutate-remodeling-1`.
- Content-edited (bulk pass, all 660 files under the new `📸️remodeling` tree) plus manually touched, outside that tree:
  - `✏️s/🔌️plugins/📸️remodel/🦀️.rs`
  - `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/🦀️.rs`
  - `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml`
  - `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🟦️.ts`

## Files touched (Task B, emoji-in-id hygiene)

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔣️.json`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🔣️.json`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs` (test fixture string, unrelated plugin — see "Bonus find" above)

## Not modified, flagged for a separate follow-up (out of this ticket's scope)

- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/package.json` is boilerplate copy-pasted from the CAD plugin (`description` says "CAD plugin TS", `scripts`/`dependencies` reference `@semio-tech/cad-js*` packages) — clearly a scaffolding bug, unrelated to the artifact-naming rename, left as-is.
