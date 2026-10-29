# Actual Corrected Slab Method Diagnostic R78

Executed the actual current `SlabPartitionContract.test_source_region_surfaces_and_sleeves_match_independent_geos` in a fresh isolated interpreter, including roof fixture prelude and every source slab. Wrappers retain original calls/arguments/return values for BaseGeometry intersection/difference/covers and module unary_union. Every unique operand is JSON-appended once, object references retained against ID reuse, with one compact pre-call marker per operation. No producer/test/source changes. Source loaded before coordinated fixture publication; before/after source remains452/d2f3. Main validators untouched.

Initial180s observer bound extended360then600seconds because operations continued advancing. Automatic worker termination at600s; no final unittest result/pass/failure was produced. Retained reproduction slab-operation-r78.py; operand registry generated/slab-operation-registry-r78.jsonl contains3602unique geometries.

| Elapsed s | Slab | Operation | Ordinal | Operand IDs |
|---:|---|---|---:|---|
|53.735|SL-1|intersection|3155|617,635|
|152.924|SL-1|intersection|11246|1416,1413|
|214.503|SL-1|intersection|15908|1884,1902|
|297.600|SL-R|intersection|21638|2388,2389|
|331.274|SL-GROUND-CELLAR|intersection|23980|2470,2481|
|384.745|SL-GROUND|intersection|28597|2787,2838|
|446.476|SL-GROUND|intersection|33362|3280,3314|
|499.455|SL-GROUND|intersection|37666|2858,3502|
|559.668|SL-STAIR-CANOPY|intersection|38695|3552,3601|
|601.441 observation after auto-stop|SL-STAIR-CANOPY|intersection|38695|3552,3601|

The actual method advanced through SL-1 (including former last difference), roof and ground slabs. Final unchanged marker is the SL-STAIR-CANOPY intersection of3552/3601, confirmed stable for about42seconds, not60seconds; no permanent-hang claim is justified. Freeze is generated/slab-operation-last-operands-r78.json SHA256`966dbc44fc4f3999b27a2464841b57d704c8fb693ebc660fdbdb76edc5ace234`. Many operations plus filesystem instrumentation account for substantial wall time; the final captured kernel separately merits bounded exact-pair diagnosis.

Same-worker callable co_names/marshal-code/constants hash capture was queued, but synchronous test did not return before worker termination, so that requested runtime-bytecode evidence is unavailable. Do not substitute a fresh interpreter file/AST hash as proof of this worker loaded bytecode. The diagnostic imported the then-current corrected method before dispatch and wrapped covers without changing its return values; no result is inferred for any possibly stale separately cached validator class.

## Exact Last-Pair Predicate And Invalid Material Finding

Fresh bounded exact-pair GEOS predicates returned disjointTrue in0.001453s and touchesFalse in0.004477s. Left validityTrue, right validityFalse. Right area1.4503954624574418e-12m²; GEOS explains `Self-intersection[4.00113325221129 10.2930748397082]`. Left area4.7152663316170855m² and valid.

Independent exact binary64 rational BigInt determinants corroborate strictly disjoint convex hulls, without assuming either ring convex/valid: right directed edge vertex3 `(2.811716948659,10.402239499261)` to vertex0 `(2.811716948652,10.402239499262)` has all its own vertices on/nonpositive side `[0,-,-,0]`, while all four left vertices are strictly positive. The separator proves intersection empty even for this invalid self-crossing right ring. No GEOS intersection overlay or tolerance is needed for that proof.

However a generic disjoint predicate short circuit would only avoid this zero-overlap call; it cannot certify the native geometry. The original method checks validity per current outer-loop polygon, so this later-index invalid right operand has not yet reached its own validity assertion. Preserving that assertion will expose a genuine invalid native SL-STAIR-CANOPY sliver. Do not hide it by area cutoff, skip invalid rings, or call the test passed after shortcutting this pair. touches isFalse and cannot address this pair. The production material kernel/source construction needs a topologically correct treatment preserving real occupied material; exact independent sliver analysis should guide it.
