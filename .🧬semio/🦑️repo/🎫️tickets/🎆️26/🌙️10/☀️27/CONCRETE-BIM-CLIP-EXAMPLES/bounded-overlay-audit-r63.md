# Bounded Parking Overlay Audit R63

Independent read-only review,2026-10-29. Root authorized R40 to change the TEST oracle only after R62 measured the actual104-polygon bottom-curtain GEOS union stall. This agent does not edit tests, production, fixture geometry or source c53. Actual helper/portable outcomes pending R40 delivery.

## Equivalence Criteria

For actual unionA and expected unionB, tile a COMMON extent covering BOTH inputs into1m cells with disjoint interiors. Within each cellC, compute union of every STRtree-selected input intersection withC, then area of their symmetric difference. Since intersection distributes over union and symmetric difference, this local set is exactlyC∩(A△B). Summing local areas equals global area(A△B); shared tile boundaries have zero area. This handles overlapping same-side input polygons without double counting, unlike naive sums of per-polygon missing areas.

Candidate queries must be exact AABB/intersects broad phases, not guessed proximity. Clip actual and expected geometry to the SAME cell before local union. Derive common integer cell coverage using floor(min coordinates) and ceil(max coordinates) over both collections, including negatives and expected-only extents. Empty candidates are valid; one empty side must still expose missing/extra material. Keep every positive-area clipped piece, with no snapping, coordinate rounding or small-area cutoff. Preserve the original total area assertion<1e−8 rather than granting each cell its own1e−8budget.

## Captured Pathology Reference

Exact actual input generated/parking-profile-current-union-r62.json contains104valid polygons, original union call657. Group is CW-1/curtain/frame/z0..0.025. Root independently measured the global union with Node polygon-clipping: area0.4594722475880584m² in0.00596s. The bounded GEOS helper should reproduce this union area and zero same-input XOR without attempting the pathological global GEOS union again. No full building_geometry rerun is needed for this proof.

## Required Portable Cases

Check overlapping same-side polygons against equivalent decompositions; missing/extra patches on opposite sides; expected-only extent outside the actual bounds; negative coordinates and exact tile-edge vertices; positive tiny slivers retained rather than thresholded; identical captured104group. A nonzero synthetic XOR must agree with an independent ordinary global GEOS union for small well-behaved cases. The captured group's union area should agree with root's independent Node engine. Actual results and helper source review will be appended.

## Actual Helper Review and Failed Completion Gate

Reviewed actual _tiled_xor_area at parking.py19–42. It satisfies common-bounds coverage, strict>0area retention, STRtree exact-intersects candidates, clipping before local union and one summed XOR area. The source oracle retains its original total<1e−8assertion. There is no snapping, rounding or per-cell tolerance budget in the helper.

However the independent runtime attempt did NOT complete its captured104-group validation before the55s observation timeout. No PASS was claimed. After cancelling that isolated attempt, runtime-only wrappers around the exact helper’s box construction and original shapely.ops.unary_union locate the new active hotspot: local union14, cell[10,4,11,5],17clipped actual polygons, starting2.182s into the bounded source-group profile. Exact local input is generated/bounded-overlay-current-local-r63.json. Its east-entrance endpoint location is consistent with the near-collinear ownership slivers; no geometry defect is inferred merely from GEOS runtime.

This separates set-theoretic correctness from actual engine completion: the proposed1mtiling does not yet satisfy the measured performance gate. R40/root were notified immediately. Runtime wrappers preserve the actual helper/GEOS return values and do not modify production/test source. Retained input bounded-overlay-profile-r63.py; live phase/events generated/bounded-overlay-profile-r63.json and bounded-overlay-events-r63.json.

## Independent Portable Runtime Results

Executed the actual helper independently on the five source fixture cases plus four additional read-only synthetic cases. All nine match the ordinary small-case global GEOS oracle, including overlapping same-side coverage, missing/extra area, equal-area displacement, cross-cell hole, expected-only negative extent, overlapping negative-coordinate decomposition, both-empty inputs and a positive1e−10m²sliver. The tiny sliver remains nonzero; no positive-area threshold was hidden.

```json
{
  "rows": [
    {
      "id": "same-side-overlap",
      "actual": 0,
      "global_oracle": 0,
      "expected": 0
    },
    {
      "id": "missing",
      "actual": 1,
      "global_oracle": 1,
      "expected": 1
    },
    {
      "id": "extra",
      "actual": 1,
      "global_oracle": 1,
      "expected": 1
    },
    {
      "id": "equal-area-displacement",
      "actual": 2,
      "global_oracle": 2,
      "expected": 2
    },
    {
      "id": "cross-cell-hole",
      "actual": 1,
      "global_oracle": 1,
      "expected": 1
    },
    {
      "id": "expected-only-negative",
      "actual": 4,
      "global_oracle": 4
    },
    {
      "id": "overlap-negative",
      "actual": 0,
      "global_oracle": 0
    },
    {
      "id": "tiny-positive-sliver",
      "actual": 1.000000082740371e-10,
      "global_oracle": 1.000000082740371e-10
    },
    {
      "id": "both-empty",
      "actual": 0,
      "global_oracle": 0
    }
  ],
  "status": "PASS"
}
```

These synthetic passes confirm the helper set algebra and edge cases, but they do not override the still-running captured104-group performance failure.

## Final Captured-Group Disposition

The original localGEOS union14 of17clipped polygons in cell[10,4,11,5] remained unchanged for198.40s. Terminated only this isolated profile worker; captured104-group validation did not complete, and no overall tiled-helperPASS or cohort-tail resume is justified from it. The exact localGeoJSON, phase/events and cancellation metadata are retained. Root/R40 also report a separately bounded binary-order experiment failing to complete the same local set. The approved algebra remains valid, but this GEOS runtime path is unsuitable for the observed fixture without another authorized test-interface decision.
