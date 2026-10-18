# 🪓 Monolith Census 5

- Date: 2026-10-17. Auditor: Haiku 5.5 (read-only). Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
- Scope: the census 4 method (`MONOLITH-CENSUS-4.md` §1 to §4) over tracked and untracked `*.rs`, `*.ts`, `*.tsx`, `*.mts`, `*.py`, `*.wit`, `*.css`. Excluded: `.🧬semio`, `generated`, `node_modules`, `.d.ts`, fixture, snapshot and corpus folders (the census 4 filter, unchanged), plus the vendored tree `♻️mit-bestand/` (76 files, 24,670 lines, see section 1). Lock files and JSON are not in the extension set. `.mts` is added to the census 5 extension set.
- Lines are newline counts. `git ls-files` ran read-only with `-c core.fsmonitor=false`. Tree scan: `🗑️generated/mc5-count.py`, the classifier `mc5-classify.py`, packages `mc5-packages.py`, this report `mc5-report.py`. Raw data: `🗑️generated/mc5-rows.json` (all kept files), `mc5-over1000.json` (all 294 files over 1,000 lines with class, slice, flags, generator header), `mc5-packages.json`.
- Running slices: taken from the last 50 lines of `📓️FLEET.md` (r9a, hdr, ohp, ohf, rg7, tt6, mwu, bd11, gtk). The brief also lists ohr and ohh. Neither has a RESULT document, neither is named in the last 50 lines, and neither has any file evidence, so they claim no file here.
- No source file was edited. No modifying git command ran. No sub-agent ran. No ticket tool was called. No recursive grep or find ran over the repo root (a plain grep alias is broken, so the scripts use `git ls-files` and direct paths). The shell grep alias was not used.

## 1. Totals compared with census 4

Census 4 is shown as it was reported, and again recomputed with the vendored tree removed from its `mc4-rows.json`, so that both columns use the census 5 basis. Census 4 was taken on 2026-10-16 at about 23:00, so the deltas also include all fleet work since then.

| Metric | Census 4 (reported) | Census 4 (no vendored) | Census 5 | Delta vs census 4 (no vendored) |
| --- | ---: | ---: | ---: | ---: |
| Files kept | 43,320 | 43,244 | 44,443 | +1,199 |
| Lines kept | 4,730,407 | 4,705,737 | 4,727,216 | +21,479 |
| Files over 1,500 lines | 258 | 257 | 143 | -114 |
| Lines in files over 1,500 | 623,221 | 606,767 | 361,505 | -245,262 |
| Files over 1,000 lines | 638 | 635 | 294 | -341 |
| Lines in files over 1,000 | 1,086,090 | 1,067,578 | 547,433 | -520,145 |

The 635 census 4 files over 1,000 lines (vendored excluded) now split out as: 283 still over 1,000 lines, 349 now at or below 1,000 lines, and 3 no longer kept (split or moved). 10 files are over 1,000 lines now and were not in census 4, and 1 grew from 1,000 lines or fewer to over 1,000. Their paths are in `mc5-over1000.json`.

Vendored tree, excluded: 3 files over 1,000 lines in census 4 (`layout.css` 16,454, `main.css` 1,050, `brand.ts` 1,008). Census 5 keeps none of them. The vendored tree has 76 files, 24,670 lines, in total.

## 2. Classification of the 294 files over 1,000 lines

Each file gets the first matching class, in this order: PARENT (`📚️library`), PEER (path rule from census 4), RUNNING (the exact path, or a path suffix of three or more components, appears in a running slice RESULT document), FINISHED (exact path in a finished RESULT, AUDIT, DEPENDENCY, CRATE-SOURCE-GATE-CENSUS or PRODUCER-CRATE-RECIPE document), SCOPE (no file-level claim, but the path falls in the area a running slice names: see below), then FREE. FREE means no claim was found. It does not mean the file was checked.

