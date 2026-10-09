@capability-bim-1-export-ifc
@oracle-bim-1-ifcopenshell-ifc
@comparison-floating-point-v1
Feature: Open the IFC 2x3 export of zones, area schemes and room finishes with IfcOpenShell and read them back
  `s.bim.model@1` groups spaces into zones (an `IfcZone` per zone, typed by its category, with `Pset_ZoneCommon.Reference`, the authored occupancy density in
  `Semio_Authoring` and the derived totals as the element quantity `Semio_ZoneTotals`, its spaces assigned through `IfcRelAssignsToGroup`), writes every area
  scheme as an `IfcGroup` of type `AreaScheme` (its measure, counted usages and counted zones in `Semio_Authoring`, no members) and names the finish of a space
  in `Pset_SpaceCoveringRequirements`. The IfcOpenShell 0.8.4.post1 oracle never sees the subject's writer: it opens the committed file, requires that it parses
  as IFC2X3 and passes every EXPRESS rule, reads every zone, group and covering with its own property-set reader, and audits them against the committed snapshot
  (member spaces, rule, material names). The subject reports the same table from the snapshot and its inference: zone name, category, reference, density,
  members and the six totals, area scheme rules and covering names. The totals and densities must agree within floating-point-v1.

  @id-export-zoning-zoned
  @level-quick
  @mode-differential
  Scenario: The zones, area schemes and coverings IfcOpenShell reads from the exported zoned house equal the subject's report
    Given the committed zoned house shared://🏗️ifc/🏘️zoned/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🏘️zoned/🏘️zoned.ifc
    When the file is opened and every IfcZone, area scheme group and covering property set is read
    Then the zones with their members and totals, the area scheme rules and the covering names equal the subject's within floating-point-v1
