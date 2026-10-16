# 🪓️ Monolith Census 2

- Date: 2026-10-16. Auditor: Haiku 5.5 (read-only). Folder: `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
- Scope: tracked `*.rs`, `*.ts`, `*.tsx`, `*.py`, `*.wit`, `*.css` from `git ls-files`. Excluded: generated folders, `.🧬semio`, `node_modules`, fixtures, corpora, snapshots, `.d.ts`, minified files, and any path that says generated.
- Totals: 41,355 files kept, 4,702,177 lines. 334 files are over 1,500 lines, holding 912,154 lines (19.4% of all kept lines).
- Raw outputs: `🗑️generated/mc2-*` (file list, wc output, parsed rows, seam dump, scripts).
- Method caveats: (1) `wc` under the default C locale printed emoji in paths as `?`, so the counts were re-run under `LC_ALL=en_US.UTF-8`. (2) "Items" is a cheap regex count, not a parse: Rust `pub`/`fn`/`impl`/`struct`/`enum`/`mod`/`trait` at line start, TS lines starting with `export `, Python `def`/`class`. TS files written with `describe`/`it` report 0. (3) Owner and status come from path rules checked against FLEET.md, FLEET-CLAIMS.md and the RESULT/PLAN files. They are not verified per file body. "free" means no fleet or peer claim was found, not that the file is safe to edit.

## 1. Top 80 files over 1,500 lines

| # | Path | Lang | Lines | Items | Owning area | Status |
| --- | --- | --- | ---: | ---: | --- | --- |
| 1 | `📜️script.ts` | TypeScript | 25,739 | 111 | repo tooling (root script) | free |
| 2 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | TypeScript | 17,627 | 10 | hub | fleet-claimed (hub) |
| 3 | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🧰️zukunft-bau-entwerfen-mit-bestand_files/📐️layout.css` | CSS | 16,454 | - | vendored presentation project | free (vendored presentation asset; not a split target) |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | TypeScript | 14,454 | 0 | framework/os | free (renderer TS, not in FLEET-CLAIMS) |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` | Rust | 12,634 | 1076 | framework/os | peer-owned (store engine) |
| 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | Rust | 12,221 | 1055 | framework/os | peer-owned (store engine) |
| 7 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | TSX | 11,948 | 2 | framework/ui | peer-owned (ui aggregate / ui-contract; wgpu under ui = UI GPU cut) |
| 8 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | TypeScript | 11,928 | 384 | framework/repo | held (repo-lib; parent overlap) |
| 9 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | TypeScript | 10,787 | 71 | framework/repo | held (repo-lib; parent overlap) |
| 10 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | Rust | 10,650 | 569 | framework/os | fleet-claimed (plugin crates) |
| 11 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py` | Python | 7,883 | 589 | plugin:🔋️energy | fleet-claimed (plugin crates) |
| 12 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts` | TypeScript | 7,656 | 337 | framework/repo | free |
| 13 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` | CSS | 7,555 | - | framework/ui | peer-owned (ui aggregate / ui-contract; wgpu under ui = UI GPU cut) |
| 14 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | TypeScript | 7,539 | 0 | framework/repo | held (repo-lib; parent overlap) |
| 15 | `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts` | TypeScript | 7,388 | 1 | apps/dev (✏️s non-plugin) | free |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | TypeScript | 7,132 | 25 | framework/os | peer-owned (store engine) |
| 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` | TypeScript | 6,765 | 1 | framework/os | free |
| 18 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | TSX | 6,568 | 365 | framework/ui | peer-owned (ui aggregate / ui-contract; wgpu under ui = UI GPU cut) |
| 19 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` | Rust | 5,953 | 464 | framework/os | peer-owned (store engine) |
| 20 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🦀️.rs` | Rust | 5,872 | 15 | plugin:🔋️energy | fleet-claimed (plugin crates) |
| 21 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` | TypeScript | 5,177 | 270 | framework/repo | held (repo-lib; parent overlap) |
| 22 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | TSX | 4,617 | 97 | framework/ui | peer-owned (ui aggregate / ui-contract; wgpu under ui = UI GPU cut) |
| 23 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` | Rust | 4,101 | 237 | framework/ui | contested (renderer-wgpu plan covers this directory; FLEET-CLAIMS leaves the UI GPU cut to the peer) |
| 24 | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | TypeScript | 4,038 | 238 | framework/modules | free |
| 25 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` | Rust | 3,909 | 154 | framework/ui | contested (renderer-wgpu plan covers this directory; FLEET-CLAIMS leaves the UI GPU cut to the peer) |
| 26 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs` | Rust | 3,819 | 278 | framework/repo | free |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | TSX | 3,783 | 1 | framework/os | free (renderer TS, not in FLEET-CLAIMS) |
| 28 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🦀️.rs` | Rust | 3,751 | 488 | plugin:🏙️bim | fleet-claimed (plugin crates) |
| 29 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` | Rust | 3,691 | 266 | framework/os | peer-owned (store engine) |
| 30 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | TypeScript | 3,640 | 1 | framework/os | fleet-claimed (plugin crates) |
| 31 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` | TypeScript | 3,600 | 176 | framework/repo | free |
| 32 | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs` | Rust | 3,597 | 239 | plugin:🏭️process | fleet-claimed (plugin crates) |
| 33 | `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx` | TSX | 3,567 | 1 | framework/products/🎤️presentation | free (presentation dedupe done) |
| 34 | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts` | TypeScript | 3,552 | 239 | apps/dev (✏️s non-plugin) | free |
| 35 | `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` | Rust | 3,450 | 138 | hub | fleet-claimed (hub) |
| 36 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` | Rust | 3,416 | 230 | plugin:🧩️puzzle | fleet-claimed (plugin crates) |
| 37 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` | Rust | 3,393 | 161 | framework/os | fleet-claimed (plugin crates) |
| 38 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` | Rust | 3,388 | 203 | framework/ui | contested (renderer-wgpu plan covers this directory; FLEET-CLAIMS leaves the UI GPU cut to the peer) |
| 39 | `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | Rust | 3,372 | 145 | apps/dev (✏️s non-plugin) | free |
| 40 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs` | Rust | 3,363 | 407 | framework/os | peer-owned (db; FLEET-CLAIMS also lists os-kernel-db in progress) |
| 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` | Rust | 3,347 | 114 | framework/os | fleet-claimed (os-infinite) |
| 42 | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | Rust | 3,325 | 143 | framework/os | free |
| 43 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` | Rust | 3,319 | 158 | framework/ui | contested (renderer-wgpu plan covers this directory; FLEET-CLAIMS leaves the UI GPU cut to the peer) |
| 44 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/⚙️engine/🦀️.rs` | Rust | 3,307 | 263 | framework/os | fleet-claimed (os-infinite) |
| 45 | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🦀️.rs` | Rust | 3,306 | 212 | plugin:🖨️raster | fleet-claimed (plugin crates) |
| 46 | `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs` | Rust | 3,306 | 297 | framework/products/🖥️server | fleet-touched (server/pets carve; file not split) |
| 47 | `🧰️framework/🔨️modules/🔢️number/🦀️.rs` | Rust | 3,271 | 378 | framework/modules | free |
| 48 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` | TSX | 3,259 | 54 | framework/os | free (renderer TS, not in FLEET-CLAIMS) |
| 49 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` | TSX | 3,213 | 1 | framework/os | free (renderer TS, not in FLEET-CLAIMS) |
| 50 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs` | Rust | 3,172 | 277 | framework/os | fleet-claimed (plugin crates) |
| 51 | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` | Rust | 3,161 | 148 | hub | fleet-claimed (hub) |
| 52 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs` | Rust | 3,144 | 232 | framework/os | fleet-claimed (os-infinite) |
| 53 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | Rust | 3,079 | 177 | framework/os | peer-owned (flow) |
| 54 | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` | TypeScript | 3,075 | 0 | framework/products/🐾️pets | fleet-touched (server/pets carve; file not split) |
| 55 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` | Rust | 3,070 | 176 | plugin:🌀️procedural | fleet-claimed (plugin crates) |
| 56 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` | TypeScript | 3,046 | 4 | framework/os | free |
| 57 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | Rust | 3,031 | 58 | plugin:📕️norm | fleet-claimed (plugin crates) |
| 58 | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs` | Rust | 3,026 | 126 | plugin:🔋️energy | fleet-claimed (plugin crates) |
| 59 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔭️unit-2/🦀️.rs` | Rust | 3,026 | 114 | framework/os | fleet-claimed (os-infinite) |
| 60 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | Rust | 3,017 | 256 | plugin:🌀️procedural | fleet-claimed (plugin crates) |
| 61 | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` | TypeScript | 3,006 | 549 | plugin:🏛️architect | fleet-claimed (plugin crates) |
| 62 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` | Rust | 2,976 | 83 | plugin:📕️norm | fleet-claimed (plugin crates) |
| 63 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs` | Rust | 2,967 | 328 | framework/os | fleet-claimed (os-infinite) |
| 64 | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | Rust | 2,958 | 247 | plugin:🌊️flow | peer-owned (flow) |
| 65 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🦀️.rs` | Rust | 2,950 | 128 | framework/os | fleet-claimed (os-infinite) |
| 66 | `🧰️framework/🔨️modules/⏳️async/🦀️.rs` | Rust | 2,931 | 323 | framework/modules | free |
| 67 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs` | Rust | 2,931 | 157 | framework/os | fleet-claimed (os-infinite) |
| 68 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` | Rust | 2,925 | 134 | plugin:📕️norm | fleet-claimed (plugin crates) |
| 69 | `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` | Rust | 2,923 | 110 | hub | fleet-claimed (hub) |
| 70 | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` | TypeScript | 2,921 | 5 | framework/os | free |
| 71 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | TSX | 2,849 | 13 | framework/os | fleet-claimed (ShellHelpers/World3dHost TSX split; parent remainder) |
| 72 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🦀️.rs` | Rust | 2,836 | 96 | plugin:🏗️fem | fleet-claimed (plugin crates) |
| 73 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | Rust | 2,833 | 127 | framework/os | peer-owned (store engine) |
| 74 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` | Rust | 2,821 | 221 | framework/os | peer-owned (db; FLEET-CLAIMS also lists os-kernel-db in progress) |
| 75 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | Rust | 2,797 | 154 | plugin:🧩️puzzle | fleet-claimed (plugin crates) |
| 76 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🦀️.rs` | Rust | 2,796 | 77 | framework/os | fleet-claimed (os-infinite) |
| 77 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/🦀️.rs` | Rust | 2,788 | 261 | framework/repo | free |
| 78 | `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs` | Rust | 2,779 | 118 | framework/modules | free |
| 79 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` | Rust | 2,769 | 141 | hub | fleet-claimed (hub) |
| 80 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs` | Rust | 2,764 | 253 | framework/os | peer-owned (db; FLEET-CLAIMS also lists os-kernel-db in progress) |

