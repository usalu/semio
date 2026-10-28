# result-rustedit (lane rustedit, 10-27): one-line Rust edit → print-rs:test < 2 min — cost anatomy + fix

## UPDATE 10-27 08:52 (round 2): anatomy of the g28p one-line edit (369 s) vs its revert (95.5 s), after universe + rustfast landed

FOR coordinator: batch ready `C/rustedit-tools/staged/testmemo` (3 edited files + 1 adapted law + 1 new law): `python3 -I C/rustedit-tools/staged/testmemo/stage.py check|apply|revert [--root DIR]` (exit 2 = drift), laws `sh C/rustedit-tools/staged/testmemo/verify.sh` (9 tests, 3 s). Base hashes = live at 09:0x. Rebuild tool `C/rustedit-tools/testmemo/edit.py <root> check|apply` (anchored, idempotent). NOTHING applied live, NO official run.
FOR coordinator (after landing): also run the existing law `🏃️process/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts` through the repo test harness (it needs an issued Script invocation; under plain `bun test` it is red on base and with the batch: `Original parent Script process invocation required`); the batch only adds two stub bindings to it. Measure: `python3 -I C/gate-g-tools/edit-probe.py` edit + revert as for g28p, twice in a row (the first run writes the admission memos, the second must show ~375 `Source admitted` rows within ~3 s instead of 45 s, and `find .🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/a -newer <start>` non-empty).
FOR bazel planner owner (graphlat/plan-once2): every content edit of a fingerprinted source replans (edit 50 s `planning again: S …/📓️print/🦀️.rs changed`, revert 40 s `l … changed`). The witnessed planner reads each input file THREE times: `bazelSourceClassesV1` (lstat → `l`), `captureBazelSourceSnapshotV1` (stat + clone copy → `S`, `f`) and `taskFingerprint` → `bazelSourceIdentityV1` (`l`,`S`,`f`) (dumped witness rows of the plan-reuse fixture, `C/rustedit-tools/refresh/`). Design: the planner records per label the fingerprint head + hashed path list and the snapshot file list; the PUBLISHING parent captures the snapshot (it already reuses unchanged copies via `snapshot.json`) and computes `<label>.input` after a fresh plan and after a reused one; the classes become a parent-verified type check. My draft `C/rustedit-tools/refresh/edit.py` moves only the fingerprint part (oracle still red for "content of an input" because of the snapshot + classes reads) — NOT staged. Saves 40-50 s per edit and per revert, for every product.
FOR graph owner: a permanent stale loop: `@semio-tech/framework-renderer-wgpu` + `@semio-tech/plugin-registry` are refreshed in EVERY print run (g27-print2 25 s, g28-print1/2, g28p-edit 52 s, g28p-revert 35 s; generations 612, 613, 614 in a row) with the same `18 stale facts (analysis 18)`: `the source analysis of …/🧊️wgpu/🎬️renderer-boot/🟦️.ts changed` (16 wgpu files + `🔌️plugin/📇️registry/🟦️.ts`), although those files are untouched since 10-16/10-19. `sourceAnalysisFactHoldsV1` (`👁️observation/🟨️.mjs:322`) fails for them after every publish: provider revision, nested observations or a volatile nested input; needs the fact rows of generation 614 (packed `x/<factIndex>.facts.br`) to name it.
FOR cargo-preparation owner: the `synchronize` child is 42 s of the edit action (23 s on 10/26): the receipt `<preparation>/<sha>.sync.json` matches (no `cargo update/fetch`), the cost is a second nested `repo` process + compiler store + owner preparation + tool identity. A parent-side receipt check before spawning would skip it.
FOR quietfc/capacity: `semio_framework_print` compiled non-incrementally in 69.7 s under `admitted run with 1 of 8 job(s)`.

