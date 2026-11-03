import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const from = "- Every representation paints the same prepared rings";
if (!t.includes(from)) throw new Error("anchor");
t = t.replace(from, `- Every view is a true orthographic view with hidden-line removal (D40). Hidden edges are never drawn dashed, and a part that lies completely under an overhead part is not drawn at all (in the plan, the mast of a parasol under its canopy is hidden). A cut plan (Grundriss) is only a named section (\`sections.plan\`, "Plan section B-B" / "Grundriss-Schnitt B-B"), never the default plan, which is the top view (Draufsicht). Glazing never occludes and is never hatched: a glazing shape that names a hatch is a validator failure (\`glazing-hatch\`), and the hatch loop skips glazing.
- Alle Ansichten sind echte orthogonale Ansichten mit Entfernen verdeckter Linien (D40). Verdeckte Kanten werden nie gestrichelt gezeichnet, und ein Teil, das vollständig unter einem darüberliegenden Teil liegt, wird nicht gezeichnet (in der Draufsicht der Mast eines Sonnenschirms unter seinem Schirmdach). Ein Grundriss-Schnitt ist nur ein benannter Schnitt (\`sections.plan\`), nie die Standard-Draufsicht. Verglasung verdeckt nie und wird nie schraffiert.
${from}`);
writeFileSync(file, t);
