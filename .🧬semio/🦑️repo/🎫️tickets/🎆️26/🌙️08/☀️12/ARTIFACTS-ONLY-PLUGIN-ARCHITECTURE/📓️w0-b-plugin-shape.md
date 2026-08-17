# W0-B — Plugin Directory Shape Census

Scope: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/` (33 owners). Read-only recon for the APA (plugin = exactly 🎛️apps + 🗿️artifacts + root 🦀️component.rs/AGENTS.md + 📦️packages wiring). All line numbers captured 2026-08-12 ~15:20; two other sessions (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, SEMANTIC-MUTATIONS-OVERHAUL) are editing this tree concurrently — re-grep the anchor strings given below before trusting a line number.

---

## 1. Headline finding — the SSOT says the opposite of the APA target

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` line 376-381:

```json
"pluginChildDirs": [
    "🛂️manifest",
    "🎟️capabilities",
    "🔧️setup",
    "🎛️apps"
  ],
```

This is **not decorative** — it is read and *enforced* at three live call sites, all requiring every plugin to carry `🛂️manifest/<leaf>`, `🎟️capabilities/<leaf>`, `🔧️setup/<leaf>` and `🎛️apps/<leaf>` at plugin root, i.e. the exact shape the APA wants dissolved:

1. **Root policy** — `/Users/ueli/Documents/semio/📜️script.ts`, function `policyPluginRootShapeBreaches`, anchor `function policyPluginRootShapeBreaches(repoRoot: string): BreachRecord[] {` — line 4646 reads `taxonomy.pluginChildDirs` (line 4648) and pushes a breach for any plugin missing `<child>/<leaf>` (loop starts ~4666, continues past what was read).
2. **OS plugin registry codegen** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`, anchor `const pluginChildDirs = TAXONOMY.pluginChildDirs;` — line 1224, loop at 1232-1236 pushes a `findings` entry for every plugin missing `<pluginRoot>/<child>/<TAXONOMY_LEAF_FILENAME>`.
3. **Rust "taxonomy gate" test/assert** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, anchor `let plugin_child_dirs = string_array(&taxonomy, "pluginChildDirs");` — line 2226, followed by a hard `assert!` per child at lines 2229-2235: `"taxonomy gate: plugin root missing {child}/{leaf} at {}"`. This is a runtime `assert!`, not a soft lint — it will panic if `pluginChildDirs` shrinks to `[apps, artifacts]` while any plugin lacks the removed facets, and equally will panic once a plugin *drops* the three facets while this assert still demands them. **Any migration wave that deletes `🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/` from a plugin must update `pluginChildDirs` in taxonomy.json AND this assert in the same wave, or the gate breaks the build.**

Validation/shape helpers that also key off `pluginChildDirs` (read-only, non-enforcing, but will need updating in lockstep):
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` — type field line 174 (`readonly pluginChildDirs: readonly string[];`), `validateTaxonomy` checks at lines 582/583/585/586.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` — literal expectation `expect(taxonomy.pluginChildDirs).toEqual(["🛂️manifest", "🎟️capabilities", "🔧️setup", "🎛️apps"]);` at line 1148; more assertions using `pluginChildDirs` at 1290/1292/1298.

No dep-cruiser config exists anywhere in the repo (`find . -iname '*dep-cruiser*'` outside ticket scratch: zero hits). All hardcodes of the three facet name literals outside `✏️s/🔌️plugins/` and outside ticket-scratch JSON are either (a) the taxonomy/policy/registry/test files above, or (b) unrelated homonyms — `.🦑️repo/🛂️manifest/` is a *different* concept (Neo4j Cypher export directory under repo root, referenced in root `📜️script.ts` lines 369/640/1785/1904/1918/1924 and the library README), and `🛂️manifest.semio` inside `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs` (lines 2/37/56) is the on-disk zip-payload manifest filename for *wasm extension packages*, unrelated to the plugin facet. Do not conflate these three unrelated uses of the "manifest" word when doing search-and-replace later.

---

## 2. Per-plugin table

All 33 owners have, at minimum: `🎛️apps/`, `🎟️capabilities/`, `📦️packages/`, `🔧️setup/`, `🗿️artifacts/`, `🛂️manifest/`, root `🦀️component.rs`. This baseline is omitted from the table below; only exceptions and extras are shown.

| plugin | 🛂️manifest/ | 🎟️capabilities/ | 🔧️setup/ | extra dirs beyond {apps, artifacts, packages} | file count each |
|---|---|---|---|---|---|
| ✒️writer | yes | yes | yes | none (has root `🛂️manifest.json` legacy file, 445B) | — |
| ➗️mathematical | yes | yes | yes | none | — |
| 🌀️procedural | yes | yes | yes | 🎮️play | 1 |
| 🌊️flow | yes | yes | yes | 🔨️modules; 🧩️extensions (has root `🛂️manifest.json`, 636B) | 🔨️modules=1; 🧩️extensions=45 |
| 🌍️gis | yes | yes | yes | 🔨️modules (also stray `.DS_Store`) | 1 |
| 🌿️vcs | yes | yes | yes | none | — |
| 🎞️animate | yes | yes | yes | none | — |
| 🎥️shooting | yes | yes | yes | none | — |
| 🎪️demonstrator | yes | yes | yes | 🎪️panes (no AGENTS.md/README.md at all) | 7 |
| 🎬️sequence | yes | yes | yes | none | — |
| 🏗️fem | yes | yes | yes | ➗️formulation, 🏗️model, 📏️elements2d, 🔢️sparse, 🕸️mesh, 🖥️app-surface, 🧊️elements3d, 🧮️analyses (8 dirs) | 1 each |
| 🏛️architect | yes | yes | yes | none | — |
| 🏭️process | yes | yes | yes | 🧩️extensions | 20 |
| 💠️lowpoly | yes | yes | yes | none | — |
| 💡️reasoning | yes | yes | yes | none (root `🛂️manifest.json`, 351B) | — |
| 📋️forms | yes | yes | yes | none (no AGENTS.md/README.md at all) | — |
| 📏️layout | yes | yes | yes | none (root `🛂️manifest.json`, 238B) | — |
| 📐️cad | yes | yes | yes | node_modules, 🔨️modules, 🖼️assets, 🧩️extensions, 🧫️fixtures (also root `🔣️machine.json`, 210KB) | node_modules=1; 🔨️modules=14; 🖼️assets=211; 🧩️extensions=52; 🧫️fixtures=1 |
| 📕️norm | yes | yes | yes | 🎚️config, 👥️presence, 📄️artifact, 🖥️app-surface | 🎚️config=6; 👥️presence=6; 📄️artifact=1; 🖥️app-surface=1 |
| 📖️playbook | yes | yes | yes | 🧩️extensions | 5 |
| 📜️imperative | yes | yes | yes | 🧩️extensions | 25 |
| 📸️remodel | yes | yes | yes | none (no AGENTS.md/README.md at all) | — |
| 🔋️energy | yes | yes | yes | ⚙️engine (plugin-root level, NOT inside an artifact/app) | 50 |
| 🔱️trinity | yes | yes | yes | 🌳️ast, 🔤️lexer, 🔨️modules, 🗣️language-service, 🧮️executor | 1,1,23,1,1 |
| 🕸️dag | yes | yes | yes | none | — |
| 🖍️draw | yes | yes | yes | 🔄️fsm (no AGENTS.md; root `🛂️manifest.json`, 476B) | 10 |
| 🖨️raster | yes | yes | yes | none (no AGENTS.md/README.md at all) | — |
| 🗄️stdio | yes | yes | yes | 📇️registry | 1 |
| 🗒️note | yes | yes | yes | none (no AGENTS.md; root `🛂️manifest.json`, 513B) | — |
| 🧩️puzzle | yes | yes | yes | 🔨️modules, 🧫️fixtures (also stray `.DS_Store`) | 1, 4 |
| 🧱️block | yes | yes | yes | none | — |
| 🪐️space | yes | yes | yes | none (no AGENTS.md/README.md at all) | — |
| 🪵️sourcing | yes | yes | yes | 🧩️extensions | 15 |

Notes on "1 file" extras: for `🏗️fem`'s eight one-file dirs (`➗️formulation`, `🏗️model`, `📏️elements2d`, `🔢️sparse`, `🕸️mesh`, `🖥️app-surface`, `🧊️elements3d`, `🧮️analyses`), each is a single sizeable `🦀️component.rs` (the plugin's headless FEM library split across domain-named top-level dirs — NOT one component.rs per artifact facet, a genuine top-level compute-module sprawl). Same shape for `📕️norm/📄️artifact` and `📕️norm/🖥️app-surface` (each one substantial `component.rs`), and `🔱️trinity`'s four one-file dirs. `🌍️gis/🔨️modules` and `🧩️puzzle/🔨️modules` each currently hold exactly one nested artifact-like leaf (`🏔️terrain`, `🎲️board-2d` respectively) but `🔨️modules` as a facet name is itself non-taxonomy (not in `pluginChildDirs`, `appChildDirs`, or `artifactChildDirs`).

Root `🦀️component.rs` sizes (bytes, all dated Aug 12 10:50 — batch-touched by another wave, harmless): range from 288B (🎪️demonstrator) to 5381B (🧱️block) and 4562B (🗄️stdio) and 4147B (🪐️space, the plugin with the one real capability call — see §4).

---

## 3. `🔧️setup/🦀️component.rs` — doc-only vs real code

Line-count sweep (`total` = wc -l, `real` = non-blank non-`//`/`/*`/`*` lines) across all 33: **30 of 33 are the 1-line doc-only stub** (`//! 🔧️ Setup facet for '<plugin>' — codec/language/importer registration hooked via '.setup(...)'.`). Exactly **3 plugins carry real code**:

