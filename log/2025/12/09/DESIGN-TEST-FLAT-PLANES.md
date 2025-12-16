---
slug: DESIGN-TEST-FLAT-PLANES
summary: >-
  Extend design E2E test to verify flat planes and centers match expected asset
  data
---

# Previously

The unit tests in `semio.test.ts` verify that flattened designs have correct planes and centers by comparing them against the "Flat" subdesigns stored in the metabolism kit asset. The E2E test (`sketchpad.test.ts`) was missing this verification - it only tested UI interactions but not the correctness of the computed piece metadata.

# Plan

1. Export the expected flat pieces (planes and centers) from `assets/index.ts` for the "Nakagin Capsule Tower" design's "Flat" subdesign
2. Expose the `piecesMetadata` function on `window.__piecesMetadata` so the E2E test can access it
3. Add helper functions (`planesEqual`, `centersEqual`) to the E2E test for tolerance-based comparison
4. Extend the Design test to:
   - Access the computed pieces metadata via `page.evaluate()`
   - Compare with expected flat pieces from the asset
   - Log matches/mismatches and fail on any mismatch

# Changes

- `assets/index.ts`: Added export `MetabolismKitNakaginCapsuleTowerFlatPieces` containing the expected planes and centers from the "Flat" subdesign
- `js/js/sketchpad/Sketchpad.tsx`:
  - Exposed `piecesMetadata` function on `window.__piecesMetadata` for E2E test access
  - Fixed `PieceStore` to include `name` property (was missing from constructor, getter/setter, snapshot, and change methods)
- `js/js/sketchpad.test.ts`:
  - Added inline import of `MetabolismKitData` from JSON with type assertion
  - Computed `MetabolismKitNakaginCapsuleTowerFlatPieces` locally (Playwright can't use TypeScript imports)
  - Added `Plane` and `Center` interfaces with `TOLERANCE` constant (0.001)
  - Added `planesEqual` and `centersEqual` helper functions (same logic as unit tests)
  - Extended Design test to verify computed flat planes/centers match expected asset data
  - Test compares 180 pieces and verifies all planes and centers match within tolerance
