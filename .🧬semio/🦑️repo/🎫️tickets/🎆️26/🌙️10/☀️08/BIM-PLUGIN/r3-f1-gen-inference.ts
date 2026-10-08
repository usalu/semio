#!/usr/bin/env bun
/** 💡️ Writes the json/ts/graphql/proto facets of `s.bim.model.inference` and of its two field slugs. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, GQLF, JSONF, PROTOF, schema, TSF } from "./r3-f1-paths.ts";

const dir = child(schema, "inferences");
const levelFields = ["elevation", "top_elevation", "absolute_elevation", "absolute_top_elevation"];
const layoutFields = ["base_z", "top_z", "height", "thickness", "length", "side_area", "footprint_area", "volume"];
const NS = "https://json.schemas.assets.semio-tech.com/s/bim/model/inference.json";
const numbers = (names: string[]) => ({ type: "object", additionalProperties: false, required: names, properties: Object.fromEntries(names.map((n) => [n, { type: "number" }])) });
const doc = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: NS,
  title: "ModelInference",
  type: "object",
  additionalProperties: false,
  required: ["storey_levels", "wall_layout"],
  properties: {
    storey_levels: { type: "object", additionalProperties: { $ref: "#/$defs/StoreyLevel" }, "x-semio-derived": true },
    wall_layout: { type: "object", additionalProperties: { $ref: "#/$defs/WallLayout" }, "x-semio-derived": true },
  },
  $defs: { StoreyLevel: numbers(levelFields), WallLayout: numbers(layoutFields) },
};
const tsFields = (names: string[]) => names.map((n) => `  ${n}: number;`).join("\n");
const ts = `/** 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id. */\n\nexport interface StoreyLevel {\n${tsFields(levelFields)}\n}\n\nexport interface WallLayout {\n${tsFields(layoutFields)}\n}\n\nexport interface ModelInference {\n  /** @derived */\n  storey_levels: Record<string, StoreyLevel>;\n  /** @derived */\n  wall_layout: Record<string, WallLayout>;\n}\n`;
const gqlFields = (names: string[]) => names.map((n) => `  ${n}: Float!`).join("\n");
const graphql = `# 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id.\n\ntype ModelInference {\n  storey_levels: [StoreyLevelRow!]! @derived\n  wall_layout: [WallLayoutRow!]! @derived\n}\n\ntype StoreyLevelRow {\n  id: String!\n  value: StoreyLevel!\n}\n\ntype WallLayoutRow {\n  id: String!\n  value: WallLayout!\n}\n\ntype StoreyLevel {\n${gqlFields(levelFields)}\n}\n\ntype WallLayout {\n${gqlFields(layoutFields)}\n}\n`;
const protoFields = (names: string[]) => names.map((n, i) => `  double ${n} = ${i + 1};`).join("\n");
const proto = `syntax = "proto3";\npackage semio.s.bim.model.inference;\n\n// 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id.\n\nmessage ModelInference {\n  // @derived\n  map<string, StoreyLevel> storey_levels = 1;\n  // @derived\n  map<string, WallLayout> wall_layout = 2;\n}\n\nmessage StoreyLevel {\n${protoFields(levelFields)}\n}\n\nmessage WallLayout {\n${protoFields(layoutFields)}\n}\n`;
writeFileSync(join(dir, JSONF), JSON.stringify(doc, null, 2) + "\n");
writeFileSync(join(dir, TSF), ts);
writeFileSync(join(dir, GQLF), graphql);
writeFileSync(join(dir, PROTOF), proto);
const slugs: [string, string[], string][] = [["storey-levels", levelFields, "StoreyLevel"], ["wall-layout", layoutFields, "WallLayout"]];
for (const [slug, fields, name] of slugs) {
  const slugDir = child(dir, slug);
  writeFileSync(join(slugDir, TSF), `/** ${slug === "storey-levels" ? "🪜️ `storey-levels`: level 0 is the building datum; positive levels stack upward, negative levels downward." : "🧱️ `wall-layout`: the resolved vertical extent and plan quantities of a wall; the footprint is unjoined."} */\n\nexport interface ${name} {\n${tsFields(fields)}\n}\n`);
}
console.log("inference facets written");
