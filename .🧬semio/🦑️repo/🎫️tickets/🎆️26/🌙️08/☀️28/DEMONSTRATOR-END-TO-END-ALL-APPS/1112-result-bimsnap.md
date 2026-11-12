# bimsnap (11/12)
## Step 1: histogram of W-ifcdetails-12 errors in semio-s-artifact-bim-model-snapshot (684 errors found by path; 1368 in file = duplicated lines)
- 558 E0277 `BTreeMap<String, X>: RetainedClone` not satisfied in `__ModelSnapshotRetainedCloneCursor` (66 map fields x 8 each) + 126 E0277 same without the "in" tail; 683 at snapshot/🦀️.rs:72:181 (derive RetainedClone on ModelSnapshot), 1 at ...io/binary/snapshot.
## Root cause
BTreeMap is no longer RetainedClone. Fix: ModelSnapshot's 66 map fields become RetainedOrderedMap<String,_> (same as remodel/energy; ToValue/FromValue/DslField exist). Consumers: ~456 bim files use BTreeMap APIs on those fields (insert 521, contains_key 681, values 287, keys 272, get_mut 223, clear 142, remove 58, ...).
step2 W-bimsnap-1 result: snapshot crate 684 -> 1 error (ArtifactSqliteSnapshotHost for ModelSnapshot missing: host impl lives in unwired io/sqlite file; removed the override so default None)
step3: W-bimsnap-2 (hub wasm, -p semio-s-artifact-bim-model-snapshot) rc 0. Additive RetainedOrderedMap API (contains_key/keys/values/first_/last_key_value/iter_mut/values_mut/get_mut/cold_clear/cold_extend/cold_retain) in value/retained-clone/ordered-map/map-core; schema values PropertySet/ClassificationSet aliases now RetainedOrderedMap. Next: ifc4x3 native_bim.rs consumers, then semio-hub-stdio-component.
## Step 4: semio-hub-stdio-component (W-bimsnap-3, rc 101)
- bim-model-snapshot compiles inside the hub chain (0 bim errors). Only red: semio-s-artifact-stdio-ifc4x3-exchange, 25 errors (RetainedWorkOutcome::map_err / receive generic arity / owned_pack::geometry in native_pack.rs, native-gltf-*.rs, native_exchange.rs) - claimed by ifcguidance, not mine.
- ifc4x3 io/native_bim.rs ported (storeys param + output.{sites,buildings,storeys}.cold_insert); its type errors would show in the same pass and none did, but borrowck of that crate is unverified until the exchange crate is green.
## Edits
- bim snapshot root 🦀️.rs: 66 BTreeMap fields -> RetainedOrderedMap<String,_> (+Default, skip_serializing_if paths).
- bim schema values: PropertySet / ClassificationSet aliases -> RetainedOrderedMap.
- value retained-clone ordered-map map-core: additive read/cold API.
- bim io/binary/snapshot: removed sqlite_snapshot_codec override.
## Debts (successor)
- BIM sqlite relational codec: io/sqlite/snapshot (75 KB + cells) is not compiled in any production crate (host impl cannot live outside the snapshot crate; only a cfg(test) path include from io tests). Needs move into the snapshot crate (mind 262144 B budget) + port to native out-slot signatures + RetainedOrderedMap (read_map/read_properties use BTreeMap). Until then ModelSnapshot has no sqlite codec (default None).
- ~456 other bim files (mutations, diff algebra 🔺️diff, io import/export, inferences, tests) still use BTreeMap APIs on ModelSnapshot fields (insert->cold_insert, remove->cold_remove, get_mut/values/keys/contains_key now exist, entry/range/extend/clear need cold_* rewrites). None are in the hub component chain; not compiled by me.
STATUS: FIXED-verified W-bimsnap-2
