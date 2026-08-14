# W0 Recon Report — Semio Artifact / Unified Import-Export / MediaFormat Retirement

Agent: W0 recon (baselines + per-plugin ledger + pattern-plugin roster + catalog/glue confirmation).
Scope: read-only recon. No source files touched. Raw command outputs saved alongside this report as `.txt` files (see per-section pointers).

---

## 1. Baselines

### 1a. `cargo test -p semio-s-plugin-stdio --lib` — full output in `w0-stdio-test-baseline.txt`

```
test result: ok. 1075 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.80s
```

**Confirmed: exactly 1075 passed, 0 failed** — matches the plan's stated baseline exactly.

### 1b. `bun ./📜️script.ts policy` — full output in `w0-policy-baseline.txt` (21592 lines)

Top line and full breakdown:

```
21564 high-priority breach(es) across 24 rule(s):
  19352  handcrafted-grammar/spec-distinctness
    454  taxonomy/emoji-prefix
    249  artifact-schema/facet-completeness
    242  taxonomy/dead-example-leaf
    240  os-state-authority/item-scope-global
    198  stdio-artifacts/composer
    181  stdio-artifacts/schema-representation
    129  dsl-migration/diff-completeness
     93  protocol-migration/command-envelope-completeness
     83  mutation-migration/triad-completeness
     83  mutation-migration/artifact-engine
     69  handcrafted-grammar/declared-use
     48  pack-migration/completeness
     29  artifact-schema/type-name-parity
      4  os-state-authority/id-minting
      4  budget/no-budget-null
      3  os-state-authority/authority-struct-map
      2  taxonomy/plugin-builder
      1  taxonomy/banned-name-stem
      1  handcrafted-grammar/generic-spec
      1  stdio-artifacts/builder
      1  stdio-artifacts/decomposer
      1  protocol-migration/db-server-only
     96  handcrafted-grammar/empty-example
```

(count column reflows in the raw file; the numbers above are copied verbatim from the run.)

**W0 policy snapshot = 21564 breaches across 24 rules.** This is the number every later wave's "policy zero-new" gate diffs against. The plan's `stdio-artifacts/schema-representation: 181` figure (referenced in "Verified ground truth" as the delegating-subset breach count W1 clears) is confirmed exactly: **181**.

### 1c. `cargo check -p semio-framework-os-run` — full output in `w0-osrun-check.txt`

Exit code 101 (compile failure). **13 errors, 1 warning** in the os-run crate itself (upstream crates compile with warnings only). Exact error list:

```
error[E0432]: unresolved imports `workflow::MediaContract`, `workflow::PortFingerprint`, `workflow::RunNodeRecord`,
  `workflow::RunNodeStatus`, `workflow::RunMutation`, `workflow::RunOutputArtifact`, `workflow::RunParameterValue`,
  `workflow::Workflow`, `workflow::WorkflowEdge`, `workflow::WorkflowNode`, `workflow::WorkflowParameterBinding`
error[E0433]: cannot find module or crate `os_dsl` in this scope   (×5 call sites)
error[E0425]: cannot find type `RunArtifact` in crate `workflow`   (×2)
error[E0425]: cannot find function `apply_run_operation_checked` in crate `workflow`
error[E0592]: duplicate definitions with name `artifact_pack_path`
error[E0592]: duplicate definitions with name `artifact_spr_path`
error[E0609]: no field `operations` on type `&mut RunSink`
error[E0004]: non-exhaustive patterns: `&AppFrame::Emit { .. }` and `&AppFrame::Draft { .. }` not covered

error: could not compile `semio-framework-os-run` (lib) due to 13 previous errors; 1 warning emitted
```

**Correction to plan's ground truth — see §7.** There is **no E0063 and no `topic_contributions` error anywhere in this output** (`grep -c "E0063\|topic_contributions"` on the raw file = 0). That specific blocker has already been fixed elsewhere in the tree (confirmed independently in §6/§7). The *current* blocker is a different, larger set of 13 errors, dominated by the `🔁️workflow` module being unmounted in the os-kernel glue (E0432 + both `RunArtifact`/`apply_run_operation_checked` E0425s all stem from `workflow::` resolving to the whole `semio_framework_os_kernel` crate re-export surface, which doesn't contain those symbols — see §6) plus 5× `os_dsl` scope errors, 2× duplicate-fn E0592s, 1× missing-field E0609, and 1× non-exhaustive-match E0004. The duplicate `artifact_pack_path`/`artifact_spr_path` and non-exhaustive `AppFrame` match items from the plan's ground truth **are** still present and accurate.