| Class | Files | Lines | Census 4 files | Census 4 lines |
| --- | ---: | ---: | ---: | ---: |
| FREE | 76 | 117,864 | 408 | 624,271 |
| SCOPE | 6 | 8,941 | — | — |
| PEER | 172 | 324,780 | 174 | 328,997 |
| PARENT | 20 | 55,573 | 19 | 53,477 |
| FINISHED | 20 | 40,275 | 18 | 35,212 |
| RUNNING | 0 | 0 | 16 | 25,621 |
| Total | 294 | 547,433 | 638 | 1,086,090 |

**Running slices.** No file over 1,000 lines is named by path in the RESULT document of any of the nine running slices (r9a, hdr, ohp, ohf, rg7, tt6, mwu, bd11, gtk). Those documents name crates, tool scripts and directories. So the RUNNING class is empty. The scope hints below are the only file-level signal left. They are a judgement from the area each slice describes, not a claim by path.

**Census 4 running claims.** Census 4 held 16 over-1,000 files for the pi2 and e51 slices. Neither slice is in the last 50 lines of FLEET.md (pi2 is last logged as PLUGIN-INTEGRATION-2 DONE at 18:1x), so the brief does not treat them as running. All 16 of those files are still over 1,000 lines, with sizes moved by at most 13 lines. Under the rules here they are FINISHED 3, FREE 13.

| Slice | Area in its RESULT or FLEET note | Files in scope hint (over 1,000) |
| --- | --- | ---: |
| r9a | BIM model rehome (`🏙️bim`) | 0 |
| hdr | crate headroom (crate-level, no file list) | 0 |
| ohp | oracle harvest, plugins (`🔮️oracles` under plugin trees) | 0 |
| ohf | oracle harvest, framework (`🔮️oracles` under framework) | 0 |
| rg7 | repo gates (3 named suites) | 0 |
| tt6 | TypeScript types (directory and renderer TS) | 4 |
| mwu | mirror unwind (`🪞️mirror` and named plugin artifacts) | 1 |
| bd11 | baseline defects (plugin and actor TS tests) | 1 |
| gtk | generator testkit (`🧰️testkit`, `📦️crate-tree`) | 0 |
| ohr | no RESULT document, no FLEET note | 0 |
| ohh | no RESULT document, no FLEET note | 0 |

## 3. Ranked table: every file over 1,000 lines

Columns: rank; lines; lang; class (the reason in brackets when it has one); running slice named by path, or a scope hint marked `scope:`; area; generated (`header` = a generator header was found, `sig` = the census 4 generated-parser signature `validateAgainst` or `(value: unknown)` without a header; `no` = neither); `W` = under a peer-held `🧊️wgpu` tree; package number (see section 4); path.

