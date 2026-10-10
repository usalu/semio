@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer walls attached to roofs and slabs, wall sweeps and authored opening reveals and audit them with shapely
  `s.bim.model@1` stores the top of a wall as a reference to the underside of a roof, a slab or a ceiling plus an offset, its base as a reference to the top of a slab, a wall sweep as a profile
  run along one face of a wall at a height above its base, and the reveal of an opening as the depth the frame is set back from the front face and a material. Nothing derived is stored. The subject
  infers, per attached wall, the lowest base, the highest top and the area of the elevation along the axis (and the volume of a free wall of constant thickness), per sweep the length of its path
  less the stretches the openings of its host interrupt, the cross-section, the volume and the visible surface, and per reveal the area of the jambs, the head and the sill and the lateral planes of
  the frame. The oracle is `🐍️.py` in this directory: it reproduces the whole table from the committed snapshot alone with `shapely` 2, re-deriving the height of a hip roof from the distance to the
  nearest edge of its footprint (`Polygon.exterior.distance`), of a gable roof from the nearest eave edge, of a sloped slab from the fall of its plane, integrating the piecewise linear elevation
  exactly (refined at every kink), cutting a sweep path with `LineString.difference` and measuring the reveal with `Polygon.length`. The committed expectation is written by that file, never by hand.

  @id-wall-depth
  @level-quick
  @mode-differential
  Scenario: Walls under a gable and a hip roof, on a sloped slab, with baseboards, a hand rail and a window reveal resolve to their extremes, areas, lengths and reveals
    Given the committed attic model shared://💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json
    When the attached walls, the sweeps and the reveals of the model are inferred
    Then every row equals the table shared://💡️inferences/🧗️wall-depth/🏠️attic/💡️inference/🧗️wall-depth/🔣️.json
