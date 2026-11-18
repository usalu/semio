# R153: standing red tests of the build-system batteries

Machine: this Mac, load 45 to 60, every run `nice -n 10` through the genuine-birth harness (`--birth=setup`), foreground, one test file at a time. Landings: ten, each under `landing.lock` (3 to 40 s), staged edits applied by anchor with a base-hash check (`🧰️scripts/R153/apply.py`), run under the lock, restored on red (tested twice: the 📥️parent file at 21:58 and 22:00, restored by my own zero-fail rule although it had improved; landed at 22:03 with the improvement). Entries are in the `landings.md` of the fleet ticket ("R153 landing A" to "J"). Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R153/` (`gA.py` .. `gJ.py` are the staged edits, `land.sh`, `stagecheck.py`).

Classes: (a) the test encoded behaviour a landed change replaced on purpose, rewritten to the new law; (b) real regression, code fixed; (c) wall-clock under load, made deterministic; (d) needs something unavailable here.

## 1. The list of the task, one row each

| Test (file) | Class | Law or fix | State |
|---|---|---|---|
| `🏡️resident/🧪️tests/🔁️restart` "steady churn" | c | Pool gets the option `now` (clock of the settle law and of `ranMilliseconds`); the test advances a logical clock by 15 per started worker against a settle time of 80. No wall-clock race. | landed E, 5/5 |
| `🛑️too-heavy` (3) | a | Grant of one owner = (admitted - parent floor 3072) / factor 2 / margin 1.25 (2048 MB on 16 GB), the solo grant of an 8 GB host leaves the system reserve 1536; the stopped-owner refusal is the one after the inference phase (the stall refusal moved out of `waitHeavy`). | landed A, 6/6 |
| `🛂️admission` "selected Source discovery yields original control" | c | The test mocks `performance.now` (+250 ms per read) against a 30000 ms loan deadline of that fake clock; the number of clock reads of the code decided the winner. The case proves cancellation, so its deadline is 8 h of fake time. Not load: deterministic failure. | landed D, 20/20 |
| engine-digest (`⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest`) spawn-only | **b** | `🏛️sealed/🚦️trampoline` and `🧭️invocation/🔄️reload` reach the sealed entry (the whole repo command line) only by its path, so the engine closure of 1935 files contained the command line and every edit of it moved every Nx task hash. Two `spawnOnly` rows declared; closure 1453 files; `📜️script.ts`, `🚪️door/🔄️restart`, `🩺️doctor` leave it. The Nx engine digest of every checkout changes once. | landed G, 11/11 |
| nx-engine (2) | a | The real-hasher graph needs an external node for every dependency the engine inputs name (`@tailwindcss/vite`); the output-classifier law runs on the private repository root, never on the live store. | landed F, 19/19 |
| host pool "at most the maximum run at once" | a | `pool.run` is `share`: identical loads of one root share one worker; different loads of one root queue. Proven by the `started` counter (2, then 3, the twin never starts), not by elapsed time. | landed E, 37/37 |
| host "a full disk reaches the client…" (new in the file) | a/c | The service refuses with the image of the live checkout (`.cache/b/l`, "repo cache gc") when `access.root` is the working directory; the test gives its own root. | landed E |
| sidecar pool (3 fail in R147, 1 now) | a | `foreground` carries `waitingMegabytes` (sum of the minimum memory of the waiters). | landed E, 7/7 |
| `🚦️governed-dispatch` (2) | a | Only `phases?.running("inference",list...)` publishes the running list; the warm wave names its own pseudo owner. | landed A |
| `🧹️parent-budget` | a | Collector, governor and `collectResident` never read the resident set (the old law banned the string in the whole file, which the owner-cost probes legitimately use). | landed A |
| `📈️inferred` | a | A farm miss refuses the inline inference (`farmJobRefusedV1`), the project is counted first. | landed A |
| `🛟️stopped-wave` | - | green on this tree before and after, no action | - |
| `🫧️take-release` (2) | a | Release after a look follows the growth `lookReleaseGrowthBytes` (128 MB), so the real-process bound is 128 + 96 MB (control still above 350); source laws follow the lap wrapper and the `deferOffer` argument. | landed C, 3/3 |
| `🪆️authority` | a | A farm below two workers is refused (`serial (memory)`): the fixture forced one worker and never opened a farm; it forces two. | landed C, 2/2 |
| `📦️project-records` main | a | The gate is found through `lookupLapAsyncV1("gate",...)`. | landed D, 6/6 |
| owner-pool main, early-wave, heavy, one-budget, lifecycle, shared-budget, exits, lanes (all 14 files) | c | "Real processes scale" and the host-matrix lab laws pass on this run (load 45 to 60); the failure of R146 did not reproduce, so nothing was changed and nothing is claimed beyond "green now". | green, not changed |
| tools-stage worker test (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent`, 48 tests) | a | The source-transpiling tests stub the capture providers by module name; the platform now reaches them through `nativeStorageOperationsV1(id).captureProtectedMaintenance/.retireWithWriter` of `🔌️backends/📋️contract`. Eight stubs replaced, the physical-custody case models the three new ports. The real capture ran, failed on `fixed.dmg`, closed the Original admission and failed every later test (29 of 48). 19 pass -> 25 pass. | **partial**, landed H4 |
| lend oracle | - | not located by name in the build library, host-capacity and sidecar batteries are otherwise green | not found |

