/** 📐️ Uses independent JSON Schema admission to validate portable publication owner outputs. */
const fs = require("node:fs");
const path = require("node:path");
const Ajv = require("ajv");
const directory = path.join(process.cwd(), "✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/📤️publication/📦️owner/🧪️tests");
const law = JSON.parse(fs.readFileSync(path.join(directory, "frontiers.json"), "utf8"));
const schema = JSON.parse(fs.readFileSync(path.join(directory, "frontiers.schema.json"), "utf8"));
const ajv = new Ajv({ allErrors: true, strict: true });
const validate = ajv.compile(schema);
if (!validate(law)) throw Error(JSON.stringify(validate.errors));
let cases = 0;
for (const header of [64, 4096, 262144]) {
    const admits = ajv.compile({ type: "object", additionalProperties: false, required: ["items", "release"], properties: { items: { type: "integer", minimum: 1 }, release: { type: "integer", minimum: header } } });
    for (const row of law.cases) {
        const admitted = admits({ items: row.items, release: row.release === "short" ? header - 1 : header });
        const output = { removed: admitted, work: Number(admitted), released: admitted ? header : 0 };
        const expected = { removed: row.removed, work: row.work, released: row.removed ? header : 0 };
        if (JSON.stringify(output) !== JSON.stringify(expected)) throw Error(row.id + "@" + header);
        cases++;
    }
}
console.log("[DEBUG] Ajv grant admission independently matches " + cases + " publication owner outputs");
