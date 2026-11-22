/** 🧮 Offline proof: prints the committed oracle count per example, no server, no browser. Run with the repo-provided bun. */
import { ORACLES } from "./🐍️example-oracle.mjs";
import { exampleFixtureFor, exampleEntries } from "./🐍️example-fixtures.mjs";
const rows = Object.values(ORACLES).map((o) => ({ slug: o.slug, meshes: o.meshes, minTriangles: o.minTriangles, minEdgeSegments: o.minEdgeSegments, previewMeshId: o.previewMeshId, matrixFixture: exampleFixtureFor(o.slug) !== null }));
console.table(rows);
console.log(`examples with fixture: ${exampleEntries().length}; oracles: ${rows.length}`);
if (rows.length === 0 || rows.some((r) => !r.matrixFixture)) process.exit(1);
