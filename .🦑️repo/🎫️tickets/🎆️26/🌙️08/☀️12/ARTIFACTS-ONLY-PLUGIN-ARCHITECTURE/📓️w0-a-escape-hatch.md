# W0-A — OS-Host Registry Escape-Hatch Family: Census

Scope: the `register_*` global-static family declared in the two OS kernel `component.rs` files, every call
site of that family repo-wide, cross-ownership of `artifact_kind` strings, and the `MeshExporter`/`MeshImporter`
trait+struct set in the plugin SDK. Read-only recon — no source files touched. Repo root: `/Users/ueli/Documents/semio`.

Grep commands used are reproduced inline so a later agent can re-run them; line numbers are a snapshot as of
this census (2026-08-12) and WILL drift — every row below also carries a grep-able anchor string.

---

## 1. The `register_*` family — definitions in the two files

Files:
- ROOT = `🧰️framework/🛍️products/💻️os/🦀️component.rs` (4425 lines) — `pub mod host { ... }` is **unconditional**.
- HOST = `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (4534 lines) — `pub mod host { ... }` is gated
  `#[cfg(feature = "os-host-full")]` (line 6). This file additionally carries a second, **stub** copy of the
  media-handler registrars gated `#[cfg(not(feature = "os-host-full"))]` inside `pub mod workflow` (itself gated
  `#[cfg(feature = "os-host-full")]` at line 2971/2972) — i.e. the stub block (lines 2576-2611) sits inside a
  module that only compiles when `os-host-full` is *on*, so `#[cfg(not(feature = "os-host-full"))]` inside it is
  never true. **Dead code**, not a live second implementation — flagging as UNVERIFIED-INTENT (may be a
  copy/paste leftover from when `workflow` was reachable under both feature states; not in scope to fix here).

Command: `grep -n "fn register_" <file>`