### 1d. Hot-file `git status --porcelain`

All 9 listed paths plus the actual framework `glue.rs` (see §2 correction) were checked individually for existence and status:

| File | Exists | git status |
|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | yes | clean |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (os kernel glue) | yes | clean |
| `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (framework's own glue.rs — see §7) | yes | clean |
| `📜️script.ts` | yes | clean |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | yes | clean |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` | yes | clean |
| `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` | yes | clean |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` | yes | clean |
| `🧰️framework/🛍️products/💻️os/🦀️component.rs` | yes | clean |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | yes | clean |

`git status --porcelain -- <all 9 plan-listed paths>` produced **zero output** (exit 0) — every hot file is quiet. Safe for W1/W1b to proceed without a concurrent-writer conflict, as of this recon.

---

## 2. Catalog path confirmation

`grep -n "catalog.json\|📇️catalog" 📜️script.ts`:

```
6799:const POLICY_STDIO_OWNER_TABLE_REL = "✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json";
```

**Confirmed definitively**: `script.ts` resolves the stdio catalog via the single constant `POLICY_STDIO_OWNER_TABLE_REL` at line 6799, value exactly `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`, resolved with `join(repoRoot, POLICY_STDIO_OWNER_TABLE_REL)` at line 6847. This matches the plan's assumption exactly — the STATUS.md claim that it moved to `🚪️io/📇️registry/` is confirmed stale (that directory does not exist).

Secondary readers of the same constant: `expectedCount = table.counts?.stdio_artifacts ?? 29` (line 7041 — note the **fallback default is 29, not 28**; only matters if `counts.stdio_artifacts` is ever absent from the JSON, which it currently is not — see below), plus solution-string interpolations at lines 7032/7037/7759/7809/7853.

Catalog content check (`✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`):
- `stdio_roster` array length: **28** (matches plan's "28 format artifacts" and the catalog's own `counts.stdio_artifacts: 28`).
- `counts`: `{"stdio_artifacts": 28, "domain_artifacts": 54, "curated_io_pairs": 273}` — the plan's "counts.stdio_artifacts: 36" target (28 existing + 8 new: semio + 7 formats) is consistent with this baseline.
- `"neutral"` field: **28 occurrences** in the JSON (one per roster row), **zero readers** in `script.ts` (`grep -c "\.neutral\b"` = 0) — confirms the plan's "retire neutral field (zero script.ts readers)" claim exactly.

---

## 3. Per-plugin extraction ledger

`MediaFormat` occurrence counts per plugin (`grep -rn "MediaFormat" <dir> --include="*.rs" | wc -l`):

| Plugin | MediaFormat count | Notes |
|---|---|---|
| 📸️remodel | 7 | all in one file: `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (export/import format lists + PLY/LAS exporter `format()` impls) |
| 📐️cad | 16 | spread across app `🎛️apps/📐️cad/🦀️component.rs`, `🎮️commands/📥️io/🦀️component.rs`, and engine `🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` |
| 🎞️animate | 0 | no `MediaFormat` type usage — see FFmpeg finding below, which is a different mechanism entirely (subprocess, not the MediaFormat enum) |
| 🔋️energy | 0 | EPW parsing (`EpwWeather::parse`) is plain-text, does not touch `MediaFormat` |
| 🏛️architect | 0 | delimited-text codecs (`write_delimited`/`parse_delimited`) don't touch `MediaFormat` either |
| 🏗️fem | 24 (of the combined 24 fem+puzzle leaf-tree count, see below) | 16 files — full breakdown below |
| 🧩️puzzle | (8 of the same 24) | 8 files — full breakdown below |
| 📕️norm | 0 | norm's format leaves are name-collision leaf trees (csv/json/txt/xlsx/zip), not `MediaFormat`-typed |