### 🌍️gis/🔧️setup/🦀️component.rs (10 lines, 6 real) — full quote
```rust
//! 🔧️ Setup facet for `🌍️gis` — codec/language/importer registration hooked via `.setup(...)`.

/// 🔌️ Plugin `setup:` hook — register GIS host exports (languages/codecs/app schema) once at load.
pub fn register_gis_exports() {
    crate::artifacts::gismap::engine::register_pilot_languages();
    crate::artifacts::gisterrain::engine::register_pilot_languages();

    crate::apps::gis2d::config::schema::register_app_schema();
    crate::apps::gis3d::config::schema::register_app_schema();
}
```

### 💠️lowpoly/🔧️setup/🦀️component.rs (25 lines, 20 real) — full quote
```rust
//! 🔧️ Setup facet for `💠️lowpoly` — codec/language/importer registration hooked via `.setup(...)`.

/// 🔌️ One call per `MeshExporter`/`MeshImporter` format so the OS workflow VFS auto-populates from
/// `required_media_formats`; also registers the
/// `ArtifactPack` codec so `.pack`/`.ops` sync/storage paths can encode/decode `LowpolySnapshot`.
pub fn register_lowpoly_exports() {
    crate::artifacts::lowpoly::engine::register();
    crate::apps::lowpoly::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::lowpoly::LowpolyPlayApp>(crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh);
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document);
    semio_framework_os::register_mesh_importer("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh);
}
```

