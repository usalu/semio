# Private root tool fixes (2026-11-23)

Source: `📓️private-root-build-2026-11-23.md` and `🗑️generated/private-*.txt`. No private-root build was started and the live build was not touched; everything was checked with unit tests (bun test, absolute paths).

## 1. Dependencies image sized at 20 % of free space
Cause: a fixed partition with `hostSharePercent` was born at `min(declared, share of host free)`; 20 % of 18.5 GiB is 3785 MiB, the install needs 6400 MiB (usable) on macOS.

Contract now (`budget/📐️elastic/🟦️.ts`: `hostShareCapacityV1`, `pinnedCapacityV1`):
- A fixed partition may declare `needBytes` per platform (`darwin`, `linux`, `win32`) in `budget/🔣️policy.json` (schema `partition.needBytes`). The born capacity is `max(share, need)` capped by the declared capacity. The dependencies need is the capacity whose usable bytes, after the ledger's nested shares (`remote-execution-worker`, 10 %), hold the install of the platform's backend (`concurrentRequiredImageBytesV1`): darwin 7144 MiB, linux and win32 (copying backend) 36057 MiB. A test proves each is the smallest such capacity with an independent Python search over the image overhead.
- The honest measure is the free space of the volume holding the partitions (`hostFreeForSizingV1`, now exported, pinnable for tests).
- Refusal only when it truly does not fit: `requireRepositoryPartitionFitsHostV1`, called first in `openRepositoryCachePartitionV1` (the one funnel of every image), throws the typed `ImageDoesNotFitHostErrorV1` for an image that is not born yet when host free bytes are below the need. The message is `en | de` with partition, needed MiB and free MiB. A born image is never refused (the commands that free space still open it).
- Side effect: the copying backend on linux and windows never fitted in 20 % of a 56 GiB host; it is now born at its need.

## 2. `Actual inherited control restriction required` in `tools setup all`
Root cause is not the child environment: `incrementalChildEnvironmentV1` dropping `SEMIO_*`/`NX_*` is intended isolation and the child mints its own invocation. The error concerns the in-process control chain. `recoverCompactCompilerHeadroomV1` was started by the store owner with the `reservationControl` captured above the deadlines of the compile phases, but it runs inside the live exclusive writer custody (`requireCompilerWriterCustodyV1` -> retained lifetime -> `requireIssuedResourceControlRefinementV1(scope.base, control)`). A control above a row with `requestedMilliseconds != 0` is no refinement of that custody. Only an image short of headroom reaches the recovery, so the roomy live image never showed it; the 4 GiB private image did.

Decision: the recovery works under the writer's own control, no inherited restriction needed in a fresh root. `recover` now receives the control of the writer scope (`recover(actual)` in `withCompilerInitialHeadroomV1` and `withCompilerSharedHeadroomV1`) and the store owner passes it on. Files: `🏗️compiler/📏️capacity/🟦️.ts`, `🏗️compiler/🟦️.ts`.

Test `🏗️compiler/📏️capacity/🧪️tests/🪢️writer-control/🟦️.ts`: a chain with a 200 s and a 5 s deadline; asserts the captured control throws exactly `Actual inherited control restriction required`, the writer's control passes, and the body is admitted after the recovery.

## 3. `Native storage capacity exhausted` / elastic image at 4 GiB against 34 GiB
Cause: `elasticMinimumV1` cut the 34 GiB minimum to the host share (34 % of 12 GiB = 4 GiB). The compiler charges a 4 GiB workspace per mode (prototype and release) plus artifacts, so a 4 GiB image can never hold them. The fixed 16 GiB edit in the clone was ignored because `partitions.rust.capacityBytes` is only the envelope; an elastic rule owns the size (`effectiveCapacityV1`, used by `repositoryCacheBudgetV1` and `repositoryStrictStorageV1` alike).

