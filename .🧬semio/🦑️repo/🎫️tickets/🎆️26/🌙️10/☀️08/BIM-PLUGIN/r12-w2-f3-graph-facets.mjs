#!/usr/bin/env bun
/**
 * 🧬 Wave W2 `w2-f3-graph`: adds the `components` and `mep` fields of `ModelInference` (and the types they need, the new enum members of `SolidFamily`, `PlanKind`, `QuantityKind` and `DiagnosticCode`, and the
 * `groups` of the quantities) to the four facets of the inference schema family, `🔣️.json`, `🟦️.ts`, `🔗️.graphql` and `🛰️.proto`. Idempotent: a facet that already holds `ComponentValue` is left alone.
 * The rows mirror the Rust types of `🪑️components` and `🌀️mep`. Usage: `bun r12-w2-f3-graph-facets.mjs`.
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

const enums = {
  MepSystem: ["Supply", "Return", "Exhaust", "DomesticWater", "Waste", "Gas", "Power", "Data", "Lighting"],
  MepSectionKind: ["Duct", "Pipe", "Tray"],
  MepIssueCode: ["NonFinite", "SectionDegenerate", "PathDegenerate"],
  ComponentIssueCode: ["FamilyMissing", "FamilyProfile", "HostMissing", "HostOtherStorey", "HostDegenerate", "Override", "NonFinite"],
};
const additions = {
  SolidFamily: ["Component", "Mep"],
  PlanKind: ["ComponentOutline", "ComponentFront", "ComponentConnector", "MepAxis", "MepBand", "MepDrop"],
  QuantityKind: ["Component", "Mep"],
  DiagnosticCode: ["ComponentOutsideStorey", "ComponentInWall", "RefComponentFamily", "RefComponentHost", "ComponentOverride", "MepDegenerate", "MepClash", "TerminalUnconnected"],
};
const field = (name, type, required = true) => ({ name, type, required });
const structs = {
  Point3: [field("x", "number"), field("y", "number"), field("z", "number")],
  HostFit: [field("wall", "string"), field("station", "number"), field("side", "number"), field("face", "Point2"), field("normal", "Point2")],
  ComponentPlacement: [field("x", "number"), field("y", "number"), field("z", "number"), field("yaw", "number"), field("mirrored", "boolean"), field("host", "HostFit", false)],
  Connector: [field("system", "MepSystem"), field("colour", "string"), field("position", "Point3")],
  ComponentIssue: [field("code", "ComponentIssueCode"), field("subject", "string"), field("detail", "string"), field("family_issue", "FamilyIssue", false)],
  ComponentValue: [
    field("storey", "string"), field("family", "string"), field("category", "FamilyCategory", false), field("placement", "ComponentPlacement"), field("footprint", "Point2[]"), field("footprint_area", "number"),
    field("bounds", "SolidBounds"), field("volume", "number"), field("connector", "Connector", false), field("overridden", "string[]"), field("parameters", "map:ResolvedParameter"), field("issues", "ComponentIssue[]"),
  ],
  MepSection: [field("kind", "MepSectionKind"), field("width", "number"), field("height", "number"), field("area", "number"), field("perimeter", "number"), field("label", "string")],
  MepSegment: [field("from", "Point3"), field("to", "Point3"), field("length", "number")],
  MepIssue: [field("code", "MepIssueCode"), field("detail", "string")],
  MepValue: [
    field("storey", "string"), field("system", "MepSystem"), field("colour", "string"), field("section", "MepSection"), field("path", "Point3[]"), field("segments", "MepSegment[]"), field("length", "number"),
    field("volume", "number"), field("surface_area", "number"), field("bounds", "SolidBounds"), field("issues", "MepIssue[]"),
  ],
};
const scalars = new Set(["string", "number", "boolean"]);
const snake = (text) => text.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

//#region 🔖️Json
{
  const path = file(".json");
  const text = readFileSync(path, "utf8");
  const doc = JSON.parse(text);
  const ref = (name) => ({ $ref: `#/$defs/${name}` });
  const typeOf = (type) => {
    if (scalars.has(type)) return { type };
    if (type.endsWith("[]")) return { type: "array", items: typeOf(type.slice(0, -2)) };
    if (type.startsWith("map:")) return { type: "object", additionalProperties: ref(type.slice(4)) };
    return ref(type);
  };
  const add = (list, values) => values.forEach((value) => { if (!list.includes(value)) list.push(value); });
  let changed = false;
  if (!doc.$defs.ComponentValue) {
    changed = true;
    for (const [name, values] of Object.entries(enums)) doc.$defs[name] = { enum: values };
    for (const [name, rows] of Object.entries(structs)) {
      doc.$defs[name] = { type: "object", additionalProperties: false, required: rows.filter((row) => row.required).map((row) => row.name), properties: Object.fromEntries(rows.map((row) => [row.name, typeOf(row.type)])) };
    }
    doc.required.push("components", "mep");
    doc.properties.components = { type: "object", additionalProperties: ref("ComponentValue"), "x-semio-derived": true };
    doc.properties.mep = { type: "object", additionalProperties: ref("MepValue"), "x-semio-derived": true };
    const quantity = doc.$defs.ElementQuantity;
    if (quantity && !quantity.properties.groups) quantity.properties.groups = { type: "array", items: { type: "string" } };
    const totals = doc.$defs.QuantityTotals;
    if (totals && !totals.properties.groups) totals.properties.groups = { type: "object", additionalProperties: ref("Totals") };
  }
  for (const [name, values] of Object.entries(additions)) {
    const def = doc.$defs[name];
    if (def?.enum) {
      const before = def.enum.length;
      add(def.enum, values);
      changed ||= def.enum.length !== before;
    }
  }
  if (changed) writeFileSync(path, JSON.stringify(doc, null, 2) + "\n");
}
//#endregion 🔖️Json

//#region 🔖️TypeScript
{
  const path = file(".ts");
  let text = readFileSync(path, "utf8");
  const tsType = (type) => {
    if (scalars.has(type)) return type;
    if (type.endsWith("[]")) return `${tsType(type.slice(0, -2))}[]`;
    if (type.startsWith("map:")) return `Record<string, ${type.slice(4)}>`;
    return type;
  };
  if (!text.includes("interface ComponentValue")) {
    const union = (names) => names.map((name) => `"${name}"`).join(" | ");
    const block = [
      ...Object.entries(enums).map(([name, values]) => `export type ${name} = ${union(values)};`),
      ...Object.entries(structs).map(([name, rows]) => `export interface ${name} {\n${rows.map((row) => `  ${row.name}${row.required ? "" : "?"}: ${tsType(row.type)};`).join("\n")}\n}`),
    ].join("\n\n");
    text = text.replace("export interface ModelInference {", `${block}\n\nexport interface ModelInference {`);
    text = text.replace("  families: Record<string, FamilyValue>;\n", "  families: Record<string, FamilyValue>;\n  components: Record<string, ComponentValue>;\n  mep: Record<string, MepValue>;\n");
    text = text.replace("  mullions?: MullionQuantity[];\n}", "  mullions?: MullionQuantity[];\n  groups?: string[];\n}");
    text = text.replace("  finishes: Record<string, Totals>;\n}", "  finishes: Record<string, Totals>;\n  groups?: Record<string, Totals>;\n}");
  }
  for (const [name, values] of Object.entries(additions)) {
    text = text.replace(new RegExp(`(export type ${name} = [^;]*)(;)`), (_, head, end) => `${head}${values.filter((value) => !head.includes(`"${value}"`)).map((value) => ` | "${value}"`).join("")}${end}`);
  }
  writeFileSync(path, text);
}
//#endregion 🔖️TypeScript

//#region 🔖️GraphQL
{
  const path = file(".graphql");
  let text = readFileSync(path, "utf8");
  const gqlType = (type) => {
    if (type === "string") return "String";
    if (type === "number") return "Float";
    if (type === "boolean") return "Boolean";
    if (type.endsWith("[]")) return `[${gqlType(type.slice(0, -2))}!]`;
    if (type.startsWith("map:")) return `[${type.slice(4)}Row!]`;
    return type;
  };
  if (!text.includes("type ComponentValue")) {
    const enumOf = (name, values) => `enum ${name} {\n${values.map((value) => `  ${value}`).join("\n")}\n}\n`;
    const row = (name) => `type ${name}Row {\n  id: String!\n  value: ${name}!\n}\n`;
    const block = [
      ...Object.entries(enums).map(([name, values]) => enumOf(name, values)),
      ...Object.entries(structs).map(([name, rows]) => `type ${name} {\n${rows.map((r) => `  ${r.name}: ${gqlType(r.type)}${r.required ? "!" : ""}`).join("\n")}\n}\n`),
      row("ComponentValue"),
      row("MepValue"),
    ].join("\n");
    text = text.replace("  families: [FamilyValueRow!]! @derived\n", "  families: [FamilyValueRow!]! @derived\n  components: [ComponentValueRow!]! @derived\n  mep: [MepValueRow!]! @derived\n");
    text = text.replace("  mullions: [MullionQuantity!]!\n}", "  mullions: [MullionQuantity!]!\n  groups: [String!]\n}");
    text = text.replace("  finishes: [TotalsRow!]!\n}", "  finishes: [TotalsRow!]!\n  groups: [TotalsRow!]\n}");
    text = text.replace(/\s*$/, "\n") + "\n" + block;
  }
  for (const [name, values] of Object.entries(additions)) {
    text = text.replace(new RegExp(`(enum ${name} \\{\\n[\\s\\S]*?)(\\n\\})`), (_, head, end) => `${head}${values.filter((value) => !new RegExp(`\\n  ${value}(\\n|$)`).test(head)).map((value) => `\n  ${value}`).join("")}${end}`);
  }
  writeFileSync(path, text);
}
//#endregion 🔖️GraphQL

//#region 🔖️Proto
{
  const path = file(".proto");
  let text = readFileSync(path, "utf8");
  const protoType = (type) => {
    if (type === "string") return "string";
    if (type === "number") return "double";
    if (type === "boolean") return "bool";
    return type;
  };
  const fieldLine = (row, index) => {
    if (row.type.endsWith("[]")) return `  repeated ${protoType(row.type.slice(0, -2))} ${row.name} = ${index};`;
    if (row.type.startsWith("map:")) return `  map<string, ${row.type.slice(4)}> ${row.name} = ${index};`;
    return `  ${row.required ? "" : "optional "}${protoType(row.type)} ${row.name} = ${index};`;
  };
  if (!text.includes("message ComponentValue")) {
    const enumOf = (name, values) => `enum ${name} {\n  ${snake(name)}_UNSPECIFIED = 0;\n${values.map((value, index) => `  ${snake(name)}_${snake(value)} = ${index + 1};`).join("\n")}\n}\n`;
    const block = [...Object.entries(enums).map(([name, values]) => enumOf(name, values)), ...Object.entries(structs).map(([name, rows]) => `message ${name} {\n${rows.map((row, index) => fieldLine(row, index + 1)).join("\n")}\n}\n`)].join("\n");
    const head = text.match(/message ModelInference \{([\s\S]*?)\n\}/);
    const next = Math.max(...[...head[1].matchAll(/= (\d+);/g)].map((match) => Number(match[1]))) + 1;
    text = text.replace(/(message ModelInference \{[\s\S]*?)(\n\})/, (_, body, end) => `${body}\n  // @derived\n  map<string, ComponentValue> components = ${next};\n  // @derived\n  map<string, MepValue> mep = ${next + 1};${end}`);
    text = text.replace(/(message ElementQuantity \{[\s\S]*?)(\n\})/, (_, body, end) => {
      const used = Math.max(...[...body.matchAll(/= (\d+);/g)].map((match) => Number(match[1])));
      return `${body}\n  repeated string groups = ${used + 1};${end}`;
    });
    text = text.replace(/(message QuantityTotals \{[\s\S]*?)(\n\})/, (_, body, end) => {
      const used = Math.max(...[...body.matchAll(/= (\d+);/g)].map((match) => Number(match[1])));
      return `${body}\n  map<string, Totals> groups = ${used + 1};${end}`;
    });
    text = text.replace(/\s*$/, "\n") + "\n" + block;
  }
  for (const [name, values] of Object.entries(additions)) {
    text = text.replace(new RegExp(`(enum ${name} \\{\\n[\\s\\S]*?)(\\n\\})`), (_, body, end) => {
      const numbers = [...body.matchAll(/= (\d+);/g)].map((match) => Number(match[1]));
      const first = Math.max(...numbers) + 1;
      const fresh = values.filter((value) => !body.includes(`${snake(name)}_${snake(value)} =`));
      return `${body}${fresh.map((value, index) => `\n  ${snake(name)}_${snake(value)} = ${first + index};`).join("")}${end}`;
    });
  }
  writeFileSync(path, text);
}
//#endregion 🔖️Proto

console.log("components and mep inference facets written");
