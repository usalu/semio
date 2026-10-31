# Exact Opaque Visibility Signature Census

Current canonical source:140 slots processed individually, discarded after hashing. No _opaque_visible, ordering, BSP, Cairo or full-tower build was called. Normal repo admission, exit0,32.109s; all renderer/model/data SHA256 endpoints unchanged. Retained audit-visibility-signatures.py and generated/visibility-signatures.json contain complete per-slot data and exact endpoints.

The digest reproduces the current _opaque_visible loop exactly: common basis and padding, then tier/topology lengths, intrinsic geometry key (or anchor-relative fallback), opacity and material. View55/24; padding0.1896090534979424 from scale.09/current default raster settings. The choice of one common padding changes key values but not reuse cardinality.

**25 distinct signatures across140 capsule calls; four exact plane-axis byte matrices and four rounded12 matrices.** No trigonometric byte-variant proliferation is present. These25 capsule keys alone fit the64-entry cache; a cold ordered traversal would have25 misses/115 hits, with no capsule-only eviction. This does not include cores, other views, different raster padding or preceding cache occupants.

| Family | Axis ID | Slots | Signatures | Item counts | Topology variants | Types |
|---|---:|---:|---:|---|---:|---|
| Backslash | 1 | 14 | 3 | 2368,2369 | 3 | A,B,C,D |
| Backslash | 2 | 7 | 2 | 2368,2369 | 2 | A,B,C,D |
| Backslash | 3 | 5 | 2 | 2368,2370 | 2 | A,B,C |
| J | 0 | 5 | 1 | 2368 | 1 | A,B,C |
| J | 1 | 14 | 1 | 2368 | 1 | A,B,C,D |
| J | 2 | 6 | 1 | 2368 | 1 | A,B,C,D |
| J | 3 | 7 | 1 | 2368 | 1 | A,B,C,D |
| L | 1 | 13 | 1 | 2368 | 1 | A,B,C,D |
| L | 2 | 7 | 1 | 2368 | 1 | A,B,C,D |
| L | 3 | 4 | 2 | 2368,2369 | 2 | A,C,D |
| P | 2 | 18 | 3 | 2368,2369 | 3 | A,B,C,D |
| S | 2 | 10 | 2 | 2368,2369 | 2 | A,B,C,D |
| S | 3 | 1 | 1 | 2368 | 1 | B |
| Slash | 0 | 6 | 1 | 2368 | 1 | A,B,C,D |
| Slash | 1 | 14 | 2 | 2368,2369 | 2 | A,B,C,D |
| Slash | 3 | 9 | 1 | 2368 | 1 | A,B,C,D |

## Axis Matrices

0: [[6.123233995736766e-17, 1.0, 0.0], [-1.0, 6.123233995736766e-17, 0.0], [0.0, 0.0, 1.0]]
1: [[1.0, 0.0, 0.0], [-0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
2: [[-1.0, 1.2246467991473532e-16, 0.0], [-1.2246467991473532e-16, -1.0, 0.0], [0.0, 0.0, 1.0]]
3: [[6.123233995736766e-17, -1.0, 0.0], [1.0, 6.123233995736766e-17, 0.0], [0.0, 0.0, 1.0]]

## Reuse Boundary

Within each exact family/axis group, extra signatures coincide with actual Item topology changes: most placements2368Items; some2369 and one2370. The source groups world polygons by rounded normal/depth before constructing intrinsic hashes. Therefore translation-dependent depth bucket grouping is a concrete cache-split mechanism; no evidence here supports rounding the cache key or ignoring topology. Preserve exact source coverage and current-object rebinding if grouping is redesigned. Program type A–D does not independently partition the common exterior signature; identical family/axis/topology keys reuse across types.

This census measures key equality only. It does not measure culling time, prove25 misses were the actual profile misses, or justify a cache-policy change without current geometry/raster tests.

## Saved Local-Grouping Delta Review

Independently reread the subsequent _assembly_items delta: bucket normal and depth now derive from local.N/index and local.V/local.faces before placement. Each bucket still appends the actual transformed world polygon, actual world normal and captured lighting/style/owner values. Intrinsic hashes retain exact placement axes/local geometry; no cache-key rounding or proxy geometry was introduced.

The permanent grouping test covers7 prior splitgroups/16 placements. It independently transforms source vertices with Rhino, checks exact Counter equality for every part-key/world polygon, then requires exact intrinsic key plus polygon-length membership equality across translations. GH observed baseline7failures while the polygon/transform controls passed. The corrected execution result is pending at this review; full140 recensus is deliberately deferred until both renderer helper owners report stable source. Actual final raster/style and performance gates remain separate.

## Final140 Census After Stable Saves

Normal repo-admitted retained script completed exit0 in28.328s with exact renderer/model/data start=end hashes. The separately retained generated/visibility-signatures-final.json records **16 distinct signatures**, four exact axis-byte matrices and four rounded12 matrices. These match the16 observed family/orientation groups; all translation-dependent extra9 cache variants are removed. Item-count distribution: {2368: 140}. A capsule-only cold traversal now has16 misses/124 hits, no64-entry eviction. This is key-equality evidence, not a measured full-profile speedup.

Renderer SHA256 bbfb1b29e6090c2bd0fd95f007063ec6bd7a8834edb24703e01b2c2cf89e7e6d; full model/data hashes are in the unchanged endpoint record. GH strengthened grouping test observed PASS1/7.996s includes exact world-polygon Counter, independent Rhino placement, intrinsic membership and fresh semantic/material owners. No cull/order/BSP/Cairo/tower was invoked by either census.

### Progress-Only Source Scope

The final progress save changes full renderer hash to3cf3ec4f14bbf318b7c6c5bb045633cebb091c84975b3eac1164835283e6a28e. GH's compare-before-write provenance record binds its exact old bytes to the census bbfb1b29… endpoint and reports equality outside precisely _section_order and _order_checkpoint (nonblank source hash c8e460dcc85a423d43c3afbab1ee7652c00b495287086e9fc2c6cecb7edd848d). The retained census script calls neither function; the reviewed scope boundary is legitimate. This preserves consumed-region16-key evidence without pretending full-file identity or repeating the census. Fresh rendering proofs must bind the new full source map. Generated visibility-census-progress-provenance.json retains the owner attestation and hashes.
