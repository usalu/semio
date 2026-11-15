# Graph slice starvation: root causes, fix and verification

Ticket: GET-ALL-PRINT-PRODUCTS-RUNNING-END-TO-END. `BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.
Builds on the audits of `26/11/09/INCREMENTAL-MULTI-AGENT-BUILD-GRAPH-SERVICE` (scoped-plan: the slice is already target-scoped; owners: governor and caps); nothing there is contradicted.

## Findings

1. The cancellation is a platform bug, not real starvation. `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` (`cgroupMeasuresV1`, default `free=freemem`) and `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs` (`hostAvailableBytesV1`) fell back to `os.freemem()` off Linux. On macOS that is the count of free pages only; the kernel keeps the rest as file cache, so a healthy 16 GB host sits at 60 to 250 MB "free" while `vm_stat` shows 4 to 7 GB reclaimable (`repo fleet status` said 5506 MB free at the same time). Any slice load that ran 30 s therefore tripped the 256 MB floor of `memoryPressure` and was cancelled: 20 of 20 runs, also the solo baseline below.
2. A slice build publishes nothing until every stale owner of the closure is re-inferred (`BUILD/🕸️graph/🕰️generations/🟨️.mjs` `buildOnce`: one `compute` for all of `found.dirty`, one publish at the end). The sealed generation is generation 0, 27 h old, 5078 owners; the closure of `print:test-pdf` is 693 stale owners. A cancelled or killed build therefore keeps zero progress, the next run starts again at owner 0 (and again pays facts, verification and rederivation, about 25 s), and the single worker process carries the memory of all 693 owners at once. That is why it never converged.
3. The scoping itself is right: `refreshClosure` verifies only the closure (`only`), `sliceClosureV1` computes it, so 693 is the real closure of the target (print reaches the framework crates through its inputs), not a whole-workspace re-derivation of 5078.

## Change

- `BUILD/🕸️graph/🕰️generations/🟨️.mjs`: `build` loops over waves; `buildOnce` re-infers at most `ownerWave` stale owners, publishes a generation, and hands the remaining dirty set to the next wave as a hint (no new verification pass while the generation did not move; a moved generation re-plans). Rules: the verified stamp (`coverage`) and `doubt.published` are written only by the final wave (a wave must not vouch for owners it did not re-derive); `retire` runs after the final wave; owners of later waves are never dropped; a lost swap or a peer merge falls back to a fresh plan.
- `BUILD/🕸️graph/🕰️generations/🧬️schema/🔣️.json` and `🟨️.d.mts`: policy `ownerWave` (48, EN and DE descriptions).
- `BUILD/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`: progress shows the overall `done of total` across waves (clamped monotonic: the first live run crashed with "Monotonic graph phase progress required" when a wave with newly discovered projects ended above the next wave's start; pinned by the fifth waves test), the "building" notice is said once.
- `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs` (+ `.d.mts`): `hostAvailableBytesV1` answers what macOS can hand out (`parseDarwinCountersV1` of the host-capacity sampler through `vm_stat`), never the free pages alone; `🌡️pressure/🟨️.mjs` uses it.
- Tests: new `BUILD/🕸️graph/🕰️generations/🧪️tests/🌊️waves/🟦️.ts` (5 tests: wave count and equality with the Python oracle, cancellation after a wave keeps it and the resume derives only the rest, closure stamp not over-claimed); new `BUILD/🏡️resident/🧪️tests/🌡️pressure/🟦️.ts` with fixture `🔣️.json` (vm_stat fixtures, independent Python parser, watch quiet on a cache-heavy host and tripping on a scarce one); updated the 300-project churn test in `🕰️generations/🧪️tests/🟦️.ts` (it pinned "one build, one generation").

## Verification

Command: `repo tools exec --light bun -- test ./<file>` (the script-level entry `repo ./📜️script.ts` is refused by the invocation guard).
- waves: 5 pass (the progress test fails without the clamp). Whole `🕰️generations/🧪️tests` (29 files): 137 pass, 5 fail before my test-file fix; the churn test I updated now passes (30 pass in that file). The other 4 failures need `SEMIO_TEST_ARTIFACT_DIR` (set by `repo run`, absent under the light entry): not related.
- pressure + cancel + host: 73 pass, 1 fail (`host service held by a long run ... retires after it is released`, idle timer test; touches nothing I changed, timing on a loaded host).

## Timings, `repo run @semio-tech/print:test-pdf --lang=en --output=static` (logs in `🗑️generated/`)

| run | result |
| --- | --- |
| before (solo, `baseline-test-pdf.log`) | 82 s, cancelled "only 65 MB of free memory", at owners 46 of 693; 20 of 20 earlier runs the same |
| after, run 1 (`after-test-pdf.log`, old service still resident with the old pressure code) | 166 s, same cancellation: the watch lives in the long-running service, which must be restarted (`repo service host stop`) to take the fix |
| after, run 2 (`after2-test-pdf.log`, fresh service) | no memory cancellation; owners 68 of 693 at 330 s, then my progress bug above (fixed, regression-tested) |
| after, run 3 (`after3-test-pdf.log`) | never admitted: another agent's `projects` run held 8.6 GB admitted (12.2 GB real, swap 7.3 GB, pressure critical) and the capacity gate kept my run waiting 568 s; I stopped it. |

What the runs do show: generation 0 became generation 5 through 48-owner waves, also across aborted and crashed runs (`repo fleet status`: "Generation 5: built delta ... 48 dirty owner(s)"), so progress is kept instead of restarting at owner 0, and the pressure watch no longer cancels on macOS.

## Not verified

- A complete `print:test-pdf` run to green after the fix (the closure still needs roughly 600 more owners at 1 to 30 s each, much slower while the host is swapping; the host was shared with another heavy run for all my attempts).
- Why every owner of the closure is stale (probably the 27 h of fleet churn since generation 0, which touched files all owners consume): not diagnosed. The per-owner project records (`observeProjectInferenceV1`) did not give cheap reuse in the observed runs; a follow-up should log `spec.reasons`, which the service's stderr (host log under `$TMPDIR/semio-repo/501/host-*.log`) carries.
- The duplicate verification between the shared reader attempt and the builder attempt (about 25 s each, seen twice in the logs) is untouched; the existing hint reuses a plan only within one refresh.
- Memory is bounded per wave (release between waves) but I did not measure the worker's peak; the 2 GB observed before was the whole closure in one build.
- Light-entry failures not caused by this change: 4 tests that need `SEMIO_TEST_ARTIFACT_DIR`, and the host idle-retirement timing test.
