# Core Fidelity

## Measured Handing Correction, Latest Snapshot

The coordinator's new primary/measured-floor correlation supersedes the former B 180-degree rotation assumption. The source-correlated B structural compound and circulation now mirror canonical A across X. The coordinator separately saved all B phase-1/phase-2 swaps and corrected A floor 3 to N/J plus W/Slash at the same phase-1 sill. Capsule frames were not globally reflected by the core implementation.

The authored strict contract now records `core.circulation.handing = {"A":"identity","B":"mirror_x"}`. `_core_frame` applies this reflection after each canonical stair rotation. The renderer-neutral `place` function detects a negative frame determinant and reverses every face winding, preserving outward positive-volume solids; points, circle centres, circle normals and bolt positions still map through the same frame. Cabin, service cells, lift enclosure, basement network, corner piers, roof return and world-space walking-route metadata all consume the reflected structural frame.

Four new reflection contracts were added, with the initial three run red before implementation. Final dedicated core suite: **12/12 PASS, 12.256 s**. NumPy matrices and Rhino mirror/vector transforms independently reproduce vertices, points and normal mapping; all compared mirrored lift/cabin/basement parts retain positive signed volumes. The complete independent support/body-coverage/all-mesh circulation suite on the new authoritative poses passes **17/17, 20.919 s**. All 140 capsule landings and every previously required lift, bridge, office and service-roof target remain ground reachable.

The latest full shared 58-test run passes all assembly/geometric/Rhino/headroom checks. Its five fixture failures (53/58, 46.872 s) are the coordinator-owned old program counts in two tests, 74 versus 75 branches, old height/base-elevation values, and old sample plane origins. Current authored programs are A35/B34/C35/D36, with 75 branches and ten single branches. These global fixture updates are coordinated separately; no geometric assertion was weakened.

The shared bilingual README now states the source-correlated B X reflection and phase-specific sparse-floor access. Retained patch inputs added for this correction are `core-mirror.py` and `core-handing-docs.py`. No generated atomic stage remains.

## Latest Gate, Superseding Intermediate Runs Below

Final repeat on the frozen core source: core **8/8 PASS, 11.048 s**; full shared suite **56/58, 48.047 s**, with only the two already-identified coordinator-owned stale datum fixtures failing. All shared geometry/assembly/clearance checks passed. The strengthened circulation gate remains **17/17 PASS, 19.588 s**.

After the coordinator saved the 2.90 m pitch, [0, 4.40, 8.12] working podium chain, 5.00 m basement and native sparse-floor phase-1/2 staggering, the core suite passed all **8 tests** (10.321 s). The strengthened complete independent circulation suite passed all **17 tests** (19.588 s) on the final core geometry. All 140 capsule landings, 26 lift approaches, six bridge approaches, three decks, two office thresholds and both service roof return landings are reachable exclusively from the two actual exterior ground thresholds. Actual convex support edges agree with independent NumPy clipping/NetworkX. Every continuous .6 m flat walking strip and every stair-centre headroom tube clears all physical structure meshes and capsule envelopes.

The final two proof additions validate the complete horizontal body footprint of every oriented corner strip against the union of actual coplanar support faces. A standard-library convex subtraction implementation agrees with an independent NumPy clipping and vertical-interval integration oracle, including portable gap, touching-plate and overlapping-triangle cases. This exposed an unsupported wedge in the former straight diagonal route. The stair inner edge was moved from 1.25 to 1.20 m while retaining .65 m stair width, the full 2.4 m compound and all 1.15 m triangular piers. Rounded supported corner paths fit the resulting .608 m normal gap and preserve the .6 m passage requirement. Stop doors and call plates are recessed within the compound front-wall plane, keeping the walking band clear. No geometric test was relaxed to conceal the gap.

The new exact roof datums exposed the old assumption that a final short roof interval could repeat three normal flights. A canonical east return flight now follows the last normal N/W/S cycle, with only a southeast bottom turn and northeast roof landing. This avoids repeating landings less than 2.1 m above one another and keeps the return flight opposite the west water-tank cradle. The authored contract explicitly labels this `inferred_east_return_after_last_capsule_phase`; its historical schedule is not asserted. No clearance threshold or barrier was weakened.

The complete 58-test shared run passed every assembly, geometric, Rhino and clearance test. Its two remaining failures at that snapshot were coordinator-owned stale datum fixtures (old 42.1333/7.5 values and old sample plane origins), being rewritten with the primary dimension update. A fresh full run belongs after those fixture updates. The core's former basement-turn and vent-curb test failures are corrected and passed both targeted and full runs.