### Edit run g28p-edit (369 s) — seconds from run start; test action from `actions/cas/2a/2a9f3e…` (its kept test.log), unit events lane p32deb6
| from-to | s | phase | evidence |
|---|---|---|---|
| 0-30 | 30 | door boot, capacity, graph check (`the memo cannot serve this request (light)`) | |
| 30-70 | 40 (52 published) | graph refresh of 2 owners the edit did not touch (stale loop above) | `refreshing 2 projects (generation 612)` |
| 70-120 | 50 | Bazel REPLAN caused by the edit | `planning again: S …/📓️print/🦀️.rs changed` |
| 120-130 | 10 | Bazel analysis + dispatch | |
| test action | 236 | | `PASSED in 236.4s` |
| +0-30 | 30 | action child, snapshot-restore, native owner-command, nested `repo …/📜️script.ts test` boot | |
| +30-72 | 42 | cargo preparation `synchronize` child (receipt hit) | `cargo:repository-tests remains active on …/bun` |
| +72-80 | 8 | workspace selection, unit metadata, unit graph | |
| +80-125 | 45 | foreign inventory + source admission of 375 units with NO memo (rustfast wired its memos only into `bun repo cargo`) | 375 `Source admitted`, no new memo under `u/a` |
| +125-140 | 15 | cargo start, writer lease, scheduler | |
| +140-228 | 88 | `cargo test --no-run`: `semio_framework_print` MISS 69.7 s (non-incremental, 1 job), `semio_framework_print_tests` incremental 5.4 s (incl. link), 295 unit-cache hits 7.9 s | `Finished … in 1m 28s` |
| +228-236 | 8 | nextest 39 tests 1.7 s, settle | |
### Revert run g28p-revert (95.5 s)
boot + graph check 20, the same 2-owner refresh 35 s (gen 613), replan 40 s (`l …/🦀️.rs changed`), Bazel `2 disk cache hit` 0.36 s.

### Ranking (edit run, projected savings)
| fix | saves | status |
|---|---|---|
| testmemo: repository tests use the foreign digest book + admission memo | ~40 s (45 → ~3; also every later print test action) | STAGED + mirror-verified |
| replan on content edit: move snapshot capture + fingerprints to the publisher | 40-50 s edit, 40 s revert, all products | design + draft, FOR planner owner |
| 2-owner graph stale loop | 25-52 s, every print run | FOR graph owner |
| synchronize child fast path | ~20-40 s | FOR cargo-preparation owner |
| compile with the run's jobs instead of 1 | part of 70 s | FOR quietfc |
With all five the edit lands near 369 − 40 − 45 − 40 − 30 − ~35 ≈ 180 s; the remaining gap to 120 s is the two nested boots (30 + 30 s) and the 88 s cargo phase.

### Batch testmemo (laws)
- `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`: `repositoryCargoPlanningPortV1(store)` (planning port with `foreignDigestMemoV1(unitCacheRootV1(store.root))`) and `repositoryCargoSourceMemoV1(store)` (`unitAdmissionMemoV1(…, store.toolchain.digest)`), used by the launcher itself (one composition).
- `🏃️process/🦀️cargo/🎟️admission/🟦️.ts` `execute` (all repository cargo tests and lint): passes `planningPort` + `sourceMemo` from the store.
- `🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`: `CargoTestAdmissionV1.sourceMemo`, forwarded as the memo argument of `admitCargoSourceBoundariesV1`.
- New law `🎟️admission/🧪️tests/🗃️memos/🟦️.ts` (3 tests): a second memo instance recalls what the first remembered under the store's unit cache, a toolchain change and a source edit miss; two planning-port inventories of a real foreign crate are equal and the first writes the digest book under the unit cache; AST wiring of the three call sites (launcher composes once). RED on base (module exports missing), GREEN with the batch; rustfast's foreign (5) + key-stability laws stay green (9/9 in the sealed mirror `reedit`, guard clean); tsc strict over the 4 TS files and closure: 0 errors (sensitivity shown by an earlier TS2345 in the new law). Apply → revert (mirror diff 0) → apply → second apply no-op.