The 80 rows total 410,997 lines.

## 2. Status of the top 80

| Status | Files | Rule |
| --- | ---: | --- |
| fleet-claimed | 33 | Plugin crates (`✏️s/🔌️plugins/*`, `os/🔨️modules/🔌️plugin`), `♾️infinite`, Shell, hub (`🌎️hub`). Matches FLEET-CLAIMS and the PLAN files. |
| fleet-touched | 2 | Server and pets: the carve is recorded, but these files are still large. |
| contested | 4 | `🖱️ui/…/🧊️wgpu` and its tests. The renderer-wgpu plan covers these directories, while FLEET-CLAIMS leaves the UI GPU cut to the peer. |
| held | 4 | `📚️library` (repo-lib). FLEET.md says package 3 overlaps the parent repo-lib work. |
| peer-owned | 15 | `🏪️store`, `🛢️db` (see conflict 2 below), `🌊️flow`, `◻️2d`, and the non-wgpu `🖱️ui` aggregate and ui-contract. |
| free | 22 | No claim found. One is the vendored `layout.css` (not a split target). |

## 3. Seam sketches for the 15 largest free files

The list excludes the vendored CSS asset, so the 15 files are the largest free source files. Folder names are kebab-case. Each child takes an emoji prefix from the taxonomy, and each prefix must be unique among its siblings. Wrong-direction imports are listed where the import headers show them.

