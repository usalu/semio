# 🪓 Monolith Census 4

- Date: 2026-10-16. Auditor: Haiku 5.5 (read-only). Folder: `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
- Scope: the census 3 method. Tracked plus untracked `*.rs`, `*.ts`, `*.tsx`, `*.py`, `*.wit`, `*.css`, minus `.🧬semio`, `generated` and `node_modules` components, `.d.ts`, and fixture, snapshot and corpus folders. The filter was checked first: applied to census 3's own tracked and untracked lists it reproduces its 42,488 files exactly. Lines are newline counts (the same as `wc -l`). `git ls-files` ran read-only with `-c core.fsmonitor=false`.
- Running slices taken as given: `pi2`, `pgr`, `tfe`, `rsl`, `ldd`, `slf`, `e51`, `ecp`. Peer areas and the parent hold are as the brief lists them.
- Raw outputs: `🗑️generated/mc4-rows.json` (all kept files with lines), `mc4-claims.json` (slice, RESULT, PLAN and HANDOFF evidence), `mc4-over1000.json` (every file over 1,000 lines with its class), `mc4-packages.json`, and the scripts `mc4-count.py`, `mc4-classify.py`, `mc4-free.py`, `mc4-flags.py`, `mc4-packages.py`, `mc4-report.py`, `mc4-filter-probe*.py`.
- No source file was edited. No modifying git command was run. No sub-agent was used. No ticket tool was called. No recursive grep or find ran over the repo root. One recursive grep was composed from the repo root during the data-check step, but zsh aborted it at glob expansion before it ran, so nothing was searched.

## 1. Totals compared with census 3

| Metric | Census 3 | Now | Delta |
| --- | ---: | ---: | ---: |
| Files kept | 42,488 | 43,320 | +832 |
| Lines kept | 4,718,753 | 4,730,407 | +11,654 |
| Files over 1,500 lines | 267 | 258 | -9 |
| Lines in files over 1,500 | 642,759 | 623,221 | -19,538 |
| Files over 1,000 lines | 658 | 638 | -20 |
| Lines in files over 1,000 | 1,121,171 | 1,086,090 | -35,081 |

The 658 files that census 3 listed over 1,000 lines now split out as: 632 still over 1,000 lines, 21 now at or below 1,000 lines, and 5 no longer kept (split or moved). 4 files are over 1,000 lines now but were not in census 3 (5,226 lines), and 2 grew from 1,000 lines or fewer to over 1,000. Those are listed in `mc4-over1000.json`.

## 2. Classification of the 638 files over 1,000 lines

Each file gets the first matching class, in this order: VENDORED, PARENT, PEER (path rule), RUNNING (exact file or crate/package-directory claim by a running slice), FINISHED (full path named in a RESULT, AUDIT, DEPENDENCY, CRATE-SOURCE-GATE-CENSUS or PRODUCER-CRATE-RECIPE document), HANDED (named in HANDOFFS groups 1 to 4), then FREE. A file with no match is FREE. FREE means no claim was found in the evidence. It does not mean the file has been checked. Peer rules: `ui-contract` = `🖱️ui/🧬️contract`; `2d` = `◻️2d` or `♾️infinite/🎲️board`; `job` = `💼️job` or `🧵️job`; `store` = any `🏪️store`; `db` = `🛢️db`; `flow` = `🌊️flow`; `ui-wgpu-cut` = `🖱️ui/` with `🧊️wgpu`; `ui-aggregate-react` = any other `🖱️ui/`; `actor`, `member`, `media` = `🎭️actor`, `🪟️member`, `🎬️media`; `3d-mesh-brep` = `framework/🔨️modules/🧊️3d/`.

| Class | Files | Lines |
| --- | ---: | ---: |
| FREE | 408 | 624,271 |
| PEER | 174 | 328,997 |
| PARENT | 19 | 53,477 |
| RUNNING | 16 | 25,621 |
| FINISHED | 18 | 35,212 |
| VENDORED | 3 | 18,512 |
| Total | 638 | 1,086,090 |

PEER breakdown by rule: ui-aggregate-react 42 (89,065 lines), db 31 (51,882 lines), store 29 (54,916 lines), 2d 24 (46,329 lines), ui-wgpu-cut 13 (26,907 lines), flow 13 (23,628 lines), 3d-mesh-brep 10 (15,628 lines), ui-contract 5 (8,147 lines), job 4 (6,515 lines), actor 3 (5,980 lines).

RUNNING files (exact or directory claims, none of them free):

- 2391 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`, by `e51`
- 2386 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs`, by `pi2`
- 2003 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`, by `pi2`
- 1896 lines, `🌎️hub/🧪️tests/🔬️bin-unit/🚀️boot/🔬️unit/🦀️.rs`, by `e51`
- 1821 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`, by `pi2`
- 1728 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`, by `pi2`
- 1720 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`, by `pi2`
- 1672 lines, `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs`, by `e51`
- 1458 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`, by `pi2`
- 1398 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`, by `e51`
- 1364 lines, `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs`, by `e51`
- 1302 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`, by `e51`
- 1241 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs`, by `pi2`
- 1137 lines, `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs`, by `e51`
- 1096 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs`, by `pi2`
- 1008 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs`, by `pi2`

FINISHED files (named by a finished document; excluded from free as the conservative reading of "split by a finished slice"):

- 3643 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`, named in RESULT-PLUGIN-RUNTIME-TSX.md
- 3070 lines, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs`, named in RESULT-PACK-QUIZ-FILE-SPLITS.md
- 3017 lines, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 2670 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`, named in RESULT-MISSING-TEST-INPUTS.md,RESULT-SMALL-FIXES-1.md
- 2311 lines, `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`, named in RESULT-REPO-TEST-VSCODE-SPLIT.md,RESULT-WRONG-DIRECTION-RELOCATION.md
- 2215 lines, `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`, named in RESULT-PRIVACY-ORACLE-SWEEP-2.md
- 2096 lines, `🌎️hub/🧪️tests/🔬️bin-unit/📇️directory/🔬️unit/🦀️.rs`, named in RESULT-REPO-HUB-RUST-FILE-SPLITS.md
- 1958 lines, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1896 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts`, named in RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md
- 1703 lines, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1596 lines, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`, named in RESULT-PACK-QUIZ-FILE-SPLITS.md
- 1583 lines, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1439 lines, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1363 lines, `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1310 lines, `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`, named in RESULT-OS-HOST-MCP-WORKSPACE.md
- 1286 lines, `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1046 lines, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md
- 1010 lines, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs`, named in AUDIT-REGIONS-NAMES-SIZES.md

VENDORED: `♻️mit-bestand/…/layout.css` (16,454 lines), `main.css` (1,050 lines) of the same vendored export, and `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` (1,008 lines). Vendored content is not a split target.

## 3. Free files over 1,000 lines, ranked

Flags: `X` plugin family (only the PLAN-PLUGIN-CRATES plan names it; no running slice holds the file), `W` renderer wgpu path (may be the peer `G-11` UI GPU cut), `M` the module root is named in a finished RESULT (the file itself is not), `G` generated parser by signature (see section 6). Package numbers refer to section 5.

| Rank | Lines | Lang | Package | Flags | Path |
| ---: | ---: | --- | ---: | --- | --- |
| 1 | 10,471 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` |
| 2 | 3,416 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` |
| 3 | 3,393 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` |
| 4 | 3,347 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` |
| 5 | 3,172 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs` |
| 6 | 3,026 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔭️unit-2/🦀️.rs` |
| 7 | 3,010 | ts | 3 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` |
| 8 | 2,967 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs` |
| 9 | 2,950 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🦀️.rs` |
| 10 | 2,931 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs` |
| 11 | 2,849 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` |
| 12 | 2,797 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 13 | 2,769 | rust | 15 |  | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` |
| 14 | 2,730 | ts | 14 | XG | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts` |
| 15 | 2,725 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/🦀️.rs` |
| 16 | 2,667 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` |
| 17 | 2,593 | rust | 36 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs` |
| 18 | 2,574 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs` |
| 19 | 2,548 | ts | 14 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts` |
| 20 | 2,526 | rust | 12 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 21 | 2,525 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs` |
| 22 | 2,524 | rust | 7 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs` |
| 23 | 2,521 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` |
| 24 | 2,511 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🦀️.rs` |
| 25 | 2,505 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs` |
| 26 | 2,498 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` |
| 27 | 2,447 | rust | 17 | X | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 28 | 2,447 | ts | 3 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` |
| 29 | 2,424 | rust | 18 | X | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️.rs` |
| 30 | 2,422 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/🦀️.rs` |
| 31 | 2,418 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs` |
| 32 | 2,407 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs` |
| 33 | 2,399 | rust | 4 | XM | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs` |
| 34 | 2,397 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🦀️.rs` |
| 35 | 2,392 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🦀️.rs` |
| 36 | 2,389 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🦀️.rs` |
| 37 | 2,381 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` |
| 38 | 2,360 | rust | 12 | X | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs` |
| 39 | 2,340 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs` |
| 40 | 2,326 | ts | 21 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts` |
| 41 | 2,321 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` |
| 42 | 2,311 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` |
| 43 | 2,291 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs` |
| 44 | 2,286 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs` |
| 45 | 2,248 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🦀️.rs` |
| 46 | 2,241 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` |
| 47 | 2,241 | rust | 28 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 48 | 2,233 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🦀️.rs` |
| 49 | 2,227 | rust | 48 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/🦀️.rs` |
| 50 | 2,211 | ts | 61 |  | `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts` |
| 51 | 2,195 | rust | 4 | X | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 52 | 2,192 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🦀️.rs` |
| 53 | 2,163 | rust | 4 | XM | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs` |
| 54 | 2,162 | rust | 25 | XM | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs` |
| 55 | 2,155 | rust | 36 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` |
| 56 | 2,093 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs` |
| 57 | 2,092 | ts | 33 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` |
| 58 | 2,083 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` |
| 59 | 2,047 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🦀️.rs` |
| 60 | 2,047 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs` |
| 61 | 2,037 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| 62 | 2,032 | rust | 23 | X | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs` |
| 63 | 2,032 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs` |
| 64 | 2,026 | rust | 7 | X | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 65 | 2,018 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs` |
| 66 | 2,003 | rust | 1 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs` |
| 67 | 1,999 | rust | 7 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 68 | 1,995 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/🦀️.rs` |
| 69 | 1,989 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` |
| 70 | 1,969 | ts | 56 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts` |
| 71 | 1,956 | rust | 4 | X | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 72 | 1,953 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` |
| 73 | 1,952 | rust | 17 | X | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs` |
| 74 | 1,947 | rust | 9 | X | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 75 | 1,939 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 76 | 1,932 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👆️gestures/🦀️.rs` |
| 77 | 1,924 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/🦀️.rs` |
| 78 | 1,906 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs` |
| 79 | 1,899 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs` |
| 80 | 1,898 | ts | 3 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts` |
| 81 | 1,897 | rust | 7 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/🦀️.rs` |
| 82 | 1,891 | rust | 28 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` |
| 83 | 1,891 | ts | 50 |  | `🧰️framework/🔨️modules/🧾️app-channel/🟦️.ts` |
| 84 | 1,879 | rust | 30 | X | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 85 | 1,877 | rust | 18 | X | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/✉️envelope/🦀️.rs` |
| 86 | 1,859 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` |
| 87 | 1,858 | ts | 53 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` |
| 88 | 1,857 | rust | 41 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🌫️fuzzy-internals/🦀️.rs` |
| 89 | 1,851 | ts | 3 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx` |
| 90 | 1,851 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🩹️repair/🦀️.rs` |
| 91 | 1,849 | rust | 42 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` |
| 92 | 1,843 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🦀️.rs` |
| 93 | 1,841 | rust | 38 |  | `🌎️hub/🧪️tests/🔬️bin-unit/🧭️open-plan/🔬️unit/🦀️.rs` |
| 94 | 1,835 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs` |
| 95 | 1,814 | ts | 46 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx` |
| 96 | 1,806 | ts | 3 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🟦️.tsx` |
| 97 | 1,799 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗂️catalog/🦀️.rs` |
| 98 | 1,796 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs` |
| 99 | 1,782 | rust | 30 | X | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs` |
| 100 | 1,782 | ts | 46 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx` |
| 101 | 1,779 | rust | 15 |  | `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs` |
| 102 | 1,750 | rust | 4 | XM | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🔏️digest/🦀️.rs` |
| 103 | 1,745 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️timber/🦀️.rs` |
| 104 | 1,745 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` |
| 105 | 1,744 | ts | 54 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` |
| 106 | 1,733 | rust | 41 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧭️causal-internals/🦀️.rs` |
| 107 | 1,710 | rust | 12 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 108 | 1,684 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️.rs` |
| 109 | 1,662 | ts | 53 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit` |
| 110 | 1,662 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🦀️.rs` |
| 111 | 1,660 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` |
| 112 | 1,652 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🦀️.rs` |
| 113 | 1,652 | ts | 33 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx` |
| 114 | 1,645 | rust | 38 |  | `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs` |
| 115 | 1,631 | ts | 50 | G | `🧰️framework/🔨️modules/🛰️backbone/🟦️.ts` |
| 116 | 1,623 | ts | 57 |  | `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts` |
| 117 | 1,616 | rust | 4 | XM | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🏟️arena/🦀️.rs` |
| 118 | 1,613 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🦀️.rs` |
| 119 | 1,611 | rust | 15 |  | `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` |
| 120 | 1,600 | ts | 3 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts` |
| 121 | 1,598 | rust | 23 | X | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs` |
| 122 | 1,595 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs` |
| 123 | 1,586 | ts | 46 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx` |
| 124 | 1,585 | rust | 15 |  | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` |
| 125 | 1,583 | rust | 37 |  | `🧰️framework/🔨️modules/✍️editor/🦀️.rs` |
| 126 | 1,576 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` |
| 127 | 1,574 | rust | 25 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 128 | 1,572 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🦀️.rs` |
| 129 | 1,569 | rust | 32 | X | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs` |
| 130 | 1,566 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx` |
| 131 | 1,565 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` |
| 132 | 1,562 | ts | 53 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` |
| 133 | 1,561 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🦀️.rs` |
| 134 | 1,555 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs` |
| 135 | 1,548 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧱️instance-layers/🟦️.tsx` |
| 136 | 1,540 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🦀️.rs` |
| 137 | 1,536 | rust | 28 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗂️document/🦀️.rs` |
| 138 | 1,529 | rust | 41 | X | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs` |
| 139 | 1,518 | rust | 25 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 140 | 1,513 | rust | 17 | X | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` |
| 141 | 1,512 | rust | 62 |  | `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs` |
| 142 | 1,501 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/🦀️.rs` |
| 143 | 1,497 | rust | 12 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 144 | 1,496 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🦀️.rs` |
| 145 | 1,495 | ts | 33 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` |
| 146 | 1,494 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs` |
| 147 | 1,493 | rust | 28 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🦀️.rs` |
| 148 | 1,487 | rust | 18 | X | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 149 | 1,478 | rust | 32 | X | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs` |
| 150 | 1,477 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs` |
| 151 | 1,476 | rust | 15 | M | `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` |
| 152 | 1,469 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| 153 | 1,459 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs` |
| 154 | 1,455 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs` |
| 155 | 1,451 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/🦀️.rs` |
| 156 | 1,449 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🦀️.rs` |
| 157 | 1,448 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` |
| 158 | 1,446 | ts | 54 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` |
| 159 | 1,443 | ts | 52 |  | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖱️interaction-dom/🟦️.tsx` |
| 160 | 1,440 | rust | 7 | X | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 161 | 1,435 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🦀️.rs` |
| 162 | 1,432 | ts | 57 | W | `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration/🟦️.ts` |
| 163 | 1,432 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs` |
| 164 | 1,430 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs` |
| 165 | 1,429 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/🦀️.rs` |
| 166 | 1,428 | rust | 7 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs` |
| 167 | 1,427 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` |
| 168 | 1,421 | rust | 45 | M | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs` |
| 169 | 1,420 | rust | 42 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs` |
| 170 | 1,417 | rust | 45 | M | `🧰️framework/🔨️modules/🔲️pixels/🦀️.rs` |
| 171 | 1,417 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css` |
| 172 | 1,414 | ts | 58 |  | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🟦️.tsx` |
| 173 | 1,413 | rust | 38 |  | `🌎️hub/🧪️tests/🔬️bin-unit/🔐️session/🔬️unit/🦀️.rs` |
| 174 | 1,411 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` |
| 175 | 1,410 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs` |
| 176 | 1,406 | ts | 52 |  | `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css` |
| 177 | 1,405 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️.rs` |
| 178 | 1,404 | ts | 14 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts` |
| 179 | 1,402 | rust | 26 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs` |
| 180 | 1,401 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` |
| 181 | 1,400 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔎️graph-projection/🦀️.rs` |
| 182 | 1,397 | rust | 15 |  | `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` |
| 183 | 1,396 | ts | 63 |  | `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` |
| 184 | 1,395 | rust | 37 | M | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🦀️.rs` |
| 185 | 1,395 | rust | 36 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs` |
| 186 | 1,389 | rust | 12 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧲️picking/🦀️.rs` |
| 187 | 1,388 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` |
| 188 | 1,385 | rust | 48 | M | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs` |
| 189 | 1,377 | rust | 49 |  | `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🔤️vocabulary/🦀️.rs` |
| 190 | 1,376 | rust | 35 | M | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs` |
| 191 | 1,373 | rust | 37 | M | `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` |
| 192 | 1,369 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 193 | 1,369 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs` |
| 194 | 1,369 | rust | 38 |  | `🌎️hub/🧪️tests/🔬️bin-unit/🎲️hostile-input/🔬️unit/🦀️.rs` |
| 195 | 1,361 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` |
| 196 | 1,357 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs` |
| 197 | 1,350 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx` |
| 198 | 1,339 | rust | 35 |  | `🧰️framework/🔨️modules/🔄️machine/✨️derive/⚙️expansion/🦀️.rs` |
| 199 | 1,336 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx` |
| 200 | 1,332 | ts | 21 | X | `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py` |
| 201 | 1,332 | rust | 23 | X | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 202 | 1,332 | rust | 23 | X | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs` |
| 203 | 1,332 | rust | 47 | X | `🧰️framework/🛍️products/💻️os/🖥️host/🔌️plugin-host/🦀️.rs` |
| 204 | 1,331 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs` |
| 205 | 1,330 | ts | 52 |  | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖼️deck-rendering/🟦️.tsx` |
| 206 | 1,323 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/🔺️diff/🧵️wire/🦀️.rs` |
| 207 | 1,322 | rust | 25 | XM | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs` |
| 208 | 1,322 | ts | 55 |  | `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts` |
| 209 | 1,321 | ts | 50 | G | `🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts` |
| 210 | 1,318 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs` |
| 211 | 1,316 | rust | 7 | X | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` |
| 212 | 1,315 | ts | 33 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` |
| 213 | 1,314 | rust | 35 | M | `🧰️framework/🔨️modules/📡️replication/📡️wire/👥️presence/🦀️.rs` |
| 214 | 1,313 | rust | 35 | M | `🧰️framework/🔨️modules/🎓️onboarding/🦀️.rs` |
| 215 | 1,311 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs` |
| 216 | 1,310 | ts | 3 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏗️export-bim-1-ifc/🐍️.py` |
| 217 | 1,307 | rust | 42 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs` |
| 218 | 1,305 | rust | 32 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` |
| 219 | 1,301 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs` |
| 220 | 1,301 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs` |
| 221 | 1,300 | ts | 21 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔌️power-and-plant/🐍️.py` |
| 222 | 1,296 | ts | 46 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx` |
| 223 | 1,294 | rust | 7 | X | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 224 | 1,292 | rust | 5 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs` |
| 225 | 1,289 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs` |
| 226 | 1,289 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles/🦀️.rs` |
| 227 | 1,288 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs` |
| 228 | 1,283 | rust | 45 | M | `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs` |
| 229 | 1,280 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️.rs` |
| 230 | 1,278 | rust | 17 | X | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔣️value/🦀️.rs` |
| 231 | 1,278 | ts | 59 |  | `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts` |
| 232 | 1,276 | rust | 23 | X | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` |
| 233 | 1,276 | rust | 64 |  | `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/✏️editor/🦀️.rs` |
| 234 | 1,276 | ts | 50 | M | `🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts` |
| 235 | 1,274 | rust | 23 | X | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 236 | 1,274 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` |
| 237 | 1,270 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🦀️.rs` |
| 238 | 1,269 | rust | 19 | X | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` |
| 239 | 1,268 | ts | 21 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔧️zone-systems/🐍️.py` |
| 240 | 1,268 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🦀️.rs` |
| 241 | 1,264 | rust | 32 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 242 | 1,264 | rust | 48 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs` |
| 243 | 1,262 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` |
| 244 | 1,261 | ts | 21 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📸️mutate-remodeling-1/🐍️.py` |
| 245 | 1,256 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` |
| 246 | 1,251 | ts | 33 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` |
| 247 | 1,248 | rust | 28 | X | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 248 | 1,248 | rust | 35 | M | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` |
| 249 | 1,245 | rust | 4 | X | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` |
| 250 | 1,239 | rust | 18 | X | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs` |
| 251 | 1,239 | rust | 32 | X | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs` |
| 252 | 1,238 | ts | 33 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🎨️mesh-render/🟦️.tsx` |
| 253 | 1,237 | rust | 32 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs` |
| 254 | 1,237 | rust | 45 | M | `🧰️framework/🔨️modules/🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs` |
| 255 | 1,235 | rust | 38 |  | `🌎️hub/🧪️tests/🔬️bin-unit/🧰️testkit/🦀️.rs` |
| 256 | 1,233 | rust | 48 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust/🦀️.rs` |
| 257 | 1,232 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🪢️session/🦀️.rs` |
| 258 | 1,228 | rust | 65 |  | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🦀️.rs` |
| 259 | 1,226 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 260 | 1,226 | ts | 66 |  | `🧰️framework/🛍️products/🖥️server/🟦️.ts` |
| 261 | 1,217 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🦀️.rs` |
| 262 | 1,215 | ts | 21 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/📆️schedules-and-layers/🐍️.py` |
| 263 | 1,215 | rust | 36 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` |
| 264 | 1,214 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🦀️.rs` |
| 265 | 1,213 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs` |
| 266 | 1,207 | rust | 49 | M | `🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/🦀️.rs` |
| 267 | 1,206 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🦀️.rs` |
| 268 | 1,206 | rust | 36 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs` |
| 269 | 1,205 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🗜️codec/🦀️.rs` |
| 270 | 1,204 | rust | 36 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs` |
| 271 | 1,202 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs` |
| 272 | 1,198 | ts | 56 | M | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts` |
| 273 | 1,195 | rust | 48 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust/🦀️.rs` |
| 274 | 1,194 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs` |
| 275 | 1,188 | ts | 21 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/💨️airflow-and-controls/🐍️.py` |
| 276 | 1,186 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🕸️graph/🦀️.rs` |
| 277 | 1,184 | ts | 21 | X | `✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🟦️.ts` |
| 278 | 1,183 | ts | 54 |  | `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts` |
| 279 | 1,181 | ts | 14 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🐍️.py` |
| 280 | 1,180 | rust | 39 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs` |
| 281 | 1,178 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/🔺️diff/🧵️wire/🦀️.rs` |
| 282 | 1,178 | rust | 28 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🦀️.rs` |
| 283 | 1,177 | rust | 49 | M | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🦀️.rs` |
| 284 | 1,175 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs` |
| 285 | 1,175 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs` |
| 286 | 1,174 | rust | 49 |  | `🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️unit/🦀️.rs` |
| 287 | 1,170 | ts | 52 |  | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🫳️interaction/🟦️.tsx` |
| 288 | 1,167 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs` |
| 289 | 1,166 | rust | 18 | X | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🦀️.rs` |
| 290 | 1,166 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` |
| 291 | 1,164 | ts | 60 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📐️projection/🟦️.tsx` |
| 292 | 1,163 | rust | 32 | X | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 293 | 1,163 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/🦀️.rs` |
| 294 | 1,156 | rust | 29 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🦀️.rs` |
| 295 | 1,156 | ts | 50 |  | `🧰️framework/🔨️modules/🔄️machine/🟦️.ts` |
| 296 | 1,156 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🧪️tests/🪜️challenge-views/🟦️.tsx` |
| 297 | 1,155 | rust | 18 | X | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 298 | 1,151 | rust | 6 | X | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs` |
| 299 | 1,149 | ts | 55 |  | `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✒️mark/🟦️.ts` |
| 300 | 1,145 | rust | 15 |  | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs` |
| 301 | 1,139 | rust | 42 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs` |
| 302 | 1,138 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` |
| 303 | 1,134 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 304 | 1,134 | rust | 45 | M | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs` |
| 305 | 1,132 | rust | 17 | X | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 306 | 1,132 | ts | 3 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts` |
| 307 | 1,132 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` |
| 308 | 1,132 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🦀️.rs` |
| 309 | 1,132 | rust | 49 | M | `🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs` |
| 310 | 1,131 | ts | 33 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` |
| 311 | 1,129 | rust | 12 | X | `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs` |
| 312 | 1,129 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` |
| 313 | 1,128 | rust | 12 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| 314 | 1,128 | ts | 52 |  | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📽️presentation-core/🟦️.ts` |
| 315 | 1,126 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🦀️.rs` |
| 316 | 1,121 | rust | 28 | X | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 317 | 1,120 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/⚙️codec/🦀️.rs` |
| 318 | 1,120 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs` |
| 319 | 1,119 | ts | 59 |  | `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts` |
| 320 | 1,119 | rust | 37 |  | `🧰️framework/🔨️modules/🌉️abi/🦀️.rs` |
| 321 | 1,117 | rust | 18 | X | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs` |
| 322 | 1,117 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 323 | 1,116 | ts | 56 | M | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/🟦️.ts` |
| 324 | 1,115 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🦀️.rs` |
| 325 | 1,115 | rust | 35 | M | `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs` |
| 326 | 1,114 | ts | 60 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx` |
| 327 | 1,113 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🧪️tests/🧾️learner-lifecycle/🐍️.py` |
| 328 | 1,109 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/🦀️.rs` |
| 329 | 1,109 | rust | 42 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs` |
| 330 | 1,105 | rust | 43 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs` |
| 331 | 1,105 | ts | 46 | MG | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/🟦️.tsx` |
| 332 | 1,103 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx` |
| 333 | 1,100 | rust | 9 | X | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🦀️.rs` |
| 334 | 1,100 | rust | 48 |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🦀️.rs` |
| 335 | 1,099 | rust | 45 |  | `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` |
| 336 | 1,099 | rust | 44 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs` |
| 337 | 1,097 | rust | 43 |  | `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs` |
| 338 | 1,097 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs` |
| 339 | 1,094 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs` |
| 340 | 1,094 | ts | 58 |  | `🧰️framework/🛍️products/🐾️pets/🔨️modules/💗️feeling/🧪️tests/🔬️unit/🟦️.ts` |
| 341 | 1,092 | rust | 49 |  | `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎚️config/🦀️.rs` |
| 342 | 1,091 | rust | 42 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs` |
| 343 | 1,090 | rust | 17 | X | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 344 | 1,090 | rust | 41 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/🦀️.rs` |
| 345 | 1,090 | rust | 37 | M | `🧰️framework/🔨️modules/⏪️time-travel/🦀️.rs` |
| 346 | 1,089 | rust | 25 | XM | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📡️wire/🦀️.rs` |
| 347 | 1,087 | rust | 13 |  | `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs` |
| 348 | 1,084 | rust | 12 | X | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` |
| 349 | 1,084 | rust | 38 |  | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs` |
| 350 | 1,082 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 351 | 1,082 | rust | 38 |  | `🌎️hub/🚀️local-bootstrap/🦀️.rs` |
| 352 | 1,081 | rust | 11 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️sheet-checks/🦀️.rs` |
| 353 | 1,079 | rust | 35 | M | `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs` |
| 354 | 1,076 | rust | 4 | X | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 355 | 1,076 | rust | 23 | X | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` |
| 356 | 1,075 | ts | 57 |  | `🌎️hub/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts` |
| 357 | 1,074 | rust | 37 | M | `🧰️framework/🔨️modules/🎒️pack/📐️format/🎮️retained/🦀️.rs` |
| 358 | 1,073 | rust | 17 | X | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 359 | 1,073 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` |
| 360 | 1,073 | rust | 37 | M | `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` |
| 361 | 1,072 | ts | 14 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🪨️fabric-and-gains/🐍️.py` |
| 362 | 1,071 | rust | 41 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🔺️diff/🦀️.rs` |
| 363 | 1,067 | rust | 20 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🔺️surface/🦀️.rs` |
| 364 | 1,067 | ts | 14 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🧊️mutate-semio-brep/🐍️.py` |
| 365 | 1,063 | rust | 23 | X | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs` |
| 366 | 1,062 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts` |
| 367 | 1,062 | rust | 24 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs` |
| 368 | 1,061 | rust | 42 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` |
| 369 | 1,056 | rust | 16 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` |
| 370 | 1,056 | rust | 45 | M | `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` |
| 371 | 1,052 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs` |
| 372 | 1,052 | ts | 53 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` |
| 373 | 1,051 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs` |
| 374 | 1,051 | rust | 35 | M | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` |
| 375 | 1,049 | rust | 22 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-kernel/🦀️.rs` |
| 376 | 1,048 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts` |
| 377 | 1,047 | rust | 40 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs` |
| 378 | 1,045 | rust | 25 | X | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs` |
| 379 | 1,041 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs` |
| 380 | 1,038 | rust | 19 | X | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` |
| 381 | 1,037 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs` |
| 382 | 1,034 | rust | 25 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️.rs` |
| 383 | 1,034 | rust | 2 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs` |
| 384 | 1,033 | ts | 55 |  | `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` |
| 385 | 1,032 | rust | 17 | X | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` |
| 386 | 1,031 | rust | 30 | X | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs` |
| 387 | 1,031 | rust | 41 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🦀️.rs` |
| 388 | 1,024 | ts | 51 |  | `🧰️framework/🛍️products/❓️quiz/🧪️tests/🧗️challenge-ladder/🟦️.ts` |
| 389 | 1,023 | rust | 32 | X | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🧱️building-elements/🦀️.rs` |
| 390 | 1,023 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs` |
| 391 | 1,021 | rust | 41 | X | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 392 | 1,019 | ts | 55 |  | `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts` |
| 393 | 1,017 | rust | 18 | X | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs` |
| 394 | 1,016 | ts | 58 |  | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🟦️.ts` |
| 395 | 1,014 | rust | 25 | XM | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs` |
| 396 | 1,014 | rust | 26 | WM | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` |
| 397 | 1,013 | rust | 31 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` |
| 398 | 1,013 | rust | 47 |  | `🧰️framework/🛍️products/💻️os/🖥️host/📡️codec-abi/🦀️.rs` |
| 399 | 1,012 | rust | 10 | XM | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs` |
| 400 | 1,012 | ts | 54 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts` |
| 401 | 1,011 | ts | 14 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🌳️mutate-semio-graph/🐍️.py` |
| 402 | 1,006 | rust | 37 | M | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs` |
| 403 | 1,006 | ts | 27 | M | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx` |
| 404 | 1,004 | ts | 46 | MG | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts` |
| 405 | 1,003 | rust | 8 | X | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs` |
| 406 | 1,003 | ts | 14 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🐍️.py` |
| 407 | 1,001 | rust | 34 | X | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🦀️.rs` |
| 408 | 1,001 | rust | 49 | M | `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs` |

Free total: 408 files, 624,271 lines. Of these, 225 are in plugin trees (flag X, 362,308 lines), 26 are renderer wgpu (flag W), and 147 have their module root named in a finished RESULT (flag M).

## 4. Packages for executors

66 packages, each in one area and one language, in path order so that a package holds neighbouring files of one module. Packages are numbered by total lines, largest first. Areas are the coarse domains: `plugin-family`, `os/🔌️plugin`, `os/♾️infinite`, `os/📺️renderer`, `os/other`, `framework/modules`, `framework/tests`, `product/<name>`, `hub`, `apps/<name>`, and the top-level folder otherwise.

| No | Lang | Area | Files | Lines | Flags |
| ---: | --- | --- | ---: | ---: | --- |
| 1 | rust | os/♾️infinite | 9 | 24,174 | M 9-file package |
| 2 | rust | os/🔌️plugin | 7 | 21,724 | MX  |
| 3 | ts | plugin-family | 8 | 15,054 | X  |
| 4 | rust | plugin-family | 8 | 14,400 | MX  |
| 5 | rust | plugin-family | 7 | 14,242 | X  |
| 6 | rust | plugin-family | 7 | 14,076 | X  |
| 7 | rust | plugin-family | 8 | 13,924 | X  |
| 8 | rust | plugin-family | 8 | 13,480 | X  |
| 9 | rust | plugin-family | 8 | 12,955 | X  |
| 10 | rust | os/🔌️plugin | 7 | 12,930 | MX  |
| 11 | rust | plugin-family | 8 | 12,837 | X  |
| 12 | rust | plugin-family | 8 | 12,823 | X  |
| 13 | rust | apps/🏗️fem | 8 | 12,711 |   |
| 14 | ts | plugin-family | 8 | 12,016 | GX  |
| 15 | rust | hub | 7 | 11,762 | M  |
| 16 | rust | plugin-family | 8 | 11,694 | X  |
| 17 | rust | plugin-family | 8 | 11,517 | X  |
| 18 | rust | plugin-family | 8 | 11,482 | X  |
| 19 | rust | plugin-family | 8 | 11,447 | X  |
| 20 | rust | plugin-family | 8 | 11,384 | X  |
| 21 | ts | plugin-family | 8 | 11,074 | X  |
| 22 | rust | plugin-family | 7 | 11,063 | X  |
| 23 | rust | plugin-family | 8 | 10,983 | X  |
| 24 | rust | os/🔌️plugin | 7 | 10,790 | MX  |
| 25 | rust | plugin-family | 8 | 10,758 | MX  |
| 26 | rust | os/📺️renderer | 7 | 10,751 | MW  |
| 27 | ts | os/📺️renderer | 7 | 10,717 | M  |
| 28 | rust | plugin-family | 7 | 10,708 | X  |
| 29 | rust | plugin-family | 7 | 10,615 | X  |
| 30 | rust | plugin-family | 8 | 10,589 | X  |
| 31 | rust | os/🔌️plugin | 6 | 10,495 | MX  |
| 32 | rust | plugin-family | 8 | 10,278 | X  |
| 33 | ts | os/📺️renderer | 7 | 10,174 | MW  |
| 34 | rust | plugin-family | 7 | 10,172 | X  |
| 35 | rust | framework/modules | 8 | 9,835 | M  |
| 36 | rust | os/📺️renderer | 6 | 9,768 | MW  |
| 37 | rust | framework/modules | 8 | 9,713 | M  |
| 38 | rust | hub | 7 | 9,669 |   |
| 39 | rust | os/📺️renderer | 6 | 9,657 | M  |
| 40 | rust | os/📺️renderer | 7 | 9,434 | MW  |
| 41 | rust | plugin-family | 7 | 9,332 | X  |
| 42 | rust | os/📺️renderer | 7 | 8,976 | MW  |
| 43 | rust | os/other | 7 | 8,883 | M  |
| 44 | rust | os/other | 7 | 8,790 | M  |
| 45 | rust | framework/modules | 7 | 8,647 | M  |
| 46 | ts | os/📺️renderer | 6 | 8,587 | GM  |
| 47 | rust | os/other | 7 | 8,501 | X  |
| 48 | rust | product/🦑️repo | 6 | 8,404 | M  |
| 49 | rust | framework/modules | 7 | 8,160 | M  |
| 50 | ts | framework/modules | 5 | 7,275 | GM  |
| 51 | ts | product/❓️quiz | 6 | 6,861 |   |
| 52 | ts | product/🎤️presentation | 5 | 6,477 |   |
| 53 | ts | os/🔌️plugin | 4 | 6,134 | MX U (undersized) |
| 54 | ts | os/other | 4 | 5,385 | M U (undersized) |
| 55 | ts | product/📓️print | 4 | 4,523 |  U (undersized) |
| 56 | ts | product/🦑️repo | 3 | 4,283 | M U (undersized) |
| 57 | ts | hub | 3 | 4,130 | W U (undersized) |
| 58 | ts | product/🐾️pets | 3 | 3,524 |  U (undersized) |
| 59 | ts | 🎓️teaching | 2 | 2,397 |  U (undersized) |
| 60 | ts | os/♾️infinite | 2 | 2,278 | M U (undersized) |
| 61 | ts | apps/📐️cad | 1 | 2,211 |  U (undersized) |
| 62 | rust | product/🖥️server | 1 | 1,512 |  U (undersized) |
| 63 | ts | framework/tests | 1 | 1,396 |  U (undersized) |
| 64 | rust | apps/🔱️trinity | 1 | 1,276 |  U (undersized) |
| 65 | rust | product/🐾️pets | 1 | 1,228 |  U (undersized) |
| 66 | ts | product/🖥️server | 1 | 1,226 |  U (undersized) |

### Package 1: os/♾️infinite (rust, 9 files, 24,174 lines)

- 2,525 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs`
- 2,003 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs`
- 2,950 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🦀️.rs`
- 3,026 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔭️unit-2/🦀️.rs`
- 2,967 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs`
- 2,018 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs`
- 2,407 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs`
- 2,931 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs`
- 3,347 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs`

