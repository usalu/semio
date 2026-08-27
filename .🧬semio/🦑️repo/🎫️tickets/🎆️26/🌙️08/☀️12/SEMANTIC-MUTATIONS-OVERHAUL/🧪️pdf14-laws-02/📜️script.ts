import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";

const root = resolve(import.meta.dir, "../../../../../../../../");
const fixtures = [
  "✳️any/🧬️schema/🧬️mutations/📥️insert-page",
  "✳️any/🧬️schema/🧬️mutations/🔀️move-page",
  "✳️any/🧬️schema/🧬️mutations/🗑️remove-page",
  "✳️any/🧬️schema/🧬️mutations/📝️replace-page-text",
  "✳️any/🧬️schema/🧬️mutations/📐️resize-page",
  "✳️a/🧬️schema/🧬️mutations/🧹️clear-page-text",
  "✳️a/🧬️schema/🧬️mutations/📝️set-page-text",
  "✳️x/🧬️schema/🧬️mutations/📉️collapse-page-size",
  "✳️x/🧬️schema/🧬️mutations/📐️set-page-size",
] as const;
const lawSchema = { type: "object", required: ["base", "mutation", "expected", "inverse"], properties: { base: { type: "object" }, mutation: { type: "object", required: ["mutation", "payload"], properties: { mutation: { type: "string" }, payload: { type: "object" } }, additionalProperties: false }, expected: { type: "object" }, inverse: { type: "array", items: { type: "object", required: ["mutation", "payload"], properties: { mutation: { type: "string" }, payload: { type: "object" } }, additionalProperties: false } } }, additionalProperties: false };
const validate = new Ajv({ allErrors: true, strict: true }).compile(lawSchema);
const base = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets");
const records = fixtures.map((relative) => {
  const fixture = join(base, relative, "🧪️tests/round-trips-the-concrete-inverse/🔣️component.json");
  const source = join(base, relative, "🦀️component.rs");
  const value = JSON.parse(readFileSync(fixture, "utf8"));
  if (!validate(value)) throw new Error(`${relative}: ${JSON.stringify(validate.errors)}`);
  const sourceText = readFileSync(source, "utf8");
  if (!sourceText.includes("let expected: PdfSnapshot") || !sourceText.includes("let expected_inverse:") || !sourceText.includes("assert_eq!(state, expected)") || !sourceText.includes("assert_eq!(inverse, expected_inverse)")) throw new Error(`${relative}: typed snapshot or inverse comparison missing.`);
  if (sourceText.includes("expected_mutation")) throw new Error(`${relative}: redundant fixture mutation decode remains.`);
  if ((sourceText.match(/assert_json_shape\(&serde_json::to_value/g) ?? []).length !== 3) throw new Error(`${relative}: exact JSON-shape checks missing.`);
  const codecStart = sourceText.indexOf("for step in std::iter::once");
  const inverseStart = sourceText.indexOf("for step in inverse");
  if (codecStart < 0 || inverseStart <= codecStart || !sourceText.slice(codecStart, inverseStart).includes("serde_json::from_value") || !sourceText.slice(inverseStart).includes("assert_eq!(state, base)")) throw new Error(`${relative}: existing codec or concrete inverse execution is no longer downstream of the fixture checks.`);
  return { relative, fixtureSha256: createHash("sha256").update(readFileSync(fixture)).digest("hex"), sourceSha256: createHash("sha256").update(sourceText).digest("hex") };
});
console.log(`[DEBUG] ${JSON.stringify({ fixtureCount: records.length, records })}`);
