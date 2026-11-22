import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import Ajv2020 from "ajv/dist/2020.js";

const read = (file) => JSON.parse(readFileSync(new URL(file, import.meta.url), "utf8"));
const fixture = read("natural-pdf-sdk-cases.json");
const schema = read("natural-pdf-sdk-cases.schema.json");
assert.equal(new Ajv2020({ strict: true }).validate(schema, fixture), true);

const db = new DatabaseSync(":memory:");
const oracle = db.prepare("select :extent between 1 and :maximum and :port = :natural and :binary = 1 and :wire = 0 as accepted");
const generated = new URL("./🗑️generated/natural-pdf-admission/", import.meta.url);
const tests = fixture.cases.map(({ name, port, binary, extent, maximum, rawWireBytes, accepted }, index) => {
    const actual = oracle.get({ extent, maximum, port, natural: fixture.naturalPort, binary: Number(binary), wire: rawWireBytes });
    assert.equal(Boolean(actual.accepted), accepted, name);
    return `#[test] fn case_${index}() {
    assert_eq!(admits_paged_input(${JSON.stringify(port)}, ${binary}, ${extent}, ${maximum}, ${rawWireBytes}), ${accepted}, ${JSON.stringify(name)});
}`;
});
db.close();
writeFileSync(new URL("sdk-law.rs", generated), `extern crate self as semio_plugin_decl_app;
pub const NATURAL_FILE_PORT: &str = ${JSON.stringify(fixture.naturalPort)};
include!("sdk-admission.rs");
${tests.join("\n")}
`);
console.log(`[DEBUG] Paged SDK admission fixture and SQLite agreed on ${tests.length} cases.`);
