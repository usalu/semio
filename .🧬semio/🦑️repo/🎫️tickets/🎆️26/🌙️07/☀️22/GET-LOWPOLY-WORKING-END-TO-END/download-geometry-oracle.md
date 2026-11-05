# Download Geometry Oracle

The retained download-geometry-oracle.mjs accepts exactly --file PATH --format obj|ply|stl --expect PATH. Both input files must be existing regular files inside the current repository, after realpath resolution. Original and resolved paths are bounded to256 UTF-16 units; geometry input is capped at64MiB and expectations at1MiB. The oracle never copies source or writes files.

Use the public route:
`repo tools exec --light node -- '<ticket>/download-geometry-oracle.mjs' --file '<ticket>/🗑️generated/<download>' --format obj --expect '<expectation.json>'`

The language-neutral expectation schema is export-oracle-inputs/geometry.schema.json. Required corners and total area are independent of duplicated raw vertex/index counts. Optional triangleCount, common windingNormal, orientedTriangles (cyclic ordering accepted; reversed winding refused), and position/normal/UV tuples provide stricter caller-controlled checks. Normal comparison uses world-space normalized directions. Oriented triangles are optional because an exporter may legitimately triangulate a polygon with a different diagonal; corners/area plus a common normal suit planar faces. For a general closed mesh, supply its oriented triangles when exact triangulation/winding is required.

OBJLoader, PLYLoader and STLLoader are the existing Three.js test-only parsing boundary. Parsed outputs leave that boundary as plain numeric data. Every parsed triangle must have finite valid positions, valid indices and nonzero area. When attribute expectations are supplied, all observed and expected vertex tuples must match; seam duplicates are handled by tuples rather than vertex indexing. STL expectations omit UVs because STL does not encode them.

A successful CLI report includes explicit input/expectation paths, format, SHA256, byte count, semantic corner/triangle counts, area and which attributes/winding were checked. This proves only that the explicitly named file matched those expectations; a browser download must still be obtained and linked to the UI journey.

## Executed Parser Self-Test

download-geometry-selftest.mjs passed exit0:9 positives and12 negatives. Positive inputs are3 retained handcrafted square files and6 independently generated Three exporter references (OBJ; ASCII/little-endian/big-endian PLY; ASCII/binary STL). All have4 semantic corners,2 triangles and area1; OBJ/PLY additionally check normals and UVs. The unindexed STL triangles therefore validate semantic geometry without requiring the same raw vertex/index count.

Negative cases reject reversed common winding, reversed oriented triangle, wrong area/corner/UV/normal, STL with required UVs, missing/duplicate CLI arguments, input outside the repository, absent mesh faces and an unsupported format. Each actual CLI entry point was also executed separately on the3 handcrafted files and passed exit0.

Generated exporter references and negative expectation outputs are only under ticket/🗑️generated/export-oracle. Handcrafted authored fixtures, schema, scripts and this report are retained. The older export-oracle.mjs remains a separate fixture-only Three plane expectation check.

No Rust production payload or browser download has been verified by this work: productionPayloadsVerified remains0. No Native/server/browser run was started.
