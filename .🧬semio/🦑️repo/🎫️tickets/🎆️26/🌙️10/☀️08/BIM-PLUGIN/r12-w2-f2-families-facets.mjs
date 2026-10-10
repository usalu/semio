#!/usr/bin/env bun
/**
 * 🧬 Wave W2 `w2-f2-families`: adds the `families` field of `ModelInference` (and the types it needs, plus the new diagnostic codes) to the four facets of the inference schema family, `🔣️.json`, `🟦️.ts`,
 * `🔗️.graphql` and `🛰️.proto`. Idempotent: a facet that already holds `FamilyValue` is left alone. The rows mirror the Rust types of `🧬️families`.
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix)));
const plugins = sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))), "plugins");
const model = sub(sub(sub(plugins, "bim"), "artifacts"), "model");
const subsets = sub(sub(sub(model, "standards"), "1"), "subsets");
const S = join(subsets, readdirSync(subsets).find((n) => n.endsWith("any")));
const I = sub(sub(S, "schema"), "inferences");
const file = (suffix) => join(I, readdirSync(I).find((name) => name.endsWith(suffix)));

const categories = ["Furniture", "Equipment", "Casework", "Plumbing", "Lighting", "Mechanical", "Electrical", "Generic", "Profile"];
const kinds = ["Length", "Angle", "Real", "Integer", "Boolean", "Text", "Material"];
const owners = ["Parameter", "Solid", "Family"];
const issueCodes = ["Syntax", "Kind", "Cycle", "Unknown", "DivisionByZero", "Negative", "Dependency", "Domain", "Outline"];
const diagnosticCodes = ["FamilySyntax", "FamilyKind", "FamilyCycle", "FamilyUnknown", "FamilyDivisionByZero", "FamilyNegative", "FamilyDependency", "FamilyDomain", "FamilyOutline", "RefProfileFamily"];
const values = ["Number", "Length", "Angle", "Boolean", "Text"];
const valueType = { Number: "number", Length: "number", Angle: "number", Boolean: "boolean", Text: "string" };

//#region 🔖️Json
{
  const path = file(".json");
  const text = readFileSync(path, "utf8");
  if (!text.includes('"FamilyValue"')) {
    const doc = JSON.parse(text);
    const ref = (name) => ({ $ref: `#/$defs/${name}` });
    const object = (required, properties) => ({ type: "object", additionalProperties: false, required, properties });
    doc.required.push("families");
    doc.properties.families = { type: "object", additionalProperties: ref("FamilyValue"), "x-semio-derived": true };
    Object.assign(doc.$defs, {
      FamilyCategory: { enum: categories },
      ParameterKind: { enum: kinds },
      IssueOwner: { enum: owners },
      FamilyIssueCode: { enum: issueCodes },
      FamilyIssue: object(["code", "owner", "subject", "field", "detail"], { code: ref("FamilyIssueCode"), owner: ref("IssueOwner"), subject: { type: "string" }, field: { type: "string" }, path: { type: "array", items: { type: "integer" } }, detail: { type: "string" }, names: { type: "array", items: { type: "string" } } }),
      ParameterValue: { oneOf: values.map((name) => object([name], { [name]: object(["value"], { value: { type: valueType[name] } }) })) },
      ResolvedParameter: object(["kind", "formula"], { kind: ref("ParameterKind"), formula: { type: "string" }, value: ref("ParameterValue") }),
      FamilySolidMesh: object(["name", "material", "visible", "bounds", "volume", "area"], { name: { type: "string" }, material: { type: "string" }, visible: { type: "boolean" }, positions: { type: "array", items: { type: "number" } }, normals: { type: "array", items: { type: "number" } }, indices: { type: "array", items: { type: "integer" } }, bounds: ref("SolidBounds"), volume: { type: "number" }, area: { type: "number" } }),
      FamilyValue: object(["name"], { name: { type: "string" }, category: ref("FamilyCategory"), order: { type: "array", items: { type: "string" } }, parameters: { type: "object", additionalProperties: ref("ResolvedParameter") }, solids: { type: "object", additionalProperties: ref("FamilySolidMesh") }, outline: { type: "array", items: ref("Vertex") }, issues: { type: "array", items: ref("FamilyIssue") } }),
    });
    const codes = doc.$defs.DiagnosticCode.enum;
    for (const code of diagnosticCodes) if (!codes.includes(code)) codes.push(code);
    writeFileSync(path, JSON.stringify(doc, null, 2) + "\n");
  }
}
//#endregion 🔖️Json

//#region 🔖️TypeScript
{
  const path = file(".ts");
  let text = readFileSync(path, "utf8");
  if (!text.includes("FamilyValue")) {
    const union = (names) => names.map((name) => `"${name}"`).join(" | ");
    const block = [
      `export type FamilyCategory = ${union(categories)};`,
      `export type ParameterKind = ${union(kinds)};`,
      `export type IssueOwner = ${union(owners)};`,
      `export type FamilyIssueCode = ${union(issueCodes)};`,
      `export interface FamilyIssue {\n  code: FamilyIssueCode;\n  owner: IssueOwner;\n  subject: string;\n  field: string;\n  path: number[];\n  detail: string;\n  names: string[];\n}`,
      `export type ParameterValue = ${values.map((name) => `{ ${name}: { value: ${valueType[name]} } }`).join(" | ")};`,
      `export interface ResolvedParameter {\n  kind: ParameterKind;\n  formula: string;\n  value?: ParameterValue;\n}`,
      `export interface FamilySolidMesh {\n  name: string;\n  material: string;\n  visible: boolean;\n  positions: number[];\n  normals: number[];\n  indices: number[];\n  bounds: SolidBounds;\n  volume: number;\n  area: number;\n}`,
      `export interface FamilyValue {\n  name: string;\n  category?: FamilyCategory;\n  order: string[];\n  parameters: Record<string, ResolvedParameter>;\n  solids: Record<string, FamilySolidMesh>;\n  outline: Vertex[];\n  issues: FamilyIssue[];\n}`,
    ].join("\n\n");
    text = text.replace("export interface ModelInference {", `${block}\n\nexport interface ModelInference {`);
    text = text.replace("  ramp_runs: Record<string, RampRun>;\n", "  ramp_runs: Record<string, RampRun>;\n  families: Record<string, FamilyValue>;\n");
    text = text.replace(/(export type DiagnosticCode = [^;]*)(;)/, (_, head, end) => `${head}${diagnosticCodes.filter((code) => !head.includes(`"${code}"`)).map((code) => ` | "${code}"`).join("")}${end}`);
    writeFileSync(path, text);
  }
}
//#endregion 🔖️TypeScript

//#region 🔖️GraphQL
{
  const path = file(".graphql");
  let text = readFileSync(path, "utf8");
  if (!text.includes("FamilyValue")) {
    const enumOf = (name, names) => `enum ${name} {\n${names.map((value) => `  ${value}`).join("\n")}\n}\n`;
    const row = (name, value) => `type ${name}Row {\n  id: String!\n  value: ${value}!\n}\n`;
    const block = [
      enumOf("FamilyCategory", categories),
      enumOf("ParameterKind", kinds),
      enumOf("IssueOwner", owners),
      enumOf("FamilyIssueCode", issueCodes),
      "type FamilyIssue {\n  code: FamilyIssueCode!\n  owner: IssueOwner!\n  subject: String!\n  field: String!\n  path: [Int!]!\n  detail: String!\n  names: [String!]!\n}\n",
      `# externally tagged: exactly one member is present\ntype ParameterValue {\n${values.map((name) => `  ${name}: ParameterValue${name}`).join("\n")}\n}\n`,
      ...values.map((name) => `type ParameterValue${name} {\n  value: ${{ number: "Float", boolean: "Boolean", string: "String" }[valueType[name]]}!\n}\n`),
      "type ResolvedParameter {\n  kind: ParameterKind!\n  formula: String!\n  value: ParameterValue\n}\n",
      row("ResolvedParameter", "ResolvedParameter"),
      "type FamilySolidMesh {\n  name: String!\n  material: String!\n  visible: Boolean!\n  positions: [Float!]!\n  normals: [Float!]!\n  indices: [Int!]!\n  bounds: SolidBounds!\n  volume: Float!\n  area: Float!\n}\n",
      row("FamilySolidMesh", "FamilySolidMesh"),
      "type FamilyValue {\n  name: String!\n  category: FamilyCategory\n  order: [String!]!\n  parameters: [ResolvedParameterRow!]!\n  solids: [FamilySolidMeshRow!]!\n  outline: [Vertex!]!\n  issues: [FamilyIssue!]!\n}\n",
      row("FamilyValue", "FamilyValue"),
    ].join("\n");
    text = text.replace("  ramp_runs: [RampRunRow!]! @derived\n", "  ramp_runs: [RampRunRow!]! @derived\n  families: [FamilyValueRow!]! @derived\n");
    text = text.replace(/(enum DiagnosticCode \{\n[\s\S]*?)(\n\})/, (_, head, end) => `${head}${diagnosticCodes.filter((code) => !head.includes(`  ${code}\n`) && !head.endsWith(`  ${code}`)).map((code) => `\n  ${code}`).join("")}${end}`);
    text = text.replace(/\s*$/, "\n") + "\n" + block;
    writeFileSync(path, text);
  }
}
//#endregion 🔖️GraphQL

//#region 🔖️Proto
{
  const path = file(".proto");
  let text = readFileSync(path, "utf8");
  if (!text.includes("FamilyValue")) {
    const head = text.match(/message ModelInference \{([\s\S]*?)\n\}/);
    const used = [...head[1].matchAll(/= (\d+);/g)].map((match) => Number(match[1]));
    const next = Math.max(...used) + 1;
    const enumOf = (name, names) => `enum ${name} {\n  ${name.toUpperCase()}_UNSPECIFIED = 0;\n${names.map((value, index) => `  ${name.toUpperCase()}_${value.toUpperCase()} = ${index + 1};`).join("\n")}\n}\n`;
    const scalar = { number: "double", boolean: "bool", string: "string" };
    const block = [
      enumOf("FamilyCategory", categories),
      enumOf("ParameterKind", kinds),
      enumOf("IssueOwner", owners),
      enumOf("FamilyIssueCode", issueCodes),
      "message FamilyIssue {\n  FamilyIssueCode code = 1;\n  IssueOwner owner = 2;\n  string subject = 3;\n  string field = 4;\n  repeated int32 path = 5;\n  string detail = 6;\n  repeated string names = 7;\n}\n",
      `message ParameterValue {\n  oneof variant {\n${values.map((name, index) => `    ParameterValue${name} ${name.toLowerCase()} = ${index + 1};`).join("\n")}\n  }\n}\n`,
      ...values.map((name) => `message ParameterValue${name} {\n  ${scalar[valueType[name]]} value = 1;\n}\n`),
      "message ResolvedParameter {\n  ParameterKind kind = 1;\n  string formula = 2;\n  optional ParameterValue value = 3;\n}\n",
      "message FamilySolidMesh {\n  string name = 1;\n  string material = 2;\n  bool visible = 3;\n  repeated double positions = 4;\n  repeated double normals = 5;\n  repeated uint32 indices = 6;\n  SolidBounds bounds = 7;\n  double volume = 8;\n  double area = 9;\n}\n",
      "message FamilyValue {\n  string name = 1;\n  optional FamilyCategory category = 2;\n  repeated string order = 3;\n  map<string, ResolvedParameter> parameters = 4;\n  map<string, FamilySolidMesh> solids = 5;\n  repeated Vertex outline = 6;\n  repeated FamilyIssue issues = 7;\n}\n",
    ].join("\n");
    text = text.replace(/(message ModelInference \{[\s\S]*?)(\n\})/, (_, body, end) => `${body}\n  // @derived\n  map<string, FamilyValue> families = ${next};${end}`);
    text = text.replace(/(enum DiagnosticCode \{\n[\s\S]*?)(\n\})/, (_, body, end) => {
      const numbers = [...body.matchAll(/= (\d+);/g)].map((match) => Number(match[1]));
      const first = Math.max(...numbers) + 1;
      const rows = diagnosticCodes.map((code, index) => `  DIAGNOSTIC_CODE_${code.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()} = ${first + index};`);
      return `${body}\n${rows.join("\n")}${end}`;
    });
    text = text.replace(/\s*$/, "\n") + "\n" + block;
    writeFileSync(path, text);
  }
}
//#endregion 🔖️Proto

console.log("families inference facets written");
