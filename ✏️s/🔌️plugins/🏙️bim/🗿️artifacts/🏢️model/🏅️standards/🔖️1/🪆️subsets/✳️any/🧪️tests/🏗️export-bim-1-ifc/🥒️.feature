@capability-bim-1-export-ifc
@oracle-bim-1-ifcopenshell-ifc
@comparison-floating-point-v1
Feature: Open the IFC 2x3 export of the BIM house with IfcOpenShell and measure it
  The subject writes the committed house model (one site, one building, four storeys, joined, free and curved walls with
  windows, doors and a void, columns, beams, level and sloped slabs, a gable roof, two stairs, a railing, a curtain wall,
  two spaces, grid lines, property sets and classifications) as an IFC 2x3 file. The IfcOpenShell 0.8.4.post1 oracle
  never sees the subject's writer: it opens the committed file, checks that it parses and declares IFC2X3, counts the
  entities of each class, reads the spatial containment of every product, and tessellates every straight wall, level
  slab, column and beam with its own C++ geometry kernel (openings are subtracted by the kernel). The subject reports the
  same table from its own document and its base quantities: class counts, containment per storey and the net volume
  of every element the kernel can measure exactly. Walls on arcs and faceted breps are counted but not measured (the
  kernel tessellates arcs), and all quantities of the file must agree with the kernel within 1e-9.

  @id-export-ifc-house
  @level-quick
  @mode-differential
  Scenario: Entity counts, containment and kernel volumes of the exported house equal the subject's report
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🏠️house/🏠️house.ifc
    When the file is opened and every product is counted, located and measured
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9

  @id-export-ifc-notated
  @level-quick
  @mode-differential
  Scenario: Every dimension, tag, note and leader of an annotated room is an IfcAnnotation that IfcOpenShell reads back with its kind, printed text, curves, literals and total
    Given the committed room shared://💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🪧️notated/🪧️notated.ifc
    When the file is opened and every IfcAnnotation is located in its storey and its Annotation2D representation and quantities are read
    Then the counts per IFC class, the containment per storey and the annotations equal the subject's within 1e-9 and every dimension total equals the distance of the shapely-adjudicated inference table

  @id-export-ifc-ceilings
  @level-quick
  @mode-differential
  Scenario: Every ceiling is an IfcCovering whose flat sweep IfcOpenShell tessellates to the volume of its written base quantities
    Given the committed ceilings shared://🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🔲️ceilings/🔲️ceilings.ifc
    When the file is opened and every IfcCovering is counted, located in its storey and measured by the kernel
    Then the counts per IFC class, the containment per storey and the net volumes of the flat ceilings, the area of the outline less its holes times the layer thickness, equal the subject's within 1e-9

  @id-export-ifc-ramps
  @level-quick
  @mode-differential
  Scenario: Every ramp is an IfcRamp aggregating an IfcRampFlight per sloped flight and a landing slab per landing, whose quantities IfcOpenShell reads back and whose parts the kernel measures
    Given the committed ramps shared://💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🛝️ramps/🛝️ramps.ifc
    When the file is opened and every IfcRamp is located in its storey, decomposed into its parts, read for its base quantities and tessellated by the kernel
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9 and every ramp equals the shapely-adjudicated ramp-runs table: flights, landings, length, width, rise, plan area and gross volume

  @id-export-ifc-stepped
  @level-quick
  @mode-differential
  Scenario: The house written by the stepped export job after a cancelled attempt has the entity counts, containment and kernel volumes IfcOpenShell measures in the committed file
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🏠️house/🏠️house.ifc
    When an export job is cancelled half-way, a second job on the same inference session runs to its end and its file is read back, while the committed file is opened and every product is counted, located and measured
    Then the counts per IFC class, the containment per storey and the net volumes equal the subject's within 1e-9
