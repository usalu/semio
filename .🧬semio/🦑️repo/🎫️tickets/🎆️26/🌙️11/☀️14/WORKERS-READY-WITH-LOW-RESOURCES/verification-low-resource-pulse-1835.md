# Worker Build Verification at 18:35 UTC

Nine of twelve workers report the exact coordinator stage `7d8c0b059bcaed05be881279fe59b4fa8305d514`. The coordinator journal contains exact PUT bytes for all 19 changed code and test files in this round. Eight fresh worker cursors cover all 19 and g1-m02 covers 16. A current stage attestation, a journal cursor, and a current Ready receipt are separate evidence. These observations confirm delivery of the stage and most of the new code. They do not establish that already-running processes loaded the new code.

g1-m02 completed its current dependency attempt at 18:27 UTC and entered workspace. The g1-m01 watchdog stopped a stalled workspace attempt at 18:21 UTC. Its own retry began producing fresh checked-owner and verified-memo counts. g1-m07 has another native dependency attempt following a low-space retirement refusal. The earlier mirror capped the error before the observed binding target. g1-m04 and g1-m06 still lack a fresh native report. g1-m09 has no operational report. The three absent reports have no newer independent pool telemetry in the coordinator mirror. [Silent worker audit](audit-silent-origins-1834.md)

The graph service now renews its reload drain from fresh worker observations rather than repeated transport frames. Dependency publication reuses the first authenticated ownership scan and rechecks it under the exclusive gate. The readiness mirror keeps the complete typed binding mismatch within its existing extended refusal limit. The resident and Ready suites passed 129 tests. All 13 producer modes and seven ownership-churn cases passed. The full native producer target was capacity-gated, and there is no physical fleet timing for the new handoff. [Graph drain and readiness evidence](audit-resident-drain-and-binding-evidence.md) · [Dependency handoff](Dependency%20ownership%20handoff.md)

| Acceptance requirement | Observed result |
| --- | --- |
| All enrolled 16 GiB workers show authenticated current Ready | 0 of 12. Six fresh workspace stages and three fresh dependency stages were active at the 18:35 check. |
| Fixed isolated 8,192 MiB, four-core, all-stage run | The fixed 41 GiB APFS run still requires 60,129,542,144 available physical bytes. The backing volume had 26,249,895,936 bytes free. No eligible alternate volume or retained lab image was found. |
| Complete, compatible fleet graph and authenticated adoption on another 16 GiB worker | `repo fleet status` reported no generation. The original exact whole build still waited for 14,336 MiB. Its stored graph identity matches generation 259 but its whole implementation differs. The authenticated verifier reported `receipt-missing` for all 12 destinations. |

[Fixed APFS preflight](audit-fixed-acceptance-image-headroom.md) · [Whole graph authority and capacity](audit-whole-graph-implementation-and-headroom.md)
