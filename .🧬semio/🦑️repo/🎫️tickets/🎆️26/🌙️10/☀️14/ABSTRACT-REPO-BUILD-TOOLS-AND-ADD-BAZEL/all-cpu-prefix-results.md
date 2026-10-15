# Actual Producer Prefix Profile

Root owned the genuine ALL producer prefix, session 95312, parent 49343 and child 49344. Its original outer envelope remained 1,200,000 ms. A private signal requested cancellation after 180,000 ms; the producer and received scope finished before normal exit. Actual outcome was exit 0 and exact diagnostic cancellation after 180,479.630833 ms, original signal still active. Policy, capabilities, remaining, publish and yield identities were preserved. Qualification was bypassed; this is not current full-workspace acceptance.

The original single-link regular profile was 7,356,185 bytes and metadata-stable after termination. SHA-256 is `8d5e8eb0361c82a4e8f0055afd18d35f6cab7489a156532d2384bdd1a4b5b786`. High's one bounded whole-child aggregation ended with exit 0 in 0.63 seconds. It admitted 22,559 nodes and 107,916 samples. Both independent SQLite self/inclusive checks agreed exactly. Profile span was 180,524,214.25 microseconds; sample delta sum was 180,483,756 with a 40,458.25 microsecond tail. Largest sample delta was 183,117 microseconds.

| Measured Frame | Sample Kind | Samples | Share |
| --- | --- | ---: | ---: |
| Optional ledger inventoryV1, ledger/🟨️.mjs:11:21 | Inclusive | 34,356 | 31.84% |
| readdirSync | Self | 23,444 | 21.72% |
| stringify | Self | 18,312 | 16.97% |
| lstatSync | Self | 11,173 | 10.35% |
| readFileSync | Self | 8,645 | 8.01% |
| Input matcher compile, inputs/🟨️.mjs:12:17 | Self | 6,016 | 5.57% |

The hottest exact leaf stack has 10,843 samples: root → processTicksAndRejections → anonymous → observeInferenceComputationV1 (`observation/🟨️.mjs:485:85`) → withBuildGraphOptionalPublicationV1 (`storage/🟨️.mjs:81:52`) → publish (`ledger/🟨️.mjs:43:40`) → reconcile (`ledger/🟨️.mjs:38:18`) → measure (`ledger/🟨️.mjs:37:16`) → inventoryV1 (`ledger/🟨️.mjs:11:21`) → readdirSync → readdirSync.

The second exact leaf stack has 10,740 samples and reaches the same ledger inventory through observeInferenceComputationV1 → flush (`records/🟦️.ts:146:10`) → writePack (`records/🟦️.ts:74:19`) → withBuildGraphOptionalPublicationV1 → publish → reconcile → measure → inventoryV1 → readdirSync → readdirSync.

The third has 5,838 samples: root → processTicksAndRejections → anonymous → inferProjectInputs (`inputs/🟨️.mjs:123:94`) → map → anonymous (`inputs/🟨️.mjs:190:81`) → compactSourceInputs (`inputs/🟨️.mjs:68:36`) → buildInputPathMatchesV1 (`inputs/🟨️.mjs:11:40`) → compile (`inputs/🟨️.mjs:12:17`). All line and column values identify the profiled Source snapshot, before later edits.

The raw profile and complete aggregation are ticket-generated artifacts, not permanent reports. Sample shares differ from sampled elapsed-delta weights, and neither is a proven CPU-duration measurement. Inclusive shares overlap and must not be summed. The profiler covers this process only and excludes child-process CPU. There were no explicit idle/GC labels, which proves neither absence nor attribution of native/unattributed samples. No phase windows or extrapolated full-ALL speedup are claimed. Concurrent codec-wrapper and OS-MCP neutral-interface repairs are disclosed; profile validity does not establish Source stability or current graph authority.

The resulting work is limited to measured paths. High owns safe ledger accounting, including crash intent, cross-process mutation custody, initialization, maintenance and old-temporary-file cleanup. Core assesses operation-scoped input-pattern preparation against language-neutral and independent minimatch laws. Full Source validation, quotas and original controls remain acceptance requirements. Neither change is accepted merely because this profile identifies a hot path.