### `📜️script.ts`

25,739 lines, TypeScript, items 111, status: free.

The file has five command classes (`NativeOsScript`, `SetupScript`, `DevScript`, `GenerateScript`, `LintScript`, lines 126 to 490). It also has taxonomy-option helpers (369 to 396), plugin-crate and warning-scope helpers (503 to 530), and a tool-job audit block from line 533 to beyond 1,330. That block covers static rows, publication contracts, proofs, factory exactness and the global payload store inventory. Suggested children: `script-commands/`, `taxonomy-options/`, `plugin-crates/` and `tool-job-audit/`. AGENTS.md allows only `📜️script.ts` as a script file, so the children must be plain `🟦️.ts` modules that the script imports, not new scripts. Wrong direction: the header imports at least 19 `🧪️tests` modules from `🏪️store`, `🔌️plugin`, `🧵️job`, `🖱️ui`, `🛠️tool-machine` and `🗄️stdio`. Tooling reads the tests as libraries, so the audit data should move to its owner and the tests should import it from there.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

14,454 lines, TypeScript, items 0, status: free (renderer TS, not in FLEET-CLAIMS).

This is a 14,454-line test file with about 30 top-level helpers and a large `registerTests1` closure. The helper clusters are: contract-tree builders (`ContractNodeSpec`, `buildContractSnapshot`, `renderContractTree`, around 2025 to 2100), canvas and label recording stubs (`recordingCanvasContext`, `recordingLabelContext`, `labelCanvasStub`), mesh and scene fixtures (`meshKindBounds`, `hexagonalMushroomColumnScene`, `portSidesSurfaceRows`), plugin-manifest and descriptor readers (`readPluginManifest`, the `PROCEDURAL_*` paths, `windowKindsDeclaring`), and AJV schema validators (`ownedExports`, `peerExport`, `rendererExport`). Suggested children: `test-contract-tree/`, `test-canvas-recording/`, `test-scene-fixtures/`, `test-descriptor-readers/`, `test-schema-validators/`, each under `🧪️tests/🔬️…`. Wrong direction: an os test reads a hub composition JSON by relative path (`…/🌎️hub/🧩️compositions/🌀️procedural/…`), and it imports `🖱️ui` fixtures and the `🛂️manifest` fixtures. Hub and ui fixtures should be reached through their owners or a shared fixture package.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`

7,656 lines, TypeScript, items 337, status: free.

This 7,656-line repo test harness has several separable parts. The taxonomy and discovery part covers `testTaxonomy`, `discoverTestCases`, `isExcludedTestPath` and the test-location helpers (lines 35 to 340). The oracle part covers `OracleEntry`, the adapter maps and `ADAPTER_*` tables, and the oracle registry (341 to 420 and 807 onward). The mutation-catalog part covers `MutationVector`, `mutationCatalogProblems` and the profile-slug rules (462 to 640). The contributions part covers the contribution schema and reader (657 to 805). Digest helpers sit at 213 to 237. Suggested children: `taxonomy/`, `discovery/`, `oracle-registry/`, `mutation-catalog/`, `contributions/`, `digests/`. The header imports look correct (test to library and process), but the file mixes registry, catalog and discovery concerns.

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts`

