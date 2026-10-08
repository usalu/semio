#!/usr/bin/env bun
/** 🪟️ Idempotently splices the `opening-frames` field into the json/ts/graphql/proto facets of `s.bim.model.inference` (other agents edit the same files, so every write is a single textual insertion re-read just before). */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "C:/git/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences";
const read = (name: string) => readFileSync(join(root, name), "utf8");
const write = (name: string, text: string) => writeFileSync(join(root, name), text);

const issues = ["HostMissing", "TypeMissing", "NonPositiveSize", "OutsideHostExtent", "BelowHostBase", "AboveHostTop", "OverlapsSibling"];
const roles = ["Leaf", "Swing", "Glazing"];
const swings = ["Left", "Right"];
const num = { type: "number" };
const ref = (name: string) => ({ $ref: `#/$defs/${name}` });
const object = (properties: Record<string, unknown>, required = Object.keys(properties)) => ({ type: "object", additionalProperties: false, required, properties });
const jsonDefs: Record<string, unknown> = {
  Point2: object({ x: num, y: num }),
  Swing: { enum: swings },
  OpeningIssue: { enum: issues },
  PlanRole: { enum: roles },
  Vec3: object({ x: num, y: num, z: num }),
  Frame: object({ origin: ref("Vec3"), x_axis: ref("Vec3"), y_axis: ref("Vec3"), z_axis: ref("Vec3") }),
  OpeningCut: object({ s_min: num, s_max: num, z_min: num, z_max: num }),
  PlanShape: {
    oneOf: [
      object({ Line: object({ from: ref("Point2"), to: ref("Point2") }) }),
      object({ Arc: object({ centre: ref("Point2"), radius: num, start_angle: num, sweep: num }) }),
    ],
  },
  PlanStroke: object({ role: ref("PlanRole"), shape: ref("PlanShape") }),
  OpeningFrame: object(
    {
      width: num, height: num, sill: num, offset: num, cut: ref("OpeningCut"), reveal_depth: num, face_front: num, face_back: num, host_length: num, host_height: num,
      point: ref("Point2"), local: ref("Frame"), world: ref("Frame"), hand: ref("Swing"), plan: { type: "array", items: ref("PlanStroke") },
      issues: { type: "array", items: ref("OpeningIssue") }, overlaps: { type: "array", items: { type: "string" } }, valid: { type: "boolean" },
    },
    ["width", "height", "sill", "offset", "cut", "reveal_depth", "face_front", "face_back", "host_length", "host_height", "point", "local", "world", "plan", "issues", "overlaps", "valid"],
  ),
};

const indent = (text: string, pad: string) => text.split("\n").map((line, index) => (index === 0 ? line : pad + line)).join("\n");

