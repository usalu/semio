const fs = require("node:fs");
const Ajv = require("ajv");
const base = "🧰️framework/🔨️modules/🧵️job/🏃️headless/📥️capture/🧪️tests/";
const validate = new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(fs.readFileSync(base + "frontiers.schema.json", "utf8")));
const law = JSON.parse(fs.readFileSync(base + "frontiers.json", "utf8"));
if (!validate(law)) throw Error(JSON.stringify(validate.errors));
console.log("[DEBUG] Ajv accepts the exact original-capture frontier contract");