### 📕️norm/🔧️setup/🦀️component.rs (51 lines, 48 real) — full quote
```rust
//! 🔧️ Setup facet for `📕️norm` — codec/language/schema registration.

/// 🔌️ Registers every norm artifact language + schema descriptor.
pub fn register_norm_exports() {
    crate::config::schema::register_app_schema();
    crate::artifacts::din4108::engine::register_pilot_languages();
    crate::artifacts::din16798::engine::register_pilot_languages();
    crate::artifacts::din18599::engine::register_pilot_languages();
    crate::artifacts::en1990::engine::register_pilot_languages();
    crate::artifacts::en1991::engine::register_pilot_languages();
    crate::artifacts::en1992::engine::register_pilot_languages();
    crate::artifacts::en1993::engine::register_pilot_languages();
    crate::artifacts::en1994::engine::register_pilot_languages();
    crate::artifacts::en1995::engine::register_pilot_languages();
    crate::artifacts::en1996::engine::register_pilot_languages();
    crate::artifacts::en1997::engine::register_pilot_languages();
    crate::artifacts::en1998::engine::register_pilot_languages();
    crate::artifacts::en1999::engine::register_pilot_languages();
    crate::artifacts::iso16757::engine::register_pilot_languages();
    crate::artifacts::vdi3805::engine::register_pilot_languages();
    crate::artifacts::din4108::engine::register_artifact_schema();
    crate::artifacts::din16798::engine::register_artifact_schema();
    crate::artifacts::din18599::engine::register_artifact_schema();
    crate::artifacts::en1990::engine::register_artifact_schema();
    crate::artifacts::en1991::engine::register_artifact_schema();
    crate::artifacts::en1992::engine::register_artifact_schema();
    crate::artifacts::en1993::engine::register_artifact_schema();
    crate::artifacts::en1994::engine::register_artifact_schema();
    crate::artifacts::en1995::engine::register_artifact_schema();
    crate::artifacts::en1996::engine::register_artifact_schema();
    crate::artifacts::en1997::engine::register_artifact_schema();
    crate::artifacts::en1998::engine::register_artifact_schema();
    crate::artifacts::en1999::engine::register_artifact_schema();
    crate::artifacts::iso16757::engine::register_artifact_schema();
    crate::artifacts::vdi3805::engine::register_artifact_schema();
    crate::artifacts::din4108::engine::register_artifact_inferences();
    crate::artifacts::din16798::engine::register_artifact_inferences();
    crate::artifacts::din18599::engine::register_artifact_inferences();
    crate::artifacts::en1990::engine::register_artifact_inferences();
    crate::artifacts::en1991::engine::register_artifact_inferences();
    crate::artifacts::en1992::engine::register_artifact_inferences();
    crate::artifacts::en1993::engine::register_artifact_inferences();
    crate::artifacts::en1994::engine::register_artifact_inferences();
    crate::artifacts::en1995::engine::register_artifact_inferences();
    crate::artifacts::en1996::engine::register_artifact_inferences();
    crate::artifacts::en1997::engine::register_artifact_inferences();
    crate::artifacts::en1998::engine::register_artifact_inferences();
    crate::artifacts::en1999::engine::register_artifact_inferences();
    crate::artifacts::iso16757::engine::register_artifact_inferences();
    crate::artifacts::vdi3805::engine::register_artifact_inferences();
}
```
Anchor for re-finding: `pub fn register_norm_exports() {`, `pub fn register_lowpoly_exports() {`, `pub fn register_gis_exports() {` — all unique repo-wide.