function json() {
  let text = read("🔣️.json");
  if (text.includes('"opening_frames"')) return;
  text = text.replace(/("required": \[)/, `$1\n    "opening_frames",`);
  const property = `\n    "opening_frames": {\n      "type": "object",\n      "additionalProperties": {\n        "$ref": "#/$defs/OpeningFrame"\n      },\n      "x-semio-derived": true\n    },`;
  text = text.replace(/("properties": \{)/, `$1${property}`);
  const present = (name: string) => new RegExp(`"${name}": \\{`).test(text);
  const defs = Object.entries(jsonDefs).filter(([name]) => !present(name)).map(([name, schema]) => `    "${name}": ${indent(JSON.stringify(schema, null, 2), "    ")},`).join("\n");
  text = text.replace(/("\$defs": \{)/, `$1\n${defs}`);
  JSON.parse(text);
  write("🔣️.json", text);
}

function typescript() {
  let text = read("🟦️.ts");
  if (text.includes("opening_frames")) return;
  const has = (name: string) => new RegExp(`export (interface|type) ${name}\\b`).test(text);
  const parts: [string, string][] = [
    ["Point2", "export interface Point2 {\n  x: number;\n  y: number;\n}"],
    ["Swing", `export type Swing = ${swings.map((name) => `"${name}"`).join(" | ")};`],
    ["OpeningIssue", `export type OpeningIssue = ${issues.map((name) => `"${name}"`).join(" | ")};`],
    ["PlanRole", `export type PlanRole = ${roles.map((name) => `"${name}"`).join(" | ")};`],
    ["Vec3", "export interface Vec3 {\n  x: number;\n  y: number;\n  z: number;\n}"],
    ["Frame", "export interface Frame {\n  origin: Vec3;\n  x_axis: Vec3;\n  y_axis: Vec3;\n  z_axis: Vec3;\n}"],
    ["OpeningCut", "export interface OpeningCut {\n  s_min: number;\n  s_max: number;\n  z_min: number;\n  z_max: number;\n}"],
    ["PlanShape", "export type PlanShape =\n  | { Line: { from: Point2; to: Point2 } }\n  | { Arc: { centre: Point2; radius: number; start_angle: number; sweep: number } };"],
    ["PlanStroke", "export interface PlanStroke {\n  role: PlanRole;\n  shape: PlanShape;\n}"],
    ["OpeningFrame", "export interface OpeningFrame {\n  width: number;\n  height: number;\n  sill: number;\n  offset: number;\n  cut: OpeningCut;\n  reveal_depth: number;\n  face_front: number;\n  face_back: number;\n  host_length: number;\n  host_height: number;\n  point: Point2;\n  local: Frame;\n  world: Frame;\n  hand?: Swing;\n  plan: PlanStroke[];\n  issues: OpeningIssue[];\n  overlaps: string[];\n  valid: boolean;\n}"],
  ];
  const block = parts.filter(([name]) => !has(name)).map(([, source]) => source).join("\n\n");
  text = text.replace(/export interface ModelInference \{/, `${block}\n\nexport interface ModelInference {\n  /** @derived */\n  opening_frames: Record<string, OpeningFrame>;`);
  write("🟦️.ts", text);
}

function graphql() {
  let text = read("🔗️.graphql");
  if (text.includes("opening_frames")) return;
  const has = (name: string) => new RegExp(`(type|enum) ${name} \\{`).test(text);
  const parts: [string, string][] = [
    ["OpeningFrameRow", "type OpeningFrameRow {\n  id: String!\n  value: OpeningFrame!\n}"],
    ["Point2", "type Point2 {\n  x: Float!\n  y: Float!\n}"],
    ["Swing", `enum Swing {\n${swings.map((name) => `  ${name}`).join("\n")}\n}`],
    ["OpeningIssue", `enum OpeningIssue {\n${issues.map((name) => `  ${name}`).join("\n")}\n}`],
    ["PlanRole", `enum PlanRole {\n${roles.map((name) => `  ${name}`).join("\n")}\n}`],
    ["Vec3", "type Vec3 {\n  x: Float!\n  y: Float!\n  z: Float!\n}"],
    ["Frame", "type Frame {\n  origin: Vec3!\n  x_axis: Vec3!\n  y_axis: Vec3!\n  z_axis: Vec3!\n}"],
    ["OpeningCut", "type OpeningCut {\n  s_min: Float!\n  s_max: Float!\n  z_min: Float!\n  z_max: Float!\n}"],
    ["PlanLine", "type PlanLine {\n  from: Point2!\n  to: Point2!\n}"],
    ["PlanArc", "type PlanArc {\n  centre: Point2!\n  radius: Float!\n  start_angle: Float!\n  sweep: Float!\n}"],
    ["PlanShape", "type PlanShape {\n  Line: PlanLine\n  Arc: PlanArc\n}"],
    ["PlanStroke", "type PlanStroke {\n  role: PlanRole!\n  shape: PlanShape!\n}"],
    ["OpeningFrame", "type OpeningFrame {\n  width: Float!\n  height: Float!\n  sill: Float!\n  offset: Float!\n  cut: OpeningCut!\n  reveal_depth: Float!\n  face_front: Float!\n  face_back: Float!\n  host_length: Float!\n  host_height: Float!\n  point: Point2!\n  local: Frame!\n  world: Frame!\n  hand: Swing\n  plan: [PlanStroke!]!\n  issues: [OpeningIssue!]!\n  overlaps: [String!]!\n  valid: Boolean!\n}"],
  ];
  text = text.replace(/type ModelInference \{/, "type ModelInference {\n  opening_frames: [OpeningFrameRow!]! @derived");
  const block = parts.filter(([name]) => !has(name)).map(([, source]) => source).join("\n\n");
  write("🔗️.graphql", `${text.trimEnd()}\n\n${block}\n`);
}

function proto() {
  let text = read("🛰️.proto");
  if (text.includes("opening_frames")) return;
  const message = /message ModelInference \{([\s\S]*?)\n\}/.exec(text)?.[1] ?? "";
  const next = Math.max(0, ...[...message.matchAll(/= (\d+);/g)].map((match) => Number(match[1]))) + 1;
  const has = (name: string) => new RegExp(`(message|enum) ${name} \\{`).test(text);
  const enumValues = (prefix: string, names: string[]) => names.map((name, index) => `  ${prefix}_${name.replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()} = ${index};`).join("\n");
  const parts: [string, string][] = [
    ["Point2", "message Point2 {\n  double x = 1;\n  double y = 2;\n}"],
    ["Swing", `enum Swing {\n${enumValues("SWING", swings)}\n}`],
    ["OpeningIssue", `enum OpeningIssue {\n${enumValues("OPENING_ISSUE", issues)}\n}`],
    ["PlanRole", `enum PlanRole {\n${enumValues("PLAN_ROLE", roles)}\n}`],
    ["Vec3", "message Vec3 {\n  double x = 1;\n  double y = 2;\n  double z = 3;\n}"],
    ["Frame", "message Frame {\n  Vec3 origin = 1;\n  Vec3 x_axis = 2;\n  Vec3 y_axis = 3;\n  Vec3 z_axis = 4;\n}"],
    ["OpeningCut", "message OpeningCut {\n  double s_min = 1;\n  double s_max = 2;\n  double z_min = 3;\n  double z_max = 4;\n}"],
    ["PlanLine", "message PlanLine {\n  Point2 from = 1;\n  Point2 to = 2;\n}"],
    ["PlanArc", "message PlanArc {\n  Point2 centre = 1;\n  double radius = 2;\n  double start_angle = 3;\n  double sweep = 4;\n}"],
    ["PlanShape", "message PlanShape {\n  oneof shape {\n    PlanLine line = 1;\n    PlanArc arc = 2;\n  }\n}"],
    ["PlanStroke", "message PlanStroke {\n  PlanRole role = 1;\n  PlanShape shape = 2;\n}"],
    ["OpeningFrame", "message OpeningFrame {\n  double width = 1;\n  double height = 2;\n  double sill = 3;\n  double offset = 4;\n  OpeningCut cut = 5;\n  double reveal_depth = 6;\n  double face_front = 7;\n  double face_back = 8;\n  double host_length = 9;\n  double host_height = 10;\n  Point2 point = 11;\n  Frame local = 12;\n  Frame world = 13;\n  optional Swing hand = 14;\n  repeated PlanStroke plan = 15;\n  repeated OpeningIssue issues = 16;\n  repeated string overlaps = 17;\n  bool valid = 18;\n}"],
  ];
  text = text.replace(/message ModelInference \{/, `message ModelInference {\n  // @derived\n  map<string, OpeningFrame> opening_frames = ${next};`);
  const block = parts.filter(([name]) => !has(name)).map(([, source]) => source).join("\n\n");
  write("🛰️.proto", `${text.trimEnd()}\n\n${block}\n`);
}

function slug() {
  const path = join(root, "🪟️opening-frames", "🟦️.ts");
  const doc = "/** 🪟️ `opening-frames`: the resolved placement of every window, door and void in its host: size and sill, frames, cut rectangle in the host's (s, z) development, reveal depth, plan strokes and validity. */\n\n";
  const source = read("🟦️.ts");
  const names = ["Vec3", "Frame", "OpeningCut", "OpeningIssue", "PlanRole", "PlanShape", "PlanStroke", "Swing", "OpeningFrame"];
  const pieces = names.map((name) => new RegExp(`export (?:interface|type) ${name}\\b[\\s\\S]*?(?=\\nexport |$)`).exec(source)?.[0].trimEnd()).filter((piece): piece is string => Boolean(piece));
  const point = "export interface Point2 {\n  x: number;\n  y: number;\n}";
  writeFileSync(path, `${doc}${point}\n\n${pieces.join("\n\n")}\n`);
}

if (!existsSync(root)) throw new Error(`missing ${root}`);
json();
typescript();
graphql();
proto();
slug();
console.log("opening-frames facets spliced");
