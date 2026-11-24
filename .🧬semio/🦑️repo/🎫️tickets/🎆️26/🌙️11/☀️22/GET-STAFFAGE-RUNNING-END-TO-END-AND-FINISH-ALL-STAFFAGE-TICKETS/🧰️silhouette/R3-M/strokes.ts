/** 🧵️ The strokes the guard compares slender solids as in one view: per solid the width it gets (followed from the drawing or true) and the box of its quads, optional ops applied first: bun strokes.ts <id|file.json> <scale> <view> <id-substring> [ops.json|-] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, strokeDrawings, strokesOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./files.ts";
const [name, sc, view, sub, opsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const solids = (worldSolids(asset, state.bones as never, Number(sc)) as any[]).filter((w) => w.texture !== true);
const drawn = strokeDrawings(view as never, solids, drawnFor(asset, state, view as never, Number(sc)));
for (const w of solids) if (w.slender === true && w.id.includes(sub!)) {
  const quads = strokesOf(view as never, w, drawn) as { x: number; y: number }[][];
  const widths = quads.map((q) => Math.min(Math.hypot(q[0]!.x - q[1]!.x, q[0]!.y - q[1]!.y), Math.hypot(q[1]!.x - q[2]!.x, q[1]!.y - q[2]!.y)));
  const all = quads.flat();
  console.log(`${w.id}[${w.bone}] ${quads.length} quads, width ${(Math.min(...widths) * 1000).toFixed(1)}..${(Math.max(...widths) * 1000).toFixed(1)} mm, x ${Math.min(...all.map((p) => p.x)).toFixed(4)} ${Math.max(...all.map((p) => p.x)).toFixed(4)} y ${Math.min(...all.map((p) => p.y)).toFixed(4)} ${Math.max(...all.map((p) => p.y)).toFixed(4)}`);
}
if (process.env.QUADS !== undefined) for (const w of solids) if (w.slender === true && w.id.includes(sub!)) for (const q of strokesOf(view as never, w, drawn) as { x: number; y: number }[][]) console.log(`   ${w.id} centre (${((q[0]!.x + q[1]!.x + q[2]!.x + q[3]!.x) / 4).toFixed(4)}, ${((q[0]!.y + q[1]!.y + q[2]!.y + q[3]!.y) / 4).toFixed(4)}) ${q.map((p) => `(${p.x.toFixed(4)},${p.y.toFixed(4)})`).join(" ")}`);