Core-owned changes are complete and saved. Remaining integration work is the coordinator's datum-fixture refresh, final complete gates, source/visual review and ticket closure. A fresh JavaScript run accepted the schema and rejected all 29 malformed models, but identified the old `items.stagger` metadata still differing from the new 2.90/3 pitch; the coordinator was notified to correct the global datum metadata. Exact lift machinery, concealed basement equipment and a complete historic room plan remain absent from the inspected primary evidence; visible topology and explicit inferred proportions are implemented without manufacturing identities.

## Files Changed

- Shared Nakagin Python assembly, authored JSON, JSON Schema, and bilingual README.
- Shared portable cases and shared Python tests: new source-consistent basement turn footprint expectations.
- Circulation portable cases/tests: office terminals, convex support contact, oriented clearance, and roof return reachability.
- New `🧪️cases/🗼️core/🔣️.json` and `🧪️tests/🗼️core/🐍️.py`.
- This report and retained ticket authoring inputs: `core-inputs.py`, `core-geometry.py`, `core-basement.py`, `core-convex-tests.py`, `core-proof-cases.py`, `core-roof.py`, `core-roof-contract.py`, `core-coverage-cases.py`, `core-route.py`, `core-lift-clearance.py`. These record chronological narrow patch inputs, not a runtime migration or a replacement for current source. Temporary atomic stage files were renamed into their final destinations; no generated stage remains.

## Intermediate Evidence

Core work is in progress. The new portable core contract has six tests and was run red before implementation: five missing-element failures and one missing-profile error. Existing datum values remain owned by the coordinator.

Authored strict schema and quantities for the primary-plan inferred square core's four 1.15 m triangular corner piers, 2.4 × 2.2 m clipped central compound with .24 m chamfers, two west service compartments and east lift chamber. These dimensions are proportions inferred from the primary raster, not survey measurements. The cabin reference is .95 × 1.1 m, parked at ground, with explicitly nonhistorical display supports; drive, guide and counterweight mechanisms remain unverified.

Saved geometry functions now reconstruct the clipped compound, separate service divider and lift chamber, reference cabin walls/floor/doors/handrail/control, stop call buttons, and short N/W/S stair flights around the larger compound. New L turning landings and local door bands preserve all authored attachment frames. The existing NumPy/NetworkX connectivity and all-mesh clearance gates remain required and have not yet been rerun after these edits.

The corner piers, office opening/threshold, basement divisions and shared short-flight basement network are saved. The office aperture now has independent triangle-segment width/headroom samples with positive jamb controls. The portable logical-floor case recovers an unoccupied phase-0 datum from phase-1/2 sparse entries using the authored pitch, validated independently with NumPy.

Latest completed circulation run: 14 tests, 13 passing. Every flat .6 m strip and stair centreline clears all physical parts and capsule envelopes. All support edges agree with independent NumPy clipping and NetworkX; 26 lifts, six bridge approaches, three bridge decks and two new office thresholds are ground reachable. Four sparse floor-3 capsules remain disconnected because their previous same-level West pair conflicts with the primary compound/pier topology. The coordinator has approved restoring their native phase-1/2 staggering during the global datum correction. Geometry will be rerun against that saved data before a green final claim.

Actual floor polygons are clipped against all four pier diagonal planes. The proof now retains actual convex upward support faces rather than filling their bounding rectangles. Shared contact uses explicit polygon clipping with a separate NumPy clipping implementation. Diagonal corner paths use continuous oriented .6 m prisms; obstacles are rigidly transformed into each prism frame before triangle SAT and containment checks. Portable regressions include a false bounding-box contact in a triangular void, a narrow contact rejection, a clipped wide-contact acceptance, and oriented clear/blocked examples independently reproduced by NumPy. Existing clearance widths, rise limits, ground seeds and all-physical-parts barriers remain unchanged.

The full shared 58-test run initially passed 56 and identified the obsolete one-metre-square basement landing expectation and roof-curb bearing at the newly enlarged compound. The source-derived turning extents replaced the square expectation; curbs now overlap the actual outer shaft walls. Both corrected tests passed on a targeted rerun. AJV accepted the authored model and rejected all 29 existing invalid cases (1,318 quantified leaves at that snapshot).

Inputs retained in this ticket: core-inputs.py and core-geometry.py. No new runtime dependency was added.

## Retained Inputs and Cleanup

Superseding earlier chronological input lists: the twelve one-off core source-rewrite patchers were removed after their changes became authoritative source and portable tests. No migration or replay patch script is shipped. Reusable source-inspection, collision/circulation, scan, capture and native-validation inputs remain, along with every Markdown audit and test fixture. Removed patchers: core-basement.py, core-convex-tests.py, core-coverage-cases.py, core-geometry.py, core-handing-docs.py, core-inputs.py, core-lift-clearance.py, core-mirror.py, core-proof-cases.py, core-roof-contract.py, core-roof.py, core-route.py.
