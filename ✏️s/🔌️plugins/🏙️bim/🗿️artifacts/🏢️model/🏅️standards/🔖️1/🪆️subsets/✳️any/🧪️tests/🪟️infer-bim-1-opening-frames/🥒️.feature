@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer where every window, door and void sits in its host and audit the placement with shapely
  `s.bim.model@1` stores an opening's host, type or void size, centre `offset` along the host axis, optional size
  override, sill and the two flips. `🪟️opening-frames` derives everything else: the resolved width, height and sill
  (an override wins over the type; a sill override replaces the type sill), the point and tangent on the host axis at
  arc length `offset` (line or arc), the local frame, the world frame after the building origin, rotation and datum,
  the cut rectangle in the host's `(s, z)` development, the reveal depth (host thickness), the door swing and window
  glazing as plan strokes, and the validity of the placement (outside the host, above its top, overlapping a
  sibling, host or type missing). The oracle is `🐍️.py` in this directory. It reproduces both tables from the
  committed snapshots, has `shapely` 2 walk a sampled axis for the point and tangent, measure the cut boxes, test
  containment in the host rectangle and intersect sibling cuts, and checks right-handedness with `numpy`. The
  committed expectations are written by that file, never by hand.

  @id-opening-frames-placed
  @level-quick
  @mode-differential
  Scenario: Windows, doors and voids on line, arc and curtain-wall hosts in a rotated building resolve to their frames and cuts
    Given the committed placed model shared://💡️inferences/🪟️opening-frames/🏡️placed/📸️snapshot/🔣️.json
    When 🪟️opening-frames is inferred for it
    Then every opening's frame equals the table shared://💡️inferences/🪟️opening-frames/🏡️placed/💡️inference/🪟️opening-frames/🔣️.json

  @id-opening-frames-invalid
  @level-quick
  @mode-differential
  Scenario: Openings outside their host, above its top, overlapping, untyped or orphaned report their issues
    Given the committed invalid model shared://💡️inferences/🪟️opening-frames/⚠️invalid/📸️snapshot/🔣️.json
    When 🪟️opening-frames is inferred for it
    Then every opening's frame equals the table shared://💡️inferences/🪟️opening-frames/⚠️invalid/💡️inference/🪟️opening-frames/🔣️.json