### Package 2: os/🔌️plugin (rust, 7 files, 21,724 lines)

- 1,899 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs`
- 1,202 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs`
- 3,172 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs`
- 1,034 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs`
- 1,565 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
- 10,471 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- 2,381 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`

### Package 3: plugin-family (ts, 8 files, 15,054 lines)

- 1,310 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏗️export-bim-1-ifc/🐍️.py`
- 1,600 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`
- 3,010 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- 2,447 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- 1,898 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts`
- 1,851 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx`
- 1,806 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🟦️.tsx`
- 1,132 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts`

### Package 4: plugin-family (rust, 8 files, 14,400 lines)

- 2,195 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,076 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- 1,245 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- 1,616 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🏟️arena/🦀️.rs`
- 1,750 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🔏️digest/🦀️.rs`
- 2,163 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs`
- 2,399 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs`
- 1,956 lines: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### Package 5: plugin-family (rust, 7 files, 14,242 lines)

- 1,369 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs`
- 3,416 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`
- 1,745 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`
- 1,292 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs`
- 1,939 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 2,248 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🦀️.rs`
- 2,233 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🦀️.rs`

### Package 6: plugin-family (rust, 7 files, 14,076 lines)

- 1,932 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👆️gestures/🦀️.rs`
- 1,455 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs`
- 2,797 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,995 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/🦀️.rs`
- 2,505 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs`
- 2,241 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs`
- 1,151 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs`

### Package 7: plugin-family (rust, 8 files, 13,924 lines)

- 1,294 lines: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 2,026 lines: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,440 lines: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,428 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs`
- 1,897 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/🦀️.rs`
- 2,524 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`
- 1,316 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs`
- 1,999 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### Package 8: plugin-family (rust, 8 files, 13,480 lines)

- 1,301 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs`
- 1,835 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs`
- 2,725 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/🦀️.rs`
- 1,003 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs`
- 2,291 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs`
- 1,477 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs`
- 1,052 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs`
- 1,796 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs`

### Package 9: plugin-family (rust, 8 files, 12,955 lines)

- 2,037 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- 1,652 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🦀️.rs`
- 1,429 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/🦀️.rs`
- 1,540 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🦀️.rs`
- 1,799 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗂️catalog/🦀️.rs`
- 1,451 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/🦀️.rs`
- 1,100 lines: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🦀️.rs`
- 1,947 lines: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### Package 10: os/🔌️plugin (rust, 7 files, 12,930 lines)

- 1,432 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs`
- 1,843 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🦀️.rs`
- 2,392 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🦀️.rs`
- 1,012 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs`
- 1,924 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/🦀️.rs`
- 2,667 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`
- 1,660 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs`