## Round 1 (universe batch, LANDED and proven: g27-print2 hot 65.5 s, `plan reused: 209691 observations`)

FOR coordinator: batch ready `C/rustedit-tools/staged/universe` (1 edited library file `🏗️build/🔧️tools/🏗️bazel/📜️script.ts` + 7 new files: module `🏗️bazel/🌌️universe` with schema/fixture/law, plan-reuse law `♻️plan-reuse/🧪️tests/🌌️whole`). `python3 -I C/rustedit-tools/staged/universe/stage.py check|apply|revert [--root DIR]` (exit 2 = drift; revert removes the new files and their emptied directories), laws `sh C/rustedit-tools/staged/universe/verify.sh` (27 tests, ~45 s; `VERIFY_ROOT=<root>`). Base hashes re-checked against live 07:3x: all `base`. NOTHING applied live, NO official run, NO edit ON.
FOR coordinator (measurement after landing, one at a time in a gate gap): `python3 -I C/gate-g-tools/product.py ue1-print @semio-tech/print-rs:test` twice back to back (expect the first to print `[bazel] plan …recorded`/no `cannot be reused later: … whole repository`, the second `[bazel] plan reused: N observations`; N ≈ 115k + the rest). Then the edit: append one `//` comment line to `🧰️framework/🛍️products/📓️print/🦀️.rs`, `python3 -I C/gate-g-tools/product.py ue2-print @semio-tech/print-rs:test`, revert the line byte-exact, `python3 -I C/gate-g-tools/product.py ue3-print @semio-tech/print-rs:test`. Read per-phase seconds with `python3 -I C/rustedit-tools/anatomy.py <s1 testlogs>/t_d71aae306395ac0f/test.log` and unit events with `python3 -I C/rustedit-g-tools/events.py <startMs> <endMs> --lane p32deb6`. Same for `@semio-tech/repo-dashboard-rs:build` (not affected by this batch: its plan has no root-anchored pattern; its replans were the slot eviction that dashplan fixed 07:24).
FOR graph/bench owners: `🧪️tests/📥️inventory/🟦️.ts` (Bazel inventory law) is RED on the live base: it evaluates the text of `bazelFilesystemSourceFilesV1` with only `dirname`/`slash`/... bound, but the function now uses `BAZEL_SOURCE_EXCLUDED_V1` (`ReferenceError: BAZEL_SOURCE_EXCLUDED_V1 is not defined`). Not caused by this batch (red on base and with it).
FOR cargo-preparation / boot-g: 23 s of every print test action is a second nested `repo` process (`🛠️preparation/📜️script.ts synchronize`), see anatomy row 17-40.

STATUS (final): anatomy done from logs; fix staged and verified on mirror `reuni` (RED on base, GREEN with the batch, existing plan-reuse + ignore-sources suites green, tsc strict clean).

Tools `C/rustedit-tools/` (`anatomy.py` streams a Bazel test.log into a 10 s timeline, `dircount.py`/`dirtop.py` directory census, `universe/walk-cost.ts` pruned-walk cost, `universe/edit-script.py <root> check|apply` = the anchored, idempotent script.ts edit used to build the batch (rebase: rerun it on a fresh mirror and `stage-flat.py make` again), `staged/universe/`). Mirror `reuni` dropped after verification (recreate: `python3 -I C/mirror-tools/mirror.py create reuni --batch C/rustedit-tools/staged/universe`). Unit-cache events read with `C/rustedit-g-tools/events.py`.

