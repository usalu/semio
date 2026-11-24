/** 🎭️ The vertical extent of the elevation cast of a vehicle as `🧍️plan-projection` reads it (`exportCast` of the fixture manifest, `boundsM.maxY - minY` of one cast), and the drawn shapes of the side elevation that reach highest and lowest, optional ops applied to a copy first.
 *  bun cast.ts <cast id> <asset id|file.json> [scale] */
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./people.ts";
const [cast, name, scaleArg] = process.argv.slice(2) as string[];
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const vectors = await Bun.file(`${S}/🧫️fixtures/🧍️plan-projection/🔣️.json`).json();
const root = mkdtempSync(join(tmpdir(), "r3v-cast-"));
const indexes = await exportCast(vectors.casts.manifest, [cast!], root);
rmSync(root, { recursive: true, force: true });
const b = indexes.find((i: any) => i.cast === cast)!.boundsM;
console.log(`${cast}: boundsM y ${b.minY} .. ${b.maxY}, extent ${(b.maxY - b.minY).toFixed(6)} m (bound ${JSON.stringify(vectors.casts.depthM[cast!])})`);
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: vectors.casts.manifest.casts[cast!].pose }, { kind: "scale", scale: { denominator: Number(scaleArg ?? 50) } });
const rows = state.shapes.elevation.filter((s: any) => s.shape.role !== "ground").map((s: any) => { const ys = s.rings.flat().map((p: any) => p.y); return { id: s.shape.id, lo: Math.min(...ys), hi: Math.max(...ys), pen: s.shape.penMm }; });
console.log("highest: " + [...rows].sort((x, y) => y.hi - x.hi).slice(0, 4).map((r) => `${r.id} ${r.hi.toFixed(5)}`).join("  "));
console.log("lowest: " + [...rows].sort((x, y) => x.lo - y.lo).slice(0, 4).map((r) => `${r.id} ${r.lo.toFixed(5)}`).join("  "));
