# Audit A3: Verification of backlog section 9 and M4, L13, L19, L20, L21, E7

Read-only lane A3, wave 1127, 2026-11-27 ~00:53 CET. Sources: live store `/Users/Shared/semio-store/v1/origins/g1-*/` (ready.json, ready-timings.json, ready-log-*.txt), ticket `landings.md`, `coordination.md`, `wave-1127/audit-A1-backlog.md`. Nothing edited.

Status key: CLOSED = landed and no contrary live evidence; STILL OPEN = contrary or unresolved evidence; UNKNOWN = not decidable from store or landings.

## M4: live stall counts per origin (ready-timings.json, last 40 runs per origin)

| Origin | Runs | Stalled | Stalled stage | Last stall (CET) |
|---|---|---|---|---|
| g1-m01 | 40 | 2 | workspace | 11-26 21:05 |
| g1-m02 | 40 | 1 | workspace | 11-26 16:35 |
| g1-m03 | 40 | 3 | workspace | 11-26 20:25 |
| g1-m04 | 40 | 0 | - | - |
| g1-m05 (windows) | 40 | 12 | dependencies | 11-25 04:04 |
| g1-m06 | 40 | 0 | - | - |
| g1-m07 | 40 | 9 | workspace | 11-26 20:28 |
| g1-m08 | 40 | 11 | workspace | 11-26 22:30 |
| g1-m10 | 40 | 19 | workspace | 11-26 22:29 |
| g1-m11 | 40 | 0 | - | - |
| g1-m12 | 40 | 2 | workspace | 11-24 23:40 |
| g1-m13 | 40 | 10 | workspace | 11-26 21:37 |
| g1-m14 | 40 | 3 | workspace | 11-26 20:36 |
| g1-m15 | 21 | 3 | workspace | 11-26 21:34 |

Total 75 stalled of 541 recorded runs (about 14 %). Only 1 of 75 stalls carries a named watchdog resource (`nx:source-verification`, m08); the rest have none. No pre-F5 baseline exists in the store, so the trend is UNKNOWN.

**M4: STILL OPEN.** The counts are now measured, and stalls still recur on 11 origins within the last 4 hours.

## Section 9 (C1-C8)

- **C1 (`#[path]` debug mounts): CLOSED.** F1 landed (landings.md 7429, 8305, 8384). Live: 0 `crate-refused` hits in any origin log or ready.json. Note: the backlog cites "coordination 16:43", which is not in the ticket `coordination.md` (only the 15:00 entry at line 53 mentions the refusal).
- **C2 (Windows spawns hidden): UNKNOWN.** F10 (landings 8247) and F17 (8331) landed. The only Windows origin is g1-m05, which is wedged (ready.json: "still working after 9372 s; last phase started: source.served"), so hidden spawns were not observed live.
- **C3 (owner rows, owner registry writer): CLOSED.** F35 train F31 (landings 8476), F53 (8637), F115 writer (9041). No live recheck in the store.
- **C4 (pid + startMs identity; F56 baseline): CLOSED.** F41 (8527), F42 (8545), F45 (8556) landed. F56 baseline reached 1 file (landings 8722, "baseline 4 -> 1 file").
- **C5 (resolver, origin naming, per-launcher lease): CLOSED.** F47 (8572), F50 (8617), F52 (8623) landed. Scheduler re-registration (E5) is a separate open item.
- **C6 (closures main, units, native pool): CLOSED.** F76 (8869), F81 (8890), F93 (8942), F94 (8933) landed.
- **C7 (F97 inventory via change journal, warm 0.15-0.42 s): UNKNOWN.** F97 landed (8970). The warm timing figure does not appear in its landing entry, and no live timing was found.
- **C8 (library typecheck 0, F95/F99): UNKNOWN.** F95 reports library whole 125 -> 0 (8953-8955). But the F97 and F102 entries (8975, and the F102 entry just above it) record 2 open errors in `process/testing/cargo/tests/admission`. That contradicts "0". A fresh `repo typecheck` is needed to settle it.

## Items M4, L13, L19, L20, L21, E7

- **L13 (pre-inventory labels): CLOSED.** Live logs show `inventory of the tree (attempt 1|2|3)` (about 50 lines) and `source.capture (N files)` host-wait text. F25 landed 17:40-17:52 (landings 8429).
- **L19 (BUILD_SOURCE_CONFLICT on m02/m15): CLOSED.** 0 hits for `BUILD_SOURCE_CONFLICT` in any origin ready-log or ready.json. F58 landed 02:36 (landings 8656). Caveat: the logs cover only the current run window.
- **L20 (m12 "Exact live repository partition cache owner required"): UNKNOWN.** 0 hits anywhere in the store, and F22 split the message into four refusals (landings 8397). But m12's current workspace log covers only the present run, so absence is weak evidence.
- **L21 (m12 closure index descriptor, F75): UNKNOWN.** F75 landed (landings 8861). No live mention of `descriptor` in any origin log, and no live recheck is possible from the store.
- **E7 (Go launcher test): CLOSED for the failing test.** F29 fixed `TestLauncherReadsItsStateBelowTheDeclaredDirectory` (landings 8445-8448); `repo run @semio-tech/repo-launcher-go:test` exit 0 at 18:05. The slowness (nx 54.9 s after about 12 min of waiting) was not re-measured.

## Side findings (not in scope, for the coordinator)

- F74 lease mapping re-entrancy was explicitly NOT landed (landings 8867), so Q2/E10 remains open.
- g1-m05 (windows) has been in `tools`/`adopted` with "still working after 9372 s" (ready.json), which bears on L15/M8.
- `norm-din4108-rs:build:prototype` (76 log hits) and `framework-rs:generate:prototype` (5 hits) still appear in live workspace logs (L17 is still live).