## Evidence used
- Run logs `C/gate-g-tools/run/g6-print.log` (23:03, 422 s), `g8-print.log` (23:42, 562 s), `g11-print.log` (00:03, 508 s), all `print-rs:test`, none with a print edit (each followed a library landing).
- Bazel keeps the test action's whole stdout/stderr: `.cache/b/p/m/bazel/outputs/server/s1/execroot/_main/bazel-out/darwin_arm64-fastbuild/testlogs/t_d71aae306395ac0f/test.log` (g11, 55k lines, 4.6 MB; earlier runs overwritten).
- Unit-cache events `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/x/*.jsonl`, lane `p32deb6` = print-rs:test, `pea977c` = print-rs:build (different lanes, different build dirs).
- Bazel slot package `.cache/b/p/m/bazel/workspaces/s1/0000000000000000/t_*.json` (task inputs).
- rustedit-g's e1 (one-line edit, print-rs:build, 2,637 s) and rustfast-g's split of the same action.

## Cost anatomy: print-rs:test (seconds, wall)
| phase | g6 | g8 | g11 | what |
|---|---|---|---|---|
| capacity | ~0 (8 wait rows inside graph) | 44 wait rows | 0 | |
| graph prepare + refresh | 200 (37 projects) | 410 (521) | 340 (352) | caused by library landings, not by print |
| tool-setup (captured planner + publish + Bazel analysis, 35k targets) | 30 | 40 | 30 | REPLAN every run: g6/g8 `a file read by the planner was modified moments before` (racy, fixed 01:20), g11 `the source universe is the whole repository` (see Finding 1) |
| 3 dependency actions (RepoTestLauncher, framework-schema:generate, dsl-derive-rs:generate-schema-index) | 10 | 10 | 10 | worker, re-run each time |
| test action (one Bazel action = one nested `repo` + one whole `cargo test` + nextest) | 163 | 95 | 114 | breakdown below |
| total | 422 | 562 | 508 | |

### Inside the test action (g11, 114 s, load ~5; `anatomy.py` timeline)
| s | phase | evidence |
|---|---|---|
| 0-17 | Bazel action child, snapshot-restore, native owner-command, nested `repo …/📜️script.ts test` boot | `[script]` rows, no work rows |
| 17-40 | cargo workspace PREPARATION: a separate `repo` process `…/🛠️preparation/📜️script.ts synchronize --manifest … --package …` (`prepareCargoWorkspaceInvocation`), 23 s holding the bun tool lease | `cargo:repository-tests remains active on …/bun (shared, 23 s)` |
| 40-50 | workspace selection, unit metadata, `cargo build --unit-graph` | `cargo:workspace-selection`, `cargo:unit-metadata`, `cargo:unit-graph` |
| 50-52 | foreign source inventory: 4,130 per-file rows (45 s under load 30-50 in e1) | `cargo:foreign-source` ×4130 |
| 52-85 | source admission of 375 units, 0 memo hits (115 s under load in e1) | `Source admitted` ×380, `Reading sources` ×745 |
| 85-105 | `cargo test --no-run`: 366 units, ALL dirty for Cargo (345 `Compiling` lines, 0 Fresh), served by the unit-cache wrapper: 364 hits (12.9 s summed), 2 incremental of the focus crate `semio_framework_print_tests` (6.4 s); `Finished … in 19.09s`; 5,276 replayed warnings | `Compiling` ×345, unit events |
| 105-110 | nextest: 39 tests in 1.8 s | `Summary [1.772s] 39 tests run` |
| 110-114 | settle | |
Lease-order notices on every run: `cargo:configuration-view` (rank 1) under the store lease (rank 2); `cargo:configuration-publication` (rank 2) under `repo:compiler-writer` (rank 7).

### Where a one-line edit in `📓️print/🦀️.rs` lands (crate `semio-framework-print`)
- print-rs:test focuses only the `-p` package `semio-framework-print-tests` (`focusFromCargoArgumentsV1`), so the edited crate is a unit-cache MISS compiled NON-incrementally (`stripIncremental`): g6 (after a peer edit of its closure) `miss semio_framework_print 49.4 s` (load ~5), then `incremental semio_framework_print_tests 3.8 s`, link, nextest 2 s. print-rs:build instead focuses `semio-framework-print` → incremental, 500 s in e1 (load 30-50, `-Zthreads=1`, macro-heavy function) / 233 s cold lane.
- Projected one-line edit at low load, today: capacity 0 + graph ? + tool-setup 30 + deps 10 + test action (ingress 50 + foreign/admission 35 + print 50 + tests 5 + hits 13 + nextest 2 + settle 4 ≈ 160) ≈ 200 s + graph. Under fleet load (e1): 2,637 s.

