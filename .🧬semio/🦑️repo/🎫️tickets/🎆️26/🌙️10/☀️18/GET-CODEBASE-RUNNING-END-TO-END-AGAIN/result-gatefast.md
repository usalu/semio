# result-gatefast (wave 1027b, lane gatefast, mirror prefix gf)

UPDATE 18:55 (G45 dash1 RED): the refusal is NOT caused by `scopedcut` — it comes from peeredit2 `providers` (G44) meeting an old slice gap; `g45b-dash` with scopedcut reverted will be RED the same way. Evidence (read-only, live generation 669, the real `sliceClosureV1`/`sliceViewV1`): `bun C/gatefast-tools/pinprobe.mjs /Users/ueli/Documents/semio @semio-tech/repo-dashboard-rs:build @semio-tech/repo-dashboard-rs:compile` → outer `build` slice 66 projects, nested `compile` slice 97; the 31 extra are exactly the refused list and every one enters through `@semio-tech/repo-script-providers` (the `scriptProvider:*` references in compile's named inputs) → `framework-process` → `framework` → …. The pin covers `result.names` = the outer slice (`🔌️load` `host.sealed`), whatever the cutoff asks, so scopedcut cannot change it. Cause: `sliceClosureV1` follows inputs, `dependsOn` and edges but not the declared nested runs (`metadata.semio.execution` kind `orchestration`, `children`; 16 targets of repo-dashboard-rs declare them, e.g. `build` → `compile`); before G44 the compile closure happened to lie inside the build closure.
FOR coordinator (batch `orchslice`, 5 files, base = live 18:50, independent of scopedcut): apply `python3 -I C/gatefast-tools/staged/orchslice/stage.py check --root /Users/ueli/Documents/semio && python3 -I C/gatefast-tools/staged/orchslice/stage.py apply --root /Users/ueli/Documents/semio` · verify `sh C/gatefast-tools/staged/orchslice/verify.sh /Users/ueli/Documents/semio` → `orchslice VERIFIED` (190/0 + Python corpus `failures: []`) · revert `… stage.py revert --root /Users/ueli/Documents/semio` · drift → `python3 -I C/gatefast-tools/build_orchslice.py /Users/ueli/Documents/semio`. Fix: `🕰️generations/🔌️load/🟨️.mjs` — `sliceViewV1` keeps a target's declared orchestration children, `sliceClosureV1` activates them as tasks (an undeclared child project refuses `GENERATION_UNKNOWN_OWNER`), so the outer verification and the pin cover every nested run; `.d.mts`; laws: pin law (outer dashboard-shaped slice → nested compile closure inside it → `generationPinOpenV1` empty with the real pin codec; RED on base), 2 corpus scenarios in the load fixture + the independent Python oracle extended (base module: 5 fails; batch: 190/0 on mirror gfcut, load/provider-closures/pin/pin-nested suites; base suites 185/0), scan.ts 2/0, tsc strict clean, stage mode law OK, guard clean. Real generation 669 with the batch's loader (`PINPROBE_STORE=/Users/ueli/Documents/semio/.cache/b/g/m/generations bun C/gatefast-tools/pinprobe.mjs <mirror> …`): dashboard build slice 66 → 97 ⊇ compile 97 (extra 0); course build 199 = unchanged.
FOR coordinator (order + proof): land `orchslice` (then `scopedcut` again if you want it; both laws compose, no shared file), verdict `repo-dashboard-rs:build` twice (dash1 must print `generation N pinned by the invoking build serves this command (97 projects)`, exit 0) + course + course:test.

DONE 15:25. One product batch staged and mirror-verified (`scopedcut`), one ticket tool (`gate-g-tools/verdict.py`) with its law. Nothing live touched, no official run started, mirror `gfcut` dropped, no process of mine running.

FOR coordinator (batch `scopedcut`, 8 files, base = live 15:20): apply `python3 -I C/gatefast-tools/staged/scopedcut/stage.py check --root /Users/ueli/Documents/semio && python3 -I C/gatefast-tools/staged/scopedcut/stage.py apply --root /Users/ueli/Documents/semio` · verify `sh C/gatefast-tools/staged/scopedcut/verify.sh /Users/ueli/Documents/semio` → `scopedcut VERIFIED` (173/0) · revert `python3 -I C/gatefast-tools/staged/scopedcut/stage.py revert --root /Users/ueli/Documents/semio` · drift (exit 1/2) → `python3 -I C/gatefast-tools/build_scopedcut.py /Users/ueli/Documents/semio` re-cuts from anchored edits on the live bytes and names any dead anchor.
FOR coordinator (product proof of scopedcut, with the next group): in `group-seq` order the course run prints `graph: N derived results asked again …` with N ≤ ~260 (was 549-652), course:test and dashboard N ≤ ~100 (were 412 / 637), and the questions asked over the whole group drop from 1,523 (G42) to ≈ the 670 distinct ones. Bounds before the run: `bun C/gatefast-tools/scope.mjs /Users/ueli/Documents/semio <memo plan files>` (read-only, 0.9 s, 0.6 GB).
FOR coordinator (tool): `python3 -I C/gate-g-tools/verdict.py g44 group` runs a whole group verdict (`… - list` shows the recipes, `… g44 group --plan` the expanded commands, `group-seq` = today's order without warm-up/parallel stage, `course`/`print`/`dash` = partial). Same `run/g44-<suffix>.{log,json}` as product.py plus exact wall (no 5 s grain), RSS every 0.5 s, macOS physical footprint every 5 s (buildmem's `top` method), phase split; `run/g44.verdict.json` = GREEN/RED, stage walls, `warm_effective`. Law `python3 -I C/gate-g-tools/tests/test_verdict.py` 3/3 (stand-in command, no official build). Old logs: `python3 -I C/gate-g-tools/phases.py C/gate-g-tools/run/g42-*.log`.

## 1. Where a group verdict's time goes (G40-G43 logs, `gate-g-tools/phases.py`)

| group | wall (sum of runs) | closure verification | inference | Bazel plan + build | rest |
|---|---|---|---|---|---|
| G40 | 1,437 s | 490 s (34 %) | 260 s | 527 s | 160 s |
| G41 | 1,788 s | 950 s (53 %) | 260 s | 458 s | 120 s |
| G42 | 2,123 s | 950 s (45 %) | 290 s | 713 s | 170 s |
| G43 | 2,540 s (load 34-40) | 730 s (29 %) | 370 s | 1,140 s | 300 s |

`closures` = the `Verifying input closures: k of N` phase (cutoff re-asks derived questions), present in every refreshing run since g40-ctest:

| run | wall | closures (asked / changed) | refreshed | inference |
|---|---|---|---|---|
| g41-course | 452 s | 280 s (549 / 547) | 171 | 80 s |
| g41-dash1 | 472 s | 300 s (637 / 629) | 66 | 20 s |
| g42-course | 697 s | 380 s (652 / 631) | 198 | 100 s |
| g42-ctest | 384 s | 280 s (412 / 412) | 16 | 20 s |
| g42-print1 | 599 s | 230 s (400 / 400) | 355 | 160 s |
| g43-print1 | 971 s | 490 s (580 / 197) | 163 | 240 s |

## 2. Product reasons (evidence) and what is done about them

1. **STAGED `scopedcut` — a scoped verification re-asks every derived question of the workspace.** `plan()` selects the fact classes naming a slice owner; shared library rows (e.g. `lstat ⚡️caching/🦀️cargo/📜️script.ts`, 3,707 holders) sit in classes with all 670 pseudo owners, so one stale shared row makes all 670 questions suspects, though only the slice's answers can dirty anything (`dirty` is filtered by `only`). A question that CHANGED stays stale (its out-of-slice holders are not re-inferred), so every later run asks it again: G42 asked 652 + 412 + 400 + 59 = 1,523 questions for 670 distinct ones. Live generation 658 (`gatefast-tools/scope.mjs`, the memo's own plan files, the planner's own `selectFactClassesV1`/`scopeWithDerivedV1`): course slice (214 owners) reaches 670 suspects, **254** have a holder in the slice; dashboard (66) 670 → **95**; print (550) 670 → 584.
   Fix: `suspectsWithinV1(index,suspects,names)` in `✂️cutoff` (pseudo owners whose derived fact a scoped project holds); `plan()` settles only those when `only` is given; the others keep their recorded stale support and are asked by the verification that selects one of their holders (sound: deferring a check never validates anything). Whole-generation verification unchanged.
   Laws: store law (real `createGenerationStoreV1`, real walks, Python walker): scoped plan of p2 asks 1 question (base: **2**, RED on base), whole plan still asks 2 (deferred stays stale), a changed shared answer dirties only the scoped holder, the deferred holders p1/p4 are refreshed by the next whole verification; cutoff law: every one of the 129 worlds under a seeded half scope equals the extended Python model (`scope` field, schema-first) and asks only questions a scoped project holds; 2 hand-written scoped worlds. Mirror `gfcut`: verify.sh **173/0** (base 169/0), base module + new laws: 4 fail (the store law `Expected: 1 Received: 2`); neighbour suites (unborn-root, world, plan-once, memo edit/slice/reseal/carry, generation-protocol) 67/0; `scan.ts` 4/0; `tsc --strict` on both laws clean (planted error caught); stage mode law OK; guard clean (one run saw the peer landing of farmid files, mirror untouched).
2. **Farm never used** — every refresh g11-g43 printed `worker processes were not used because they differ from this process (identity differs)`, 0 projects adopted (inference 12-15 % of a group). Lane farmid owns it (`C/result-farmid.md`).
3. **print1 refreshes 355 projects after every landing** (g40-g42): the cargo driver / lock-owner walks are held by 506-516 Rust/Go crates each (`gatefast-tools/questions.mjs`: 670 derived questions, 545 distinct walks, median support 22,438 rows). That is peeredit2 batch 2 `tools`; product walks reaching the library through provider ports are peeredit2 batch 1 `providers`. Not duplicated.
4. **NOT staged — permission-only membership changes** (peeredit §4.3): every `fix-modes` landing (G36 213 files, G39 21, G43 31) is a membership change because frontier members carry the full `st_mode`. A fix must mask type + execute bits at every producer and comparer: `🌿️frontier-walk/🟨️.mjs` (walk/advance), `🕰️generations/🟨️.mjs` (`createFrontierV1`, `frontierHoldsV1`, `diffFrontiersV1`), `🧠️memo/🟦️.ts` (`memoFrontierHoldsV1`), and the resident path that builds frontiers without `createFrontierV1` (`🏡️resident/🕰️graph-hooks/🟨️.mjs`, `🗃️state/🟨️.mjs`, Rust watcher `🔭️watch/🦀️rust/🦀️.rs` scan output). 7 files incl. a watcher rebuild; with xplat5's mode-keeping stage template the trigger is rarer now, so it is lower value than its size → for a graph/resident owner.
5. **NOT staged — every test owner holds the shared `testRows`** (`🕸️graph/🟦️.ts` part discovery: rows of `discoverTestBuildDependenciesV1` over ALL tests + `withholdDanglingTestProjectsV1` digests), i.e. raw `file`/`analysis` rows of ~1,220 library files (`peeredit-tools/holders.mjs` on gen 658: 264 test owners, 2,424 analysis + 1,221 file rows each). A body-only edit of any of those files dirties all test owners. Not in the gate's slices (0 `test-` owners among the inferred names of g40-ctest/g41-course/g42-course/g43-print1), so no gate time; costs every `repo test`. Fix direction: capture test edges per test owner.

## 3. Verdict recipe `group` (`verdict.py <label> group`)

| stage | runs | why |
|---|---|---|
| warm | `run-many --targets=build,test --projects=@teaching/architecture-course,@semio-tech/print-rs,@semio-tech/repo-dashboard-rs --dry-run` | ONE graph refresh for the union of the slices (one closure pass, one inference batch); executes nothing (dry-run uses the default `fresh` graph policy) |
| first (parallel) | course build ‖ print-rs:test ‖ dashboard build | graph already clean for all three (no builder custody, shared reads), separate Bazel slots, capacity admission sizes them; stage guard 90 % of RAM stops only the largest run (RED, rerun it alone) |
| second | course:test, hot1, hot2, hot3 only if hot2 not `served from the memo`, print2, print3 only if print2 not served, dash2 | same coverage as hot x3-x5, print x3, dash x2 |
| edits (alone) | course one-line edit + byte-exact revert, print one-line edit + revert | edit probes mutate live files, never beside another run |

Removed redundancy: course build and course:test no longer refresh separately (both covered by the warm), print1 no longer pays its refresh serially before anything else, the 3-4 repeated closure passes become one, the three long builds overlap. `warm_effective:false` in the verdict json means a first-stage run still refreshed (then use `group-seq`).

## 4. Expected minutes per group

| product state | today's sequential chain | `group` recipe |
|---|---|---|
| today | 24-42 min (G40-G43) | warm 12-14 + first 3-5 + second 3 + edits 3-4 = **21-26 min** |
| + `scopedcut` | 17-30 min (closures ~950 s → ~420 s in G42 terms) | **20-25 min** (the warm already asks each question once) |
| + peeredit2 providers + tools | 12-16 min | warm 2-4 + 3-5 + 3 + 3-4 = **11-16 min** |
| + farmid | 11-14 min | **10-14 min** (big landings: inference ÷ 3-4) |

Files: `C/gate-g-tools/{verdict.py,phases.py,tests/test_verdict.py}`, `C/gatefast-tools/{build_scopedcut.py,verify_scopedcut.sh,neighbours_scopedcut.sh,scope.mjs,questions.mjs,staged/scopedcut/}`; evidence `C/🗑️generated/gatefast/` (red-/green-scopedcut.txt, holders-658.txt, inferred-name lists).
