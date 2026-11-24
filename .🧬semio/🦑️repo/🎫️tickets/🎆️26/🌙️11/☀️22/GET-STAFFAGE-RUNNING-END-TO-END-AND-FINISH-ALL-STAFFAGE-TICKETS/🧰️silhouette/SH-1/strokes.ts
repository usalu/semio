/** 🧵️ The reference strokes of the slender solids of a person in a view as the guard builds them: per solid the number of quadrilaterals and their bounding box and largest width in millimetres, with and without the drawing they follow; optional ops applied first.
 *  bun strokes.ts <slug|file.json> <scale> <view> [ops.json|-] [id-filter] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, strokeDrawings, strokesOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";

const [name, scaleArg, view, opsArg, filter] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const drawn = strokeDrawings(view as never, solids, drawnFor(asset, state, view as never, scale));
for (const w of solids.filter((x) => x.slender === true && (filter === undefined || x.id.includes(filter)))) {
  const rings = strokesOf(view as never, w, drawn);
  const p = rings.flat();
  const width = Math.max(...rings.map((r) => Math.min(Math.hypot(r[0]!.x - r[1]!.x, r[0]!.y - r[1]!.y), Math.hypot(r[1]!.x - r[2]!.x, r[1]!.y - r[2]!.y))));
  console.log(`${w.id}[${w.bone}] ${rings.length} quads, x ${(Math.min(...p.map((q) => q.x)) * 1000).toFixed(0)} … ${(Math.max(...p.map((q) => q.x)) * 1000).toFixed(0)}, y ${(Math.min(...p.map((q) => q.y)) * 1000).toFixed(0)} … ${(Math.max(...p.map((q) => q.y)) * 1000).toFixed(0)}, short side up to ${(width * 1000).toFixed(1)} mm`);
}
