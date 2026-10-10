@capability-bim-1-export-ifc-energy
@oracle-bim-1-ifcopenshell-ifc-energy
@comparison-floating-point-v1
Feature: Open the thermal property sets of the IFC 2x3 and IFC4 exports of the BIM house with IfcOpenShell and check them against the standard templates
  The subject writes the conditions of a space as `Pset_SpaceThermalRequirements` (heating and cooling set point in kelvin, air conditioning, outdoor air as air changes per hour), the
  U-value of a wall, slab and roof as `ThermalTransmittance` of the common sets (the area-weighted value of the envelope surfaces the element holds; IFC 2x3 has no such property in
  `Pset_RoofCommon`), and the thermal data of a window or door type as `ThermalTransmittance`, `GlazingAreaFraction` and `SolarHeatGainTransmittance` of the occurrence, wherever the model has no authored property of that
  name (an authored property wins, and a derived property is listed in the `DerivedRows` row of its element). Exact copies travel in the `Conditions` row of the space and the `UValue`, `GValue` and
  `FrameFraction` rows of the type. The IfcOpenShell oracle never sees the subject's writer: it opens the committed files, reads every thermal property with its IFC value type, checks each against the
  official property set templates of the schema, restates the rules from the snapshot alone (kelvin set points, air changes from the clear height of the space, type values, ISO 6946 bounds of the
  layer stacks, authored values kept, derived values listed) and runs the EXPRESS rules. The subject reports the same table from its document. The committed files are written by the subject's export test
  (`BIM_BLESS=1`), never by hand.

  @id-export-ifc-energy-2x3
  @level-quick
  @mode-differential
  Scenario: The IFC 2x3 file of the house carries the thermal property sets the templates define and keeps the authored values
    Given the committed house model shared://🏗️ifc/🔥️energy/📸️snapshot/🔣️.json and its IFC 2x3 export shared://🏗️ifc/🔥️energy/energy-2x3.ifc
    When the file is opened, audited against the templates and measured
    Then the thermal table equals the subject's within 1e-9

  @id-export-ifc-energy-4
  @level-quick
  @mode-differential
  Scenario: The IFC4 file of the house carries the same thermal data and a roof U-value
    Given the committed house model shared://🏗️ifc/🔥️energy/📸️snapshot/🔣️.json and its IFC4 export shared://🏗️ifc/🔥️energy/energy-4.ifc
    When the file is opened, audited against the templates and measured
    Then the thermal table equals the subject's within 1e-9
