/** 🕵️ Which drawn shapes cover a point (filled closed shapes by area, others as strokes of their pen) and which volume solids project onto it. OPS env applies ops first. bun humansa-who.ts <asset> <scale> <view> <x> <y> */
import { existsSync, readFileSync } from "node:fs";
import { loadAsset, stateOf, shapesOf, type Viewpoint } from "./humansa-lib.ts";
import { Mask, strokeQuad, worldSolids, referenceOf, drawnFor, distanceToTriangle, project } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, xs, ys] = process.argv.slice(2) as [string, string, Viewpoint, string, string];
const asset = await loadAsset(name);
const scale = Number(sc);
if (process.env.OPS && existsSync(process.env.OPS)) for (const o of JSON.parse(readFileSync(process.env.OPS, "utf8"))) {
  if (o.scale !== String(scale)) continue;
  const list: any[] = o.op.startsWith("shape") ? shapesOf(asset, scale, o.view) : (asset as any).volumes[o.scale].body;
  const find = (id: string): number => list.findIndex((x) => x.id === id);
  if (o.op.endsWith("-add")) list.splice(find(o.after) + 1, 0, o.shape ?? o.solid);
  else if (o.op.endsWith("-set")) Object.assign(list[find(o.id)], o.set);
  else list[find(o.id)] = o.shape ?? o.solid;
}
const [x, y] = [Number(xs), Number(ys)];
const state = stateOf(asset, scale);
const rep = (asset.representations as any)[sc];
const cell = 0.0005;
const box = { minX: x - 0.01, minY: y - 0.01, maxX: x + 0.01, maxY: y + 0.01 };
const has = (m: Mask): boolean => m.has(x, y);
for (const s of (state.shapes as any)[view]) {
  if (!["body", "detail", "glazing"].includes(s.shape.role) || s.rings.length === 0) continue;
  const m = new Mask(box, cell);
  const pen = ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
  if (s.closed && s.shape.filled !== false) m.fill(s.rings);
  else for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) m.fill([strokeQuad(r[i], r[i + 1], Math.max(pen, 1e-4))]); }
  if (has(m)) console.log(`DRAWN ${s.shape.id} role=${s.shape.role} closed=${s.closed} filled=${s.shape.filled} pen=${pen.toFixed(4)} z=${s.shape.z}${s.shape.solid ? " solid" : ""}`);
}
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
for (const w of solids) {
  const ref = referenceOf(view, [w], undefined, drawnFor(asset, state, view, scale)).cut;
  const m = new Mask(box, cell);
  for (const g of ref) m.fill(g, true);
  if (has(m)) console.log(`VOLUME ${w.id} bone=${w.bone}${w.slender ? " slender" : ""}`);
}
