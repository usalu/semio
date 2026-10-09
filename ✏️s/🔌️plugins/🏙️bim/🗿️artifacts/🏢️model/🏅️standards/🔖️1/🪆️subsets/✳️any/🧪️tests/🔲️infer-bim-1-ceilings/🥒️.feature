@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the take-off, the span and the underside of every ceiling and audit them with shapely
  `s.bim.model@1` stores a ceiling as a boundary loop with holes, the distance its top hangs below the top of its storey (`offset`), an optional slope and a ceiling type whose
  layers stack downward from there. The take-off (`🧮️quantities`) derives the gross area (the boundary), the net area (the boundary less its holes), the sloped surface area
  (`net / cos(angle)`), the perimeter of the boundary and every hole, the width (the sum of the layer thicknesses), the gross and net volume and the mass of the layers. The solid
  (`🧊️element-solids`) gives the vertical span: the top at the storey top less the offset and the bottom a fall lower, the fall being the extent of the boundary along the fall direction
  times `tan(angle)`. The underside at a plan point, the number `🏠️spaces` takes the clear height from, is the top plane (which keeps its drop at the uphill edge) less the width, and is
  absent in a hole or outside the boundary. A ceiling whose type is unknown or has no layers has no row. The oracle is `🐍️.py` in this directory. It reproduces the whole table from the
  committed snapshot: `shapely` 2 builds the region of the boundary with its holes (arcs sampled into 4096 chords), audits the closed-form areas and the perimeter against
  `Polygon.area` and `Polygon.length` and answers the probe with `Polygon.contains`. The committed expectation is written by that file, never by hand.

  @id-ceilings-takeoff
  @level-quick
  @mode-differential
  Scenario: A ceiling with a hole, two sloped ceilings, a half-round ceiling, a bare type and an unknown type resolve to their areas, volumes, mass, span and underside
    Given the committed ceilings model shared://💡️inferences/🔲️ceilings/🔲️takeoff/📸️snapshot/🔣️.json
    When the take-off and the span of its ceilings are inferred
    Then every ceiling's row equals the table shared://💡️inferences/🔲️ceilings/🔲️takeoff/💡️inference/🔲️ceilings/🔣️.json
