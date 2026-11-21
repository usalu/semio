# Mesh Unwrap with Authored UVs

The registered Unwrap UV operation now accepts meshes carrying a named UV set. It writes the new coordinates to raw corners and replaces the active rendered channel under its existing name with linear corner samples. A vertex-domain input can therefore split at a seam. Secondary UV sets retain their values, domain, interpolation and indices. The application phase processes at most 256 corners per turn and retains an old active channel for the existing bounded retirement path.

The shared fixture uses two coplanar right triangles separated by a marked diagonal and an indexed vertex UV for the primary set. Three.js independently reproduced each island's side ratios, surface area and shared seam. The native regression reproduced the prior authored-channel refusal, then passed cancellation without publishing, identical results across turn budgets, distinct UVs at the shared point, protected normals and shading, and the active UV samples in normal preview.

The preview test exposed a separate failure in replaced attribute indices. The old preview path stopped at a 424-byte nested retirement quote, and granting that quote entered a child that required an original producer invocation. Replaced index buffers are contiguous destructor-free u32 storage. Normal projection now quotes their exact allocation release and publishes only after the admitted turn. The source indices stay within the pending attribute before release, including during cancellation. The test verified that the indexed secondary UV is still present in the rendered preview.

The standalone native authored-unwrap proof passed. Bun passed all 16 tests and 3,792 assertions across the portable Mesh jobs and packaging suites.
