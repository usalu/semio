# Authored Point Normals in Retained Mesh Transforms

The retained Transform, Rotate, Scale, Rotate Components and Scale Components routes now apply the direction matrix to authored point normals. The existing f64 inverse-transpose and normalization also drive the named normal channels. Unselected authored normals in component edits keep their submitted values. Missing normals still use the geometric fallback, and reflected whole-mesh transforms repair winding. UVs, seam flags and face shading survive the in-place edit. General position deformations still use their geometric normal policy.

The point-normal phase quotes one original scalar write or phase transition, plus the depth of any selected-vertex lookup. Its funding is separate from existing named attribute discovery. A short copy allowance leaves the original progress and receipt empty. The cancellation owner retains the edited and unedited rows until close.

The neutral fixture supplies authored normals different from the geometric face normal, one missing normal, corner UVs and a seam. The current native Mesh facade performed a reflected affine shear and a selected rotation in four funded normal turns each, copying 48 bytes per operation. Both rehearsed one-byte-short grants were inert. Cancelling after two normal turns retired 17,531 bytes for the affine case and 23,152 bytes for the selected rotation, with no publication. Three.js independently matched the inverse-transpose normals and reflected winding, selected rotation, existing UVs, smooth shading and seam. The registered Flow workbench covers affine and component rotation.

All 27 portable Mesh tests passed with 4,069 assertions. The 3D facade admitted 2,533 active lines and 219,779 bytes. The official linked Flow suite remains storage-gated.