7,388 lines, TypeScript, items 1, status: free.

The file lives under the `gis` dev variant, but most of its 142 `describe`/`it` titles (plus `vitest.it` calls, which the regex missed) cover hub and replication behavior. The describe blocks are: space artifact creation owner; `DirectoryEventPageBootstrapV1`; hub session capability lane; document actor recovery; backbone-worker wire bridge; artifact bootstrap atomic restore; identity config facet; config-mutation TypeScript parity; and backbone-worker space administration. The gis-specific part is the inference-port tests (`vitest.it` calls from about line 22 to 70, plus gis inference types from line 419). Suggested split: one test file per describe block under the owning hub, replication or identity area, and a gis-only file that keeps the inference-port tests. Wrong placement: `🌍️gis` variant tests cover `🌎️hub` and replication, and the file imports `🧰️framework` os fetch stubs.

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`

6,765 lines, TypeScript, items 1, status: free.

This 6,765-line file is the headless end-to-end gate for time travel, run against a puzzle2d dev serve. Its parts are: CLI configuration and routing (`configure`, `flagValue`, `routeUrl`, 114 to 183), verdict, note and NDJSON reporting (185 to 232), localized COPY tables for `en` and `de` (234 to 360), console and page-error capture (369 to 413), browser lifecycle (431 onward), and 24 numbered steps. Suggested children: `reporting/`, `locales/`, `browser-capture/`, `boot/`, and `steps/` split into four or five step groups. Wrong direction: the os dev test hard-codes the puzzle2d plugin id, the `framework.panel.history` tab id and the `:6012` serve URL, and it imports `🦑️repo/…/🎯️acceptance` and `🔌️plugin/🏗️build/📋️plan`. A puzzle2d-specific gate belongs with the puzzle plugin, and the os should only expose the generic harness.

### `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`

4,038 lines, TypeScript, items 238, status: free.

This 4,038-line TypeScript kernel is the TS counterpart of the Rust kernel that the os-kernel carve split. The Rust carve did not touch it. Its clusters are: descriptor fetch and text reads (lines 45 to 150, already partly split into `📇️descriptor/` children), turn-outcome broadcast (156 to 226), plugin registry expansion and contributions JSON (227 to 360), contribution metadata types (363 to 440), `AppRouter` (458 to 650), the IO entry graph with `ioRun` and `ioIdentify` (650 to 870), opening preferences (871 to 980), and plugin-catalog targets (981 onward). Suggested children: `router/`, `io-graph/`, `preferences/`, `registry/`, `catalog/`, with the existing descriptor children kept. Wrong direction: the kernel imports `🎭️actor/📮️shard-client` and `🎭️actor/🧵️turn-scheduler`, which makes the framework-tier kernel depend on the actor frontier that is out of scope for this fleet. Flag it for the parent.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs`

