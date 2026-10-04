import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { createHash } from "node:crypto";
import { createPatch, applyPatch } from "diff";
import Ajv from "ajv";
import JSON5 from "json5";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), helper = resolve(ticket, "graph-os-record-cut-proposal/📜️script.ts");
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const before = "use old_owner::Value;\nfn retained() {}\n", authored = "use canonical_owner::Value;\nfn retained() {}\n", prefix = "fn concurrent() {}\n";
const fixture = { version: 1, before, authored, cases: [
  { id: "original", current: before, expected: authored },
  { id: "exact-authored", current: authored, expected: authored },
  { id: "foreign-original", current: prefix + before, expected: prefix + authored },
  { id: "foreign-authored-requires-admission", current: prefix + authored, expected: null },
  { id: "unrelated-refused", current: "fn unrelated() {}\n", expected: null },
] };
const schema = {
  $schema: "http://json-schema.org/draft-07/schema#", type: "object", additionalProperties: false, required: ["version", "before", "authored", "cases"],
  properties: { version: { const: 1 }, before: { type: "string" }, authored: { type: "string" }, cases: { type: "array", minItems: 5, maxItems: 5, items: {
    type: "object", additionalProperties: false, required: ["id", "current", "expected"], properties: { id: { enum: fixture.cases.map(row => row.id) }, current: { type: "string" }, expected: { anyOf: [{ type: "string" }, { type: "null" }] } },
  } } },
};
const extract = (source: string) => {
  const expressions = [...source.matchAll(/before=(overlay\?\([^;]+\):actualBefore);assert\.notEqual\(before,false,/g)].map(match => match[1]);
  if (expressions.length !== 1) throw new Error("actual overlay selector absent or ambiguous");
  return expressions[0]!;
};
if (process.argv[2] === "prepare") {
  const source = readFileSync(helper, "utf8"), expression = extract(source), output = resolve(import.meta.dir, "root-actual-selector-before-1.json");
  if (existsSync(output)) throw new Error("baseline authority already exists");
  writeFileSync(output, JSON.stringify({ state: "actual-helper-before-test-first", helper, source, sha256: hash(source), inverse: source, expression }, null, 2) + "\n");
  writeFileSync(resolve(import.meta.dir, "🔣️fixture.json"), JSON.stringify(fixture, null, 2) + "\n");
  writeFileSync(resolve(import.meta.dir, "🧬️schema.json"), JSON.stringify(schema, null, 2) + "\n");
  console.log("[DEBUG] captured actual overlay selector and five closed language-neutral cases; no helper/source changes");
} else if (process.argv[2] === "prove") {
  const label = process.argv[3]; if (!label) throw new Error("unique proof label required");
  const source = readFileSync(helper, "utf8"), expression = extract(source), corpusText = readFileSync(resolve(import.meta.dir, "🔣️fixture.json"), "utf8"), corpus = JSON.parse(corpusText), actualSchema = JSON.parse(readFileSync(resolve(import.meta.dir, "🧬️schema.json"), "utf8"));
  const { validateJsonSchemaSubset } = await import(resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
  const oracle = new Ajv({ strict: true }).compile(actualSchema), hostiles = [Object.assign(structuredClone(corpus), { unknown: true }), (() => { const value = structuredClone(corpus); value.cases[0].unknown = true; return value; })(), Object.assign(structuredClone(corpus), { version: 2 }), (() => { const value = structuredClone(corpus); value.cases[0].expected = 1; return value; })(), (() => { const value = structuredClone(corpus); value.cases.pop(); return value; })()];
  if (validateJsonSchemaSubset(actualSchema, corpus).length || !oracle(corpus) || hostiles.some(value => !validateJsonSchemaSubset(actualSchema, value).length || oracle(value)) || JSON.stringify(JSON5.parse(corpusText)) !== JSON.stringify(corpus)) throw new Error("schema/oracle/hostile corpus admission differs");
  const select = new Function("overlay", "actualBefore", "applyPatch", "return " + expression) as (overlay: unknown, current: string, patch: typeof applyPatch) => string | false;
  const overlay = { before: corpus.before, authored: corpus.authored, forward: createPatch("owner.rs", corpus.before, corpus.authored) };
  const rows = corpus.cases.map((row: { id: string; current: string; expected: string | null }) => {
    const observed = select(overlay, row.current, applyPatch), actual = observed === false ? null : observed;
    return { ...row, actual, passed: actual === row.expected };
  });
  const plain = select(null, corpus.before, applyPatch), unchangedNoOverlay = plain === corpus.before;
  const directory = resolve(ticket, "🗑️generated/current-overlay-selector"); mkdirSync(directory, { recursive: true });
  const output = resolve(directory, `root-actual-${label}.json`); if (existsSync(output)) throw new Error("proof already exists");
  const failed = rows.filter((row: { passed: boolean }) => !row.passed);
  writeFileSync(output, JSON.stringify({ state: failed.length ? "actual-selector-red" : "actual-selector-green", helper, source, sha256: hash(source), inverse: source, expression, rows, unchangedNoOverlay, hostileCount: hostiles.length, firstPartySchemaAndAjv: true, independentJson5Parity: true, thirdPartyPatchLibrary: "diff", sourceWrites: 0, passed: rows.length - failed.length, failed: failed.length }, null, 2) + "\n");
  console.log(`[DEBUG] actual overlay selector ${rows.length - failed.length}PASS/${failed.length}FAIL; no-overlay=${unchangedNoOverlay}; five closed hostile refusals; no helper/source changes`);
  if (failed.length || !unchangedNoOverlay) process.exitCode = 1;
} else {
  throw new Error("unknown overlay ticket command");
}
