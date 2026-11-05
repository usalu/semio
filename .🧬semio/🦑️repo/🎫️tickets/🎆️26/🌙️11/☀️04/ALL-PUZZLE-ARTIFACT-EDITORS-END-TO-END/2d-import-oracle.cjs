const { readFileSync } = require("node:fs");
const { resolve } = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv");
const corpus = JSON.parse(readFileSync(resolve("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json"), "utf8"));
const validate = new Ajv().compile(corpus.admissionSchema);
for (const sample of corpus.cases) {
    const value = typeof sample.args.payload === "string" ? JSON.parse(sample.args.payload) : sample.args.payload;
    assert.equal(validate(value), sample.accepted, JSON.stringify(sample));
}
process.stdout.write(`${corpus.cases.length} portable Puzzle 2d import vectors match Ajv.\n`);
