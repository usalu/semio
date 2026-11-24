# O19-patched: the requester side of O19 for the play snapshot

Snapshot `/Users/ueli/.sinc/play` (O17 + O18-patched applied; offload files equal the O18 post-images). Nothing was written to the snapshot or its sealed copies. Apply: `T/wave-1122-dist/O19-patched/apply.sh <root> [sealed]` (`list.tsv`: stage 1 checks every target against its pre-image sha256 and refuses on any mismatch before changing anything, a file that already has the post-image is skipped, so it is idempotent; stage 2 writes `<name>.o19tmp` and renames; schema first, `offload.ts` last).

## What it fixes (all REQUESTER-ONLY: no worker, scheduler or cache restart is needed for any of it)
1. **Complete input sets (the `guillotiere` failure, 150 s per final, also `semio_hub_puzzle` 33 s and the `puzzle_5d` tests).** Cause, shown with the play files: the closure followed the 16-hex extra-filename of each artifact's metadata. The restored `semio_hub_puzzle` (lane `p9a4254`) was built against `guillotiere-908214a9568d8e4f`, but that lane holds the same crate as `guillotiere-687b27ccbef9a905` (same svh `7a4dab55...`, other extra-filename). rustc finds a dependency by crate name and svh, so locally it loads 687b; the shipped set looked for 908214, found nothing and left guillotiere out. Now `dependencyClosureV1` follows the unit cache's own recorded edges (`<rmeta>.semio-deps`: svh plus name, extra-filename and svh of every crate), takes a dependency by extra-filename when an artifact of that name has it, else by name and svh, and falls back to the metadata tokens for an artifact without a record. Measured on the real lane (1214 `-L dependency=` dirs, `semio_hub_puzzle` rlib as the direct extern): snapshot code ships 2004 files without guillotiere in 3.0 s; the new code ships 2010 files with `libguillotiere-687b...{rlib,rmeta}` in 0.64 s (the sidecar replaces reading every rmeta).
2. **Decline before upload (`no-compatible-worker`).** The requester reads the scheduler's worker list (`/status`, signed like the idle call, one request per 15 s per machine, memoised in `offload-workers.json`), and declines a unit in microseconds, with no upload and no remote attempt, when no other, non-draining worker advertises every required property (the scheduler's own rule, `remoteExecutionPlatformMatchesV1`). The detail names what no worker offers (`semio-linker=...`). It declines only on a known list: no answer means the old path. Today (13 workers listed, all advertise `semio-linker=0fbcef956b2adb56`, the id this snapshot's requester asks for: `offload-linker.json` of the play cache) it declines nothing.
3. **Finals are counted only on capable workers.** `awaitFleetFitV1` (the finals gate) and the `componentJobs` provider of the driver use `fleetLinkedSlotsV1`: the scheduler's idle count at the predicted peak, never more than the free slots of the workers advertising the requester's linker. The component-jobs plan is `1 + eligible slots` (`local: 1`; the extra local jobs are decided by the memory admission of the O14 code), and the peak is the median of the MEASURED crate-ledger peaks (not the 7.9 GB family estimate) whenever one exists. The old plan counted the 4 compile tokens as component jobs: "4 local + 1 on the fleet" meant 4 light slots without a memory reservation.
4. **Hold-off after a full store.** A requester that got `507` on an upload no longer lets every following unit pay its own 8 to 11 s: for `limits.storeFullMemoMilliseconds` (30 s) every call declines at once (`infrastructure`, detail `memo the content store is full (507)`). 892 of the 1,427 `infrastructure` records of the play log are 507 (4,004 s wasted).

Also (live tree, not in this set because workers are not in the snapshot): a worker now refuses an action whose own platform requires a property it does not advertise (`properties` passed to `executeRemoteExecutionActionV1`; the scheduler was already strict, `remoteExecutionPlatformMatchesV1` demands every required property).

## Files
| file | kind | pre sha256 (16) | post sha256 (16) |
|---|---|---|---|
| `offload-schema.json` | patched | `ff16be1831728b7a` | `e56728613480ea5c` |
| `policy.ts` | patched | `fa01ce2684583d66` | `4029c850b58e1eb0` |
| `inputs.ts` | patched | `4408f8bdbcb482e4` | `f6c2c16f7c8e4b46` |
| `admission.ts` | patched | `26af3a08033ae478` | `450befab8e111215` |
| `closure-test.ts` | new | `NEW` | `99cc33cac100ebbb` |
| `closure-oracle.py` | new | `NEW` | `790f600b9e669271` |
| `eligible-test.ts` | new | `NEW` | `c13adcf78dc950d8` |
| `driver-stages.ts` | patched | `8a73052077c9abab` | `8363c2fbc379ae83` |
| `offload.ts` | patched | `cff4016a6d13d929` | `98b462cadf36d659` |

`edit-offload.py` and `edit-driver.py` are the scripted edits that produced `offload.ts` and `driver-stages.ts` from the snapshot's own files (live main differs in both files for other reasons; the patched copies here are the snapshot's files plus exactly these edits).

