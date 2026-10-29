# Native Boiler Chimney Verification R44

Two English/German selected native IFC fragments for BOILER-FLUE-VISIBLE passed independent web-ifc0.0.78 parsing and mesh-volume verification. Captured geometry source hash33d8277c was after R43 bearing correction and before the R41 metadata-only24-field update. This is selected-product validation; it does not certify a full exported building or rendered frames.

The source has one continuous native IfcChimney with96 occupied annular sectors, axis(6.64,11.50)m and vertical extent0..9.49m. Independent SciPy convex-hull sum is0.607765641909m³. Both parsed meshes have96parts,1152triangles,volume0.607765625324m³,absolute difference about1.66e−8m³. Native bounds are X6.44..6.84,Y11.30..11.70,Z0..9.49 within floating-point tessellation precision. Identity,name,predefined type and SweptSolid representation are checked independently.

Retained reproduction inputs: chimney-native-r44.py and native-products-r44.mjs. Generated JSON is disposable and remains under 🗑️generated until ticket completion. Historical axis,height,diameter and wall-thickness uncertainty are documented in chimney-register-r40.md; this check establishes implementation agreement, not survey accuracy.
