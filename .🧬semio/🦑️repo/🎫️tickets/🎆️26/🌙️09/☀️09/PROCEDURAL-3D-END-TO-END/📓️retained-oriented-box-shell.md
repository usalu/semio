# Retained Box Shell in an authored frame

The retained \`brep.solid.shell\` operator now accepts a rotated, translated or reflected certified rectangular Box. It identifies each selected face by its outward normal in the authored frame. It runs the same bounded orthogonal cavity cut in that frame, places the exact planar supports and lines in world geometry, and reverses trim traversal for a reflection. The cavity may stay closed, open to any of the six sides, join adjacent openings, or form a through tunnel. Leaving a single material wall is also supported.

The placed material certificate supports world-space point Classify. The operation retains the existing funded selection and private topology publisher. Foreign faces, zero thickness, collapsed cavities, removal of all six faces, and frames whose precision would collapse the transported boundary are refused.

## Evidence

- The neutral \`🔣️original-oriented-box-shell.json\` covers all six independent openings, the closed cavity, adjacent and through openings and a single retained wall. Manifold independently subtracts the cavity from stock, transforms the result and checks bounds, area, volume, Euler characteristic and classifications. The combined existing and new Shell oracles passed 19 tests and 291 assertions.
- \`🐍️oriented-box-shell-source-check.py\` passed 33 compact Rust tests. It compiles the native receptacle, exact frame geometry, cell topology, cap lookup and placed Classify route. It verifies outward and inward shell signs, reciprocal incidence, coplanar trim adherence, six local opening identities, area, volume, bounds and representative boundary contacts. Adjacent cylindrical Shell and world-orthogonal Classify passed 29 and 11 compact tests, respectively.
- Registered \`original-brep-booleans\` drives the retained operator with real selected faces after Rotate, Mirror and Translate. It verifies foreign references and geometric refusals, rollback after private face insertion, topology and preview, and the registered Measure and Classify outputs. The managed target remains pending shared build capacity.
- Canonical source closure passed BRep Engine at 3,940 lines and 233,743 bytes, Native Registry at 3,972 and 217,253, Solid Transaction at 3,996 and 206,582, Boolean at 3,999 and 208,240, Vector at 1,234 and 52,173, and original-booleans at 1,904 and 232,043.

The placed proof has a distinct provenance case so the existing axis-aligned Boolean operand format and fixed quotation continue to use its original cell certificate.
