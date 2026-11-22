/** 🔍️ PW-16 diagnostic: where the drawn outline and the projected body of one view and scale are farthest apart (vertex to nearest edge of the other side, both directions), with an SVG overlay.
 *
 * `repo tools exec bun -- diag.ts <asset.json> <scale> <view> [out.svg]` */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids, type Group } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [file, scaleText, view, out] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string | undefined];
const asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
const scale = Number(scaleText);
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale as never } });
const m = measureView(asset, state, worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true), view, scale, undefined, featureOf(scale), 1e9);
type P = { x: number; y: number };
const edges = (groups: readonly Group[]): [P, P][] => groups.flatMap((g) => g.flatMap((r) => r.map((p, i) => [p, r[(i + 1) % r.length]!] as [P, P])));
const dist = (p: P, [a, b]: [P, P]): number => {
  const [dx, dy] = [b.x - a.x, b.y - a.y];
  const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
  return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy);
};
const worst = (from: readonly Group[], to: readonly Group[]): { d: number; p: P } => {
  const e = edges(to);
  let best = { d: 0, p: { x: 0, y: 0 } };
  for (const g of from) for (const r of g) for (const p of r) {
    let d = Infinity;
    for (const s of e) d = Math.min(d, dist(p, s));
    if (d > best.d) best = { d, p };
  }
  return best;
};
const drawn = [...m.filled, ...m.outlines];
{
  const box = m.box;
  const [vol, area, ink] = [new Mask(box, m.cell), new Mask(box, m.cell), new Mask(box, m.cell)];
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  for (const g of m.filled) area.fill(g);
  for (const g of m.outlines) ink.fill(g);
  const W = vol.width, H = vol.height;
  const dm = area.cells.map((x, i) => x | ink.cells[i]!);
  const edge = (c: Uint8Array): Uint8Array => c.map((x, k) => (x && (k % W === 0 || k % W === W - 1 || k < W || k >= W * (H - 1) || !c[k - 1] || !c[k + 1] || !c[k - W] || !c[k + W]) ? 1 : 0));
  const holes = Math.round((4 * featureOf(scale) / m.cell) ** 2);
  void edge;
  const [ed, ev] = [boundaryOf(fillSmallHoles(dm, W, H, holes), W, H), boundaryOf(fillSmallHoles(vol.cells, W, H, holes), W, H)];
  const [fd, fv] = [distanceField(ed, W, H), distanceField(ev, W, H)];
  const report = (name: string, from: Uint8Array, field: Float64Array): void => {
    const list: [number, number][] = [];
    from.forEach((x, k) => { if (x) list.push([Math.sqrt(field[k]!) * m.cell, k]); });
    list.sort((p, q) => q[0] - p[0]);
    console.log(`[DEBUG] ${name}: ` + list.slice(0, 6).map(([d, k]) => `${(d * 1000).toFixed(0)}mm@(${(box.minX + (k % W + 0.5) * m.cell).toFixed(3)},${(box.minY + (Math.floor(k / W) + 0.5) * m.cell).toFixed(3)})`).join(" "));
  };
  report("drawn edge far from body edge", ed, fv);
  report("body edge far from drawn edge", ev, fd);
}
const a = worst(drawn, m.reference);
const b = worst(m.reference, drawn);
console.log(`[DEBUG] 1:${scale} ${view} iou ${m.iou.toFixed(3)} h ${(m.hausdorffM * 1000).toFixed(0)} mm; drawn→body ${(a.d * 1000).toFixed(0)} mm at (${a.p.x.toFixed(3)}, ${a.p.y.toFixed(3)}); body→drawn ${(b.d * 1000).toFixed(0)} mm at (${b.p.x.toFixed(3)}, ${b.p.y.toFixed(3)})`);
if (out) {
  const all = [...drawn, ...m.reference].flatMap((g) => g.flat());
  const [x0, x1, y0, y1] = [Math.min(...all.map((p) => p.x)), Math.max(...all.map((p) => p.x)), Math.min(...all.map((p) => p.y)), Math.max(...all.map((p) => p.y))];
  const k = 1000 / Math.max(x1 - x0, y1 - y0);
  const path = (groups: readonly Group[]): string => groups.map((g) => g.map((r) => `M${r.map((p) => `${((p.x - x0) * k).toFixed(1)} ${((y1 - p.y) * k).toFixed(1)}`).join("L")}Z`).join("")).join("");
  writeFileSync(out, `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-10 -10 1020 ${((y1 - y0) * k + 20).toFixed(0)}" style="background:white"><path d="${path(m.reference)}" fill="rgba(255,0,0,0.35)" fill-rule="evenodd"/><path d="${path(drawn)}" fill="none" stroke="blue" stroke-width="0.6"/><circle cx="${((a.p.x - x0) * k).toFixed(1)}" cy="${((y1 - a.p.y) * k).toFixed(1)}" r="8" fill="none" stroke="green"/><circle cx="${((b.p.x - x0) * k).toFixed(1)}" cy="${((y1 - b.p.y) * k).toFixed(1)}" r="8" fill="none" stroke="orange"/></svg>`);
}
