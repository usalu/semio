/** 📋️ Collects the report numbers of the machinery assets: solids, rest and default-pose bounding box error, IoU per view, volume validator findings, poses and clip samples evaluated.
 * Usage: bun authoring-machinery-report.ts ; writes 🗑️generated/authoring-machinery/report.json */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, QUALITY_FOR, STAFFAGE, analyse, loadAsset, resolveAsset, toWorld, type V3 } from "./volume-lib.ts";

const slugs = ["concrete-mixer-truck", "diesel-generator", "evaporative-humidifier", "excavator", "expansion-valve", "finned-heat-exchanger", "forklift", "heat-pump-outdoor-unit", "oil-pumpjack", "plate-heat-exchanger", "scissor-lift", "scroll-compressor", "solar-thermal-collector", "sorption-wheel", "tower-crane", "wheel-loader"];
const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema", "🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
const rows: any[] = [];
for (const slug of slugs) {
  const asset = loadAsset(resolveAsset(slug).path);
  const d = asset.dimensions;
  const dims: V3 = [d.widthM, d.heightM, d.depthM];
  const a = analyse(asset, asset.volume, 50);
  const pose = asset.presentation.defaultPose;
  const st: any = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } } as any);
  const q = QUALITY_FOR(Math.max(...dims));
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const s of asset.volume.solids) {
    const b = st.bones[s.bone];
    const f = { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z] as V3, u: [b.frame.u.x, b.frame.u.y, b.frame.u.z] as V3, v: [b.frame.v.x, b.frame.v.y, b.frame.v.z] as V3, w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] as V3 };
    const g = tessellateSolid({ ...s, node: s.bone } as never, q);
    for (let i = 0; i < g.positions.length; i += 3) { const p = toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]); for (let k = 0; k < 3; k++) { if (p[k]! < min[k]!) min[k] = p[k]!; if (p[k]! > max[k]!) max[k] = p[k]!; } }
  }
  const poseErr = [0, 1, 2].map((k) => (max[k]! - min[k]!) / dims[k]! - 1);
  check(asset);
  const volAjv = (check.errors ?? []).filter((e) => e.instancePath.startsWith("/volume")).length;
  const issues = validateAsset(asset) as readonly { path: string; code: string }[];
  const volIssues = issues.filter((i) => i.code.startsWith("volume-") || i.path.startsWith("volume")).length;
  rows.push({ slug, solids: asset.volume.solids.length, restErr: a.bboxRelativeError, poseErr, defaultPose: pose, iou: [a.views.elevation.iou, a.views.front.iou, a.views.plan.iou], volAjv, volIssues, poses: asset.poses.length, clips: asset.clips.length });
  console.log(slug, asset.volume.solids.length, a.bboxRelativeError.map((e) => (e * 100).toFixed(1)).join("/"), poseErr.map((e) => (e * 100).toFixed(1)).join("/"), [a.views.elevation.iou, a.views.front.iou, a.views.plan.iou].map((e) => e.toFixed(3)).join("/"), "ajvVolume", volAjv, "issues", volIssues);
}
writeFileSync(join(GENERATED, "authoring-machinery", "report.json"), JSON.stringify(rows, null, 1));
