# Finite IFC Geometry Payloads

The goal remains active. This change fixes payload admission and camera arithmetic; it does not implement the retained serialization plan in ifc-geometry-payload-audit.md.

## Behavior

The shared TypeScript and Rust geometry source rejects nonfinite coordinates, invalid or repeated triangle indices, and numerically unrepresentable face normals before indexing or encoding. A refused payload emits neither meshes nor instances. Native IFC2x3 and IFC4 rendering adapters expose the limited status and exact diagnostic. IFC4x3 uses the same IFC4 adapter over its existing shared graph.

Camera framing computes a midpoint from a finite span, avoiding overflow from adding two large same-sign endpoints. It tries the opposite view direction when adding a finite camera offset would overflow. An unrepresentable scene span or camera range is refused atomically rather than serialized as null. This preserves source coordinates; it is not GPU render-origin rebasing.

Unused vertices and exactly cancelling normal sums have zero normals, matching the existing Three normal policy. Zero-area or nonfinite scaled face normals are refused. The validator does not claim Boolean manifold validity or complete floating-point coverage for extreme aspect ratios.

## Test Evidence

The neutral camera fixture was authored first. Actual source tests then reproduced nine failures: same-sign midpoint overflow, camera offset overflow, unrepresentable extent published as complete, out-of-bounds/negative/fractional indices throwing, repeated indices admitted, and nonfinite positions encoded as null.

After the change, the current IFC4 geometry source suite passes all 58 groups with zero strict TypeScript diagnostics. Existing decimal.js 10.6.0 independently computes the neutral camera centers and positions. Existing Three BufferGeometry independently computes area-weighted normals, including unused zero normals. A separate zero-area regression verifies diagnostic refusal.

Native tests consume the same camera contract, test malformed indices and NaN/infinity, and verify empty output on refusal. Independent Rust grammar checks pass for shared geometry, native tests, both rendering adapters, and retained Details work/tests. Grammar checks are not compilation. Actual native execution remains behind the previously recorded Windows repository cache case-sensitivity failure.

## Remaining Presentation Work

Normal/bounds/JSON generation still lacks retained original-caller serialization control. The audit identifies the existing first-party JsonWriteSource and JsonBorrowedWriteCursor to reuse. Do not describe this arithmetic/admission repair as yielding or resumable serialization. Physical units, configurable curve quality, and large-world render-origin handling remain separate completion gates.

A read-only peer probe found sparse JavaScript tuples were skipped by Array.some/every. The permanent regression first failed, then passed after fixed ordinal checks and explicit tuple-array admission. Both sparse coordinates and sparse indices now refuse before access. Native fixed arrays enforce arity structurally. The final actual geometry source count is 58 passing groups, zero diagnostics.
