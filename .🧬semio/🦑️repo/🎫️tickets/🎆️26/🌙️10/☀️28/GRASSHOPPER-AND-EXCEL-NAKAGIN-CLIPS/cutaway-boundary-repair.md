# Cutaway Boundary Identity Repair

The clipping helper now uses the shared fragment kernel's existing per-source 64-ULP coordinate equivalence to classify plane contacts. A crossing at an equivalent source endpoint reuses that endpoint's identifier. It does not construct an indistinguishable new vertex. There is no additional epsilon, material-area cutoff, fragment-kernel change, or exception suppression.

The minimized lining-back corner and edge contacts have mathematically positive float slivers under exact arithmetic. Their coordinates are equivalent under the existing declared numerical policy. They are contacts under that policy, rather than exact-zero-area polygons. Endpoint reuse removes these artificial wedges before boundary assembly; a genuine 1e-10-high material remnant remains positive and publicly prepares successfully.

`capsule_axon` also enables physical bolts by default. An explicit `bolts=False` retains the diagram option. Its docstring describes the enabled default.

## Verification

- RED: `CutawayMaterial.test_source_boundary_identity_removes_only_roundoff_contacts_and_keeps_positive_material` failed both portable one-ULP contacts because the prior helper emitted one face instead of zero. The positive control did not fail.
- GREEN: the same test passed after endpoint classification and identity reuse (1 test, 0.003 seconds).
- GREEN: all dedicated cutaway tests passed (4 tests, 23.667 seconds). They include independent SciPy area and Pillow raster comparisons, original normals and names, uncut mesh identity, the positive remnant control, and all six physical families with both nearest-wall signs. Every emitted clipped facet is submitted separately to the existing public `ordered_fragments` preparation; the suite avoids a full aggregate BSP.
- GREEN: the portable JSON schema accepts six clipping contracts and the boundary-identity contracts, and rejects seven malformed contracts, including a changed equivalence multiplier and malformed boundary-normal dimensions.
- GREEN: `VisualContracts.test_default_reference_drawings_keep_all_physical_attachment_bolts` passed in the combined hardware/shared-geometry run.
- GREEN: actual `VisualContracts.test_capsule_parts_are_shared_geometry` passed. It draws both the ordinary shared capsule and `capsule_axon(capsule_type('A'), cutaway=True)` through the real renderer, covering the original full-suite failure. The combined hardware/shared-geometry run passed 2 tests in 132.337 seconds.

The amended visual source raw SHA256 is `4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2`. All owned validation commands are terminal.

## Files

- `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/✂️cutaway/🐍️.py`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/✂️cutaway/🟨️.cjs`

The existing portable boundary case and schema were already present at execution handoff and were preserved. Root owns the broader visual test/case changes, freeze amendment, ticket lifecycle, and generated-output cleanup.
