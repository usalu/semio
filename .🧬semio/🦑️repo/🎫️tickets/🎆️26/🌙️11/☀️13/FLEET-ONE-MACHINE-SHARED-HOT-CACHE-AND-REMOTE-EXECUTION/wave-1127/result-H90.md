# H90: line-tables-only debug info for native prototype/test builds; early per-crate memory preflight

Stage-only (freeze 16:50). Nothing landed, nothing in the live tree changed. Stage dir (scratch, ready for `repo land`): `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H90/stage-H90` (backup archive: `T/🧰️scripts/H90/stage-H90.tgz`, measurement scripts `measure.sh`, `warm.sh`, raw rows `warm-rows-*.txt` in the same folder).

```
repo land --wave "Wave 1127" --lane H90 --title "Line-tables-only debug info for native prototype and test builds; memory preflight" <stage-dir>
```
Gates on the overlay (`repo land check`): debug-lines, dangling-imports, typecheck (15.7 s) passed; rust/suite-law not touched. The tests gate fails for reasons of the overlay, not of the change (see Findings): the live `repo land` will refuse until H74/H89 fix them, so run my tests by hand in the overlay as below.

## 1. Test builds keep only line-level debug information
### Where profiles are rendered
`🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts` (`renderCargoProfileConfigurationV1`) renders `profile.<mode>.<key>=<json>` selectors from the authored table `🧬️schema/🔣️.json` (`PolicyProtocol.settings`), exactly as `trim-paths="object"`. Tests are not a separate profile: `repo test` builds under `prototype` with role `native`, so `settings["prototype-native"]` is the dev-test profile (the workspace Cargo.toml has no `[profile]`). Before: `debug: 1` (limited), not full as the proposal assumed.
### Change (staged)
`prototype-native.debug: 1` -> `"line-tables-only"`; `Settings.debug` enum gains it (host settings stay 0/1); TS type widened; policy-case corpus updated. Wasm, descriptor, release and the build-override (host) keep their levels. No new `split-debuginfo` key (measured, below). No `outputDeterminism` bump: `-C debuginfo` is in the compiler arguments, hence in the unit key, so a unit built with another level never shares an entry (test below); a bump would only invalidate every wasm/release unit for nothing.
### Measurement (scratch, nothing in the repo): synthetic crate, 1500 derive+serde+toml+syn tests (test binary 48 MB), 6 registry deps, `nightly-2026-07-20`, macOS arm64, opt 0, cgu 256, load 28-45 and swap full on the coordinator
Full non-incremental recompile of the test crate + link, interleaved reps (user CPU s, median; peak RSS MB median; `repo` scripts `warm.sh`):

| level | user CPU | user+sys | peak RSS | link_binary | target dir | test binary |
|---|---|---|---|---|---|---|
| A `1` limited (before) | 53.1 | 57.2 | 2096 | 0.5-0.9 s | 1259 MB | 49.1 MB |
| B `line-tables-only` (after) | 50.4 | 54.2 | 2376 (2.0-2.7 GB, noisy) | 0.6-1.8 s | 1246 MB | 47.7 MB |
| C B + `split-debuginfo=off` | 49.9 | n/a | 2530 | 0.7-0.9 s | 1222 MB | 47.7 MB |
| D `2` full | 54.5 | 58.5 | 2831 | 0.8-1.2 s | 1388 MB | 49.3 MB |
| E `0` none | 47.7 | 50.9 | 2422 | 1.1-1.4 s | 1131 MB | 42.7 MB |

Reading: line-tables-only saves about 5 % compiler CPU against the current `1` and 9 % against full, 1 % target size against `1` (10 % against full). Link time and peak memory do NOT move on macOS: with the platform default `split-debuginfo=unpacked` the DWARF stays in the object files and the linker only writes a debug map, so the 1 s link is the same for every level, and peak RSS is dominated by the LLVM back end (differences are inside the noise of 2.0-2.7 GB). `split-debuginfo=off` saves 2 % disk and no CPU: NOT adopted. Honest limit: the proposal's real win (link time and size with DWARF inside the binary) is a Linux/Windows effect that cannot be measured here (no Linux host in this lane); expect it there, unverified. Cold-build wall times (A 180 s, B 77 s, C 74 s, D 91 s, E 124 s) are load noise and not used.
### Tests (written first, language-agnostic)
`🎚️mode/🦀️profile/🧪️tests/🔬️line-tables/` (4 pass): every case (native build/test/check on three targets, descriptor, component, release) gets the level the schema table gives it and exactly one `profile.<mode>.debug=` selector; Python oracle (`🐍️.py`) reads the schema table independently; the real pinned Cargo (`nightly-2026-07-20`) accepts the rendered selectors and reports `debuginfo: "line-tables-only"` in `--unit-graph` (third-party validation); `unitKeyV1` differs for `-C debuginfo=1` vs `line-tables-only`.