| Function | ROOT line | HOST line (dead stub / live) | Signature | Global static mutated | Anchor |
|---|---|---|---|---|---|
| `register_os_fixture_json` | 2292 | 2300 | `pub fn register_os_fixture_json(slug: &str, json: &str)` | `OS_FIXTURE_JSON: OnceLock<Mutex<HashMap<String,String>>>` | `fn register_os_fixture_json(slug` |
| `register_2d_export_handlers` | 2675 | dead:2765 (live copy is the same body, see below) | `pub fn register_2d_export_handlers(artifact_kind: &'static str, file_stem: &'static str, document_to_svg: Svg2dDocumentRenderer)` | none directly — calls `register_os_media_export_handler_kind` 3× (svg/png/dwg) | `fn register_2d_export_handlers(artifact_kind` |
| `register_dwg_import_handler` | 2693 | 2783 | `pub fn register_dwg_import_handler(artifact_kind: &'static str, from_dwg: fn(&DwgDrawing) -> Result<Value, String>)` | none directly — calls `register_os_media_import_handler_kind` | `fn register_dwg_import_handler(artifact_kind` |
| `register_mesh_exporter` | 2701 | 2791 | `pub fn register_mesh_exporter(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<MeshData, String>, exporter: Box<dyn MeshExporter>)` | none directly — calls `register_os_media_export_handler_kind` | `fn register_mesh_exporter(artifact_kind` |
| `register_mesh_importer` | 2716 | 2806 | `pub fn register_mesh_importer(artifact_kind: &'static str, document_from_mesh: fn(&MeshData) -> Result<Value, String>, importer: Box<dyn MeshImporter>)` | none directly — calls `register_os_media_import_handler_kind` | `fn register_mesh_importer(artifact_kind` |
| `register_mesh_dwg_import_handler` | 2725 | 2815 | `pub fn register_mesh_dwg_import_handler(artifact_kind: &'static str, document_from_mesh: fn(&MeshData) -> Result<Value, String>)` | none directly — calls `register_os_media_import_handler_kind` | `fn register_mesh_dwg_import_handler(artifact_kind` |
| `register_mesh_dwg_export_handler` | 2734 | 2824 | `pub fn register_mesh_dwg_export_handler(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<MeshData, String>)` | none directly — calls `register_os_media_export_handler_kind` | `fn register_mesh_dwg_export_handler(artifact_kind` |
| `register_solid_exporter` | 2763 | 2853 | `pub fn register_solid_exporter(artifact_kind: &str, exporter: Box<dyn SolidExporter>)` | `solid_exporters()` → `OnceLock<Mutex<SolidExporterRegistry>>` (`HashMap<String, Box<dyn SolidExporter>>`) | `fn register_solid_exporter(artifact_kind` |
| `register_solid_importer` | 2769 | 2859 | `pub fn register_solid_importer(artifact_kind: &str, importer: Box<dyn SolidImporter>)` | `solid_importers()` → `OnceLock<Mutex<SolidImporterRegistry>>` | `fn register_solid_importer(artifact_kind` |
| `register_os_media_export_handler_kind` (live) | 3351 | 3457 | `pub fn register_os_media_export_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static)` | `export_handlers()` → `OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>>`, key = `"{artifact_kind}:{format_artifact_kind}"` | `fn register_os_media_export_handler_kind(` |
| `register_os_media_export_handler_kind` (dead stub) | — | 2588-2598 | same signature, `#[cfg(not(feature = "os-host-full"))]` inside an `os-host-full`-only module | `OS_MEDIA_EXPORT_HANDLERS: LazyLock<Mutex<HashMap<(String,String), _>>>` — separate static from the live one, never reachable | `🔖️MediaRegistryRegistryStubs` |
| `register_os_media_import_handler_kind` (live) | 3427 | 3525 | `pub fn register_os_media_import_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static)` | `import_handlers()` → `OnceLock<Mutex<HashMap<String, OsMediaImportHandler>>>` | `fn register_os_media_import_handler_kind(` |
| `register_os_media_import_handler_kind` (dead stub) | — | 2600-2610 | same as above, dead | `OS_MEDIA_IMPORT_HANDLERS: LazyLock<Mutex<...>>`, never reachable | (same region) |
| `register_artifact_descriptors` | 4036 | 4135 | `pub fn register_artifact_descriptors(manifest: &PluginManifest)` | `RESOURCE_KIND_REGISTRY: LazyLock<Mutex<HashMap<OsArtifactKindId, ArtifactKindEntry>>>` | `fn register_artifact_descriptors(manifest` |
| `register_artifact_descriptor` | 4050 | 4149 | `pub fn register_artifact_descriptor(spec: &ArtifactKindSpec)` | same `RESOURCE_KIND_REGISTRY` | `fn register_artifact_descriptor(spec` |
| `register_app_io` | 4137 | 4236 | `pub fn register_app_io(plugin_id: &str, app: &AppDefinition)` | `APP_REGISTRATIONS: LazyLock<Mutex<HashMap<(String,String), OsAppRegistration>>>` | `fn register_app_io(plugin_id: &str, app: &AppDefinition)` |

Sibling found but **not** part of this global-static family — `register_app(&mut self, app: AppDefinition)` at
ROOT:4304 / HOST:4403 is a plain `&mut self` method on `PluginRegistry` (`self.apps.insert(...)`), not a
free function mutating process-global state. Out of scope for the census.

### Byte-identity proof

`diff <(sed -n '2671,2778p' ROOT) <(sed -n '2761,2868p' HOST)` → **empty diff, IDENTICAL** — covers
`register_2d_export_handlers` through `register_solid_importer` inclusive, doc comments included.

`diff` of the `register_os_media_export_handler_kind`/`register_os_media_import_handler_kind` **live** bodies
(ROOT:3351-3466 vs HOST:3457-3572) and of `register_artifact_descriptors`/`register_artifact_descriptor`/
`register_app_io` (ROOT:4030-4160 vs HOST:4125-4260, read side by side above) — also byte-identical.

**Conclusion: the two files are hand-maintained mirrors for this entire function family** — every live
function, docstring, and helper is character-for-character duplicated between ROOT and HOST. The only
structural difference is HOST's extra dead `#[cfg(not(feature = "os-host-full"))]` stub duplicate of the two
media-handler registrars (never compiles alongside the live one), and HOST gating its whole `pub mod host`/
`pub mod workflow` behind `os-host-full` where ROOT does not gate them at all.

---

## 2. Every call site of the family, repo-wide

