# 📈 Brief: Fleet Monitoring With Multi-Resolution Metrics

Coordinator: main chat (Opus 5.5 medium). Executors: Sonnet 5.5 medium lanes `M*`. Auditors: Haiku 5.5 medium lanes `MA*`. Started 2026-11-26 02:40 CET.

`M` = this ticket folder. Fleet ticket rules apply: read `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1125/brief.md` (rules + all addenda: per-area landing locks in that ticket's folder `T/landing.lock.<area>`, atomic writes, dangling-imports check, no live experiments, `repo test-file` one file at a time) — they are binding here, except: results go to `M/result-<lane>.md`, generated output to `M/🗑️generated/<lane>/`, kept scripts to `M/🧰️scripts/<lane>/`, landings ALSO into the fleet ticket's `landings.md` with header `## Monitoring <lane>: <title>`.

## Goal (user, binding)
Complete monitoring of the fleet: CPU, memory, disk, compilation tasks, graph reads, cache hits (remote and local), remote execution, readiness, etc. Visualized in the `repo dashboard` TUI. Sample rate NOT homogeneous: short windows dense, older data progressively squashed (tiered downsampling). Full system across all workers (macOS, Linux, Windows), minimal overhead, fully end to end, zero-touch. Never block on user input; reclaim disk / kill zombie processes yourself when needed (only processes of this repo's tooling, never user apps).

## Extra rules
- Agents MUST NOT run `repo dashboard` interactively (CLAUDE.md). Verify the TUI with its headless render/test harness only.
- Machine identity = origin id; no IPs or OS host names anywhere (see fleet ticket `wave-1125/design-fleet-naming.md`).
- Schema-first for every new format (metric catalogue, sample record, tier layout, wire/store files); en/de texts for every user-visible label.
- Overhead budget per worker: sampler < 0.5 % of one core average, < 30 MB RSS, store writes bounded and append-friendly; measure and report.
