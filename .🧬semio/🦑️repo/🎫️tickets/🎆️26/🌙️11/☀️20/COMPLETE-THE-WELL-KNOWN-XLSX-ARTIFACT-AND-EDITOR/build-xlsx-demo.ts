import { strict as assert } from "node:assert";
import { createRequire } from "node:module";
import { join } from "node:path";

const root = process.cwd();
const require = createRequire(join(root, "🌎️hub/package.json"));
const { zipSync, strToU8 } = require("fflate") as typeof import("fflate");
const family = "application/vnd.openxmlformats-officedocument.spreadsheetml";
const spreadsheet = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const documentRelationships = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const packageNamespace = "http://schemas.openxmlformats.org/package/2006";
const sheetPaths = ["worksheets/sheet1.xml", "worksheets/sheet2.xml"];
const sharedStrings = ["Name", "Score", "Alice"];
const parts = {
  "[Content_Types].xml": `<Types xmlns="${packageNamespace}/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="${family}.sheet.main+xml"/><Override PartName="/xl/sharedStrings.xml" ContentType="${family}.sharedStrings+xml"/>${sheetPaths.map(path => `<Override PartName="/xl/${path}" ContentType="${family}.worksheet+xml"/>`).join("")}</Types>`,
  "_rels/.rels": `<Relationships xmlns="${packageNamespace}/relationships"><Relationship Id="rId1" Type="${documentRelationships}/officeDocument" Target="xl/workbook.xml"/></Relationships>`,
  "xl/workbook.xml": `<workbook xmlns="${spreadsheet}" xmlns:r="${documentRelationships}"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/><sheet name="Totals" sheetId="2" r:id="rId2"/></sheets></workbook>`,
  "xl/_rels/workbook.xml.rels": `<Relationships xmlns="${packageNamespace}/relationships">${sheetPaths.map((path, index) => `<Relationship Id="rId${index + 1}" Type="${documentRelationships}/worksheet" Target="${path}"/>`).join("")}<Relationship Id="rId3" Type="${documentRelationships}/sharedStrings" Target="sharedStrings.xml"/></Relationships>`,
  "xl/sharedStrings.xml": `<sst xmlns="${spreadsheet}" count="3" uniqueCount="3">${sharedStrings.map(text => `<si><t xml:space="preserve">${text}</t></si>`).join("")}</sst>`,
  "xl/worksheets/sheet1.xml": `<worksheet xmlns="${spreadsheet}"><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c></row><row r="2"><c r="A2" t="s"><v>2</v></c><c r="B2"><v>95.5</v></c></row><row r="3"><c r="A3" t="b"><v>1</v></c><c r="B3"><f>SUM(B2:B2)</f><v>95.5</v></c></row></sheetData></worksheet>`,
  "xl/worksheets/sheet2.xml": `<worksheet xmlns="${spreadsheet}"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t xml:space="preserve">Total Score</t></is></c></row></sheetData></worksheet>`,
};
const content = Object.fromEntries(Object.entries(parts).map(([path, xml]) => [path, strToU8(`<?xml version="1.0" encoding="UTF-8"?>${xml}`)]));
const bytes = zipSync(content, { level: 6, mtime: new Date(1980, 0, 1) });
const path = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/📕️example.xlsx");
const existing = Buffer.from(await Bun.file(path).arrayBuffer());
if (process.argv[2] === "--write" && existing.byteLength === 0) await Bun.write(path, bytes);
else assert.deepEqual(existing, Buffer.from(bytes));
console.log(`XLSX demo package is reproducible (${bytes.byteLength} bytes)`);
