import { Author, type V3, type Section } from "../../veg-lib.ts";
const a = new Author("miscanthus-grass");
a.loftFromViews("tuft", "root-plate", { elev: "tuft", front: "tuft", range: [0, 0.2], n: 4, exp: 2.2, pinch: 0.9 });
const culms: { bone: string; meta: any }[] = [];
for (const n of [1, 2, 3, 4]) for (const p of ["a", "b"]) culms.push({ bone: `culm-${n}${p}`, meta: a.meta(`culm-${n}${p}`), ...(p === "b" ? { zEnd: 0.02 } : {}) } as any);
a.limbs(culms);
for (const n of [1, 2, 3, 4]) {
  const bone = `plume-${n}`, o = a.origin(bone), t = a.tip(bone);
  const f = a.box2("front", bone), end = a.ends.get(`culm-${n}b`)!.t;
  const dir: V3 = [t[0] - o[0], t[1] - o[1], 0];
  const prof: [number, number, number][] = [[0, 0.35, 0.5], [0.3, 1, 1], [0.7, 0.8, 0.85], [1.15, 0.12, 0.15]];
  const secs: Section[] = prof.map(([q, w, h]) => ({ c: [o[0] + dir[0] * q, o[1] + dir[1] * q, 0] as V3, rx: ((f.maxX - f.minX) / 2) * w, ry: 0.07 * h, exp: 2 }));
  a.loft(bone, bone, secs, a.meta(bone), { axis: dir });
}
const AXIS_O: V3 = [0, 0, 0];
const blade = (id: string, bone: string, eId: string, tanPsi: number, repeat?: any) => {
  const E = a.rings("elevation", eId), bb = a.box2("elevation", eId);
  const alongX = bb.maxX - bb.minX >= bb.maxY - bb.minY;
  const lo = alongX ? bb.minX : bb.minY, hi = alongX ? bb.maxX : bb.maxY;
  const secs: Section[] = [];
  const n = 6;
  for (let q = 0; q < n; q++) {
    const t = q / (n - 1);
    const s0 = Math.min(hi - 1e-4, Math.max(lo + 1e-4, lo + (hi - lo) * t));
    const r = a.slice(E, alongX ? "x" : "y", s0);
    if (!r) continue;
    const mid = (r[0] + r[1]) / 2, half = Math.max(0.01, (r[1] - r[0]) / 2) * 0.6 * (q === 0 || q === n - 1 ? 0.6 : 1);
    const x = alongX ? s0 : mid, y = alongX ? mid : s0;
    secs.push({ c: [x, y, Math.abs(x) * tanPsi] as V3, rx: 0.03 * (q === n - 1 ? 0.4 : 1), ry: alongX ? half : half * 0.6, exp: 2.2 });
  }
  a.loft(id, bone, secs, a.meta(eId), { extra: repeat ? { repeat } : undefined });
};
for (const side of ["L", "R"] as const) for (const i of [1, 2, 3]) {
  const sl = side.toLowerCase();
  for (const part of ["a", "b"] as const) {
    const bone = `bl-${side}${i}${part}`;
    blade(`blade-${sl}${i}-1${part}`, bone, `blade-${side}${i}-1${part}`, 0);
    for (const kBase of side === "L" ? [2, 4] : [3, 4]) {
      const kb = `${side}${i}-${kBase}-b`;
      const eb = a.box2("elevation", `blade-${side}${i}-${kBase}b`), fb = a.box2("front", kb);
      const xTip = Math.max(Math.abs(eb.minX), Math.abs(eb.maxX)), zTip = Math.max(Math.abs(fb.minX), Math.abs(fb.maxX));
      const tanPsi = zTip / xTip, psi = Math.atan(tanPsi);
      const repeat = { count: 2, rotationAxis: a.D(bone, [0, 1, 0]), rotationRad: (side === "L" ? -2 : 2) * psi, origin: a.L(bone, AXIS_O) };
      blade(`blade-${sl}${i}-${kBase}${part}`, bone, `blade-${side}${i}-${kBase}${part}`, tanPsi, repeat);
    }
  }
}
a.note("A Miscanthus clump: four arching culms (two capsules each, leaning sideways as the front view draws them) end in plume lofts; six blade sheaves (three left, three right, each in two bones) fan out around the tuft.", "Ein Miscanthus-Horst: vier überhängende Halme (je zwei Kapseln, seitlich geneigt wie in der Front gezeichnet) enden in Fahnen-Lofts; sechs Blattbüschel (drei links, drei rechts, je zwei Knochen) fächern um den Horst auf.");
a.note("Each sheaf is drawn in the sagittal plane in the elevation but spreads to both sides in the front view, so every sheaf is three ribbon lofts per bone turned about the vertical axis by 0 and plus/minus 38 degrees (the front tips reach 0.9 m sideways); thickness comes from the elevation outline, the five authored blades per sheaf are merged into three.", "Jedes Büschel ist in der Ansicht in der Längsebene gezeichnet, breitet sich in der Front aber nach beiden Seiten aus; jedes Büschel besteht deshalb je Knochen aus drei Bandlofts, um die Hochachse um 0 und plus/minus 38 Grad gedreht (die Frontspitzen reichen 0,9 m seitlich); die Dicke stammt aus dem Ansichtsumriss, die fünf gezeichneten Blätter je Büschel sind zu drei zusammengefasst.");
a.save();
