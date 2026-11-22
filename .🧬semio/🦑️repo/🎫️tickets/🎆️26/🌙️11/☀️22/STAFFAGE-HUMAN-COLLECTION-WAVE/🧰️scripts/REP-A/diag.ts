/** 🗺️ The catalog-silhouette guard of one asset file as the test measures it (bbox, IoU, largest boundary distance per scale and view) and, for every failing view, the worst boundary points with the drawn shapes and the volume solids next to them.
 *  bun diag.ts <asset.json> [scales=500,200,100,50] [views=elevation,front,plan] [--all] */
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, VIEWPOINTS, bodyDimensions, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

type P = { x: number; y: number };
const args = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const all = process.argv.includes("--all");
const [path, scalesArg, viewsArg] = args as [string, string | undefined, string | undefined];
const asset = (await Bun.file(path).json()).asset;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
const dseg = (p: P, a: P, b: P) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l)); return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy); };
const inside = (p: P, ring: readonly P[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > p.y) !== (ring[j]!.y > p.y) && p.x < ((ring[j]!.x - ring[i]!.x) * (p.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c; return c; };
let bad = 0;
for (const scale of (scalesArg ?? "500,200,100,50").split(",").map(Number)) {
  if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)]?.status !== "authored") continue;
  const feature = featureOf(scale as never), lim = 2 * feature;
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: any) => w.texture !== true);
  const bbox = bodyDimensions(solids as never);
  const bb = bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ");
  if (bb.includes("!")) bad++;
  console.log(`1:${scale} bbox ${bb}`);
  for (const view of (viewsArg ?? VIEWPOINTS.join(",")).split(",")) {
    const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature);
    const fail = m.iou < 0.92 || m.hausdorffM > lim;
    if (fail) bad++;
    console.log(`1:${scale} ${view} iou ${m.iou.toFixed(3)} haus ${(m.hausdorffM * 1000).toFixed(1)} mm (limit ${(lim * 1000).toFixed(0)})${fail ? " FAIL" : ""}`);
    if (!fail && !all) continue;
    const drawn = new Mask(m.box, m.cell);
    for (const g of m.filled) drawn.fill(g);
    for (const g of m.outlines) drawn.fill(g);
    const vol = new Mask(m.box, m.cell);
    for (const g of m.reference) vol.fill(g, true);
    if (m.section) for (const g of m.visible) vol.fill(g, true);
    vol.seal();
    const [w, h] = [vol.width, vol.height];
    const holes = Math.round((4 * feature / m.cell) ** 2);
    const dc = fillSmallHoles(drawn.cells, w, h, holes), vc = fillSmallHoles(vol.cells, w, h, holes);
    const bd = boundaryOf(dc, w, h), bv = boundaryOf(vc, w, h);
    const toV = distanceField(bv, w, h), toD = distanceField(bd, w, h);
    const out: { side: string; x: number; y: number; d: number }[] = [];
    for (const [side, from, field] of [["drawn-only", bd, toV], ["volume-only", bv, toD]] as const) {
      const seen = new Set<string>();
      const list: { i: number; j: number; d: number }[] = [];
      for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * m.cell > (fail ? Math.min(lim, 0.9 * m.hausdorffM) : 0.7 * m.hausdorffM)) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * m.cell });
      list.sort((a, b) => b.d - a.d);
      for (const c of list) {
        const x = m.box.minX + (c.i + 0.5) * m.cell, y = m.box.minY + (c.j + 0.5) * m.cell;
        const key = `${Math.round(x / 0.04)},${Math.round(y / 0.04)}`;
        if (seen.has(key)) continue;
        seen.add(key);
        out.push({ side, x, y, d: c.d });
      }
    }
    out.sort((a, b) => b.d - a.d);
    for (const c of out.slice(0, Number(process.env.TOP ?? 6))) {
      const r = Math.max(0.012, c.d * 1.2);
      const shapes = (state.shapes[view] as any[]).map((s) => { let best = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - (s.closed ? 0 : 1); i++) best = Math.min(best, dseg(c, ring[i], ring[(i + 1) % ring.length])); const cov = s.closed && s.shape.filled !== false && s.rings.some((ring: P[]) => inside(c, ring)); return { s, best, cov }; }).filter((x) => x.best <= r || x.cov).sort((a, b) => a.best - b.best).slice(0, 5)
        .map((x) => `${x.s.shape.id}[${x.s.shape.bone}${x.s.shape.solid ? "," + x.s.shape.solid.kind : ""}${x.s.shape.filled === false ? ",open" : ""}${x.cov ? ",covers" : ""}] ${(x.best * 1000).toFixed(0)}`);
      const near = solids.map((s: any) => { let best = Infinity; for (let k = 0; k < s.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = s.indices[k + j]! * 3; return project(view as never, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); }) as [P, P, P]; best = Math.min(best, distanceToTriangle(c, t)); if (best === 0) break; } return { s, best }; }).filter((x: any) => x.best <= r).sort((a: any, b: any) => a.best - b.best).slice(0, 5)
        .map((x: any) => `${x.s.id}[${x.s.bone ?? ""}${x.s.slender ? ",slender" : ""}] ${(x.best * 1000).toFixed(0)}`);
      console.log(`   ${c.side.padEnd(11)} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${(c.d * 1000).toFixed(1)} mm | drawn: ${shapes.join("; ")} | solids: ${near.join("; ")}`);
    }
  }
}
console.log(`${asset.id}: ${bad === 0 ? "OK" : `${bad} failing`}`);
