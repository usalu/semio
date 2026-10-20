# Host Capacity Liveness and Inheritance Review

The actual accounting owner is TypeScript `process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts`, not Go. Read-only store snapshots showed two current-boot grants with fresh heartbeats: exec PID 51886 reserved 3584 MiB/observed 127 MiB; run PID 44358 reserved 9728 MiB/observed 695 MiB. Both recorded boot 1792504809000, matching the reported current boot at 14:00:09Z. A later snapshot showed both modification ages reset, establishing continuing holder heartbeat activity. This does not establish useful compiler progress. No stale-reboot claim or process/store mutation is warranted. The status API was deliberately not invoked because it prunes stale records.

One separate concrete source defect exists: the inherited-grant fast path at line 118 checks valid Grant and live PID only. The normal `readGrants` path at lines 62–67 also requires boot timestamp within the declared tolerance and heartbeat modification age within policy stale milliseconds. A carried inherited ID could therefore adopt a prior-boot or stale grant whose numeric PID is live again, before normal pruning runs. Current observed grants do not exhibit this condition.

The narrow correction is one shared admission predicate for these existing liveness facts, used by inherited admission and ordinary roster filtering. An invalid inherited candidate should fall through to ordinary admission and its existing cleanup protocol; it must not release a live parent's grant, manufacture a new inherited grant or mutate store records from the fast path. Preserve valid current inheritance even when host headroom is full.

Neutral failing-vector proposal, using now 1000000 ms, current boot 800000 ms, boot tolerance 120000 ms and stale window 60000 ms:

| Case | PID live | Recorded boot | Modified | Inherit |
|---|---:|---:|---:|---:|
| Current fresh | true | 800000 | 999000 | true |
| Prior boot, reused live PID | true | 1000 | 999000 | false |
| Current boot, stale heartbeat | true | 800000 | 939999 | false |
| Dead PID | false | 800000 | 999000 | false |
| Exact boot tolerance boundary | true | 680000 | 999000 | true |
| Beyond boot tolerance | true | 679999 | 999000 | false |
| Exact heartbeat boundary | true | 800000 | 940000 | true |
| Invalid Grant shape | true | 800000 | 999000 | false |

These boundary choices preserve the existing greater-than comparisons, rather than silently changing policy. Use the existing neutral schema/fixture and independent SQL conjunction oracle. Execute the actual inherited branch through bounded AST ports: fake filesystem reads/mtime and alive observation only, assert stale candidate never reaches the body as inherited and valid current candidate never queues/samples, and assert no unlink/write/release in the fast path. No host store mutation is needed for this regression. Existing isolated-directory integration tests at gate tests lines 93 and 102 cover normal reboot pruning and valid nested inheritance, but do not combine a stale inherited environment with a reused live PID. No test was run by this auditor.

## Current Narrow Repair Source Review

The current gate calls the same `isCurrentHostGrantV1` predicate from ordinary roster accounting and inherited admission. The predicate validates closed observation DATA, matching identifier, live PID, the existing inclusive boot tolerance, and the existing inclusive heartbeat age. The inherited identifier is checked against the anchored closed schema pattern before directory preparation or grant-path construction. Refused inherited candidates enter the existing ordinary queue; this branch does not delete or release a grant.

The new liveness suite imports the canonical shared build-test invocation helper and extracts the actual full gate function. Its bounded ports stop ordinary admission at the queue boundary, so it proves inherited-branch routing and absence of fast-path body effects, not real queue admission. Eleven neutral rows include both inclusive boundaries, reused PID across boots, stale heartbeat, identifier mismatch/traversal, invalid grant, and extra observation authority. The SQLite predicate is independently expressed. Public barrel export resolves through the existing decision export. No concrete source blocker found; no test body or live grant modification was performed.


Fresh read-only capacity observation: current grants PID41122 show reserved5120MiB/observed457 and PID35471 exec reserved3584/observed8180 both record current boot14:00:09Z (os-derived14:00:10Z tolerance). First heartbeat ages12.2s/17.9s; subsequent stat observation advances recorded below. No status API invoked because it prunes; no process signals or owner inference. Heartbeat activity does not establish useful compiler progress. Current physical gate source has reverted to the earlier inherited validGrant+alive predicate at118 and inline ordinary boot/mtime filtering at67; the previously reviewed shared liveness predicate is absent in this fresh read. Root notified; no production edits.
[{"pid":41122,"advanced":true,"heartbeatAgeMs":14469.86376953125},{"pid":35471,"advanced":true,"heartbeatAgeMs":12820.3017578125}]
