import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🗺️ordered-map/🧫️fixtures/📦️paging";
const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(`${root}/🧬️schema/🔣️.json`, "utf8")));
const fixture = JSON.parse(readFileSync(`${root}/🔣️.json`, "utf8"));
const variant = (mutate: (ops: Record<string, unknown>[]) => void) => { const copy = structuredClone(fixture); mutate(copy.operations); return validate(copy); };
console.log(JSON.stringify({
  fixture: validate(fixture),
  lookupWithValue: variant(ops => { ops[0].value = "x"; }),
  insertWithoutValue: variant(ops => { delete ops[2].value; }),
  duplicateWithoutValue: variant(ops => { delete ops[3].value; }),
  unknownKind: variant(ops => { ops[0].kind = "remove"; }),
}));
