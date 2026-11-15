# Package N7: One Memory Authority, One Learner, Slice Worker Restart (2026-11-14)

Landings: `T/landings.md`, "Package N7 landing 1" (12:18) and "landing 2" (12:49), T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`. Scripts and scratch copies: scratchpad `N7/` (not in the worktree).

## Verdicts

1. Available memory has one reader: YES. `hostAvailableBytesV1` / `createHostAvailableBytesV1` in `P/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` (P = `🧰️framework/🔨️modules/🏃️process`): macOS `vm_stat` (free, file-backed, purgeable, never more than free+inactive+speculative; the same parser as the capacity gate), Linux `MemAvailable`, Windows and others the OS available physical memory, capped by `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, reused for 5 s, a source that cannot be read answers `undefined` and says so once in EN and DE (never the free-page number). Limits and texts are in the schema (`Limits.availableAgeMilliseconds`, `availableCommandMilliseconds`, message `availableUnread`).
2. `os.freemem()` is gone from every build-system decision (list below). The m06 "143 MB free" cancel cannot recur: the pressure watch and the slice limit now read the same function.
3. One learner, one store: YES. The temp file `semio-slice-peak-<uid>.json` is no longer read or written (a test asserts the source has no such name). The slice peak is class `slice:graph` in the host estimate store (decay, 90 % quantile, minimum 3 samples, package V's outlier rule), recorded only from completed or memory-breached loads, and clamped to `sliceCeilingMegabytesV1` (40 % of the machine, so 6553 MB on 16 GB) at read and at write.
4. A stored history never stops a 16 GB machine: PROVEN (`🧷️history`, 5 histories: none, one 90 GB spike, sixteen, spikes among sane runs, a damaged file; real gate, simulated 16 GB, 7500 MB available): booking 2048 to 6553 MB, admitted every time. The service limit caps the booking where a limit applies without a lending cgroup.
5. Plan memory is paid once: VERIFIED by reading and tests. The run that the service plans for holds a small quiet grant (`🛰️planned-grant`); the slice pays through one `graph-plan` reservation per load (`🧷️history` asserts one reserve per run). No second request beside it; nothing to fix.
6. Exit 75 explained and fixed: 75 is the stale-implementation restart code (`🧷️format-stamps`): a lease yield or a moved format stamp ends the process "so the outermost command starts again". In the slice worker nobody restarted it, so the service reported a failure (g1-m07, a peer's landing moved the code during the round). The pool now restarts the load up to 3 times with an EN/DE progress line (`host-slice-owners-restarted`), then refuses with typed `slice-restarts` (`host-slice-restarts`, retry "later" so the client exits 75 itself).
7. Stale resident service on old code: fixed. The live identity of the host service (`hostImplementationV1`) now includes `hostServiceRevisionV1()`, a digest of the files in `HostPolicy.serviceRevisionFiles` (slice pool, memory, service, schema, sampling). A service started on older code has another identity; the new client starts its own service and the old one retires after 60 s without connections (existing `newerSibling` rule). The sealed digest still wins.
8. Service limit from the grant: the booking is `min(wanted, host share, service limit left)`; the load's learned peak is the load's growth (not the whole cgroup).
9. Real `repo show projects` under `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384`: NOT COMPLETED. It passed capacity (`admitted show: 1 job(s), 3346 MB reserved (16384 MB available, 2048 MB floor)`) and then failed with `ENOSPC` on `.cache/b/g/m` (the graph cache image `/dev/disk15s1` is 12 GiB, 100 % full, 32 MiB free). Not mine; a worker can meet the same.

## Memory Readers Replaced

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.
- LIB/🧠️memory/🟨️.mjs: own `darwinAvailableBytesV1`, `linuxAvailableBytesV1`, `createAvailableMemoryV1`, `availableMemoryBytesV1` removed (free + inactive only); headroom reads the authority; `hostMemoryMegabytesV1` honours the simulated total.
- LIB/🏡️resident/…/👷️slice-pool/🧱️limit/🟨️.mjs: `hostAvailableBytesV1` (with its `freemem()` fallback) removed; ceiling and booking use the shared numbers.
- LIB/🏡️resident/…/👷️slice-pool/🌡️pressure/🟨️.mjs: imports the authority.
- LIB/🏡️resident/…/👷️slice-pool/🟨️.mjs: host size from `hostProfileV1` (was `totalmem()`).
- LIB/⌨️command/🌐️host/🟦️.ts: headroom from the authority; temp peak file removed; learner from the estimate store; host size from `hostProfileV1`.
- LIB/🗓️scheduler/💾️store/🟦️.ts: `freemem()` plus its own `/proc/meminfo` parse replaced.
- LIB/🕸️graph/🧮️derived-nodes/🟨️.mjs: two `freemem()` defaults (heavy-derivation concurrency) replaced; unknown means unlimited.
- LIB/📏️incremental/🏎️bench/🧭️planning/🏃️runner/🟦️.ts: `freemem()` and its Linux parse replaced.
- LIB/🧾️ci-report/🗂️suites/📜️script.ts: `freemem()` replaced (unknown shows the total).
- P/📦️artifacts/🏗️native-build/📈️telemetry/🟦️.ts: last-resort `freemem()` replaced (its macOS and Linux paths already used the shared parsers).
- LIB/🌡️host-capacity/🟦️.ts: wait facts use the simulated total.
- Left alone on purpose: the authority's own `os.freemem()` for Windows and unknown platforms, the sampler's partial-counters fallback, and test files that use `freemem` as a sanity bound. Total-memory (not availability) decisions in `🕸️graph/🤝️prefetch`, `⚡️caching/…/🚏️admission` (nx daemon), `native-build/🎟️admission`, the TypeScript heap in `🎛️commands/✅️verify` and `🔧️tools/🏗️bazel` still use `totalmem()`; they ignore the simulated total (not changed, not availability).

## Tests (all live, after landing)

- NEW `P/🏗️build/🌡️host-capacity/🧪️tests/🔋️available` 12: hand-written `vm_stat` of 16 GB Macs (healthy under load: 186 MB free pages, 6.5 GB available; tight; scarce; Intel 4 KiB pages) against an independent Python parser; `/proc/meminfo` of 16 GB Linux; Windows; simulated cap; cache age; unreadable source EN and DE.
- NEW `LIB/🏡️resident/🧪️tests/🧷️history` 7, `🔁️restart` 3, `🔖️revision` 2 (revision against an independent Python digest).
- Neighbours: sampling 11 + 6, resident booking 3, cancel 37, pressure 7, service 37, scheduler 24 + learn 2, memory available 1; unchanged reds as before the landings: resident `🌐️host` 1, `🏗️provision`, memory main 1 and `🎫️reservation`, scheduler soak client, telemetry, derived-nodes 2 files, bench-planning 1, ci-report suites 1 (identical counts in `baseline-live1.txt` and `baseline-live2.txt` taken before landing).

## What A Worker Sees In The Next Round

A worker that syncs to this code starts a new host service (new identity); the old one (g1-m06 pid 67415, 1.8 GB) retires about a minute after it has no clients. The slice asks for 2048 to 6553 MB (not 7776), is admitted at once with what is free above the 2048 MB floor, and ignores `$TMPDIR/semio-slice-peak-*.json`. If the code changes under the worker while it loads, it sees `the code on this machine changed under the slice worker … attempt 1 of 3` instead of a failure.

## Not Verified

- A real slice load on a real 16 GB machine; the live `repo show projects` (ENOSPC on the graph cache image).
- That the 1.8 GB service on g1-m06 retires (rule exists, was not observed).
- Whether a 16 GB machine finishes the graph step inside the grant (peak of the load 3 GB+ at m06/m07; booking is not the peak).
- Windows memory source (`os.freemem`) on a real Windows host; `vm_stat` failure notice on a real Mac.
- Restart after a real moved stamp (fake worker exit 75 only).
- Every service restarts once at landing and after every later edit of the files in `serviceRevisionFiles`; the resident graph is rebuilt then.
