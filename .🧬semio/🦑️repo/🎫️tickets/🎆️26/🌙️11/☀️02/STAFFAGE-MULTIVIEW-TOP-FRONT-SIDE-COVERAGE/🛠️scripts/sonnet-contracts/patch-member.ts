import { readFileSync, writeFileSync } from "node:fs";
const dir = process.argv[2]!;
const sub = (t: string, from: string, to: string): string => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
  return t.replace(from, to);
};
let json = readFileSync(`${dir}/🔣️.json`, "utf8");
json = sub(
  json,
  '            "z1": {\n              "type": "number"\n            }\n          }\n        }\n      ]\n    },\n    "Shape": {',
  `            "z1": {
              "type": "number"
            }
          }
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind",
            "from",
            "to"
          ],
          "properties": {
            "kind": {
              "const": "member"
            },
            "from": {
              "description": "The start point in the bone-local frame of the shape's own bone: x along the bone (u), y along its normal (v), z lateral (w, left is +), all in metres from the bone origin at rest.",
              "$ref": "#/$defs/BonePoint"
            },
            "to": {
              "description": "The end point in the bone-local frame of \`bone\` (default: the shape's own bone), same axes as \`from\`. A second bone lets one bar span two articulated parts (a brace, a strut, a cable).",
              "type": "object",
              "additionalProperties": false,
              "required": [
                "x",
                "y",
                "z"
              ],
              "properties": {
                "x": {
                  "type": "number"
                },
                "y": {
                  "type": "number"
                },
                "z": {
                  "type": "number"
                },
                "bone": {
                  "$ref": "#/$defs/Slug"
                }
              }
            },
            "diameterM": {
              "description": "A circular section of this diameter in metres; excludes widthM and depthM.",
              "type": "number",
              "exclusiveMinimum": 0
            },
            "widthM": {
              "description": "A rectangular section: the size in metres across the bar in the sagittal plane (along the bone normal v at the start); needs depthM and excludes diameterM.",
              "type": "number",
              "exclusiveMinimum": 0
            },
            "depthM": {
              "description": "A rectangular section: the size in metres across the bar laterally (along w, left is +); needs widthM and excludes diameterM.",
              "type": "number",
              "exclusiveMinimum": 0
            }
          }
        }
      ]
    },
    "BonePoint": {
      "description": "A point in the bone-local frame of a bone, metres from its origin at rest: x along the bone, y along its normal, z lateral (left is +).",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "x",
        "y",
        "z"
      ],
      "properties": {
        "x": {
          "type": "number"
        },
        "y": {
          "type": "number"
        },
        "z": {
          "type": "number"
        }
      }
    },
    "Shape": {`,
);
json = sub(json, "extruded: a laterally prismatic part spanning z0..z1 (left is +); front is z0..z1 by the height range of the profile, plan is the forward range of the profile by z0..z1.", "extruded: a laterally prismatic part spanning z0..z1 (left is +); front is z0..z1 by the height range of the profile, plan is the forward range of the profile by z0..z1. member: a prismatic bar between two bone-local points (from, and to in the frame of an optional second bone) with a circular (diameterM) or rectangular (widthM, depthM) section, carried by the full 3D frames of its bones and drawn in its true foreshortened outline in every viewpoint; the shape geometry itself is not used, a diagonal is one member and never a bounding rectangle.");
writeFileSync(`${dir}/🔣️.json`, json);
let ts = readFileSync(`${dir}/🟦️.ts`, "utf8");
ts = sub(ts, '| { readonly kind: "extruded"; readonly z0: number; readonly z1: number };', '| { readonly kind: "extruded"; readonly z0: number; readonly z1: number }\n  | { readonly kind: "member"; readonly from: BonePoint; readonly to: BonePoint & { readonly bone?: Slug }; readonly diameterM?: number; readonly widthM?: number; readonly depthM?: number };');
ts = sub(ts, "/** 🌀️ How a shape of any viewpoint", "/** 📍️ A point in the bone-local frame of a bone, metres from its origin at rest: `x` along the bone (u), `y` along its normal (v), `z` lateral (w, left is +). */\nexport type BonePoint = { readonly x: number; readonly y: number; readonly z: number };\n\n/** 🌀️ How a shape of any viewpoint");
ts = sub(ts, "the plan the forward range of the profile by `z0`..`z1`. */", "the plan the forward range of the profile by `z0`..`z1`.\n * `member`: a prismatic bar between two bone-local points (`from`, and `to` in the frame of the optional `to.bone`) with a circular (`diameterM`) or rectangular (`widthM` across the bar in the sagittal plane, `depthM` laterally) section; it is drawn in its true foreshortened outline in every viewpoint, so a diagonal is one member and never a bounding rectangle. */");
ts = sub(ts, "  readonly frictionCoefficient?: number;\n};\n//#endregion 📍️Anchors", "  readonly frictionCoefficient?: number;\n};\n//#endregion 📍️Anchors");
writeFileSync(`${dir}/🟦️.ts`, ts);
