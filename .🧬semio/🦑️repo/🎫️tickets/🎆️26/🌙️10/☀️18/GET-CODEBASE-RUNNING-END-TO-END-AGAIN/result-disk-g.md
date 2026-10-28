FOR gate-g: batch ready C/disk-g-tools/staged/rootguard (4 files; `sh disk-g-tools/staged/rootguard/apply.sh|check.sh|revert.sh|verify.sh`, anchored edits + 2 new files, base hashes in `bases.json`). Independent of the others; land first.
FOR gate-g: batch ready C/disk-g-tools/staged/unitcache (6 files: unit-cache schema line, gc watcher, memo retention, 3 tests). Apply AFTER jobsrv-g (it replaces the unit-cache schema as a whole file; my anchor line is present in its staged copy, checked). unit-cache is on the "max 3 per group" list.
FOR gate-g: batch ready C/disk-g-tools/staged/sweep (6 files: sweep rule + launcher + runtime phase `budget.sweep` + ledger schema constants + texts). Independent; adds one detached low-priority `repo cache gc --area=…` per 12 h (first one right after landing).
FOR gate-g: batch ready C/disk-g-tools/staged/ledger (12 files: machine budget numbers, 2 native areas, door root parameter, 2 laws, README + build guide EN/DE, control budget 8 MiB, two producer paths). Apply AFTER nxfix core / testmem2 / walk-g / obs-g (they edit README.md and 📓️build-guide.md; mine are anchored). The new law `⚖️law/🌐️native` ends red on THIS machine until the stray `.cache` roots below are deleted (user decision 2); all other laws are green.
FOR gate-g: batch `procstore` (7 files) is staged but `apply` refuses (exit 3) until the user has deleted `🧰️framework/🤖️generated/process` (user decision 1).
FOR keys-g: the `ledger` batch edits the ledger (already in the Nx engine closure per nxret-g): one more re-keying edit; the `cache-retention` prefix request of nxret-g still stands.
FOR all lanes: a test that opens cache storage runs from the checkout root. Started in a module directory (the "graph-free single file" habit) it created a second set of images there: 16.2 GiB below `🧰️framework` right now, 4 of them attached. `rootguard` refuses this (EN + DE); about 77 test files / 157 lines still use `process.cwd()` as the workspace.

# result-disk-g (wave 1026g, executor disk-g): disk usage under budget, one source of truth, closed native root, enforcement that fires

`C` = this ticket. Tools `C/disk-g-tools/` (`stage.py` engine, `batches/*.py` anchored patches, `staged/<batch>/{apply,check,revert,verify}.sh + files + bases.json`, `verify.sh`, `verify-all.sh`, `runtests.sh`), output `C/🗑️generated/disk-g/` (logs, file lists). Report of the audit: `C/audit-disk-1026g.md`. No official run, no gc, nothing deleted, nothing applied live (all verification on `mirror.py` mirrors, guard clean after every run). Not closed: the ticket.

## 1. Why the audit's six symptoms stayed (root causes, with live evidence)

