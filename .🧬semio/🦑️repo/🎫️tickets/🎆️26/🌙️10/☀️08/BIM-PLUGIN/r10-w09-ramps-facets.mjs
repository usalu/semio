#!/usr/bin/env bun
/**
 * 🛝️ Facets of the ramp inference (`ramp_runs`) in the four languages of `s.bim.model.inference`: the `ramp_runs` field of `ModelInference`, the types of `🛝️ramp-runs`
 * (`RampRun`, `RampFlight`, `RampLanding`, `RampCompliance`) and the members of the enums the ramps extend (`SolidFamily`, `QuantityKind`, `PlanKind`, `DiagnosticCode`), read from the Rust sources.
 * Idempotent: `bun r10-w09-ramps-facets.mjs`.
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const em = (...points) => String.fromCodePoint(...points) + "️";
const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix)));
let at = sub(sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))), "plugins"), "bim");
at = sub(sub(at, "artifacts"), "model");
const standards = sub(sub(at, "standards"), "1");
const subsets = sub(standards, "subsets");
const any = join(subsets, readdirSync(subsets).find((n) => n.endsWith("any")));
const I = sub(sub(any, "schema"), "inferences");
const file = (name) => join(I, name);
const read = (path) => readFileSync(path, "utf8");

const rust = {
  SolidFamily: read(join(sub(I, "element-solids"), em(0x1f980) + ".rs")),
  QuantityKind: read(join(sub(I, "quantities"), em(0x1f980) + ".rs")),
  PlanKind: read(join(sub(I, "plan-linework"), em(0x1f980) + ".rs")),
};
const members = (source, name) => {
  const match = source.match(new RegExp(`pub enum ${name} \\{([\\s\\S]*?)\\n\\}`));
  return match[1].split("\n").map((line) => line.trim().replace(/,$/, "")).filter((line) => /^[A-Z][A-Za-z0-9]*$/.test(line));
};
const enums = Object.fromEntries(Object.entries(rust).map(([name, source]) => [name, members(source, name)]));
const snake = (text) => text.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

const fields = {
  RampRun: [["base_z", "number"], ["top_z", "number"], ["rise", "number"], ["length", "number"], ["run_length", "number"], ["slope", "number"], ["angle", "number"], ["width", "number"], ["flights", "RampFlight[]"], ["landings", "RampLanding[]"], ["compliance", "RampCompliance"]],
  RampFlight: [["from", "number"], ["to", "number"], ["length", "number"], ["z_from", "number"], ["z_to", "number"]],
  RampLanding: [["from", "number"], ["to", "number"], ["length", "number"], ["z", "number"]],
  RampCompliance: [["run_ok", "boolean"], ["slope_ok", "boolean"], ["compliant", "boolean"]],
};

const edit = (path, change) => {
  const before = read(path);
  const after = change(before);
  if (after !== before) writeFileSync(path, after);
  console.log(path.slice(-24), before === after ? "unchanged" : "updated");
};

//#region Sub facets
const ownTs = { SolidFamily: "element-solids", QuantityKind: "quantities", PlanKind: "plan-linework" };
for (const [name, dir] of Object.entries(ownTs))
  edit(join(sub(I, dir), em(0x1f7e6) + ".ts"), (text) => text.replace(new RegExp(`export type ${name} = [^;]*;`), `export type ${name} = ${enums[name].map((item) => `"${item}"`).join(" | ")};`));
const rampDir = join(I, em(0x1f6dd) + "ramp-runs");
const interfaceText = ([name, rows]) => ["export interface " + name + " {", ...rows.map(([field, type]) => "  " + field + ": " + type + ";"), "}", ""].join("\n");
writeFileSync(join(rampDir, em(0x1f7e6) + ".ts"), ["/** 🛝️ ramp-runs: the resolved run of every ramp: rise, length, sloped run, slope, flights, landings and the code flags. */", "", ...Object.entries(fields).map(interfaceText)].join("\n"));
//#endregion

