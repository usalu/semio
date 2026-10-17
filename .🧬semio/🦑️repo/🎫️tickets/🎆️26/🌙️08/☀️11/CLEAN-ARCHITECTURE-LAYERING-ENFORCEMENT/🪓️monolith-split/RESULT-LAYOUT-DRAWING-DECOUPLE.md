# Result: Layout Drawing Decouple (slice LAYOUT-DRAWING-DECOUPLE, prefix `ldd`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 21:45 to 23:30. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/ldd-tools/`. Logs: `F/🗑️generated/ldd-*` (nothing deleted). Nothing was compiled by cargo (the launcher does not reach cargo, see section 6); all Rust evidence is static, from rustc analysis-only oracles.

## 0. Verdict

**Done, statically verified, not compiled.** Design (A), the port, was chosen and is implemented completely for the Rust side; the TypeScript twin already had it.

| Target | Before | After |
| --- | --- | --- |
| Layout Cargo edges into stdio (11 manifests, `F/ldd-tools/cargo-r2.ts`) | **32** member lines (+ 11 `[workspace.dependencies.semio-s-artifact-stdio-*]` tables) | **0** (and 0 tables). The one remaining cross-plugin edge is layout to forms (dev-dependency of `editor`, a separate item of `RESULT-PLUGIN-CROSS-EDGES.md` W2.6 #2) |
| `include_str!` of the stdio drawing DDL in layout | 1 (`🪶️sqlite/📸️snapshot/🦀️.rs`) | **0**; the owner exposes the DDL through its existing `ArtifactSqliteSnapshot::SQLITE_SCHEMA` and the binding hands it to the slot |
| Layout Rust files naming a stdio crate (production + tests) | 30 files, 155 uses | **0** in mounted files. One unmounted dead test (`✏️editor/🧪️tests/🔬️unit/🎞️intrinsic/🦀️.rs`, 2 uses of the stdio facade plus a forms dependency) is left, see section 7 |
| Plugin gate `new crates … --check-applied --base=f856dff0813` layout package | exit **1** (4 s, `Missing Rust source input` from a wrong `include_str!` depth in a layout scene test) | exit **0** (485 s; native base 2578 / combined 2508 / only-combined 0; `--test` 6064 / 5761 / 0) |
| Same gate, stdio `🧿️semio` package | exit 142 (alarm 1700 s, no verdict) | no stdio file touched by this slice. Rerun with a 6500 s cap: exit **1** after 2102 s; native base 4592 / combined 5290 / only-combined **2** (both in `🔢️value/🚪️io`, a peer area: `cannot find semio_framework_schema_registry / semio_framework_os_kernel in the crate root`), `--test` 7840 / 7724 / 0. The baseline gave no verdict, so there is nothing to compare; the 2 extras are not from this slice (section 5.2) |
| Typeck oracle, layout family (18 layout crates + the two new crates) | canary reached; baseline 5306 lib / 10420 test errors | canary reached; lib **0** new non-stub errors in every category; test: 3 rows, all oracle derive-blindness (section 5.3) |
| Typeck oracle, stdio Semio drawing family (4 crates) | canary reached | canary reached, **NEW 0** lib and test (the family is unchanged) |
| Source-boundary census | all layout crates within limits | all within limits, new crates 297 and 261 lines (section 5.5) |
| rustfmt parse (`--config skip_children=true --emit stdout`) | n/a | 40 touched files, 0 failures |
| Paths | n/a | longest new path 157 bytes (limit 240) |

## 1. Decision: (A) embedded-child port, not (B) framework formats module

### 1.1 What the drawing standard really is (measured)

| Fact | Evidence |
| --- | --- |
| Its model is small and generic: `SemioDrawingSnapshot { canvas, styles, layers }`, `DrawNode` (`Path`, `Text`, `Group`, `Image`), `PathSegment` (6 variants), `DrawStyle`. About 350 lines. | `🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs` |
| In use it is shared: the drawing crate is a dependency of procedural, gis, shooting, layout (11 manifests), cad, draw, raster, stdio, note, puzzle and three hub compositions; `semio-base` is used by 23 plugins (57 stdio manifests). | `git grep` over all `Cargo.toml` (`F/🗑️generated/ldd-uses-before.txt` holds the layout side) |
| Its implementation is not self-contained. `semio-base` depends on `semio-s-artifact-stdio-contract` (the stdio registry contract) and on `framework-os-kernel`/`framework-plugin`; `…-semio-drawing` on that contract, on `semio-base` and on `base-examples`; `…-semio-drawing-export` and `…-import` on the stdio format artifacts `dwg-model`, `dwg-write`/`dwg-read`, `dxf`, `pdf`, `png`, `svg`, `xml`. | the manifests of those crates |
| Layout does not only use the model: it also used the dwg, dxf, svg, json and png codecs (ten io leaves), the sqlite DDL and two png decoders. | `ldd-uses-before.txt` |

### 1.2 Why (B) is not available

(B) moves the drawing standard and everything it stands on into the framework. That closure is the stdio registry contract (12 crates), `semio-base`, `base-examples`, `drawing` with its io, and the dwg, dxf, pdf, png, svg, xml artifacts with their own dependencies: the "relocate the native Semio standard and the format artifacts into the framework" option of `RESULT-PLUGIN-CROSS-EDGES.md` section 4 (7,459 files, about 366 Cargo edges, neutral ids so the framework does not host `s.stdio.*`). Two rules close it:
- The framework must not depend on `✏️s` (R1, `cargo-framework-no-implementation`); every moved crate depends on `semio-s-artifact-stdio-contract`.
- AGENTS.md: domain-neutral framework, domain-specific extensions. A vector document standard with its `s.stdio.semio` ids, DDL and codecs is an extension.

Only PNG is a neutral format that already lives in the framework (`🧰️framework/🔨️modules/🔲️pixels`, "so `✏️s/🔌️plugins/**` crates never need the third-party png crates"). Layout's PNG use (decode, validate, one 1x1/RGBA author step) therefore re-points there: for that one format the (B) answer is already implemented.

### 1.3 Why (A)

- The TS side chose it: `embeddedChildSlotV1` (`🧰️framework/🔨️modules/🚪️io/🔌️ports/🟦️.ts`), binding in `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🔗️binding/🟦️.ts`. A different Rust mechanism would be two shapes for one rule.
- What layout needs from the child is: carry it opaque, validate it, store it in its DDL, look at it (render, export, edit text), and read and write external formats. That is a codec plus a typed lowering, not ownership of the standard.
- The one place that knows both owners is a hub composition crate. It is the sanctioned direction (hub to s) and `semio-hub-layout` already depended on the stdio dwg, dxf, json, pdf, png and svg crates.

### 1.4 No dual mechanism

The Rust twin has one mechanism: the slot. There is no second path for PNG (layout uses the framework pixels crate, which is neutral) and no second path for SVG (`layout_document_json_to_svg` used to compose through `io_dispatch` and `write_svg_xml`, while the export leaf used `encode_drawing`; both now go through `codec.export("svg", …)`).

## 2. Design as implemented

### 2.1 Framework port crate `semio-framework-io-ports` (new)

`🧰️framework/🔨️modules/🚪️io/🔌️ports/📦️packages/🦀️rust` (crate at the node of the TS port; lib root `🔌️ports/🦀️.rs`, tests `🔌️ports/🧪️tests/🦀️.rs`, feature `testkit`).
- `EmbeddedChildSlotV1`: `const fn new(id)`, `bind(&'static dyn codec)` (idempotent for the same codec, `Rebound` for a different one), `codec()` (`Unbound` refusal). Same laws and the same message texts as `embeddedChildSlotV1`.
- `EmbeddedChildCodecV1` (the Rust facets): `validate(content, control)`, `lower(content) -> VectorScene`, `lift(scene) -> content`, `sqlite_schema()`, `sqlite_owns_table()`, `sqlite_project()`, `sqlite_reconstruct()`, `import_formats()/export_formats()`, `import(format, bytes)`, `export(format, content)`. Content is a plain `DslValue`, so the wire form of `LayoutSnapshot` is unchanged (it already was `Shape::Value`).
- `VectorScene` (schema id, canvas, styles, layers; `VectorNode::{Path, Text, Group, Image}`, `VectorSegment` ×6, point, quaternion, rgba, transform): the typed lowering. Its field names and tags equal the drawing's wire form, so a real drawing validates and lowers without rewriting fixtures. It has `texts()` and `set_text()` (the text edit logic that used to live in layout).
- `testkit` (`VectorSceneCodec`, `bind`, `scene_content`): the codec layout binds in its own unit tests.

### 2.2 Layout (all of it now stdio-free)

| Area | Change |
| --- | --- |
| model `🧬️schema/🦀️.rs` | `LayoutDrawingChild { handle: ArtifactChild<LayoutBackgroundDrawing>, content: DslValue }`; `LAYOUT_BACKGROUND_DRAWING` slot (id `layout.background-drawing`, the TS id); `drawing_codec()`, `background_drawing_scene()`, `set_drawing_text()` (lower, edit, lift) and `drawing_labels()` over the slot; the content is validated on every wire path (`deserialize_with` on the derived reader, the controlled `drawing_child::decode`, the uncontrolled `DslField::from_value`) |
| model component codec `📸️snapshot/🧩️component` | controlled decode/encode/retirement over `DslValue`, validation under the caller's `NativeDecodeControl` |
| model sqlite `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` | `schema()` composes the layout DDL and the bound codec's DDL once (like TS `layoutSqliteSchema()`); `project`, `reconstruct` and the unowned-rows check go through the codec (`sqlite_project`, `sqlite_reconstruct`, `sqlite_owns_table`) |
| framework trait `ArtifactSqliteSnapshot` | one provided method `sqlite_schema()` (default = `SQLITE_SCHEMA`); `codec-operations` reads the schema through it. Needed because an associated const cannot come from a runtime slot. Layout overrides it |
| io root + 10 leaves | no stdio imports. `layout_snapshot_to_scene` builds the neutral scene (merging the lowered background), `export_scene`/`layout_snapshot_to_format` hand it to the codec, `layout_document_json_from_format`/`…_from_drawing` frame pages from imported content. The dwg, dxf, svg, png, json leaves keep their taxonomy mounts and composer entries, so layout's declared capabilities and the component descriptor are unchanged. The typed `JsonSnapshot` and `DwgSnapshot` leaf functions had no callers and are removed |
| scene | one lowering per display list (`background_scene`), then the unchanged stroke/label/raster algorithms over `VectorNode`/`VectorSegment`/`VectorStyle`; PNG decode through `semio_framework_pixels::decode_png` |
| export-json | the typed streaming cursor walks the lowered scene (cached in the cursor, `Arc<VectorScene>`), byte-identical output by construction (same node order and scalar formatting) |
| export, canvas | PNG through `semio_framework_pixels` (`decode_png`, `encode_png`, `RasterImage`); dead stdio imports removed |
| editor root, diff, mutations, layout root | dead stdio imports removed; diff `apply` uses the fallible `set_drawing_text` |

### 2.3 Composition crate `semio-hub-layout-drawing` (new, hub)

`🌎️hub/🧩️compositions/📏️layout/🖊️drawing/📦️packages/🦀️rust` (lib root `🖊️drawing/🦀️.rs`). `SemioDrawingCodec` implements the port over the unchanged stdio crates (validate through the controlled typed decode and cold retirement, lower/lift mapping, DDL from `SemioDrawingSnapshot::SQLITE_SCHEMA`, component projection and reconstruction, svg/dxf/dwg/png export, svg/dxf/dwg import including the dwg rectangle-page detection that used to sit in layout io). `bind_layout_background_drawing()` is idempotent. `semio-hub-layout`'s `plugin()` calls it first, before the declaration reads the sqlite schema.

Tests that need the real standard moved here with their fixtures (`🧪️tests/🪶️sqlite/**` with `💰️backing` and `🧫️fixtures`, `🧪️tests/🌉️media`), exactly as the TS twin moved its sqlite and document tests to the composition project, plus new codec laws (`🧪️tests/🦀️.rs`: bind idempotence, lower/lift identity against the typed standard, validation budget, DDL ownership and composition, real export files in four formats, import round trips, refusals).

## 3. Registrations

| Item | Change |
| --- | --- |
| Manifests (10 layout crates + workspace) | 32 stdio member lines and 11 `[workspace.dependencies.semio-s-artifact-stdio-*]` tables removed; `semio-framework-io-ports` and `semio-framework-pixels` added where used (workspace entries in `📏️layout/Cargo.toml`), `io-ports[testkit]` as dev-dependency of the 7 crates whose tests author drawings |
| `semio-hub-layout/Cargo.toml` | + `semio-hub-layout-drawing` |
| Workspace members | `🧰️framework/Cargo.toml` (+ `🔌️ports`), `🌎️hub/Cargo.toml` (+ `🖊️drawing`), byte-sorted |
| Build owners | `📋️build-owners.json`: `@semio-tech/io-ports-rs`, `@semio-tech/hub-layout-drawing-rs` in lexical order (JSON parses) |
| Launch routes | six routes each (build, check, test × prototype, release) in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, orders 900.02004/5, 922.05012/3, 903.59012/3 (framework) and 900.03001/2, 922.0604/41, 903.60088/9 (hub), each unused before; both parse (`Bun.JSONC`) |
| Descriptors / generated registries | none change: the layout capability rows, `reads()`, composer entries and the component descriptor are untouched; the TS slot and its tests are untouched |
| Taxonomy | new nodes reuse registered emojis `🔌️ports` (`🚪️io`), `🖊️drawing`; `verify taxonomy report` result in section 5.6 |
| Locks | `Cargo.lock` files were not hand-edited. Needed: `update --workspace --offline` for the layout workspace, the layout `🏭️bridge`, `🌎️hub` and `🧰️framework` (new packages `semio-framework-io-ports`, `semio-hub-layout-drawing`, new edges of layout) |

## 4. Baselines (before any edit)

| Check | Result |
| --- | --- |
| `F/ldd-tools/cargo-r2.ts HEAD` | 20 manifests; layout to other plugins: 33 edges (32 stdio, 1 forms) |
| Plugin gate, layout package (`F/🗑️generated/ldd-layout-before.*`) | exit 1 in 4 s: `Missing Rust source input: ✏️s/🧰️framework/…/🔤️text-fonts/🔣️.json`. Cause: the scene test's `include_str!` had 13 `../` where the file needs 14 |
| Plugin gate, stdio `🧿️semio` package (`ldd-stdio-before.*`) | exit 142 after 1701 s (the cap) |
| Plugin gate, stdio drawing subset directory (`ldd-stdiodraw-before.*`) | exit 1: `ENOENT …/🖊️drawing/Cargo.toml` (not a facade package; not used further) |
| Oracle base side (HEAD, `lddlayout`) | lib 5306, test 10420 errors, canary reached |
| Census before | `ldd-census.txt` section "BEFORE" |

## 5. Verification actually run

### 5.1 Layout plugin gate after
`nice bun ./📜️script.ts new crates ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust --check-applied --base=f856dff0813 --samples=400 --sandbox=<scratch>/ldd-sbx/layout-after3` (`F/ldd-tools/run-gate-once.sh layout-after3 … 3000`): **exit 0, 485 s**, log `ldd-layout-after3.{log,err,exit}`. `healedBase` lists 4 files whose base copy did not parse (pre-existing). Two earlier "after" runs (exit 1, 5 s each) found two more pre-existing wrong include depths in the files I was editing; both were repaired (section 8) and the gate then reached its comparison. Before exit 1, after exit 0: improved.

### 5.2 Stdio plugin gate
No stdio file was changed by this slice (`git status` shows 345 stdio paths modified by peers, none by me; the stdio crates are consumed by the binding unchanged). Command: `F/ldd-tools/run-gate-once.sh stdio-after '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust' 6500`, i.e. `new crates … --check-applied --base=f856dff0813 --samples=400`. Result `ldd-stdio-after.{log,err,exit}`: **exit 1, 2102 s**, 77 crates, `native` base 4592 / combined 5290 / only-combined 2, `--test` 7840 / 7724 / 0. The two extras are `E0433 cannot find semio_framework_schema_registry in the crate root` (`🔢️value/🚪️io/🦀️.rs:119`) and `cannot find semio_framework_os_kernel in the crate root` (`🔢️value/🚪️io/💾️binary/📸️snapshot/🦀️.rs:35`): the stdio `🔢️value` subset, edited by peers today. The pre-edit run timed out at its 1700 s cap without a verdict, so before and after cannot be compared; the stdio input of the gate is identical to what the binding consumes, and this slice changed none of it.

### 5.3 Typeck oracle (`pos-tools`, `POS_PREFIX=ldd`, families in `F/ldd-tools/families/`)
- `python3 pos.py run ../ldd-tools/families/lddlayout.json --cfgs lib,test --label l3`: lib `cur errors 5184 (stub-involved 4960); base 5306; NEW 62` (all new rows stub-involved); non-stub new: privacy 0, trait-scope 0, resolve 0, type 0, other 0. Test: `cur 9963; base 10420; NEW 150`; privacy 0, trait-scope 0, resolve 0, other 0, type 3.
- The 3 test rows are `E0599 no associated function or constant named from_value found for struct VectorScene` (port lib lines 315, 318; port test line 15). The sandbox does not expand the `ToValue`/`FromValue` derives of `semio-framework-value-derive`, so every call to a derive-provided method on a type of the family reports this. The baseline carries the same class (3 rows lib, 13 rows test in `ldd-lddlayout-l3-base-*-final.err`, e.g. `LayoutSnapshot::from_value`), hidden there only because the keyed multiset matches by message. They are not real errors; I did not restructure the code to hide them.
- Real errors the oracle found and I fixed in this slice: `?` on a temporary `Arc` inside a `match` scrutinee (export-json, 2 sites) and a closure-inference cascade in `layout_document_json_from_drawing`. After the fixes (run l3) lib type errors are 0.
- `python3 pos.py run ../ldd-tools/families/lddstdio.json --cfgs lib,test --label s1` (base, drawing, drawing-export, drawing-import): lib `2679 / 2679, NEW 0`, test `4146 / 4146, NEW 0`, canary reached in both.
- Canary: `typeck canary (E0616,E0624,E0308 expected): cur ['E0308','E0616','E0624']` for all four configuration runs, so bodies were type-checked.

### 5.4 Direction reports
- Cargo R2 over the layout plugin: `bun F/ldd-tools/cargo-r2.ts HEAD` → layout → stdio **32 → 0** (see section 0).
- Authoritative strict report (`cargoDependencyDirectionReport`, `F/ldd-tools/cargo-direction-report.ts`, two attempts 22:5x and 23:4x): did not run, a peer manifest is missing (`…/🧊️wgpu/🧿️test-support/📦️packages/🦀️rust/Cargo.toml` ENOENT, `ldd-cargo-direction-after.txt`, `…-after2.txt`). My crates add no framework-to-s or framework-to-product edge by construction (the port depends only on framework crates; the binding is a hub crate).
- TS: scoped cruiser, the exact command of `DEPENDENCY-DIRECTION-AUDIT-2.md` section 1 over `✏️s/🔌️plugins/📏️layout` and the composition `📏️layout-drawing` project: exit 51, 1075 hygiene violations (`ui-no-framework-packages` and similar), **0** `no-cross-plugin`, **0** `hub-to-✏️s`, **0** `framework-no-implementation` lines (`ldd-cruise-ts.txt`). This slice edited no TypeScript file; the TS side is therefore unchanged.

### 5.5 Census (`bun F/ldd-tools/census.ts <rev|WORKTREE> <crate dirs>`, canonical closure counter, own mounted closure per lib)
| crate | before (files / bytes / lines) | after |
| --- | --- | --- |
| model | 19 / 149116 / 1820 | 18 / 143328 / 1799 |
| diff | 5 / 72106 / 1484 | 5 / 72211 / 1484 |
| mutations-frames | 21 / 99211 / 1695 | 21 / 99141 / 1694 |
| io | 17 / 56717 / 942 | 17 / 45460 / 812 |
| engine | 3 / 99931 / 1481 | 3 / 99642 / 1484 |
| engine-export | 1 / 122852 / 2344 | 1 / 122371 / 2338 |
| engine-json | 2 / 104673 / 1924 | 2 / 105305 / 1943 |
| editor | 2 / 132380 / 2006 | 2 / 131935 / 2000 |
| editor-commands | 32 / 171430 / 3172 | 32 / 171000 / 3168 |
| layout root | 1 / 10656 / 182 | 1 / 10574 / 181 |
| `semio-framework-io-ports` | new | 1 / 14691 / 297 |
| `semio-hub-layout-drawing` | new | 1 / 15108 / 261 |
| `semio-hub-layout` | 2 / 6550 / 114 | 2 / 6944 / 116 |
All under 32 files, 262144 bytes, 4000 lines (editor-commands stays at its 32 files).

### 5.6 Taxonomy
The whole-repo `bun ./📜️script.ts verify taxonomy report` does not finish under the present load (first run: alarm at 1700 s inside `plan/references`, no verdict, `ldd-verify-taxonomy-report.exit`; the second run was at 18k of 112k references after 27 min and was stopped). I used the inventory engine it uses (`inventoryTaxonomy`) through the fallback of `RESULT-TAXONOMY-SWEEP.md` (`🗑️generated/taxsweep/inv.ts <out> <scope>`), scoped (19 s each):
- scope `🌎️hub/🧩️compositions/📏️layout` (50 entries, includes the new `🖊️drawing` crate and its moved test tree): **1 violation, not mine**: `fixed-source-disposition-unresolved 🌎️hub/…/📏️layout/📦️packages/🦀️rust/📜️script.ts` (an existing file).
- scope `🧰️framework/🔨️modules/🚪️io/🔌️ports` (13 entries): **0 violations**. Scope `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📸️snapshot` (6 entries): **0**.
Results: `ldd-tax-hub-layout.json`, `ldd-tax-*.json`.

### 5.7 Other
- rustfmt parse (`rustfmt --edition 2021 --config skip_children=true --emit stdout`) over the 40 existing touched `.rs` files: 40 ok, 0 failures (`ldd-rustfmt.log`).
- Path lengths: 31 new files, longest 157 bytes.
- Include/mount scan of all 267 layout, port and binding Rust files: every `include_str!/include_bytes!` target exists, except the one in the dead intrinsic test (section 7).

### 5.8 Real checks to queue (no compile was possible; the launcher shows only `stage: running` for the other executor too, mutex `rsl` was held)
Run under the fleet mutex, one call each, after `-- update --workspace --offline [--manifest-path …]` for each workspace:
```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-io-ports --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
… -- check -p semio-s-artifact-layout-layout-model -p semio-s-artifact-layout-layout-diff -p semio-s-artifact-layout-layout-io -p semio-s-artifact-layout-layout-engine -p semio-s-artifact-layout-layout-engine-export -p semio-s-artifact-layout-layout-engine-json -p semio-s-artifact-layout-layout-editor -p semio-s-artifact-layout-layout-editor-commands -p semio-s-artifact-layout-layout-mutations-frames -p semio-s-artifact-layout-layout --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml
… -- check -p semio-hub-layout-drawing -p semio-hub-layout --lib --tests --keep-going --message-format=short --manifest-path 🌎️hub/Cargo.toml
… -- test -p semio-framework-io-ports --lib --manifest-path 🧰️framework/Cargo.toml
… -- test -p semio-hub-layout-drawing --lib --manifest-path 🌎️hub/Cargo.toml
```
Expected: 0 errors. Anything else is in `semio-hub-layout-drawing` (new code) or in the two crates whose generics I could not check against the real derive output (`VectorScene`, `LayoutDrawingChild`).

## 6. Files changed

New: `🧰️framework/🔨️modules/🚪️io/🔌️ports/{🦀️.rs, 🧪️tests/🦀️.rs, 📦️packages/🦀️rust/{Cargo.toml,📋️project.json}}`; `🌎️hub/🧩️compositions/📏️layout/🖊️drawing/{🦀️.rs, 🧪️tests/🦀️.rs, 📦️packages/🦀️rust/{Cargo.toml,📋️project.json}}`; moved there from layout: `🧪️tests/🪶️sqlite/**` (10 files, 76 KB) and `🧪️tests/🌉️media/🦀️.rs`.
Edited (layout `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/…/✳️any/` = `A/`): `A/🧬️schema/🦀️.rs`, `A/🧬️schema/📸️snapshot/🧩️component/🦀️.rs`, `A/🧬️schema/🔺️diff/🦀️.rs`, `A/🧬️schema/🧬️mutations/🏷️set-drawing-text/🦀️.rs` (+ its test), `A/🚪️io/🦀️.rs`, 9 io leaves under `A/🚪️io/{📤️export,📥️import}/**` (+ the json leaf test), `A/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, `A/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`, `A/✏️editor/⚙️engine/🎬️scene/🦀️.rs` (+ tests), `A/✏️editor/⚙️engine/📤️export/🦀️.rs`, `…/📤️export/🔣️json/🦀️.rs`, `…/📤️export/🧪️tests/🔬️unit/🦀️.rs`, `A/✏️editor/🖼️canvas/🦀️.rs` (+ tests), `A/✏️editor/🦀️.rs`, `A/✏️editor/🎮️commands/🩹️patch-frame/🦀️.rs`, the layout root `🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`; the 10 layout manifests and `📏️layout/Cargo.toml`; framework `🪶️sqlite-snapshot/📸️snapshot/🦀️.rs` and `🧰️codec-operations/🦀️.rs`; hub `🧩️compositions/📏️layout/🦀️.rs` and its `Cargo.toml`; `🧰️framework/Cargo.toml`, `🌎️hub/Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.
Tools (`F/ldd-tools/`): `usescan.py`, `retype.py`, `manifests.py`, `cargo-r2.ts`, `census.ts`, `cargo-direction-report.ts`, `run-gate.sh`, `run-gate-once.sh`, `run-both-before.sh`, `io-region.rs.txt`, `families/{lddlayout,lddstdio}.json`.

## 7. Open points and hand-offs

- **Not compiled, behaviour not run.** The statements above are static. Behaviour that rests on assumptions I could not run: `DecodedValue` drops through its retire closure on `drop`; the derive attribute `deserialize_with` on `content`; the stub-free `ValueError`/`NativeDecodeControl` signatures of the real crates. The first real compile is the check list of section 5.8.
- **Dead test left in place:** `A/✏️editor/🧪️tests/🔬️unit/🎞️intrinsic/🦀️.rs` is mounted nowhere; it names `semio_s_artifact_stdio_semio::SemioMembers` (not a declared dependency), uses the forms plugin, and includes a fixture from the sqlite tests directory that moved to the hub crate. It belongs to the queued Forms-to-Layout integration crate (`RESULT-PLUGIN-CROSS-EDGES.md` W2.6 #2); it must be relocated there, not repaired in place.
- **Schema documents still reference the drawing standard as declared data**: `🧬️schema/🔣️.json` `$ref` to the Semio drawing JSON schema, `🛰️.proto` import and message field, GraphQL `@child(kind: "s.stdio.semio")`. The TS twin left the same annotations; they are the child kind declaration, not a code dependency.
- **Performance:** the scene lowers the background once per display list build (before: no copy). If a profile shows interactive cost on large plans, cache the lowered scene by `handle.child_id` in the model (the id is content-addressed); no measurement was possible.
- **Removed API without callers:** the typed `JsonSnapshot`/`DwgSnapshot` functions of the json and dwg leaves; `layout_document_json_from_dwg`, `compose_svg_from_drawing`, `layout_snapshot_to_semio_drawing`, `ensure_stdio_semio_drawing_registered`. The text variants (`serialize_text`, `deserialize_text`, `deserialize_bytes`) stay.
- **Unbound slot behaviour:** a process that decodes a document with a background drawing, reads the layout sqlite schema or exports without calling `bind_layout_background_drawing()` gets the named refusal (`embedded child slot layout.background-drawing has no bound codec`); the layout sqlite schema read panics with that text, like TS `layoutSqliteSchema()` throws. Every composition root of layout must bind (today only `semio-hub-layout::plugin()` and the moved/layout tests).
- **Other plugins** (draw, cad, note, raster, gis, …) still depend on the same stdio crates; they are the same R2 class and can reuse the port (`VectorScene` is generic). Out of scope here.
- Locks: see section 3. `cpd-scan`/strict cargo report blocked by a peer manifest (section 5.4).
- **Language-agnostic test:** the Rust slot test (`🔌️ports/🧪️tests/🦀️.rs`) repeats the three laws and message texts of the TS test (`🔌️ports/🧪️tests/🟦️.ts`) as a hand-written truth table; the two do not yet read one shared fixture file. The sqlite and document-contract laws of layout already share their JSON fixtures with TS (the contract fixture is decoded by the Rust root test through the testkit codec).
- **Test output lines:** the moved sqlite tests keep their original permanent `eprintln!("[DEBUG] …")` lines; this slice added no temporary debug output.
- **Git state:** my edits were staged by the fleet's auto-commit flow while I worked (`git status` shows `M ` for them); "before" numbers are measured at `HEAD` `b84bf526c7b` (`cargo-r2.ts HEAD`, `census.ts HEAD`).

## 8. Defects found and fixed on the way (pre-existing)

- Scene test `FONT_FIXTURE` `include_str!`: 13 `../` instead of 14 (made every gate run fail with `Missing Rust source input`); repaired.
- Canvas test schema include used `concat!(env!("CARGO_MANIFEST_DIR"), "/../" × 7, …)`, right for the facade but wrong for the split `editor` crate (needs 12); replaced by a file-relative path that is correct in both contexts.
- Sqlite `backing/📏️preflight` test mounted a framework file by a 15-level `#[path]` tuned to its old location; it now uses the `semio-framework-test-original-snapshot-owners` crate.
- Duplicate `use semio_framework_value_derive::{FromValue, ToValue}` in the model schema removed.