| Symptom (audit) | Root cause found | Fix (batch) |
|---|---|---|
| rust partition 84 % "above the 80 % trigger", no collector stamp | The trigger is 87.5 % (`highWaterShare` in the unit-cache schema); README said 80/60. The stamp is inside the image (`u/gc.stamp` 20:08, `gc.log` `{"state":"running"}`, `fill.jsonl` written every 30 s): a WATCHER is alive all day. A watcher only collected above 87.5 %; between 75 % (low) and 87.5 % nobody ran the share-bounded maintenance (`maintain` fires only below 75 %). Result on the image: lanes `u/l` 10.7 GiB (share 20 % = 9.56), objects `u/o` 13.5 GiB (share 25 % = 11.95). | `unitcache`: in the band the watcher maintains once at its start and every 10 min (`maintenanceIntervalMilliseconds`); README/guide say 87.5/75. |
| unit cache 26.6 GiB against 24 GiB ledger budget | Third part without any bound: the admission memos `u/a` (232 596 files, 2.37 GiB, oldest 143 h, written continuously, grep: no reader of retention, no collector). Shares cover only objects, lanes (incremental lies inside the lanes). | `unitcache`: `pruneAdmissionsV1` (age 7 d, share 3 % of the image = 1.43 GiB, temp files 1 h) in every single-flight collection; law: objects 25 % + lanes 20 % + memos 3 % = 48 % x 48 GiB = 23.0 GiB <= the 24 GiB budget. Read-only dry run on the live dir: 140 982 files / 1.41 GiB would go (RSS 42 MB). |
| `compile-cache-rust` 1.0/0.75 GiB, `.cache/bazel` retired 182 MiB, undeclared dirs (14-day rule), native loose root 243/256 MiB | Collectors exist and use budgets (`planCacheCollectionV1`), but NOTHING starts them: only explicit `repo cache gc` and three free-space triggers (graph, products, tools image). `🧭️prune-stamp.json` (09-13) has no reader any more. | `sweep`: every heavy command reads the size ledger and the last-launch stamp; detached low-priority single-flight `repo cache gc --area=<all collectable areas>,retired,undeclared` every 12 h, only the over-budget areas every 30 min. Pure rule == independent Python on 300 seeded states. |
| nx-tooling 4.4/2 GiB, collector removed 0 | The store grows one generation per 6-7 min under peer edits; retiring old ones needs the whole tools image idle, but 5-6 `bun repo` runs hold it shared at every sample (nxret-g measured 178 of 178 failed try-locks). Owned by nxret-g / lease-g (their FOR line stands). The sweep deliberately does NOT include owner-managed areas. | none here (ownership); budget stays honest. |
| Bazel workspaces/outputs "over" (du) | Not physically over: products image 15.8/24 GiB (67 %). `du` and the size ledger count APFS clones (`cache dedupe` clones on purpose) in full: areas sum to 36 GiB apparent on 15.8 GiB physical. Managed by bazel, reclaimed by the free-space trigger (soft 3 GiB free). | docs (`ledger`): sizes of nested areas are apparent bytes, the physical number is the partition row. |
| control dir 4 MiB / 1 MiB | The 4 MiB `metadata-headroom.bin` (leases metadata schema constant) lives in the control directory by design; the declared 1 MiB could never hold. | `ledger`: `maximumControlBytes` 8 MiB (schema max 16 MiB), fixture headroom 468 713 472 -> 461 373 440, README; law: control >= headroom file + 1 MiB. |
| 15 entries without budget | see 3. | |
| conflicting totals | see 2. | |

## 2. One machine budget (EN + DE in `📓️build-guide.md` "Disk budget", README tables)

Sources before: budget policy 98 GiB (images + control + headroom), machine default 40 GiB, doc 40 GiB, policy tools logical 64 GiB vs capacity 8 GiB, graph 6 (policy) vs 4 (README x4, guide), repository root 24 GiB vs README 20, README control 1 MiB, README watermarks 80/60.
After (all pinned by `🧾️ledger/🧪️tests/📐️model` with an independent Python largest-remainder split):
- `defaultTotalBytes` = sum of the declared image capacities (97.56 GiB) + repository root 24 GiB + native root 256 MiB = 130 795 175 936 B = 121.8 GiB (derived, not chosen: the most a workspace with 6 build + 4 service slots + 16 Cargo lanes may hold). At the default every store may fill exactly its declared capacity; a lower budget shrinks all proportionally. 16 GB laptops with small disks: `SEMIO_CACHE_BUDGET_BYTES=64GiB` (documented). No `🎚️machine-budget.json` needed.
- store weights = share of each store in that total: rust 3940, products 1970, tools 657, dependencies 657, graph 493, diagnostics 164, nx 123, leases 5, loose 1991 basis points (were 4400/2300/800/700/500/300/150/10/840).
- tools logical limit 64 GiB is intentional (test-pinned `maximumLogicalBytes`: hardlinked toolchains count once per name); law: every other partition's logical limit <= capacity, tools <= 8x.
- watermarks 87.5 % / 75 % in README (EN/DE) and law-checked against the schema.

