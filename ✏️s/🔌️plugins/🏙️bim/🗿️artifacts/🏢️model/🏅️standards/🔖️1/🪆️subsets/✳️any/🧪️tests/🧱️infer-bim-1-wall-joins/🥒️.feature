@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the face curves, joins and join-trimmed footprints of every wall and audit them with shapely
  `s.bim.model@1` stores a wall's axis (line or arc), type, location line and base/top constraints. `🧱️wall-layout`
  derives the distances from the axis to its two faces (layers run from the left, interior face to the right, exterior
  face along the axis; Center, Interior, Exterior and CoreCenter place the axis on the mid plane, the left face, the right
  face and the middle of the Core layers), the offset curves (parallel lines, concentric arcs) and, among the walls of
  one storey, the joins: axis ends within a micrometre form a node mitered pairwise around it, an end on another axis
  butts against the near face (the end edge follows a curved face), two axes crossing in both interiors form a cross that
  trims nothing. The oracle is `🐍️.py` in this directory. It reproduces the whole table from the committed snapshot with
  an analytic route and has `shapely` 2 re-derive every straight corner from GEOS `offset_curve` carriers and measure the
  footprint polygons: validity, areas, face lengths, the absence of overlap between joined walls, the cross overlap, the
  `buffer(join_style="mitre")` identity of two-wall nodes and the `difference` identity of butts. The committed
  expectations are written by the sibling oracle `../🪜️infer-bim-1-levels-and-wall-heights/🐍️.py`, never by hand.

  @id-wall-joins
  @level-quick
  @mode-differential
  Scenario: Miters, butts, crosses, nodes of three and four walls, tangent and right-angle arcs and every location line resolve to their footprints
    Given the committed joins model shared://💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json
    When 🧱️wall-layout is inferred for it
    Then every wall's layout equals the table shared://💡️inferences/🔗️wall-joins/💡️inference/🧱️wall-layout/🔣️.json
