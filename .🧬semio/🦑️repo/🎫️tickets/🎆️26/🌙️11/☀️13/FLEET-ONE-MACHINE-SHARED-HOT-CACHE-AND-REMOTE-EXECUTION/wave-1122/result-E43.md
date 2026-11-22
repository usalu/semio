# Result E43 (tests-caching-3)

All runs with `repo test-file --json`, 4-5 files in parallel, load 27-51 (the runner waits while load > 40), 420 s cap per file (the plain budget is 300 s). Table builder: `T/🧰️scripts/E43/table.py`, runner `run-list.py`, lock helper `lock.sh` (taken in alphabetical order, released after the edit), error-head helper `head.sh`.

## What was measured
- Population: every `.ts/.mjs` file below `C` (⚡️caching) inside a `🧪️tests` folder that calls a test registrar: 524 files. One of them is a declared helper that `test-file` refuses (`🏭️producer`, exit 2) and is excluded from the counts.
- Before: one full pass (11:20-12:15), taken partly while I edited, so a few files are already counted at their fixed state. After: the red files were re-run once (pass 2, 69 of 122 reached before its budget ended) and the files I changed were re-run after the last edit; files red before and not re-run are carried over unchanged (they are marked "not re-run" below). No full second pass was possible in the time (a full pass is about 70 minutes at this load).

## Before / after (files and test counts: 523 test files of `C` = ⚡️caching plus the 2 `🗂️workspaces/🧪️tests` files of E41's open list)
| | files passed | files failed | files timed out | tests passed | tests failed |
|---|---|---|---|---|---|
| before | 398 | 124 | 3 | 2778 | 475 |
| after | 414 | 108 | 3 | 2867 | 402 |

