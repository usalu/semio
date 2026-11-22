/** 📐️ World extents of the drawn shapes and of the volume solids of an asset (catalog slug) per scale and view in a pose; names the shapes at the extremes.
 *  bun extents.ts <slug> <pose> [scale ...] */
import { join, resolve } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [slug, pose, ...scales] = process.argv.slice(2);
const asset = (await Bun.file(join(catalog, slug!, "🔣️.json")).json()).asset;
const f = (n: number): string => n.toFixed(3);
for (const sc of scales.length ? scales : ["500", "200", "100", "50"]) {
  const scale = Number(sc);
  const state = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: scale } }) as any;
  for (const view of ["elevation", "front", "plan"] as const) {
    const items: { id: string; x0: number; x1: number; y0: number; y1: number }[] = [];
    for (const s of state.shapes[view]) {
      const pts = s.rings.flat();
      if (!pts.length) continue;
      items.push({ id: `D:${s.shape.id}`, x0: Math.min(...pts.map((p: any) => p.x)), x1: Math.max(...pts.map((p: any) => p.x)), y0: Math.min(...pts.map((p: any) => p.y)), y1: Math.max(...pts.map((p: any) => p.y)) });
    }
    const vols: typeof items = [];
    for (const w of worldSolids(asset, state.bones as never, scale)) {
      const xs: number[] = [], ys: number[] = [];
      for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as Viewpoint, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); xs.push(p.x); ys.push(p.y); }
      vols.push({ id: `V:${w.id}`, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) });
    }
    for (const [name, list] of [["drawn", items], ["volume", vols]] as const) {
      const by = (k: "x0" | "x1" | "y0" | "y1", sign: number) => [...list].sort((a, b) => sign * (a[k] - b[k]))[0]!;
      const a = by("x0", 1), b = by("x1", -1), c = by("y0", 1), d = by("y1", -1);
      console.log(`${sc} ${view.padEnd(9)} ${name.padEnd(6)} x ${f(a.x0)} (${a.id}) .. ${f(b.x1)} (${b.id}) = ${f(b.x1 - a.x0)} | y ${f(c.y0)} (${c.id}) .. ${f(d.y1)} (${d.id}) = ${f(d.y1 - c.y0)}`);
    }
  }
}
