import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

const root = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";
const declaration = readFileSync(root + "/../../../../🦀️.rs", "utf8");
const registration = readFileSync(root + "/🚪️io/🦀️.rs", "utf8");
const schema = { type: "string", pattern: "^s\\.stdio\\.[a-z]+@[A-Za-z0-9.]+/\\*$" };
const validate = new Ajv().compile(schema);
const source = "s.layout.layout@1/*";
const route = (from, to) => Buffer.byteLength(from) + ":" + from + Buffer.byteLength(to) + ":" + to;
const claimed = new Map([...declaration.matchAll(/"s\.layout\.layout\.composer\.([a-z]+)", "composer", "([^"]+)", &\[\("composition-route", "([^"]+)"\)\]/g)]
  .map(([, format, dialect, claim]) => [format, { dialect, claim }]));
const authors = [...registration.matchAll(/const EXPORT_([A-Z]+)_DIALECT: Dialect = Dialect \{ artifact_kind: "([^"]+)", standard: StandardId\("([^"]+)"\), subset: SubsetId\("([^"]+)"\) \};/g)];
assert.equal(authors.length, 8);
assert.equal(claimed.size, authors.length);
for (const [, name, kind, standard, subset] of authors) {
  const dialect = kind + "@" + standard + "/" + subset;
  assert.ok(validate(dialect), dialect);
  assert.deepEqual(claimed.get(name.toLowerCase()), { dialect, claim: route(source, dialect) }, name);
}
console.log("Verified all " + authors.length + " Layout export dialect claims");
