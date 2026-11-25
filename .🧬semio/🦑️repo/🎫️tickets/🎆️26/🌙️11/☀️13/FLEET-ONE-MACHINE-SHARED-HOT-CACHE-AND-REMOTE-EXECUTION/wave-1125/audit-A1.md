# Audit A1: Open Fleet-Setup Items (wave 1125)

Read-only audit at 2026-11-25 16:30 CET. Live state from `/Users/Shared/semio-store/v1/origins/*/ready.json` (all written 16:28-16:30). "Active" = touched within 6 h (since 10:30 today) or a live worker/lane.

Live census: Ready 1/14 (g1-m07, refreshing workspace). Stage per worker: workspace on m01, m02, m03, m06, m08, m10, m11, m12, m13, m14; dependencies on m04 (waiting) and m05 (running, adopted); m09 has no ready.json and no origin.

## Ordered open items

| # | Item | Source file | Worker / area | Active now (<6 h)? | Owner |
|---|------|-------------|---------------|--------------------|-------|
| 1 | Whole-graph seal stuck at gen 260 (`current.json` quarantine, closure-missing); MCP rebuild pending | `wave-1122/HANDOVER-1122.md` open 1; `wave-1125/coordination.md` 16:40 (F6) | coordinator graph / repo-mcp | Yes (F6 launched) | executor |
| 2 | Workspace stage refused: `crate-refused semio-framework-value-native-receiver-protocol` ("Cargo Source outside admitted owner") in `framework:cargo` source admission | `wave-1125/brief.md` failure class 1; `wave-1125/coordination.md` 16:35 (F1) | m01, m02, m10, m12, m14 (and likely m03, m06, m08, m11, m13) | Yes (F1) | executor |
| 3 | `@semio-tech/ui-styling-tokens:generate:prototype` failing | `wave-1125/brief.md` class 1; coordination 16:35 (F2) | m02 | Yes (F2) | executor |
| 4 | Dependency installer `EACCES` on patch file (`failed applying patch file`), stage `disk-short` / inode-short | `wave-1125/brief.md` class 2; coordination 16:35 (F3) | g1-m04 (Linux) | Yes (F3) | executor |
| 5 | One-time privileged Linux service bootstrap: `repo ... service-bootstrap --group g1 --user deck \| sudo sh` in `/home/deck/semio`, run after m04 replica applies code (logs `service definition differs`) | `wave-1122/result-E64.md` lines 24-29 | g1-m04 | No (since 15:49) | USER (sudo, physical machine) |
| 6 | `disk-unfit` / disk-full (`d`, `m`, `remote-execution` roots) blocking workspace; m01 still full at 15:59 | `wave-1125/brief.md` class 3; `wave-1122/coordination.md` 15:40 and 15:59 (E71, E77) | m01, m08, m11, m12 | Yes (hourly gc --idle) | executor; USER if host disk must be freed by hand |
| 7 | Dependencies attempt 46, stage `dependencies` running/adopted, no Ready yet | `wave-1125/brief.md` class 3 | g1-m05 (Windows) | Yes (live) | executor |
| 8 | m04 dependency stage slow: `Source-aliases` 1.7 s per step, memory-starved; profile not done | `wave-1122/result-E8.md` finding (a) | g1-m04 | Unknown | executor (profile); USER if run on the box |
| 9 | Worker never joined: no `ready.json`, `head.json`, `attest.json`, `pool.json` (enrolled 11-14 only) | `wave-1122/result-E8.md` finding (d); `wave-1125/brief.md` | g1-m09 | No | USER (needs a human at the machine) |
| 10 | Workspace stall: graph owner read stops, no ready receipt. Provisioning marker `native-bootstrap.done` absent; readiness `ready:false` at workspace 5/5 | `☀️21/PROVISION-MAC-WORKER-G1-M13/verification.md` (ticket still `open`) | g1-m13 | Yes (backoff, live) | executor |
| 11 | Workspace must exit successfully on m13 and marker must be written; foreground provisioner earlier exited 3 after a Rustup failure | `☀️21/PROVISION-MAC-WORKER-G1-M13/verification.md` | g1-m13 | Yes | executor; USER only if Terminal provisioning must be re-run |
| 12 | Join-report coordinator items (ticket closed, items still open): preflight probes closed port 6090 instead of `ports.cache` 6094; sealed code snapshot missing a runtime file; stray `replica/sa` and `-inf` paths staged across fleet. The `-inf` path is already staged (`AD -inf` in the session git status) | `☀️25/PROVISION-MAC-WORKER-G1-M14/🎫️ticket.json` summary; `verification.md` | g1-m14, fleet-wide | No (ticket 04:33) | executor |
| 13 | Shared replica round failures 9 to 29; publish p95 up to 183 s; leftover `*.tmp` files in `.cache/f/boot` and store origin | `☀️25/CHECK-G1-M05-REPLICA-SYNC-WITHOUT-LOSING-LOCAL-OR-PEER-CHANGES/sync-check.md` (ticket closed 13:17) | replica daemon, all machines (m05 seen) | Yes (touched 13:14-13:17) | executor (audit first) |
| 14 | Warden coverage: rustc/cargo started by shell scripts, cargo test binaries and Playwright are not warden subjects; native cargo skips host admission; swap not bounded by free disk; packages over the 16 GB unit ceiling must be carved | `☀️24/WARDEN-COVERAGE-FOR-EVERY-REPO-TOOLCHAIN-PROCESS-AND-DISK-BOUNDED-SWAP/🎫️ticket.json` (status open) | Mac workers, host warden | Borderline (last touch 10:05 today) | executor |
| 15 | cad component final needs 8,036 MB charged vs 4,096 MB unit ceiling; closure must shrink; puzzle not measured | `☀️24/FLEET-MEMORY-WATCHDOG-ADMISSION-AND-UNIT-CARVING/🎫️ticket.json` summary (ticket closed 11-24) | unit carving, Mac workers | No | executor |
| 16 | Windows popups from background sync (workfleet, replica) still recurring; issue usalu/semio#2795 | `☀️22/PREVENT-BACKGROUND-SYNC-TERMINAL-POPUPS/🎫️ticket.json` (open) | g1-m05 (Windows) | No (11-23) | executor; USER to confirm on the physical Windows box |
| 17 | `repo test-file` red clusters: offload cluster, workspaces/cargo route, caching files (111 red of 525 as of E43); E83 open: caller-environment "Nx native funding required", 6 unrouted suites, history test pins a missing `rc/<digest>` receipt (decision: history builds its own fixture receipt, E83 DECISION) | `wave-1122/HANDOVER-1122.md` open 5; `wave-1122/coordination.md` 16:37-17:27 | tests (not a worker) | Unknown (E84 launched) | executor |
| 18 | Nx orphan funding reservation and Nx daemon bundle restart decision | `wave-1122/HANDOVER-1122.md` open 4; `coordination.md` 15:27 (E57 retired orphan) | Nx daemon | No | executor |
| 19 | Layout target admits ~1,054 owners vs ~74 declared (E33 says ~2,300 real). Confirm the layout trace fix is still needed | `wave-1122/HANDOVER-1122.md` open 3 | graph closure | Unknown | audit |
| 20 | Farm-child and late-refs owner isolation (E29) not covered; `history` / `playground` private-workspace gaps | `wave-1122/HANDOVER-1122.md` open 8; `coordination.md` 16:45 | graph owner isolation | No | executor |