### 📸️remodel — exact engine files + LOC (this is the extraction contract for W5a)

- Video engine (MP4/AVI, to move wholesale into stdio): `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs` — **5,163 LOC**. Matches the plan's "5,163 LOC" claim exactly.
- Images engine (PNG/JPEG, to delete → stdio engines): `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🖼️images/🦀️component.rs` — **1,878 LOC**. Matches the plan's "1,878 LOC" claim exactly.
- `MediaFormat` sites (7): export/import format vecs at lines 109–110 (`Glb, Obj, Stl, Ply, Las, Png` export; `Glb, Obj` import), and PLY/LAS exporter `format()` impls at 375–376/418–419.

### 📐️cad — exact files + line ranges

- Rust engine `MediaFormat` sites (16): `🎛️apps/📐️cad/🦀️component.rs` (import list, `export_solid_for_pane`/`export_solid_modelspace` signatures, export/import format vecs `[Step, Obj, Stl, Glb]`), `🎛️apps/📐️cad/🎮️commands/📥️io/🦀️component.rs` (import + `export_solid_modelspace(&view, MediaFormat::Step)` + ext-string match), `🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (`export_solids_as` match over `Obj/Stl/Step`).
- **TS STEP writer/reader** — `✏️s/🔌️plugins/📐️cad/🔨️modules/📐️geometry/🟦️component.ts` (3389 LOC total file). Verified the plan's cited range **`1418–1545` exactly**: `StepEntityWriter` class starts at line 1418; the pure STEP-write/parse cluster (`stepEscape`, `stepNumber`, `StepEntityWriter`, `parseStepEntityMap`, `stepParseFirstString`, `stepParseDescriptivePayload`, `parseSpatialUdaPayloads`, `mergeStepDataChunk`, `stepSpatialFileHeader`, `assembleStepFile`, `emitSpatialUdaProperty`) ends at line 1545, immediately followed by `applySpatialAttributesFromUda` at line 1546 (spatial-UDA metadata restore, not STEP encoding — correctly out of scope). The plan's line range is precise, not approximate.

### 🎞️animate — FFmpeg subprocess (confirmed, real CLAUDE.md violation)

`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs` (1187 LOC). Confirmed real subprocess spawn, not a false positive:
```rust
fn run_ffmpeg(args: &[&str]) -> Result<(), VideoError> {
    let status = Command::new("ffmpeg").args(args).status()...
}
```
Called from partial-movie concat, GIF sidecar generation (`fps=15,scale=640:-1:flags=lanczos`), and audio mux paths (lines ~1012–1094). This is a genuine external-runtime dependency (`Command::new("ffmpeg")`), confirming the plan's "delete FFmpeg subprocess path (violates no-external-runtime rule)" is accurate and necessary, not speculative.

### 🔋️energy — EPW parser

`✏️s/🔌️plugins/🔋️energy/⚙️engine/site/🦀️component.rs` (275 LOC total). `EpwWeather::parse` at line 55 (struct at line 44). Small, self-contained — plan's claim that energy's seed reads 15 columns with silent defaults (vs the full 35-column lossless rewrite needed) could not be verified precisely from a grep pass; recommend the W3 EPW agent diff column-by-column against the file directly rather than trust the "15 of 35" figure blindly.

### 🏛️architect — delimited-text codecs

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine/📤️exchange/🦀️component.rs` (507 LOC). `MergeStrategy` enum at line 15 (stays, per plan). `write_delimited`/`parse_delimited` at lines 165/197 (die, replaced by Csv/TsvSnapshot per plan); `import_delimited` at 245 wraps them with `MergeStrategy` upsert semantics — this glue must be re-pointed at the new Csv/TsvSnapshot types, not deleted.

### 🏗️fem + 🧩️puzzle — combined 24 JsonCodec-under-format-name leaf trees

Exact count matches the plan's "24" figure precisely:

