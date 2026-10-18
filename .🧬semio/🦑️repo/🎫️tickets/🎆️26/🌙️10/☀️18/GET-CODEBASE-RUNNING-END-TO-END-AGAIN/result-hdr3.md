# Result HDR3 (executor prefix `hdr3`): crate source-gate census 2026-10-18 and carves

Sonnet 5.5, foreground, no sub-agents, no cargo/nx/`bun repo`, no modifying git command. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/hdr3-tools/` (`census.ts`, `near.ts`, `cl.ts`, `meas.ts`, `pos/` oracle copy, `spec-*.json`); the generators (`movetests.py`, `movemods.py`, `mkcomp.py`, `h3reg.py`, `usetree.py`, `loop.sh`, `pre.sh`, `cfgtest.py`, `widen_names.py`, `peel_pdf.py`, `peel_repomodel.py`) sit in `/Users/ueli/.cache/hdr3/tools/` (copied to `F/hdr3-tools/gen/`). Backups of first-touched shared files: `F/hdr3-tools/orig/`. Census logs: `F/🗑️generated/hdr3-*` (`hdr3-crate-census.csv` is the fresh census, `hdr3-crate-census-final.csv` the one after the carves).

## 0. Verdict

Fresh census (04:0x, 3,071 crates, canonical reader, lib and test closures of every target): **2 crates over the gate** (`stdio-pdf-model` lib 37 files = 1.156, `bim-model` 40x), 24 at >= 0.95. Final census (05:1x, 3,320 crates): **0 lib over the gate, 0 test closure over the gate**, 9 crates still >= 0.95 (section 3), 147 >= 0.90. `bim-model` (cg2), `puzzle-2d-retained` (0.938) and `os-store-tests-05` (0.906) were fixed by peers while I worked. **13 crates carved to <= 0.90 by me** (section 1); every carve was checked with the typeck/privacy oracle (`pos`, nightly-2026-07-20 `--emit=metadata`, baseline git `7880cc36f78`): lib and test NEW 0 / privacy 0 / resolve 0 / type 0 / other 0 non-stub in every family. Nothing was compiled.

## 1. Carved (worst ratio before -> after, owner / new crate)

| crate | before | after (owner lib / test) | new crate | technique |
|---|---|---|---|---|
| `semio-s-plugin-energy-engine-sim` | 0.968 lines | 0.755 / 0.769 | `-unit-tests` (0.205) | whole test root to companion, `testkit` glob of the six hub modules, fields/methods widened by `pos2widen`, five `cfg(test)` consts gated `any(test, testkit)` |
| `semio-framework-os-kernel-db-sync` | 0.993 lines | 0.690 / 0.729 | `-laws` (0.264) | three law files (`wal`, `codec`, `retained hello`) to a framework companion; 4 root `cfg(test)` imports removed; `serde_json` dev-dep added (the wal law used it undeclared); fixture literals re-based |
| `semio-framework-3d-brep-construction` | 0.967 lines | 0.725 / 0.827 | `-unit-tests` (0.188) | primitives (4 mounts) and sew heal tests; `is_degenerate_edge` + `Edge` import gated `testkit` |
| `semio-framework-os-kernel-db-io-core` | 0.956 lines | 0.865 / 0.890 | `-laws` (0.094) | retained-fixtures + unit tests; testkit glob |
| `semio-s-artifact-procedural-3d-editor` | 0.967 bytes | 0.846 / 0.847 | `-unit-tests` (0.140) | wasm mounted-registry laws |
| `semio-s-artifact-gis-gismap-gis2d` | 0.963 lines | 0.598 / 0.599 | `-unit-tests` (0.365) | all five test mounts (editor unit root with the shared `context`, commands, panels, features, map window); `crate::` -> owner paths; `history_edit_acceptance_law!` path re-based |
| `semio-s-artifact-vcs-vcs-editor` | 0.969 files | 0.781 / 0.781 | `-unit-tests` (0.293) | all six test mounts; `text_edit_operations` widened to `pub`; law path + 3 runtime fixture literals re-based |
| `semio-s-artifact-stdio-semio-document-examples` | 0.969 files | 0.781 / 0.781 | `-unit-tests` (0.219) | six tiny test mounts |
| `semio-framework-plugin-interpreter` | 0.950 lines | 0.852 / 0.877 | `-core-laws` (0.077) | the first test (`original_owned_job_export_selectors...`, referenced by `📜️script.ts` `test-job-exports`) stays; the wasm core laws moved; companion has `📋️project.json` + 6 launch routes |
| `semio-framework-ui-prepared` | 0.958 lines | 0.645 / 0.699 | `-laws` (0.263) | unit root + `physical-job-close` child; `ALLOCATOR` (HeapWitness) and `measured` moved to the companion root; `use super::tests::` re-pointed |
| `semio-framework-os-renderer-wgpu-dock` | 0.953 lines | 0.538 / 0.539 | `-laws` (0.416) | four test mounts (non-wasm gate kept on the companion modules); inner `//!` docs kept first in each moved file |
| `semio-s-artifact-stdio-pdf-model` | **1.188 files (over the gate)** | 0.813 / 0.844 | `semio-s-artifact-stdio-pdf-model-profiles` (0.406) | the 11 tiny profile mirror leaves (`editor::pdf14*/pdf17a`, `e`/`h` io) peeled; the 11 `pub use` lines of `📖️pdf/🦀️.rs` and the root manifest point to the new crate |
| `semio-framework-repo-model` | 0.969 lines | 0.775 / 0.775 | `semio-framework-repo-model-entity` (0.194) | `ansi` + `entity` modules moved (files `mv`'d, root keeps no re-export); 7 consumer files in 4 crates (`event-stream`, `tree-source-and-search`, `verbs`, `tree`) now import `semio_framework_repo_model_entity`; the new crate depends on `repo-model` for `derive_file_kind`/`build_section_id` |

Registrations (no compat re-exports): new members in the owning workspace `Cargo.toml` (energy, layout none, gis, vcs, document, pdf, root, 🧰️framework), owner rows in `📋️build-owners.json` (crate-name rows; `db-sync-laws`, `db-io-core-laws` and `plugin-interpreter-core-laws` use `@semio-tech/<id>-rs` rows with `📋️project.json` and six `launch.json` routes each, next orders after 96042), `testkit = []` feature added to the owners that had none (energy-sim, brep-construction, procedural-3d-editor, db-io-core, interpreter, ...), companion dev-dependencies computed from the moved closure. Gates: no `launch.json`/seed entry ran a moved test by name (checked `energy-engine-sim`, `db-sync`, `db-io-core`, `ui-prepared` [`prepared-close` runs a TS test, `physical-close-native` filters the `ui` crate], `interpreter` [script gate test stays in the owner]); no TS/JSON reader reads a moved path (grepped). Moved tests that used manifest-relative paths (`fixture_text!`, `fixture_path!`, `runtime_fixture_text(`, `history_edit_acceptance_law!`) were re-based to the companion manifest dir.

## 2. Reverted / not touched on purpose

* `semio-s-artifact-layout-layout-engine-export` (0.996 test lines): started and **fully reverted** (backups restored, member and owner row removed). Its test files reference `LayoutExportJobFactory`, `LayoutExportToolPayload`, `StepOutcome`, `JobPayloadCloseStep` and 70 other names that no file of the export crate defines (they live in the editor crate), so the laws do not resolve in the working tree; moving them would only relocate the defect. Needs the owner of the tool-job-factory work.
* `procedural-3d-editor` was first moved with a bug of my generator (it dropped a nested `use super::*;`), detected by the oracle (85 resolve rows), reverted, fixed and redone; the final state is the clean one above.

## 3. Still >= 0.95 (exact list, final census) and why

| crate | ratio | reason |
|---|---|---|
| `semio-s-artifact-layout-layout-engine-export` | 0.996 lines (test) | section 2 (broken laws; an `-unit-tests` carve is a mechanical `movetests.py` run once the references resolve) |
| `semio-framework-plugin-instance-app` | 0.988 bytes (lib 0.950) | one type `VcsArtifactApp` with inherent role impls in every module (`plugin-app-impl` 72 KB, `staging/*`); inherent impls cannot leave the type's crate; needs a type-core crate with public fields plus extension traits |
| `semio-s-artifact-forms-forms-commands`, `stdio-pptx-base-model`, `stdio-semio-table-mutations`, `stdio-semio-audio-mutations` | 0.969 files (lib 30-31 of 32) | mutation leaves implement `MutationKind<Snapshot, Aggregate>` and the inverse names `Aggregate::Variant`, the aggregate enum names every leaf: cycle, no leaf family can move |
| `semio-s-artifact-stdio-svg-snapshot` | 0.968 lines | the text/binary snapshot and diff modules call each other (printers <-> diff codecs); only a tree partition of the mirror helps |
| `semio-framework-ui-contract-values` | 0.963 lines (test) | grew from 0.949 during this run (peer edits); not analysed |
| `semio-framework-quiz-tests` | 0.956 bytes | the seven unit files share `pub(crate)` fixtures (`crate::sheet::tests`, `crate::lifecycle::tests`); needs a fixtures support crate first, then a second companion for `views`/`challenge` |

Known near-gates asked for in the brief: `ex` (= layout export, above) and `instance-app` (above); `pdf p3` and `pdf` parts are all <= 0.906 now; `semio-framework-plugin-reactor-turn` stays at 0.909 (lib bytes 238,196; 2.3 KB over the 0.90 line, no cohesive module peeled); `semio-framework-os-kernel-db-sync` 0.729, `3d-brep-construction` 0.827 are done.

## 4. Crates to compile (`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`, after the offline lock refresh of the touched workspaces: energy, gis/gismap, vcs, stdio/semio, procedural/generation3d, stdio/pdf, 🧰️framework, root)

New: `semio-s-plugin-energy-engine-sim-unit-tests`, `semio-framework-os-kernel-db-sync-laws`, `semio-framework-3d-brep-construction-unit-tests`, `semio-framework-os-kernel-db-io-core-laws`, `semio-s-artifact-procedural-3d-editor-unit-tests`, `semio-s-artifact-gis-gismap-gis2d-unit-tests`, `semio-s-artifact-vcs-vcs-editor-unit-tests`, `semio-s-artifact-stdio-semio-document-examples-unit-tests`, `semio-framework-plugin-interpreter-core-laws`, `semio-framework-ui-prepared-laws`, `semio-framework-os-renderer-wgpu-dock-laws`, `semio-s-artifact-stdio-pdf-model-profiles`, `semio-framework-repo-model-entity`.
Owners (feature `testkit` now exists on them; check with `--features testkit` too): `semio-s-plugin-energy-engine-sim`, `semio-framework-os-kernel-db-sync`, `semio-framework-3d-brep-construction`, `semio-framework-os-kernel-db-io-core`, `semio-s-artifact-procedural-3d-editor`, `semio-s-artifact-gis-gismap-gis2d`, `semio-s-artifact-vcs-vcs-editor`, `semio-s-artifact-stdio-semio-document-examples`, `semio-framework-plugin-interpreter`, `semio-framework-ui-prepared`, `semio-framework-os-renderer-wgpu-dock`, `semio-s-artifact-stdio-pdf-model`, `semio-s-artifact-stdio-pdf` (root, 11 re-pointed lines), `semio-framework-repo-model`.
Consumers of the repo split: `semio-framework-repo-cli-event-stream`, `semio-framework-repo-cli-tree-source-and-search`, `semio-framework-repo-cli-verbs`, `semio-framework-repo-tree`.
Tests to run: each new companion crate (`cargo test -p <crate> --lib`; db-sync/db-io-core/interpreter companions also via `@semio-tech/<id>-rs:test`), plus the owners' remaining lib tests.

## 5. Open risks (not compiled)

* The oracle stubs everything outside a family, so cross-crate **feature gating** is only checked by hand: items used by moved tests that are `cfg(test)` in the owner were gated `any(test, feature = "testkit")` (`cfgtest.py` lists them; last run of each carve: 0 hits left), but a companion dev-dependency feature (e.g. a `testkit` of a third crate) that only the old in-crate test target enabled could still be missing.
* Widened visibilities (`pub(super)`/private -> `pub`, applied by `pos2widen`) are not `#[doc(hidden)]`; they sit in private modules or behind the `testkit` glob. `EnergyJobAuthority` already was `#[doc(hidden)] pub` with public fields.
* `plugin-interpreter-core-laws`: the owner's `📜️script.ts` `test` target only runs the owner lib; the moved laws run through the companion's own routes.
* `semio-framework-test-fixtures` macros resolve against `CARGO_MANIFEST_DIR`; all literal paths of moved files were re-based and logged (`rebased fixture literals` lines), non-literal uses were scanned for (none left).
* `Cargo.lock` of the eight touched workspaces needs `update --workspace --offline`; nothing was hand-edited.
