/** 🛏️ What the bodies of a plant show beside its drawn outlines in the default pose: `bun rest.ts <id|file.json> [--prefix mvb-]` evaluates the default pose at every authored scale and prints, per scale and view, how far the farthest point of a body lies outside the filled outlines its bone draws there (0 when every body is hidden) with the body, and a last line with the largest over the plant.
 * @see ../MV-3/body.ts — the bodies; its search holds them inside the outlines
 * @see ./caps.ts — the rounding after it */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
type P = { x: number; y: number };
const args = process.argv.slice(2);
const prefix = args.indexOf("--prefix") < 0 ? "mvb-" : args[args.indexOf("--prefix") + 1]!;
const asset = JSON.parse(readFileSync(fileOf(args[0]!), "utf8")).asset;

/** 🎯️ How far a point lies outside the outlines (0 inside). */
function outside(p: P, outlines: readonly (readonly (readonly P[])[])[]): number {
  let nearest = Infinity;
  for (const rings of outlines) {
    let inside = false;
    for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [u, v] = [ring[i]!, ring[j]!];
      if ((u.y > p.y) !== (v.y > p.y) && p.x < u.x + ((v.x - u.x) * (p.y - u.y)) / (v.y - u.y)) inside = !inside;
      const [dx, dy] = [v.x - u.x, v.y - u.y];
      const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - u.x) * dx + (p.y - u.y) * dy) / (dx * dx + dy * dy)));
      nearest = Math.min(nearest, Math.hypot(p.x - u.x - t * dx, p.y - u.y - t * dy));
    }
    if (inside) return 0;
  }
  return nearest;
}

let most = 0;
for (const key of Object.keys(asset.representations).filter((name) => engine.authoredOf(asset, name) !== undefined)) {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
  for (const view of ["front", "plan"]) {
    let worst = 0, where = "", bodies = 0;
    for (const body of state.shapes[view].filter((entry: any) => String(entry.shape.id).startsWith(prefix))) {
      bodies++;
      const drawn = state.shapes[view].filter((entry: any) => entry.shape.bone === body.shape.bone && !String(entry.shape.id).startsWith(prefix) && entry.shape.filled === true).map((entry: any) => entry.rings);
      const beyond = Math.max(0, ...body.rings.flat().map((p: P) => outside(p, drawn)));
      if (beyond > worst) { worst = beyond; where = body.shape.id; }
    }
    if (bodies > 0) console.log(`1:${key} ${view}: ${bodies} bodies, at most ${(worst * 1000).toFixed(1)} mm beside the outlines of the default pose${where === "" ? "" : ` (${where})`}`);
    most = Math.max(most, worst);
  }
}
console.log(`${asset.id}: bodies at most ${(most * 1000).toFixed(1)} mm beside the drawn outlines in the default pose`);