## 3. Declared or eliminated (undeclared / budget-less entries)

| Entry | Verdict |
|---|---|
| `🧰️framework/.cache/b` 8.1 GiB (+ 6 more nested `.cache` roots: `📥️inputs/💾️storage/.cache` 6.0 GiB, `📦️packages/🟦️typescript/.cache` 2.0 GiB, `optional-cache` 67 MiB, `process`, `plugin/registry`, `tools` ~4 MiB each; 4 of their images ATTACHED: `typescript/.cache/b/{l,o}`, `storage/.cache/b/{l,g}`) | Producer: a process whose working directory is a module directory passes `process.cwd()` as the workspace; the partition opener creates `.cache/b/<partition>` below it (evidence: 10-22 15:08 framework, 10-26 08:35 `inputs/storage`, lease sqlite in `framework/.cache/b/l/m/data` touched 18:59; `framework/⚙️configuration/🏃️process` has `workspace: "../.."` = framework). Fix `rootguard`: `openRepositoryCachePartitionAttemptV1` refuses a directory inside a checkout that is not its root (scratch `.cache .tmp .🧬semio` pass), EN + DE; law `⚖️law/🌐️native`: no stray `.cache` below the root (git ignored-directory listing, 53 s under load). Deleting what exists: USER DECISION 2. |
| native `semio/typescript` (verify typescript), `test-sandbox` | Real producers: declared as areas (64 MiB / 32 MiB, collected after 30 d / 7 d). |
| native `.cache/framework-actor` fallback | Producer moved into the declared `command` store (`.cache/command/framework-actor/tests`). |
| repo `🧪️plugin-registry` (session test, touched 10-26 15:25) | Test now writes into the declared `🧪️tests` evidence area. |
| repo `next-services`, `nx-admission`, `edfix`, `e2b`, `p5`, `f1`, `e2`, `at`, `e1`, `plug`, `python`, `bun-workers`, ... (10-12 to 10-15) and native `impl-exp`, `gg` | No code writes them (grep); lane scratch and old producers. The 14-day (native 7-day) rule now actually runs (`sweep`); the oldest become eligible 10-26 21:05. |
| `🧰️framework/🤖️generated/process` 193 MiB | Proven obsolete: the command configuration stores into `.cache/command` (working-tree change of 10-26); readers of the old path: the caching policy entry `runtimeStateDirectories` and one test fixture only; the last write is 10-26 08:59 by a process that loaded the old configuration. Stage `procstore` removes the policy key, schema property, the input-glob loop, the fixture/schema field and rewrites the law (nothing tracked may name the old store). NOT deleted: USER DECISION 1. |
| laws | repository root: existing law + the test redirect; native root: new static scan (3 tests incl. self-test) with the door `requireDeclaredCacheSegmentsV1(path, ledger, "native")`. |

## 4. Batches, order, proof

Order: `rootguard` -> `unitcache` (after jobsrv-g) -> `sweep` -> `ledger` (after README/guide editors) ; `procstore` after user decision 1. Each: `sh disk-g-tools/staged/<batch>/verify.sh` (fresh mirror, apply, graph-free single-file suites; `... verify.sh before` = only the new laws against the unedited tree, they must fail). `python3 -I stage.py check <batch>` says base/applied/drift per file; tsc strict over the 14 TS files: 0 errors.

