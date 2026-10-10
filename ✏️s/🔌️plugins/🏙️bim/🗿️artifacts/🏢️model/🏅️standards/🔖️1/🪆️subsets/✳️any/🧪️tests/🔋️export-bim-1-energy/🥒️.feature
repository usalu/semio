@capability-bim-1-export-energy
@oracle-bim-1-jsonschema-numpy-energy
@comparison-floating-point-v1
Feature: Open the energy model of a BIM room with jsonschema and measure it again with shapely and numpy
  The subject writes the thermal envelope and the conditions of a BIM model as the snapshot of `s.energy.model@1`, the document the energy artifact simulates: a zone per space with its volume and floor
  area, a surface per wall, floor and ceiling with its polygon, its boundary (outdoor air, ground, adiabatic, or an interzone partner that points back) and its layered construction listed outside first,
  a fenestration per window and exterior door with its U-value and solar heat gain, people, lighting and equipment gains with their schedules, a thermostat and an ideal loads system per conditioned space.
  The oracle never sees the writer. It opens the committed JSON file, validates it with `jsonschema` against the schema of the energy artifact's snapshot and against the strict schema of the export (the
  field layout of the energy engine's model), audits the references and the partner surfaces, and measures the model again from its polygons alone: `numpy` gives the Newell area and normal of every
  polygon, the volume of every zone by the divergence theorem, the azimuth of every wall from the north axis and the ISO 6946 transmittance of every construction from its layers (`R_T = R_si + sum(d / lambda) + R_se`),
  `shapely` the footprint of the floors, and the conditioned zones of every scope add up to the envelope area A, A/V, H_T, H'_T and the areas by compass sector. It audits that table against the one the
  sibling oracle of the `energy-envelope` inference measures from the same snapshot with its own shapely rooms (a window or door behind a partition is merged into its wall, the export has none). The
  subject reports the same table from its inference. The committed files are written by the subject's export test (`BIM_BLESS=1`), never by hand.

  @id-energy-export-room
  @level-quick
  @mode-differential
  Scenario: A closed insulated room with a window and an exterior door resolves to the zone, surfaces, constructions and totals of its energy model
    Given the committed room shared://🚪️energy/🏠️room/📸️snapshot/🔣️.json and its export shared://🚪️energy/🏠️room/🔋️model.json
    When the file is validated, audited and measured
    Then the spaces and the totals of every scope equal the subject's within 1e-9

  @id-energy-export-pair
  @level-quick
  @mode-differential
  Scenario: Two rooms in two zones with different set points are two interzone partners and the partition door is merged into its wall
    Given the committed pair shared://🚪️energy/🏘️pair/📸️snapshot/🔣️.json and its export shared://🚪️energy/🏘️pair/🔋️model.json
    When the file is validated, audited and measured
    Then the spaces and the totals of every scope equal the subject's within 1e-9

  @id-energy-export-stack
  @level-quick
  @mode-differential
  Scenario: A cellar, a ground floor and an upper floor under a roof are ground walls, adiabatic and adjacent floors and an exterior roof
    Given the committed stack shared://🚪️energy/🧱️stack/📸️snapshot/🔣️.json and its export shared://🚪️energy/🧱️stack/🔋️model.json
    When the file is validated, audited and measured
    Then the spaces and the totals of every scope equal the subject's within 1e-9