## Closed or resolved (verified against ticket metadata)

- `☀️19/RESTORE-G1-M10-WORKER-STATE-UPDATES`: closed 11-19. Live m10 is still in workspace backoff (see item 1/2). The worker-state fix holds.
- `☀️20/SERVE-G1-CACHE-ON-BOTH-INTERFACES`: closed 11-20.
- `☀️20/PRESERVE-WORK-WHILE-RECONCILING-WORKER-GIT-STATE`: closed 11-21.
- `☀️22/FLEET-REPLICA-SYNC-HEALTH-ON-G1-M05`: closed 11-23. Old checkout merged. The empty `C:\git\semio` folder is the working directory of a live Claude session (pid 31448), so it is not removed.
- `☀️23/REPLICA-MERGES-STALE-LOCAL-EDITS-AT-APPLY`: closed 11-23.
- `☀️23/SHALLOW-GRAFT-DETACHES-ARCHIVIST-HISTORY-AT-COMMIT-749`: closed 11-23. History reads 749 to 748 again.
- `☀️25/CHECK-G1-M05...`: closed 11-25. m05 in sync. Its manual replica restart (findings-E6) is no longer needed; the open points are item 13.
- `wave-1122` HANDOVER items 7 (DEBUG lines, removed E34), 10 (fseventsd, restarted by USER 12:45), E52 stray caches (16.28 GiB removed), and E57 orphan Nx reservation are resolved.

## Notes

- Items 2, 3, 4, 6, 7, 10 are each covered by a live wave-1125 lane (F1-F5) per `coordination.md`. Do not staff them twice.
- Items 5 and 9 cannot be done by an agent. They need the user (sudo on g1-m04, a human at g1-m09).
- Audit lane A1 only read files and ready receipts. No code edits, no git commands, no tickets opened or closed.
