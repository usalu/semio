# Exact Canonical Image Equality

`RasterImageContentEquality` shares the existing canonical field walker in the Raster model's `🔑️content-hash` module. A reused 64-bit content key requires exact content verification before adoption; equal digests alone do not establish equality. The helper covers schema, dimensions, colorspace, bit depth, frame count/delays/RGBA, profile presence/content, and ordered metadata keys/values. It allocates no storage and borrows the captured immutable source images.

Each advance accepts 2–32,768 bytes of comparison authority and counts both images' reads. Eight-byte field-length prefixes also spend authority, so arbitrarily many empty fields still yield. Both original addresses remain bound. `equal` becomes true only with `done`; invalid grants and replacement roots refuse. The six-route publication owner is being connected to this helper before destination reuse.

The neutral fixture contains sixteen exact/different image pairs, plus grant and empty-metadata traversal policies. Rust tests cover every pair across five grants, source replacement, invalid grants, long equal prefixes, late differing pixels, and 4,096 empty metadata entries. The existing model unit-test module mounts these tests. They have not been compiled or executed because the documented repository source-admission blocker remains unresolved.

The retained ticket input `image-equality-oracle.ts` independently checks the same sixteen pairs using the existing third-party Ajv `const` implementation. That oracle executed successfully: sixteen vectors matched. This validates the fixture expectations, not the Rust implementation or integrated publication behavior.
