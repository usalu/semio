import { readFileSync, writeFileSync } from "node:fs";
import { modify, applyEdits } from "jsonc-parser";

const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const path = `${base}/🧬️schema/🔣️.json`;
let text = readFileSync(path, "utf8");
const before = text;
const document = JSON.parse(text);
const envelope = JSON.parse(readFileSync(`${base}/🧬️schema/🛡️untrusted-content-schemas/🧬️schema/capability-output.json`, "utf8")).$defs.Envelope;
const sorted = (value: any): any => Array.isArray(value) ? value.map(sorted) : value && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map(key => [key, sorted(value[key])])) : value;
const changes: [Array<string | number>, unknown][] = [];
const visit = (value: any, keys: Array<string | number>) => {
  if (!value || typeof value !== "object") return;
  if (value.const?.startsWith?.("Content authored in a shared document")) changes.push([keys, { enum: [value.const, envelope.properties.notice.const] }]);
  if (Array.isArray(value.enum) && value.enum.join("|") === "artifact-body|artifact-export|hub-checkpoint|space-directory") changes.push([[...keys, "enum"], [...value.enum, "capability-output"]]);
  if (Array.isArray(value.oneOf) && value.oneOf.length === 2 && value.oneOf[0]?.properties?.kind?.const === "local-principal" && value.oneOf[1]?.properties?.kind?.const === "space-writers") changes.push([[...keys, "oneOf"], [...value.oneOf, { type: "object", properties: { kind: { const: "unattributed" } }, required: ["kind"], additionalProperties: false }]]);
  for (const [key, member] of Object.entries(value)) visit(member, [...keys, Array.isArray(value) ? Number(key) : key]);
};
visit(document, []);
const nullable = { anyOf: [envelope, { type: "null" }] };
changes.push([["$defs", "InvocationReport", "properties", "untrusted"], nullable]);
changes.push([["$defs", "InvocationReport", "required"], [...new Set([...document.$defs.InvocationReport.required, "untrusted"])].sort()]);
changes.push([["$defs", "PreparedActionReport", "properties", "preview"], {
  type: "object", properties: {
    opsCount: { type: "object", properties: Object.fromEntries(["document", "config", "draft", "childOpBytes"].map(key => [key, { type: "integer", minimum: 0 }])), required: ["document", "config", "draft", "childOpBytes"], additionalProperties: false },
    warnings: { type: "array", items: { type: "string" } }, untrusted: nullable,
  }, required: ["opsCount", "warnings", "untrusted"], additionalProperties: false,
}]);
for (const [keys, value] of changes) text = applyEdits(text, modify(text, keys, sorted(value), { formattingOptions: { insertSpaces: true, tabSize: 2, eol: "\n" }, getInsertionIndex: siblings => siblings.filter(key => key < String(keys.at(-1))).length }));
const oldFloats = before.match(/\b\d+\.0\b/g) ?? [];
const newFloats = text.match(/\b\d+\.0\b/g) ?? [];
if (JSON.stringify(oldFloats) !== JSON.stringify(newFloats)) throw new Error("unrelated numeric formatting changed");
writeFileSync(path, text);
const script = readFileSync(`${base}/📦️packages/🦀️rust/📜️script.ts`, "utf8");
const source = script.slice(script.indexOf("type SchemaNode ="), script.indexOf("const router ="));
if (!source.includes("function renderSchemaTypescript")) throw new Error("owned renderer not found");
const render = new Function(new Bun.Transpiler({ loader: "ts" }).transformSync(source) + ";return renderSchemaTypescript;")();
const rendered = render(JSON.parse(text));
writeFileSync(`${base}/🧬️schema/🟦️.ts`, rendered);
console.log(JSON.stringify({ edits: changes.length, preservedFloatLiterals: oldFloats.length, exports: Object.keys(document.$defs).length, renderer: "owned native script source", byteExactEmitterCheck: "pending native binary" }));
