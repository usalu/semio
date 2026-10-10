@capability-bim-1-export-gbxml
@oracle-bim-1-lxml-gbxml
@comparison-floating-point-v1
Feature: Open the gbXML 7.03 export of a BIM thermal envelope with lxml, shapely and numpy and measure every surface again
  The subject writes the thermal model of the committed snapshots as a gbXML 7.03 document through the `s.stdio.xml` dialect: a `Campus` per site with its `Location`, a `Building` with its
  `BuildingStorey`s and `Space`s (area, volume, people, lighting, equipment and outdoor air per area, the condition type from the set points), the thermal zones with `DesignHeatT` and `DesignCoolT`,
  every surface of the inferred `energy-envelope` as a `Surface` (the surface type by what lies behind it, one `AdjacentSpaceId` for an outer surface and two for a partition shared by two spaces,
  `RectangularGeometry`, `PlanarGeometry`, `CADObjectId` the id of the envelope surface) with its windows and doors as `Opening`s, the constructions with their `Layer`s and `Material`s and the
  `WindowType`s with U-value and solar heat gain coefficient. The polygons are turned so +Y is true north. The lxml, shapely and numpy oracle never sees the subject's writer: it opens the committed
  files as namespaced XML (and validates them with the official XSD when that is committed), checks ids, references, units, ranges and the one-description-per-partition rule, recomputes every
  normal, area, azimuth, tilt, bounding rectangle and opening position from the polygons, closes every space (the outward area vectors sum to zero, the divergence theorem gives the volume, the floors give
  the area) and regroups the surfaces by space, kind, boundary, neighbour and compass sector to compare area and heat loss with the table of the energy inference oracle. The subject reports the same
  table from the plan it writes. The committed files are written by the subject's export test (`BIM_BLESS=1`), never by hand.

  @id-export-gbxml-box
  @level-quick
  @mode-differential
  Scenario: A closed room with a window and a door, turned by true north minus the building rotation, is written as one space with six surfaces and two openings
    Given the committed box model shared://💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json and its export shared://🌿️gbxml/🏠️box/gbxml.xml and the inference table shared://💡️inferences/🌡️energy-envelope/🏠️box/💡️inference/🌡️energy-envelope/🔣️.json
    When the file is parsed, audited and measured
    Then the counts, spaces, surfaces, constructions, window types and totals equal the subject's table within 1e-9

  @id-export-gbxml-zoning
  @level-quick
  @mode-differential
  Scenario: Two rooms with different set points share one partition that is described once with two adjacent spaces
    Given the committed zoning model shared://💡️inferences/🌡️energy-envelope/🏘️zoning/📸️snapshot/🔣️.json and its export shared://🌿️gbxml/🏘️zoning/gbxml.xml and the inference table shared://💡️inferences/🌡️energy-envelope/🏘️zoning/💡️inference/🌡️energy-envelope/🔣️.json
    When the file is parsed, audited and measured
    Then the counts, spaces, surfaces, constructions, window types and totals equal the subject's table within 1e-9

  @id-export-gbxml-stack
  @level-quick
  @mode-differential
  Scenario: A cellar, a ground floor and an upper floor under a roof are written with underground walls, interior floors shared by two spaces and an exterior roof
    Given the committed stack model shared://💡️inferences/🌡️energy-envelope/🧱️stack/📸️snapshot/🔣️.json and its export shared://🌿️gbxml/🧱️stack/gbxml.xml and the inference table shared://💡️inferences/🌡️energy-envelope/🧱️stack/💡️inference/🌡️energy-envelope/🔣️.json
    When the file is parsed, audited and measured
    Then the counts, spaces, surfaces, constructions, window types and totals equal the subject's table within 1e-9

  @id-export-gbxml-house
  @level-quick
  @mode-differential
  Scenario: The family house with its conditions, thermal zones, windows and doors is a closed, consistent gbXML document
    Given the committed house model shared://🌿️gbxml/🏠️house/📸️snapshot/🔣️.json and its export shared://🌿️gbxml/🏠️house/gbxml.xml
    When the file is parsed, audited and measured
    Then the counts, spaces, surfaces, constructions, window types and totals equal the subject's table within 1e-9
