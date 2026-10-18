# Result plugfix (pre-existing plugin defects from `result-c6plug.md` plus a resolve-level sweep)

Executor Sonnet 5.5, foreground, no sub-agents, no cargo / `bun repo` / nx / bazel, no modifying git command, no ticket tool. `C` = this folder. `PL` = `✏️s/🔌️plugins`. Tools: `C/plugfix-tools/`. Nothing was compiled by cargo; evidence is `rustc +nightly-2026-07-20 --emit=metadata` over sandbox copies (see section 4) and Python twins.

## 0. Verdict

The three named defects are fixed. The sweep found and fixed further defects of the same classes (duplicate definitions, qualifier used without import, missing manifest dependency, stale source-text reader). Everything below is static; the compile list in section 6 is the proof still owed. Items I did not fix are in section 5 with evidence.

## 1. The three named defects

| # | Crate | Defect | Fix |
| --- | --- | --- | --- |
| 1 | `semio-s-artifact-energy-model-mutation-fixtures` | `⚖️laws/🦀️.rs` had `fn committed(case, label, text) -> Value` (private, JSON parse of one committed document, 6 call sites) and `pub fn committed(case) -> (snapshot, mutation)` (public, the leaf API: 40+ leaves call `fixtures::committed(&case())`). `E0428`, and the root `pub use laws::committed` then hit the private one (`E0603`). | Renamed the private one to `committed_document` (definition and 6 call sites). The public name is unchanged, so no behaviour or API loss. |
| 2 | `semio-s-artifact-norm-{vdi3805,iso16757}-model` | `app_surface::artifact_kind_spec(..)` with only `use semio_s_plugin_norm_app_surface;` (binds nothing named `app_surface`). The dependency `semio-s-plugin-norm-app-surface` was already declared in both manifests. | vdi3805: `use semio_s_plugin_norm_app_surface as app_surface;`. iso16757: the use moved into its owner, the child `🧾️artifact-definition/🦀️.rs`, and the dead root line was removed. **The same defect existed in 12 more norm snapshot crates** (en1990, en1991, en1992, en1994, en1995, en1996, en1997, en1998, en1999, din4108, din16798, din18599; en1993 already used the full path): all got the same alias line. The cause is commit 688 (a carve codemod turned `pub use X as alias;` into `use X;`). |
| 3 | `semio-s-artifact-procedural-2d-editor`, test `domain_local_static_verifier_rejects_raw_routes_and_proves_two_dimensional_coverage` | Not only the `decode_pack` word. Two independent staleness defects: (a) it took everything after `//#region 🔖️MountedCanonicalPackSession` instead of the region, so it also scanned `decode_pack_with` / `decode_document` of the generated `ArtifactPack` impls after the region (this already failed on the original file, lines 1035 and 1040); (b) since commit 673 moved the owner catalog and `Generation2dPackSnapshotAuthority` to `generation2d/🔨️modules/🏠️host`, `owner_source` (the binary mutations file) no longer contained `mutations.clear-widget-layout.2d-only` nor the authority struct, so the first assertion and the `mounted_field` slice would fail even before (a). | The test now (1) concatenates only the text between each region start and its matching end across the six snapshot files (the c6plug split repeated the region markers per child), (2) reads the host crate module tree through a new `module_tree_text(dir)` helper (root file first, then child directories in name order, `🧪️tests` and `📦️packages` skipped) for the catalog marker and the authority slice, and keeps `owner_source` for the discriminator constant that still lives in the binary mutations file. The assertion set and semantics are unchanged. |

Twin for item 3 (Python, mirrors the new algorithm byte for byte over the live tree): 17 of 17 assertions PASS (region has none of the four forbidden words, authority slice has none of the five, the three required routes are inside the region, scalar marker, both fixtures). The same twin FAILS on the old algorithm (catalog marker absent from the binary mutations file; region tail contains `decode_pack_with`).

## 2. Further defects fixed by the sweep