| Rank | Lines | Lang | Class | Slice | Area | Gen | W | Pkg | Path |
| ---: | ---: | --- | --- | --- | --- | --- | --- | ---: | --- |
| 1 | 11,949 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` |
| 2 | 11,928 | ts | PARENT |  | framework/other | sig |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` |
| 3 | 10,787 | ts | PARENT |  | framework/other | sig |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` |
| 4 | 7,555 | css | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` |
| 5 | 7,535 | ts | PARENT |  | framework/other | sig |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` |
| 6 | 7,083 | ts | PEER [store] |  | os/other | sig |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` |
| 7 | 6,570 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` |
| 8 | 5,182 | ts | PARENT |  | framework/other | sig |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` |
| 9 | 4,620 | ts | PEER [ui-aggregate-react] |  | framework/modules | sig |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` |
| 10 | 4,101 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` |
| 11 | 3,909 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` |
| 12 | 3,643 | ts | FINISHED [named-in:RESULT-PLUGIN-RUNTIME-TSX.md] | scope:bd11 | os/🔌️plugin | sig |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` |
| 13 | 3,388 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` |
| 14 | 3,363 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs` |
| 15 | 3,318 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` |
| 16 | 3,307 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/⚙️engine/🦀️.rs` |
| 17 | 3,172 | rust | FREE |  | os/🔌️plugin | no |  | 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs` |
| 18 | 3,144 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs` |
| 19 | 3,077 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` |
| 20 | 3,070 | rust | FINISHED [named-in:RESULT-PACK-QUIZ-FILE-SPLITS.md,RESULT-SYNTAX-SWEEP-7.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` |
| 21 | 3,019 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 22 | 2,858 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` |
| 23 | 2,821 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` |
| 24 | 2,796 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🦀️.rs` |
| 25 | 2,764 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs` |
| 26 | 2,749 | rust | PEER [flow] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 27 | 2,676 | rust | FINISHED [named-in:RESULT-MISSING-TEST-INPUTS.md,RESULT-SMALL-FIXES-1.md] |  | os/📺️renderer | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` |
| 28 | 2,670 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs` |
| 29 | 2,667 | rust | FREE |  | os/🔌️plugin | no |  | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` |
| 30 | 2,664 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs` |
| 31 | 2,637 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs` |
| 32 | 2,634 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` |
| 33 | 2,621 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs` |
| 34 | 2,614 | ts | PEER [actor] |  | framework/modules | sig |  |  | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` |
| 35 | 2,573 | rust | PEER [ui-contract] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` |
| 36 | 2,548 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🦀️.rs` |
| 37 | 2,524 | rust | FREE |  | plugin-family | no |  | 7 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs` |
| 38 | 2,508 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs` |
| 39 | 2,489 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🦀️.rs` |
| 40 | 2,481 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🦀️.rs` |
| 41 | 2,479 | rust | PEER [flow] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` |
| 42 | 2,478 | rust | FREE |  | os/🔌️plugin | no |  | 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🦀️.rs` |
| 43 | 2,475 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🖥️native/🎭️actor/🦀️.rs` |
| 44 | 2,473 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧾️log/🦀️.rs` |
| 45 | 2,418 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs` |
| 46 | 2,406 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🖼️scene/🦀️.rs` |
| 47 | 2,404 | rust | FREE |  | os/other | no |  | 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` |
| 48 | 2,392 | rust | FINISHED [named-in:RESULT-RETAINED-STEP-PRODUCTION.md] |  | os/🔌️plugin | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🦀️.rs` |
| 49 | 2,386 | rust | FINISHED [named-in:AUDIT-WAVE7-COMPLIANCE.md] |  | os/🔌️plugin | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs` |
| 50 | 2,371 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs` |
| 51 | 2,354 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` |
| 52 | 2,321 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` |
| 53 | 2,311 | ts | FINISHED [named-in:RESULT-REPO-TEST-VSCODE-SPLIT.md,RESULT-WRONG-DIRECTION-RELOCATION.md] |  | framework/modules | sig |  |  | `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` |
| 54 | 2,215 | rust | FINISHED [named-in:RESULT-PRIVACY-ORACLE-SWEEP-2.md] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` |
| 55 | 2,198 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs` |
| 56 | 2,184 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/📜️declaration/🦀️.rs` |
| 57 | 2,182 | rust | PEER [job] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs` |
| 58 | 2,180 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` |
| 59 | 2,158 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx` |
| 60 | 2,155 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` |
| 61 | 2,143 | rust | FREE |  | os/🔌️plugin | no |  | 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-5/🦀️.rs` |
| 62 | 2,136 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` |
| 63 | 2,135 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` |
| 64 | 2,132 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🎯️focus/🦀️.rs` |
| 65 | 2,109 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🏗️catalog-create/🦀️.rs` |
| 66 | 2,096 | rust | FINISHED [named-in:RESULT-REPO-HUB-RUST-FILE-SPLITS.md] |  | hub | no |  |  | `🌎️hub/🧪️tests/🔬️bin-unit/📇️directory/🔬️unit/🦀️.rs` |
| 67 | 2,093 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs` |
| 68 | 2,092 | ts | SCOPE [tt6] | scope:tt6 | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` |
| 69 | 2,076 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🦀️.rs` |
| 70 | 2,063 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🗃️model/🦀️.rs` |
| 71 | 2,053 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx` |
| 72 | 2,047 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs` |
| 73 | 2,044 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🎬️actions-a/🦀️.rs` |
| 74 | 2,035 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` |
| 75 | 2,018 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🎞️actions-b/🦀️.rs` |
| 76 | 2,004 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs` |
| 77 | 2,003 | rust | FINISHED [named-in:AUDIT-WAVE7-COMPLIANCE.md] |  | os/🔌️plugin | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` |
| 78 | 2,001 | rust | FREE |  | plugin-family | no |  | 7 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 79 | 2,000 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🦀️.rs` |
| 80 | 1,996 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🎫️handle/🦀️.rs` |
| 81 | 1,985 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🕸️graph/🦀️.rs` |
| 82 | 1,972 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📇️io-registry/🦀️.rs` |
| 83 | 1,965 | ts | PARENT | mwu+r9a | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️crate-tree/🟦️.ts` |
| 84 | 1,963 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` |
| 85 | 1,958 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` |
| 86 | 1,942 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🗃️model/🦀️.rs` |
| 87 | 1,941 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🔀️reconcile/🦀️.rs` |
| 88 | 1,924 | rust | FREE |  | os/🔌️plugin | no |  | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/🦀️.rs` |
| 89 | 1,916 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs` |
| 90 | 1,909 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/📜️declaration/📐️layout/🦀️.rs` |
| 91 | 1,896 | rust | FREE |  | hub | no |  | 12 | `🌎️hub/🧪️tests/🔬️bin-unit/🚀️boot/🔬️unit/🦀️.rs` |
| 92 | 1,895 | ts | FINISHED [named-in:RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md] | scope:ohf+tt6 | os/📺️renderer | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts` |
| 93 | 1,893 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🦀️.rs` |
| 94 | 1,876 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` |
| 95 | 1,867 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` |
| 96 | 1,865 | rust | FREE |  | os/🔌️plugin | no |  | 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🦀️.rs` |
| 97 | 1,862 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🦀️.rs` |
| 98 | 1,861 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs` |
| 99 | 1,858 | ts | SCOPE [bd11] | scope:bd11 | os/🔌️plugin | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` |
| 100 | 1,856 | rust | FREE |  | os/🔌️plugin | no |  | 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs` |
| 101 | 1,849 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` |
| 102 | 1,848 | rust | FREE |  | os/🔌️plugin | no |  | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🦀️.rs` |
| 103 | 1,839 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel-part-1/🦀️.rs` |
| 104 | 1,820 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` |
| 105 | 1,817 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs` |
| 106 | 1,812 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🦀️.rs` |
| 107 | 1,800 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run-part-1/🦀️.rs` |
| 108 | 1,796 | rust | PEER [actor] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` |
| 109 | 1,777 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` |
| 110 | 1,765 | rust | PEER [job] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🎭️actor/💼️job/📨️publication/🦀️.rs` |
| 111 | 1,764 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🦀️.rs` |
| 112 | 1,750 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/📏mass-properties/🦀️.rs` |
| 113 | 1,728 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` |
| 114 | 1,725 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs` |
| 115 | 1,720 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` |
| 116 | 1,698 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs` |
| 117 | 1,681 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs` |
| 118 | 1,679 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs` |
| 119 | 1,672 | rust | FREE |  | framework/modules | no |  | 10 | `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs` |
| 120 | 1,662 | wit | FREE |  | os/🔌️plugin | no |  | 13 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit` |
| 121 | 1,656 | rust | FREE |  | os/🔌️plugin | no |  | 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` |
| 122 | 1,644 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🧪️tests/🔬️board-host/🦀️.rs` |
| 123 | 1,603 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` |
| 124 | 1,601 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📜️history/🦀️.rs` |
| 125 | 1,599 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🕹️gestures/🦀️.rs` |
| 126 | 1,596 | rust | FINISHED [named-in:RESULT-PACK-QUIZ-FILE-SPLITS.md] |  | os/🔌️plugin | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs` |
| 127 | 1,593 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️unit/🦀️.rs` |
| 128 | 1,588 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx` |
| 129 | 1,587 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs` |
| 130 | 1,584 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs` |
| 131 | 1,584 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/⏭️cursor/🦀️.rs` |
| 132 | 1,578 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` |
| 133 | 1,578 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` |
| 134 | 1,572 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🦀️.rs` |
| 135 | 1,570 | rust | PEER [actor] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🎭️actor/📖️vocabulary/🦀️.rs` |
| 136 | 1,569 | rust | FREE |  | os/🔌️plugin | no |  | 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` |
| 137 | 1,564 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 138 | 1,562 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` |
| 139 | 1,562 | ts | FREE |  | os/🔌️plugin | no |  | 11 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` |
| 140 | 1,548 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs` |
| 141 | 1,525 | ts | PARENT | scope:gtk | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts` |
| 142 | 1,505 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔘️state/🦀️.rs` |
| 143 | 1,501 | ts | PEER [ui-aggregate-react] |  | framework/modules | sig |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout/🟦️.ts` |
| 144 | 1,500 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts` |
| 145 | 1,499 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs` |
| 146 | 1,497 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs` |
| 147 | 1,494 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs` |
| 148 | 1,493 | rust | PEER [ui-contract] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs` |
| 149 | 1,490 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🟦️.ts` |
| 150 | 1,485 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🦀️.rs` |
| 151 | 1,479 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🖌️render/✨️shader-contract/🦀️.rs` |
| 152 | 1,479 | rust | PEER [ui-contract] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🦀️.rs` |
| 153 | 1,473 | rust | FREE |  | os/🔌️plugin | no |  | 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🦀️.rs` |
| 154 | 1,470 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🔌️backend/🦀️.rs` |
| 155 | 1,464 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 156 | 1,464 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️.rs` |
| 157 | 1,462 | rust | PEER [2d] |  | os/♾️infinite | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🧪️tests/🔬️dag-unit/🦀️.rs` |
| 158 | 1,459 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs` |
| 159 | 1,458 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs` |
| 160 | 1,456 | rust | FREE |  | plugin-family | no |  | 7 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/🦀️.rs` |
| 161 | 1,450 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs` |
| 162 | 1,449 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🎨️blend/🦀️.rs` |
| 163 | 1,433 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🦀️.rs` |
| 164 | 1,432 | ts | SCOPE [tt6] | scope:tt6 | hub | no | W |  | `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration/🟦️.ts` |
| 165 | 1,432 | rust | FREE |  | os/🔌️plugin | no |  | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs` |
| 166 | 1,430 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🥾️catalog-bootstrap/🦀️.rs` |
| 167 | 1,428 | rust | FREE |  | plugin-family | no |  | 7 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs` |
| 168 | 1,420 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs` |
| 169 | 1,420 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🦀️.rs` |
| 170 | 1,412 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 171 | 1,407 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts` |
| 172 | 1,405 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/🦀️.rs` |
| 173 | 1,405 | rust | FREE |  | os/other | no |  | 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` |
| 174 | 1,398 | ts | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts` |
| 175 | 1,395 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs` |
| 176 | 1,394 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🦀️.rs` |
| 177 | 1,386 | ts | PEER [ui-aggregate-react] |  | framework/modules | sig |  |  | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` |
| 178 | 1,372 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🦀️.rs` |
| 179 | 1,368 | rust | FINISHED [named-in:DEPENDENCY-DIRECTION-AUDIT-2.md] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs` |
| 180 | 1,368 | rust | PEER [ui-contract] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` |
| 181 | 1,368 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🦀️.rs` |
| 182 | 1,361 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` |
| 183 | 1,357 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs` |
| 184 | 1,356 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🌳️host-evaluation-history/🦀️.rs` |
| 185 | 1,348 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🦀️.rs` |
| 186 | 1,337 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/📖️catalog-read/🦀️.rs` |
| 187 | 1,337 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🟦️.ts` |
| 188 | 1,336 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` |
| 189 | 1,336 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs` |
| 190 | 1,329 | rust | FREE |  | os/🔌️plugin | no |  | 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel-part-2/🦀️.rs` |
| 191 | 1,322 | rust | FREE |  | plugin-family | no |  | 7 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` |
| 192 | 1,318 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` |
| 193 | 1,316 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🦀️.rs` |
| 194 | 1,315 | ts | SCOPE [tt6] | scope:tt6 | os/📺️renderer | sig | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` |
| 195 | 1,307 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs` |
| 196 | 1,306 | ts | FINISHED [named-in:RESULT-OS-HOST-MCP-WORKSPACE.md] |  | hub | no |  |  | `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` |
| 197 | 1,295 | rust | FREE |  | os/other | no |  | 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs` |
| 198 | 1,293 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs` |
| 199 | 1,287 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📁️fs/🦀️.rs` |
| 200 | 1,285 | rust | PEER [job] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧵️job/👷️worker/🦀️.rs` |
| 201 | 1,283 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs` |
| 202 | 1,282 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎬️action/🦀️.rs` |
| 203 | 1,281 | rust | PEER [job] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧵️job/🦀️.rs` |
| 204 | 1,279 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` |
| 205 | 1,278 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling/🦀️.rs` |
| 206 | 1,264 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs` |
| 207 | 1,259 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🔤️typography/🧩️component/🦀️.rs` |
| 208 | 1,257 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 209 | 1,255 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` |
| 210 | 1,253 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/✏️host-authoring/🦀️.rs` |
| 211 | 1,243 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/🦀️.rs` |
| 212 | 1,241 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs` |
| 213 | 1,238 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts` |
| 214 | 1,236 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/🦀️.rs` |
| 215 | 1,233 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/📟️vt/🧱️screen/🦀️.rs` |
| 216 | 1,230 | rust | PEER [ui-contract] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🦀️.rs` |
| 217 | 1,218 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🦀️.rs` |
| 218 | 1,215 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` |
| 219 | 1,213 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs` |
| 220 | 1,211 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` |
| 221 | 1,211 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs` |
| 222 | 1,204 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/📟️vt/🧪️tests/🔬️unit/🦀️.rs` |
| 223 | 1,204 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs` |
| 224 | 1,202 | rust | FREE |  | os/🔌️plugin | no |  | 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs` |
| 225 | 1,198 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/🦀️.rs` |
| 226 | 1,194 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs` |
| 227 | 1,190 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 228 | 1,183 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 229 | 1,179 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs` |
| 230 | 1,175 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/📤️staging/🦀️.rs` |
| 231 | 1,175 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs` |
| 232 | 1,175 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs` |
| 233 | 1,170 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs` |
| 234 | 1,163 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🗃️memory/🦀️.rs` |
| 235 | 1,156 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts` |
| 236 | 1,154 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 237 | 1,154 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx` |
| 238 | 1,142 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🦀️.rs` |
| 239 | 1,142 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs` |
| 240 | 1,140 | ts | SCOPE [tt6] | scope:tt6 | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` |
| 241 | 1,139 | rust | FREE |  | framework/modules | no |  | 10 | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs` |
| 242 | 1,139 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs` |
| 243 | 1,138 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✂️intersect/🏄️surface-surface/🦀️.rs` |
| 244 | 1,135 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs` |
| 245 | 1,135 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🟦️.ts` |
| 246 | 1,125 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs` |
| 247 | 1,120 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs` |
| 248 | 1,114 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔺️euler/🦀️.rs` |
| 249 | 1,110 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts` |
| 250 | 1,109 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs` |
| 251 | 1,105 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs` |
| 252 | 1,105 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🦀️.rs` |
| 253 | 1,104 | rust | SCOPE [mwu] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` |
| 254 | 1,100 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs` |
| 255 | 1,098 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` |
| 256 | 1,097 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/🦀️.rs` |
| 257 | 1,097 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs` |
| 258 | 1,096 | rust | FREE |  | os/🔌️plugin | no |  | 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs` |
| 259 | 1,091 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🚇️pty/🦀️.rs` |
| 260 | 1,089 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🦀️.rs` |
| 261 | 1,086 | rust | PARENT | scope:r9a | plugin-family | no |  |  | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/📚️library/🦀️.rs` |
| 262 | 1,084 | ts | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` |
| 263 | 1,078 | rust | PEER [flow] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/⏱️eval-session/🦀️.rs` |
| 264 | 1,069 | rust | PEER [ui-wgpu-cut] |  | framework/modules | no | W |  | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` |
| 265 | 1,069 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🦀️.rs` |
| 266 | 1,065 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` |
| 267 | 1,061 | rust | FREE |  | os/🔌️plugin | no |  | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs` |
| 268 | 1,057 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs` |
| 269 | 1,055 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs` |
| 270 | 1,055 | ts | PARENT | rg7 | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` |
| 271 | 1,054 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` |
| 272 | 1,052 | ts | FREE |  | os/🔌️plugin | sig |  | 11 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` |
| 273 | 1,051 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🌐️wasm/🎭️actor/🦀️.rs` |
| 274 | 1,047 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs` |
| 275 | 1,042 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🦀️.rs` |
| 276 | 1,040 | rust | PEER [3d-mesh-brep] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🦀️.rs` |
| 277 | 1,040 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/🟦️.ts` |
| 278 | 1,036 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs` |
| 279 | 1,035 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🔡️ansi/🦀️.rs` |
| 280 | 1,034 | rust | PEER [store] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` |
| 281 | 1,034 | rust | FREE |  | os/🔌️plugin | no |  | 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs` |
| 282 | 1,034 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` |
| 283 | 1,027 | ts | PARENT |  | framework/other | sig |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 284 | 1,026 | rust | PEER [2d] | scope:mwu | plugin-family | no |  |  | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs` |
| 285 | 1,022 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs` |
| 286 | 1,018 | rust | FINISHED [named-in:AUDIT-REGIONS-NAMES-SIZES.md] |  | plugin-family | no |  |  | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs` |
| 287 | 1,016 | ts | PARENT |  | framework/other | no |  |  | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🟦️.ts` |
| 288 | 1,014 | rust | FREE |  | os/📺️renderer | no | W |  | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` |
| 289 | 1,013 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` |
| 290 | 1,013 | rust | PEER [db] |  | os/other | no |  |  | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` |
| 291 | 1,012 | ts | FREE |  | plugin-family | no |  | 14 | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` |
| 292 | 1,012 | rust | FREE |  | os/🔌️plugin | no |  | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs` |
| 293 | 1,008 | rust | PEER [ui-aggregate-react] |  | framework/modules | no |  |  | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖋️text/🦀️.rs` |
| 294 | 1,008 | rust | FREE |  | os/🔌️plugin | no |  | 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs` |

## 4. Free packages

The packages cover the 54 FREE files that are not under the peer-held `🧊️wgpu` tree. They form 14 packages of 54 files and 86,276 lines. The rule is one language and one area per package, path order inside it, at most 8 files and at most 10,000 lines. A chunk of under 4 files merges into the previous chunk of the same group when the merge stays within 8 files and 11,000 lines. The brief asks for 6 to 8 files per package. The line cap binds first, because free files average about 1,600 lines, so most packages hold 5 to 6 files.

| No | Lang | Area | Files | Lines |
| ---: | --- | --- | ---: | ---: |
| 1 | rust | os/🔌️plugin | 6 | 9,780 |
| 2 | rust | os/🔌️plugin | 5 | 9,528 |
| 3 | rust | os/🔌️plugin | 6 | 9,197 |
| 4 | rust | os/🔌️plugin | 6 | 8,934 |
| 5 | rust | os/🔌️plugin | 5 | 8,920 |
| 6 | rust | os/🔌️plugin | 5 | 8,883 |
| 7 | rust | plugin-family | 5 | 8,731 |
| 8 | rust | os/🔌️plugin | 6 | 7,204 |
| 9 | rust | os/other | 3 | 5,104 |
| 10 | rust | framework/modules | 2 | 2,811 |
| 11 | ts | os/🔌️plugin | 2 | 2,614 |
| 12 | rust | hub | 1 | 1,896 |
| 13 | wit | os/🔌️plugin | 1 | 1,662 |
| 14 | ts | plugin-family | 1 | 1,012 |

Package 1 (rust, os/🔌️plugin, 6 files, 9,780 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run-part-1/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel-part-1/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel-part-2/🦀️.rs`

