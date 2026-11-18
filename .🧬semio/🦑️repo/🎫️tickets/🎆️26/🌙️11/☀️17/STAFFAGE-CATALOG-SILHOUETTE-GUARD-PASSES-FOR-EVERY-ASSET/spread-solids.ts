/** 🧊 Projected extents of the volume solids (default pose or a pose/clip) per view. [REV=<rev>] bun spread-solids.ts <asset> <scale> <view> [pose:ID|clip:ID:T] [filter] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { loadAsset } from "./humansa-lib.ts";
import { execFileSync } from "node:child_process";
import { relative } from "node:path";
import { assetPathsFromFiles } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [name, scale, view, q = "pose:", filter = ""] = process.argv.slice(2);
const rev = process.env.REV;
const a = rev === undefined ? await loadAsset(name) : JSON.parse(execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "show", `${rev}:${relative("/Users/ueli/Documents/semio", (await assetPathsFromFiles()).find((x) => x.folder === name)!.path)}`], { maxBuffer: 1 << 28 }).toString("utf8")).asset;
const p = q.split(":");
const query: any = p[0] === "pose" ? { kind: "pose", pose: p[1] || a.presentation.defaultPose } : { kind: "clip", clip: p[1], timeS: Number(p[2]) };
const st = evaluate(a, query, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
for (const w of worldSolids(a, st.bones as never, Number(scale)).filter((x: any) => x.texture !== true)) {
  if (!w.id.includes(filter)) continue;
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) { const t = project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); x0 = Math.min(x0, t.x); x1 = Math.max(x1, t.x); y0 = Math.min(y0, t.y); y1 = Math.max(y1, t.y); }
  console.log(String(w.id).padEnd(30), `x[${x0.toFixed(3)},${x1.toFixed(3)}] y[${y0.toFixed(3)},${y1.toFixed(3)}]`);
}