| Class | Where | Fix |
| --- | --- | --- |
| Duplicate test name (`E0428`) | trinity jack `💡️inferences/🧪️tests/🔬️unit/🦀️.rs`: `inference_determinism_law` and `inference_default_law` each defined twice (commit 693 appended retained-lifecycle versions). | The appended `inference_default_law` supersedes the old identical one (deleted). The old determinism law ran over the non-trivial `chain_snapshot()`, which the new one does not, so it was kept as `inference_determinism_law_over_chain_snapshot` (no coverage lost). |
| Self-referential import (`E0255`), 8 sites | drawing root (`use crate::standards::v1::subsets;` beside `pub mod subsets`), drawing schema (`pub use crate::DrawingArtboard/DrawingImageAsset/DrawingLayerNode` beside the definitions), energy `✏️edit/🦀️.rs` (`use crate::tools;` beside `pub mod tools`), puzzle2d mutations (`use ..::Puzzle2dMutation;` beside the enum), pptx and xlsx snapshots (`use ..::STDIO_*_DOCUMENT_SCHEMA;` beside the const). | Imports removed. |
| Alias dropped by the carve, `X::` used without a binding | norm en1992 (`document::LocalizedCopy`: `use semio_s_plugin_norm_compliance as document;`), cad modes edit (`picking::`, 25 uses) and cad inspection panel (`picking::`), lowpoly modes edit (`options::`), cad `edit::` in 9 files (5 window files had `use crate;` which is itself an error: now `use crate as edit;`; `use crate;` in a test with zero uses removed; panel `🗿️artifact`, its unit test, test support and the editor root now `use semio_s_artifact_cad_cad_modes as edit;`), process3d editor root (`use semio_s_artifact_process_process3d_modes as edit;`), remodeling `any/🦀️.rs` (`use io::{binary, text};`). Each alias was checked against the target crate root (for example `CAD_WORLD_PICK_GRANULARITY`, `PROCESS3D_MODE_EDIT`, `definition()`, `layout()` exist at those roots). | One-line imports. |
| Lost module body | layout editor: commit 688 left `//#region 🔖️ActionBridge` empty, `args_bridge::command_from_action` (called from `🕹️layout-play-app`) had no module (181 lines vanished; present in `HEAD~6`). | Restored verbatim from `HEAD~6` as `✏️editor/🔀️args-bridge/🦀️.rs` (same convention as bim, shooting, lowpoly, forms) and mounted with `#[path] mod args_bridge;` in the region. |
| Missing manifest dependency | bim model: `semio-s-artifact-stdio-contract` (commit 687 retargeted 76 uses). layout editor and editor-commands: `pack` (workspace entry added). semio-value: `pack`, `semio-framework-2d-compute`. din18599: `semio-s-artifact-stdio-semio-value-model` (root, snapshot, workspace table). layout tests: dev-deps engine → editor-commands and engine-export, engine-export → editor, mutations → layout-host (tests in lower crates call upper crates; dev-dependency cycles are legal). Cargo.lock dependency edges were added by hand for the same packages (`plugfix-tools/lockdep.py`). | Manifests and locks. |
| Orphaned and stale root of din18599 | The root crate kept `din18599_climate_data_from_table` (used `MonthlyClimate`, `Din18599Snapshot` with no import) and mounted a unit test that still imports through the deleted `standards::v1::..` facade. | The inverse converter moved next to its forward partner `din18599_climate_table_from_data` in the model crate; the root keeps only the `BalancingInputs` alias with a real import; the test imports from `-checks`, `-io`, `-model`, `-mut-zoning`, `-mutations`; root manifest gets `-model` (normal) and `-checks` (dev). |
| `examples()` in the wrong crate | en1995: `-model` defined `examples()` over `glulam_floor_beam::source()` etc., modules that live in `-examples` (which dev-depends on `-model`). | `examples()` moved into the `-examples` crate; the five callers (editor, schema compliance test, io-text unit test, catalogue panel test) use `semio_s_artifact_norm_en1995_examples::examples()`; manifests and locks updated. |

## 3. Files changed (all under `PL` unless stated)

