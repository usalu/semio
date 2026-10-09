@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Re-derive the volume and bounds of every planar column, beam, slab, ceiling, roof, stair and railing with shapely
  Seven authored cases hold a snapshot each: columns of every profile, beams of every profile, slabs and ceilings with
  holes and a slope (a ceiling hangs below the storey top instead of standing on the floor), roofs of every shape over straight
  footprints, concave L, T and U ones included, with their documented flat and hip fallbacks, stairs of every flight kind with
  closed, open and mono stringers, nosing, tread thickness, open and closed risers and deep landings, and railings with authored
  rail and post sections, balusters and glass or panel infill.
  The shapely 2.1.2 (GEOS) oracle never sees the subject's meshes. It recomputes, from the snapshot alone, the
  storey levels, the polygon areas (`Polygon.area`, mitred `buffer` for the roof eaves, `affinity` for placement),
  the collapse depth of a hip or mansard roof (the latest node of the independent `py_straight_skeleton` library, audited by the
  mitred negative `buffer` and, on convex footprints, by a linear program), the tread slabs, riser boards, stringer bands (clipped
  with `shapely`) and landing slabs of the runs the sibling stair-runs oracle derives, and the railing posts, balusters and infill
  slabs (`LineString.interpolate`, footprints rotated by `affinity`), and reports for each planar element its volume, its volume
  per part and its axis-aligned bounds. The subject's tessellated solids must agree within 1e-9.
  Elements with tessellated arcs are audited inside the subject's own unit tests within the chord tolerance; an
  element without geometry is absent on both sides.

  @id-solids-rest
  @level-quick
  @mode-differential
  Scenario: Volume and bounds of every planar element equal the oracle's closed forms
    Given the committed frame cases shared://💡️inferences/🧊️element-solids/🏛️columns-profiles/🔣️.json and shared://💡️inferences/🧊️element-solids/➖️beams-profiles/🔣️.json
    And the committed horizontal cases shared://💡️inferences/🧊️element-solids/⬜️slabs-holes-slope/🔣️.json, shared://💡️inferences/🧊️element-solids/🔲️ceilings-holes-slope/🔣️.json, shared://💡️inferences/🧊️element-solids/🪵️ceilings-meshes/🔣️.json and shared://💡️inferences/🧊️element-solids/🏔️roofs-shapes/🔣️.json
    And the committed circulation cases shared://💡️inferences/🧊️element-solids/🪜️stairs-flights/🔣️.json and shared://💡️inferences/🧊️element-solids/🛤️railings-posts/🔣️.json
    When every planar element of each case is measured
    Then the volume and the bounds equal the oracle's within 1e-9
