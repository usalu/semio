Feature: DIN V 18599 structural scene
  The building snapshot and its check report project to a locale-free 3D scene of thermal zones and envelope elements, 2D diagrams and drag handles.
  The monthly heat balance of DIN V 18599-2 is reproduced by an independent numpy implementation (`../../🔮️oracles/🐍️.py`) that reads only the snapshot JSON.

  Further specification assertions, asserted by the Rust unit tests of the crate and by the numpy oracle:
  - Every zone and every envelope element stands in the scene: Given any of the four committed example buildings When the structural scene is projected Then the scene passes the structural scene validation And it holds exactly one entity per zone and per envelope element at the path "zones[id=ID]" or "elements[id=ID]" And two projections of the same revision have byte-identical canonical JSON And the live scene equals the full projection
  - Zone boxes follow the documented size: Given the two-zone house When the layout is resolved Then every zone box is a square of side sqrt(area) and boxes never overlap
  - The monthly balance equals the numpy oracle: Given the compliant house, the non-compliant house, the two-zone house and the cooled office When the twelve monthly balances of every zone are computed Then transmission, ventilation, internal and solar gains, the utilisation factor and the net heating and cooling demand equal the numpy values within a relative 1e-9
  - The monthly balance sums to the annual zone demand: Given any example building When the months of a zone are summed Then the net heating demand equals the annual Q_h of the zone balance and the net cooling demand equals its annual Q_c
  - The energy flow matches the building balance: Given the cooled office When the useful, final and primary energy per service are summed Then the primary energy minus the photovoltaic credit equals Q_p of the derived balance
  - A focus restricts the diagrams: Given the two-zone house When the diagrams of the first envelope element are requested Then the U-value diagram of the element, the monthly balance of its zone and the building energy flow are returned and no other zone appears
  - Handle "<element>:u-value" edits "elements[id=<element>].uValueWM2k": Given any example building When the grip at the current U-value is read back through its affine map Then the resolved value equals the stored U-value