### Package 11: plugin-family (rust, 8 files, 12,837 lines)

- 1,401 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`
- 2,311 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`
- 1,132 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`
- 1,256 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`
- 1,989 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`
- 1,270 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🦀️.rs`
- 1,081 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️sheet-checks/🦀️.rs`
- 2,397 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🦀️.rs`

### Package 12: plugin-family (rust, 8 files, 12,823 lines)

- 1,497 lines: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- 1,389 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧲️picking/🦀️.rs`
- 1,710 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 2,526 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,084 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- 1,129 lines: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs`
- 2,360 lines: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs`
- 1,128 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`

### Package 13: apps/🏗️fem (rust, 8 files, 12,711 lines)

- 1,087 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs`
- 2,340 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs`
- 1,126 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🦀️.rs`
- 1,132 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🦀️.rs`
- 1,311 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs`
- 2,574 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs`
- 2,047 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🦀️.rs`
- 1,094 lines: `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs`

### Package 14: plugin-family (ts, 8 files, 12,016 lines)

- 1,072 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🪨️fabric-and-gains/🐍️.py`
- 2,548 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`
- 2,730 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts`
- 1,404 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts`
- 1,003 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🐍️.py`
- 1,011 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🌳️mutate-semio-graph/🐍️.py`
- 1,181 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🐍️.py`
- 1,067 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🧊️mutate-semio-brep/🐍️.py`

### Package 15: hub (rust, 7 files, 11,762 lines)

- 1,611 lines: `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`
- 1,397 lines: `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs`
- 1,779 lines: `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs`
- 1,476 lines: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs`
- 1,585 lines: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`
- 2,769 lines: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`
- 1,145 lines: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs`

### Package 16: plugin-family (rust, 8 files, 11,694 lines)

- 1,289 lines: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs`
- 2,422 lines: `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/🦀️.rs`
- 1,268 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🦀️.rs`
- 1,301 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs`
- 1,906 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- 1,289 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles/🦀️.rs`
- 1,163 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/🦀️.rs`
- 1,056 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs`

### Package 17: plugin-family (rust, 8 files, 11,517 lines)

- 1,952 lines: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs`
- 1,132 lines: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,073 lines: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,032 lines: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs`
- 2,447 lines: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,090 lines: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,278 lines: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔣️value/🦀️.rs`
- 1,513 lines: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`

### Package 18: plugin-family (rust, 8 files, 11,482 lines)

- 1,877 lines: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/✉️envelope/🦀️.rs`
- 1,487 lines: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,166 lines: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🦀️.rs`
- 1,017 lines: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`
- 1,155 lines: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 2,424 lines: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️.rs`
- 1,117 lines: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs`
- 1,239 lines: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs`

### Package 19: plugin-family (rust, 8 files, 11,447 lines)

- 1,038 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- 1,082 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- 2,083 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs`
- 1,595 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs`
- 1,745 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️timber/🦀️.rs`
- 1,166 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`
- 1,469 lines: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- 1,269 lines: `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`

### Package 20: plugin-family (rust, 8 files, 11,384 lines)

- 1,684 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️.rs`
- 1,115 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🦀️.rs`
- 1,067 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🔺️surface/🦀️.rs`
- 1,405 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️.rs`
- 1,851 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🩹️repair/🦀️.rs`
- 1,369 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,613 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🦀️.rs`
- 1,280 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️.rs`

