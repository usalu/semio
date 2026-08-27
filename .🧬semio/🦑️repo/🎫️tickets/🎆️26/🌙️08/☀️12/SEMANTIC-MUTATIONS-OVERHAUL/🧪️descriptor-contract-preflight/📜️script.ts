import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";

//#region 🧪️DescriptorTagContract
const root = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️vectors.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
const { validateJsonSchemaSubset } = await import("../../../../../../../../📜️script.ts");
let failures = 0;
console.log(`[DEBUG] retained fixture root: ${root}`);
for (const vector of fixture.cases) {
  const descriptor = { ...fixture.descriptor, binaryTag: vector.value };
  if (vector.missing) delete descriptor.binaryTag;
  const field = vector.missing ? "" : `binary_tag: ${vector.value === null ? "None" : `Some(${JSON.stringify(vector.value)})`}`;
  const source = `struct Descriptor { binary_tag: Option<u32> } const VALUE: Descriptor = Descriptor { ${field} };`;
  writeFileSync(join(root, `${vector.name}.rs`), source);
  const compiled = spawnSync("rustc", ["--crate-name", "descriptor_tag_contract", "--crate-type", "lib", "--edition", "2021", "--emit=metadata", "-o", join(root, `${vector.name}.rmeta`), "-"], { input: source, encoding: "utf8", timeout: 30000 });
  const schemaAccepted = validate(descriptor);
  const internalAccepted = validateJsonSchemaSubset(schema, descriptor).length === 0;
  const rustAccepted = compiled.status === 0;
  writeFileSync(join(root, `${vector.name}.log`), compiled.stderr ?? "");
  console.log(`[DEBUG] ${JSON.stringify({ name: vector.name, expected: vector.expected, schemaAccepted, internalAccepted, rustAccepted, compilerStatus: compiled.status })}`);
  if (schemaAccepted !== vector.expected || internalAccepted !== vector.expected || rustAccepted !== vector.expected) failures += 1;
}
console.log(`[DEBUG] descriptor tag contract mismatches=${failures} vectors=${fixture.cases.length}`);
process.exit(failures === 0 ? 0 : 1);
//#endregion 🧪️DescriptorTagContract
