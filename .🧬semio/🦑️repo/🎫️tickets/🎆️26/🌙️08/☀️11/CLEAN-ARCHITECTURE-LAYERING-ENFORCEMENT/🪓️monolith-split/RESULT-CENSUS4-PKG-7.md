# 🪓 Result CENSUS4-PKG-7 (slice `c4p7`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. Tools: `F/c4p7-tools/`. Logs: `F/🗑️generated/c4p7-*`. Scratch (sandboxes, stage): the session scratchpad `c4p7-work/`.

## 0. Verdict

DONE. All three target files are split and installed (process3d unit test 1,294 -> 134 + 5 children, lowpoly editor 2,026 -> 305 + 5, forms editor 1,440 -> 352 + 6; largest file 590 lines, limit 1,000). Every gate is green or unchanged against its baseline (section 3, section 5). The 25 new process3d tool-job rows the coordinator asked about are fixed (section 4). Nothing pending.

## 1. Scope

| File | Lines | Disposition |
|---|---:|---|
| `process3d` editor `🧪️tests/🔬️unit/🦀️.rs` | 1,294 | split: root 134 + 5 children (210 / 462 / 98 / 269 / 190) |
| `lowpoly` editor `🦀️.rs` | 2,026 | split: root 305 + 5 children (189 / 590 / 282 / 404 / 293) |
| `forms` editor `🦀️.rs` | 1,440 | split: root 352 + 6 children (95 / 137 / 272 / 140 / 322 / 185) |
| layout `⚙️engine/🎬️scene/🦀️.rs` (1,428), `📤️export/🔣️json/🦀️.rs` (1,897), `📤️export/🦀️.rs` (2,524), `📤️export/🧪️tests/🔬️unit/🦀️.rs` (1,316), `✏️editor/🦀️.rs` (1,999) | 9,164 | skipped: all five are in the edited list of `RESULT-LAYOUT-DRAWING-DECOUPLE.md` (slice `ldd`); hand-off |

Not in this slice: the process3d editor lib root `✏️editor/🦀️.rs` (1,947 lines) was split by slice `c4p9` (`RESULT-CENSUS4-PKG-9.md`), not by `c4p7`. Only the process3d editor **unit test** file is mine.

## 2. Recovery after the two session restarts (00:00 and 00:18)