3,819 lines, Rust, items 278, status: free.

This 3,819-line crate holds several concerns. The allowed-values part (`include_str!` of `🧬️schema/🔣️allowed-values.json`, plus `allowed_llms`, `allowed_efforts`, `allowed_clients`, lines 95 to 240) has to stay in step with the schema. The slug-normalization part (`slugify`, `normalize_*_slug`, lines 131 to 240) is a small leaf. The kind enums (`DefinitionKind`, `FileKind`, `FolderKind`, `BundleKind`, `TechnologyKind`, 249 to 530) each carry a `Display` impl. The status enums (`TicketStatus`, `GoalStatus`, `BreachPriority`) form another group. The metrics structs (`Range`, `LineMetrics`, `DiffLines`, `CountMetrics`, `PriorityCount`, `AnalyzeMetrics`, 564 to 630) are a fourth. The entity structs (`Repo`, `Technology`, and others from 628 on) are a fifth. Suggested children: `slug/`, `allowed-values/`, `kinds/`, `status/`, `metrics/`, `entities/`. Imports show no wrong direction: std, serde, serde_json and the repo identity crate.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`

3,783 lines, TSX, items 1, status: free (renderer TS, not in FLEET-CLAIMS).

This 3,783-line test file has three top-level helpers and a `registerTests1` closure that holds about 160 test titles. The titles cover: the plugin UI close-ladder budget; isolated job admission batches; the shared context-menu ViewModel; the surface render ViewModel; reserved refresh sections; the extension invocation WIT request identity; icon export wire decoding; host effect address decoding; extension completion publication; the job lifecycle and actor binding; and command-ingress fault diagnostics. Suggested children: `close-ladder/`, `job-admission/`, `view-model/`, `wit-identity/`, `completion-publication/`, `actor-binding/`, each as its own test file. Wrong direction: the os renderer test imports `🎭️actor` retained-turn fixtures and `🎭️actor/📮️shard-client`, and `framework-replication`. Most of the job and actor tests belong with the job and actor owners.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts`