- Energy fixtures: `🔋️energy/…/🧬️mutations/🧪️tests/🔬️fixtures/⚖️laws/🦀️.rs`; energy `…/✏️editor/🎭️modes/✏️edit/🦀️.rs`.
- Norm: 14 `…/🧬️schema/📸️snapshot/🦀️.rs` (en1990–en1999, din4108, din16798, din18599, vdi3805, iso16757), iso16757 `🧾️artifact-definition/🦀️.rs`; din18599 `🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `Cargo.toml`, `Cargo.lock`, root and snapshot `Cargo.toml`; en1995 `📚️examples/🦀️.rs`, editor `🦀️.rs`, four caller tests, editor, io-text and schema `Cargo.toml`, `Cargo.lock`.
- Generation2d test: `🌀️procedural/…/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs`.
- bim model: `Cargo.toml`, package `Cargo.toml`, `Cargo.lock`.
- Layout: editor `🦀️.rs`, new `✏️editor/🔀️args-bridge/🦀️.rs`, `Cargo.toml` (workspace `pack`), editor, editor-commands, engine, engine-export and mutations manifests, `Cargo.lock`.
- semio-value: package `Cargo.toml`, `Cargo.lock`. Trinity jack inference unit test. stdio pptx and xlsx snapshots. Drawing root and schema. Puzzle2d mutations. cad: modes edit `🦀️.rs`, five window files, config window test, panels inspection, panels artifact (+ unit test), editor root, test support. lowpoly modes edit. process3d editor root. remodeling `any/🦀️.rs`.
- Tools only: `C/plugfix-tools/{sweep,classify,summary,report,readers,lockdep,manifestdep}.py`.

## 4. Method, evidence and what the oracle cannot see

`sweep.py <crate manifest dir>`: copy the crate source tree and every `#[path]`/`include` target into a sandbox below `/Users/ueli/.cache/plugfix`, strip `extern crate` lines, neutralise external globs (as the c6plug oracle), run `rustc --emit=metadata` for lib and `--test`, keep every error. `classify.py` buckets the errors: DUP (`E0428 E0252 E0254 E0255 E0201 E0592`), UNDECLARED (first path segment that is neither a dependency, a dependency's **lib name** (framework `pack` is package `semio-framework-pack` with `[lib] name = "pack"`: the first run missed this), an `extern crate` alias nor `std`), MISSING, IMPORT, PRIVATE, NOISE.

| Check | Result |
| --- | --- |
| Sensitivity, seeded regression: energy `committed_document` reverted in the sandbox | oracle reports `E0428 the name committed is defined multiple times` and `E0603 function committed is private` (the exact c6plug symptoms) |
| Touched-today crates (46 by file mtime, grown to 64 with the crates I edited), lib and `--test`, final run | DUP 0, PRIVATE 0; UNDECLARED left: `transform_tool` (macro-generated by `machine::statechart!`), host `create_generation` (arrives through an external glob; the same unqualified use is at `HEAD`), and the drawing crates (section 5) |
| Broader: every plugin crate with a lib root, lib cfg (1,888 analysed, 94 have no `src`) | 8 DUP, all 8 fixed (section 2); the UNDECLARED leads are in section 5 |
| Rust syntax of all 15 edited Rust files | `rustfmt --check` parse errors 0 |
| Source-boundary gate on changed closures (`census-own.ts`) | layout-editor lib 0.565, test 0.923 (bytes bind: 241,913 of 262,144), din18599 root 0.063/0.375, din18599-model 0.301, en1995-examples 0.281/0.406, en1995-model 0.223 |

Usage: `python3 C/plugfix-tools/sweep.py [--cfgs lib] [--raw DIR] <manifest dir>…` then `classify.py` and `summary.py <classified.txt> UNDECLARED DUP …`; `readers.py <dir>…` for the static-reader check; `lockdep.py` and `manifestdep.py` are the edit helpers. The tools build `libnames.json` and `libroots.json` under `/Users/ueli/.cache/plugfix` on first use (about 20 s).

Not visible to this oracle: macro-defined names (`app_commands!`, `labels!`, `statechart!`, derives), names that arrive through an external glob, field and method privacy (`E0616`, `E0624`), trait-in-scope (`E0599`), coherence, and item-level resolution inside external crates (an alias import is checked against the crate root by reading the source, not by rustc). `IMPORT` (1,414 for the touched set) is dominated by these and was not treated as findings.

## 5. Not fixed (evidence for the owners)

