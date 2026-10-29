# Slab Partition Verification R23

The physical slab implementation now validates material ownership against the reference outline before applying editable holes. A portable JSON case first exposed an invalid overlap hidden wholly inside a hole; the previous implementation accepted it. The new implementation rejects it. Gross-region area coverage and occupied material area remain checked. Exact bounding-box rejection avoids unnecessary disjoint cuts without changing approximation facets.

Three independent Shapely/GEOS contracts passed in1.513s: near/boundary/far holes; hidden ownership overlap; every current source slab with containment, area conservation and STRtree pairwise interior intersection checks. A global coincident-fragment union stalled and was cancelled; the alternative proves the same occupied partition through containment, conserved area and no interior overlaps. This is not a weakened overlap tolerance.

A controlled same-input comparison used448 source products and SHA256 9418d0b426f084daa13f03d17720fba8cbd3aef0787552ef768a4b7e0660255d. The old slab-owned functions were cloned in interpreter memory, with both versions sharing the same current helpers and model. Old:35.0348706s,4583 parts,81.16840635537285m³. New:0.5049694s,598 parts,81.16840635589575m³. Region tags, layer thicknesses and levels match; per-region areas differ by less than1e-7m²; curve facet counts are unchanged. This isolates the slab algorithm from concurrently changed profile helpers.

Fresh whole-source evaluation of all7 slabs took1.336s. The first floor took.4412s and roof.3733s. These are independent WASM CPython timing observations, not native production performance promises.

Permanent inputs: model/tests/⚡️partitions.py and ⚡️partitions.json, registered by the main suite. API and schema changes preserve the gross material partition even after new service sleeves. No product runtime dependency was added. Generated benchmark and check outputs remain temporary under this ticket's generated directory.