### Package 21: plugin-family (ts, 8 files, 11,074 lines)

- 2,326 lines: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts`
- 1,332 lines: `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`
- 1,184 lines: `✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🟦️.ts`
- 1,261 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📸️mutate-remodeling-1/🐍️.py`
- 1,188 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/💨️airflow-and-controls/🐍️.py`
- 1,215 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/📆️schedules-and-layers/🐍️.py`
- 1,300 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔌️power-and-plant/🐍️.py`
- 1,268 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔧️zone-systems/🐍️.py`

### Package 22: plugin-family (rust, 7 files, 11,063 lines)

- 1,496 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🦀️.rs`
- 2,511 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🦀️.rs`
- 1,178 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/🔺️diff/🧵️wire/🦀️.rs`
- 1,323 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/🔺️diff/🧵️wire/🦀️.rs`
- 1,049 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-kernel/🦀️.rs`
- 2,389 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🦀️.rs`
- 1,117 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`

### Package 23: plugin-family (rust, 8 files, 10,983 lines)

- 1,332 lines: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,076 lines: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- 1,274 lines: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- 1,276 lines: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- 1,598 lines: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`
- 1,332 lines: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs`
- 2,032 lines: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`
- 1,063 lines: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs`

### Package 24: os/🔌️plugin (rust, 7 files, 10,790 lines)

- 2,321 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
- 1,361 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`
- 1,572 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🦀️.rs`
- 1,062 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs`
- 1,214 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🦀️.rs`
- 1,167 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs`
- 2,093 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs`

### Package 25: plugin-family (rust, 8 files, 10,758 lines)

- 1,034 lines: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️.rs`
- 2,162 lines: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs`
- 1,322 lines: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs`
- 1,014 lines: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs`
- 1,089 lines: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📡️wire/🦀️.rs`
- 1,574 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,518 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,045 lines: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs`

### Package 26: os/📺️renderer (rust, 7 files, 10,751 lines)

- 1,175 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs`
- 1,953 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs`
- 1,576 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs`
- 1,213 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs`
- 2,418 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs`
- 1,014 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`
- 1,402 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs`

