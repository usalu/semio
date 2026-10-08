@capability-bim-1-export-svg
@oracle-bim-1-lxml-shapely-svg
@comparison-floating-point-v1
Feature: Open the SVG floor plans of the BIM house with lxml and measure them with shapely
  The subject writes the committed house model (four storeys with joined, free and curved walls, windows, doors, columns, a slab with a stair opening, a roof, stairs,
  spaces and grid lines) as one SVG 1.1 sheet at 1:100 in paper millimetres, one group per storey. The lxml and shapely oracle never sees the subject's writer: it opens
  the committed file as namespaced XML, finds the storey groups, counts the regions, lines and texts of each, the paths in each line-style class (cut, projection, hidden,
  annotation) and the arc commands, reads every path with its own SVG path reader and measures with GEOS the even-odd area of the straight cut poche regions and the
  length of the straight lines per style, converted from paper millimetres to model metres. The subject reports the same table from its plans and the sheet frames its
  writer snaps the coordinates with. Curved geometry is counted but not measured here (GEOS only samples arcs); the sampled poche area including arcs is committed as an
  audit value and held to the closed form of the bulges by the subject's unit tests.

  @id-export-svg-house
  @level-quick
  @mode-differential
  Scenario: Group, path, arc, area and length tables of the exported house equal the subject's report
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its export shared://🚪️svg/🏠️house/🏠️house.svg
    When the file is parsed and every storey group is counted and measured
    Then the storey groups, the path counts per style, the arcs, the poche areas and the line lengths equal the subject's within 1e-9