Command: `rg -n --type rust "register_mesh_exporter\(|register_mesh_importer\(|register_mesh_dwg_export_handler\(|register_mesh_dwg_import_handler\(|register_solid_exporter\(|register_solid_importer\(|register_2d_export_handlers\(|register_dwg_import_handler\(|register_app_io\(|register_os_media_export_handler_kind\(|register_os_media_import_handler_kind\(|register_artifact_descriptors\(|register_artifact_descriptor\(" .`
→ **137 matches total**; 28 are the `pub fn register_...` definition lines themselves (14 distinct names × 2
files, tabulated in §1); **109 are real call sites**. Of those, **40 are internal to ROOT/HOST themselves**
(each function's own body calling into `register_os_media_*_handler_kind`, `PluginHost::load_plugin`/
`hot_swap_plugin` calling `register_app_io`/`register_artifact_descriptors` in production, plus each file's
own `#[cfg(test)]` fixtures) and **69 are external, repo-wide, outside the two definition files** — tabulated
below. `register_os_fixture_json` was checked separately (not in the regex above): 2 more external call sites.

Command for external sites: `awk -F: '{print $1}' <hits> | grep -v "🛍️products/💻️os" | sort | uniq -c`

### 2a. Compliant call sites (inside a plugin's own artifact `⚙️engine`)

