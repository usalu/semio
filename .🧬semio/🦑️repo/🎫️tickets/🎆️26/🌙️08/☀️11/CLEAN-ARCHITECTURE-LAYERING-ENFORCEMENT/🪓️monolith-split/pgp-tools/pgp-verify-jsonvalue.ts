import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const base = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema";
const snapshot = JSON.parse(readFileSync(`${base}/📸️snapshot/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: false, validateFormats: false });
ajv.addSchema(snapshot);
const validate = ajv.getSchema(`${snapshot.$id}#/$defs/JsonValue`)!;
const module = (await import(pathToFileURL(`${base}/🔺️diff/🟦️.ts`).href)) as { parseJsonValue: (value: unknown) => unknown };
const samples: unknown[] = [
  { kind: "null" }, { kind: "bool", value: true }, { kind: "number", lexeme: "1.5" }, { kind: "string", value: "x" },
  { kind: "array", items: [{ kind: "null" }, { kind: "array", items: [] }] },
  { kind: "object", members: [{ key: "a", value: { kind: "bool", value: false } }] },
  { kind: "bool" }, { kind: "bool", value: "true" }, { kind: "number", lexeme: 1 }, { kind: "array", items: [{ kind: "bogus" }] }, { kind: "object", members: [{ key: 1, value: { kind: "null" } }] },
  { kind: "other" }, {}, null, 42, "x", [],
];
let agree = 0;
for (const sample of samples) {
  let accepted = true;
  try { module.parseJsonValue(sample); } catch { accepted = false; }
  const expected = validate(sample);
  if (accepted === expected) agree += 1;
  else console.log("DISAGREE", JSON.stringify(sample), "ajv", expected, "parser", accepted);
}
console.log(`parseJsonValue (diff leaf) vs Ajv on the snapshot JsonValue: ${agree}/${samples.length} agree; the old parser overflowed the stack on every input`);