- Restart 1 (00:12 note): the three target files were byte-identical to `c4p7-tools/orig/`, nothing installed. Only repo edit before it: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts` (`readFileSync` of the editor root replaced by `readModuleTreeSource`).
- Between 00:12 and 00:18 process3d and forms were installed (stage `scratchpad/c4p7-work/stage/{process3d,forms}`); lowpoly was staged (00:12) but **not installed** when the second restart hit.
- Restart 2 recovery (this write): compared every live file with its stage file.
  - forms: 7 of 7 files identical. process3d: 5 of 6 identical; `🧵️command-surface/🦀️.rs` differs by an edit of the peer slice `rsd` (RETAINED-STEP-TEST-DRIFT, 00:16: `UNBOUNDED_STEP_GRANT`, `StepBudget::new` 3 args, `StepContext::new` with `RetainedCloneProgress`). That edit is theirs and kept.
  - lowpoly: root identical to the original (`cmp`), no child folder, no `*.pfa-tmp` leftover. Nothing half-applied, so the install was simply finished: `pfa.py lossless lowpoly` OK (regions 10 -> 15 / 15, only doc / path / mod / re-export / marker lines added), `pfa.py paths lowpoly` max absolute path 197 bytes (limit 240), oracle on the stage clean (below), then `pfa.py install lowpoly` ("installed lowpoly"; refuses unless the root equals the original).
- Stale artefacts: `🗑️generated/c4p7-checkapplied-lowpoly-after.log` is an invalid first attempt (wrong package dir, ENOENT); superseded by the `after2` logs.

## 3. Installed result and evidence

| Check | Command (tools in `F/c4p7-tools/`) | Result |
|---|---|---|
| Losslessness (non-blank line multiset, regions) | `pfa.py lossless <tag>` | OK for lowpoly (re-run this session); process3d and forms were OK when installed |
| Typeck differential oracle on the live tree | `PFA_LIVE=1 oracle.py <tag> lib,test[,p2,p2test]` (`c4p7-oracle-*-live2.log`) | lowpoly lib/test/wasm32-wasip2/p2test: errors 679/682/679/682 = 679/682/679/682, 0 only-new, CLEAN. forms lib/test: 2044/3588 = 2044/3588, 7 = 7 warnings, CLEAN |
| process3d oracle | `oracle.py process3d lib,test` (stage overlay, `c4p7-oracle-process3d-stage2.log`) | CLEAN: 502 -> 501 and 599 -> 598 errors, the one removed error is the old `super::` E0433 of the unsplit file. The live-tree run (`...-process3d-live2.log`) shows 5 extra `E0433 semio_framework_job` rows: they are exactly the new `semio_framework_job::...` paths of the peer `rsd` edit in `command-surface` (the sandbox neutralizes extern crates), not the split |
| E0451 privacy scan (blind spot of the oracle) | `rpv-tools/lit.py c4p7-tools/c4p7.json` (`c4p7-lit-live.log`) | 0 candidate sites (106 brace structs, 242 files) |
| Source closure census (budget 32 files / 262,144 bytes / 4,000 lines) | `census.ts` (`c4p7-census-lowpoly-after.txt`) | lowpoly lib 8 files / 164,510 bytes / 2,081 lines, test 9 files: ok. Before: 3 files / 2,039 lines |
| Taxonomy resolution of the new folders | `tax.ts` (live engine `semanticDirectoryKindId`) | all 5 new lowpoly folders resolve to `crate-module-children`, 0 unresolved (forms and process3d folders were checked at install) |
| Tool-job evidence for the three roots | `tooljob-probe.ts` before / after, `diff` of every row, disposition, proof, scan and failure line | identical (the only change is the `source` byte / line lines) |
| Lowpoly script source laws | `lowpoly-script-probe.ts` | identical to before: 48 Migrated, 0 BatchOnlyPendingRewrite, policies agree with Ajv, the same 2 pre-existing failing checks (`copy_from_slice(b"LPC3")`, caller-owned default child payload); not caused by the split |
| process3d unit TS consumer | `bun test 🟦️.ts` in the unit folder | 1 pass. It had never been able to pass: line 8 resolved `../🧫️fixtures/production-envelope-edit` without a trailing slash to `🧫️fixtures/🔣️.json` (ENOENT, untouched by the split). Fixed with the trailing slash; line 13 already read `🏭️production-envelope/🦀️.rs` |
| Lib-wiring ratchet | pin of the lowpoly editor root lowered 2027 -> 306 in `📏️lib-wiring-ratchet/🔣️.json`; `bun test` of the law | the lowpoly root and my files are not in the breach list. The law still fails on rows that are not mine: lowpoly `⚙️engine` 495 -> 496, procedural `generation3d` mesh-edit 153 -> 154, layout export json 1879 -> 1898, and 7 stale pins (norm, procedural, puzzle 3d). The process3d editor root pin (1948, now 479) belongs to `c4p9` |

## 4. Reader probe (coordinator question: tool-job failures 138 -> 163 for process3d)

Cause found, and it was not the unit-test split: `c4p9` split the process3d editor lib root, so `Process3dBoundedCommandJobFactory` and its reducer rows now live in `🧵️retained-commands` and `📬️store-preparation`, but `toolJobPluginProductionSources` reads a root through the module tree only if the root is in `POLICY_SPLIT_SOURCE_ROOTS` (`🧰️framework/.../📖️source-access/🟦️.ts`). lowpoly and forms were registered, process3d was not, so the reader saw only the 478-line root and reported 75 rows as forged: 25 each of "forged bounded reducer factory or compiler witness", "owner/file authority" and "document schema" (total 988 -> 1063; with the other probe's smaller sample 138 -> 163 is the +25 per kind).

Fix: registered `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` in `POLICY_SPLIT_SOURCE_ROOTS` (one line, after the forms root). Verified with `c4p7-tools/failures-dump.ts`: total failures **1063 -> 988**, and `Bun.hash` of the sorted failure list **3458757958838400468**, equal to `failuresHash` of `c4p5-probe-before.json` (byte-identical baseline). The 25 process3d rows that remain (of the 988) are in the baseline.

## 5. Layout gate (`new crates --check-applied --base=f856dff0813`)

Script: `c4p7-tools/checkapplied.sh after2 <tag:plugin-artifact-dir>...` with `LC_ALL=en_US.UTF-8`, logs `🗑️generated/c4p7-checkapplied-<tag>-after2.log`. Before: process3d exit 0 (20 crates, 0 only-combined), forms exit 0 (0 crates, 0 only-combined); lowpoly crashed twice under machine load with `EMFILE`; the earlier `pgr` run of 2026-10-16 20:47 gave exit 0.

After (live tree, all `exit 0`, `onlyCombined` 0 for native and `--test`, `admission` empty, `siblings` empty):

| Package | crates / base | native base -> combined | `--test` base -> combined |
|---|---|---|---|
| lowpoly | 21 / 20 (same 21 / 20 as the pre-split `pgr` run) | 2683 -> 2700 | 2957 -> 2976 |
| process3d | 20 / 20 | unchanged vs before, 0 only-combined | 0 only-combined |
| forms | 0 / 0 | 0 only-combined | 0 only-combined |

Module owners (`pch-tools/modowners.ts`, `c4p7-modowners-after.tsv`): 18 files reached by more than one package, 0 of them in lowpoly, forms or process3d. Before 15; the 3 additional rows are `os/.../plugin/tests/mutation-fixtures-{job-close,no-state,wire}`, created by a peer slice. (The first scan attempt hung on a git call and was killed; the second finished.)

## 6. Files

Created: `lowpoly` editor children `🌉️args-bridge`, `🧵️retained-commands`, `📬️store-preparation`, `🕹️play-app`, `📋️manifest`; forms editor children `🧩️contributions`, `🌉️args-bridge`, `🧵️retained-commands`, `📬️store-preparation`, `🕹️play-app`, `📋️manifest`; process3d unit children `🏭️production-envelope`, `🧵️command-surface`, `📋️manifest-and-io`, `🎮️interaction-behavior`, `🧩️contributions-and-examples` (each `🦀️.rs`, all under the editor / unit folders named in section 1).
Rewritten: the three roots of section 1, `process3d/.../🧪️tests/🔬️unit/🟦️.ts`, `💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts`, `📖️source-access/🟦️.ts` (process3d split root registered), `📏️lib-wiring-ratchet/🔣️.json` (lowpoly pin).
Tools: `F/c4p7-tools/` (new this session: `c4p7.json`, `failures-dump.ts`).
