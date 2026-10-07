import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { parseProgramArtifact } from "../../🟦️.ts";
import {programArtifactFromJson} from "../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import {programDiffFromJson} from "../../../🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts";
import { parseProgramSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseProgramDiff } from "../../🔺️diff/🟦️.ts";

type JsonObject = Record<string, any>;

const here = dirname(fileURLToPath(import.meta.url));
const schemaRoot = join(here, "../..");
const subsetRoot = join(schemaRoot, "..");
const snapshotRoot = join(subsetRoot, "🧫️fixtures/🧬️mutations");
const neutralPath = join(schemaRoot, "🧫️fixtures/🔬️document-contract/🔣️.json");
let repoRoot = here;
while (!existsSync(join(repoRoot, "📋️project.json"))) {
  const parent = dirname(repoRoot);
  if (parent === repoRoot) throw new Error("repository root not found");
  repoRoot = parent;
}

const readJson = (path: string): JsonObject => JSON.parse(readFileSync(path, "utf8"));
const collect = (root: string, suffix: string): string[] => {
  const paths: string[] = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const path = join(root, entry.name);
    if (entry.isDirectory()) paths.push(...collect(path, suffix));
    else if (path.endsWith(suffix)) paths.push(path);
  }
  return paths;
};

const expectedFields = [
  "schema", "meta", "project", "stakeholders", "users", "activities", "functions", "elements", "quantities", "relationships", "adjacencies", "processes", "flows", "accessRules", "operations", "equipment", "resources", "storage", "environmental", "humanFactors", "accessibility", "privacy", "safety", "security", "regulatory", "siteContext", "organizational", "services", "infrastructure", "information", "communication", "wayfinding", "schedules", "flexibility", "growth", "sustainability", "resilience", "costs", "delivery", "risks", "conflicts", "requirements", "priorities", "scenarios", "options", "decisions", "validations", "performance", "quality", "artifacts", "assumptions", "constraints", "complianceRecords", "approvals", "meetings", "changes", "collaboration", "analyses", "reports", "searchFilters", "statusRecords", "workshops", "surveys", "issues", "auditEvents", "templates", "knowledgePayload", "knowledge", "benchmarksPayload", "benchmarks", "traces", "governance",
] as const;

const assertChild = (child: JsonObject, label: string): void => {
  assert.equal(typeof child.childId, "string", `${label}.childId`);
  assert.equal(child.target.artifactId, child.childId, `${label} target identity`);
  assert.deepEqual(child.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset: "table" }, `${label} target dialect`);
};

const compile = (schema: JsonObject, dependencies: JsonObject[] = []) => {
  const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(artifactReferenceSchema);
  for (const dependency of dependencies) ajv.addSchema(dependency);
  return ajv.compile(schema);
};

export function testProgramDocumentContract(): void {
  const artifactSchema = readJson(join(schemaRoot, "🔣️.json"));
  const snapshotSchema = readJson(join(schemaRoot, "📸️snapshot/🔣️.json"));
  const diffSchema = readJson(join(schemaRoot, "🔺️diff/🔣️.json"));
  const childSchema = readJson(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json"));
  const ioSchema = readJson(join(repoRoot, "🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json"));
  const validateArtifact = compile(artifactSchema, [childSchema, ioSchema]);
  const validateSnapshot = compile(snapshotSchema, [childSchema, ioSchema]);
  const validateDiff = compile(diffSchema, [artifactSchema, childSchema, ioSchema]);
  assert.deepEqual(Object.keys(artifactSchema.properties), expectedFields);
  assert.deepEqual(artifactSchema.required, expectedFields);
  assert.deepEqual(Object.keys(snapshotSchema.properties), expectedFields);
  assert.deepEqual(snapshotSchema.required, expectedFields);
  assert.deepEqual(Object.keys(diffSchema.properties), ["artifact", ...expectedFields]);
  assert.deepEqual(diffSchema.required, ["artifact", ...expectedFields]);

  for (const field of expectedFields.filter((name) => artifactSchema.properties[name]?.type === "array")) {
    const ref = artifactSchema.properties[field].items?.$ref;
    assert.equal(typeof ref, "string", `${field}: typed register item reference`);
    const definition = artifactSchema.$defs[ref.slice("#/$defs/".length)];
    assert.equal(definition.additionalProperties, false, `${field}: exact register row`);
    assert(definition.required.length > 0, `${field}: register row required fields`);
  }

  const snapshotPaths = collect(snapshotRoot, "/📸️snapshot/⬅️before/🔣️.json").concat(collect(snapshotRoot, "/📸️snapshot/➡️after/🔣️.json")).sort();
  const diffPaths = collect(snapshotRoot, "/🔺️diff/🔣️.json").sort();
  assert.equal(snapshotPaths.length, 532, "committed snapshot count");
  assert.equal(diffPaths.length, 260, "committed diff count");
  const registerFields = expectedFields.filter((field) => artifactSchema.properties[field]?.type === "array");
  const covered = new Set<string>();
  for (const path of snapshotPaths) {
    const value = readJson(path);
    assert(validateArtifact(value), `${path}: ${JSON.stringify(validateArtifact.errors)}`);
    assert(validateSnapshot(value), `${path}: ${JSON.stringify(validateSnapshot.errors)}`);
    const canonical=programArtifactFromJson(value);
    assert.deepEqual(parseProgramArtifact(canonical), canonical, `${path}: artifact parser`);
    assert.deepEqual(parseProgramSnapshot(canonical), canonical, `${path}: snapshot parser`);
    assert.deepEqual(Object.keys(value), expectedFields, `${path}: exact root fields`);
    assertChild(value.knowledge, `${path}.knowledge`);
    assertChild(value.benchmarks, `${path}.benchmarks`);
    for (const field of registerFields) if (value[field].length > 0) covered.add(field);
  }
  assert.deepEqual([...covered].sort(), [...registerFields].sort(), "every typed register has committed row coverage");
  for (const path of diffPaths) {
    const value = readJson(path);
    assert(validateDiff(value), `${path}: ${JSON.stringify(validateDiff.errors)}`);
    const canonicalDiff=(() => { try { return programDiffFromJson(value); } catch (error) { throw new Error(`${path}: ${error instanceof Error ? error.message : String(error)}`); } })();
    assert.deepEqual(parseProgramDiff(canonicalDiff), canonicalDiff, `${path}: diff parser`);
    assert.deepEqual(Object.keys(value), ["artifact", ...expectedFields], `${path}: exact diff fields`);
    if (value.knowledge !== null) assertChild(value.knowledge, `${path}.knowledge`);
    if (value.benchmarks !== null) assertChild(value.benchmarks, `${path}.benchmarks`);
  }

  const neutral = readJson(neutralPath);
  assert(validateArtifact(neutral), JSON.stringify(validateArtifact.errors));
  const canonicalNeutral=programArtifactFromJson(neutral);
  assert.deepEqual(parseProgramSnapshot(canonicalNeutral), canonicalNeutral);
  assert.throws(() => parseProgramSnapshot({ ...canonicalNeutral, documents: [] }), /unknown field/, "obsolete documents alias is rejected");
  assert.throws(() => parseProgramSnapshot({ ...canonicalNeutral, project: { ...canonicalNeutral.project, foreign: true } }), /unknown field/, "nested register primitives reject foreign fields");
  assert.throws(() => assertChild({ ...neutral.knowledge, target: { ...neutral.knowledge.target, artifactId: "wrong" } }, "wrong-child"), /target identity/, "child identity mismatch is rejected");
  assert.equal(neutral.meta.locale, "en", "ProgramMeta.locale remains document content language");
}