## Avoidable costs (ranked, with owner)
1. REPLAN EVERY print RUN since keys5 (23:55): Finding 1 below. 30 s quiet / 250 s loaded (e1 tool-setup). Fix: this lane (staging).
2. Foreign inventory + admission memo misses: 35 s quiet / 160 s loaded. Covered by rustfast-g (staged, rebasing).
3. Cargo preparation child (`synchronize`, a second nested `repo` boot): 23 s every action. Not covered. Owner: cargo preparation / boot-g.
4. Nested boot 17 s + unit graph 10 s per action. boot-g.
5. Test action re-runs after every library landing (its fingerprint includes the cargo command closure `commandSources0/1`): 95-163 s per landing; exec-f's OPEN item.
6. Graph refresh after library landings (37-521 projects, 200-410 s): graph lanes; not caused by Rust edits.
7. Edited first-party crate compiled non-incrementally in the test lane (49 s) while the build lane compiles it incrementally (500 s loaded): needs a quiet A/B measurement before any policy change (FOR coordinator below).

## Finding 1: print's Bazel plan can never be reused (whole-repository source universe)
`@semio-tech/dsl-derive-rs:generate-schema-index` (a dependency of print-rs:test) carries keys5's precision rule `dsl-schema-index` (landed 23:55): input `{workspaceRoot}/**/🧬️schema/**/*.json` with negations `!{workspaceRoot}/**/{target,node_modules,dist,🗑️generated,📦️packages}/**`, `!{workspaceRoot}/**/.*/**`, … (slot package `t_9a1b254c41f5ec4e.json`). `bazelInputTraversalRootsV1` turns that into the traversal root `""`, and `observeBazelSourceUniverseV1` taints every plan whose roots contain `""`: `[bazel] this plan cannot be reused later: the planner read something that cannot be checked: the source universe is the whole repository` (g11). The read set is genuinely repository-wide (keys5's `schema-index/read-set.py`), so narrowing the key would under-declare it.
Census (`dirtop.py`, warm): a walk with only the Bazel exclusions visits 497,750 directories (91 s), but with the rule's own negations (dot dirs, `target`, `node_modules`, `dist`, `🗑️generated`, `📦️packages`, `⚡️cache`, …) 115,524 directories (6.9 s; 84,942 of them under `✏️s/🔌️plugins`). The course plan already reuses 73,500 observations in 0.6-1.0 s.

Measured for print's plan (`rustedit-tools/universe/walk-cost.ts`, live repo, load 13-17): the schema-index task's negations prune 22 names + dot directories; the pruned walk visits 115,523 directories in 9.3-9.6 s at 93 MB RSS. Without pruning 497,750 directories, 79-115 s.

## Design (batch `universe`)
- New module `🏗️bazel/🌌️universe/🟦️.ts` (schema `🧬️schema/🔣️.json`): `bazelSourceUniverseV1(groups)` derives from the expanded inputs of every cacheable task of the plan (one string array per task, `!` = negation) `{roots, whole}`: `roots` = minimal traversal roots of all non-root-anchored patterns (identical to `bazelInputTraversalRootsV1`, proven by a law over the inventory corpus); `whole` = `null` (no root-anchored pattern), `unbounded` (some root-anchored pattern's task has no pruning negation) or `{names, dot}` = the directory names every root-anchored pattern's OWN task excludes completely. A negation prunes only in the schema-declared shapes `**/<name>/**`, `**/{a,b}/**`, `**/.*/**`, each name confirmed by the runtime glob (`GlobV1`) on two policy probes.
- `📜️script.ts`: `generateBazelWorkspaceV1` passes the per-task groups; `sourceFiles` takes the universe; `observeBazelSourceUniverseV1` walks the roots completely (as before), then, when `whole` is bounded, walks the whole checkout skipping the Bazel exclusions plus the pruned names (directories already covered by a complete walk are skipped). `unbounded` keeps the old taint `the source universe is the whole repository`. Git listing observation unchanged (`g` over `[]` = whole tracked listing, 0.17 s). The planner's own source enumeration is unchanged.
- Soundness argument: a file can enter or leave the root-anchored pattern's selection only by appearing/vanishing in a directory that is not below a pruned name (below a pruned name every path is excluded by that task's negation), and every such directory's identity is observed; other tasks' roots are still walked completely.

