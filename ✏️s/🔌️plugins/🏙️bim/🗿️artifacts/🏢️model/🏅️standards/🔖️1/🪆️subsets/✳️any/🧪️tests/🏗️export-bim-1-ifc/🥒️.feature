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
