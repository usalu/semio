# Result M4: Fleet window in the Rust dashboard TUI

Paths below `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/` (`DASH`) unless noted. Built on M1 charts (`semio-ui-tui-charts`), M2 Rust reader (`read_series`) and M6 fleet scope. `repo dashboard` was not run; everything is proven by render-to-text and daemon tests.

## What landed
- **Data plumbing.** New crate `📊️fleet` (`semio-framework-repo-dashboard-fleet`, std + serde, depends on the M2 reader by path): `Request {range, columns, origin?}`, `Snapshot`, and `read_snapshot(origins_dir, request, now)`. Schema-first: `🧬️schema/📊️fleet/🔣️.json` (ranges, refresh times, policy numbers, overview roles, metric groups are schema data read at runtime). Store root from config, never hardcoded: `SEMIO_FLEET_STORE_ROOT`, else `storeRoot` of `<state>/f/machine.json` (`SEMIO_MACHINE_STATE_DIRECTORY` aware), below `v1/origins`; the view's hello environment wins over the daemon's. No store -> an `unavailable` snapshot. A missing origin (folder without ready.json and rings) and a stale origin (beat older than 2 beats, or rings only) are rows with their own state. No address or OS host name is ever put in a snapshot (law test).
- **IPC.** `ClientMsg::FleetWatch {request}` / `FleetUnwatch`, `ServerMsg::Fleet {snapshot}` (`🌀️daemon/✉️ipc`, schema `🧬️schema/🌀️daemon`, three valid + four invalid protocol vectors). PROTOCOL stays 2 (additive messages; an older daemon answers `decode` and the build-skew notice already names the mismatch) - decision recorded here, change it if you want a hard break. Daemon side `🌀️daemon/📊️fleet`: one reader thread (never on the supervisor turn), only for the watched request, never sooner than the range's `refreshMs` (5m 1 s, 1 h 2 s, 24 h 5 s, 7 d 30 s, 90 d 60 s), sent only when the content changed (or as a 5 s heartbeat so beat ages advance). Compact: overview points are folded into the requested column count (16), drill-down series are thinned per tier run so tier boundaries stay.
- **M6 fleet scope.** Metrics with catalogue `scope: fleet` (queue running/queued, offload in, remote cache entries/bytes) are read from `<store>/v1/fleet/metrics/<source>/` with the source taken from the catalogue (not hardcoded); they feed the fleet facts line and total row and appear in the exec/cache drill-down marked "(fleet)"; origin rows no longer carry offload-in (shows `-/N`).
- **Window.** `Kind::Fleet` / `Body::Fleet` (`🖥️terminal/📊️fleet`), opened with prefix `W` (`show-fleet`, customizable keymap action, en/de label). Overview: one row per origin: status symbol (success/running/waiting/fault glyph), origin id, readiness + stage, beat age, CPU/memory/disk gauges (word `ok/warn/critical` and threshold ticks, so no colour-only meaning), CPU sparkline, units compiled/min sparkline + rate, cache hit ratio local / remote, offload in/out, machine-name warning (triangle + `dup N`/`off`). Stale and missing rows are dimmed (new `ListState.dimmed`, a `DIM` attribute in `🧱️elements/📃️List`) and carry the words stale / missing. Fleet total row (`Σ fleet`, `n/m ready`, mean gauges of live origins, summed rate, ratio of summed hits, summed offload) and a fleet facts line.
- **Zoom.** `1`-`5` or left/right: 5 min / 1 h / 24 h / 7 d / 90 d; each change re-tells the daemon exactly once. The merged multi-resolution series is used (dense recent, coarse older).
- **Drill-down.** Enter on an origin: header, tier ruler (tick at every tier boundary with its age, `now` at the right) and per group (host, build, cache, exec, graph, readiness) one two-row band chart per metric with now/min/max; Esc leaves.
- **Customizable.** `s` cycles the order (problems first, CPU, memory, disk, units/min, beat age, origin), `c` cycles column presets (all, host, build, compact), not persisted (no preference journal field added). Group and metric labels come from the schema / metrics catalogue (en/de data); every other word is in the `fleet_*` entries of the label macro (compile-checked en/de).
- Edits to shared files (all anchored, atomic temp+rename): `Cargo.toml` and `🧰️framework/Cargo.toml` (member line), `🧱️elements/📃️List` (dimmed rows), terminal `windows/panes/controls/sessions/labels`, keymap defaults.