All three call only into `crate::artifacts::<x>::engine::register_*` / `crate::apps::<x>::config::schema::register_app_schema` / `semio_framework_os::register_*` — i.e. every real setup call is a thin fan-out to code that already lives under `🗿️artifacts/…/⚙️engine/` or `🎛️apps/…/🎚️config/`. The `🔧️setup/` facet itself contains zero domain logic in all 33 cases — it is purely a registration-order fan-out. This is consistent with folding `🔧️setup/` into a `plugin()`/`register()` call inside the root `🦀️component.rs` (or into the artifact's own registration path) rather than needing a standalone facet directory.

---

## 4. `🛂️manifest/` and `🎟️capabilities/` — doc-only vs real

### 🛂️manifest/🦀️component.rs
**32 of 33 are 1-line doc-only stubs** (`//! 🛂️ Manifest facet for '<plugin>' — …`). The lone exception:

**🗄️stdio/🛂️manifest/🦀️component.rs — 362 lines, 344 real.** Despite its own doc-comment on line 1 still calling itself `"library plugin stub"`, it carries a genuinely large, real `stdio_format_descriptors()` function: a 28-entry `Vec<FormatDescriptor>` literal (binary/txt/xml/deflate/zip/json/csv/md/gltf/obj/stl/ply/las/step/ifc/dwg/dxf/svg/png/jpg/gif/bmp/tiff/pdf/docx/pptx/xlsx/bcf) plus a `register_stdio_format_descriptors()` wrapper that calls `register_format_descriptors(...)`, wrapped in `//#region 🔖️FormatCatalog` / `//#endregion 🔖️FormatCatalog`. Anchor: `pub fn stdio_format_descriptors() -> Vec<FormatDescriptor> {` (unique repo-wide). Full body already stored verbatim in the ticket's tool history if needed; omitted here only for length — every row present in `stdio_format_descriptors()` is data (kind_id/short_id/mime/extension/name/full_name/neutral/dir_name/is_binary), not behavior, and the wrapper is a single call-through — this is a real declaration, not a stub, and per APA belongs under `🗿️artifacts/<kind>/…`, not the manifest facet.

### 🎟️capabilities/🦀️component.rs
**All 33 are the identical 1-line doc-only stub**, differing only by plugin name substitution:
```
//! 🎟️ Capabilities facet for `<plugin>` — declare rights via `PluginBuilder::capability` / `.local_backbone_storage()`.
```
Zero plugins have real content in this facet.

### Which plugins declare any capability at all?
`grep -rln '\.capability(' ✏️s/🔌️plugins/` → **zero matches, repo-wide, in any file.** No plugin anywhere calls `PluginBuilder::capability(...)` for real.

`grep -rn '\.local_backbone_storage(' ✏️s/🔌️plugins/` → 33 matches inside the doc-comment text quoted above (false positives — text, not code) **plus exactly one real call**:

**🪐️space/🦀️component.rs line 63**, anchor `.local_backbone_storage()` inside `pub fn plugin() -> Plugin {`:
```rust
pub fn plugin() -> Plugin {
    crate::register_s_exports();
    Plugin::builder("s")
        .label("S Studio")
        .version("0.1.0")
        .local_backbone_storage()
        .register_document_app::<crate::apps::home::HomeApp>(crate::apps::home::create_home_app())
        .register_document_app::<crate::apps::space::SpaceApp>(crate::apps::space::create_space_app())
        .build()
}
```
**Finding: `🪐️space` is the only plugin in the whole repo with a real capability declaration, and it lives in the plugin ROOT `🦀️component.rs`, not in its own (doc-only) `🎟️capabilities/🦀️component.rs`.** Under the APA target this call belongs to the plugin's registration wiring at the root/📦️packages layer, not a dedicated `🎟️capabilities/` facet — so this is actually the one piece of evidence that `🎟️capabilities/` as a standalone directory has zero live tenants and can be dissolved outright without moving any code, only updating the taxonomy/policy/gate in §1.

---

## 5. Extra-dir → APA destination proposals

| extra dir | plugin(s) | proposed APA destination | confidence |
|---|---|---|---|
| 🎮️play | 🌀️procedural | fold into `🗿️artifacts/<kind>/📚️examples/` (single AGENTS.md-only dir today — near empty, likely a placeholder) | UNVERIFIED — dir holds only an AGENTS.md, no code; confirm with owner before deciding examples vs delete |
| 🔨️modules (flow) | 🌊️flow | `🔨️modules/🧮️compute/🟦️component.ts` → `🗿️artifacts/flowcompute/🏅️standards/🔖️1/⚙️engine/compute/` (TS compute leaf; needs an artifact-kind name — none obvious yet) | UNVERIFIED — needs a domain owner decision on artifact kind name |
| 🧩️extensions (flow, 9 crates) | 🌊️flow | each crate is `role="extension", extends="flow"` — these are NOT plugin-shape violations in the APA sense (extension crates are a separate axis, see §6); leave in place under `🌊️flow/🧩️extensions/<name>/📦️packages/🦀️rust/` unless the extension-crate location itself is later ruled out-of-APA (flag to ticket owner) | needs owner ruling |
| 🔨️modules (gis) | 🌍️gis | `🔨️modules/🏔️terrain/🦀️component.rs` → `🗿️artifacts/gismap/🏅️standards/🔖️1/⚙️engine/terrain/` (own doc-comment already says it's a GIS-specific descriptor moved out of a generic framework engine — natural fit is the gismap or gisterrain artifact's engine) | high |
| 🎪️panes | 🎪️demonstrator | app-surface UI compositional units (`🧩️aggregator`, `🗺️verfolgen`, `🗂️aussuchen`, `📐️koordinator`, `🌱️generator`, `🏭️bearbeiten` — German verb-named panes) → `🎛️apps/<app>/📌️panels/` (taxonomy already has `📌️panels` as an `appChildDirs` entry) | high, but need to know which single app of demonstrator owns them (demonstrator's 🎛️apps/ children not yet enumerated this wave) |
| ➗️formulation, 🏗️model, 📏️elements2d, 🔢️sparse, 🕸️mesh, 🧊️elements3d, 🧮️analyses | 🏗️fem | headless FEM compute library, explicitly "no UI, no VCS" per its own doc-comment → `🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/{formulation,model,elements2d,sparse,mesh,elements3d,analyses}/` (one compute artifact, many engine submodules) | high — doc-comment in 🏗️model itself says these are siblings of one calculation library |
| 🖥️app-surface (fem) | 🏗️fem | shared UI helpers used by BOTH `fem2d_ui` and `fem3d_ui` app crates → cannot go under a single `🎛️apps/<app>/`; APA has no "shared-across-apps-of-one-plugin" slot today. Candidate: `🗿️artifacts/fem/📚️examples/` is wrong (not examples); more likely needs a new sanctioned location such as `🎛️apps/⚡️shared/` or duplicate into both `fem2d`/`fem3d` apps' own `⚙️engine/`. **Flag: cannot classify — needs an explicit APA ruling on cross-app-shared-within-plugin code**, since CLAUDE.md's own banned-stems list (`bannedNameStems`: core/common/util/shared/base/lib/impl) already forbids the obvious fallback names. | **CANNOT CLASSIFY — needs ruling** |
| 🧩️extensions (process, 4 crates) | 🏭️process | extension-crate axis, see 🌊️flow row above | needs owner ruling |
| node_modules | 📐️cad | delete/gitignore — build tool cache, not source (vitest results.json inside) | high |
| 🔨️modules (cad, 14 files) | 📐️cad | needs per-subdir inspection (not yet enumerated this wave — only sampled top of tree) → likely same pattern as gis/puzzle: nested artifact-shaped leaves that belong under `🗿️artifacts/<cad-artifact>/🏅️standards/…/⚙️engine/` | UNVERIFIED — not enumerated |
| 🖼️assets (cad, 211 files) | 📐️cad | → `🗿️artifacts/<kind>/📚️examples/…/🖼️assets/` per taxonomy's `exampleAssetsDirName`, or a fixture image set — needs to know which artifact(s) consume it | UNVERIFIED — not enumerated |
| 🧩️extensions (cad, 4 crates) | 📐️cad | extension-crate axis, see above | needs owner ruling |
| 🧫️fixtures (cad, 1 file: a PNG) | 📐️cad | → `🗿️artifacts/<kind>/📚️examples/<slug>/🖼️assets/` | high, once artifact kind is named |
| 🔣️machine.json (cad, root file, 210KB) | 📐️cad | too large to classify blind — likely a generated/vendored machine-learning or CAD-kernel data file; **flag: cannot classify without reading contents (out of scope this wave, large binary-ish JSON)** | **CANNOT CLASSIFY** |
| 🎚️config, 👥️presence | 📕️norm | these are **plugin-root-level** app-facet dirs (taxonomy's `appChildDirs`/`configChildDirs`/`presenceChildDirs` define `🎚️config`/`👥️presence` as children of an **app**, not a plugin). norm's own doc-comment (`🖥️app-surface/🦀️component.rs`) says all fifteen norm apps are structurally identical and differ only per-standard — so `🎚️config`/`👥️presence` were hoisted to plugin-root as a shared-default for those 15 apps. → APA-legal home is unclear: either duplicate into each of the 15 `🎛️apps/<norm-app>/🎚️config,👥️presence/`, or (if APA later grows a shared-app-defaults notion) a new sanctioned slot. **Flag: needs ruling**, same shared-across-apps problem as fem's 🖥️app-surface above. | **NEEDS RULING** |
| 📄️artifact | 📕️norm | "Norm core: shared quantities, clause identity, compliance results, national annex selection" — feeds all 15 norm standard artifacts (din4108, en199x, iso16757, vdi3805, …) → `🗿️artifacts/norm/🏅️standards/🔖️shared/⚙️engine/core/` or, if APA disallows a artifact-spanning shared engine, this needs the same cross-artifact-shared ruling as above | **NEEDS RULING** (blocked on same shared-code question) |
| 🖥️app-surface (norm) | 📕️norm | same shared-across-15-apps machinery as fem's row above | **NEEDS RULING** |
| 🧩️extensions (playbook, 5) | 📖️playbook | extension-crate axis | needs owner ruling |
| 🧩️extensions (imperative, 25) | 📜️imperative | extension-crate axis | needs owner ruling |
| ⚙️engine (energy, 50 subdirs) | 🔋️energy | **The single largest violation found.** A full headless HVAC/building-physics engine (air_exchange, airflow_network, calendar, coils, comfort, controls, curves, daylight, dispatch, economics, electrical, envelope, evaporative, fans, faults, fenestration, gains, geometry, heat_recovery, humidity_eq, hvac_topo, iaq, ideal_hvac, kernel, material, meters, metrics, model, … 50 total) sitting directly at plugin root, not under any artifact. → `🗿️artifacts/energy/🏅️standards/🔖️1/⚙️engine/<module>/` for each of the 50 submodules, i.e. exactly the taxonomy's own `standardChildDirs: ["⚙️engine","🪆️subsets"]` shape one level down from where it sits today. | high confidence on destination pattern; high effort — 50 submodules to relocate |
| 🌳️ast, 🔤️lexer, 🗣️language-service, 🧮️executor | 🔱️trinity | each is a single-file compute module (parser AST, lexer, LSP-shaped language service, executor) for trinity's own DSL → `🗿️artifacts/<trinity-artifact>/🏅️standards/🔖️1/⚙️engine/{ast,lexer,language-service,executor}/`; trinity's own root doc names the plugin as language-tooling for "s"/"jack"/"wire" DSLs, consistent with folding under one shared trinity artifact's engine | high |
| 🔨️modules (trinity, 23 files, includes `🔌️jack/🐚️shell`, `🔌️jack/🧠️lsp`) | 🔱️trinity | same engine-relocation pattern as above, per-sub-language (`jack` has a shell + LSP) → `🗿️artifacts/<kind>/🏅️standards/…/⚙️engine/jack/{shell,lsp}/` | medium — needs the artifact-kind naming decision trinity's owner would make |
| 🔄️fsm (draw, 10 files incl. a `✨️macros` sub-crate + its own `📦️packages/🦀️rust/`) | 🖍️draw | a nested **second crate** (macros) inside a plugin-root extra dir — this is its own crate boundary, not just a folder of code → likely wants to become its own `🗿️artifacts/draw/🏅️standards/🔖️1/⚙️engine/fsm/` with the macro crate re-homed under that engine's own `📦️packages/🦀️rust/` (macro crates commonly need their own Cargo.toml — verify with a crate-boundary specialist, not a plain directory move) | medium — crate-boundary nuance flagged |
| 📇️registry (stdio, 1 file: `📇️catalog.json`) | 🗄️stdio | → `🗿️artifacts/stdio/📚️examples/` or a registry-specific slot; taxonomy has a generic `rootDataDirNames` entry `📇️registry` at **owner root** (see taxonomy line ~401) which may make this *already* taxonomy-legal as an owner-root data dir, not a violation — **recheck against `rootDataDirNames` before flagging as a breach** | needs re-check — may be a false positive in this census |
| 🔨️modules (puzzle, 1 file: `🎲️board-2d/🦀️component.rs`), 🧫️fixtures (puzzle, 4 files) | 🧩️puzzle | `🔨️modules/🎲️board-2d` → `🗿️artifacts/puzzle2d/🏅️standards/🔖️1/⚙️engine/board-2d/` (own doc-comment says it's a wasm session wrapper for puzzle2d's engine, moved out of a generic framework module); `🧫️fixtures` → `🗿️artifacts/<kind>/📚️examples/…` | high for 🔨️modules; medium for 🧫️fixtures (not enumerated) |
| 🧩️extensions (sourcing, 3) | 🪵️sourcing | extension-crate axis | needs owner ruling |

**Cross-cutting flag:** several rows above hit the same unresolved question — *where does code shared across ALL of a plugin's sibling apps (norm's 15 apps, fem's 2 apps) live under APA?* Neither `🎛️apps/<app>/` (too narrow) nor `🗿️artifacts/` (wrong axis — this is UI/app-surface code, not artifact schema/engine/io) fits cleanly, and `bannedNameStems` rules out the obvious `core`/`shared`/`common` escape hatch. This needs an explicit decision from whoever owns the APA spec before wave 3 (mass migration) touches `🏗️fem/🖥️app-surface`, `📕️norm/🖥️app-surface`, or `📕️norm/📄️artifact`.

**Extension-crate axis flag (recurring across 🌊️flow, 🏭️process, 📐️cad, 📖️playbook, 📜️imperative, 🪵️sourcing):** all `🧩️extensions/` dirs hold `role="extension"` crates that `extends` their owner plugin (see §6). Whether `🧩️extensions/` itself counts as an APA violation (i.e. must fold into `🗿️artifacts/…`) or is a sanctioned third axis alongside apps/artifacts is NOT decided by this census — flagging as a single open question rather than repeating it per-plugin.

---

## 6. The 26 extension crates (`role = "extension"`)

`grep -rl 'role = "extension"' --include='Cargo.toml' .` finds **29** hits repo-wide; **3 are ticket scratch** under `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/SOURCING-PLUGIN-EXTENSIONS-DE-SANDWICH/{verify-beams,verify-slabs,verify-windows}/Cargo.toml` (not live plugin code — exclude). The remaining **26** all live at `✏️s/🔌️plugins/<owner>/🧩️extensions/<name>/📦️packages/🦀️rust/Cargo.toml`:

| owner plugin | extension count | extension names |
|---|---|---|
| 🌊️flow | 9 | 🏗️bim, 📃️list, 📐️brep, 📖️dictionary, 📝️text, 🔤️primitive, 🖍️draw, 🧠️logic, 🧮️math |
| 📜️imperative | 5 | 🎮️control, 📝️text, 📣️effect, 🧠️logic, 🧮️math |
| 🏭️process | 4 | 🔩️metal, 🤖️robotic, 🧱️concrete, 🪵️wood |
| 📐️cad | 4 | 🏛️aec-building-structure, 🏢️aec-building, 📐️spatial-shape, 🔥️aec-building-energy |
| 🪵️sourcing | 3 | 🧱️slabs, 🪟️windows, 🪵️beams |
| 📖️playbook | 1 | 🌀️procedural |
| **total** | **26** | — |

Every crate's `[package.metadata.semio]` block carries `role = "extension"` plus `extends = "<owner-slug>"` matching its containing plugin exactly (`flow`→🌊️flow, `process`→🏭️process, `cad`→📐️cad, `playbook`→📖️playbook, `imperative`→📜️imperative, `sourcing`→🪵️sourcing) — confirmed by direct grep of every one of the 26 `Cargo.toml` files, not sampled. Most also carry `contributes = [...]` (e.g. `["flow.extension"]`, `["process.machines"]`, `["cad.computer"]`, `["playbook.blockKind"]`, `["imperative.module"]`) naming the extension point on the host plugin they populate.

These crates already live inside their owner's own plugin directory (`<owner>/🧩️extensions/<name>/`), so structurally they are additional plugin-root children beyond `{apps, artifacts, packages}` in exactly the same way as every other row in §5's table — see the "extension-crate axis" flag there for the open policy question.

---

## 7. Taxonomy verbatim + all hardcode sites

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (schemaVersion 2).

```json
"pluginChildDirs": [
    "🛂️manifest",
    "🎟️capabilities",
    "🔧️setup",
    "🎛️apps"
  ],
```
```json
"artifactsDirName": "🗿️artifacts",
```
```json
"appChildDirs": [
    "⚙️engine",
    "🎭️modes",
    "🎮️commands",
    "📌️panels",
    "🎚️config",
    "👥️presence",
    "🗣️terminology",
    "🌉️wasm",
    "📚️examples"
  ],
```
```json
"artifactChildDirs": [
    "🧬️schema",
    "⚙️engine",
    "🚪️io",
    "📚️examples"
  ],
```
```json
"standardChildDirs": [
    "⚙️engine",
    "🪆️subsets"
  ],
```
```json
"subsetChildDirs": [
    "🧬️schema",
    "🚪️io"
  ],
```

No key named `🔧️setup`, `🛂️manifest`, or `🎟️capabilities` exists anywhere else in taxonomy.json as a *value* inside these arrays other than `pluginChildDirs` itself — i.e. taxonomy.json has exactly one place that names the three facets (the array quoted at the top of this section), which is also the array §1 shows is actively enforced by three separate live call sites.

Also present but not directly asked for, included for completeness since it bears on §5's classification of `🎪️panes`/`🖥️app-surface`/config-presence questions — `appComponentDirs` (subset of appChildDirs used for completeness checks): `["⚙️engine", "🎮️commands", "🎚️config", "👥️presence"]`; and the `newArtifactChildDirs`/`newArtifactComponentDirs` pair (`["🏅️standards","📚️examples"]` / `["🏅️standards"]`) which is the *current* target shape one level under `artifactsDirName` — i.e. taxonomy.json already has a "new" artifact shape (standards+examples) sitting alongside the "old" flat shape (schema+engine+io+examples) in `artifactChildDirs`. Which one is authoritative for APA wave 3 needs an explicit call from the ticket owner; this census does not adjudicate it.

### Complete hardcode site list (file:line) for the three facet name literals, live code only (ticket-scratch and homonym hits excluded — see §1 for the homonym explanation)

| file | line(s) | anchor | role |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | 376-381 | `"pluginChildDirs": [` | SSOT declaration |
| `📜️script.ts` (repo root) | 4646-4648, ~4666-4669 | `function policyPluginRootShapeBreaches(repoRoot: string): BreachRecord[] {` | enforcing policy (soft breach report) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` | 1224, 1232-1236 | `const pluginChildDirs = TAXONOMY.pluginChildDirs;` | enforcing policy (registry codegen findings) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | 2226, 2229-2235 | `let plugin_child_dirs = string_array(&taxonomy, "pluginChildDirs");` | **hard `assert!` gate — breaks build if facets removed without updating this** |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` | 174, 582, 583, 585, 586 | `readonly pluginChildDirs: readonly string[];` | type + `validateTaxonomy` shape checks |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` | 1148, 1290, 1292, 1298 | `expect(taxonomy.pluginChildDirs).toEqual(["🛂️manifest", "🎟️capabilities", "🔧️setup", "🎛️apps"]);` | unit test pinning the exact array literal |

---

## Summary of files touched this wave

Read-only census. Only file written: this report,
`/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w0-b-plugin-shape.md`.
