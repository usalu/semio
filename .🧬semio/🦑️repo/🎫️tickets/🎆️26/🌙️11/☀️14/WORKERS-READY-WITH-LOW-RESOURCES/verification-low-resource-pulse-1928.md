# Low-Resource Build Verification at 19:28 UTC

The authority still reports staged tree `7d8c0b059bcaed05be881279fe59b4fa8305d514`. All nine fresh workers acknowledged the exact 11 files from the previous rollout. The former holdout m05 also began reporting native replica digest `b0cc3cfb71fb53cc3d8524013031bc4201285165ba2ed86f01f3873e2a7fa6fa` while its incoming queue still held approximately 5,072 paths. The present code attestation does not measure completion of its workspace process or clear its Windows sharing violations. m04 and m06 remain stale. m09 has no operational attestation. [Missing endpoint evidence](audit-missing-endpoints-1918.md)

The native touch entry accepted the next 11 scoped paths. Each current SHA-256 appeared in an origin journal within its committed head, and all nine fresh workers' cursors reached the corresponding bytes by approximately 19:28 UTC. Those arguments covered physical owner metadata sharing, the cold fact-index serializer, and the fleet install metadata pass. Journal progress and process adoption are distinct. Scoped whitespace checks passed.

## Verified performance changes

- The owner-admission test on 512 native paths reduced stat calls from 1,536 to 512, with median CPU time falling from 12.20 ms to 5.64 ms. The final graph seal still checks independently. The full observation suite passed 69 tests and 382 assertions. A focused graph inventory and owner-admission run passed 13 tests and 256 assertions. The broad run also exposed a stale export-roster fixture, which now matches the two existing exports in both runtime and declarations. [Owner metadata benchmark and tests](audit-owner-admission-metadata-sharing.md)
- The whole fact-index serializer emitted the same Python-verified 2,103,691-byte document in a 2,103,691-byte backing allocation, down from 4,201,850 bytes for that fixture. The native regressions passed 140 tests with 29,603 assertions. That result does not measure whole-workspace peak memory. [Fact index residency](audit-exact-fact-index-residency.md)
- Fleet dependency publication uses a single metadata walk to prove link relocatability and compute the portable fingerprint after the mandatory installed scan. The native fleet install suite passed 27 tests with 329 assertions. m07 had already published a 546,877,446-byte signed pack covering 66,178 installed entries at 19:18 UTC. Its same adopted dependency child continued store ownership work at 19:26:46, with its read counter advancing from 1,570,880 to 1,864,256. There is no new terminal dependency success or current binding receipt. [Fleet install metadata walk](Fleet%20install%20metadata%20walk.md)

## Physical acceptance

| Requirement | Current verification |
| --- | --- |
| Native Ready for all twelve enrolled 16 GiB workers | Zero of twelve. Nine report the exact stage. Eight of those remain in workspace and m07 remains in dependencies. |
| Exact constrained lab | The prior preflight had exited. A new exact 41 GiB, 8,192 MiB, four-core run now waits at the existing disk gate. At 19:28 UTC the wrapper 77618, native ingress 77640 and executing child 77718 were live and the latest gate sample recorded 21,114,179,584 bytes free of the required 60,129,542,144. No grant, backing image, or machine receipt was admitted. |
| Complete compatible current graph and physical adoption | PID 24613 and child 24861 remained live. Native fleet status continued to put the request in resource admission for 14,336 MiB while reporting no settled generation. The independent native read-only adoption verifier returned `pending/receipt-missing` for all twelve destinations. |

The current Rust image had dropped from the earlier 42,833,526,784 bytes used at 19:23 UTC to about 39.9 GB while work was still in custody. The lab's integrated disk waiter will move through the existing capacity gate once physical bytes meet its threshold. The current evidence does not establish enough reclaimable storage or a compatible complete graph on another worker. [Physical disk and custody audit](audit-cold-lab-disk-pressure-1923.md)

[Prior end-to-end checkpoint](verification-low-resource-pulse-1914.md)