### Package 27: os/📺️renderer (ts, 7 files, 10,717 lines)

- 2,849 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- 1,548 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧱️instance-layers/🟦️.tsx`
- 1,336 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx`
- 1,566 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx`
- 1,006 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx`
- 1,062 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts`
- 1,350 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx`

### Package 28: plugin-family (rust, 7 files, 10,708 lines)

- 2,241 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- 1,178 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🦀️.rs`
- 1,493 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🦀️.rs`
- 1,536 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗂️document/🦀️.rs`
- 1,891 lines: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- 1,121 lines: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,248 lines: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### Package 29: plugin-family (rust, 7 files, 10,615 lines)

- 1,501 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/🦀️.rs`
- 1,435 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🦀️.rs`
- 1,156 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🦀️.rs`
- 1,400 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔎️graph-projection/🦀️.rs`
- 2,286 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`
- 1,410 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs`
- 1,427 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs`

### Package 30: plugin-family (rust, 8 files, 10,589 lines)

- 1,134 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,226 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,031 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs`
- 1,051 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs`
- 1,449 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🦀️.rs`
- 1,037 lines: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`
- 1,782 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`
- 1,879 lines: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### Package 31: os/🔌️plugin (rust, 6 files, 10,495 lines)

- 2,521 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`
- 3,393 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`
- 1,448 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`
- 1,023 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs`
- 1,097 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs`
- 1,013 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`

### Package 32: plugin-family (rust, 8 files, 10,278 lines)

- 1,163 lines: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,478 lines: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs`
- 1,569 lines: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs`
- 1,239 lines: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs`
- 1,305 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
- 1,264 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,237 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs`
- 1,023 lines: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🧱️building-elements/🦀️.rs`