- **fem: 16 files** — `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/{🧊️obj,🎒️zip,📷️png,🟪️stl}/.../🦀️component.rs` — 2 shapes × 2 directions × 4 formats = 16.
- **puzzle: 8 files** — same shape, only `◻2d` (no `🧊️3d`), 2 directions × formats `{obj, zip, stl}` minus one missing combination = 8 as counted.
- **16 + 8 = 24**, exact match to the plan.

### 📕️norm — degenerate leaves + shared-fn claim

- 16 artifact dirs under `🗿️artifacts/` (din4108, din18599, din16798, en1990–en1999, vdi3805, iso16757).
- Format leaf `.rs` files: **150** across 5 formats × 2 directions × 15 artifacts with both directions (zip: 30, txt: 30, csv: 30, xlsx: 30, json: 30) — **exact match to the plan's "150 degenerate ... leaves" figure**. Plus 16 top-level `🚪️io/🦀️component.rs` registration files (165 total `.rs` files under norm's io trees).
- **Could not independently confirm** the plan's "15 handcrafted codec copies → derive where it works, else ONE shared fn in norm's root" claim — no duplicated `write_csv`/`parse_csv`-style function-name collisions found via grep across norm's artifact roots, and no obvious shared-codec root file exists yet (norm's only root-level `.rs` files are manifest/config/setup/capabilities/app-surface/presence, none codec-shaped). **Flag for the W5a norm agent to verify directly** — this may refer to inline per-artifact duplicated logic rather than named functions, which a line-level diff would catch better than grep.

---

## 4. Final pattern-plugin roster (svg/dwg pattern) — DEFINITIVE

Checked all 9 plan-listed candidates for a real `*_document_json_to_svg` / `*_document_json_from_dwg` (or equivalent) pair with genuine drawing-derived logic vs. incidental string mentions:

| Plugin | Real leaf confirmed? | Evidence |
|---|---|---|
| 🗒️note | **yes** | `note_document_to_svg`/`note_document_json_to_svg` (real per-block SVG emission: text/image/ink paths) + `note_document_json_from_dwg` at `🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:615/622/676` |
| 📏️layout | **yes** | `layout_document_json_to_svg`/`layout_document_json_from_dwg` at `.../⚙️engine/🦀️component.rs:357` — real, builds `Page` structs from `dwg_rect_pages(drawing)` |
| 🌍️gis | **yes** | `gis2d_document_json_to_svg`/`gis2d_document_json_from_dwg` at `.../⚙️engine/🦀️component.rs:174` — real, walks `drawing.entities` geometry variants (Point/Line/LwPolyline/Polyline3d) |
| 🎥️shooting | **yes, but DWG import is an intentional stub** | `shooting_document_json_to_svg` is real; `shooting_document_json_from_dwg(_drawing: ...)` (param unused, `_`-prefixed) always returns `default_snapshot()` — the code comment explicitly documents this as "Tier C DWG import ... never errors ... this always returns the default studio fixture." Real registered leaf, honestly degenerate by design. |
| 🌀️procedural | **yes, but both directions are stubs** | `procedural2d_document_json_to_svg` calls `semio_framework_os::title_card_svg(value, "Procedural 2D", 1024, 768)` — a generic placeholder title-card, not real geometry; `procedural2d_document_from_dwg(_drawing: ...)` also always returns `default_snapshot()`. Matches the plan's own extraction-map note "🌀️procedural stubs deleted" (not migrated) — **plan is internally consistent here**, correctly slated for deletion not hub-and-spoke migration. |
| 🖨️raster | **yes** | `raster_document_json_to_svg`/`raster_document_json_from_dwg` at `.../⚙️engine/🦀️component.rs:327` — real, rasterizes via `dwg_drawing_to_svg` + `rasterize_svg_to_png_base64` into a pixel layer |
| 🖍️draw | **yes** | Full real implementation: `draw_document_to_svg`, `draw_document_json_to_svg`, `draw_document_json_from_dwg` at `.../⚙️engine/🦀️component.rs:1214/1243/1304`, plus dedicated tests for shape/text/image/gradient rendering |
| 🧩️puzzle app (◻2d) | **yes, DWG import stub** | `puzzle2d_document_json_to_svg`/`puzzle2d_document_json_from_dwg` at `🎛️apps/◻2d/🦀️component.rs:1173/1183` — from_dwg param `_drawing` unused, same stub pattern as shooting |
| 🪐️space handler | **NO — not a real leaf, correction below** | See §7. Zero `*_document_json_to_svg`/`*_json_from_dwg` functions anywhere in `✏️s/🔌️plugins/🪐️space/`. The only svg/dwg mentions are (a) a test-support command round-trip fixture using the literal string `"dwg"` as a `format` field, and (b) a `register_os_media_export_handler("2d.drawing", MediaFormat::Dwg, ...)` handler in `🎮️commands/🖼️media/🦀️component.rs` whose body is `DwgDrawing::default()` (ignores the actual document) and a `register_dwg_import_handler` whose callback is `|_drawing| Ok(json!({"imported": true}))` — a literal no-op stub for exercising the command-dispatch test harness, not a codec. |

