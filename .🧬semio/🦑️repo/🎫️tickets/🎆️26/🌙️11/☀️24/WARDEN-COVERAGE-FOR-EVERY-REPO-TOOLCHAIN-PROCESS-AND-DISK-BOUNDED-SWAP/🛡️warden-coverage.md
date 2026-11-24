# 🛡️ Warden Coverage: Identity Roots and Disk-Bounded Swap (2026-11-24)

Scope: `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/` (`W/` below). Incident: `🔍️incident-2026-11-24.md`. Nothing was committed; no unrelated process was touched.

## 1. Ancestry gap: identity roots

Before: a process was a subject only when it or an ancestor matched `Roots` (repo Bun runtimes, `semio-build-scheduler`). A pinned `rustc` started by an agent shell was invisible.

Now a process is a subject when it has a root ancestor **or** matches an identity. Identities are schema data (`Identities`), with two anchors (`Anchors`):

| anchor | meaning | where it comes from |
|---|---|---|
| `<workspace>` | alternation of every workspace root | `discoverWardenWorkspacesV1`: every command line starting with `<root>/.cache/b/t/m/(bun\|r)/` names `<root>`; plus the root of the warden's own checkout (derived from its module path). No home directory is hardcoded. |
| `<browsers>` | the repository browser runtime directories | `Anchors.browsers` = `<workspace>/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`; a test proves it equals `repoPlaywrightBrowsersPath(root)` (the default of `ensureRepositoryBrowserPathV1`) |

A pattern whose anchor has no root is inactive. Identities:

1. `^<workspace>/\.cache/b/t/m/r/<hash>/toolchains/<toolchain>/bin/` (cargo, rustc, rustdoc, clippy-driver, ...).
2. `^<workspace>/(?:\S*/)?deps/<name>-<16 hex>` (cargo test and bench executables of any target directory below the workspace; the hashed name is the cargo metadata hash, so it also covers ticket target dirs such as `🗑️generated/native-wasm/debug/deps/`, which are neither named `target` nor `build`).
3. `^<browsers>/` (the repo browser runtime).

`Never`, `protect.json` and `within` still apply to identity subjects. Not subjects: `~/.cargo/bin`, `~/.rustup`, Homebrew, user-level Playwright cache, Google Chrome, the Claude app and its helpers, IDEs (none matches an identity; covered by fixtures).

Kinds (rank = cost to redo, ascending = cancelled first; existing cheapest-first order is unchanged):

| kind | rank | pattern | actions | signal | notes |
|---|---|---|---|---|---|
| local-unit | 20 | `rustc`, `rustdoc`, `clippy-driver` (widened) | suspend, cancel | SIGKILL tree | shell-launched pinned rustc lands here |
| browser | 45 (new) | main process of `<browsers>`, excluding `--type=` helpers and crashpad | suspend, cancel | SIGKILL tree | helpers count into the main process megabytes |
| test-binary | 55 (new) | `<workspace>` hashed `deps/` executable | suspend, cancel | SIGKILL tree | |
| cargo | 65 (new) | `cargo` | cancel only | SIGTERM tree | ranks after every unit and test binary of its tree, so the ladder cancels or suspends rustc units and test binaries first (ties inside a rank: youngest first, then heaviest, as before); cancelling cargo ends the build and takes its rustc with it. Suspending cargo frees nothing, so it is not offered. |

`owner` of a subject below a root is still the root label; a subject without a root is its own owner (`rustc --crate-name ...`).

## 2. Disk-bounded swap

Host facts (schema `Host`, required, `null` = unknown): `swapFreeMegabytes`, `swapVolumeFreeMegabytes`. Read every `Swap.intervalMilliseconds` (2 s) by the default port (`readWardenSwapV1`), not by the shared host sampler (which stays untouched: no ripple to the capacity gate and its fixtures):

- macOS: `sysctl -n vm.swapusage` (free) and `df -k` of the first answering volume of `/System/Volumes/VM`, `/System/Volumes/Data`, `/` (Available column).
- Linux: `/proc/meminfo` `SwapFree` (null when `SwapTotal` is 0); swap does not grow, volume is null.
- Windows: null (observe only).

Law (`headroomHostWardenV1`, schema `Policy` numbers):

```
growable = max(0, swapVolumeFree - swapDiskReserveMegabytes)            (null volume = 0)
room     = min(swapFree + growable, total * swapRoomMaximumBasisPoints / 10000)
headroom = available + room                                              (null when swapFree is null)
headroomGuard  = clamp(total * 2500bp, 2048, 8192)
headroomDanger = clamp(total * 1250bp, 1024, 4096)
level: danger if available < danger || pressure critical || headroom < headroomDanger
       guard  if available < guard  || headroom < headroomGuard || (swap worked or pressure warn) && available < resume
deficit (for suspend/cancel coverage) = max(guard - available, headroomGuard - headroom, 1)
```

