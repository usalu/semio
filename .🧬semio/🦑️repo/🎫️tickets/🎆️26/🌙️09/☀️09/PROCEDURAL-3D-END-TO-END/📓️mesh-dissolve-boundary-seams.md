# Authored Boundary Seams in Dissolve Vertices

Dissolving a boundary vertex replaces its incoming and outgoing segments with one edge. The boundary loop used to inspect the outgoing segment alone, so it could extend a seam from an authored incoming segment across the unmarked span. The loop now checks both source edges for a seam or authored edge channel and refuses the edit if either cannot be retained. An unaffected seam still follows its exact source edge.

The fixture removes a collinear corner in a planar polygon with authored UVs, normals and smooth shading. The production-linked Rust regression failed before the fix on the incoming seam. After the fix, it passed atomic refusal for both adjacent seams and successful removal with an unrelated seam. The accepted edit retained the other corners' UVs, point normals and face shading.

Three.js independently confirmed that the replacement edge is twice the span of either source edge and that polygon area and affine UV interpolation agree before and after safe dissolution. The portable Mesh jobs file passed 13 tests and 285 assertions in Bun and in a Node replay. The standalone compact native proof launcher passed in dissolve-seam mode.
