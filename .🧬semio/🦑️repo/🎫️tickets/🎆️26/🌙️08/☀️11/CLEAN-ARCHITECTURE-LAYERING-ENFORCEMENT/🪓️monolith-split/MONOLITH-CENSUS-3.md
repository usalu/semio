# 🪓️ Monolith Census 3

- Date: 2026-10-16. Auditor: Haiku 5.5 (read-only). Folder: `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
- Scope: tracked plus untracked `*.rs`, `*.ts`, `*.tsx`, `*.py`, `*.wit`, `*.css`, with the census 2 exclusions (`🗑️generated`, `🤖️generated`, `node_modules`, `.🧬semio`, `generated`, fixtures, corpora, snapshots, lock files, minified, `.d.ts`). Counted with `wc -l` under `LC_ALL=en_US.UTF-8`.
- Raw outputs: `🗑️generated/mc3-*` (file lists, `mc3-wc-raw.txt`, `mc3-rows.json`, `mc3-status.tsv`, `mc3-evidence.json`, `mc3-claims2.json`, scripts `mc3-*.py`).
- No source file was edited. No modifying git command was run. No sub-agent was used. No ticket tool was called.

## 1. Totals compared with census 2

Census 2 was taken at 04:51 from `git ls-files`. The tree has moved since then (1,229 paths are new, 96 are gone, 1,182 changed size), so the deltas below mix the work of the fleet with peer churn.

| Metric | Census 2 | Census 3 | Delta |
| --- | ---: | ---: | ---: |
| Files kept | 41,355 | 42,488 | +1,133 |
| Lines kept | 4,702,177 | 4,718,753 | +16,576 |
| Files over 1,500 lines | 334 | 267 | -67 |
| Lines in files over 1,500 | 912,154 | 642,759 | -269,395 |
| Files over 1,200 lines | 525 | 469 | -56 |
| Lines in files over 1,200 lines | 1,169,514 | 914,559 | -254,955 |
| Files over 1,000 lines | 702 | 658 | -44 |
| Lines in files over 1,000 lines | 1,363,048 | 1,121,171 | -241,877 |

By language, files over 1,500 lines (census 2 to census 3): Rust 261 to 217 (608,109 to 472,117 lines); TypeScript 48 to 31 (206,984 to 97,445); TSX 21 to 16 (63,507 to 47,526); CSS 2 to 2 (24,009 unchanged); WIT 1 to 1 (1,662); Python 1 to 0 (the energy mutation test, 7,883 lines, is now split).

Disposition of the 334 census-2 files over 1,500 lines: 12 are absent (63,387 lines; includes the `app-channel` move from `os/🔨️modules` to `framework/🔨️modules`, which is a relocation, not a split), 64 are now at or below 1,500 lines (220,902 lines before), and 258 are still over 1,500 (621,999 lines now). Nine files are new above 1,500 lines: store lifted engine pieces (3,189, 2,677, 2,637), new store test bins (2,489, 2,001, 1,862, 1,539) and the store sync native actor (2,475), and the app-channel TS file (1,891), which moved from the os product path (in the absent list) to `framework/🔨️modules`.

Census 3 is the total over all files, so the 1,200 threshold is an extra column that census 2 did not publish.

## 2. Top 60 files over 1,200 lines

Status legend: VENDORED; PEER (ui aggregate, UI GPU cut, ui-contract, store, db, flow, 2d, actor, job, media, member); PARENT-HELD (`📚️library`); RUN <slice> (a running slice). "inferred from tree" means the slice names the plugin family or the `os/🔌️plugin` tree in its tools (for example `pi1-tools`: 220 mentions of `os/🔌️plugin`) but not this file. SPLIT-LARGE means a finished slice split this file or its parent and it is still over 1,200 lines. FLEET-CLAIMED means a plan or claim without a running slice naming the file. FREE means no slice tool, RESULT, PLAN or FLEET mention was found for the path or its directory.

| # | Lines | Path | Owning area | Status | Parent of a split shrank as claimed? |
| ---: | ---: | --- | --- | --- | --- |
| 1 | 16,454 | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🧰️zukunft-bau-entwerfen-mit-bestand_files/📐️layout.css` | vendored presentation export | VENDORED (not a split target) | n/a |
| 2 | 11,948 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | framework/ui test | PEER (ui aggregate) | n/a |
| 3 | 11,928 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | repo library (📚️library) | PARENT-HELD | n/a |
| 4 | 10,787 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | repo library (📚️library) | PARENT-HELD | n/a |
| 5 | 10,650 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | os plugin runtime (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 6 | 7,555 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` | framework/ui styling | PEER (ui aggregate) | n/a |
| 7 | 7,539 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | repo library test (📚️library) | PARENT-HELD (touched by finished smg) | n/a |
| 8 | 7,122 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | os store engine (worker) | PEER (store engine); not split | n/a |
| 9 | 6,570 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | framework/ui react target | PEER (ui aggregate) | n/a |
| 10 | 5,177 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` | repo library (📚️library) | PARENT-HELD | n/a |
| 11 | 4,620 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | framework/ui elements | PEER (ui aggregate) | n/a |
| 12 | 4,101 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` | framework/ui wgpu (UI GPU cut) | PEER (UI GPU cut) | n/a |
| 13 | 3,909 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` | framework/ui wgpu (UI GPU cut) | PEER (UI GPU cut) | n/a |
| 14 | 3,640 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | os plugin runtime (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 15 | 3,416 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` | plugin puzzle family | RUN pch (puzzle family, inferred) | n/a |
| 16 | 3,393 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` | os plugin test (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 17 | 3,388 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` | framework/ui wgpu (UI GPU cut) | PEER (UI GPU cut) | n/a |
| 18 | 3,372 | `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | apps/dev composition tests | FREE (no slice or note names the file) | n/a |
| 19 | 3,363 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs` | os db (🛢️db) | PEER (db; FLEET-CLAIMS os-kernel-db in progress) | n/a |
| 20 | 3,347 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` | os infinite, world half | FLEET-CLAIMED (os-infinite; no running slice) | n/a |
| 21 | 3,319 | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` | framework/ui wgpu test | PEER (UI GPU cut) | n/a |
| 22 | 3,307 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/⚙️engine/🦀️.rs` | os infinite, board (2d) | PEER (2d board; waits for relocation session) | n/a |
| 23 | 3,189 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs` | os store engine (lifted from store root) | SPLIT-LARGE (okeng finished) + PEER (store) | store root 12,221 -> 1,033 (claimed 11,416 -> 1,345): shrank as claimed |
| 24 | 3,172 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs` | os plugin interpreter (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 25 | 3,144 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs` | os infinite, board (2d) | PEER (2d board) | n/a |
| 26 | 3,079 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | os flow (🌊️flow) | PEER (flow) | n/a |
| 27 | 3,070 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` | plugin procedural family | FLEET-CLAIMED (plugin crates plan; no running slice names it) | n/a |
| 28 | 3,026 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔭️unit-2/🦀️.rs` | os infinite, world half | FLEET-CLAIMED (os-infinite) | n/a |
| 29 | 3,017 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | plugin procedural family | FLEET-CLAIMED (plugin crates plan) | n/a |
| 30 | 3,006 | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` | plugin architect family | FLEET-CLAIMED (plugin crates plan) | n/a |
| 31 | 2,967 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs` | os infinite, world half | FLEET-CLAIMED (os-infinite) | n/a |
| 32 | 2,958 | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | plugin flow (🌊️flow) | PEER (flow) | n/a |
| 33 | 2,950 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🦀️.rs` | os infinite, world half | FLEET-CLAIMED (os-infinite) | n/a |
| 34 | 2,931 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs` | os infinite, world half | FLEET-CLAIMED (os-infinite) | n/a |
| 35 | 2,857 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | os store engine (sync tests) | RUN okwp8 (touched list) + PEER (store) | n/a |
| 36 | 2,849 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | os renderer TSX hosts | SPLIT-LARGE (SHELLHELPERS-WORLD3DHOST) + RUN rhu2 (renderer hosts) | 8,551 -> 2,845 claimed; 2,849 now: shrank as claimed |
| 37 | 2,821 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` | os db (🛢️db) | PEER (db) | n/a |
| 38 | 2,797 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | plugin puzzle family | FLEET-CLAIMED (plugin crates plan; pch puzzle overlap possible) | n/a |
| 39 | 2,796 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🦀️.rs` | os infinite, board (2d) | PEER (2d board) | n/a |
| 40 | 2,769 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` | hub trusted catalog | FLEET-CLAIMED (hub, W4-HUB in progress) | n/a |
| 41 | 2,764 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs` | os db (🛢️db) | PEER (db) | n/a |
| 42 | 2,746 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs` | os plugin testing (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 43 | 2,730 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts` | plugin stdio artifacts | RUN pch (stdio artifacts) | n/a |
| 44 | 2,726 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` | renderer Shell tests | QUEUED H1 hand-off (compile-blocked; not running) | n/a |
| 45 | 2,725 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/🦀️.rs` | plugin remodel family | FLEET-CLAIMED (plugin crates plan) | n/a |
| 46 | 2,677 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs` | os store engine (lifted from store root) | SPLIT-LARGE (okeng finished) + PEER (store) | store root shrank as claimed (see row 23) |
| 47 | 2,665 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` | os plugin host (os/🔌️plugin) | RUN pi1 (inferred from tree) | n/a |
| 48 | 2,664 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs` | plugin fem, 2d artifact (◻️2d) | PEER (2d) | n/a |
| 49 | 2,640 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` | renderer wgpu test | FLEET-CLAIMED (renderer-wgpu WP0 running; handed to ui peer) | n/a |
| 50 | 2,637 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs` | os store engine (lifted from store root) | SPLIT-LARGE (okeng finished) + PEER (store) | store root shrank as claimed (see row 23) |
| 51 | 2,634 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` | framework/ui wgpu (UI GPU cut) | PEER (UI GPU cut) | n/a |
| 52 | 2,625 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs` | os store engine (codec) | RUN okwp8 (touched list) + PEER (store) | n/a |
| 53 | 2,614 | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | framework/actor (🎭️actor) | PEER (actor) | n/a |
| 54 | 2,609 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` | repo test harness child | FREE (probable child of finished REPO-TEST-VSCODE split; ts3 overlap to confirm) | parent 7,656 gone; child shrank as claimed |
| 55 | 2,593 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs` | renderer wgpu test (ui-command wiring) | FLEET-CLAIMED (renderer-wgpu WP0; handed to ui peer) | n/a |
| 56 | 2,574 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs` | apps/dev fem engine | FREE (no slice or note names the file) | n/a |
| 57 | 2,573 | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` | framework/ui contract (🧬️contract) | PEER (ui-contract) | n/a |
| 58 | 2,549 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🦀️.rs` | os infinite, board (2d) | PEER (2d board) | n/a |
| 59 | 2,548 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts` | plugin energy family | FLEET-CLAIMED (plugin crates plan; pfb finished, file not in its scope) | n/a |
| 60 | 2,530 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs` | plugin layout family | FLEET-CLAIMED (plugin crates plan; H3 deferred) | n/a |

Summary of the top 60 (primary status per row): PEER 22, FLEET-CLAIMED 15, RUN 10, PARENT-HELD 4, SPLIT-LARGE 4, FREE 3, VENDORED 1, QUEUED 1. Rows 23, 36, 46 and 50 are SPLIT-LARGE with a second status; row 54 is FREE by probable parentage.

Across all 469 files over 1,200 lines (rule-plus-evidence classification, `mc3-status.tsv` and `mc3-evidence.json`): free by evidence 57 files (86,528 lines); peer-owned 136 (286,507); plugin family without a named running slice 127 (242,679); renderer 39 (65,404, of which renderer-wgpu WP0 is claimed); named slice or note reference 28 (46,724); parent-held 10 (43,789); running pch 24 (41,186); running pfa 22 (34,643); hub 16 (26,471); fleet-claimed other 9 (24,174); vendored 1 (16,454).

### Running slices and what they touch (evidence level)

- pi1: `os/🔌️plugin` tree (220 mentions in `pi1-tools`; plugin tier-1 flips). Inferred, not per file.
- pch: stdio artifacts, puzzle and wfc families (`pch-tools`: stdio 6, puzzle 5, wfc 3). Inferred.
- pfa: bim, process, raster and norm (norm 12, bim 4, process 4, raster 4). Inferred.
- okwp8: store sync and codec tests (`okwp8-tools/touched.txt` lists `🔄️sync/🧪️tests/🔬️unit/🦀️.rs`). Named.
- rhu2: renderer TS hosts (World3dHost, `rhu2-tools/work/roots-baseline.txt`). Named.
- hdc: hub directory port (the 9 untracked `🌎️hub/📇️directory/🔌️port/…` files, 15 to 77 lines each, are its in-progress output).
- ftc: quiz crate root and companion directories (`ftc-tools/families/quiz-root*.json`). Named by directory.
- opd: os pack dedupe (`os/🎒️pack`, `pack` family). Named by directory.
- otcp, ttc, psc, pos, rtd, tx3: no source-file claims found on the top-60 paths. rtd edits manifests only. psc rewrites import specifiers, and its skip list names only the ts3 geometry file.

## 3. Spot-check of RESULT claims (11 checks)

| # | Claim (RESULT file) | Claim | Now (wc -l) | Verdict |
| ---: | --- | --- | ---: | --- |
| 1 | RESULT-ROOT-SCRIPT-SPLIT.md | root `📜️script.ts` 25,739 to 166 lines | 166 | confirmed |
| 2 | RESULT-HUB-SCRIPT-SPLIT.md | hub `📜️script.ts` 17,556 to 80 lines | 80 | confirmed |
| 3 | RESULT-INTERPRETER-KERNEL-TS.md | `Interpreter/🟦️.tsx` 3,259 to a 150-line entry | 150 | confirmed |
| 4 | RESULT-REPO-TEST-VSCODE-SPLIT.md | vscode client 3,600 to 153 lines | 153 | confirmed |
| 5 | RESULT-FRAMEWORK-RUST-FILE-SPLITS.md | number parents 52 to 222 lines | `🔢️number/🦀️.rs` 64 | confirmed (in range) |
| 6 | RESULT-TS-SPLITS-3.md | spatial geometry 3,552 to 201 lines plus 14 children | 201 | confirmed |
| 7 | RESULT-RENDERER-TS-TESTS-SPLIT.md | engine-contract 14,454 deleted; largest topic 829 lines | `refresh-coalescing` 828; parent absent | confirmed, 1-line drift |
| 8 | RESULT-EPHEMERAL-SINGLE-OWNER.md | UI twin `🫧️ephemeral-state` deleted; kernel owner kept | twin absent; `🧰️framework/🔨️modules/🫧️transient/🟦️.ts` present | confirmed |
| 9 | RESULT-MISPLACED-E2E-TESTS.md | gis service-composition 7,388 relocated | file absent | confirmed |
| 10 | RESULT-PRODUCT-FILE-SPLITS.md (and ts3) | pets stage test 3,075 and presentation deck test 3,567 gone | both absent | confirmed (child sizes not re-measured) |
| 11 | RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md | World3dHost 8,551 to 2,845 lines | 2,849 | confirmed, 4-line drift |
| 12 | RESULT-OS-KERNEL-ENGINE.md | store root 11,416 to 1,345 lines | 1,033 | shrank further; claim conservative |

Also confirmed: the TS kernel `🎠️kernel/🟦️.ts` (4,038) is absent and its directory exists. Not re-measured: the 34 kernel children and the 53 engine-contract suites.

## 4. Ranked free packages for the next executors

Each package is one area. Sizes come from `wc -l`. Package 1 is clean and needs no hold. Packages 2 and 3 have a staged edit in git now, so they wait until the tree is clean. Packages 4 and 5 need a ts3 check first, and package 5 is also staged. Package 6 needs the user. Packages 7 and 8 are held. "git status" was run read-only on each package and is shown per package.

### Package 1: Repo product Rust packages (repo modules, not library)

- Status: FREE. Files: 5. Lines: 7,415.
- git status (read-only): `clean (no modified or untracked entries)`
  - 1,700 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🦀️rust/🦀️.rs`
  - 1,480 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧭️repo-cli/🚦️verbs/🦀️.rs`
  - 1,464 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust/🦀️.rs`
  - 1,451 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust/🦀️.rs`
  - 1,320 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🦀️rust/🦀️.rs`

### Package 2: Apps/dev composition generation3d tests

- Status: FREE by evidence (taf touched the directory only), but the 3,372-line unit test is staged (M) in git now: wait until clean. Files: 3. Lines: 5,975.
- git status (read-only): `M  ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  - 3,372 lines: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  - 1,368 lines: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🦀️.rs`
  - 1,235 lines: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧩️generation3d-example-geometry/🦀️.rs`

### Package 3: Apps/dev spatial-kernel engine (2 files; psc skip-list names only ts3 geometry)

- Status: FREE by evidence, but the preview file is staged (M) in git now: hold until clean. Files: 2. Lines: 2,934.
- git status (read-only): `M  ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts`
  - 1,676 lines: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs`
  - 1,258 lines: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts`

### Package 4: Repo test harness children (2 files; ts3 list needed first)

- Status: FREE (WAVE7 flags ts3 overlap). Files: 2. Lines: 4,171.
- git status (read-only): `clean (no modified or untracked entries)`
  - 2,609 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
  - 1,562 lines: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`

### Package 5: Presentation tests and pets climbing unit (mixed; ts3 and pfs finished)

- Status: HOLD: the two presentation test files are staged as new (A) in git now, so they are in flight; ts3 children to check. Files: 3. Lines: 4,001.
- git status (read-only): `A  🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖱️interaction-dom/🟦️.tsx
A  🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖼️deck-rendering/🟦️.tsx`
  - 1,443 lines: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖱️interaction-dom/🟦️.tsx`
  - 1,330 lines: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🖼️deck-rendering/🟦️.tsx`
  - 1,228 lines: `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🦀️.rs`

### Package 6: Framework 3D mesh and BREP (gated: ask the user first)

- Status: FREE by evidence, GATED by user memory (BREP boolean work). Files: 3. Lines: 5,661.
- git status (read-only): `clean (no modified or untracked entries)`
  - 2,462 lines: `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs`
  - 1,750 lines: `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/📏mass-properties/🦀️.rs`
  - 1,449 lines: `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🎨️blend/🦀️.rs`

### Package 7: Quiz product (HOLD: ftc running; quiz-root and companion families)

- Status: HOLD. Files: 4. Lines: 5,477.
- git status (read-only): `clean (no modified or untracked entries)`
  - 1,450 lines: `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts`
  - 1,396 lines: `🧰️framework/🛍️products/❓️quiz/🧪️tests/🐾️pet-companions/🟦️.tsx`
  - 1,385 lines: `🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs`
  - 1,246 lines: `🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx`

### Package 8: Framework pack (HOLD: opd running pack dedupe; dm2 finished)

- Status: HOLD. Files: 3. Lines: 4,186.
- git status (read-only): `clean (no modified or untracked entries)`
  - 1,505 lines: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs`
  - 1,358 lines: `🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs`
  - 1,323 lines: `🧰️framework/🔨️modules/🎒️pack/🧬️schema/📡️record-token/🦀️.rs`

No package has a running-slice mention of its exact paths. Package 2 includes the 3,372-line generation3d unit test (top-60 row 18), whose only tool mention is the finished `taf-tools` directory reference. The package 3 preview file is named once in the finished RESULT-TS-SPLITS-3.md.

Shortfalls: only package 1 has five clean free files. Package 2 has 3 files, packages 3 and 4 have 2 each, and package 5 mixes two products. The os product modules (`📡️spr` channel, `🗣️dsl` derive, the dev staging test) were left out: `okwp0`, `dm2`, `okeng` and `otc` touched them, and they sit in the os-kernel plan.

## 5. Risks and hand-offs

- Status is evidence-based: a slice tool, RESULT, PLAN or FLEET mention, or a family rule. It is not verified per file body. "free" means no claim was found, not that the file is safe to edit.
- The plugin family (127 files over 1,200 lines) has no named running slice on most of its files, but FLEET-CLAIMS lists "plugin crates (79 oversized)" as claimed. Treat all of it as claimed until the plugin plan says otherwise.
- Compile is still gated (FLEET 16:xx). Packages 1 to 6 are static-only until the window opens.
- ts3 has no recorded file list (WAVE7 §6.1). Check it before packages 4 and 5.
- Packages 7 and 8 are held for ftc and opd. Package 6 is gated by the BREP memory.
- The count of 54,900 tracked matches is larger than census 2 (52,903 `git ls-files` entries) because of churn. The method is the same. `git` ran with `-c core.fsmonitor=false` to avoid the fsmonitor daemon error.
- `mc3-claims.py` is an unfinished first attempt (killed by timeout); `mc3-claims2.py` is the one that produced the evidence.