3,600 lines, TypeScript, items 176, status: free.

This 3,600-line VS Code client mixes two things. Lines 58 onward are GraphQL codegen output pasted into the source, and the codegen-style types run at least to line 440: `Maybe`, `Exact`, `Incremental`, `MakeOptional`, the `Bundle`, `Checkpoint`, `Contributor`, `Definition`, `File`, `Folder` and `Mutation*Args` types. Under schema-first, these belong in `🤖️generated`, produced from `🧬️schema`. The rest of the file holds the command, deserialize and validate helpers (`execAsync`, `deserializeKit`, `validateKit`, lines 25 to 40) and the RepoEvent type, plus the client code that uses `vscode`, `child_process` and `fs`. Suggested children: `generated-types/` (moved to `🤖️generated`), `commands/`, `kit/`, `events/`. Wrong direction: none in the imports, but the handwritten copy of the schema types is a drift risk.

### `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx`

3,567 lines, TSX, items 1, status: free (presentation dedupe done).

This 3,567-line test file has two top-level items and about 135 titles. The title clusters are: `resolveRevealArrangement` and the morph companion logic; the `PresentationDeck` rendering and intro morph DOM; reveal auto-animate sheet rewriting and inline layout; split and disposition rendering; figure crop, zoom and scroll math (about 20 `figure*` titles); PDF cover scale and page subsets; asset URL resolution; and morph ghost and FLIP rest state. Suggested children: `arrangement/`, `morph/`, `auto-animate/`, `figure-crop/`, `figure-zoom/`, `pdf/`, `dispositions/`. The file imports `reveal.js` and a presentation target, which is fine for a deck test. There is no wrong-direction import in the header.

### `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts`

3,552 lines, TypeScript, items 239, status: free.

The file is named `geometry`, but its 338 top-level items are mostly an interaction DSL. The clusters are: selection vocabulary (`SelectionTarget`, `SelectionSpec`, `filterSelectionTargets`, `expandSelectionTargetsForAccept`, lines 19 to 120); the path and attribute model (`PathSegment`, `read/write/clearPathSegments`, `AttributeTable`, 122 to 300); spatial entity flags (301 to 328); the expression language (`Expr*` types, `evalExpr`, `evalGuard`, 329 to 540); and interaction state-machine specs (`TransitionSpec`, `EffectSpec`, `InteractionSpec`, `StateDefSpec`, `DisplayItemSpec`, `CommitOperationSpec`, from 538 on). Suggested children: `selection/`, `paths/`, `expr/`, `interaction-spec/`, `spatial-flags/`, with the geometry vocabulary left in the parent. Imports are clean (framework, `framework-3d-js` and a sibling spatial module). The issue is naming drift, not direction.

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

3,372 lines, Rust, items 145, status: free.