1. **vdi3805 root crate `semio-s-artifact-norm-vdi3805` (`🏭️vdi3805/🦀️.rs`, 820 lines) is a half-carved remnant** (2,685 lines at `HEAD~6`). It still holds `VdiValue`, `SchemaCatalog`, `CorrectionOverlay`, ten `*_blatt_*_dataset` builders and `impl From<VdiQuantityKind> for QuantityKind` with no import of any model type, and the helpers it calls (`sheet_attribute_*`, `vdi3805_invalid`, `bilingual`) are private to the model children. The orphan impl needs to move into the model crate (it depends on `semio-s-plugin-norm-compliance`), the rest into the model or the examples crate; the mounted root tests (`part_NN::check`, `…_examples`) are stale in the same way. Not a one-line fix and it overlaps the vd carve; left for its owner.
2. **generation3d** has the twin of item 1.3 (`🧊️generation3d/…/🔬️mounted-registry/🦀️.rs`), but a peer changed its paths from `../../../` to `../../../../../../` at 04:37 and is splitting the editor (`MM` status), so I did not touch it. Its `owner_source` read will have the same moved-to-host problem (`mutations.delete-widget-position.3d-only`, `Generation3dPackSnapshotAuthority` now under `generation3d/🔨️modules/🏠️host`); verify with the same helper.
3. **drawing** (peer split in flight; `mod args_bridge` vanished between `HEAD~1` and the working tree): UNDECLARED `pack` (drawing `🛂️manifest` and the schema crate, 38 uses), `semio_s_artifact_draw_drawing_model` (schema io binary and sqlite snapshots, 17), `…_drawing_io` and `…_drawing` (tests), plus `args_bridge`, `export_job`, `image_job`, `path_job`.
4. **Plugin-wide UNDECLARED leads** from the lib-only run (first-party names only; `C/🗑️generated` was cleaned, rerun `sweep.py` + `classify.py` to regenerate): `semio_repo_test_host` in 45 oracle crates (stdio `🔮️oracles`), `pack` in the layout io crates (35 uses; the `io` crates still lack the dependency I added to editor and commands), block `semio_s_artifact_block_{2d,3d,5d}_mut_*` (13 names in editor and mutation crates), puzzle2d `semio_s_artifact_puzzle_2d` and `…_unit_tests`, stdio ifc `artifact`/`snapshot`/`mutations`/`text`/`inferences`, vdi3805 io-text `standards`, lowpoly editor `edit`, cad kit `interaction`, `semio_framework_ui_contract` in norm app-surface, `semio_framework_action_bus` in raster editor, dag, wires, playbook, presentation, bmp, flow mutations dependencies, `semio_s_artifact_layout_layout_editor_panels`. External libraries in the oracle crates (`quick_xml`, `zip`, `lopdf`, `image`, …) are feature-gated dev oracles and were not judged.
5. **Stale cross-crate references in tests** (`MISSING`, 194 in the touched set), for example wfc `crate::editor::wfc2d::*`, puzzle `crate::editor::puzzle2d::*`, pdf `crate::standards::v1_7::..::io::export::encode_pdf`: the editor/io components were carved into other crates but their tests still address the old paths. They belong to the carve owners of those areas.
6. Source-text readers: `readers.py` found only the two generation tests above as true stale readers (markers absent from every read target); puzzle 2d/5d, flow and fem readers resolve relative to a manifest directory the tool could not determine for the mirrored `🪞️mirror` crates (false positives, not judged).
7. `Cargo.lock` of en1995 has no `-io-text` package at all (stale lock; the three edges I added are for packages it does have). The refresh of the 179 stale locks (cg1 note) is still needed.

## 6. Crates to compile

Each plugin directory is its own Cargo workspace, so run one `check` per workspace root (norm: per artifact directory). Packages whose sources or manifests changed:

```
semio-s-artifact-energy-model-mutation-fixtures      semio-s-artifact-energy-model-editor-edit
semio-s-artifact-norm-{en1990,en1991,en1992,en1993,en1994,en1995,en1996,en1997,en1998,en1999,din4108,din16798,din18599,vdi3805,iso16757}-model
semio-s-artifact-norm-din18599                       semio-s-artifact-norm-en1995-{examples,editor,io-text,schema}
semio-s-artifact-procedural-2d-editor (--tests)      semio-s-artifact-bim-model
semio-s-artifact-layout-layout-{editor,editor-commands,engine,engine-export,mutations}  (--tests)
semio-s-artifact-stdio-semio-value                   semio-s-artifact-stdio-{pptx,xlsx}
semio-s-artifact-trinity-jack (--tests)              semio-s-artifact-draw-drawing{,-model}
semio-s-artifact-puzzle-2d                           semio-s-artifact-cad-cad-{modes,panels,editor}
semio-s-artifact-lowpoly-lowpoly-modes               semio-s-artifact-process-process3d-editor
semio-s-artifact-remodel-remodeling
```

`bun repo cargo --store=shared --configuration=prototype -- check -p <package…> --lib --tests --keep-going --message-format=short` from each workspace. Expected remaining failures (not caused by this work): section 5 items 1, 3 and 5.

## 7. Open risks

- Nothing was compiled; the five hand-edited `Cargo.lock` files may be re-sorted by cargo (harmless).
- New dev-dependency cycles (layout engine ↔ editor-commands/engine-export, engine-export ↔ editor, mutations ↔ host, en1995 io-text/schema ↔ examples) are legal for cargo but move layering debt into the dev graph.
- Layout-editor test closure is at 0.923 of the byte gate; another split or import block needs headroom.
- `args_bridge` was restored from `HEAD~6`; if a peer split of the layout editor also re-creates it, there will be a second `mod args_bridge` (`E0428`): grep before merging.
- The din18599 root tests mount two other test files (`diff_unit`, `sqlite_snapshot_tests`) that I did not audit beyond the oracle's clean result for lib and test cfg.