## HOT-LOADABLE? Which files, and which not
- cargo keeps `RUSTC_WRAPPER=<clone>/.cache/b/r/m/u/bin/rustc-wrapper-05fd3e08470211af` for its whole life. That shell script `exec`s the snapshot bun on `<clone>/.cache/b/r/m/u/sealed/05fd3e08470211af/<path>/script.ts`: a SEALED COPY of the wrapper's closure (346 files) named by digest. The wrapper is a fresh process per rustc call, so it imports every module again at each call.
- Therefore: copying the patch into the clone's SOURCE tree is NOT picked up by the running build (the sealed copy is what runs; a new seal, i.e. the next driver/build start, takes the source). Copying the same files into the SEALED directory IS picked up by every rustc-wrapper spawned afterwards, without restarting cargo or the driver: `apply.sh <clone>/.cache/b/r/m/u/sealed/05fd3e08470211af sealed` (drops the framework prefix, patches the 5 wrapper files that exist there: schema json, policy, inputs, admission, offload; tests and driver are skipped). Verified on a copy of the sealed directory (5.1 MB, deleted afterwards): the 5 files apply, `bun build` of `script.ts` succeeds with the snapshot bun, and the patched closure returns guillotiere for the real lane. A wrapper that is already running keeps its old code until its unit ends. The sealed digest in the directory name no longer matches its content after that; nothing in the wrapper shell script or `script.ts` checks it, and the next seal makes a new directory from the (then patched) source tree: apply to the tree too.
- Not hot-loadable: `driver-stages.ts` (the `componentJobs` plan is made once at driver start: needs the native-build driver restart, i.e. the play restart that the session plans anyway).
- Not in the snapshot at all: worker code (`worker/🟦️.ts`), picked up by the workers themselves (below).

## Expected behaviour per combination
| situation | result |
|---|---|
| sealed hot-load only (driver not restarted) | new wrapper processes ship the complete closure (guillotiere and every svh-matched dependency) so finals and library units that failed with `incomplete-inputs` succeed on workers; decline-before-upload, 507 hold-off and the linked-slot gate in the wrapper are active; the component-jobs plan of the running driver is unchanged (4 light + 1 plan) |
| driver restart with tree + sealed patched (play restart) | plus the plan `1 + eligible slots`: finals run on workers up to the number of capable free slots, local finals by memory only |
| workers not yet running worker-side code | nothing is lost: every worker already advertises the linker, the new strictness check is a backstop only |
| every worker lacking the property (e.g. a toolchain roll) | `no-compatible-worker` at once, final compiles here, no 150 s attempt |
| scheduler or `/status` unreachable | unchanged old behaviour (no decline) |

## Verify (live tree, `repo test-file`, one run at a time)
See `O19-decline-before-upload.md` (test table). In the snapshot: no `repo test-file`; run the new test files with the snapshot bun on exact `./` paths after applying to a scratch copy.