## 2. Early per-crate memory check
### Design (schema-first)
New module `⚡️caching/🦀️cargo/🗃️unit-cache/🧠️preflight/` with schema `🧬️schema/🔣️.json` (policy bounds, plans, bilingual texts en/de, row definition). Before the first compiler of a compile-kind Cargo launch starts, the launcher calls `reportMemoryPreflightForCargoV1` (one staged line after the `spaceReport` definition, `launcher/🟦️.ts`). It never fails a build (any error prints one bilingual `unavailable` line; the build goes on: H91 fail-open law).
- Peaks (highest per unit wins): build telemetry of this machine `components/build-telemetry/units-<platform>-<arch>.jsonl` (`peakRssBytes`, rows of the requested build mode; own tolerant reader: the admission reader needed 3 s and a 0.45 s import), the host warden alarms (`lastAlarms` reason `ceiling` of the local `memory.json` and of every `S/origins/*/memory.json`, not older than 7 days), the carving list.
- Scope: the dependency closure of the packages named with `-p` (from the `Cargo.lock` of the manifest or the workspace, hyphen/underscore normalised); no `-p` or a root missing in the lock: every known unit.
- Plan per unit above the budget of its kind, by the admission broker's own `decideGrantGrowthV1` (so the preflight can never disagree with the running build): `constrained-threads` (library this machine can hold: one front-end thread, H6), `serial-backend` (linked final it can hold: one front-end thread + serial back end, H49), `beyond-host` (more than this machine can ever hold; queued for the larger fleet machine, named with its megabytes), `unbuildable` (no machine of the fleet holds it: carve first). `waits` when free memory is short now.
- Output (stderr, language from the launcher's resolution; at most 20 rows, then a count): header with count, host total and holdable MB; one line per unit with peak, budget, source and the plan.
### Real-data run (coordinator, 32 GB, `build` without `-p`, 222-269 ms): 81 known units over budget, all `constrained-threads` (this host holds up to 20480 MB; the largest listed peak is 9306 MB); sample en/de output shown in the run log. Note: units known only from the carving list carry no crate type, so finals such as `*_component` are shown as libraries until telemetry or a warden alarm names them `final`.
### Tests (`🗃️unit-cache/🧪️tests/🧠️preflight/`, 7 pass in the overlay)
Corpus of 10 plan cases (below budget, grows, waits, final, beyond-host with known and unknown fleet, unbuildable, scope, ordering, listed, wasm target) + telemetry/warden/merge/lock-closure/lines cases; Python oracle `🐍️.py` recomputes everything (growth law, peaks, closure with `tomllib`, else a block regex; texts from the schema) and equals the TS output; lock closure also equals `@iarna/toml`; en/de placeholder parity; carving list of the real policy plans; end-to-end read with a torn telemetry tail, a failing source and the bilingual `unavailable` line; telemetry path equals `localUnitHistoryPathV1`.

## Findings (for H74/H89, the `repo land` gate)
1. `repo land check` tests gate: the overlay has no `🌎️hub/Cargo.toml` (root manifests), so two existing laws of `🎚️mode/🦀️profile/🧪️tests/🟦️.ts` (`current Cargo profile vocabulary…`, `current prototype host inheritance…`) fail with ENOENT in ANY landing that touches that folder; the other 9 of 11 pass with my corpus change.
2. The declared tests of the suite `cargo-unit-cache-processes` fail in the overlay with `A directory of the native tools is not a plain directory: …/.cache/b/t/m` (also the untouched `unit-cache/🧪️tests/🟦️.ts`), and a NEW test file is looked up in the live tree ("No such file"), so a landing that adds test files cannot pass that gate.
3. Verification done by hand instead: bun in the overlay (`/Users/ueli/.cache/semio/l/3f08bb/H90`, repo-managed bun): preflight 7 pass, line-tables 4 pass, profile `🟦️.ts` 9 pass + the 2 overlay ENOENT.

## Unverified
Linux/Windows link time and size; peak RSS effect of the level (noise); live launch output (launcher hook typechecks, ran only through the module entry point in the overlay, not through a real `repo cargo`); that the new test dirs are admitted by the suite table (`suite-law` skipped: no suite table edit; new test folders may need a `suites` membership check after the seal).
