/** 🔬️ Probe (author 42): holes and parts of the 3D reference mask of a section view: bun author-42-probe2.ts <asset> <scale> <view> */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const sil: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const ev: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts");
const [spec, sc, view] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const state = ev.evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const solids = sil.worldSolids(m, state.bones, Number(sc)).filter((w: any) => w.texture !== true);
const section = m.representations[sc!].sections?.[view!];
const r = sil.referenceOf(view, solids, section);
console.log("section", JSON.stringify(section), "groups cut", r.cut.length, "visible", r.visible?.length, "full", r.full?.length);
const bb = (gs: any[]) => { let a = [1e9, 1e9, -1e9, -1e9]; for (const g of gs) for (const ring of g) for (const p of ring) { a = [Math.min(a[0], p.x), Math.min(a[1], p.y), Math.max(a[2], p.x), Math.max(a[3], p.y)]; } return a.map((x) => +x.toFixed(3)); };
console.log("cut bbox", bb(r.cut), "visible bbox", bb(r.visible ?? []), "full", bb(r.full ?? []));
const cell = 0.004;
const box = { minX: -0.4, minY: -1.4, maxX: 0.6, maxY: 0.6 };
const M = (gs: any[]) => { const k = new sil.Mask(box, cell); for (const g of gs) k.fill(g); return k; };
const mc = M(r.cut), mv = M(r.visible ?? []), mf = M(r.full ?? []);
const comp = (mask: any, name: string) => {
  const w = mask.width, h = mask.height, seen = new Uint8Array(w * h);
  const out: string[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const k = j * w + i; if (mask.cells[k] || seen[k]) continue;
    const st = [[i, j]]; seen[k] = 1; let n = 0, touches = false, x0 = i, x1 = i, y0 = j, y1 = j;
    while (st.length) { const [a, b] = st.pop()!; n++; x0 = Math.min(x0, a); x1 = Math.max(x1, a); y0 = Math.min(y0, b); y1 = Math.max(y1, b); if (a === 0 || b === 0 || a === w - 1 || b === h - 1) touches = true; for (const [da, db] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const na = a + da!, nb = b + db!; if (na < 0 || nb < 0 || na >= w || nb >= h) continue; const kk = nb * w + na; if (!mask.cells[kk] && !seen[kk]) { seen[kk] = 1; st.push([na, nb]); } } }
    if (!touches) out.push(`hole ${n} cells x ${(box.minX + x0 * cell).toFixed(3)}..${(box.minX + (x1 + 1) * cell).toFixed(3)} y ${(box.minY + y0 * cell).toFixed(3)}..${(box.minY + (y1 + 1) * cell).toFixed(3)}`);
  }
  console.log(name, out.length ? out.join("\n  ") : "no holes");
};
comp(mc, "cut"); comp(mv, "visible");
const u = new sil.Mask(box, cell); for (let i = 0; i < u.cells.length; i++) u.cells[i] = mc.cells[i] | mv.cells[i];
comp(u, "cut|visible");