//#region JSON
edit(file(em(0x1f523) + ".json"), (text) => {
  const doc = JSON.parse(text);
  const jsonTy = (type) => {
    if (type.endsWith("[]")) return { type: "array", items: jsonTy(type.slice(0, -2)) };
    if (type === "number" || type === "boolean") return { type };
    return { $ref: `#/$defs/${type}` };
  };
  const properties = {};
  for (const [key, value] of Object.entries(doc.properties)) {
    properties[key] = value;
    if (key === "stair_runs") properties.ramp_runs = { type: "object", additionalProperties: { $ref: "#/$defs/RampRun" }, "x-semio-derived": true };
  }
  doc.properties = properties;
  if (!doc.required.includes("ramp_runs")) doc.required.splice(doc.required.indexOf("stair_runs") + 1, 0, "ramp_runs");
  const defs = {};
  for (const [key, value] of Object.entries(doc.$defs)) {
    defs[key] = value;
    if (key === "StairCompliance")
      for (const [name, rows] of Object.entries(fields)) defs[name] = { type: "object", additionalProperties: false, required: rows.map((row) => row[0]), properties: Object.fromEntries(rows.map(([field, type]) => [field, jsonTy(type)])) };
  }
  for (const [name, list] of Object.entries(enums)) if (defs[name]) defs[name] = { ...defs[name], enum: list };
  doc.$defs = defs;
  return JSON.stringify(doc, null, 2) + "\n";
});
//#endregion

//#region TypeScript
edit(file(em(0x1f7e6) + ".ts"), (text) => {
  let out = text;
  if (!out.includes("export interface RampRun")) {
    const interfaces = Object.entries(fields).map(([name, rows]) => `export interface ${name} {\n${rows.map(([field, type]) => `  ${field}: ${type};`).join("\n")}\n}\n\n`).join("");
    out = out.replace("export interface SpaceRoom {", `${interfaces}export interface SpaceRoom {`);
  }
  if (!out.includes("ramp_runs:")) out = out.replace("  stair_runs: Record<string, StairRun>;\n", "  stair_runs: Record<string, StairRun>;\n  /** @derived */\n  ramp_runs: Record<string, RampRun>;\n");
  for (const [name, list] of Object.entries(enums)) out = out.replace(new RegExp(`export type ${name} = [^;]*;`), `export type ${name} = ${list.map((item) => `"${item}"`).join(" | ")};`);
  return out;
});
//#endregion

//#region GraphQL
edit(file(em(0x1f517) + ".graphql"), (text) => {
  let out = text;
  const gql = (type) => (type.endsWith("[]") ? `[${gql(type.slice(0, -2))}!]!` : type === "number" ? "Float!" : type === "boolean" ? "Boolean!" : `${type}!`);
  if (!out.includes("type RampRun {")) {
    const types = Object.entries(fields).map(([name, rows]) => `type ${name} {\n${rows.map(([field, type]) => `  ${field}: ${gql(type)}`).join("\n")}\n}\n\n`).join("");
    out = out.replace("type SpaceRoomRow {", `type RampRunRow {\n  id: String!\n  value: RampRun!\n}\n\n${types}type SpaceRoomRow {`);
  }
  if (!out.includes("ramp_runs:")) out = out.replace("  stair_runs: [StairRunRow!]! @derived\n", "  stair_runs: [StairRunRow!]! @derived\n  ramp_runs: [RampRunRow!]! @derived\n");
  for (const name of Object.keys(enums)) out = out.replace(new RegExp(`enum ${name} \\{[^}]*\\}`), `enum ${name} {\n${enums[name].map((item) => `  ${item}`).join("\n")}\n}`);
  return out;
});
//#endregion

//#region Proto
edit(file(em(0x1f6f0) + ".proto"), (text) => {
  let out = text;
  const proto = (type) => (type.endsWith("[]") ? `repeated ${proto(type.slice(0, -2))}` : type === "number" ? "double" : type === "boolean" ? "bool" : type);
  if (!out.includes("message RampRun {")) {
    const messages = Object.entries(fields).map(([name, rows]) => `message ${name} {\n${rows.map(([field, type], index) => `  ${proto(type)} ${field} = ${index + 1};`).join("\n")}\n}\n\n`).join("");
    out = out.replace("message SpaceRoom {", `${messages}message SpaceRoom {`);
  }
  if (!out.includes("ramp_runs =")) {
    const body = out.match(/message ModelInference \{([\s\S]*?)\n\}/)[1];
    const next = Math.max(...[...body.matchAll(/= (\d+);/g)].map((m) => Number(m[1]))) + 1;
    out = out.replace("  map<string, StairRun> stair_runs = 5;\n", `  map<string, StairRun> stair_runs = 5;\n  // @derived\n  map<string, RampRun> ramp_runs = ${next};\n`);
  }
  for (const name of Object.keys(enums)) {
    const prefix = snake(name);
    out = out.replace(new RegExp(`enum ${name} \\{[^}]*\\}`), `enum ${name} {\n${enums[name].map((item, index) => `  ${prefix}_${snake(item)} = ${index};`).join("\n")}\n}`);
  }
  return out;
});
//#endregion
