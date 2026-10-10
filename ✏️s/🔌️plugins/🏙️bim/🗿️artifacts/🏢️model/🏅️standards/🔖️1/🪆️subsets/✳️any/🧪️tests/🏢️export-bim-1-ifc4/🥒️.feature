@capability-bim-1-export-ifc4
@oracle-bim-1-ifcopenshell-ifc4
@comparison-floating-point-v1
Feature: Open the IFC4 export of the BIM model with IfcOpenShell, validate and measure it
  The subject writes the committed models (the house with joined, free and curved walls, windows, doors, columns, beams, slabs, a roof, stairs, a railing, a curtain wall and spaces; the psets model with property
  set templates and classification chains; ceilings; an annotated room; ramps; an attic with attached walls, sweeps and reveals) as IFC4 (ADD2 TC1) files: no owner history, window and door types with their
  partitioning and operation, roof types, bodies as triangulated face sets, the property set templates and classification systems of the project library declared by the project, and the property sets named like a
  template related to it. The IfcOpenShell 0.8.4.post1 oracle never sees the subject's writer: it opens the committed file, requires the schema IFC4, validates every attribute type and every EXPRESS WHERE rule,
  counts the entities of each class, reads the spatial containment of every product, tessellates every straight wall, level slab, column, beam and covering with its own C++ geometry kernel and requires the
  volume to equal the written base quantity, reads the operation of every door, the partitioning of every window, the classification chains and the type property sets. The subject reports the same table
  from its own document and its base quantities. A second scenario per model exports it, imports the file with the subject's own reader and exports the result again: the table of the second file is the table the
  oracle measured in the first.

  @id-export-ifc4-house
  @level-quick
  @mode-differential
  Scenario: Entity counts, containment and kernel volumes of the IFC4 house equal the subject's report
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🏠️house/🏠️house.ifc
    When the file is validated against the IFC4 schema and every product is counted, located and measured
    Then the counts per IFC class, the containment per storey, the net volumes and the door and window types equal the subject's within 1e-9

  @id-export-ifc4-psets
  @level-quick
  @mode-differential
  Scenario: The project library, the classification chains, the template relations and the type property sets of the IFC4 psets model equal the subject's report
    Given the committed psets model shared://🏗️ifc/🏷️psets/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🏷️psets/🏷️psets.ifc
    When the file is validated and every IfcPropertySetTemplate, IfcClassification chain, IfcRelDefinesByTemplate and type property set is read
    Then the template types, applicable entities, measure types, enumerated values, definition lists, reference parents in sort order and attached codes equal the subject's

  @id-export-ifc4-ceilings
  @level-quick
  @mode-differential
  Scenario: Every IFC4 ceiling is an IfcCovering whose flat sweep IfcOpenShell tessellates to the volume of its written base quantities
    Given the committed ceilings shared://🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🔲️ceilings/🔲️ceilings.ifc
    When the file is validated and every IfcCovering is counted, located in its storey and measured by the kernel
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9

  @id-export-ifc4-notated
  @level-quick
  @mode-differential
  Scenario: Every dimension, tag, note and leader of the annotated room is an IfcAnnotation in the IFC4 file with its kind, printed text, curves and literals
    Given the committed room shared://💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🪧️notated/🪧️notated.ifc
    When the file is validated and every IfcAnnotation is located in its storey and its Annotation2D representation and quantities are read
    Then the counts per IFC class, the containment per storey and the annotations equal the subject's within 1e-9

  @id-export-ifc4-ramps
  @level-quick
  @mode-differential
  Scenario: Every IFC4 ramp aggregates its flights, landings and railings whose classes and counts IfcOpenShell reads back
    Given the committed ramps shared://💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🛝️ramps/🛝️ramps.ifc
    When the file is validated and every IfcRamp is located in its storey and decomposed into its parts
    Then the counts per IFC class and the containment per storey equal the subject's

  @id-export-ifc4-wall-depth
  @level-quick
  @mode-differential
  Scenario: Walls under roofs, on a sloped slab and with sweeps and reveals are valid IFC4 products with the counts and containment the subject reports
    Given the committed attic shared://💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🧗️wall-depth/🧗️wall-depth.ifc
    When the file is validated and every wall, every IfcMember sweep run and every IfcRelConnectsElements is counted and located
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9

  @id-roundtrip-ifc4-house
  @level-quick
  @mode-round-trip
  Scenario: The house exported, imported and exported again measures like the committed IFC4 file
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🏠️house/🏠️house.ifc
    When the subject imports its own export and exports the result again while the committed file is measured
    Then the counts per IFC class, the containment per storey and the net volumes of the second file equal the oracle's measurement of the first within 1e-9

  @id-roundtrip-ifc4-psets
  @level-quick
  @mode-round-trip
  Scenario: The psets model exported, imported and exported again keeps its templates and classification chains
    Given the committed psets model shared://🏗️ifc/🏷️psets/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🏷️psets/🏷️psets.ifc
    When the subject imports its own export and exports the result again while the committed file is measured
    Then the counts per IFC class, the classification tables and the type property sets of the second file equal the oracle's measurement of the first

  @id-stepped-ifc4-house
  @level-quick
  @mode-differential
  Scenario: The house written by the stepped IFC4 export job after a cancelled attempt has the counts, containment and kernel volumes IfcOpenShell measures in the committed file
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its IFC4 export shared://🏢️ifc4/🏠️house/🏠️house.ifc
    When an IFC4 export job is cancelled half-way, a second job on the same inference session runs to its end and its file is read back, while the committed file is opened and measured
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9
