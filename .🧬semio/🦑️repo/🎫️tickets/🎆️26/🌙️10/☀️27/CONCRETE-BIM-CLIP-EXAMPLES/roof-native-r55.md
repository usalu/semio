# Native Roof Assembly Verification R55

All ten selected English/German native IFC fragments passed independent web-ifc0.0.78 parsing,identity,type/assigned-type,representation,occupied volume,part count and placed-bounds checks. Captured452 source SHA256dc4a83066bd3360806189e4827c281783eff7267fe29c2282bb5b85a30f7480e. Product source stayed unchanged through capture; the final analysis-link refresh has not yet been published. This is selected-product validation,not full-building export or native-frame certification.

| Product | Parts | Independent GEOS Volume m³ | Native Mesh Volume m³ | Absolute Difference m³ |
| --- | ---: | ---: | ---: | ---: |
| CHIMNEY-SALON-ROOF | 12 | 0.274000000000 | 0.274000002179 | 2.179e-9 |
| CHIMNEY-KITCHEN-ROOF | 12 | 0.309000000000 | 0.308999992798 | 7.202e-9 |
| SL-R | 462 | 46.712866864161 | 46.712867513387 | 6.492e-7 |
| W-STAIR-ROOF | 280 | 7.083897499159 | 7.083897573221 | 7.406e-8 |
| D-STAIR-ROOF | 66 | 0.035485514507 | 0.035485513879 | 6.288e-10 |

English/German results are identical geometrically. Mesh bounds agree within1e−5m. Roof slab native Body is exclusive owned structural material,with462parts and the two.15m square bores; full/owned/shared finish semantics remain distinct. The corrected path wall has280occupied parts and the door66parts,including hollow timber leaf material instead of a solid35mm shortcut. The two chimney bodies each retain12layered prisms and real.1000m²support rings.

The initial independent adapter incorrectly required an occurrence-level door PredefinedType. Inspection showed the writer correctly assigns the type through IfcRelDefinesByType and leaves the occurrence field empty. [buildingSMART IfcDoor documentation](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcDoor.htm) requires that distinction for typed doors. The adapter now verifies the actual native type relationship and inherited enumeration; no product writer change or duplicated occurrence classification was introduced.

Retained inputs roof-native-r54.py and native-products-r44.mjs reproduce the checks. Generated native fragments/results remain disposable under 🗑️generated until ticket completion. Architectural registration and uncertainty are documented separately in R44/R49/R50/R54 reports.