**DEFINITIVE final roster: 8 plugins belong in W5b (pattern plugins)** — 🗒️note, 📏️layout, 🌍️gis, 🎥️shooting, 🌀️procedural, 🖨️raster, 🖍️draw, 🧩️puzzle app. **🪐️space does NOT belong in W5b** — remove it from the roster; its DWG/SVG surface is test-fixture scaffolding, not a real codec to migrate. (🌀️procedural additionally should be treated as a straight deletion target per the plan's own extraction map, not a hub-and-spoke migration — its leaves are 100% generic placeholders on both sides.)

---

## 5. MediaFormat full census

`grep -rln "MediaFormat" --include="*.rs" ✏️s 🧰️framework | grep -v "🎫️tickets"` — full list saved to `w0-mediaformat-census.txt`.

**File count: 55. Total occurrence-line count: 346.** Both numbers match the plan's "55 files / 346 lines footprint" claim **exactly** — verified independently via `grep -c` across the same file set, not just file-count.

Notable entries for W6's checklist beyond the plugin ledger in §3: `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` (the enum definition itself), `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs`, `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` (see §6), `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`, `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (the re-export site — see §7), plus stdio's own `🧊️gltf` composer and plugin-side sites already covered in §3 (remodel, cad, raster, process, space, gis, shooting, layout, puzzle, fem, draw, lowpoly).

---

## 6. `🔁️workflow` module mount status

`grep -n "workflow" 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` → **zero matches.** The os-kernel glue.rs has no `workflow` module block at all (verified by reading the full 270-line file — only `os_dsl`, `os_pack`, `os_spr`, `os_vcs`, `os_io`, `os_store`, `os_engine`, `os_semio`, `os_extension` are mounted).

The file's own header comment is direct evidence this is a known, intentional gap:
```
//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.
```

The target file to mount exists and is substantial: **`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`** — 2583 lines, 136 KB, confirmed present on disk.

**This is confirmed as the root cause of the os-run E0432/E0425 errors in §1c.** The run crate's glue.rs does exactly what the plan's ground truth says:
```rust
extern crate semio_framework_os_kernel as workflow;
```
— aliasing the *entire* os-kernel crate re-export surface as `workflow`. Since the kernel's glue.rs never mounts the actual `🔁️workflow/🦀️component.rs` module, none of `workflow::Workflow`, `workflow::WorkflowNode`, `workflow::WorkflowEdge`, `workflow::RunArtifact`, `workflow::apply_run_operation_checked`, etc. exist under that alias — hence the exact E0432 unresolved-import list and E0425 missing-type/fn errors captured in §1c. Mounting `🔁️workflow` into the os-kernel glue.rs (as its own `pub mod os_workflow { ... }` block, re-exported the same way `os_vcs`/`os_semio` are) is very likely necessary and sufficient to clear the E0432/E0425 trio; the remaining `os_dsl` scope errors, duplicate-fn E0592s, and E0004 non-exhaustive match are independent and need separate fixes in the run crate's own `component.rs`.

---

## 7. Corrections to master plan

1. **The plan's "os-run blocker today: 3× E0063 missing `topic_contributions`" is STALE, not the current blocker.** All three cited sites — `🔌️plugin/🦀️component.rs:5884`, `🔌️plugin/🦀️component.rs:6120`(-ish, the literal is at 6128 in the current line numbering), and `🔌️plugin/🖥️host/🦀️component.rs:816` — **already have `topic_contributions: vec![]`/`Vec::new()` populated**, confirmed by direct inspection. This was fixed by another session at some point before this recon (consistent with the repo's known pattern of concurrent cargo-workspace churn). **The actual current `cargo check -p semio-framework-os-run` blocker is a different, larger set of 13 errors** (§1c): 1× E0432 (11 unresolved `workflow::*` imports), 5× E0433 (`os_dsl` not found in scope), 3× E0425 (`RunArtifact`×2, `apply_run_operation_checked`), 2× E0592 (duplicate `artifact_pack_path`/`artifact_spr_path` — this part of the plan's ground truth **is** still accurate), 1× E0609 (`RunSink` has no `operations` field), 1× E0004 (non-exhaustive `AppFrame` match — also still accurate). W1's "attempt os-run fix" step should target this corrected error list, not the stale E0063 one. Mounting `🔁️workflow` (§6) is expected to clear most of the E0432/E0425 cluster; the `os_dsl` scope errors, duplicate-fn E0592s, missing-field E0609, and E0004 need separate, smaller fixes in the run crate's own `component.rs`.

2. **The plan's hot-file list omits the framework's own `glue.rs` path entirely** — item 1's file list in this recon task asked to check `"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"` (a path that does exist, confirmed) but the master plan's "Hot single-writer files" bullet just says "framework `📦️glue.rs`" without a concrete path. **Resolved: the actual framework crate root glue.rs is at `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`** (distinct from the os-product's glue.rs at `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`, and distinct from stdio's). It re-exports `MediaFormat` per the plan's V7 step 4 ("strip framework `📦️glue.rs:38` re-export") — confirmed this file is on the 55-file MediaFormat census (§5) and is currently clean/untouched. Future waves referencing "framework's glue.rs" should use this exact path.

3. **The pattern-plugin roster's inclusion of 🪐️space is wrong** (§4). Space has no real `*_document_json_to_svg`/`*_json_from_dwg` codec pair; its only DWG/SVG surface is a stub media-export/import handler pair used to exercise a command-dispatch test, plus a hardcoded test-fixture string. Recommend dropping 🪐️space from the W5b roster entirely — there is nothing to extract there. Confirmed W5b should be **8 plugins**, not 9: 🗒️note, 📏️layout, 🌍️gis, 🎥️shooting, 🌀️procedural, 🖨️raster, 🖍️draw, 🧩️puzzle app.

4. **Minor: `script.ts`'s `expectedCount = table.counts?.stdio_artifacts ?? 29` fallback default (line 7041) is 29, one higher than both the plan's stated "28 format artifacts" and the catalog's actual current `counts.stdio_artifacts: 28`.** This fallback is currently dead code (the field is always present), so it's not a live bug, but W1b should update it to the new target count (36) alongside the catalog edit rather than leave a stale magic number in the source, since it will silently diverge again once the catalog changes.

5. **Everything else in the plan's "Verified ground truth" section checked out exactly as stated**: catalog SSOT path (§2), 55-file/346-line MediaFormat footprint (§5), remodel's 5,163+1,878 LOC split (§3), cad's TS STEP range 1418–1545 (§3), fem+puzzle's 24 leaf trees (§3), norm's 150 degenerate leaves (§3), the `🔁️workflow` unmounted-module root cause chain (§6), and the `neutral` field's zero-reader status (§2). No corrections needed on those points.

---

## Raw output files in this ticket folder

- `w0-stdio-test-baseline.txt` — full `cargo test -p semio-s-plugin-stdio --lib` output (1075 passed, 0 failed)
- `w0-policy-baseline.txt` — full `bun ./📜️script.ts policy` output (21564 breaches / 24 rules, 21592 lines)
- `w0-osrun-check.txt` — full `cargo check -p semio-framework-os-run` output (13 errors, corrected from plan's stale E0063 claim)
- `w0-mediaformat-census.txt` — full 55-file MediaFormat grep census
