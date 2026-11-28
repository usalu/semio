# H109: silent ready role counted Ready (g1-m13), self-healing of silent roles

Stage (not landed, hand to H101): `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H109/stage-H109` (10 files, `repo land check` green 11/28 ~02:05: tests, typecheck, dangling imports, debug lines).
Land: `repo land --wave "Wave 1127" --lane H109 --title "A silent ready role is no longer Ready ..." /private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H109/stage-H109` (areas fleet-status, remote-execution, replica sidecars; sidecar closure restarts every worker once).

## Cause (store evidence of g1-m13, no SSH)
- ready.json says `ready:true, state:ready, beatMilliseconds:120000`, updatedMs 14:12; sidecar-code.json (all five roles) 14:12; attest fresh. The roles of m13 stopped together.
- Hole 1 (why H65's watchdog never fired): `readySidecarSilenceV1` treated every ready record as `idle` (owes no beat), but ready roles rewrite the record every 120 s. A ready role that froze or died was never judged silent, neither by the agent watchdog nor by the fleet reader. The reader then took min(record age, replica heartbeat) so the fresh heartbeat of the machine hid the 9 h old record: Ready.
- Hole 2: the daemon sidecar supervisor only reacted to a launcher process that ENDED. A launcher whose pid lives but whose record (written every 2 s) stopped (frozen/stopped process, blocked loop) stayed "running" for good, and with it the agent and its ready child.
- Why all five stopped at once cannot be told from the store; the daemon-level fix (2) covers any frozen launcher, fix (1) covers a frozen/dead ready child under a live agent.

## Fixes
1. `🚦️ready/💔️silence`: a ready (non refreshing) record owes a beat (declared, else `beatByState.ready`); 3 missed beats (6 min) = `silent`: the agent watchdog (H65) SIGKILLs and restarts the child, `ready-wedge.json` bilingual; the fleet reader shows `silent` instead of `ready`. Python oracle + tests updated.
2. `replica/🧷️sidecars`: every 5 s the daemon checks the record of each running sidecar; record older than 300 s (missing: 900 s after start) = frozen: SIGKILL of the process group, normal back-off restart, `lastExit` bilingual. A daemon stall/suspend (> 20 s between checks) skips the check once. Test with real processes (frozen restarted, fresh record untouched).
3. `🚢️fleet-status`: Ready now requires (a) fresh ready record (1), (b) fresh `sidecar-code.json` (< 300 s), (c) ready role code current (stale < 900 s), (d) a registered worker (scheduler machine row, worker not absent, not expired). Otherwise liveness `working`, row field `unready` (role-record-stale | role-code-stale | not-registered), text "not ready: ..." / "nicht bereit: ..." in detail, summary and a new warn finding `worker-unready` (en/de). New reader field `roleCode`, policy keys `workerRoleRecordStaleSeconds`, `workerRoleCodeStaleSeconds`; independent Python oracle extended and seeded; schema closed.
4. Test isolation: fleet-status tests now delete `SEMIO_MACHINE_STATE_DIRECTORY` (on the coordinator they read and BUMPED the real offload counters and bazel slots); two heavy parity tests get a 30 s timeout.

## Unverified
- Not run against the live fleet (not landed). m13 itself still needs its roles to come back (launcher restart on the machine); after landing, the new daemon code reaches m13 only if its daemon accepts the code.
