const fs = require("fs");
const path = require("path");
const Ajv = require("ajv");
const root = process.cwd();
const directory = path.join(root, "✏️s/🔌️plugins/🀄️wfc/⚙️engine/🧿️oracle/🧰️step-context/🧪️tests");
const law = JSON.parse(fs.readFileSync(path.join(directory, "contract.json"), "utf8"));
const schema = JSON.parse(fs.readFileSync(path.join(directory, "contract.schema.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
if (!validate(law)) throw Error(JSON.stringify(validate.errors));
for (const row of law.cases) {
    const steps = row.id === "pending" ? 2 : 1;
    const observed = row.id === "refusal" ? "refusal" : row.id === "cancel" ? "cancelled" : "yield";
    const returned = row.id === "refusal" ? "continuation" : "outcome";
    if (JSON.stringify([steps, observed, returned]) !== JSON.stringify([row.businessSteps, row.observed, row.expectedReturned])) throw Error(row.id);
}
console.log("[DEBUG] Ajv accepted portable driver schema; independent Node lifecycle oracle matches 4 cases");
