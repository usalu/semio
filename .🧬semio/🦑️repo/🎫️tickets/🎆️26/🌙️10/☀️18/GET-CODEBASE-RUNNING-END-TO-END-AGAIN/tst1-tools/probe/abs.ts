import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { policyExtractGraphqlSchemaFields } = await import(L + "/🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts");
const { policyExtractProtobufSchemaFields } = await import(L + "/🧬️schema/🔍️field-discovery/🛰️protobuf/🟦️.ts");
const { policyExtractRustSchemaFields } = await import(L + "/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts");
const { policyExtractTypescriptSchemaFile } = await import(L + "/🧬️schema/🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts");
const root = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(L + "/📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json", "utf8"));
const reps: [string, any][] = [["🦀️.rs", policyExtractRustSchemaFields], ["🟦️.ts", null], ["🔗️.graphql", policyExtractGraphqlSchemaFields], ["🛰️.proto", policyExtractProtobufSchemaFields]];
let bad = 0;
for (const path of fixture.artifactSchemas) {
  const artifact = JSON.parse(readFileSync(join(root, path), "utf8"));
  for (const [filename, extract] of reps) {
    const pathName = `${dirname(path)}/${filename}`;
    let declaration: any;
    try {
      const text = readFileSync(join(root, pathName), "utf8");
      declaration = filename === "🟦️.ts" ? policyExtractTypescriptSchemaFile(join(root, pathName), text, artifact.title) : extract(text, artifact.title);
    } catch (error) { console.log("[DEBUG] THROW", pathName.slice(-110), String(error).slice(0, 120)); bad++; continue; }
    if (!declaration.typeName) { console.log("[DEBUG] MISSING", artifact.title, pathName.slice(-110)); bad++; }
    else if (declaration.fields.some((f: any) => f.state && f.state !== "artifact")) { console.log("[DEBUG] STATE", artifact.title, pathName.slice(-110)); bad++; }
  }
}
console.log("[DEBUG] total", fixture.artifactSchemas.length, "bad", bad);
