# Same Table Audit

Version 12 contracts require the worked example to use the native table chrome and shared matrix implementation: three unnamed half-scale solids, a 16 mm header, four filled rows and four completely blank rows. Generic bilingual table contracts already require three native AssignmentTable blocks. Validation pending production geometry and compiled candidates.

Validation: parametrization semantic/source verifier passed with schema v12 (AJV, mathjs and D3 differential oracles). Generic assignment Bun suite: 18 passed, 0 failed, 75 assertions. An initial generic regression assertion assumed inline blank-row replication; it was updated to verify the shared expandable component and its common row generator. PDF verification awaits newly compiled candidates.

## Fresh German PDF Verification

All native PDF checks passed against compiled/parametrization-de.pdf. The worked matrix is four columns by nine rows: four filled L/B/H/Form rows and four entirely empty rows. Its native projected box, pyramid and elliptical cylinder use the same half-scale, remain unnamed in their header cells, and keep at least 2 mm vertical padding. Its header pitch is 16.203 mm: nominal 16 mm plus half the 0.75 pt chrome top stroke and 0.4 pt lower grid stroke. Both task headers match this pitch within 0.02 mm.

The unchanged task matrices retain their nine-by-nine grids, 6.14 mm centerline writing-row pitch, 2.13–2.22 mm artwork padding, and maximum center offsets below 0.032 mm.

Directed PDF verification passed all 240 cubic K16 arcs, 20 selected directions, 266 native arrowheads, endpoint/icon clearance, ordered ledger, and six directed K3 arcs. The graph visible height remains 125.603 mm. Schema/source checks retain the outer 16→1 closing direction and inner 1→5→10→13→1 cycle.

Native titles and framework numbering passed: Beispiel / Tabelle: 1.1.a; Tabelle A / Tabelle: 1.2.a; Tabelle B / Tabelle: 1.2.b. The PDF has three pages with de-DE metadata. Last body text bottoms are 254.47, 134.53 and 254.33 mm. Root visual review independently confirmed all three pages fit.

The initial header tolerance failure was a verifier assumption about equal stroke widths, resolved by deriving pitch from actual top and bottom native border strokes and comparing all three headers. No production geometry adjustment was required.
