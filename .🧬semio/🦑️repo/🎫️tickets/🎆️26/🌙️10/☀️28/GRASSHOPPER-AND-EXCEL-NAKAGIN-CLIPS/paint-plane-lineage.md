# Paint Plane Lineage: Actual Red to Green Accepted

The independent ray oracle has a source-traced conditioning gap. FragmentPaint checks the exported normal for finite nonzero values but then reconstructs a plane from each tiny fragment. Its three-vertex source_facets branch can return an empty tuple when that reconstruction fails a 64-ULP self-plane check. The recovered BSP sliver is the portable counterexample candidate. This is not yet an executed NumPy failure; the auditor explicitly performed source-only review.

The strict language-neutral input is pixel-plane-lineage-cases.json with pixel-plane-lineage.schema.json. It retains the original source5860 octagon, the exact derived positive triangle, the stable source normal, expected one material triangle and unchanged64-ULP policy. AJV accepts it and rejects three malformed controls. validate-paint-lineage.py independently records positive NumPy area and distinguishes retained material, silent positive-material drop and explicit rejection.

The focused baseline did not run in41373: that attempt stopped on a separate ray-boundary helper failure after its nine-test record. The helper defect is independently diagnosed and corrected without a kernel change. The focused paint baseline is now appended after owner-held retry35286: kernel10 and cached2,443-face stages. Those stages finalize their own source-bound durable records first. A later deliberate oracle RED therefore cannot be confused with a kernel result. No duplicate process was submitted.

If the baseline confirms the issue, the correction will associate each paint piece with a carrier reconstructed independently from its original physical polygon/canonical warped facets, validate its declared stable plane there, and use that carrier for paint triangulation/projection and depth. Positive material must either remain represented or produce an explicit diagnostic error; an empty fallback is prohibited. Physical-nearest geometry and facing continue to derive only from original canonical vertices/winding, never paint normals. Source/polygon aliases and compound rank/piece alpha union remain unchanged.

No renderer, physical model or kernel change belongs to this oracle correction.

## Retry Stage Disposition

35286 accepted kernel10/actual2,443 first, then the paint probe failed beforeimport because its model-schema dependency path omitted 🧬️schema/. It did not run geometry and is not a paint RED. The path is corrected; isolated baseline79208 is queued under normal2-GB admission. No oracle implementation change has been made.

## Current Result

The isolated baseline79208 executed and confirmed silent positive-material loss: area6.211953942447299e−11 but zero triangles, exact endpoints unchanged. The ticket-only original-carrier correction retains one exact triangle, with independent original-source SVD depth error5.551115123125783e−17. The complete13-test independent forward/original oracle suite passes0.815s on the final helper/driver map; all three final finite queries also pass their physical point checks. See final-carrier-visibility.md for complete source maps, hashes, limits and the two unchanged Cairo boundary ownership discrepancies. No renderer, model or kernel source changed for this oracle correction.
