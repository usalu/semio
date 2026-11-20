import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { mutationInputAudit } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts";

const root = process.cwd();
const stdio = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
type Schema = { $id?: string; [field: string]: unknown };
const files = new Map<string, Schema>();
function collect(directory: string): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory() && !entry.name.startsWith(".") && entry.name !== "node_modules") collect(path);
    else if (entry.isFile() && path.includes("🧬️schema") && entry.name.endsWith(".json")) {
      const schema: Schema = JSON.parse(readFileSync(path, "utf8"));
      if (schema?.$id) files.set(schema.$id, schema);
    }
  }
}
for (const artifact of ["📕️xlsx", "🎒️zip", "📰️xml"]) collect(join(stdio, artifact));
const allFindings = [];
for (const [id, schema] of files) {
  if (!/\/xlsx\/.*\/mutation\/[^/]+\/schema\.json$/.test(id)) continue;
  const report = mutationInputAudit(schema, ref => files.get(ref));
  allFindings.push(...report.findings.map(finding => `${id} ${finding.message}`));
}
assert.deepEqual(allFindings, []);
const insertion = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕insert-sheet/🧬️schema/🔣️.json"), "utf8"));
const slot = mutationInputAudit(insertion, ref => files.get(ref)).inputs.find(input => input.id === "/slot");
assert.equal(slot?.presentation?.kind, "hidden");
assert.equal(slot?.schema.kind, "any");
console.log("XLSX history input schemas resolve and the verbatim restoration slot stays hidden");
