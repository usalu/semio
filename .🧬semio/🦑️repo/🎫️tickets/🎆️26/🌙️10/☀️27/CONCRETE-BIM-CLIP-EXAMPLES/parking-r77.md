# Current Physical Parking Verification R77–R85

The final parking fixture is bound to current source528b3ffeab26e4c038b9eb603e7ae1e9bc52c66a1f068b3e799de8fcd66ccc8f with signaturefc63dc8c97fce83dfeed940e6d293d989332b7bdcfd046e4ea0248d9b048f607. Only source_signature and physical_obstacles changed. Every car dimension, positive/rejected pose, reserve, target/status, historical capacity and previous expected_results value is preserved, proved by equal protected-control fingerprintd0ca178a2b4b9f30f2fe78814fb1a068316093d99ab5590c0be8cab33738f871.

## Independently Reviewed Geometry Delta

The catalogue changes2542→2540records. Eighteen laundry/tap records are exactly the intended rigidY+0.50m translations. Every other changed group belongs to the already reviewed final-only clipping normalization: CW-1/frame318→318records and W-G-W/solid41→39. No column, vehicle, garage boundary or operational reserve moved.

Exact binary64 comparisons ignore only JSON integer/float spelling. Exact ring identity is invariant to cyclic start/reversal and closing duplication; no coordinate rounding or snap is introduced. Identical polygons are deduplicated within each projected union and exact shared polygons cancelled for a conservative review bound. For A=C∪Ar and B=C∪Br, AΔB=(ArΔBr)\C⊆ArΔBr. Thus residual XOR bounds the full union difference and cannot create a false pass. CWframe158unique polygons on each side share156, leaving2+2; residual bound8.7818477e−13m². W-G-W22new/23old unique polygons share21, leaving1+2; residual bound9.7766975e−13m². These are conservative residual bounds, not claimed exact full-union XOR values. The existing1e−8criterion is unchanged. A bounded slower full-group exact-reference attempt was retained and stopped; the permanent oracle was not altered.

All five fixed scenarios retain exactly the previous per-car and pair-gap numerical metrics and identical semantic violation multisets. Only diagnostic ordering can differ. Independent polygon-clipping JavaScript versus GEOS results differ by at most8.8817842e−15. The two full-size parked envelopes remain verified; four retained rejected demonstrations remain invalid. Three modern-car packing remains unverified, with no global infeasibility claim. Static parking results do not certify an entire arrival trajectory or native rendered frame.

## Fresh Test Gate

The complete actual10-method ParkingContract passed10/10,0failures,0errors in126.6935947seconds. Before/after raw source/provider/33production identities and selected loaded function bytecode/defaults/closures stayed stable; no recorded input file changed. Existing independent JavaScript validateParking also passes. Root/source geometry was not modified by this fixture publication. The exact publication time is2026-10-29T08:21:31.738Z.

Reproduction inputs: parking-refresh-r77.py, parking-delta-r77.py and current-feature-cohort-r85.py with cohort_mode='parking'. Generated parking-review/delta/independent/publication-r77.json and parking-r85-selected/before/progress/failures/summary.json retain exact source inputs, controls, observations and runtime evidence. Expensive verification uses observable boundaries and cancellation; all output stays in the ticket generated directory. Native frames remain unverified.