### Package 33: os/📺️renderer (ts, 7 files, 10,174 lines)

- 1,495 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
- 1,131 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- 2,092 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- 1,315 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`
- 1,251 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`
- 1,652 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx`
- 1,238 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🎨️mesh-render/🟦️.tsx`

### Package 34: plugin-family (rust, 7 files, 10,172 lines)

- 1,459 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- 1,555 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs`
- 1,331 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`
- 1,073 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs`
- 1,001 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff/🦀️.rs`
- 2,192 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🦀️.rs`
- 1,561 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🦀️.rs`

### Package 35: framework/modules (rust, 8 files, 9,835 lines)

- 1,376 lines: `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`
- 1,079 lines: `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs`
- 1,313 lines: `🧰️framework/🔨️modules/🎓️onboarding/🦀️.rs`
- 1,248 lines: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`
- 1,115 lines: `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs`
- 1,314 lines: `🧰️framework/🔨️modules/📡️replication/📡️wire/👥️presence/🦀️.rs`
- 1,051 lines: `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
- 1,339 lines: `🧰️framework/🔨️modules/🔄️machine/✨️derive/⚙️expansion/🦀️.rs`

### Package 36: os/📺️renderer (rust, 6 files, 9,768 lines)

- 1,215 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- 1,395 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs`
- 1,204 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs`
- 2,593 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- 2,155 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`
- 1,206 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`