Verification (logs `C/🗑️generated/disk-g/verify-all.log`, `verify-final.log`; every run ended with `mirror: guard clean`, live tree unchanged). "after" = batch applied on a fresh mirror; "before" = only the new test files on an unedited mirror.
| batch | after | before (new laws against the unedited tree) |
|---|---|---|
| unitcache | gc 13/0, budget law 2/0, memo retention 2/0, needs 8/0, compiler lane 3/0 | budget law 0/2, memo retention 0/2 (red) |
| rootguard | root guard 2/0; budget suite 8 pass / 3 fail = the identical 3 envelope failures of the live baseline | root guard 0/2 (red) |
| sweep | sweep 4/0, cache-gc 19/0, launch 9/0, budget 7/0, ledger 11/0 | sweep 0/1 (red) |
| ledger | model 3/0, native law 4/0 (3 tests + stray scan, git-less mirror skips the scan), machine 8/0, needs 8/0, ledger 11/0, cache-gc 19/0; only baseline reds: `every writer named by the ledger exists` (print-fonts writer is outside the mirror), budget suite 3 envelope reds, `pool-model` 1 (all identical on live) | model 0/3, native 1/3 (red) |
| procstore | owner-view inference 2/0, runtime-store 2/0 (git init in the mirror) | n/a (no new files; `apply` refuses while the old store exists) |
Baseline on LIVE (unchanged tree, same single-file method): cache-gc 19/9/7/2/4, units 5 pass 1 fail (+1 flaky pid test), ledger 11, law 7, sqlite 3, machine 8, needs 8, gc 13, budget suite 8/3, parent 17/31, opening 3/2, store 1/8, pool-model 4/1 (envelope or pre-existing, not mine).
tsc strict over the 14 TS files of the batches: 0 errors. Emoji docstring uniqueness checked per file (the only duplicate, `launch`'s `🧰️`, pre-exists).

## 5. USER DECISIONS (nothing below was done)

1. Delete `🧰️framework/🤖️generated/process` (193 MiB: captures 365 entries, cargo-target, leases 20 entries), then land `procstore`. Removing the policy entry first would put the 193 MiB into the inputs of every framework project, so `apply` refuses while it exists.
2. Delete the stray cache roots (16.2 GiB of fixed images): first detach the four attached images (`hdiutil detach` of the mount points `…/📦️packages/🟦️typescript/.cache/b/l/m`, `…/b/o/m`, `…/📥️inputs/💾️storage/.cache/b/l/m`, `…/b/g/m`), then remove `🧰️framework/.cache`, `…/📚️library/📥️inputs/💾️storage/.cache`, `…/📦️packages/🟦️typescript/.cache`, `…/🧊️optional-cache/.cache`, `🧰️framework/🔨️modules/🏃️process/.cache`, `…/🔌️plugin/📇️registry/.cache`, `…/🛠️tools/.cache` (list: `git ls-files -z --others --ignored --exclude-standard --directory | tr '\0' '\n' | grep '\.cache/$'`). Check no run is using them first.
3. Native `.cache/impl-exp` (37 MiB, nested `.cache/impl-exp/.cache`) and `.cache/gg`: lane scratch, left to the 7-day rule unless the owners say otherwise.

## 6. Not done / open
- Live after-numbers: none by rule (no gc, no official run, nothing applied); the first sweep after landing is the measurement (`budget-sweep.log`, then `repo cache status`). Expected: `u/a` 2.4 -> <= 1.4 GiB, `u/l` and `u/o` back under their shares, `compile-cache-rust` <= 0.75 GiB, `.cache/bazel` gone.

- The 77 test files / 157 lines that use `process.cwd()` as workspace (mostly `join(process.cwd(), ".cache/b/...")`): the opener guard stops the image creation, raw lease paths in 8 process-module tests still join below the cwd. A codemod (`repositoryCheckoutRootV1` / `findWorkspaceRoot`) is the follow-up; not done blind.
- nx-tooling enforcement (nxret-g/lease-g), live after-numbers (no gc, no official run by rule): the first sweep after landing is the measurement (`budget-sweep.log`, `cache status`).