## 2. Further standing reds found by the broad battery (205 test files of the sidecar, host capacity, resident, farm, take, script-inputs and inference batteries, one at a time)

| Test | Class | Law or cause | State |
|---|---|---|---|
| containment "native artifact input step counter" | a | Every staging, verification and read site carries `nativeArtifactInputStepV1()` (six sites now), not a literal 5. | landed I |
| resident `🪜️plan-refusal` | a | The worker forwards each plan-stage event through the wider callback. | landed I |
| `🆔️host-identity` (5), `🧭️start-identity` (2) | c/d | Runtime sockets need a base below 96 bytes; the harness `TMPDIR` is 100+ bytes. Tests use `/tmp` on POSIX (as the host test does). | landed I |
| host-capacity `👥️agents`, `🧾️label` | a | A farm is admitted with one worker of 768 MB, so a waiting farm needs a host with 300 MB free (3000 admits at once). | landed I |
| `🥇️first-child` | a | Least admission of show and graph = one worker plus a lean parent of half a worker (2304). | landed I |
| host-capacity `🔼️upgrade` | a | Quiet learning needs a peak below `quietMaximumMegabytes` (1024); the quiet case observes 1.2 x `quietReferenceMegabytes`. | landed J |
| sidecar graph `🌳️tree` | a | A caller whose signal ends detaches from the read (`detached`), the read is not stopped. | landed J |

## 3. Still red, with the reason (no test weakened)

| Test | Class | Finding |
|---|---|---|
| 📥️parent: 23 of 48 | a, partly d | `canonical maintenance keeps all retirement guards`, `owned disposable detach`, `child factories` read the removed retirement shapes (`recomputable` branch, `🍎️image` stubs); the dispatch, accepted-lifetime, private-continuation, nested and closed-protocol workers spawn `bun test` children of the file and the first failing child closes the Original admission for the rest. Needs the owner of the native image storage (R150 area); a staged copy cannot prove the child tests (the child runs the live file). |
| inference `🧪️tests/🔒️lint` "no module level Map or Set" | **b** | Six module-level collections without CONSTANT_CASE names: `📦️project-nodes/🟨️.mjs:358 stagesV1`, `🪆️heavy-questions/🟨️.mjs:40 running` and `:72 preflights`, `🧭️script-inputs/🟨️.mjs:1400 verdictMapsV1`, `:1427 verdictDirsMadeV1`, `:1579 closureFleetLanesV1`. New today in the files R143/R149/R151 edit; not touched here. |
| `🔌️providers` | b or a | With a checkout-scoped artifact directory 4 of 5 pass; "an evicted closure entry is answered again" finds `helper.ts` missing from the inputs of a `loads` command (`declared.helper` false). Under the standard harness artifact directory (outside the checkout) the first case refuses; run with `SEMIO_TEST_ARTIFACT_DIR` inside the checkout. |
| `🚶️repository-walks` (2), `🧷️closure-facts` (2), `👓️walk-views` (2) | a/b | repository-walks: step 0 `crates` answer differs from the oracle digest (deterministic); closure-facts: the text-model fixture looks for `emojiProjectJsonNodes` whose start marker no longer exists (code moved); walk-views: two cases not examined. |
| `🌊️layer-stream`, `💾️entries` sidecar growth | b or a | Retained heap after three rounds is 14.3 MB against a bound of a tenth of the bytes read (10.2 MB), repeatable to 4 digits; `💾️entries`: 10.04 against 10. Probably the raised closure-store caps of R134 (`closureStoreMaximumLayers` 524288, sidecar cache 96 MiB): the law needs a new bound derived from the declared cache, owner decision. |
| `⏫️child-priority` (2) | d | The runner is niced (`nice -n 10` plus background nice 5 = 15), so a "normal priority" child cannot be made; passes from an unniced runner. |
| `🎓️course-inputs`, `🎯️target-fanout` | d | Did not finish within 10 minutes at load 50; no result. |
| `🌡️host-capacity/🧪️tests/🔁️restart/📜️child`, `🧮️arguments`, `🧫️fixtures/🪆️kernel-role` | - | Fixtures named `🟦️.ts`, not tests (my list included them; ignore). |

## 4. State of the batteries on this machine

Green after the landings (this run): all 14 project-nodes test files and all 14 owner-pool files, all four `🔁️restart` files (resident, host capacity, fleet, cache server, sidecar), host (37), sidecar pool (7), nx-engine (19), engine-digest (11), admission (20), project-records main (6), take-release (3), authority (2) and the seven files of landing I and two of J. Not green: section 3.

## 5. Not verified

Real worker machines; the effect of the engine digest change on a warm Nx cache (every checkout recomputes its engine digest once); the 📥️parent child-process tests after a rewrite; owner-pool "real processes scale" under a worse load than 60 (it passed twice, it failed for R146).