### Package 37: framework/modules (rust, 8 files, 9,713 lines)

- 1,090 lines: `🧰️framework/🔨️modules/⏪️time-travel/🦀️.rs`
- 1,073 lines: `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`
- 1,583 lines: `🧰️framework/🔨️modules/✍️editor/🦀️.rs`
- 1,119 lines: `🧰️framework/🔨️modules/🌉️abi/🦀️.rs`
- 1,373 lines: `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`
- 1,006 lines: `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs`
- 1,395 lines: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🦀️.rs`
- 1,074 lines: `🧰️framework/🔨️modules/🎒️pack/📐️format/🎮️retained/🦀️.rs`

### Package 38: hub (rust, 7 files, 9,669 lines)

- 1,645 lines: `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs`
- 1,082 lines: `🌎️hub/🚀️local-bootstrap/🦀️.rs`
- 1,084 lines: `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs`
- 1,369 lines: `🌎️hub/🧪️tests/🔬️bin-unit/🎲️hostile-input/🔬️unit/🦀️.rs`
- 1,413 lines: `🌎️hub/🧪️tests/🔬️bin-unit/🔐️session/🔬️unit/🦀️.rs`
- 1,841 lines: `🌎️hub/🧪️tests/🔬️bin-unit/🧭️open-plan/🔬️unit/🦀️.rs`
- 1,235 lines: `🌎️hub/🧪️tests/🔬️bin-unit/🧰️testkit/🦀️.rs`

### Package 39: os/📺️renderer (rust, 6 files, 9,657 lines)

- 1,180 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`
- 1,411 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs`
- 1,262 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`
- 1,274 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
- 2,498 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- 2,032 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`

### Package 40: os/📺️renderer (rust, 7 files, 9,434 lines)

- 2,047 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs`
- 1,047 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs`
- 1,494 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs`
- 1,357 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs`
- 1,175 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs`
- 1,194 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs`
- 1,120 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs`

### Package 41: plugin-family (rust, 7 files, 9,332 lines)

- 1,733 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧭️causal-internals/🦀️.rs`
- 1,090 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/🦀️.rs`
- 1,031 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🦀️.rs`
- 1,071 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🔺️diff/🦀️.rs`
- 1,857 lines: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🌫️fuzzy-internals/🦀️.rs`
- 1,021 lines: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- 1,529 lines: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs`

### Package 42: os/📺️renderer (rust, 7 files, 8,976 lines)

- 1,139 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs`
- 1,307 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs`
- 1,420 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs`
- 1,109 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs`
- 1,091 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`
- 1,061 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`
- 1,849 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`

### Package 43: os/other (rust, 7 files, 8,883 lines)

- 1,097 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs`
- 1,859 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- 1,120 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/⚙️codec/🦀️.rs`
- 1,105 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`
- 1,205 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🗜️codec/🦀️.rs`
- 1,109 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧩️command-decode/🦀️.rs`
- 1,388 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`

### Package 44: os/other (rust, 7 files, 8,790 lines)

- 1,217 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🦀️.rs`
- 1,206 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🦀️.rs`
- 1,099 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`
- 1,318 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs`
- 1,430 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs`
- 1,288 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs`
- 1,232 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🪢️session/🦀️.rs`

### Package 45: framework/modules (rust, 7 files, 8,647 lines)

- 1,417 lines: `🧰️framework/🔨️modules/🔲️pixels/🦀️.rs`
- 1,056 lines: `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`
- 1,237 lines: `🧰️framework/🔨️modules/🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs`
- 1,099 lines: `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs`
- 1,421 lines: `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs`
- 1,134 lines: `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs`
- 1,283 lines: `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs`

### Package 46: os/📺️renderer (ts, 6 files, 8,587 lines)

- 1,782 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx`
- 1,814 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`
- 1,586 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx`
- 1,004 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts`
- 1,105 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/🟦️.tsx`
- 1,296 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx`

### Package 47: os/other (rust, 7 files, 8,501 lines)

- 1,186 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🕸️graph/🦀️.rs`
- 1,662 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🦀️.rs`
- 1,129 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`
- 1,138 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs`
- 1,013 lines: `🧰️framework/🛍️products/💻️os/🖥️host/📡️codec-abi/🦀️.rs`
- 1,332 lines: `🧰️framework/🛍️products/💻️os/🖥️host/🔌️plugin-host/🦀️.rs`
- 1,041 lines: `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs`

### Package 48: product/🦑️repo (rust, 6 files, 8,404 lines)

- 1,385 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs`
- 1,195 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust/🦀️.rs`
- 1,264 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
- 1,100 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🦀️.rs`
- 2,227 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/🦀️.rs`
- 1,233 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust/🦀️.rs`

### Package 49: framework/modules (rust, 7 files, 8,160 lines)

- 1,001 lines: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs`
- 1,132 lines: `🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs`
- 1,177 lines: `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🦀️.rs`
- 1,207 lines: `🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/🦀️.rs`
- 1,174 lines: `🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️unit/🦀️.rs`
- 1,092 lines: `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎚️config/🦀️.rs`
- 1,377 lines: `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🔤️vocabulary/🦀️.rs`

