/** 🧾️ An independent JSONC editing library computes the authored replay projections. */
import Ajv from "ajv";
import { applyEdits, modify, parse } from "jsonc-parser";
import { deepStrictEqual, equal } from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync, appendFileSync } from "node:fs";
import { join } from "node:path";
const root = "✏️s/🔌️plugins/🀄️wfc/🧪️tests/🗄️store-replay";
const corpus = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
if (!validate(corpus)) throw Error(JSON.stringify(validate.errors));
const recipes: Record<string, object> = {
    bitmap: { seed: 99, pin: { x: 0, y: 0, color: 1 }, geometry: { width: 5, height: 4, periodic: true } },
    wfc2d: { seed: 99, pin: { slotId: "room-a", tileId: "room" }, geometry: { slotId: "room-a", x: 7, y: 9, width: 2, height: 2 } },
    grid2d: { seed: 99, pin: { x: 1, y: 0, tileId: "pipe-h" }, geometry: { width: 6, height: 6 } },
    grid3d: { seed: 99, pin: { x: 1, y: 0, z: 0, tileId: "wall" }, geometry: { width: 4, height: 3, depth: 4 } },
};
deepStrictEqual([...new Set(corpus.cases.map((row: { artifact: string }) => row.artifact))].sort(), Object.keys(recipes).sort());
equal(corpus.cases.length, 4);
const rows = corpus.cases.map((row: { artifact: string; expected: object }) => {
    let text = '{"seed":0,"pin":null,"geometry":null}';
    for (const [key, value] of Object.entries(recipes[row.artifact]!)) text = applyEdits(text, modify(text, [key], value, {}));
    const projection = parse(text);
    deepStrictEqual(projection, row.expected);
    return { artifact: row.artifact, projection, referenceMatches: true };
});
const generated = join(import.meta.dir, "🗑️generated");
mkdirSync(generated, { recursive: true });
writeFileSync(join(generated, "store-replay-oracle.json"), JSON.stringify({ reference: "jsonc-parser.modify/applyEdits/parse", rows }, null, 2) + "\n");
appendFileSync(join(import.meta.dir, "run-r2-browser-reload.md"), "\n## Independent Authored Projection Reference\n\nAt " + new Date().toISOString() + ", Ajv validated the shared store-replay schema and exact four-artifact roster. Existing third-party jsonc-parser independently applied each authored seed/pin/geometry projection recipe; all4 outputs equal the same portable expected projections consumed by actual native store laws. Grid dimensions remain authored baseline; bitmap size/periodicity and graph location are actual edits. This oracle checks expected event/field semantics, not the SPR binary codec. Native laws use canonical store pack+SPR replay and remain unexecuted until their actual suites run.\n");
console.log("[DEBUG] portable store replay schema and independent JSONC projections: 4/4");
