# Read-Only Native Batch Driver Audit

Reviewed replay-native-pairs.py, native-pairs.json/schema and validate-scenes.py without launching a native process. No concrete new acceptance bypass was established in this source review.

The plan requires exactly the registered five GH/four Excel scenes, both languages and current contract counts. Driver/plan/schema/acceptance ledger are unconditionally fingerprinted by the recorder. A full all-scene contract run supplies the frozen dependency baseline. Before and after each pair, disk semantic fingerprints, interpreter/package versions and tracked rendering configuration must match. Cached modules are not silently reloaded after drift; the batch aborts. Pair records require success, nonempty equal start/end source maps, stable runtime endpoints, no errors/failures and one lifecycle test method. Every pair dependency must agree with the all-scene baseline; a newly discovered dependency outside it is rejected.

The driver resets tempfile selection for the pair-specific environment and collects unreachable scene objects between runpy invocations. Each pair uses its own exact output directory. Warm module/geometry caches are intentional; this source audit does not claim a cold-isolation test or proof that no library global can ever mutate. Current contract/native records and the hardened ledger remain the actual acceptance evidence. No native18 or final image acceptance follows merely from syntax/schema checks or this review.

## Fresh Scene-Worker Redesign Source Reread

The current driver now runs fresh repo-governed child contracts and one fresh worker per scene, reusing caches only for that scene's EN/DE pair. The worker is the same fingerprinted replay driver under --worker-scene, not an untracked external helper. verify.py is explicitly fingerprinted; child command/output use explicit UTF-8 and exact per-scene/pair directories. This bounds full-series rich-cache retention without private shared-cache mutation. Root owns the nested repo/temporary-output/exit-code stub validation and actual native runs. No new concrete source-provenance bypass was found in this bounded reread, and no native acceptance is inferred.