This 3,372-line unit test has about 107 top-level items. Its clusters are: the declaration and manifest tests (`every_command_*`, `the_editor_declares_*`, `the_manifest_stitches_every_taxonomy_node`, lines 640 to 870); preview and run tests (`preview_eval_*`, `generation_preview_*`, `undo_redo_*`, `two_instances_converge_*`, 900 to 1000); menus and localized labels (`context_menu_*`, `window_measure_labels_*`, 1010 to 1200); mesh bridges and the OBJ, GLB and STL round trip (`mesh_*`, `generation3d_mesh_bridges_*`, 1200 to 1500); and the preview payload and channel tests (`preview_payload_*`, 1287 to 1620). Suggested children under `🧪️tests/🔬️…`: `declarations/`, `preview/`, `menus-labels/`, `mesh-bridge/`, `payload/`, `history/`, each with its own fixture helpers. Wrong direction: the composition test reads a procedural plugin fixture by a nine-level `include_str!` path. It is a dev composition test, so this is acceptable, but the path should go through the plugin owner.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs`

3,325 lines, Rust, items 143, status: free.

This 3,325-line file mixes four things: the channel version and admission (`CHANNEL_VERSION`, `admit_guest_channel_version`), the wire types and presence roster (`PresenceRosterWire`, `PresenceCommandCursor`, lines 200 to 290), the paged decoders for archives, routes and media (`PagedDocumentArchiveDecode`, `PagedRouteFieldsDecode`, `PagedMediaExportHandleDecode`, lines 364 to 806), the `AppCommand` and `AppFrame` enums with their paged cursor (1464 to 2000), and the document-archive load host (1953 to 2119), plus the binary read and write helpers (2119 onward). Suggested children: `presence/`, `archive-decode/`, `route-decode/`, `app-command/`, `app-frame/`, `archive-load-host/`, `binary-codec/`. The file already mounts `archive_schema` and `archive_decoding` as files. The `semio_framework_trace_memory_testing` import is `cfg(test)`-gated, so it is not a finding.

### `🧰️framework/🔨️modules/🔢️number/🦀️.rs`

3,271 lines, Rust, items 378, status: free.

This is the cleanest seam on the list. The file has eight inline `pub mod` blocks: `traits` (lines 20 to 285), `natural` (286 to 862), `integer` (863 to 1254), `rational` (1255 to 1607), `modular` (1608 to 1943), `primes` (1944 to 2449), `interval` (2450 to 2702), and `algebra` (2703 onward). Each block can become a `#[path]` child file with a `pub use` re-export, following EXECUTOR-RULES. The blocks depend on each other only through `traits`, so the split is mechanical. Imports are std and crate-local only, with no wrong direction.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`

3,259 lines, TSX, items 54, status: free (renderer TS, not in FLEET-CLAIMS).

The file is the TSX Interpreter for the renderer engine, not the Rust Interpreter that the fleet already carved. It has about 155 top-level items. Its clusters are: presence overlay contexts and hooks (199 to 245); panel-tree adapters (`PanelTreeNode`, `uiTreeNodeToTreePanelConfig`, 246 to 340); drag controllers (338 to 402); component scene-host resolution (`resolveComponentSceneHost`, 403 to 446); surface-kind scene lanes (`SURFACE_KIND_SCENE_*`, 447 to 492); menu and ref conversion (493 onward); and the main `InterpretedUiNode` dispatcher. Suggested children: `presence-overlay/`, `panel-tree/`, `drag/`, `scene-hosts/`, `surface-lanes/`. Wrong direction: the generic Interpreter imports twelve concrete element hosts (`Shell`, `NodeGraph`, `TextEditor`, `Table`, `Paint2dHost`, `TiledMapHost`, `Board2dHost`, `IconRenderHost`, `InkCanvasHost`, `GraphTimelineHost`, `BlockListHost`, `MediaTransportHost`), so the general layer depends on specific ones. Those hosts should register into a host contract, such as the existing `🌉️scene-host-contract`, that the Interpreter consumes.

## 4. Totals

### By language

| Language | Files (all kept) | Lines (all kept) | Files over 1,500 | Lines over 1,500 |
| --- | ---: | ---: | ---: | ---: |
| Rust | 30,045 | 3,526,070 | 261 | 608,109 |
| TypeScript | 10,519 | 842,880 | 48 | 206,984 |
| TSX | 523 | 200,715 | 21 | 63,507 |
| Python | 226 | 101,193 | 1 | 7,883 |
| CSS | 29 | 29,256 | 2 | 24,009 |
| WIT | 13 | 2,063 | 1 | 1,662 |
| **Total** | 41,355 | 4,702,177 | 334 | 912,154 |

### By area (files over 1,500 lines)

