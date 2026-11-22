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

## Update 13:50 (weekly 59 %, coordinator context 77 %)
- MILESTONE: g1-m11 Ready 11:14 (first worker). User restarted fseventsd 12:45 → events 6/6, swap 32.3→27.4 GB.
- Landed since 08:55: E28 Nx split; E29/E36 owner isolation everywhere + withheld re-validation; E33 closure is real (~2,300 owners, per-workspace Cargo); E34 [DEBUG] removed + Nx daemon bundle drain; E35 receipt profileSha256 + crate digest memo; E37/E41 .test-scratch kind, cargo test family green, 5 product bugs; E38/E43 Nx SDK read-only seam, neutrality ratchet, caching red files 127→111; E39/E45 stall watch + custody progress + resumable journal + plan cutoff 2h47→~5-11 min + Rust owner cap 2 + restart causes visible; E40 yield throttle; E42 no duplicate Cargo work exists; E44 m07 lease retry, m02 roster names, m13 purge classifier.
- Whole graph: still unsealed (gen 260). Read 29643 (since 11:45) resumed from stored records, wave 1 82/112 with 39 owners fetched from the fleet. Cold Rust owners cost 22–70 s CPU; host memory is the limit (24 GB serve minimum keeps sealing on the coordinator).
- Tests: B 581/662 (E31) + later fixes; C 111 red files of 525 (E43) — open clusters in result-E43.md.
- Running: E46 (m02 second cause, m13 bun cache co-writer, m10 3 h fleet claim wait, m12 captureClosureProof).
- Coordinator hot fixes today: doctor rules schema, hostUnderPressureV1, dependency partition role remote-execution, cache child restart, CAS collections (2x).
- USER items: m05 manual replica restart (findings-E6.md); stray `.cache` dirs inside source tree (🧰️framework/.cache 8.4 GB) — retire via ledger or delete on your OK.

## HALT at user request (~14:55 CET)
Lanes E47 harness drift, E48 Original-required tests, E49 Nx watcher tests, E50 B open rows, E51 per-scope dependency receipts, E53 stage survival were told to stop safely (finish/revert in-flight edit, record landing, release locks, write `result-<lane>.md`). E52 finished: 16.28 GiB stray `.cache` collected via ledger `strays` rule. E46 finished (see result-E46.md). On resume FIRST check for leftover `T/landing.lock.*` dirs (owner + age) and half-landed edits (`repo doctor`, `repo cargo … metadata`, `repo test-file` on the files each halted lane listed).

## Paste-ready resume prompt (new chat)
Resume wave 1122 of ticket 26/11/13 FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION (goal: finish the build system — low-resource machines, shared hot cache over the remote cache, remote execution, nothing computed twice, all workers Ready). Read first: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/HANDOVER-1122.md` (all sections), `STATUS-1122-1355.md`, the tail of `coordination.md`, `brief.md` with all addenda (per-area locks, Bash reports, no whole-file writes, rule 10 = `repo test-file`), and the `result-E47..E53.md` files of the halted lanes. Then: (1) verify the tree is healthy — no stale `landing.lock.*`, `repo doctor` loads, `repo cargo --store=shared --configuration=prototype -- metadata --no-deps --format-version 1` works, fleet status/ready receipts readable; (2) check `/Users/ueli/Documents/semio/.cache/b/g/m/generations/current.json` (sealed generation?) and `/Users/Shared/semio-store/v1/origins/*/ready.json` (Ready count, only g1-m11 at halt); (3) continue the open items in HANDOVER order with Opus coordinating, Sonnet executor lanes (one test file at a time, load gate 40) and Haiku audits; keep the fleet at 6–8 lanes because the 32 GB coordinator's memory is the binding constraint for the whole-graph seal; retire agents near 500k context and continue with fresh lanes. Act and iterate autonomously.

## Halt confirmed (all six lanes stopped cleanly, no landing locks left)
- E47: `loadProductFunctionsV1` (real-module harness loader); 6 caching files green; corpus row `missing-parent` changed → recheck `reading/`, `producer/`; ratchet law (≈95 files using extracted function text) not written.
- E48: nothing landed; `repo test-file` already builds the suite-runner envelope; 19 "Original required" files blocked by Nx-namespace births, harness drift or test-side defects.
- E49: `nx-watcher-namespace` birth landed; all formerly gated watcher/journal/service tests pass now that FSEvents works; OPEN: orphan `reserved` Nx funding allocation `516cc48f…` in `.cache/b/n/m/storage-events.jsonl` wedges the plain `nx-namespace` birth (needs retirement of orphan reservations); `native-caller` needs `codeRoot:"/repo"` in its fixture.
- E50: 7 B test files green; OPEN: classify `🔬️test-file` member in `scheduling/🔣️.json` and `scheduling/🧪️tests/🧭️boundary/🔣️.json`; real-Bazel test `/tmp` denied inside sandbox.
- E51: nothing landed; staged patch + untested test copied to `T/🧰️scripts/E51-staged/` (per-scope `texts` AND `inputs`, walk stamp /4).
- E53: closure-directory error names the directory (landed). FINDING: supervisor restarts do NOT lose stage attempts (~20 adoptions); attempts end on no-progress caps (m10 graph counter 10/604, m08 84 min in `door.script`, m06 silent after Planning) or real errors → next worker work = those stalls. Staged stale-code-stop hardening in `T/🧰️scripts/E53/` (not applied).
