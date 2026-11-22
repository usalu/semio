# 🧭 Handover Wave 1122 (08:55 CET, 2026-11-22)

Resume from: this file, `coordination.md` (timeline + agentIds), `brief.md` (rules + addenda 00:56 per-area locks, 01:20 Bash reports, 01:30 no whole-file writes, 06:05 rule 10 = `repo test-file`), `T/landings.md` (every landing with sha before/after). Weekly usage 34 % at 08:55.

## Landed and proven (highlights)
- Fleet cache server: zero-downtime handover (0 failed requests across a real restart, E5); four event-loop blockers removed (E21); services at OS priority 31 via replica self-reprovisioning (E27) → 0 stalls, healthz p50 2.8 ms.
- CAS: cap as host share, detached collection, 30 s tick (E16); coordinator collected CAS twice (97 % disk incidents); compact-on-idle hourly on every worker (E16); rust image 21 → 3.2 GiB.
- Unit cache correctness: key covers every read source (`sources-v2`, dep-info verified) and shared units compile without `-C incremental` (E3); hermetic Cargo config (E4); diagnostics flags out of key; skip reasons recorded.
- Live fleet objectives blocking in bench (E12): unit cache, Nx, Bazel hit ratio 1.0 / 0 re-executed; Nx front breaker (E10).
- Offload: door asks scheduler for headroom, one idle truth `/v1/fleet/idle`, refused offload re-placed (E5); live remote executions observed.
- Graph: per-owner keys with recorded global rows (E1); canonical row order; records newest-wins (E3); farm widening + LRU pinning (E2); owner isolation in selected and whole reads (E29); lazy memo verification + progress liveness (E20); ledger convoy fixed + interactive priority (E17); stale suites JSON import fixed (E29).
- Nx generation split (E28): `nx/<D>` keyed only on platform/package.json/bun.lock/patches/SDK; authorizer bundle separate.
- Memory admission: swap gate needs busy swap + short memory (E7); learned estimates only from calm successful runs, starved refusal after 30 min, dry-run ceiling 2304 MB (E18).
- Robustness: service key keeper vs macOS dirhelper (E23); worker restart debounce + progress-based deferral, stale/offline workers in fleet status (E8); Windows replica EPERM/read-only, rollout bounded under churn, self-heal paths (E6); Nx native shard 160 MiB with stale-shard retirement (E13); doctor storage-images repair (E14).
- Tooling: `repo test-file` (E26), package-export-targets verify gate, MCP enum generator fix (E12, needs MCP rebuild).
- Coordinator hot fixes: doctor rules schema, `hostUnderPressureV1` restore, dependency partition role `remote-execution`, cache child restart.

## Open (ordered)
1. Whole-graph seal on coordinator: read pid 67632 (parent 67467 `repo show projects`) in phase `decisions` (76266/76266 at 08:51); `current.json` still generation 260 quarantine. When sealed: `repo run repo-mcp:build` + `repo run repo-mcp:install` (MCP gets `test_file`, enum fix) — then restart the MCP server.
2. Workers: 0 of 12 Ready. tools-stage break fixed 06:15 (m03 progressed). m05 Windows needs ONE manual replica restart (steps in `findings-E6.md`). m04 Linux slow disk (profile plan in `findings-E8.md`). m09 never ran its daemon.
3. Layout target admits 1,054 owners vs ~74 declared: dynamic edge unidentified; E33 landed `[graph-closure]` trace; layout session asked for the trace lines → fix the edge.
4. Running Nx daemon keeps its start-time authorizer bundle until idle/namespace change (E28) — decide restart-on-bundle-change.
5. Tests: E31 (offload cluster 8/9 red, workspaces/cargo route, ~500 B files unrun), E32 (wire read-only toolchain adoption; ~107 caching files red).
6. Single-target `repo run` still minutes (dependency adoption 52 min at load 40–55; Nx admission ~255 s before split).
7. `[DEBUG]` lines in `repo cargo` output (owned admitted, fleet claim, cargo fetch spawn, views body) — remove.
8. Farm-child/late-refs owner isolation (E29) not covered.
9. Linux root-owned units need one privileged provisioning (E27); server-only cache/relay agents not re-rendered.
10. Host (USER): `fseventsd` 7–8 GB / ~110 % CPU, FSEvents delivers 0 events; swap 27/28.7 GB. `sudo killall fseventsd` or a reboot.

## Running lanes at 08:55
E31 (offload tests), E32 (adoption wiring), E33 (closure size; waiting on trace). All others retired.