Net: 16 files red to green (14 in `C`, 2 workspaces), 73 test failures removed. (The first two lines of the raw table below count 524 `C` rows in the before line and 526 rows in the after line, because the two workspaces files only exist in the after column.) Of the 22 files whose row changed, 17 are files I edited or whose cause I fixed (listed below); the others (`🚀️bootstrap/🛠️tools/🔏️authorizers` data and growth, `📦️sdk` binary and snapshot, `🦀️cargo/🗃️unit-cache`, `🚀️bootstrap/🛠️tools/🧪️tests`) changed state between my two runs without an edit of mine (peer landings, or load: the first pass ran at load 40-50). Two files got worse and were not touched by me: `📦️artifacts/🧪️tests` 3/2 -> 0/6 (a new product refusal of the native receiver cleanup, "Native receiver ... has neither fixed.dmg nor image.json but is not an empty shell", raised from `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup`; the test creates `.cache/b/l/m/data` and expects its own refusal) and `🕸️graph/📤️publication/🧪️tests` 1/5 -> 0/6 (private-repository / live-store message, peers' area). No file I edited got worse.

## What I changed
Product (2, both real bugs found by tests):
1. `🗂️workspaces/🟦️.ts` `getWorkspaceRoot(environment, search)`: the search directory and a ceiling are explicit now (defaults: working directory, file system root). The old code could only test "no workspace above the cwd" when the temporary directory is outside every checkout, which is false under the hermetic test sandbox (`.cache/scratch` inside the checkout). Test `🗂️workspaces/🧪️tests` 18/2 -> 20/0 without `chdir`.
2. `🧪️testing/📥️invocation/🟦️.ts`: the receiver registered `afterAll` at module top level, which Bun 1.4 refuses outside the test runner, so any child script that imported `🧫️private-repository` (which imports this module) died with "Cannot use afterAll() outside of the test runner". The registration now ignores exactly that refusal. This is what held the `📥️required` child laws at 3/6; it also unblocks every child program that needs the private repository. Dependants: `📥️invocation/🧪️tests` 16/0, `🪟️isolation` 5/0.

Tests / data fixed against the current contract (each verified in the product, not loosened):
- `♻️reset/🧪️tests/⌛️wait` 0/3 -> 2/0: three causes. (a) E16 changed `nativeImageReclaimDueV1`: on a tight host the ratio no longer applies, floor/8 stays; the SQL oracle and `pressure.json` still demanded the ratio (case renamed `host-pressure-ignores-ratio-while-host-is-tight`, `due:true`). (b) The file used the raw `declareBuildTestEnvelopeV1({capabilities:{artifacts:true}})` birth: "capabilities-differ" in the child. Now `declareLibraryBuildTestEnvelopeV1({suite:"storage-opening-processes"})`. (c) The `not-due` fixture used a tight host (33 GB free); it now uses a roomy one.
- Declared-birth migration (same E37 rule, caching side): `🌐️vite/🏗️compiler/🎟️admission` (2/2 -> 3/0), `🏗️compiler/🚪️ingress/⚙️configuration/📮️order.ts`, `🚀️bootstrap/📦️dependencies/💾️storage/📥️read-context` (2/0), `🔭️providers/pressure.test.ts` (corpus `birth` is now `{suite,maximumElapsedMilliseconds}`, schema and JSON changed together; 2/0), `👓️reading` (birth), `🔍️discovery/📂️source` (2/0).
- `👓️reading/📥️recheck` 0/2 -> 1/0: the native-churn classification ("vanished") only fires for a path inside the workspace root and outside `.cache`; the artifact directory is now outside the checkout, so the test builds its area under `<workspace>/.test-scratch` (git-ignored, closure-refused, swept).
- `👓️reading/🟦️.ts` 49/36 -> 84/0: one `ReferenceError: dependencyNameKindV1 is not defined` (the product moved the foreign-name handling to `📤️publication/🧹️foreign-names`; the extracted function text now needs `dependencyNameKindV1`, `dependencyGenerationNamesV1`, `settleDependencyForeignNamesV1` and `dependencyForeignPolicy` in its ports). Same harness gap fixed in `🧹️retirement/📄️generation/🧪️tests` (the "valid" case; the file stays 5/1 on its separate `alias-drift` vector, see below).
- `🏪️store/🧪️tests/📥️required` 3/6 -> 8/0: the entry function has overload declarations before its body (test now picks the declaration with a body); the graph consumer call was refactored (`const root=graphDatasetWorkspaceV1(workspace)`, `...(root,"graph",control,run)`); the reuse law pinned a source position that the refactoring inverted (now: body defined before the first record check, consumer receives `body`); child-spawning laws now run under the private repository (`repository.invoke`).
- `🏪️store/🧪️tests/📥️queued` 2/3 -> 4/0: the declared roster of non-waiting maintenance sites was stale (new legitimate sites: cache-gc command and reclaim, private-repository retirement, mounted-parent observation, contract type, compiler capacity); roster rows and reasons updated.
- `🏪️store/🎟️reservation` 1/1 -> 2/0 and `🏪️store/🧪️tests/🟦️.ts` (reservation law): `machinePathV1` missing in the extracted-function ports. `🏪️store/🧪️tests/📦️child`: `nxGraphReleaseFailureV1` injected, and the single-failure contract ("one failure is thrown as it is") replaces the stale `AggregateError` assertion (4/6 -> 5/5). `🏪️store/🧪️tests/🟦️.ts`: the spawned child `bun test` now gets `./<relative path>` (a bare absolute path is a filter, so the child ran nothing and printed only the banner), and the child law loops once per mode (a refusal closes the shared Original, so the second loop iteration could only fail); still 3/6, see below.
- `🧾️closures/🧪️tests/📇️formats` 2/2 -> 4/0: `requireClosureReadDescriptorV1` added to the extracted worker functions.
- `🗂️workspaces/🧪️tests/🧬️inheritance` 1/2 -> 3/0: the live manifest no longer inherits `semio-framework-diagnostic` (path dependency since the last refactoring); the law now pins `semio-framework-artifact-reference`, which both chain members inherit (corpus and schema: the baseline list holds two rows).
- `🕸️graph/🏁️startup/🗃️lease-directory/🧪️tests/🏠️outside-checkout` 0/1 -> 1/0: "outside every checkout" cannot mean `os.tmpdir()` under the hermetic sandbox; follows the existing private-repository precedent (`/tmp`, Windows `os.tmpdir()`).
- `📤️publication/🧪️tests/⚓️anchor` 0/2 -> 0/1: moved to the private repository (the "captured runtime entry provider required" failure is gone); the remaining failure is the environment: "Case-sensitive host filesystem required for a directory partition" (default macOS APFS is case-insensitive; the directory-envelope law needs a case-sensitive volume or a gate with a console warning, not done).

Attempted and reverted (no gain, or worse): converting `🏦️nx/📦️graph/🧪️tests/⏱️deadline` to the private repository (2/4 in 14 s became 2/5 in 350 s over the plain budget, "private-repository-partition-has-readers"); converting `📦️artifacts/💾️storage/🧪️tests/📥️physical` wholesale to `repository.invoke` (every law then hit "Original physical body cutoff reached" and left a refusal of `hdiutil detach` status 16; restored byte for byte from my copy, no image remained attached); adding an artifacts birth to `native-orchestration/📥️invocation` (no change).

Genuine birth (`🧪️testing/🌱️genuine-birth/🧪️tests`, 4/1): the `executed` row `target-execution` runs the real preparation of the execution inventory of ALL emitted targets inside the birth; at load 30-50 it exhausts its own deadline ("Original target execution preparation deadline exhausted", measured 135 s inside the test, same via the script directly). The three `setup` rows pass. This is the slice read cost of the whole graph (E35: 276 s quiet), not a hang; the law needs either a preparation of a small target set or a cached inventory. Not changed.

## Still red after (108 files + 3 timeouts), by cluster (full list: table below)
- Extracted-function harnesses that evaluate product function text with a hand-built `ports` object and drift whenever the product helper set changes (the most frequent root cause in this run; about 15 files): borrower scope (`📦️child` "scope.top.size"), `📐️layout` (injected fault no longer fails), `🧹️retirement/📄️generation` (`alias-drift` accepted), dependency storage main file 28/20 (`busy:undefined` from the bindings gate; the fixture holds a lease on the same resource), `🏗️native-build/📦️owner` ("success" expected false), cohort-controller. Recommended fix: one harness module that extracts a function together with its transitive module-level helpers (or an exported test seam), instead of editing the port list per file.
- "Original ... required" (a test registered against the checkout Original where the product wants the repository Original / native caller grant): `📦️artifacts/*`, `📋️native-orchestration/*`, `🏗️native-build`, `💾️storage/…/📦️budget/🧪️tests` (about 25 files). The reliable fix is `privateRepositoryWorkspaceV1().invoke`, but a mechanical conversion fails (see reverted); each law needs its own scratch under the private root.
- Nx namespace / watcher (`🔌️nx/📦️graph`, `📦️outputs`, `🕸️graph/🔏️seal`, `🔗️import-edges`, `📥️inference/🧪️tests/🔍️imports`): need a registered Nx invocation with published project graph and file map; `test-file` provides the bounded namespace (E38) but not the watcher-bound one.
- Timeouts at 420 s: `🧾️closures/🧪️tests/🟦️`, `♻️groups`, `🏦️funded`; heavy files (> 300 s plain budget): `📏️capacity` 9/4 (166-250 s alone, red when a refusal law outlives its control under load), `💾️storage/…/📥️parent` 24/24.
- Environment: `🧾️ledger/🧪️tests/⚖️law/🌐️native` "stray cache roots": `.cache` directories left in subfolders by commands run from a non-root working directory on Nov 10-15, among them `🧰️framework/.cache` (8.4 GB), `🧰️framework/🔨️modules/🏃️process/.cache`, `…/🚀️bootstrap/🛠️tools/.cache`. I did not delete them (not mine, and the rule forbids `rm -r` outside my folder); the law tells the owner to delete them.

## Findings for others
- Bun 1.4 refuses `afterAll` outside the runner; any module that registers one at import time and is imported by a child program is a trap (fixed for the invocation receiver; `🧫️private-repository` guards its own with try/catch already).
- The hermetic sandbox puts `TMPDIR` inside the checkout (`.cache/scratch`, `.cache/test-sandbox`); every test that means "outside the checkout" or "no workspace above" must not use `os.tmpdir()` (two fixed here; likely more among the red files: grep `tmpdir()` next to workspace-root detection).
- Isolated build tests (`registerIsolatedBuildTestV1`) and laws that end in an expected refusal: a refusal closes the shared Original of the file (cause of the second-iteration failures in `🏪️store/🧪️tests`).
- Locks: my `caching-storage` lock was removed after 40 minutes by a peer (allowed); the `formats` edit was made while it had lapsed (recorded in `T/landings.md`).

## Not verified
- No full second pass; the 122 red files were re-run once after my edits (pass 2 reached 69, a final pass the other 53), the others carry their "before" row. Files I changed were each re-run after the last edit (workspaces 20/0, inheritance 3/0, wait 2/0, vite admission 3/0, order 1/0, read-context 2/0, pressure.test 2/0, reading 84/0, recheck 1/0, discovery 2/0, required 8/0, queued 4/0, reservation 2/0, formats 4/0, outside-checkout 1/0; child 5/5, store 3/6, anchor 0/1, generation 5/1 stay red).
- Dependants of `getWorkspaceRoot` were not enumerated beyond `🗂️workspaces` (its only caller with a second argument is the test).
- `reading` took 54 s in the last run (9 s when it stopped at the first error); its budget margin at load 40+ is not known.

## Table (changed rows and the remaining red list)
    files 524 before {'pass': 398, 'fail': 122, 'timeout': 3, 'helper': 1} tests pass/fail 2759 471
    after {'pass': 414, 'fail': 108, 'timeout': 3, 'helper': 1} tests pass/fail 2867 402
    changed 22
    🌐️vite/🏗️compiler/🎟️admission/🧪️tests/🟦️.ts|fail|2|2|pass|3|0
    🌐️vite/💾️storage/🧪️tests/🟦️.ts|fail|5|5|fail|7|2
    💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/⌛️wait/🟦️.ts|fail|0|3|pass|2|0
    💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🟦️.ts|fail|1|1|pass|2|0
    💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️queued/🟦️.ts|fail|2|3|pass|4|0
    💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts|fail|4|6|fail|5|5
    💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts|fail|3|6|pass|8|0
    💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts|fail|10|1|pass|11|0
    💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/⚓️anchor/🟦️.ts|fail|0|2|fail|0|1
    💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts|fail|2|2|pass|4|0
    💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts|fail|16|11|fail|25|2
    💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔢️batch/🟦️.ts|fail|0|2|pass|1|0
    📦️artifacts/🧪️tests/🟦️.ts|fail|3|2|fail|0|6
    🕸️graph/🏁️startup/🗃️lease-directory/🧪️tests/🏠️outside-checkout/🟦️.ts|fail|0|1|pass|1|0
    🕸️graph/📤️publication/🧪️tests/🟦️.ts|fail|1|5|fail|0|6
    🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts|fail|49|36|pass|84|0
    🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests/🟦️.ts|fail|None|None|fail|8|1
    🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/🟦️.ts|fail|None|None|pass|4|0
    🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️data/🟦️.ts|fail|4|2|pass|5|0
    🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth/🟦️.ts|fail|None|None|pass|11|0
    🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts|fail|4|6|fail|5|5
    🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts|fail|26|1|pass|27|0
    STILL RED
    fail 7 2 🌐️vite/💾️storage/🧪️tests/🟦️.ts :: error: Isolated build test did not end with its declared primary rejection: concurrent cancellable Vite listen
    fail 9 4 💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts :: error: Isolated build test did not end with its declared primary rejection: compact recovery is refused while 
    fail 4 2 💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🧪️tests/🟦️.ts :: error: rust/generic-macos-receipt-without-platform
    fail 5 5 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts :: 
    fail 3 6 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts :: error: expect(received).toContain(expected)
    fail 0 1 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/⚓️anchor/🟦️.ts :: 
    fail 1 11 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/📦️owner/🟦️.ts :: error: success
    fail 3 4 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🟦️.ts :: error: Explicit original setup-all asset purpose required
    fail 0 1 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts :: error: Closed fresh standalone declared asset entry required
    fail 0 1 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️graph-purpose/📜️script.ts :: error: Closed fresh standalone Graph maintenance entry required
    fail 1 2 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/🟦️.ts :: error: expect(received).toBe(expected)
    fail 2 1 💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🟦️.ts :: error: expect(received).toBe(expected)
    fail 25 2 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts :: error: Exact created Native physical identity required
    timeout None None 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/♻️groups/🟦️.ts :: 
    fail 0 1 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🔄️cohort-controller/🟦️.ts :: error: small-unknown-reuse
    fail 4 1 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts :: error: actual closure worker cancellation
    fail 1 2 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧪️admitted-worker-harness/🟦️.ts :: error: compileNxAuthorizerSnapshotV1
    timeout None None 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts :: 
    fail 4 1 💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts :: error: Nested Original process envelope must preserve its actual parent
    fail 24 24 💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts :: error: publish:/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%E2%9A%A1%EF%B8%8Fcaching/%F0%9F%92%
    fail 17 2 💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts :: error: Actual Native platform image required
    fail 10 1 💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧪️tests/🟦️.ts :: error: Required original cache consumer unavailable
    fail 2 4 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts :: error: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a private
    fail 2 3 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts :: error: expect(received).toContain(expected)
    timeout None None 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🏦️funded/🟦️.ts :: 
    fail 0 2 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts :: error: Exact graph purpose context required
    fail 2 2 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: error: Exact carried graph purpose receipt required
    fail 13 34 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts :: error: Exact created Native physical identity required
    fail None None 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/👁️observation/🧪️tests/🔑️kernel/🟦️.ts :: 
    fail 1 2 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: 
    fail 7 8 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts :: 
    fail 3 1 💾️storage/🧾️ledger/🧪️tests/⚖️law/🌐️native/🟦️.ts :: error: stray cache roots (delete them and run the command from the checkout root)
    fail 3 1 📥️inference/🧪️tests/🔍️imports/🟦️.ts :: 
    fail 2 1 📥️inference/🧪️tests/🟦️.ts :: 
    fail 5 1 📦️artifacts/🏗️native-build/⏱️observation/🧪️tests/🟦️.ts :: error: expect(received).toBe(expected)
    fail 1 15 📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts :: error: Actual artifact repository authority required
    fail 2 2 📦️artifacts/📋️native-orchestration/📥️invocation/🧪️tests/🟦️.ts :: error: Original Repo build caller required
    fail 5 6 📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts :: error: [script] {"owner":"native-owner-command-original-law","command":"native","stage":"loading"}
    fail 0 9 📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts :: error: Live Original process issuance required
    fail 0 1 📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts :: 
    fail None None 📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts :: 
    fail 0 6 📦️artifacts/🧪️tests/🟦️.ts :: error: expect(received).toBe(expected)
    fail 10 1 🕸️graph/🏁️startup/🚦️service/🧪️tests/👁️outputs/🧊️provider/🟦️.ts :: error: Isolated build test did not end with its declared primary rejection: admitted consumer refuses missing 
    fail 1 1 🕸️graph/🏁️startup/🚦️service/🧪️tests/🔭️startup-journal/🟦️.ts :: error: Workspace journal cannot vouch: No change journal service is running; the tree is walked.
    fail 0 2 🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts :: error: Original loaded SDK root required
    fail 8 2 🕸️graph/🏁️startup/🧬️lifetime/🧪️tests/🟦️.ts :: error: private-repository-partition-has-readers
    fail 0 1 🕸️graph/🏇️waiter/🧪️tests/🟦️.ts :: error: Original installed tooling identity and ticket artifact directory required
    fail 1 2 🕸️graph/📤️publication/🧪️tests/📦️transaction/🟦️.ts :: error: Build test receiver closed with active callback
    fail 0 6 🕸️graph/📤️publication/🧪️tests/🟦️.ts :: error: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a private
    fail 6 12 🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts :: error: Required original cache consumer unavailable
    fail 3 13 🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts :: error: [{"instancePath":"/inference/0/authorityObservations/28/kind","schemaPath":"#/$defs/ObservationV1/prope
    fail None None 🕸️graph/🔏️seal/📥️membership/📥️journal/🧪️tests/🟦️.ts :: 
    fail 0 4 🕸️graph/🔏️seal/🧪️tests/🟦️.ts :: error: Nested persistent image requires actual parent scope
    fail 3 2 🗂️suites/🧪️tests/🟦️.ts :: error: expect(received).toEqual(expected)
    fail 0 3 🚀️bootstrap/📥️admission/💾️storage/🧪️tests/🟦️.ts :: error: Original canonical collection graph backing required
    fail 3 1 🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts :: error: expect(received).toThrow(expected)
    fail 28 20 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts :: error: busy:undefined
    fail 5 1 🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/📄️generation/🧪️tests/🟦️.ts :: error: alias-drift: undefined
    fail 1 1 🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🗃️unready/🧪️tests/♻️roles.ts :: error: unready-generation:none
    fail 7 1 🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts :: error: expect(received).rejects.toThrow(expected)
    fail 8 11 🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts :: error: Managed Python test interpreter binding required
    fail 0 1 🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts :: error: Original bounded Nx namespace required
    fail 1 5 🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts :: error: Original bounded Nx namespace required
    fail 0 1 🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures/🟦️.ts :: error: crates/a tests=false
    fail 10 1 🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🟦️.ts :: error: cut 🏗️build/📜️script.ts
    fail 1 3 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: error: Exact same-wire toolkit refusal receipt required
    fail 0 2 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts :: error: Original SDK and artifacts required
    fail 0 1 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/🟦️.ts :: error: Original artifacts and immutable SDK identity required
    fail 0 3 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/⚠️failure/🟦️.ts :: Expected: 0
    fail 3 5 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts :: error: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a private
    fail 7 1 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🟦️.ts :: 
    fail None None 🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🧪️tests/🟦️.ts :: 
    fail 8 1 🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests/🟦️.ts :: 
    fail 4 1 🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🧪️tests/🪟️live-recipe/🟦️.ts :: error: expect(received).toContain(expected)
    fail 17 7 🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts :: error: Isolated build test failed: unsealed derivative publication preserves genuine phase through EX seal and
    fail 2 1 🚀️bootstrap/🛠️tools/🔁️probe/🧪️tests/🟦️.ts :: error: expect(received).toEqual(expected)
    fail 2 2 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️scope/🟦️.ts :: error: expect(received).toBe(expected)
    fail 3 1 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts :: error: computed sites outside the declared table
    fail 5 5 🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts :: 
    fail 1 2 🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/🟦️.ts :: 
    fail 1 3 🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🧪️tests/🟦️.ts :: error: Exact private retention workspace required
    fail 41 3 🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts :: error: Isolated build test failed: selected tools role generation retirement integrity is privately non-runnab
    fail 7 1 🚀️bootstrap/🛰️fleet-fetch/🎟️work-claim/🧪️tests/🟦️.ts :: 
    fail 3 1 🚀️bootstrap/🛰️fleet-fetch/🦀️crates/🧪️tests/🟦️.ts :: error: error: current package believes it's in a workspace when it's not:
    fail 1 2 🦀️cargo/📦️units/🏗️compiler/🔏️code/🧪️tests/📖️reader/🟦️.ts :: error: expect(received).toBeLessThanOrEqual(expected)
    fail 0 1 🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/📦️child/📜️script.ts :: error: Exact owned dependency fixture arguments required
    fail 5 13 🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts :: error: expect(received).toBe(expected)
    fail 17 7 🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts :: error: expect(received).toHaveLength(expected)
    fail 0 1 🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts :: error: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a private
    fail 11 17 🦀️cargo/📦️units/🧪️tests/🟦️.ts :: 
    fail 0 1 🦀️cargo/🗃️unit-cache/🏛️seal/🧪️tests/🟦️.ts :: error: [toolchain] Managed test binding SEMIO_TEST_PYTHON_BINARY for python is required and must pass its prob
    fail 0 9 🦀️cargo/🗃️unit-cache/🧪️tests/🏖️scratch/🧪️tests/🟦️.ts :: error: expect(received).toContain(expected)
    fail 0 2 🦀️cargo/🗃️unit-cache/🧪️tests/🕰️freshness/🟦️.ts :: 
    fail 12 1 🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🧪️tests/🟦️.ts :: error: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a private
    fail None None 🧪️tests/🌍️caller-environment/🟦️.ts :: 
    fail 3 1 🧪️tests/🎮️playground-preparation/🧪️tests/🟦️.ts :: error: Original Script deadline refused
    fail 3 1 🧪️tests/📋️values/🧪️tests/🟦️.ts :: error: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:projectBuildTargetForNxV1
    fail 0 1 🧪️tests/📐️phases/🟦️.ts :: 
    fail 0 1 🧪️tests/📦️native-dependencies/📥️purpose/📜️script.ts :: error: Strict bounded Native fixture negative required
    fail 2 6 🧪️tests/📦️native-dependencies/🧪️tests/🟦️.ts :: 
    fail 0 1 🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts :: error: Original bounded Nx namespace required
    fail 0 2 🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts :: 
    fail 1 8 🧪️tests/🔗️import-edges/📁️paths/🟦️.ts :: error: Original bounded graph writer custody required
    fail 2 11 🧪️tests/🔗️import-edges/🔁️context/🟦️.ts :: 
    fail 15 2 🧪️tests/🗃️compiler-state/🟦️.ts :: error: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🧪️tests/🟦️.ts
    fail 0 1 🧪️tests/🗿️artifact-source/🟦️.ts :: error: Select the active ticket with --ticket <directory> or SEMIO_TICKET_DIR
    fail 0 1 🧪️tests/🛑️coordinator/🔮️oracle/🟦️.ts :: 
    fail 4 2 🧪️tests/🧾️cargo-provenance/🟦️.ts :: 
    fail 1 2 🧹️pruning/🏪️budget/📜️history/🧪️tests/🟦️.ts :: 
    fail 11 1 🧹️pruning/🏪️budget/🧪️tests/🟦️.ts :: 
    fail None None 🧹️pruning/🔒️incremental/🧪️tests/🟦️.ts :: 
