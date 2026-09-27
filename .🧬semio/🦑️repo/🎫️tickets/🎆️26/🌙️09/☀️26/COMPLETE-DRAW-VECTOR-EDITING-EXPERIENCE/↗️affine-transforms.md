# Exact Affine Transform Foundation

Direct resize and rotation must preserve artwork inside transformed groups. The current five-scalar transform record decomposes a six-coefficient matrix by discarding its shear component. This changes geometry when a world operation is conjugated through a nonuniformly scaled parent. It also erases the second column when the first axis is collapsed.

The schema now requires a sixth scalar, `shear`, using the decomposition `translation × rotation × [[scaleX, shear], [0, scaleY]]`. This is a coefficient rather than an angle, so it remains meaningful when an axis collapses. Signed scales represent reflections. No legacy decoding or default shear is added; authored fixtures and producers are updated together.

Ten neutral matrix cases cover ordinary/sheared/rotated/reflected/rank-one/collapsed and extreme-magnitude transforms. TypeScript tests compare recomposition and point mapping with Three.js. The initial registered run (10207, exit 1) failed on the absent affine module. Rust/TypeScript twins, retained hashing/admission/cloning, semantic mutations and bilingual numeric editing are now updated. This foundation alone does not establish usable on-canvas handles; that workflow remains required.

## Implemented

- Added `geometry/↗️affine` Rust/TypeScript compose/decompose twins and routed the existing native schema functions through them. QR decomposition uses normalized column dot products instead of determinant products that overflow at large magnitudes; a zero first column retains the second column through its rotation and magnitude.
- Added required `shear` to DrawingTransform, the semantic transform JSON schema, GraphQL and TypeScript mirror. All authored JSON transform records under Draw now include it; a follow-up scan found zero missing records. Updated Rust constructors, the JSON generator and TS authored fixture values. No compatibility default or migration script was added.
- Reflections and collapsed axes are accepted as finite affine data by ordinary and retained mutation paths. Parent inversion still rejects singular parents when an operation requires inversion. Numeric fields now expose Shear/Scherung through both inspector and action arguments. The independent field-patch fixture accepts signed/zero scales and validates shear.
- Retained cloning and transform mutation publication copy the new scalar. Layer and mutation digests consume it as a separate bounded step. Digest regressions distinguish shear-only edits; rich clone/mutation fixtures carry nonzero shear.
- Translation gestures preserve existing shear when publishing their transformed records. Direct point transforms and scene projection use the complete matrix.
- Native tests cover the neutral matrices, preserved scene matrix after an affine edit, exact inverse restoration, updated collapsed-axis expectation, and retained digest distinctions. These native tests have not yet executed successfully.

## Verification

- Registered `@semio-tech/draw-js:test`: **65 passed / 1,287 assertions** after affine implementation (71610, exit 0).
- Added independent Ajv transform-schema checks: every neutral transform is accepted and omission of required shear is rejected.
- Final registered suite: **66 passed / 1,307 assertions**, plus **44 independent Ajv field-patch cases** (42750, exit 0; `tests-affine-contract.txt`). Three.js verifies matrix recomposition and a world rotation conjugated through a sheared/nonuniform parent.
- Scoped `git diff --check` passed.
- Existing native test 30810 and component build 43904 were polled and remain live. They were launched before these changes; their eventual source coverage must be inspected before claiming native validation. No duplicate native run was started.
- Browser tab 4 is retained. No rebuilt affine inspector or handle runtime verification is claimed.

## Next Integration

The later [transform handle checkpoint](🎛️transform-handles.md) confirms that the current request-context render hook already provides InteractionView. Draw now uses that hook to pass selection into the shared canvas scene. The existing host Gumball sends incremental mutations and lacks start/preview/commit/cancel semantics. Draw's new implementation instead uses its existing retained pointer gesture authority; verification and remaining interaction work are recorded in that later checkpoint. Group opacity/isolation, SVG editable import, typography and the remaining acceptance workflows also remain open.
