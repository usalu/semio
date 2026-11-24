/** 📦️ The world bounding box (x, y, z in metres, default pose) of every copy of the named solids of one scale, and the plan, elevation and front extents they project to: bun boxes.ts <id|file.json> <scale> <solid-id-substring>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

const [name, scaleArg, ...wanted] = process.argv.slice(2);
const scale = Number(scaleArg);
const { asset } = await Bun.file(fileOf(name!)).json();
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
for (const w of worldSolids(asset, state.bones as never, scale)) {
  if (wanted.length > 0 && !wanted.some((x) => w.id.includes(x))) continue;
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  const views = Object.fromEntries(VIEWPOINTS.map((v) => [v, [Infinity, Infinity, -Infinity, -Infinity]]));
  for (let i = 0; i < w.positions.length; i += 3) {
    for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, w.positions[i + k]!); hi[k] = Math.max(hi[k]!, w.positions[i + k]!); }
    for (const v of VIEWPOINTS) { const p = project(v, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); const b = views[v]!; b[0] = Math.min(b[0]!, p.x); b[1] = Math.min(b[1]!, p.y); b[2] = Math.max(b[2]!, p.x); b[3] = Math.max(b[3]!, p.y); }
  }
  const f = (a: number[]): string => a.map((x) => x.toFixed(4)).join(" ");
  console.log(`${w.id}[${w.bone}${w.slender ? ",slender" : ""}${w.texture ? ",texture" : ""}] world ${f(lo)} .. ${f(hi)} | ${VIEWPOINTS.map((v) => `${v} ${f(views[v]!)}`).join(" | ")}`);
}
