@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the thermal envelope of every space and what it adds up to and audit it with shapely and numpy
  `s.bim.model@1` stores the thermal conditions of a space (occupancy, set points, ventilation, lighting and equipment power density, schedule), the U-value, g-value and frame fraction
  of a window type and the U-value of a door type; a wall, slab or roof keeps only its layer stack, and the conductivity of a layer is the one of its material. `🌡️energy-envelope` derives
  the envelope: every edge of a room is classified by the wall it lies on and the room behind it (exterior, ground below the datum, adjacent, adiabatic when both rooms have the same set
  points), floors and ceilings are split by the rooms below and above, each surface gets its net area (openings are surfaces of their own), azimuth from true north and the building
  rotation, tilt and a U-value after ISO 6946 (`R_T = R_si + sum(d / lambda) + R_se`, surface resistances by the direction of the heat flow), and the conditioned spaces of a zone, a building
  and the project add up to the envelope area A, A/V, the transmission H_T with the correction factors 1.0, 0.6 and 0.5, H'_T with the flat thermal bridge allowance of 0.05, the areas by
  compass sector and the solar aperture. The oracle is `🐍️.py` in this directory. It reproduces the whole table from the committed snapshot: `shapely` 2 gives the rooms through the
  sibling oracle `../🏠️infer-bim-1-spaces`, `contains_xy` samples every edge every centimetre to find the wall and the room behind it, `intersection` splits floors and ceilings, and
  `numpy` evaluates the resistances and the sums. It also audits the closed forms (wall, window and door areas equal the perimeter times the clear height, floor plus ceiling equal twice the
  floor) and the metamorphic laws that thicker insulation never raises the heat loss and that turning the building by a quarter turn moves every sector by two. The committed expectation is
  written by that file, never by hand.

  @id-energy-box
  @level-quick
  @mode-differential
  Scenario: A closed insulated room with a window and a door resolves to its surfaces, U-values and aggregates
    Given the committed box model shared://💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json
    When 🌡️energy-envelope is inferred for it
    Then every space's surfaces and every scope's totals equal the table shared://💡️inferences/🌡️energy-envelope/🏠️box/💡️inference/🌡️energy-envelope/🔣️.json

  @id-energy-zoning
  @level-quick
  @mode-differential
  Scenario: Two rooms in two zones with different set points resolve to an adjacent partition counted by the zones and not by the building
    Given the committed zoning model shared://💡️inferences/🌡️energy-envelope/🏘️zoning/📸️snapshot/🔣️.json
    When 🌡️energy-envelope is inferred for it
    Then every space's surfaces and every scope's totals equal the table shared://💡️inferences/🌡️energy-envelope/🏘️zoning/💡️inference/🌡️energy-envelope/🔣️.json

  @id-energy-stack
  @level-quick
  @mode-differential
  Scenario: A cellar, a ground floor and an upper floor under a roof resolve to ground walls, vertical neighbours and an exterior roof
    Given the committed stack model shared://💡️inferences/🌡️energy-envelope/🧱️stack/📸️snapshot/🔣️.json
    When 🌡️energy-envelope is inferred for it
    Then every space's surfaces and every scope's totals equal the table shared://💡️inferences/🌡️energy-envelope/🧱️stack/💡️inference/🌡️energy-envelope/🔣️.json