| File | Line | Fn called | `artifact_kind` literal | Anchor |
|---|---|---|---|---|
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` | 44 | `register_mesh_dwg_import_handler` | `"3d.process"` | `register_mesh_dwg_import_handler("3d.process"` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` | 649 | `register_mesh_dwg_import_handler` | `"3d.procedural"` | `register_mesh_dwg_import_handler("3d.procedural"` |

These two are the **only** call sites anywhere in the repo that live inside the owning artifact's own
`⚙️engine` module — everything else below is a violation.

### 2b. Violating call sites — plugin `🔧️setup` facet

| File | Lines | Fn(s) called | `artifact_kind` literal(s) | Anchor |
|---|---|---|---|---|
| `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs` | 10-17 (8 calls) | `register_mesh_exporter`×3, `register_mesh_dwg_export_handler`, `register_mesh_importer`×3, `register_mesh_dwg_import_handler` | `"3d.lowpoly"` (own kind) | `register_lowpoly_exports` |
| same file | 18-24 (7 calls) | `register_mesh_exporter`×3 (Obj/Glb/Stl), `register_mesh_dwg_export_handler`, `register_mesh_importer`×2 (Obj/Glb **only** — no `StlImporter` registered for this kind), `register_mesh_dwg_import_handler` | `"3d.mesh"` (**not** this plugin's declared `artifact_kind()` — see §3) | `register_mesh_exporter("3d.mesh"` |

Full 15-call breakdown at lines 10,11,12,13,14,15,16,17,18,19,20,21,22,23,24 inside `pub fn
register_lowpoly_exports()`.

### 2c. Violating call sites — app/pane/command/panel files (same-plugin, wrong layer)

| File | Line | Fn | `artifact_kind` | Owning artifact | Anchor |
|---|---|---|---|---|---|
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🧭️navigation/🦀️component.rs` | 78 | `register_app_io` | (per-app, from `entry.app`) | n/a (app registration, not artifact_kind) | `fn apply_app_registrations` — **production**, not gated by `#[cfg(test)]` (test mod starts line 102) |
| `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` | 1366-1367 | `register_2d_export_handlers`, `register_dwg_import_handler` | `"2d.puzzle"` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs:420` (same plugin) | `register_puzzle2d_exports` — doc comment says "Called by the plugin `setup:` hook" but lives in the app file, not `🗿️artifacts/…/⚙️engine` |
| `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs` | 2004-2011 (8 calls) | `register_mesh_exporter`×3, `register_mesh_importer`×3, `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler` | `"5d.puzzle"` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs:520` (same plugin) | `"5d.puzzle"` |
| `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs` | 2686-2693 (8 calls) | same set | `"3d.puzzle"` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️component.rs:504` (same plugin) | `"3d.puzzle"` |

Also **test-only, not production** (verified each sits after that file's first `#[cfg(test)]`/`mod tests`
line — excluded from the violation count but listed for completeness):
`✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🔗️connections/🦀️component.rs:64,78,110,124`
(`register_artifact_descriptor` on synthetic `test.contract.*` kinds inside `#[cfg(test)] mod tests` at line 48);
`✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs:121,126` (inside `#[cfg(test)] mod
tests` at line 103); `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs:605` (inside `#[cfg(test)] pub(crate)
mod testkit` at line 554); `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs:120`
(inside `#[cfg(test)] mod tests` at line 108).

Also: `✏️s/🔌️plugins/🪐️space/🦀️component.rs:30-31` — `register_os_fixture_json` called from
`ensure_space_fixtures_registered()`, the plugin-root `🦀️component.rs` (not `🔧️setup`, not an artifact
`⚙️engine`) — production code, sibling family member.

### 2d. Violating call sites — cross-plugin god-pane (`🎪️demonstrator`), the worst offender class

`🎪️demonstrator` registers exporters/importers for **four artifact kinds it does not declare**, each owned by
a wholly separate plugin. None of these four calls sit behind any `#[cfg(test)]` — all four blocks are plain
production `pub fn register_exports()`/equivalent functions.

| File | Lines | Fn(s) | `artifact_kind` | True owner (declares `id: "<kind>"`) | Anchor |
|---|---|---|---|---|---|
| `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🏭️bearbeiten/🦀️component.rs` | 34-38 (5 calls) | `register_mesh_exporter`×3, `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler` | `"3d.process"` (`PROCESS_3D_KIND`, const at line 15) | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:511` — plugin `🏭️process` | `pub fn register_exports()` |
| `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🌱️generator/🦀️component.rs` | 20-27 (8 calls) | `register_mesh_exporter`×3, `register_mesh_dwg_export_handler`, `register_mesh_importer`×3, `register_mesh_dwg_import_handler` | `"3d.procedural"` (`PROCEDURAL_3D_KIND`, const at line 14) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs:37` — plugin `🌀️procedural` | `PROCEDURAL_3D_KIND` |
| `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🗺️verfolgen/🦀️component.rs` | 19-20 (2 calls) | `register_2d_export_handlers`, `register_dwg_import_handler` | `"2d.map"` (`GIS_MAP_KIND`, const at line 13) | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs:61` — plugin `🌍️gis` | `GIS_MAP_KIND` |
| `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs` | 20-29 (10 calls) | `register_solid_exporter`×3, `register_solid_importer`×3, `register_mesh_exporter`, `register_mesh_importer`, `register_mesh_dwg_export_handler`, `register_dwg_import_handler` | `"3d.cad"` (`CAD_KIND`, const at line 13) | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs:379` — plugin `📐️cad` | `CAD_KIND` |

**Severity split within this class, verified by grep — no self-registration exists in the owning plugin for
two of the four kinds:**
- `rg -n "register_(2d_export_handlers|dwg_import_handler|mesh_exporter|mesh_importer|mesh_dwg_export_handler|mesh_dwg_import_handler|solid_exporter|solid_importer|os_media_export_handler_kind|os_media_import_handler_kind)\(" --type rust "✏️s/🔌️plugins/🌍️gis/"` → **zero matches**. Plugin `🌍️gis` declares `"2d.map"` but registers **no** io handler for it anywhere in its own tree — `🎪️demonstrator` is the *sole* registrant. Without the demonstrator plugin loaded, `2d.map` export/import silently has no handler despite being declared.
- Same command against `"✏️s/🔌️plugins/📐️cad/"` → **zero matches**. Same situation for `"3d.cad"`.
- `"3d.process"` and `"3d.procedural"` **are** self-registered by their owning plugins too (§2a) — so for
  these two, the kind is registered from **both** the owning artifact engine (once) and the demonstrator pane
  (3-5 times, all formats) — last-writer-wins on the shared `HashMap`, i.e. plugin load order silently decides
  whose DWG-import closure answers for `"3d.process"`/`"3d.procedural"`. This is a correctness risk beyond the
  architecture violation, not just a layering complaint.

---

## 3. Cross-ownership — who registers a kind they don't own

`"3d.mesh"` specifically (per assignment):

- **Declared** (`ArtifactKindSpec { id: "3d.mesh", .. }`) at `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs:265`, inside `pub fn mesh_artifact_kind()`, whose docstring reads (verbatim, one line, under-15-word excerpt): *"declared alongside `artifact_kind()` because several sibling plugins declare the identical shape privately"*. This function lives in the **same file** as `pub fn artifact_kind()` (line 242) which declares the plugin's real own kind, `"3d.lowpoly"`.
- **Registered** (all 7 exporter/importer/dwg calls) exclusively from `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs:18-24`, i.e. the same plugin (`💠️lowpoly`) that declares it — so `"3d.mesh"` is *not* cross-plugin-registered by a foreign plugin's setup facet. It is, however, structurally anomalous: `💠️lowpoly` is bundling a second, nominally-shared/generic artifact kind (`"3d.mesh"`, component_kind `"mesh"`, schema `"mesh.reference"`) into its own `🗿️artifacts/💠️lowpoly` folder and its own `🔧️setup` facet rather than that kind having a dedicated `🗿️artifacts/…mesh…` folder of its own. Four other plugins (`✏️s/🔌️plugins/📸️remodel`, `🌍️gis`, `🌀️procedural`) tag their own engine output with `schema: "3d.mesh"` as a *consumer* of the shared interchange shape (`remodel/🎛️apps/📸️remodel/🦀️component.rs:359`, `gis/🗿️artifacts/🏔️gisterrain/…component.rs:218`, `procedural/🎛️apps/🧊️3d/🦀️component.rs:152`) but none of them call any `register_*` for `"3d.mesh"` themselves — `💠️lowpoly` is the sole registrant AND the sole implementer of the interchange codec on behalf of every consumer. **Verdict: `"3d.mesh"` is architecturally ownerless under APA — it needs its own dedicated artifact (its own `🚪️io` tree), not a second `ArtifactKindSpec` grafted onto `💠️lowpoly`'s.**

Broader cross-ownership sweep (kinds registered by a plugin other than the one that declares `id: "<kind>"`)
— the four `🎪️demonstrator` cases in §2d are the clear-cut instances, confirmed by dedicated greps:

| Kind | Declared by (owner) | Registered by (foreign) | Owner self-registers too? |
|---|---|---|---|
| `"3d.process"` | `🏭️process` (`process3d/🦀️component.rs:511`) | `🎪️demonstrator/🎪️panes/🏭️bearbeiten` | Yes (§2a) — double-write |
| `"3d.procedural"` | `🌀️procedural` (`procedural3d/🦀️component.rs:37`) | `🎪️demonstrator/🎪️panes/🌱️generator` | Yes (§2a) — double-write |
| `"2d.map"` | `🌍️gis` (`gismap/🦀️component.rs:61`) | `🎪️demonstrator/🎪️panes/🗺️verfolgen` | **No** — foreign-only |
| `"3d.cad"` | `📐️cad` (`cad/🦀️component.rs:379`) | `🎪️demonstrator/🎪️panes/📐️koordinator` | **No** — foreign-only |
| `"3d.mesh"` | nominally `💠️lowpoly` (co-declared, see above) | `💠️lowpoly` itself | Same plugin — ownerless-by-design rather than foreign-registered |

---

## 4. `MeshExporter`/`MeshImporter`/format-struct set in the plugin SDK

All defined in **one file**: `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, region `//#region MeshCodec`
(lines ~811-882). Re-exported into the plugin-facing surface at `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:50-55`
(`pub use mesh::{ … MeshExporter, MeshImporter, ObjExporter, ObjImporter, GlbExporter, GlbImporter,
StlExporter, StlImporter, … }`), which is what plugin code imports as `semio_framework_plugin::MeshExporter`
etc.

| Item | Line | Kind | Implements |
|---|---|---|---|
| `trait MeshExporter: Send + Sync` | 814 | trait — `fn format_kind(&self) -> &'static str; fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;` | — |
| `trait MeshImporter: Send + Sync` | 820 | trait — `fn format_kind(&self) -> &'static str; fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;` | — |
| `struct ObjExporter` / `impl MeshExporter for ObjExporter` | 825 / 826 | struct + impl | `format_kind() == "obj"` |
| `struct ObjImporter` / `impl MeshImporter for ObjImporter` | 835 / 836 | struct + impl | `format_kind() == "obj"` |
| `struct GlbExporter` / `impl MeshExporter for GlbExporter` | 846 / 847 | struct + impl | `format_kind() == "glb"` |
| `struct GlbImporter` / `impl MeshImporter for GlbImporter` | 856 / 857 | struct + impl | `format_kind() == "glb"` |
| `struct StlExporter` / `impl MeshExporter for StlExporter` | 866 / 867 | struct + impl | `format_kind() == "stl"` |
| `struct StlImporter` / `impl MeshImporter for StlImporter` | 876 / 877 | struct + impl | `format_kind() == "stl"` |

**Implementor count: exactly 3 `MeshExporter` impls (Obj/Glb/Stl) and 3 `MeshImporter` impls (Obj/Glb/Stl) —
6 total, all in this one file, all framework-owned. No plugin anywhere in the repo defines its own
`MeshExporter`/`MeshImporter` impl** — confirmed via `rg -n "impl\s+(\w+::)*MeshExporter\s+for|impl\s+(\w+::)*MeshImporter\s+for" --type rust .` returning exactly these 6 hits.

Bonus (not explicitly requested but directly adjacent — the `SolidExporter`/`SolidImporter` half of
`register_solid_exporter`/`register_solid_importer`): defined+implemented entirely in
`🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs:1238-1316` — `trait SolidExporter`/`trait
SolidImporter` plus 4 format pairs (`Step`/`Stl`/`Obj`/`Glb` × Solid{Exporter,Importer}) = 8 structs, all
framework-owned, same one-file pattern as the mesh family.

---

## 5. Artifact-native equivalent per violating call site

| Violating call site | Artifact-native form |
|---|---|
| `💠️lowpoly/🔧️setup` (§2b, `"3d.lowpoly"` calls) | A `ComposerEntry` row (export leaf) + deserializer leaf under `💠️lowpoly`'s own artifact `🚪️io` tree, invoked by `💠️lowpoly`'s `⚙️engine`'s own registration hook — not the plugin's `🔧️setup` facet. |
| `💠️lowpoly/🔧️setup` (§2b, `"3d.mesh"` calls) | A dedicated `mesh` artifact (new top-level artifact, own `🗿️artifacts/…mesh…` folder, own `🏅️standards/🔖️1/⚙️engine`) with its own `🚪️io` tree owning the Obj/Glb/Stl serializer + deserializer leaves — not grafted onto lowpoly's. |
| `🪐️space/…/🧭️navigation/🦀️component.rs:78` (`register_app_io`) | An app-registration `ComposerEntry`-equivalent driven declaratively from the `AppDefinition`/`PluginManifest` at plugin-load time inside the app's own artifact `⚙️engine` registration path, not imperatively from a command handler. |
| `🧩️puzzle` app files (§2c, `2d.puzzle`/`5d.puzzle`/`3d.puzzle`) | Move each block verbatim into that same kind's `🗿️artifacts/<kind>/🏅️standards/🔖️1/⚙️engine` registration function (the plugin already owns the kind — this is a same-plugin relocation, not a new artifact). |
| `🎪️demonstrator/…/🏭️bearbeiten` (`"3d.process"`) | Delete from demonstrator; a serializer/deserializer leaf under `process3d`'s own `🚪️io` tree (the `document_from_mesh`/`mesh_from_document` bridge functions move with it, owned by `🏭️process`). |
| `🎪️demonstrator/…/🌱️generator` (`"3d.procedural"`) | Delete from demonstrator; leaf under `procedural3d`'s own `🚪️io` tree, owned by `🌀️procedural`. |
| `🎪️demonstrator/…/🗺️verfolgen` (`"2d.map"`) | Delete from demonstrator; leaf under `gismap`'s own `🚪️io` tree, owned by `🌍️gis` — currently `🌍️gis` has **no** io registration of its own, so this is new code for that plugin, not a move of existing code. |
| `🎪️demonstrator/…/📐️koordinator` (`"3d.cad"`) | Delete from demonstrator; leaf(s) under `cad`'s own `🚪️io` tree, owned by `📐️cad` — same "no existing self-registration" gap as gismap. |

---

## Appendix — raw grep evidence files (ticket-folder scratch, not deleted)

- `/private/tmp/claude-501/-Users-ueli-Documents-semio/5128c8d3-abfa-49da-81ac-33286ba73278/scratchpad/all_callsites.txt` — full 137-line raw hit list backing §2 (session scratchpad, not ticket-persisted; re-run the command in the header if this file is gone by the time you read this).

## UNVERIFIED

- Whether the HOST-file dead `#[cfg(not(feature = "os-host-full"))]` stub registrars (§1) are truly
  unreachable in every build configuration this repo ships (checked only the two `#[cfg]` conditions
  textually — did not enumerate the full Cargo feature-unification graph across every crate that depends on
  this one with `os-host-full` off).
- Whether any WASM/`wasm32` build target excludes some of the puzzle/lowpoly production call sites via a
  target-cfg not visible to a text grep (`◻2d`'s `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`
  guard at line 1363 was seen and is noted; did not check the other files for similar guards beyond what's
  shown above).
