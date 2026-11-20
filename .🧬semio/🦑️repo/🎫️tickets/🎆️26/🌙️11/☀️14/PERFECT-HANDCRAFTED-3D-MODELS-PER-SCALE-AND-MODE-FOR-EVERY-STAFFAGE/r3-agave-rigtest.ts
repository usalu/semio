/** 🧪 Round 3, vegetation/agave: applies the rig fix to a copy of the catalog asset and prints the report and the frame check of every pose. Usage: bun r3-agave-rigtest.ts */
import { readFileSync } from "node:fs";
import { fixRig, rigCheck } from "./r3-agave-rig.ts";
const asset = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/agave/🔣️.json", "utf8")).asset;
const { report } = fixRig(asset);
for (const r of report) console.log("[DEBUG]", r.slice(0, 1500));
const errs = rigCheck(asset, "summer-turgid").map((l) => Number(l.split("error ")[1]));
console.log("[DEBUG] max frame error", Math.max(...errs));
console.log("[DEBUG] joints", asset.rig.bones.filter((b: any) => b.id.endsWith("-a")).map((b: any) => b.joint.maxDeg).join(","));
console.log("[DEBUG] drought", JSON.stringify(asset.poses.find((p: any) => p.id === "drought-infolded").bones["leaf-01-b"]), JSON.stringify(asset.poses.find((p: any) => p.id === "frost-sag").bones["leaf-05-b"]));
