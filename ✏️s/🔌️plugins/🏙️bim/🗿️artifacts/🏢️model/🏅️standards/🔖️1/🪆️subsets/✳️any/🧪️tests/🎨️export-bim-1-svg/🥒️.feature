@capability-bim-1-export-svg
@oracle-bim-1-lxml-shapely-svg
@comparison-floating-point-v1
Feature: Open the SVG views of the BIM house with lxml and measure them with shapely
  The subject writes the committed house model (four storeys with joined, free and curved walls, windows, doors, columns, a slab with a stair opening, a roof, stairs,
  spaces and grid lines) and its authored views (a plan per storey, two sections, four elevations and a camera) as one SVG 1.1 sheet in paper millimetres, one group per drawn view at the scale of the view; the camera is not drawn. The lxml and shapely oracle never sees the subject's writer: it opens
  the committed file as namespaced XML, finds the view groups with their kind, scale and storey, counts the regions, lines and texts of each, the paths in each line-style class (cut, projection, hidden,
  annotation) and the arc commands, reads every path with its own SVG path reader and measures with GEOS the even-odd area of the straight cut poche regions and the
  length of the straight lines per style, converted from paper millimetres to model metres. The subject reports the same table from the `view-linework` of every view and the sheet frames its
  writer snaps the coordinates with. Curved geometry is counted but not measured here (GEOS only samples arcs); the sampled poche area including arcs is committed as an
  audit value and held to the closed form of the bulges by the subject's unit tests.

  @id-export-svg-house
  @level-quick
  @mode-differential
  Scenario: Group, path, arc, area and length tables of the exported house equal the subject's report
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its export shared://🚪️svg/🏠️house/🏠️house.svg
    When the file is parsed and every view group is counted and measured
    Then the view groups, the path counts per style, the arcs, the poche areas and the line lengths equal the subject's within 1e-9

  @id-export-svg-notated
  @level-quick
  @mode-differential
  Scenario: The annotation layer of an annotated room counts its dimension lines, extension lines, marks and texts and measures their lengths as the subject reports
    Given the committed room shared://💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json and its export shared://🚪️svg/🪧️notated/🪧️notated.svg
    When the file is parsed and the paths and texts of its annotation layer are counted per kind and measured
    Then the paths and texts per kind, the straight lengths of the dimension, extension and leader lines and the sorted printed texts equal the subject's within 1e-9

  @id-export-svg-components
  @level-quick
  @mode-differential
  Scenario: The plan symbols of the components and routed MEP elements, their centre lines, bands, outlines and the colour of their service, equal the subject's report
    Given the committed components room shared://🏗️ifc/🪑️components/📸️snapshot/🔣️.json and its export shared://🚪️svg/🪑️components/🪑️components.svg
    When the file is parsed and the paths of every component and routed element are read, the centre lines and the areas of outlines and bands measured with shapely and the stroke colours collected
    Then the paths per kind, the centre line lengths, the outline and band areas and the stroke colours of every element equal the subject's within 1e-9