| Area | Files | Lines |
| --- | ---: | ---: |
| framework/os | 117 | 316,426 |
| framework/ui | 30 | 87,227 |
| framework/repo | 19 | 75,665 |
| framework/modules | 30 | 64,174 |
| hub | 13 | 44,167 |
| plugin:🧩️puzzle | 17 | 35,793 |
| plugin:🗄️stdio | 14 | 28,273 |
| plugin:🔋️energy | 8 | 26,201 |
| repo tooling (root script) | 1 | 25,739 |
| apps/dev (✏️s non-plugin) | 8 | 25,192 |
| plugin:📕️norm | 10 | 23,412 |
| vendored presentation project | 1 | 16,454 |
| plugin:🏛️architect | 7 | 14,064 |
| plugin:🌀️procedural | 6 | 13,882 |
| plugin:📸️remodel | 7 | 13,795 |
| plugin:🖍️draw | 7 | 13,784 |
| plugin:📐️cad | 6 | 12,127 |
| framework/products/🐾️pets | 5 | 10,684 |
| plugin:🏗️fem | 4 | 8,653 |
| plugin:📏️layout | 3 | 6,413 |
| framework/products/🎤️presentation | 2 | 5,578 |
| plugin:🏭️process | 2 | 5,544 |
| plugin:🌊️flow | 2 | 5,477 |
| plugin:🏙️bim | 2 | 5,351 |
| plugin:🖨️raster | 2 | 5,262 |
| framework/products/🖥️server | 2 | 4,818 |
| framework/products/📓️print | 2 | 3,975 |
| plugin:🎞️animate | 1 | 2,424 |
| plugin:🀄️wfc | 1 | 2,184 |
| framework/products/❓️quiz | 1 | 2,138 |
| plugin:💠️lowpoly | 1 | 2,026 |
| plugin:➗️mathematical | 1 | 1,952 |
| other | 1 | 1,771 |
| plugin:🗒️note | 1 | 1,529 |
| **Total** | 334 | 912,154 |

## 5. Findings for the coordinator

1. The largest file in the repo is the root `📜️script.ts` (25,739 lines). It holds the repo command classes plus a tool-job audit, and it imports at least 19 test modules. AGENTS.md allows only `📜️script.ts` as a script file, so the split must use plain `🟦️.ts` modules.
2. Status conflict on `🛢️db`. The brief lists db as peer-owned, but FLEET-CLAIMS.md lists `semio-framework-os-kernel-db` as this fleet's in-progress claim, and FLEET.md says db was carved into 41 crates. The three db files in the top 80 (3,363, 2,821 and 2,764 lines) are still large. Confirm the owner before anyone splits them.
3. Status conflict on the wgpu directory under `🖱️ui`. The renderer-wgpu plan covers it, while the UI GPU cut is left to the peer. Four files are marked contested.
4. Misplaced test: the `gis` service-composition test (7,388 lines) is mostly hub, replication and identity tests. Its owner is not gis.
5. Schema-first violation: the VS Code client (`🧩️vscode`) contains GraphQL codegen-style types from line 58 to at least line 440. Generate it into `🤖️generated` instead.
6. The TS kernel `🎠️kernel/🟦️.ts` (4,038 lines) is not named in FLEET-CLAIMS. It is the TS counterpart of the Rust kernel carve and imports the out-of-scope `🎭️actor` frontier.
7. Wrong-direction imports seen in headers: os tests reach into hub and ui fixtures (engine-contract), a framework dev test hard-codes the puzzle2d plugin (time-travel), the generic renderer Interpreter imports twelve concrete hosts, and the root script imports test modules.
8. Not a finding: `🧵️channel` imports the trace testing crate, but only under `cfg(test)`.
9. `♻️mit-bestand/…/layout.css` (16,454 lines) is a vendored presentation export. It should be excluded from split candidates, and it is the largest non-script source file in the census.
10. `git ls-files` printed `error: daemon terminated` on stderr twice (the fsmonitor daemon). The output was complete: 52,903 entries, and the kept list was 41,355.

No source file was edited. No git command that changes state was run. No ticket tool was called. Scripts and raw outputs are in `🗑️generated/mc2-*`.