### Package 50: framework/modules (ts, 5 files, 7,275 lines)

- 1,276 lines: `🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts`
- 1,321 lines: `🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- 1,156 lines: `🧰️framework/🔨️modules/🔄️machine/🟦️.ts`
- 1,631 lines: `🧰️framework/🔨️modules/🛰️backbone/🟦️.ts`
- 1,891 lines: `🧰️framework/🔨️modules/🧾️app-channel/🟦️.ts`

### Package 51: product/❓️quiz (ts, 6 files, 6,861 lines)

- 1,417 lines: `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css`
- 1,048 lines: `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts`
- 1,103 lines: `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx`
- 1,024 lines: `🧰️framework/🛍️products/❓️quiz/🧪️tests/🧗️challenge-ladder/🟦️.ts`
- 1,113 lines: `🧰️framework/🛍️products/❓️quiz/🧪️tests/🧾️learner-lifecycle/🐍️.py`
- 1,156 lines: `🧰️framework/🛍️products/❓️quiz/🧪️tests/🪜️challenge-views/🟦️.tsx`

### Package 52: product/🎤️presentation (ts, 5 files, 6,477 lines)

- 1,170 lines: `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🫳️interaction/🟦️.tsx`
- 1,406 lines: `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css`
- 1,128 lines: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📽️presentation-core/🟦️.ts`
- 1,443 lines: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖱️interaction-dom/🟦️.tsx`
- 1,330 lines: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖼️deck-rendering/🟦️.tsx`

### Package 53: os/🔌️plugin (ts, 4 files, 6,134 lines)

- 1,562 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`
- 1,052 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`
- 1,858 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts`
- 1,662 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`

### Package 54: os/other (ts, 4 files, 5,385 lines)

- 1,446 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
- 1,012 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts`
- 1,744 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- 1,183 lines: `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts`

### Package 55: product/📓️print (ts, 4 files, 4,523 lines)

- 1,033 lines: `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts`
- 1,322 lines: `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts`
- 1,149 lines: `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/✒️mark/🟦️.ts`
- 1,019 lines: `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts`

### Package 56: product/🦑️repo (ts, 3 files, 4,283 lines)

- 1,969 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts`
- 1,198 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`
- 1,116 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/🟦️.ts`

### Package 57: hub (ts, 3 files, 4,130 lines)

- 1,075 lines: `🌎️hub/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts`
- 1,432 lines: `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration/🟦️.ts`
- 1,623 lines: `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`

### Package 58: product/🐾️pets (ts, 3 files, 3,524 lines)

- 1,414 lines: `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🟦️.tsx`
- 1,094 lines: `🧰️framework/🛍️products/🐾️pets/🔨️modules/💗️feeling/🧪️tests/🔬️unit/🟦️.ts`
- 1,016 lines: `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🟦️.ts`

### Package 59: 🎓️teaching (ts, 2 files, 2,397 lines)

- 1,278 lines: `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts`
- 1,119 lines: `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts`

### Package 60: os/♾️infinite (ts, 2 files, 2,278 lines)

- 1,164 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📐️projection/🟦️.tsx`
- 1,114 lines: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx`

### Package 61: apps/📐️cad (ts, 1 files, 2,211 lines)

- 2,211 lines: `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts`

### Package 62: product/🖥️server (rust, 1 files, 1,512 lines)

- 1,512 lines: `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs`

### Package 63: framework/tests (ts, 1 files, 1,396 lines)

- 1,396 lines: `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts`

### Package 64: apps/🔱️trinity (rust, 1 files, 1,276 lines)

- 1,276 lines: `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/✏️editor/🦀️.rs`

### Package 65: product/🐾️pets (rust, 1 files, 1,228 lines)

- 1,228 lines: `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🦀️.rs`

### Package 66: product/🖥️server (ts, 1 files, 1,226 lines)

- 1,226 lines: `🧰️framework/🛍️products/🖥️server/🟦️.ts`

## 5. Top six packages, by total lines

- Package 1: os/♾️infinite (rust), 9 files, 24,174 lines.
- Package 2: os/🔌️plugin (rust), 7 files, 21,724 lines.
- Package 3: plugin-family (ts), 8 files, 15,054 lines.
- Package 4: plugin-family (rust), 8 files, 14,400 lines.
- Package 5: plugin-family (rust), 7 files, 14,242 lines.
- Package 6: plugin-family (rust), 7 files, 14,076 lines.

## 6. Data and generated check

- No free file over 1,000 lines is data. The largest literal share (lines that are only a string, number or key-value literal) is 0.335, and no file has a generator header. Table-like content is not the problem here.
- 5 free files carry the generated-parser signature (`validateAgainst` or `(value: unknown)` validators). They are 7,791 lines. The largest is the stdio `🔺️diff` parser (2,730 lines). Check first whether the parser generator (`schema parsers`) owns it; if so, change the generator, not the file. The other four need the same check before any edit: `🛰️backbone`, `📇️directory/🧬️schema`, and two ShellHelpers files, which may be hand-written.
- 71 free files use `include_bytes!` or `include_str!`. Their data lives outside the file, so the line counts are code.

## 7. Caveats and decisions for the coordinator

1. Plugin trees. A free file is one that no running slice claims. Plugin files pass that test, but the only plan that names them, PLAN-PLUGIN-CRATES, is not running. The free plugin files are 193 in `✏️s/🔌️plugins` (298,903 lines) and 31 in `os/🔌️plugin` (62,073 lines). They are the largest share of the list. Confirm with the plugin owner before an executor starts on them, since FLEET-CLAIMS lists "plugin crates (79 oversized)" as claimed.
2. Running claims over 1,000 lines. `pi2` claims 9 files over 1,000 lines by exact list, and `e51` claims 7. Those 16 files are in section 2. The directory claims of `pi2` (21 crate directories), `pgr` (75 package directories), `rsl` (167 crate directories) and `ldd` (22) cover no file over 1,000 lines.
3. `e51` holds 33 exact files, the E0451 sites before its sweep (`🗑️generated/e51-before-e0451.json`). The sweep runs over all families, so a free Rust file can gain an E0451 edit that is not in that list. Refresh the site list before dispatching a Rust package.
4. `ecp` has no trace here. It has no tools folder, no FLEET or HANDOFFS entry, and no generated log. I could not place it, so it claims no file in this census. Treat its scope as unknown.
5. `tfe` has no file list. Its work set comes from `gen.ts` at run time, and `orig/` holds 215 entry files that re-export fixtures, not the consumers it re-points. FLEET says its RESULT was never written (F-09). Its consumers are TS test files that may be in the free list.
6. Renderer wgpu. 26 free files (37,546 lines) sit under `os/📺️renderer/…/🧊️wgpu`. They are not under the `🖱️ui` path that the peer rule names, but HANDOFFS G-11 gives wgpu renderer artifacts to the peer. Flag `W` keeps them free for now. Confirm with the UI peer.
7. Module root named in a RESULT (flag `M`): 147 free files (234,905 lines). The RESULT named the module, not the file. Read that RESULT before a split.
8. "Free" means no claim was found by path, crate, package directory or named document. It is not verified per file body, which is the same caveat census 3 gives.
9. Undersized packages: 14 packages have fewer than five files because their area has fewer than five free files. Package 1 (`os/♾️infinite`, Rust) has nine files, because splitting it 5 and 4 would leave a four-file package. The coordinator can merge small packages within the same product.
10. Parent and peer files are excluded from the free list and are not reviewed here. No file over 1,000 lines is named in HANDOFFS groups 1 to 4, so the HANDED class is empty.