## Proof
- Fleet crate `cargo test` 14/0: ranges and policy from schema, request validation, catalogue ids and en/de labels of every declared metric, store root resolution, unavailable store, synthetic 3-origin fleet (live, stale + name warning, missing), no address leak, overview equals the M2 golden series through the same reader, memory fallback, torn files, fold and thin laws, round trip, three goldens of the overview and one of a drill-down (`📊️fleet/🧪️tests/🧫️fixtures/🏅️golden/*.json`, language-agnostic).
- `repo test-file 📊️fleet/🧪️tests/🟦️.ts` 5/0: the checked-in rings (`🏪️store`) are byte-identical to what the TypeScript ring writer produces; **Ajv** (third party) accepts every golden and refuses broken ones; independent arithmetic on the TypeScript reader reproduces every overview number (cpu, memory, disk, units/min, hit/miss sums, freshness, fleet totals) of the Rust goldens.
- Daemon: IPC round trip + validation unit test, 2 integration tests (a real daemon streams snapshots from the store named by the view's environment, no resend within the heartbeat, drill-down request, unwatch stops; no store answers unavailable; bad requests refused), 39 unit tests 0 failed incl. the protocol vectors; `🧪️tests/🌀️control-plane/🟦️.ts` 12/0 (Ajv validates the new vectors).
- Terminal laws (`🖥️terminal/🧪️tests/🏛️laws`, new `📊️fleet` tests 11/0): snapshot text pinned en and de, drill-down text, symbols+words+dimming, totals, zoom requests sent once, loading/older-range answers dropped, sort/columns, unavailable/empty store, prefix key + real frame, ASCII glyphs. Preferences 26/0, labels 7/0.
- Run route: `repo cargo` wedged 40 min on a cold `syn`/`libc` unit (same as M1 reported), so cargo ran with the repo toolchain without the rustc wrapper, offline, scratch target: `🧰️scripts/M4/cargo.sh test --manifest-path <Cargo.toml>`.

## Text snapshots (80+ columns, `FLEET_TEXT_WRITE=1` rewrites them, fixtures `🧪️tests/🧫️fixtures/📊️fleet/`)
Overview, English, 5 min:
```
Fleet · range 5 min · sort problems first · columns all
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
Origin     Readiness                  Beat     CPU            Memory         Disk           CPU trend        Units/min           Hit L/R     Offload in/out Name
Σ fleet    1/3 ready                           ████▎╎╎ 60% ok █████▏╎ 73% ok ████▎╎╎ 60% ok ▅▅▃▃▂▄▅▆▅▃▂▂▄▅▆· ▅▄▆▄▆▅▄▅▁ 42.9/min  80% / 91%   6/16           ▲ 1
Fleet facts: queue 5 running / 0 queued · offloaded in 6 · remote cache 1.30k entries, 5.05MiB
g1-m03     ! missing                  -        no data        no data        no data        ················ ········· -         - / -       -              ok
g1-m02     ◌ stale · running workspa… 4min 5s  █▋░░░╎╎ 23% ok ████▊╎╎ 67% ok ████▎╎╎ 60% ok ▃▃▃············· ▃▇······· 36.8/min  80% / 91%   0/4            ▲ dup 1
g1-m01     ready                      10s      ████▎╎╎ 60% ok █████▏╎ 73% ok ████▎╎╎ 60% ok ▅▅▃▃▂▄▅▆▅▃▂▂▄▅▆· ▅▄▆▄▆▅▄▅▁ 42.9/min  80% / 91%   0/16           ok
```
Overview, German (header and total):
```
Flotte · Zeitraum 5 Min. · Sortierung Probleme zuerst · Spalten alle
1-5 Zeitraum · ←/→ Zoom · s Sortierung · c Spalten · Eingabe Details · Esc zurück
Herkunft   Bereitschaft               Takt     CPU            Speicher       Platte         CPU-Verlauf      Einheiten/Min.      Treffer L/R Auslageru
Σ Flotte   1/3 bereit                          ████▎╎╎ 60% ok █████▏╎ 73% ok ████▎╎╎ 60% ok ▅▅▃▃▂▄▅▆▅▃▂▂▄▅▆· ▅▄▆▄▆▅▄▅▁ 42,9/Min. 80% / 91%   6/16     
Flottenwerte: Warteschlange 5 laufend / 0 wartend · eingelagert 6 · Remote-Cache 1,30k Einträge, 5,05MiB
```
Drill-down g1-m01, 1 h (first 26 lines, cut at 150 columns; the full text is `detail-en.txt`):
```
g1-m01 · 1 h · ready
1-5 range · ←/→ zoom · s sort · c columns · Enter details · Esc back
                           ┬─────────────────────────┬─────┬───────────────────────────────┬──────────────────────────────┬
                           7min 30s                  5min  4min 30s                        3min                         now
▾ Host
CPU busy share             ▂▂▂▂▂▂▂▂▂▂▂           ▁▁▁▁▁▃▃▃▃▃░░░░░░                     ▁▁▁▁▁▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▂▂▂▂▂ now 60%  min 10%  max 70%
                           ███████████▅▅▅▅▅▃▃▃▃▃▃██████████▇▇▇▇▇▇▄▄▄▄▄▃▃▃▃▃▃▃▃▃▃▃▆▆▆▆▆█████████████████████████████████████
Load average (1 min)       ▅▅▅▅▅▅▆▆▆▆▆░░░░░      ▃▃▃▃▃▇▇▇▇▇▁▁▁▁▁▁                ░░░░░▄▄▄▄▄▇▇▇▇▇▇████████████████▇▇▇▇▇▆▆▆▆▆ now 1.21  min 0.20  max 1.
                           ███████████▅▅▅▅▅▃▃▃▃▃▃████████████████▄▄▄▄▄▁▁▁▁▁▃▃▃▃▃▃▇▇▇▇▇█████████████████████████████████████
Memory available           ▇▇▇▇▇▇▁▁▁▁▁           ░░░░░▆▆▆▆▆██████▇▇▇▇▇▅▅▅▅▅▁▁▁▁▁▁                                           now 4.29GiB  min 4.00GiB  
                           ███████████▃▃▃▃▃▂▂▂▂▂▂▇▇▇▇▇███████████████████████████▆▆▆▆▆▃▃▃▃▃▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▂▂▂▂▂
Swap used                                                                                                                   now 2.00GiB  min 2.00GiB  
                           ████████████████████████████████████████████████████████████████████████████████████████████████
Disk free                  ▇▇▇▇▇▇▆▆▆▆▆▄▄▄▄▄▃▃▃▃▃▃▁▁▁▁▁                                                                      now 372GiB  min 372GiB  ma
                           ████████████████████████████████▇▇▇▇▇▇▆▆▆▆▆▅▅▅▅▅▄▄▄▄▄▄▄▄▄▄▄▃▃▃▃▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▁▁▁▁▁▁▁▁▁▁
Repo tool processes        ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁ now 14  min 10  max 14
                           ████████████████████████████████████████████████████████████████████████████████████████████████
▾ Build
Compilation tasks running  ░░░░░░▂▂▂▂▂▁▁▁▁▁░░░░░░▄▄▄▄▄░░░░░▅▅▅▅▅▅░░░░░░░░░░▃▃▃▃▃▃▃▃▃▃▃     ▁▁▁▁▁▁▁▁▁▁▁▄▄▄▄▄▄▄▄▄▄▄██████████ now 4  min 0  max 4
                           ▅▅▅▅▅▅██████████▇▇▇▇▇▇█████▅▅▅▅▅██████▆▆▆▆▆▆▆▆▆▆███████████▄▄▄▄▄████████████████████████████████
Units compiled                                                                             ░░░░░░     ░░░░░░░░░░░░░░░░      now 0/min  min 0  max 180
                           ▅▅▅▅▅▅▅▅▅▅▅▄▄▄▄▄▅▅▅▅▅▅▄▄▄▄▄▅▅▅▅▅▄▄▄▄▄▄▅▅▅▅▅▅▅▅▅▅▄▄▄▄▄▄▄▄▄▄▄▅▅▅▅▅▅▅▅▅▅▅▃▃▃▃▃▅▅▅▅▅▆▆▆▆▆▆▅▅▅▅▅▂▂▂▂▂
Units restored from the l…                                                                 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ now 0/min  min 0  max 120
                           ▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▄▄▄▄▄▄▅▅▅▅▅▅▅▅▅▅▄▄▄▄▄▄▄▄▄▄▄▇▇▇▇▇▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
▾ Cache
Local cache hits           ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁░░░░░░▁▁▁▁▁░░░░░▁▁▁▁▁▁░░░░░▁▁▁▁▁░░░░░░▂▂▂▂▂░░░░░▂▂▂▂▂▂░░░░░▂▂▂▂▂ now 300/min  min 180  max 
```

## Not verified / notes
- Terminal laws: 14 tests fail in the same crate and are not about the fleet: 10 `sources::*` ("caller-owned source editor artifacts required", needs the repo test harness environment) and 4 docking/tooltip tests (pointer geometry, glyph). Not confirmed pre-existing by a run without my changes (no live experiments); the docking ones touch only window chrome.
- `repo dashboard` never run; real fleet rings do not exist in the live store yet (M3 publication not live), so live behavior is only proven on fixtures. Linux/Windows runs not done.
- Metric label text in the drill-down is metrics-catalogue data (en/de), not the dashboard label macro.
- The partial coarse bucket that straddles a tier switch keeps its visible width, so a ruler may show one extra width change.
