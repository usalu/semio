#!/usr/bin/env bun
/** 🏠️ Idempotently splices the `stair-runs`, `spaces` and `quantities` fields into the json/ts/graphql/proto facets of `s.bim.model.inference` and writes the per-leaf `🟦️.ts` (other agents edit the same files, so every write is a single textual insertion re-read just before). */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "C:/git/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences";
const read = (name: string) => readFileSync(join(root, name), "utf8");
const write = (name: string, text: string) => writeFileSync(join(root, name), text);

type Fields = [string, string][];
const enums: Record<string, { prefix: string; values: string[] }> = {
  SpaceStatus: { prefix: "SPACE_STATUS", values: ["Inferred", "Explicit", "NotEnclosed", "SeedInsideWall", "InvalidOutline"] },
  QuantityKind: { prefix: "QUANTITY_KIND", values: ["Wall", "CurtainWall", "Slab", "Roof", "Column", "Beam", "Window", "Door", "Void", "Stair", "Railing", "Space"] },
};
const messages: Record<string, Fields> = {
  Point2: [["x", "f64"], ["y", "f64"]],
  Vertex: [["point", "Point2"], ["bulge", "f64"]],
  VertexLoop: [["vertices", "[Vertex]"]],
  StairWinder: [["centre", "Point2"], ["inner_radius", "f64"], ["outer_radius", "f64"], ["start_angle", "f64"], ["sweep", "f64"]],
  StairFlightRun: [["first_riser", "u32"], ["risers", "u32"], ["treads", "u32"], ["start", "Point2"], ["direction", "f64"], ["tread", "f64"], ["base_z", "f64"], ["length", "f64"], ["winder", "?StairWinder"]],
  StairLanding: [["after_flight", "u32"], ["z", "f64"], ["centre", "Point2"], ["direction", "f64"], ["width", "f64"], ["depth", "f64"]],
  StairCompliance: [["rise_positive", "bool"], ["riser_ok", "bool"], ["tread_ok", "bool"], ["blondel_ok", "bool"], ["compliant", "bool"]],
  StairRun: [["base_z", "f64"], ["top_z", "f64"], ["rise", "f64"], ["riser_count", "u32"], ["riser_height", "f64"], ["tread_count", "u32"], ["tread", "f64"], ["stride", "f64"], ["width", "f64"], ["run_length", "f64"], ["flights", "[StairFlightRun]"], ["landings", "[StairLanding]"], ["compliance", "StairCompliance"]],
  SpaceRoom: [["status", "SpaceStatus"], ["outline", "[Vertex]"], ["holes", "[[Vertex]]"], ["point", "Point2"], ["area", "f64"], ["perimeter", "f64"], ["net_floor_area", "f64"], ["floor_z", "f64"], ["clear_height", "f64"], ["volume", "f64"], ["ceiling_slab", "string"], ["bounding_walls", "[string]"]],
  LayerQuantity: [["material", "string"], ["thickness", "f64"], ["area", "f64"], ["volume", "f64"], ["mass", "f64"]],
  ElementQuantity: [["kind", "QuantityKind"], ["storey", "string"], ["type_id", "string"], ["count", "u32"], ["length", "f64"], ["width", "f64"], ["height", "f64"], ["perimeter", "f64"], ["gross_side_area", "f64"], ["opening_area", "f64"], ["net_side_area", "f64"], ["gross_area", "f64"], ["net_area", "f64"], ["surface_area", "f64"], ["gross_volume", "f64"], ["net_volume", "f64"], ["mass", "f64"], ["risers", "u32"], ["layers", "[LayerQuantity]"]],
  Totals: [["count", "u32"], ["length", "f64"], ["area", "f64"], ["volume", "f64"], ["mass", "f64"]],
  QuantityTotals: [["kinds", "{Totals}"], ["types", "{Totals}"], ["materials", "{Totals}"]],
  ModelQuantities: [["elements", "{ElementQuantity}"], ["storeys", "{QuantityTotals}"], ["buildings", "{QuantityTotals}"], ["project", "QuantityTotals"]],
};
const leaves: Record<string, { doc: string; names: string[] }> = {
  "🪜️stair-runs": { doc: "🪜️ `stair-runs`: the resolved run of a stair; the foot of riser `k` of a flight stands at `start + k * tread` along `direction` and rises `riser_height` per riser.", names: ["Point2", "StairWinder", "StairFlightRun", "StairLanding", "StairCompliance", "StairRun"] },
  "🏠️spaces": { doc: "🏠️ `spaces`: the resolved room of a space: outline and islands, areas, clear height, volume and the bounding walls, or the reason the seed does not bound a room.", names: ["Point2", "Vertex", "SpaceStatus", "SpaceRoom"] },
  "🧮️quantities": { doc: "🧮️ `quantities`: the quantity take-off per element and the totals per kind, type and material for every storey, building and the project.", names: ["LayerQuantity", "QuantityKind", "ElementQuantity", "Totals", "QuantityTotals", "ModelQuantities"] },
};
const fieldsOfInference: [string, string, boolean][] = [["stair_runs", "StairRun", true], ["spaces", "SpaceRoom", true], ["quantities", "ModelQuantities", false]];
const closure = (names: string[]): string[] => {
  const found = new Set<string>();
  const visit = (name: string) => {
    if (found.has(name)) return;
    found.add(name);
    (messages[name] ?? []).forEach(([, type]) => {
      const base = /[A-Z]\w*/.exec(type)?.[0];
      if (base) visit(base);
    });
  };
  names.forEach(visit);
  return [...found];
};
const wanted = closure(fieldsOfInference.map(([, type]) => type));
const scalar = new Set(["f64", "u32", "bool", "string"]);
const shape = (type: string) => ({ optional: type.startsWith("?"), list: type.replace(/^\?/, "").startsWith("["), map: type.replace(/^\?/, "").startsWith("{"), depth: (type.match(/\[/g) ?? []).length, base: type.replace(/^\?/, "").replace(/[[\]{}]/g, "") });

const json = (type: string): unknown => {
  const { optional, list, map, depth, base } = shape(type);
  const leaf = base === "f64" ? { type: "number" } : base === "u32" ? { type: "integer" } : base === "bool" ? { type: "boolean" } : base === "string" ? { type: "string" } : { $ref: `#/$defs/${base}` };
  void optional;
  if (map) return { type: "object", additionalProperties: leaf };
  return list ? Array.from({ length: depth }).reduce<unknown>((inner) => ({ type: "array", items: inner }), leaf) : leaf;
};
const jsonDef = (name: string): unknown => {
  if (enums[name]) return { enum: enums[name].values };
  const fields = messages[name];
  return { type: "object", additionalProperties: false, required: fields.filter(([, type]) => !type.startsWith("?")).map(([field]) => field), properties: Object.fromEntries(fields.map(([field, type]) => [field, json(type)])) };
};
const indent = (text: string, pad: string) => text.split("\n").map((line, index) => (index === 0 ? line : pad + line)).join("\n");

const tsType = (type: string): string => {
  const { list, map, depth, base } = shape(type);
  const leaf = base === "f64" || base === "u32" ? "number" : base === "bool" ? "boolean" : base === "string" ? "string" : base;
  return map ? `Record<string, ${leaf}>` : list ? leaf + "[]".repeat(depth) : leaf;
};
const tsDef = (name: string): string => {
  if (enums[name]) return `export type ${name} = ${enums[name].values.map((value) => `"${value}"`).join(" | ")};`;
  return `export interface ${name} {\n${messages[name].map(([field, type]) => `  ${field}${type.startsWith("?") ? "?" : ""}: ${tsType(type)};`).join("\n")}\n}`;
};

const gqlRows = new Set<string>();
const gqlType = (type: string): string => {
  const { optional, list, map, depth, base } = shape(type);
  const leaf = base === "f64" ? "Float" : base === "u32" ? "Int" : base === "bool" ? "Boolean" : base === "string" ? "String" : base;
  if (map) {
    gqlRows.add(leaf);
    return `[${leaf}Row!]!`;
  }
  if (list && depth === 2) return "[VertexLoop!]!";
  return list ? `[${leaf}!]!` : optional ? leaf : `${leaf}!`;
};
const gqlDef = (name: string): string => (enums[name] ? `enum ${name} {\n${enums[name].values.map((value) => `  ${value}`).join("\n")}\n}` : `type ${name} {\n${messages[name].map(([field, type]) => `  ${field}: ${gqlType(type)}`).join("\n")}\n}`);

const protoType = (type: string): string => {
  const { optional, list, map, depth, base } = shape(type);
  const leaf = base === "f64" ? "double" : base === "u32" ? "uint32" : base === "bool" ? "bool" : base === "string" ? "string" : base;
  if (map) return `map<string, ${leaf}>`;
  if (list && depth === 2) return "repeated VertexLoop";
  return list ? `repeated ${leaf}` : optional ? `optional ${leaf}` : leaf;
};
const protoDef = (name: string): string => {
  if (enums[name]) return `enum ${name} {\n${enums[name].values.map((value, index) => `  ${enums[name].prefix}_${value.replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()} = ${index};`).join("\n")}\n}`;
  return `message ${name} {\n${messages[name].map(([field, type], index) => `  ${protoType(type)} ${field} = ${index + 1};`).join("\n")}\n}`;
};

function jsonFacet() {
  let text = read("🔣️.json");
  const missing = fieldsOfInference.filter(([field]) => !text.includes(`"${field}": {\n      "type"`) && !text.includes(`"${field}": {\n      "$ref"`));
  if (missing.length) {
    text = text.replace(/("required": \[)/, `$1\n${missing.map(([field]) => `    "${field}",`).join("\n")}`);
    const property = (field: string, type: string, map: boolean) => `\n    "${field}": ${indent(JSON.stringify(map ? { type: "object", additionalProperties: { $ref: `#/$defs/${type}` }, "x-semio-derived": true } : { $ref: `#/$defs/${type}`, "x-semio-derived": true }, null, 2), "    ")},`;
    text = text.replace(/("properties": \{)/, `$1${missing.map(([field, type, map]) => property(field, type, map)).join("")}`);
  }
  const present = (name: string) => new RegExp(`"${name}": \\{`).test(text);
  const defs = wanted.filter((name) => !present(name)).map((name) => `    "${name}": ${indent(JSON.stringify(jsonDef(name), null, 2), "    ")},`).join("\n");
  if (defs) text = text.replace(/("\$defs": \{)/, `$1\n${defs}`);
  JSON.parse(text);
  write("🔣️.json", text);
}

function tsFacet() {
  let text = read("🟦️.ts");
  const has = (name: string) => new RegExp(`export (interface|type) ${name}\\b`).test(text);
  const block = wanted.filter((name) => !has(name)).map(tsDef).join("\n\n");
  const fields = fieldsOfInference.filter(([field]) => !new RegExp(`^  ${field}:`, "m").test(text.slice(text.indexOf("export interface ModelInference"))));
  if (fields.length) text = text.replace(/export interface ModelInference \{/, `${block ? block + "\n\n" : ""}export interface ModelInference {\n${fields.map(([field, type, map]) => `  /** @derived */\n  ${field}: ${map ? `Record<string, ${type}>` : type};`).join("\n")}`);
  else if (block) text = text.replace(/export interface ModelInference \{/, `${block}\n\nexport interface ModelInference {`);
  write("🟦️.ts", text);
}

function graphqlFacet() {
  let text = read("🔗️.graphql");
  const has = (name: string) => new RegExp(`(type|enum) ${name} \\{`).test(text);
  const fields = fieldsOfInference.filter(([field]) => !new RegExp(`^  ${field}:`, "m").test(text.slice(text.indexOf("type ModelInference"))));
  const parts = wanted.filter((name) => !has(name)).map(gqlDef);
  if (wanted.some((name) => messages[name]?.some(([, type]) => type === "[[Vertex]]")) && !has("VertexLoop")) parts.push(gqlDef("VertexLoop"));
  const rowNames = [...fieldsOfInference.filter(([, , map]) => map).map(([, type]) => type), ...gqlRows].filter((name, index, all) => all.indexOf(name) === index);
  const rows = rowNames.filter((name) => !has(`${name}Row`)).map((name) => `type ${name}Row {\n  id: String!\n  value: ${name}!\n}`);
  if (fields.length) text = text.replace(/type ModelInference \{/, `type ModelInference {\n${fields.map(([field, type, map]) => (map ? `  ${field}: [${type}Row!]! @derived` : `  ${field}: ${type}! @derived`)).join("\n")}`);
  const block = [...rows, ...parts].filter((part, index, all) => all.indexOf(part) === index).join("\n\n");
  write("🔗️.graphql", block ? `${text.trimEnd()}\n\n${block}\n` : text);
}

function protoFacet() {
  let text = read("🛰️.proto");
  const has = (name: string) => new RegExp(`(message|enum) ${name} \\{`).test(text);
  const message = /message ModelInference \{([\s\S]*?)\n\}/.exec(text)?.[1] ?? "";
  let next = Math.max(0, ...[...message.matchAll(/= (\d+);/g)].map((match) => Number(match[1]))) + 1;
  const fields = fieldsOfInference.filter(([field]) => !new RegExp(`\\b${field} = \\d+;`).test(message));
  const lines = fields.map(([field, type, map]) => `  // @derived\n  ${map ? `map<string, ${type}>` : type} ${field} = ${next++};`).join("\n");
  const parts = wanted.filter((name) => !has(name)).map(protoDef);
  if (wanted.some((name) => messages[name]?.some(([, type]) => type === "[[Vertex]]")) && !has("VertexLoop")) parts.push(protoDef("VertexLoop"));
  if (lines) text = text.replace(/message ModelInference \{/, `message ModelInference {\n${lines}`);
  write("🛰️.proto", parts.length ? `${text.trimEnd()}\n\n${parts.join("\n\n")}\n` : text);
}

function leafFacets() {
  for (const [dir, { doc, names }] of Object.entries(leaves)) {
    const path = join(root, dir, "🟦️.ts");
    writeFileSync(path, `/** ${doc} */\n\n${closure(names).map(tsDef).join("\n\n")}\n`);
  }
}

if (!existsSync(root)) throw new Error(`missing ${root}`);
jsonFacet();
tsFacet();
graphqlFacet();
protoFacet();
leafFacets();
console.log("stair-runs, spaces and quantities facets spliced");
