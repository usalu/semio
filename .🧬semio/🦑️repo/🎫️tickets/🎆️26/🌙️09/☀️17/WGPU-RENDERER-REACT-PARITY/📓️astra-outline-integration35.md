# Outline Materializer Integration

Root review found that the new outline pass consumed every planned instance, but the following allocation phase only changed its phase tag. The position phase therefore saw an exhausted instance cursor and skipped all vertex writes. The repair uses the existing `advance_phase(Positions)` operation, which resets instance, item and normal substep together.

The existing end-to-end paged GLB law already compares actual published positions, normals and indices. It now also requires four perimeter outline edges with no semantic edge identities and checks that local outline scaling happens before the translated node and world transforms. The real World asset publication law now requires three presentation-only boundary edges alongside its three vertices and indices. The independent actual Three `EdgesGeometry` fixture oracle passed 4/4 in Sol's recorded run; the expanded native laws have not run yet.

Root native batch 35 and browser build 26 remain active. Their renderer compilation has not yet produced a receipt. The existing native batch filter does not include the expanded GLB laws, so a subsequent focused run is required even if batch 35 succeeds.

## Browser Build 26 Receipt

Build 26 ended after 13m50s before publication on one E0599: its compiled World dependency did not yet expose `World3dShadowProfile::IconSvg`, while renderer source already selected that variant. Current source defines the variant and the UI `SvgFlatLit` profile. Sol's actual production WGSL GPU readback now passes 3/3 alongside the Three SVGRenderer oracle, matching each fixture RGB within one byte and exercising the separate PBR branch. Cross-crate production interfaces are held for coherent build 27; the native batch remains active. No fresh integrated runtime pass is claimed.

## Retained Paint Logging Review

The new successful-progress reset leaves the document cursor's stall count at zero. The old modulo-only notice condition also matched zero and therefore logged every normal paint opportunity. Root added the positive-count guard so diagnostics report actual stall intervals. The paint/candidate behavior is unchanged. Sol is auditing whether the scalar progress witness can distinguish cursor advancement from authority back-pressure; this is not yet a runtime receipt.
