# 🗺 Coordination Plan

Host: macOS, 10 cores, 16 GB RAM, 36 GB free disk, load ~8; at most two concurrent cargo builds at `-j 4`.

## Done
- 00:29 Continued ticket 26/09/01/SOURCING-END-TO-END (duplicate 26/11/27 closed).
- 00:33 Agent rules written (`📓️agent-rules.md`).

## Wave 1
| Agent | Model | Scope | Report |
|---|---|---|---|
| curation | Sonnet | `🗿️artifacts/🗂️curation` Rust workspace: check + test | `curation-rust.md` |
| extensions | Sonnet | `🧩️extensions` beams/slabs/windows + oracle Rust | `extensions-rust.md` |
| typescript | Sonnet | sourcing TS, dev variant, dev-route tests, commands | `typescript.md` |
| audit | Haiku | catalog/descriptor/launch state, prior sourcing tickets, test gaps | `audit.md` |

## Log
- 00:31 Found this open ticket (same task); duplicate 26/11/27 closed; agents redirected here.
- 00:32 Peer pipeline `rebuild-sourcing-retry.sh semio-s-plugin-sourcing-windows-component` (pid 69744) still building windows (3.5 h); it stages, restarts 6081 and runs all journeys afterwards.
- 00:40 Last all-journeys run (11-26 11:55) failed at boot: hub 10:12 carries the live-cleanup `zero-progress` fault (fix in tree 11:53, `🧯️boot-zero-progress.md`). Hub rebuild 16:40 failed on a peer break in `semio-plugin-time-travel-turns` (`observe_turn_funding`), now present in tree.
- 00:42 Queued `rebuild-sourcing-retry.sh semio-hub-sourcing-component 8` (pid 45081, `🗑️generated/rebuild-hub-2026-11-27.out`) behind the build lock; it re-stages, restarts 6081 and runs all journeys.
- 00:58 ✅ audit (`audit-2026-11-27.md`). `sourcing-react` target is generated (`🎮️playground-targets/🟨️.mjs`), not missing.
- 01:00 ✅ typescript (`typescript-2026-11-27.md`): 54 TS tests pass, 0 fail; typecheck + sourcing-js:test queued on host memory (pressure warn, 3.3 GB free, 5 grants of several sessions).
- 01:02 Wave 2: oracles agent (`oracles-2026-11-27.md`): make python oracles runnable, add third-party checks for codecs/filters.
- 01:20 ✅ oracles (`oracles-2026-11-27.md`): 3 python oracles pass via repo test-file (curation mutate, io vs sqlite3/json/zipfile, stock filter/sort/counts); 2 registered in 🔮️oracles; 8 Rust fixture tests written, compile handed to curation agent.
