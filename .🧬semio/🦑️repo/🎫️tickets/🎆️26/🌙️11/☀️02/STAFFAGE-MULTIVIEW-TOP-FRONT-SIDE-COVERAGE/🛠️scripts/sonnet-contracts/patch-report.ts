import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const from = "See the final section appended when the second full run finished.";
if (!t.includes(from)) throw new Error("anchor");
t = t.replace(from, `Full library suite, \`bunx vitest run -c 🧪️tests/🎚️config/🟦️.ts\` in \`📦️packages/🟦️typescript\` (output \`🗑️generated/contracts/vitest-full2.txt\`): 15 of 76 files and 30 of 1718 tests failed (61 files and 1681 tests passed, 7 skipped). That run was taken while rigs, structural, exports and the category workers were still editing, so it is a snapshot, not a verdict.

Failures in my area, and what I did:
- \`asset-validation\` "accepts the sample figure", \`smooth-continuity\` sparse walk: the three fixture figures broke the new rules (\`depthM\` 0.46 against a drawn 0.54 to 0.56; fixed by setting \`depthM\` to 0.55 in the three fixtures). One finding remains: \`unavailable-but-drawable\` at 1:500 (the 1.75 m figure is 3.5 mm on paper, at least the D17 limit of 3 mm). The fixture needs a 1:500 band or a true-extent symbol; I did not author one. This keeps those two tests red until validation decides.
- \`asset-validation\` manifest case: the fixture expectation was renamed to \`manifest-dimension-mismatch\` (fixed).
- \`print-suitability\` "authors no front or plan geometry": measuring a representation without front or plan now throws by design (D15); the test now expects the throw (fixed).
- \`section-views\`: the caption expectations already follow the new vocabulary; the two remaining failures are catalog data (\`three-seat-sofa\` sections) and the cast manifest of the test (\`viewpoint\` required), not contracts.
- Other red files (\`three-views\`, \`plan-projection\`, \`front-projection\`, \`continuous-rotation\`, \`pdf-reopen\`, \`zip-reopen\`, \`frame-timing\`, \`export-performance\`, \`cast-export\`, \`clip-sampling\`, \`multiview-coverage\`) are catalog geometry, rig, export or other workers' areas; the first of them shows the migration still running.

Typecheck at the end: my modules compile; \`🧮️evaluation\` currently fails on the new \`member\` shape kind and revolved \`axis\` until rigs finishes \`solid3\`/\`flattenGeometry\`, and other files carry their workers' in-flight errors.`);
writeFileSync(file, t);