`swapDiskReserveMegabytes` 4096, `swapRoomMaximumBasisPoints` 5000. On a 16 GB host: lines 4096 / 2048 MB, room at most 8192 MB. A host without swap reports null headroom and keeps the available-memory law. A property test proves a roomy disk never escalates what the old law called calm (2 GB to 128 GB hosts).

Incident check: 3 GB available, 1 GB free swap, 1.5 GB free disk: old law calm, new law guard (headroom 4000 < 4096); 1.9 GB available with no swap room: danger.

Schema additions: `Policy` (8 numbers), `Swap`, `Anchors`, `Identities`; `Host` (+2 fields), `Decision`, `State` (+`swapRoomMegabytes`, `headroomMegabytes`, `headroomGuardMegabytes`, `headroomDangerMegabytes`), `Alarm.host` (+`headroomMegabytes`, `headroomGuardMegabytes`, `headroomDangerMegabytes`), `Messages` (en and de).

Alarm text: `triggerHostWardenV1` now returns `available | headroom | pressure | swap` in that precedence (the swap/pressure wording bug from `results.md` was already fixed by the other session; this adds the headroom trigger). Alarms append `, headroom X MB (guard Y MB, danger Z MB)` / `, Reserve X MB (Wache Y MB, Gefahr Z MB)` when known; en trigger: "available memory plus swap that can still grow is below the headroom guard/danger line", de: "der verfügbare Speicher plus noch wachsender Swap liegt unter der Wachlinie/Gefahrenlinie der Reserve".

## 3. Tests

Language-agnostic fixtures `W/🧪️tests/🧫️fixtures/🔣️.json` (inputs hand-written, every `expect` written by `W/🧪️tests/🐍️.py`, an independent stdlib-only oracle that reads the schema as data):

| section | before | now |
|---|---|---|
| decisions | 38 | 51 (+13 headroom: guard by full disk, danger, roomy disk calm, no-swap host, boundary at reserve, cap, 4 GB minimums, hysteresis, resume, pressure) |
| classifications | 15 | 22 (+7: shell-launched pinned rustc/cargo/rustdoc/clippy-driver, user rustc and Claude app not, test binaries, browser runtime, no workspace = inactive, protect/never on identities, owner semantics) |
| discoveries | 0 | 3 |
| swaps | 0 | 9 (macOS M/G units, empty, failed, garbage; Linux, no swap, no meminfo; Windows) |

New oracle modes `--discover`, `--swap`; `--write`/`--check` cover all four sections (`--check` passes). Third-party validation: the `jsonschema` check is skipped where the module is not installed (as before); the repo validator checks every record.

Runs (repo bun, `NODE_ENV=test bun test ./<file>`; `repo test-file` not used; logs in `🗑️generated/`):

- warden suite: 27 pass, 0 fail, 1441 expects (20 before, 7 new tests, several extended); includes a MOUNTED test with real children: a symlinked `rustc` under a pinned-toolchain path and a hashed `deps/` test binary, tables with no roots at all, are stopped (`T`) by the ladder; a `rustc` outside the pinned path of the workspace is untouched.
- fleet-status tests (their warden fixtures needed the new required fields): 28 pass.
- admission available test: 4 pass; ready stage warden-hold test: 2 pass.

## 4. Live proof (this machine)

Warden restarted itself on the changed digest: state code `917d755d9c846f84` equals `hostWardenCodeDigestV1()` on disk, pid 20540, mode enforce. State carries `swapFreeMegabytes 1279`, `swapVolumeFreeMegabytes 62352`, `swapRoomMegabytes 8192`, `headroomMegabytes 14456`, level calm. No cargo build was running at the end of the work, so `live-proof.sh` (ticket) starts two real pinned `rustc` from a plain shell (no repo runtime ancestor, blocked on stdin for 14 s). State 7 s later (age 0.3 s):

```
subjects {"count":5, "byKind":{"stage":1,"slice-worker":1,"pool-child":1,"local-unit":{"count":2,"megabytes":60}}}
```

Before the change the same processes were invisible. Overhead of the new warden: p50 25 ms, p95 52 ms, 0.6 % CPU, 76 MB resident.

## Caveats

- The real warden also sees the fake pinned `rustc` of the mounted test (it matches the identity by design) and may resume it after three calm ticks; the test window is under a second.
- The browser anchor assumes the default repo cache layout; if the cache root is overridden by environment, the browser path differs (the equality test would catch a layout change, not an override).
- Heaviest-first inside one rank is kept as the existing order (youngest, then heaviest); no new tie rule.