Contract now:
- New rule field `floorBytes` (schema `elasticPartition`; rust 16 GiB = two workspaces, the one that must stay free and cargo home; graph 3 GiB; products 4 GiB). The minimum is cut by the host share only down to the floor. The 34 GiB minimum still applies where the share allows it, so the declared user goal `fleet-host-16g-store-max` (images on a 56 GiB free disk stay under 50 GiB) holds: rust is 19 GiB there. Honouring 34 GiB literally would put that host at 66 GiB.
- Floor or refuse, on every path: a new image (`requireRepositoryPartitionFitsHostV1`: host free below the floor refuses before anything is created); budget and compiler strict storage (same `effectiveCapacityV1`; a test compares them on 7 host sizes against a Python model); a born image below the floor: `requireImageAtFloorV1` in `withRepositoryCompilerStoreOwnedV1` refuses with the bilingual `ImageBelowFloorErrorV1` instead of the late exhausted storage, and `repositoryPartitionRegrowDueV1` marks an automatic image below its floor due for the existing regrow when the host can give the floor (`elasticBelowFloorDueV1`, pure and tested).
- Fixed size: an elastic rule with equal minimum, floor and maximum is a fixed size that no host changes (tested). That is how a fixed size is declared for an elastic partition.
- Fleet scenario `host-16g` (`🛰️fleet/🏃️runner/🟦️.ts`) sizes with the new functions and the darwin need; it passes.
- On a host with 16 to 22 GiB free the private rust image is now born at 16 GiB; below 16 GiB free it is refused with the numbers (matches the free-space finding of the lane report).

## 4. Graph service could not start in the private root
Diagnosis: not a separate defect. `Compiler body and image reader close failed: Native storage capacity exhausted` is raised while the graph service reads the sources of the build implementation through the compiler store of the rust image (`withRepositoryCompilerStoreOwnedV1` -> `repositoryCompilerReservationV1` -> `acquireNativeStorageV1` of the 4 GiB workspace into a 4 GiB image, `native-build/💾️storage/📜️journal`). The 4 GiB image is fix 3; the graph service then only reported an exit-record-less end. Fixes: the floor (image large enough or refused before it exists) and the compiler's up-front `ImageBelowFloorErrorV1` naming size, needed size and remedy in both languages. Wiring is asserted in the budget tests (the compiler source calls `requireImageAtFloorV1` with the rust need).

## Tests run (bun test, absolute paths)
- `budget/🧪️tests/🟦️.ts`: 21 pass, 2 fail. The 2 failures are the pre-existing `the nx partition provisions its own backing...` and its unnamed follow-up (`Actual Native platform image required`), identical before the changes. New tests: fixed need sizing with Python model on all platforms and 6 host sizes; ledger-share need (smallest capacity, Python search); bilingual refusal and born-image exemption; elastic floor, budget vs compiler envelope on 7 hosts, fixed-size rule, below-floor refusal and regrow due, compiler wiring. The disk-policy test model follows the new sizing; its born-within-free law applies to hosts of at least 40 GiB (smaller hosts are refused per image).
- Ledger 15, ledger needs 9, headroom 20, pruning budget 12, reset wait 2, fleet `host-16g` 1, `🪢️writer-control` 1, capacity `🤝️shared` 1: pass.
- Incremental `🧪️tests/🟦️.ts`: 39 pass, 3 skip, 9 fail; all failures are environmental or foreign: `zstd` not installed (Python oracles) and `the command lists, prepares...` expects 4 log lines and gets 3 (prepare step list, not touched here).
- `🏗️compiler/📏️capacity/🧪️tests/🟦️.ts` (real disk images) and `🔒️exclusive-writer`: not conclusive here; they need the canonical suite (`SEMIO_TEST_PYTHON_BINARY`) and the image test timed out at 120 s on the loaded host.

## Not done
- No end-to-end private root run (forbidden for now). After the live build repeat `repo incremental prepare --root=$HOME/.sinc/g3d --fresh` with at least 20 GiB free.
- The stale-holder hardening named in `📓️component-staging-2026-11-23.md` is separate.
