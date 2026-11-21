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

const leaf = files.get("https://json.schemas.assets.semio-tech.com/s/stdio/xlsx/ecma-376/base/mutation/insert-cell/schema.json")!;
const Ajv = (await import("ajv")).default;
const ajv = new Ajv({ strict: false });
for (const schema of files.values()) ajv.addSchema(schema);
const validate = ajv.getSchema(leaf.$id!)!;
const wireRoot = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base", "🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-cell/🧾️wire-witness");
const read = (path: string) => JSON.parse(readFileSync(join(wireRoot, path), "utf8"));
const original = read("🦠️mutation/🔣️.json");
const before = read("📸️snapshot/⬅️before/🔣️.json");
const after = read("📸️snapshot/➡️after/🔣️.json");
assert.equal(validate(original), true, JSON.stringify(validate.errors));
const address = original.address.worksheet;
type Element = { name: string; attrs: {name: string; value: string}[]; children: Element[] };
type WitnessSnapshot = { xmlParts: { path: string; document: { root: Element } }[] };
const cells = (snapshot: WitnessSnapshot): string[] => {
    const sheet = snapshot.xmlParts.find(part => part.path === address.partPath);
    assert.ok(sheet);
    assert.equal(sheet.document.root.attrs.find(attr => attr.name === "xmlns")?.value, address.namespaceUri);
    const sheetData = (address.nodePath as number[]).reduce((node, index) => node.children[index]!, sheet.document.root);
    assert.equal(sheetData.name, address.localName);
    return sheetData.children.flatMap(row => row.children.map(cell => {
        const reference = cell.attrs.find(attr => attr.name === "r")?.value;
        assert.ok(reference);
        return reference;
    }));
};
assert.ok(!cells(before).includes("B2"));
assert.ok(cells(after).includes("B2"));
const preview = JSON.parse(readFileSync(join(wireRoot, "../../../../../✏️editor/🧪️tests/🧫️fixtures/📊️sheet-grid/🔣️.json"), "utf8")).historyPreview;
const vacancyEdit = structuredClone(original);
for (const {path, value, cell} of preview.inputs) {
    const coordinate = path.slice("/address/".length);
    assert.ok(coordinate === "row" || coordinate === "column");
    vacancyEdit.address[coordinate] = value;
    assert.equal(validate(vacancyEdit), true, JSON.stringify(validate.errors));
    assert.equal(`${String.fromCharCode(65 + vacancyEdit.address.column)}${vacancyEdit.address.row}`, cell);
    assert.ok(!cells(before).includes(cell));
    assert.deepEqual(vacancyEdit.address.worksheet, address);
}
assert.equal(preview.inputs.at(-1).cell, preview.expected.cell);
assert.equal(vacancyEdit.address.row - 1, preview.expected.row);
assert.equal(vacancyEdit.address.column, preview.expected.column);
assert.equal(vacancyEdit.value.value, preview.expected.value);
console.log("Ajv accepts both successive vacant coordinates with the original worksheet identity");
