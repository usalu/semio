/** 🔬 Independent JSON Schema validation of the language-neutral route claim corpus. */
import Ajv from "ajv";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";

const root = resolve(import.meta.dir,"../../../../../../..");
const base = resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️identity/🧪️tests");
const schema = JSON.parse(await readFile(resolve(base,"composition-route.schema.json"),"utf8"));
const fixture = JSON.parse(await readFile(resolve(base,"composition-route.json"),"utf8"));
const validator = new Ajv({allErrors:true}).compile(schema);
if (!validator(fixture)) throw new Error(JSON.stringify(validator.errors));
const bytes = new TextEncoder();
for (const route of fixture.valid) {
  const match = /^(\d+):(.+)$/.exec(route)!;
  const sourceBytes = Number(match[1]);
  const source = match[2]!.slice(0,sourceBytes);
  const target = /^(\d+):(.+)$/.exec(match[2]!.slice(sourceBytes))!;
  if (bytes.encode(source).length !== sourceBytes || bytes.encode(target[2]!).length !== Number(target[1])) throw new Error("route framing mismatch");
}
console.log(JSON.stringify({oracle:"Ajv",fixtureValid:true,validRoutes:fixture.valid.length,invalidRoutes:fixture.invalid.length}));
