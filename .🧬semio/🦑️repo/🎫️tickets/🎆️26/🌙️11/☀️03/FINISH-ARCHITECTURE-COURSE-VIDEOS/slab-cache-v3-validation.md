# BIM native slab validation, corrected vehicle model

Gate session 55491 uses the corrected model JSON SHA-256 bc4c150e562c0a3c113dbd9433ade854f28d74d7c493dde8a014ace7803e854c, together with the 19 unchanged producer/cache code and manifest inputs recorded in the 20-file generated cache-freeze-v3.json. All 20 matched at launch. Its dedicated cache is 🗑️generated/full-slab-v3; the old v2 cache and proof remain historical evidence for the previous model.

The retained input full-slab-validation.py enumerates all native IfcSlab entries and validates exact typed cold/warm equality, digest equality and a real persistent warm-cache hit per entry. A passing exit and post-run recheck of all 20 frozen input hashes are required.

| Slab | Cold seconds | Warm seconds | Warm hits | Exact digest | v2 comparison |
| --- | ---: | ---: | ---: | --- | --- |
| SL-1 | 36.172 | 21.375 | 1 | b2c76435d641b0984ad0f0c7cec3823f0c02438b78868dfa7abd2f09a9460c79 | matches |

The v2 comparison uses the typed geometry digests in full-slab-verification.md; it does not replace the fresh cold/warm gate.

The gate was stopped and is incomplete; no full-gate pass is asserted.

V3 was deliberately stopped after discovery of stale bilingual model notes. Session 55491 exited 1; the all-ten gate is incomplete. Its SL-1 result above is historical evidence for the prior model, not a v4 cache hit.
