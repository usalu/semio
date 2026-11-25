# 🛡️ Host warden: measure what the kernel charges, not the resident set (2026-11-25)

## Cause

The warden table is one `ps -A -o pid=,ppid=,pgid=,rss=,...`. The subject megabytes are the sum of `rss` over a subject's tree. On macOS, `rss` counts pages that PartitionAlloc already freed but left mapped (`MADV_FREE_REUSABLE`) as resident, so a headless Chrome tree with a 2.8 to 3.1 GB `phys_footprint` read 2.98 to 5.05 GB and was SIGKILLed above the 4,096 MB `browser` cap. The loop did have a footprint probe (`ports.footprint`), but it ran only for cap subjects (at most 8, every 5 s) and merged with `max(resident, footprint)`, so an over-stated resident set always won. The cap was measured in a unit that is not what the kernel charges.

Note the measure cuts both ways: macOS compresses memory, so `rss` also under-counts compressed pages. On this machine (loaded) the Claude app tree reads 5,647 MB by `rss` and 11,264 MB by footprint; the whole machine 7,196 MB vs 15,864 MB. The footprint is the honest charge; no cap was raised.

## Chosen measurement per OS

| OS | Warden table row (`memoryKilobytes`) | Cost |
|---|---|---|
| macOS | `phys_footprint` of every readable process through `proc_pid_rusage(pid, RUSAGE_INFO_V4)` via `bun:ffi` on `/usr/lib/libproc.dylib`, offset 72 of `rusage_info_v4`; a process the kernel refuses (other users, root) keeps its `ps` resident set | 0.84 ms for 526 rows (about 2 us per process); the whole table including `ps` 62 ms on a load-19 machine |
| Linux | `Pss + SwapPss` of `/proc/<pid>/smaps_rollup` for processes whose resident set reaches `chargeMinimumKilobytes` (32,768 KB), others keep `rss` (honest charge: RSS counts shared pages once per process, e.g. a Chrome zygote) | not measurable here (no Linux host); the kernel walks page tables per read, hence the threshold and the parallel reads; the same read already served `observeTreeMegabytesV1` |
| Windows | private usage (`K32GetProcessMemoryInfo` `PrivateUsage`) instead of the working set, in `readHostProcessRowsV1` (feeds the warden table and the unmanaged census) | unchanged (same call) |

Options measured on macOS (this machine, loaded):

- `footprint -f bytes --noCategories -p ...`: about 200 ms for 28 pids, 960 ms for a 300-process tree; it was the previous `observeTreeMegabytesV1` path.
- `vmmap --summary`: slower still, not tried at tree size.
- `proc_pid_rusage` through `bun:ffi`: 1 to 2 ms for all 478 pids (272 readable), 18.7 ms for the whole `observeTreeMegabytesV1` call (dominated by its own `ps`). Values equal `footprint(1)` byte for byte on 6 processes (verified) and within 0.5% on a live tree.

The `footprint` tool stays as the fallback when the native port cannot load (outside bun).

## Interface and consistency

- New module `🍏️native-footprint` (schema `Layout`, typed port `DarwinFootprintPortV1`, injectable kernel), mirroring `🪟️native-memory`. External API stays behind the port; no new dependency.
- `chargeWardenRowsV1(platform, rows, ports)` in the warden loop replaces the `ps` resident sets in the default `table()` port. Caps keep their meaning in megabytes and are not changed. `measure()` (the cap probe) is unchanged: with the table now in the same unit, `max` only guards under-count.
- `ProcessRow.rssKilobytes` renamed to `memoryKilobytes` (schema, parser, loop, oracle, fixtures) because it no longer holds the resident set. The unrelated sampling `ProcessRow.rssKilobytes` is untouched.
- Other consumers of the same measure: `observeTreeMegabytesV1` (gate, test-worker learning, file budgets, capped run, ceiling tests) now uses the native macOS port, same unit as before, 50 times cheaper; `readHostProcessRowsV1` on Windows uses private usage; the host-capacity product re-export is unchanged. No other reader of `ps rss` found (`grep rssKilobytes` outside the warden and sampling is only unrelated test/ticket copies).

## Tests and results

Run with `bun test` (the existing suites' runner), not cargo.

- Language-agnostic: `🛡️warden/🧪️tests/🧫️fixtures/🔣️.json` key `charges` (5 cases: macOS footprint replaces resident set, refused process keeps resident, zero-byte footprint, Linux threshold and missing `Pss`, Windows untouched), expectations written by the Python oracle `🛡️warden/🧪️tests/🐍️.py --charge` (`--write`/`--check` green). `🍏️native-footprint/🧪️tests/🧫️fixtures/🔣️.json` (5 rusage buffer readings validated by Ajv against the schema).
- TS: `chargeWardenRowsV1` equals the oracle and stored expectation; the regression (5,050 MB resident tree over the 4,096 cap, 3,050 MB by footprint, under it); live macOS read equals `footprint(1)` (third-party oracle) for a spawned child, null for an exited pid, under 0.5 ms per process; `observeTreeMegabytesV1` with a fake port and with a port that reads nothing (tool fallback).
- Results: native-footprint 5 tests, sampling (incl. Linux, footprint, unmanaged, windows suites) all pass; warden suite: 47 to 49 pass, all new tests pass. The `MOUNTED:` tests (real children, SIGSTOP/SIGKILL timing, they use their own `realTable` port) pass in isolation under normal load but failed in 1 to 2 cases at load average 15 to 19 (this machine is shared and busy); they do not touch the new code.
- Not verified: Linux cost and live Linux behavior (fixture-driven only); Windows live (fake kernel only).

## Files changed

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🍏️native-footprint/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (new)
- `.../📡️sampling/🟦️.ts` (native darwin path, `readLinuxChargesV1`)
- `.../🛡️warden/🔁️loop/🟦️.ts` (`chargeWardenRowsV1`, table port), `🛡️warden/🟦️.ts`, `🛡️warden/🧬️schema/🔣️.json` (rename, `chargeMinimumKilobytes`, docs), `🛡️warden/🧪️tests/🟦️.ts`, `🐍️.py`, `🧫️fixtures/🔣️.json`
- `.../🧺️unmanaged/🟦️.ts` (Windows private usage)
- `.../🧪️tests/📡️sampling/🟦️.ts` (explicit null port, new native test)
- Ticket: `warden-footprint-bench.ts` (benchmark script).