Package 2 (rust, os/🔌️plugin, 5 files, 9,528 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-5/🦀️.rs`

Package 3 (rust, os/🔌️plugin, 6 files, 9,197 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`

Package 4 (rust, os/🔌️plugin, 6 files, 8,934 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧩️app/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs`

Package 5 (rust, os/🔌️plugin, 5 files, 8,920 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs`

Package 6 (rust, os/🔌️plugin, 5 files, 8,883 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`

Package 7 (rust, plugin-family, 5 files, 8,731 lines):
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

Package 8 (rust, os/🔌️plugin, 6 files, 7,204 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`

Package 9 (rust, os/other, 3 files, 5,104 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`

Package 10 (rust, framework/modules, 2 files, 2,811 lines):
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs`
- `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs`

Package 11 (ts, os/🔌️plugin, 2 files, 2,614 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`

Package 12 (rust, hub, 1 files, 1,896 lines):
- `🌎️hub/🧪️tests/🔬️bin-unit/🚀️boot/🔬️unit/🦀️.rs`

Package 13 (wit, os/🔌️plugin, 1 files, 1,662 lines):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`

Package 14 (ts, plugin-family, 1 files, 1,012 lines):
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`

## 5. Held and excluded

- Peer-held `🧊️wgpu` tree: 22 FREE files (31,588 lines) are kept out of the packages. Census 4 kept them in package 31 and in other packages. Census 5 treats them as held, because the brief asks for the peer-held status, and HANDOFFS G-11 gives wgpu renderer artifacts to the UI peer.
- Scope hints: 6 files (8,941 lines). They are not in the packages until the slice that names their area finishes.
- Peer rules: 172 files (324,780 lines). Parent (`📚️library`): 20 files (55,573 lines). Finished claims: 20 files (40,275 lines).

## 6. Generated check

- No file over 1,000 lines has a generator header in its first 25 lines. The header test is `@generated`, `DO NOT EDIT`, `auto-generated`, `GENERATED FILE`, or `generated by`. The generated-parser signature (`validateAgainst` or `(value: unknown)`) matches 14 files. Their class and package are in the table. The census 4 claim that the parsers come from `schema parsers` is not re-checked here, so each of them still needs the owner check before an edit.
- 39 files over 1,000 lines are under a `🧊️wgpu` tree: framework/modules 13, hub 1, os/📺️renderer 25. By class: FREE 22, PEER 13, SCOPE 4.

## 7. Caveats

1. "FREE" means no claim was found by exact path, by a path suffix in a running RESULT, or by a finished document. It does not mean the file was read.
2. The scope hints are a judgement from area names. They are not claims. The hdr slice works on crates at the gate limit, so it has no file-level scope, and it is not hinted on any file.
3. The RUNNING class is empty because the running slices name crates, not files. A slice that edits a free file not named in its RESULT would not be seen here. Refresh the list before any package is dispatched.
4. The signature check reads whole files. It is a heuristic: a file can match it without being generated, and a generated file can lack the signature.
5. Census 4 was taken on a different tree (2026-10-16 about 23:00). The census 4 class counts are its own, and the census 5 class counts are not a like-for-like re-run of census 4.
6. ohr and ohh are in the brief, but no RESULT, tool folder or FLEET note names them, so they are not hinted on any file.
