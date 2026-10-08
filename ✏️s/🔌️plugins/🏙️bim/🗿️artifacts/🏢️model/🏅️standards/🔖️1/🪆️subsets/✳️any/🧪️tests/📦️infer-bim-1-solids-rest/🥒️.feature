@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Re-derive the volume and bounds of every planar column, beam, slab, roof, stair and railing with shapely
  Six authored cases hold a snapshot each: columns of every profile, beams of every profile, slabs with holes and
  a slope, roofs of every shape with their documented flat fallbacks, stairs of every flight kind and railings.
  The shapely 2.1.2 (GEOS) oracle never sees the subject's meshes. It recomputes, from the snapshot alone, the
  storey levels, the polygon areas (`Polygon.area`, mitred `buffer` for the roof eaves, `affinity` for placement),
  the largest inscribed distance of a hip roof (a linear program audited by `polylabel`), the stair prisms from the
  documented `stair-runs` contract and the railing post positions (`LineString.interpolate`), and reports for each
  planar element its volume and its axis-aligned bounds. The subject's tessellated solids must agree within 1e-9.
  Elements with tessellated arcs are audited inside the subject's own unit tests within the chord tolerance; an
  element without geometry is absent on both sides.

  @id-solids-rest
  @level-quick
  @mode-differential
  Scenario: Volume and bounds of every planar element equal the oracle's closed forms
    Given the committed frame cases shared://💡️inferences/🧊️element-solids/🏛️columns-profiles/🔣️.json and shared://💡️inferences/🧊️element-solids/➖️beams-profiles/🔣️.json
    And the committed horizontal cases shared://💡️inferences/🧊️element-solids/⬜️slabs-holes-slope/🔣️.json and shared://💡️inferences/🧊️element-solids/🏔️roofs-shapes/🔣️.json
    And the committed circulation cases shared://💡️inferences/🧊️element-solids/🪜️stairs-flights/🔣️.json and shared://💡️inferences/🧊️element-solids/🛤️railings-posts/🔣️.json
    When every planar element of each case is measured
    Then the volume and the bounds equal the oracle's within 1e-9
