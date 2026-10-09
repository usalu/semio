@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the drawing of every section and elevation of a room and audit its cut areas, projection outlines and datum lengths with shapely
  `s.bim.model@1` stores authored parameters only. `🖼️view-linework` derives, per authored view, the drawing of a section or elevation: the vertical plane of the view cuts every solid
  it passes through (the section cut, a filled region per solid), everything inside the slab of the view behind the plane is projected (the silhouettes) and the storey levels run across the
  drawing as datum lines, all in the coordinates of the view (`u` along the plane, `z` above the building datum). The view hides categories, filters by phase, limits its depth and crops its
  drawing. The oracle is `🐍️.py` in this directory. It rebuilds the walls (flat-capped bands and the mitred ring of a closed room) and the rotated columns as real shapely polygons from the
  committed snapshot, cuts them with the plane as a `LineString`, clips them to the slab of the view with a polygon intersection, projects every connected piece to its extent along the plane,
  unites the projections with `unary_union`, clips to the crop with `box`, and sums the storey levels as datum lines. It proves the parametric law as a metamorphic property: turning scene and
  planes together leaves every measure unchanged. Edges are decided by hidden-line removal, which no third-party library offers; the subject pins them in its unit tests. The committed
  expectation is written by that file, never by hand.

  @id-view-metrics-room
  @level-quick
  @mode-differential
  Scenario: The sections and elevations of a two-storey room measure as shapely measures their cut, projection and datums
    Given the committed room shared://💡️inferences/🖼️view-linework/🏠️room/📸️snapshot/🔣️.json
    When 🖼️view-linework is inferred for it
    Then every section's and elevation's measures equal the table shared://💡️inferences/🖼️view-linework/🏠️room/💡️inference/📐️view-metrics/🔣️.json
