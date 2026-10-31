# Current Capacity Parent Audit

The official capacity snapshot is timestamped 2026-10-29T17:58:57.636000+00:00. Bounded Linux parent inspection ran at 2026-10-29T17:59:47.361620+00:00. All37 snapshot waiter PIDs and all5 snapshot active grant PIDs had readable current stat records. None of the37 waiter parent chains contained any of the five active grant PIDs. This census therefore provides no evidence that an active grant is retaining its reservation while its own descendant waits for a separate capacity grant.

Each chain started only from a numeric waiter PID in the official snapshot, read /proc/PID/stat through at most64 ancestors, parsed after the final closing parenthesis, and retained PID/parent/state/starttime. No process enumeration, ps, Native command, signal, lease/registry access or mutation was performed. Cmdline was read only for the five authorized grant PIDs; there were no proven related waiters requiring cmdline inspection. Eval/code arguments and environment were not emitted; only executable and script basenames were retained.

| Active grant PID | Snapshot owner / role | Reserved MiB | Snapshot observed MiB | Current executable / script basenames |
|---|---|---:|---:|---|
| 1218516 | @teaching/architecture-course:dev | 5946 | 2620 | bun / 📜️script.ts |
| 1346889 | @semio-tech/framework-os-mcp-rs:build | 2934 | 3482 | bun / 📜️script.ts |
| 1358440 | command | 2048 | 241 | bun / 📜️script.ts, daemon.ts |
| 1361861 | command | 2048 | 98 | bun / 📜️script.ts, probe-server.ts, handler.ts |
| 1335228 | command | 2048 | 245 | bun / 📜️script.ts, watch.ts |

All five observed grant processes were sleeping (S) at their individual stat reads; this does not prove inactivity or a blocked child. Maximum observed waiter chain length was 8. The requested oldest dependency waiter1364967 chain was `[1364967, 1364966, 1364965, 2339, 1]`, with no active-grant ancestor.

The official snapshot reports available5855MiB, headroom3874MiB and normal pressure, with its leading6144MiB request refused for memory and later requests refused for queue order. These are snapshot admission facts, not fresh policy reevaluation. The low98–245MiB observed values of three command grants do not by themselves authorize shrinking their2048MiB reservations, stopping their services, changing fairness, or bypassing capacity.

Parent relationships are sampled sequentially and can change during the census. Snapshot grants do not contain process startTicks for independent exact PID incarnation matching; current startTicks were recorded but cannot establish historical identity at the prior snapshot. No missing process or stale ledger inference is made. Existing authenticated admission/owner lifecycle remains the authority for release.

Reference-only sanitized evidence: `🗑️generated/capacity-parent-census.json`. No owners were stopped and no jobs/tests were launched.

## Fresh Capacity Parent Relationships 2026-10-29T20:34:16.450Z

Read-only /proc stat ancestry of all 8 official snapshot waiters, bounded64parents each. 0 waiters have an ancestor PID listed as an active grant. The snapshot and process reads are separate point-in-time observations; a matching ancestor is a potential nested-custody issue, not automatic proof of a deadlock or authority to signal a process. No processes/leases/capacity settings changed. Raw metadata: 🗑️generated/capacity-current-ancestry.json.

- 1584976 test: active grant ancestors none
- 1585001 @semio-tech/plugin-instance-typed-ops-rs:test: active grant ancestors none
- 1502252 @semio-tech/course-react:test: active grant ancestors none
- 1505202 command: active grant ancestors none
- 1493176 @semio-tech/framework-os-mcp-rs:build: active grant ancestors none
- 1562273 command: active grant ancestors none
- 1534497 sync: active grant ancestors none
- 1585009 check: active grant ancestors none
