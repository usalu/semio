# 🚀 Wave 1127 Brief: Finish and Optimize the Fleet End to End

Coordinator: main chat (Opus 5.5 medium). Executors: Sonnet 5.5 medium lanes `H*`. Auditors: Haiku 5.5 medium lanes `A*`. Started 2026-11-27 00:05 CET.

`T` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, `W` = `T/wave-1127/`, `S` = `/Users/Shared/semio-store/v1/` (fleet store, `S/origins/<origin>/ready.json`, `ready-log-<stage>[-failed|-stalled].txt`, `memory.json`, `metrics/`).
`B`, `C`, `HC` as in `T/wave-1122/brief.md`.

## Goal
Every live worker Ready and staying Ready on 16 GB machines; one hot fleet cache (nothing computed twice); idle machines take offloaded work; monitoring shows the truth live; nothing fails or hangs when a remote part is down.

## Rules
ALL rules of `T/wave-1122/brief.md` (hard rules 1-12 + addenda) and `T/wave-1125/brief.md` (addenda: dangling imports, atomic writes, no live experiments, `repo typecheck` before releasing a lock, suite tables + project targets, lock hygiene) are binding, with these replacements:
- Reports `W/result-<lane>.md`, findings `W/findings-<lane>.md`, generated output `W/🗑️generated/<lane>/` (delete when done), kept scripts `T/🧰️scripts/<lane>/`, landings appended to `T/landings.md` as `## Wave 1127 <lane>: <title>`, test claims `W/claims-tests.md`.
- Scratch root: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/<lane>/`.
- No sub-agents, foreground only, no background processes, never pkill/killall (kill only pids you started), no git command that modifies anything, never edit AGENTS.md, never open/close/reopen tickets or goals, no sudo, no prompts that wait for a human.
- Landing locks per area: `mkdir T/landing.lock.<area>` + owner file; hold only for the atomic write; release right after.
- Load gate: 1-min load < 40 and swap not full before any test heavier than one file; coordinator Mac is the busiest host (other agents: Antigravity, archivist git signing).
- Fix the root cause, not the symptom. If the fix lies outside your lane but blocks it, fix it anyway when small, otherwise write `W/findings-<lane>.md`.
- Final message to the coordinator ≤ 10 lines: point to the report, name everything unverified.

## Live state 00:00 (11/27) from `repo fleet status --json`
- Workers: ready 8 (m02 m06 m07 m10 m11 m12 m13 m15), working 6 (m01 m03 m08 m14 workspace failing; m04 dependencies inode-short; m05 Windows tools), absent m09 + g1-server.
- Workspace failures: m01/m14 `@semio-tech/framework-rs:generate:prototype`, `framework-actor-rs:typegen:prototype`, `norm-din4108-rs:build:prototype` (Nx 130); m03 `ReferenceError: nativeImageShardMountedV1 is not defined` (bundled code).
- m05: `EPERM unlink …\@esbuild\win32-x64\esbuild.exe` and `Directory partition capacity exceeded`.
- Cache: m11/m12 `unit-cache-denied` (448/409 units expired unshared: credential), g1-server read-only (5053 units unshared). m01 counters hit 82.8k / miss 270k / skipped 77k: spool-full 27.9k, upload-entry-gone 18.0k, upload-failed 14.7k, upload-expired-down 7.9k, key-incomplete 7.3k, upload-expired 611, path-dependent 386.
- Offload: out 6531, refused 895, fellBack 3029; reasons local-slot 12.8k, cheap 5.1k, unmapped-path 3.3k, infrastructure 1.7k, unmeasured 1.1k, pin-failed 676, not-portable 484, worker-policy 235, no-idle-machine 180, queue-timeout 176, incomplete-inputs 165.
- Monitoring: fleet status `source-failed` for readers `storage` and `workers` (no answer within 5 s); generation null.
- Warden: libraries above their 1536 MB ceiling not on the carving list (e.g. `semio_s_artifact_process_process3d_host` 1750 MB on m01); swap-high on m03 m04 m07 m08 m10.

## Addendum 00:15 (11/27): no whole-checkout searches
Load 60-75 on the coordinator is partly our own `rg`/`ugrep`/`grep -r .` over the whole checkout (hundreds of thousands of files). Scope every search to the folders of your area (B, C, HC, the product you touch) and exclude `.cache`, `node_modules`, `target`, tickets. Never run a recursive search from the repo root.

## Addendum 00:40 (11/27): load gate replaced
The coordinator's 1-minute load sits at 50-55 from processes outside the wave (archivist git signing, Antigravity, GitKraken, Activity Monitor); a load-40 gate is never met and lanes idle. Replacement: do NOT wait for load. Gate only on memory: run when `memory_pressure` free ≥ 20 % and swap free ≥ 1 GB; otherwise wait 60 s (max 10 min) and then run anyway. Run every test, typecheck and single-crate build with `nice -n 10`. One heavy run (cargo build, whole-suite test) per lane at a time. No waiting monitors longer than 10 min.

## Addendum 01:40 (11/27): landing slots
Ready fell 9 → 5 because our landings in build-system code keep restarting every worker's workspace stage (same as wave 1125). From now on product landings in B, C, HC, 🏃️process, 🚦️ready, graph and any Cargo.toml/package.json land ONLY in landing slots: minutes :00-:10 of each hour (next: 02:00-02:10, 03:00-03:10, …). Between slots: stage the complete patch in `T/🧰️scripts/<lane>/` (or a scratch copy), run its tests against the scratch copy, and keep working on the next item. Test-only edits (files under 🧪️tests) and docs may land any time. Exception: a fix for a fleet-wide outage (every worker failing) — tell the coordinator in your final message.

## Addendum 04:15 (11/27): graph lock queue in slots
Several lanes land in `graph` each slot (05:00: H9 edge heal, H31 yield, H38 narrow reads, H37). Take `landing.lock.graph` only for the atomic apply (seconds), release it, then typecheck/test and fix forward. If the lock is held > 3 min inside a slot, write `T/landing.queue.graph` (append your lane + time) and wait; the coordinator clears stuck holders. A landing that misses its slot by < 15 min may finish in that overrun.

## Addendum 08:15 (11/27): SEAL QUIET WINDOW
H9's proof read 5 (pid 90730, started 07:05, round 4 code) is converging (stale owners 1225 → 460 → 336). Until the coordinator lifts this window: NO landings in graph or inference-relevant code — B/🕸️graph, B/📥️inputs, B/🗂️workspaces, B/🧩️units, inference/closure/owner code, any `📋️project.json`, `🗂️suites/🔣️.json`, Cargo.toml, package.json. Other areas (caching, host-capacity, remote-execution sidecars, process, tests) may land in the 09:00 slot as usual. Stage graph-relevant patches and keep working.

## Addendum 13:00 (11/27): seal quiet window LIFTED
No whole read runs now. Graph code may land again in hourly slots (13:00, 14:00, …). Staged graph patches land in the 13:00 slot via lane H84 (H60, H78). A new quiet window will be declared when the next proof read starts.

## Addendum 14:10 (11/27): land through `repo land`
From now on every product landing in B, C, HC, 🏃️process, 🚦️ready, graph and manifests goes through `repo land` (MCP tool `land`): stage → `repo land check` → `repo land --wave "Wave 1127" --lane <id> --title "…" <stage>`. Usage and semantics: `W/brief-addendum-repo-land.md`. It runs the gates (debug lines, dangling imports, typecheck, cargo check, suite law, declared tests) on an overlay, waits for the slot, locks only for the atomic apply, writes the rollback point and the landings.md entry. Manual landing scripts are no longer allowed except `--outage` emergencies.

## Addendum 14:40 (11/27): SEAL QUIET WINDOW 2
H84's proof read 10 (pid 48808, started 13:16) is running. Same rule as 08:15: no landings in graph/inputs/workspaces/units/inference code, project.json, suite tables, manifests until the coordinator lifts it. `repo land` users: stage and `repo land check` only for those areas.

## Addendum 16:50 (11/27): FULL BUILD-SYSTEM FREEZE until the seal
The proof read (pid 48808) restarted after the 14:30 and 16:00-16:13 landings in process/caching code: the whole read's closure includes ALL build-system code, not just graph. Until the coordinator lifts this: NO product landings anywhere under B, C, HC, 🧰️framework/🔨️modules/🏃️process, 🚦️ready/sidecars, manifests, project.json, suite tables — also no `repo land` applies and no manual exceptions (the 17:00 exceptions are revoked). Test-only edits under 🧪️tests and ticket files are allowed. Stage everything; it lands in the first slot after the seal.

## Addendum 19:35 (11/27): freeze LIFTED at the 20:00 slot; landing train
The proof read does not converge while peers outside this wave keep editing build-system code (stale edges 1 → 59 → 219 → 292 between waves); holding the freeze blocks ~25 staged landings with no seal in sight. From the 20:00 slot: product landings resume, but ONLY through lane H101 (landing train), which lands the staged queue in order via `repo land`, one changeset at a time, re-checking each against live. Lanes do not land themselves; they hand their stage path to H101 (message the coordinator). The seal becomes a structural task (incremental per-owner publication), not a freeze.