## Laws (mirror `reuni`, all under `mirror.py run` seal + guard, guard clean)
| law | base | batch |
|---|---|---|
| `♻️plan-reuse/🧪️tests/🌌️whole`: sealed witness for a root-anchored task with pruning negations | RED (`tainted: the source universe is the whole repository`) | GREEN |
| same file, oracle over 11 edits: relevant (content, new doc, removed doc, new deep schema dir, new top-level dir) never hold and change the complete plan; edits below excluded `📦️packages` / dot directories and an unrelated content edit hold and the complete plan (second planner run, byte digest) is identical; conservative (non-matching file in a schema dir, new directories) may replan, plan identical; `git ls-files -co` oracle unchanged for all non-relevant | RED | GREEN |
| same file: a root-anchored pattern without negation stays tainted | GREEN | GREEN (unchanged behaviour) |
| `🌌️universe/🧪️tests`: fixture (10 cases) = schema (Ajv) = implementation; independent Python derivation equal; picomatch 4.0.7 (third party) excludes every probe below each pruned name and the 5 refused negation shapes prune nothing; roots equal `bazelInputTraversalRootsV1` | (module new) | GREEN 4/4, 165 expects |
| existing `♻️plan-reuse/🧪️tests/🟦️.ts` (5) + `🧭️ignore-sources` (15) | GREEN | GREEN |
| `tsc --strict` over the 5 TS files and their 1,115-file closure | | 0 errors (sensitivity proven with an injected TS2322) |
verify.sh run: 27 pass, 0 fail (43.9 s). Apply → revert → `mirror.py diff` shows none of the 8 files → re-apply → idempotent second apply (0 changes).

## Expected effect (not measured live yet, FOR coordinator above)
- print-rs:test hot: replan (captured planner + publish, 30 s quiet, 250 s under fleet load in e1) replaced by plan reuse (course: 73,500 observations verified in 0.6-1.0 s with the change journal). Cost moved to the plan-recording run: +9.5 s walk.
- One-line edit in `📓️print/🦀️.rs` (no new file): the plan stays valid (content is fingerprinted by the publish step, not by the universe), so the edit run also skips the replan.
- Residual risks: (1) another task of print's plan could carry a root-anchored pattern without pruning negations (only the first taint reason is printed; graph-free I found only the schema-index rule: whole-workspace keys are otherwise reported as violations by `workspaceKeys`); the second ue1 run proves it either way. (2) 115k directory rows make `racy` (a directory modified within 2 s of planning) more likely on a busy host, and without the change journal the reuse check re-reads ~115k directories (~9 s).

## Remaining gap to < 2 min (one-line edit, quiet host, projected)
capacity 0 + graph (graph-memo; 0 projects re-inferred for a Rust content edit in e1) + plan reuse ~3 + deps 10 + test action: ingress 17 + preparation 23 + unit graph 10 + foreign/admission 35 (≈3 with rustfast) + print miss 50 + tests 5 + hits 13 + nextest 2 + settle 4 ≈ 140 s now → ≈ 105 s with rustfast + this batch, provided the graph phase stays < 15 s. The next levers: the preparation child (23 s), the nested boot (17 s), and the test-lane policy for the edited first-party crate (49 s non-incremental vs incremental; needs an A/B on a quiet host).
