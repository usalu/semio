@capability-bim-1-infer
@oracle-bim-1-ifcopenshell-kernel
@comparison-floating-point-v1
Feature: Infer which elements each storey shows under each view phase and audit it with IfcOpenShell on the exported file
  `s.bim.model@1` authors the construction phase of every phased element (existing, new, demolished, temporary) and the storey it stands on. `🎭️phase-visibility` derives,
  for every storey and every view phase, the sorted ids of the elements a view with that phase filter shows: the view phase `all` shows every element of the storey, any
  other view phase shows exactly the elements of that phase. An opening takes the phase of the wall or curtain wall that hosts it; an element kind that carries no phase of its
  own (ceilings, ramps) counts as new construction; an element stands on the storey it is authored on, so moving it moves it to the visibility of the other storey. The
  oracle is `🐍️.py` in this directory. It never sees the subject's inference: it opens the committed IFC 2x3 export of the house with IfcOpenShell, reads the construction phase
  of every product from its `Semio_Authoring` property set (no row means new work), takes the phase of an opening element from the wall it voids and the storey of every
  product from its spatial containment or aggregation, and groups the element ids by storey and view phase. It also audits the sets: the phases partition the elements of a
  storey and `all` is their union.

  @id-phases-house
  @level-quick
  @mode-differential
  Scenario: The elements each storey of the house shows under each view phase equal what IfcOpenShell reads from the file
    Given the committed house shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json and its export shared://🏗️ifc/🏠️house/🏠️house.ifc
    When 🎭️phase-visibility is inferred for it and the phases and storeys of the exported products are read back
    Then the sorted element ids of every storey under every view phase equal the oracle's within 1e-9
