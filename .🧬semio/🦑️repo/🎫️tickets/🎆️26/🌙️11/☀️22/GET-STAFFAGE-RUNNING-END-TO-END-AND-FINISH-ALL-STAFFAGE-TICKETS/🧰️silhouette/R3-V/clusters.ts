/** 🧭️ Worst mismatch clusters per view with the nearest drawn shapes and volume solids and the drawn shapes that cover the point (after `humansb-D-clusters.ts` of the 2026-11-17 guard ticket), optional ops applied first. bun humansb-D-clusters.ts <asset> <scale> [view|all] [ops.json|-] [count]; LIMIT=<m> lists places above another distance */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, VIEWPOINTS, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, only, patch, cnt, px, py, pr] = process.argv.slice(2);
const scale = Number(scaleArg);
const FILE = name!.endsWith(".json") ? name! : `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/${name}/🔣️.json`;
const { asset } = (await Bun.file(FILE).json()) as { asset: Asset };
if (patch && patch !== "-") (await import("./ops.ts")).applyOps(asset, JSON.parse(await Bun.file(patch).text()));
const feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const dseg = (P: any, a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((P.x - a.x) * dx + (P.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(P.x - a.x - t * dx, P.y - a.y - t * dy); };
const near = (view: any, P: any, R: number) => {
  const d: string[] = [];
  for (const s of state.shapes[view]) { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(P, ring[i], ring[(i + 1) % ring.length])); if (m < R) d.push(`${s.shape.id}[${s.shape.bone}]${m.toFixed(3)}`); }
  const v: string[] = [];
  for (const w of solids) { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [any, any, any]; m = Math.min(m, distanceToTriangle(P, t)); } if (m < R) v.push(`${w.id}[${w.bone}]${m.toFixed(3)}`); }
  const inside = (ring: any[]): boolean => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i].y > P.y) !== (ring[j].y > P.y) && P.x < ((ring[j].x - ring[i].x) * (P.y - ring[i].y)) / (ring[j].y - ring[i].y) + ring[i].x) c = !c; return c; };
  const over = state.shapes[view].filter((s: any) => s.closed && s.rings.filter(inside).length % 2 === 1).map((s: any) => `${s.shape.id}[${s.shape.bone}${s.shape.solid ? ":" + s.shape.solid.kind : ""}]`);
  return `drawn: ${d.join(" ")} | vol: ${v.join(" ")} | covering: ${over.join(" ")}`;
};
for (const view of VIEWPOINTS) {
  if (process.env.SHOW !== undefined) for (const s of state.shapes[view]) if (String(s.shape.id).includes(process.env.SHOW)) { const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); console.log(`  [shape ${view}] ${s.shape.id} x ${Math.min(...xs).toFixed(3)} ${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)} ${Math.max(...ys).toFixed(3)} closed=${s.closed} filled=${s.shape.filled}`); }
  if (only !== undefined && only !== "all" && only !== view) continue;
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const cell = m.cell, box = m.box;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = (4 * feature / cell) ** 2;
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, Math.round(holes)), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, Math.round(holes)), w, h);
  const toVol = distanceField(bv, w, h), toDrawn = distanceField(bd, w, h);
  if (process.env.HOLES !== undefined) {
    const label = (cells: Uint8Array) => {
      const seen = new Uint8Array(w * h); const out: { n: number; x: number; y: number; border: boolean }[] = [];
      for (let s = 0; s < w * h; s++) if (!cells[s] && !seen[s]) {
        const st = [s]; seen[s] = 1; let n = 0, sx = 0, sy = 0, border = false;
        while (st.length) { const c = st.pop()!; const i = c % w, j = (c - i) / w; n++; sx += i; sy += j; if (i === 0 || j === 0 || i === w - 1 || j === h - 1) border = true; for (const [a, b] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) { const ii = i + a, jj = j + b; if (ii >= 0 && jj >= 0 && ii < w && jj < h && !cells[jj * w + ii] && !seen[jj * w + ii]) { seen[jj * w + ii] = 1; st.push(jj * w + ii); } } }
        if (!border) out.push({ n, x: +(box.minX + (sx / n + 0.5) * cell).toFixed(3), y: +(box.minY + (sy / n + 0.5) * cell).toFixed(3), border });
      }
      return out;
    };
    console.log(`  holes threshold ${Math.round((4 * feature / cell) ** 2)} cells`);
    for (const [nm, c] of [["drawn", drawn.cells], ["vol", vol.cells]] as const) console.log(`  ${nm}: ` + label(c).filter((o) => o.n > 3).map((o) => `${o.n}@(${o.x},${o.y})`).join(" "));
  }
  const limit = Number(process.env.LIMIT ?? 2 * feature);
  const clusters: { side: string; x: number; y: number; d: number }[] = [];
  for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
    const list: { i: number; j: number; d: number }[] = [];
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
    list.sort((a, b) => b.d - a.d);
    const seen = new Set<number>();
    for (const c of list) {
      const key = Math.round(c.i / (0.1 / cell)) * 100000 + Math.round(c.j / (0.1 / cell));
      if (seen.has(key)) continue;
      seen.add(key);
      clusters.push({ side, x: +(box.minX + (c.i + 0.5) * cell).toFixed(3), y: +(box.minY + (c.j + 0.5) * cell).toFixed(3), d: +c.d.toFixed(3) });
    }
  }
  const fd = fillSmallHoles(drawn.cells, w, h, Math.round((4 * feature / cell) ** 2)), fv = fillSmallHoles(vol.cells, w, h, Math.round((4 * feature / cell) ** 2));
  if (px !== undefined && py !== undefined) {
    const r = Number(pr ?? 0.1);
    const step = Math.max(1, Math.round((2 * r) / cell / 70));
    for (let y = Number(py) + r; y > Number(py) - r; y -= step * cell * 2) {
      let line = "";
      for (let x = Number(px) - r; x < Number(px) + r; x += step * cell) {
        const i = Math.floor((x - box.minX) / cell), j = Math.floor((y - box.minY) / cell);
        const [a, b] = process.env.FILLED !== undefined ? [fd[j * w + i], fv[j * w + i]] : [drawn.cells[j * w + i], vol.cells[j * w + i]];
        line += a && b ? "#" : a ? "D" : b ? "V" : ".";
      }
      console.log(line);
    }
  }
  console.log(`== ${view} 1:${scale} iou ${m.iou.toFixed(3)} H ${m.hausdorffM.toFixed(4)} limit ${limit} cell ${cell.toFixed(4)} drawn=${m.counts.drawn} vol=${m.counts.volume}`);
  for (const c of clusters.sort((a, b) => b.d - a.d).slice(0, Number(cnt ?? 6))) console.log(`  ${c.side} (${c.x}, ${c.y}) d=${c.d}  ${near(view, c, 0.05)}`);
}
