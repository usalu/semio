/** 🔬 Round 3, furniture/cafe-chair: prints, per slender body solid of a scale and view, the width the product silhouette draws it with and the 2D shape it follows (diagnosis of the slender stroke rule). Usage: bun r3-cafe-chair-probe.ts <asset document json> <scale> <view> */
import { readFileSync } from "node:fs";
const M = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const sil = await import(`${M}/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { evaluate } = await import(`${M}/🧮️evaluation/🟦️.ts`);
const { centreLineOf, clusterPoints } = await import(`${M}/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts`);
const [file, scaleArg, view] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const scale = Number(scaleArg);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = sil.worldSolids(asset, state.bones, scale);
const drawn = sil.drawnFor(asset, state, view, scale);
console.log("[DEBUG] drawn", drawn.map((d: any) => `${d.bone}:${d.closed ? "c" : "o"}${d.filled ? "f" : ""}:${d.penM.toFixed(3)}`).join(" "));
for (const w of solids.filter((s: any) => s.slender)) {
  const per = w.copyVertices ?? w.positions.length / 3;
  const line = centreLineOf(w.positions, w.positions.subarray(0, per * 3), w.indices);
  const centre = line ? clusterPoints(line, w.positions, 0).map((q: number[]) => sil.project(view, q[0], q[1], q[2])) : [];
  console.log("[DEBUG]", w.id, w.bone, "penM", w.penM, "line", line ? `${line.clusters.length} clusters, thick ${line.thickness.toFixed(4)}, len ${line.length.toFixed(3)}` : "none", "followed", sil.followedWidth(w.bone, centre, drawn));
}
