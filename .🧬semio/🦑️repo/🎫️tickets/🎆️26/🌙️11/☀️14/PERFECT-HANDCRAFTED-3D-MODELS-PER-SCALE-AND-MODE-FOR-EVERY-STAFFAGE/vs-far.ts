import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const S = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next-b/🌓️silhouette/🟦️.ts`);
const [slug, scale, view, limitMm] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!, Number(process.argv[5] ?? 30)];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry) as any;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = S.worldSolids(asset, state.bones, scale, "body").filter((w: any) => !w.texture);
const m = S.measureView(asset, state, solids, view, scale, undefined, (0.15 * scale) / 1000, Infinity, true);
console.log("p95", m.p95M, "iou", m.iou, "cell", m.cell, "shapes", m.shapes);
const { reference, filled, outlines, box, cell } = m;
const vol = new S.Mask(box, cell); for (const g of reference) vol.fill(g, true); vol.seal();
const drawn = new S.Mask(box, cell); for (const g of [...filled, ...outlines]) drawn.fill(g);
const bv = S.boundaryOf(vol.cells, vol.width, vol.height), bd = S.boundaryOf(drawn.cells, drawn.width, drawn.height);
const dv = S.distanceField(bv, vol.width, vol.height), dd = S.distanceField(bd, vol.width, vol.height);
const far = new Map<string, number>();
for (let j = 0; j < vol.height; j++) for (let i = 0; i < vol.width; i++) { const k = j * vol.width + i;
  if (bd[k] && Math.sqrt(dv[k]!) * cell * 1000 > limitMm) { const key = `drawn-boundary-far-from-3D @${(box.minX + i * cell).toFixed(1)},${(box.minY + j * cell).toFixed(1)}`.replace(/(\d+\.\d)\d*/g, "$1"); far.set("D " + key.slice(-12), (far.get("D " + key.slice(-12)) ?? 0) + 1); }
  if (bv[k] && Math.sqrt(dd[k]!) * cell * 1000 > limitMm) { const key = `${(Math.floor((box.minX + i * cell) * 4) / 4).toFixed(2)},${(Math.floor((box.minY + j * cell) * 4) / 4).toFixed(2)}`; far.set("V " + key, (far.get("V " + key) ?? 0) + 1); }
}
console.log([...far].sort((a, b) => b[1] - a[1]).slice(0, 25).map(([k, v]) => `${k}:${v}`).join("\n"));
console.log("boundary cells drawn", bd.reduce((s: number, x: number) => s + x, 0), "3d", bv.reduce((s: number, x: number) => s + x, 0));
const hist = m.spread.hard as number[];
let tot = 0; hist.forEach((x) => (tot += x));
console.log("hard total", tot, "soft total", (m.spread.soft as number[]).reduce((a: number, b: number) => a + b, 0), "far bins (mm:count)", hist.map((x, d) => [Math.round(d * cell * 1000), x]).filter(([d, x]: any) => d > limitMm && x > 0).slice(0, 12).map(([d, x]: any) => `${d}:${x}`).join(" "));
{
  const holes = Math.round((4 * ((0.15 * scale) / 1000) / cell) ** 2);
  const dm = new Uint8Array(drawn.cells.length); for (let i = 0; i < dm.length; i++) dm[i] = drawn.cells[i]!;
  const A = S.fillSmallHoles(dm, vol.width, vol.height, holes), B = S.fillSmallHoles(vol.cells, vol.width, vol.height, holes);
  const ba = S.boundaryOf(A, vol.width, vol.height), bb = S.boundaryOf(B, vol.width, vol.height);
  const da = S.distanceField(ba, vol.width, vol.height), db = S.distanceField(bb, vol.width, vol.height);
  const out = new Map<string, number>();
  for (let j = 0; j < vol.height; j++) for (let i = 0; i < vol.width; i++) { const k = j * vol.width + i;
    const x = (box.minX + i * cell), y = (box.minY + j * cell);
    if (ba[k] && Math.sqrt(db[k]!) * cell * 1000 > limitMm) { const key = `drawn@${(Math.floor(x * 10) / 10).toFixed(1)},${(Math.floor(y * 10) / 10).toFixed(1)}`; out.set(key, (out.get(key) ?? 0) + 1); }
    if (bb[k] && Math.sqrt(da[k]!) * cell * 1000 > limitMm) { const key = `3d@${(Math.floor(x * 10) / 10).toFixed(1)},${(Math.floor(y * 10) / 10).toFixed(1)}`; out.set(key, (out.get(key) ?? 0) + 1); }
  }
  console.log("holes", holes, [...out].sort((a, b) => b[1] - a[1]).slice(0, 20).map(([k, v]) => `${k}:${v}`).join("  "));
}
{
  const W = vol.width, H = vol.height;
  const comps = (mask: Uint8Array) => { const seen = new Uint8Array(mask.length); const res: any[] = [];
    for (let s = 0; s < mask.length; s++) { if (mask[s] || seen[s]) continue; const st = [s]; seen[s] = 1; let n = 0, border = false, x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
      while (st.length) { const k = st.pop()!; n++; const i = k % W, j = (k / W) | 0; if (i === 0 || j === 0 || i === W - 1 || j === H - 1) border = true; x0 = Math.min(x0, i); x1 = Math.max(x1, i); y0 = Math.min(y0, j); y1 = Math.max(y1, j);
        for (const q of [k - 1, k + 1, k - W, k + W]) if (q >= 0 && q < mask.length && !mask[q] && !seen[q] && !(Math.abs((q % W) - i) > 1)) { seen[q] = 1; st.push(q); } }
      if (!border) res.push({ cells: n, x: [(box.minX + x0 * cell).toFixed(3), (box.minX + x1 * cell).toFixed(3)], y: [(box.minY + y0 * cell).toFixed(3), (box.minY + y1 * cell).toFixed(3)] }); }
    return res; };
  console.log("drawn holes", JSON.stringify(comps(drawn.cells).filter((c) => c.cells > 100)));
  console.log("3d holes", JSON.stringify(comps(vol.cells).filter((c) => c.cells > 100)));
  const ids = new Map<string, number>(); for (const g of [...filled]) { }
}
for (const s of state.shapes[view]) { const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); if (!xs.length) continue; const b = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)]; if (b[0] < -0.2 && b[1] > -0.4 && b[2] < 1.0 && b[3] > 0.5) console.log("2D", s.shape.id, s.shape.bone, s.shape.role, s.closed ? "closed" : "open", s.shape.filled, b.map((x) => +x.toFixed(3)).join(",")); }
for (const w of solids) { const e = S.extentOf([w]); if (e.min[2] < 0.5 + 1 && true) { const [x0, x1] = [e.min[2], e.max[2]]; if (x0 < -0.2 && x1 > -0.4 && e.min[1] < 1.0 && e.max[1] > 0.5) console.log("3D", w.id, w.bone, "z", x0.toFixed(3), x1.toFixed(3), "y", e.min[1].toFixed(3), e.max[1].toFixed(3)); } }
