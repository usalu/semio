@capability-din4108-1-evaluate
@oracle-din4108-1-compliance-independent
@comparison-numerical-v1
Feature: DIN 4108 structural scene
  The building-envelope snapshot and its check report project to a deterministic, locale-free 3D scene, a set of 2D diagrams and drag handles.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - envelope elements are layer sections: Given the default dwelling When the scene is projected Then the entity "wall-north" is a wall assembly whose layers are the stored layers interior first with their real thicknesses And the roof and the floor are drawn beside it
  - zones and thermal bridges are drawn: Given the default dwelling When the scene is projected Then every thermal zone is a floor plate of its floor area and every thermal bridge a line of its length
  - utilisation colours the element: Given an envelope element whose insulation layer is too thin When the scene is projected Then the element is drawn in the error style
  - the temperature profile follows the nodal solve: Given the default dwelling When the diagrams are built Then the temperature at every layer interface equals a numpy nodal solve of the series resistance network And an interface marker bound to the U-value check writes the layer thickness
  - the Glaser profile follows the standard: Given the default dwelling When the diagrams are built Then the vapour pressure interpolates linearly over the equivalent air layer thickness And the saturation pressure agrees with the formula of DIN EN ISO 13788 within one percent And condensation markers are bound to the Glaser check
  - handles write document leaves: Given the default dwelling When the masonry layer of "wall-north" is selected Then its grip writes "elements[id=wall-north].layers[id=masonry].thicknessM" and the thermal bridge grips write the length and the psi value
  - every check is drawn on its owner row or on the building badge: Given any example document and its check report When the check outcomes are routed Then each check resolves to the row its subject names or to the building badge through a document-level family
  - the drying reserve is checked: Given a vapour-tight exterior layer on an open interior stack When the Glaser check condenses in winter Then the drying check "din4108-3.drying.<element>" compares the condensed mass with the mass that evaporates in summer And the vectors in "🧪️tests/🧫️fixtures/🧮️glaser-drying" agree with the numpy oracle within two percent
  - the U-value bars follow EN ISO 6946: Given the vectors in "🧪️tests/🧫️fixtures/🧮️u-values" (homogeneous wall, roof, floor on ground and timber studs in insulation) When the total resistance and the U-value are computed Then they equal the numpy series and combined-method values of the oracle to 1e-9
