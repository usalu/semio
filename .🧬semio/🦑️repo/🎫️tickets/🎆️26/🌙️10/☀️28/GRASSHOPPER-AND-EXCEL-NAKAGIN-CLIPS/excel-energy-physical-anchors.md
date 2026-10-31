# Excel Energy Physical Drawing Anchors

The Energy scene's static POINT dictionary used nominal X ±1.25, front Y 4 and back Y 0 while the reference drawing now shows a physical 2.60 × 4.20 m casing. Its window leader also used the nominal front plane instead of the actual recessed glass midplane. The volume overlay manually recomputed bounds and did not honor explicit usable-interior controls.

Portable cases/schema and pure AST-extracted helper tests precede correction. The planned scene-only helpers derive physical points and normals from capsule_casing/capsule_window and volume faces from capsule_interior. Thermal workbook areas, heat-loss formulas and nominal study dimensions remain unchanged. No shared model/data source is edited. Full scene import/native verification follows the final renderer freeze.

## Red Evidence And Saved Correction

The portable reference-point assertion was executed directly against the existing authored source before editing: the old left anchor −1.25 failed the physical −1.30 expectation. This was a filesystem-REPL source-value assertion; the Python geometry suite was still awaiting normal admission, so no baseline Python execution is claimed. The saved scene now derives every leader and its normal from physical APIs, places the back-wall marker beside the door opening, and uses authoritative usable bounds for the translucent bounding-box overlay. Nominal thermal data/formulas and narration/transcripts are unchanged. Three Python API/AST/NumPy/Rhino regressions are queued for the saved implementation.

## Accepted Gate

All three Python geometry/API/AST tests pass in 0.377 seconds with exact queued-start/end scene, case, test and shared-model byte fingerprints unchanged. Independent AJV/source-value checks pass and reject three malformed contracts. Current Excel engine85 and registry5 also pass with matching complete endpoint maps. The source-only scope audit is clean. See excel-static-gates.md for all exact identities. Native remains on HOLD.
