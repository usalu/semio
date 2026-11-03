import { load, save, stateAt, three, transformPath } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const x = load(id); const a = x.asset;
const D = a.dimensions; const root = a.rig.root;
const r4 = (v: number) => Math.round(v * 10000) / 10000;
type P = [number, number];
const st = stateAt(a, 200); const E = three.extentOf(st.shapes.elevation), F = three.extentOf(st.shapes.front);
const ex0 = r4(E.minX + (E.maxX - E.minX - D.widthM) / 2), ex1 = r4(ex0 + D.widthM);
const fz0 = r4((F.minX + F.maxX) / 2 - D.depthM / 2), fz1 = r4(fz0 + D.depthM);
const H = D.heightM;
const leafy = st.shapes.elevation.filter((s: any) => /^(fol|curt|cap|crown|bl-|leaf|bush|frond|fr-)/.test(s.shape.bone) || /leaf|leaves|needle|blade|frond|plume|curtain|crown/i.test(s.shape.region ?? (s.shape.regions ?? []).join(" ")));
const yb = leafy.length ? Math.min(...leafy.flatMap((s: any) => s.rings.flat().map((p: any) => p.y))) : H * 0.3;
const STYLE: Record<string, any> = {
  "apple-tree": { kind: "cloud", n: 9, amp: 0.12, stems: [[0, 0, 0.19]] },
  "english-oak": { kind: "cloud", n: 11, amp: 0.1, stems: [[0, 0, 0.4]] },
  "small-leaved-lime": { kind: "cloud", n: 10, amp: 0.1, stems: [[0, 0, 0.28]] },
  "japanese-maple": { kind: "cloud", n: 13, amp: 0.1, stems: [[0, 0, 0.085], [-0.03, 0, 0.075], [0.04, 0, 0.07]], tiers: true },
  "london-plane-pollarded": { kind: "cloud", n: 9, amp: 0.1, stems: [[0, 0, 0.32]], pollard: true },
  "lombardy-poplar": { kind: "cloud", n: 15, amp: 0.06, stems: [[0, 0, 0.24]] },
  "scots-pine": { kind: "cloud", n: 7, amp: 0.2, stems: [[0, 0, 0.25]] },
  "weeping-willow": { kind: "cloud", n: 13, amp: 0.07, stems: [[0, 0, 0.425]], fringe: true },
  "multi-stem-birch": { kind: "cloud", n: 11, amp: 0.1, stems: [[0, -0.1, 0.07], [0, 0, 0.09], [0, 0.1, 0.075]], spread: true },
  "date-palm": { kind: "palm", stems: [[0, 0, 0.25]] },
  "agave": { kind: "rosette", n: 17 },
  "hosta-clump": { kind: "mound", n: 7, amp: 0.14 },
  "miscanthus-grass": { kind: "mound", n: 23, amp: 0.2 },
  "bamboo-clump": { kind: "bamboo" },
  "hornbeam-hedge": { kind: "hedge" },
  "ivy-on-trellis": { kind: "panel" },
};
const S = STYLE[id]; if (!S) throw new Error("no style " + id);
const norm = (pts: P[], x0: number, x1: number, y0: number, y1: number): P[] => { const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]); const ax = Math.min(...xs), bx = Math.max(...xs), ay = Math.min(...ys), by = Math.max(...ys); return pts.map(([px, py]) => [r4(x0 + ((px - ax) / (bx - ax)) * (x1 - x0)), r4(y0 + ((py - ay) / (by - ay)) * (y1 - y0))]); };
const cloud = (x0: number, x1: number, y0: number, y1: number, n: number, amp: number, phase: number, floor = false): P[] => { const pts: P[] = []; for (let i = 0; i < 240; i++) { const t = (i / 240) * 2 * Math.PI; const r = 1 - amp + amp * Math.pow(Math.abs(Math.sin((n * t) / 2 + phase)), 0.7) + 0.03 * Math.sin(3 * t + phase); let py = Math.sin(t) * r; if (floor && py < 0) py = 0; pts.push([Math.cos(t) * r, py]); } return norm(pts, x0, x1, y0, y1); };
const star = (x0: number, x1: number, y0: number, y1: number, n: number, inner: number, rot: number): P[] => { const pts: P[] = []; for (let i = 0; i < 2 * n; i++) { const t = (i / (2 * n)) * 2 * Math.PI + rot; const r = i % 2 === 0 ? 1 : inner; pts.push([Math.cos(t) * r, Math.sin(t) * r]); } return norm(pts, x0, x1, y0, y1); };
const mk = (sid: string, role: string, z: number, filled: boolean, tone: number, points: P[], closed = true) => ({ id: sid, bone: root, role, z, filled, tone, kind: "poly", points, closed });
const circle = (cx: number, cy: number, r: number): P[] => Array.from({ length: 24 }, (_, i) => [r4(cx + r * Math.cos((i / 24) * 2 * Math.PI)), r4(cy + r * Math.sin((i / 24) * 2 * Math.PI))] as P);
const trunkPoly = (cx: number, r: number, top: number, lean = 0): P[] => ([[cx - r * 1.35, 0], [cx - r * 1.05, 0.3], [cx - r + lean * 0.5, top * 0.5], [cx - r * 0.8 + lean, top], [cx + r * 0.8 + lean, top], [cx + r + lean * 0.5, top * 0.5], [cx + r * 1.05, 0.3], [cx + r * 1.35, 0]] as P[]).map(([a2, b]) => [r4(a2), r4(b)] as P);
const rect = (x0: number, x1: number, y0: number, y1: number): P[] => [[x0, y0], [x1, y0], [x1, y1], [x0, y1]];
const el: any[] = [], fr: any[] = [], pl: any[] = [];
const crownTop = H;
switch (S.kind) {
  case "cloud": {
    el.push(mk("crown", "body", 2, false, 0.5, cloud(ex0, ex1, yb, crownTop, S.n, S.amp, 0.4)));
    fr.push(mk("crown", "body", 2, false, 0.5, cloud(fz0, fz1, yb, crownTop, S.n + 2, S.amp, 1.3)));
    pl.push(mk("crown", "body", 2, false, 0.5, cloud(ex0, ex1, fz0, fz1, S.n + 1, S.amp, 2.1)));
    S.stems.forEach(([sx, sz, r]: number[], i: number) => {
      const top = yb + (crownTop - yb) * 0.55; const lean = S.spread ? (i - 1) * 0.12 * D.widthM / 2 : 0; const leanZ = S.spread ? (i - 1) * 0.12 * D.depthM / 2 : 0;
      el.push(mk(`stem-${i + 1}`, "body", 3, true, 0.3, trunkPoly(sx!, r!, top, lean)));
      fr.push(mk(`stem-${i + 1}`, "body", 3, true, 0.3, trunkPoly(sz!, r!, top, leanZ)));
      pl.push(mk(`stem-${i + 1}`, "body", 4, true, 0.25, circle(sx!, sz!, r!)));
    });
    const k = 4 + (S.tiers ? 2 : 0);
    for (let i = 0; i < k; i++) { const t = ((i + 0.5) / k) * 2 * Math.PI + 0.3; const rx = (ex1 - ex0) / 2 * 0.62, rz = (fz1 - fz0) / 2 * 0.62; const mx = (ex0 + ex1) / 2, mz = (fz0 + fz1) / 2; pl.push(mk(`scaffold-${i + 1}`, "detail", 3, false, 0.35, [[S.stems[0][0], S.stems[0][1]], [r4(mx + rx * Math.cos(t)), r4(mz + rz * Math.sin(t))]], false)); }
    if (S.pollard) pl.push(mk("head-ring", "detail", 3, false, 0.4, circle(0, 0, 0.9)));
    if (S.fringe) for (let i = 0; i < 11; i++) { const px = ex0 + ((i + 0.5) / 11) * (ex1 - ex0); el.push(mk(`curtain-${i + 1}`, "detail", 3, false, 0.45, [[r4(px), r4(yb + 0.08 * (crownTop - yb))], [r4(px), r4(yb)]], false)); const pz = fz0 + ((i + 0.5) / 11) * (fz1 - fz0); fr.push(mk(`curtain-${i + 1}`, "detail", 3, false, 0.45, [[r4(pz), r4(yb + 0.08 * (crownTop - yb))], [r4(pz), r4(yb)]], false)); }
    break;
  }
  case "palm": {
    const cy0 = H * 0.62;
    el.push(mk("crown", "body", 2, false, 0.5, star(ex0, ex1, cy0, H, 15, 0.3, 0.2)));
    fr.push(mk("crown", "body", 2, false, 0.5, star(fz0, fz1, cy0, H, 13, 0.3, 0.5)));
    pl.push(mk("crown", "body", 2, false, 0.5, star(ex0, ex1, fz0, fz1, 16, 0.28, 0.1)));
    el.push(mk("stem", "body", 3, true, 0.3, trunkPoly(0, 0.25, cy0 + 0.6)));
    fr.push(mk("stem", "body", 3, true, 0.3, trunkPoly(0, 0.25, cy0 + 0.6)));
    pl.push(mk("stem", "body", 4, true, 0.25, circle(0, 0, 0.25)));
    break;
  }
  case "rosette": {
    const half = (x0: number, x1: number, n: number, rot: number) => norm(star(x0, x1, -H, H, n, 0.55, rot).map(([px, py]) => [px, Math.max(py, 0)] as P), x0, x1, 0, H);
    el.push(mk("crown", "body", 2, false, 0.5, half(ex0, ex1, S.n, Math.PI / 2 - 0.1)));
    fr.push(mk("crown", "body", 2, false, 0.5, half(fz0, fz1, S.n - 2, Math.PI / 2 + 0.2)));
    pl.push(mk("crown", "body", 2, false, 0.5, star(ex0, ex1, fz0, fz1, S.n, 0.5, 0.15)));
    pl.push(mk("heart", "detail", 3, false, 0.4, circle((ex0 + ex1) / 2, (fz0 + fz1) / 2, 0.12)));
    break;
  }
  case "mound": {
    el.push(mk("crown", "body", 2, false, 0.5, cloud(ex0, ex1, 0, H, S.n, S.amp, 0.2, true)));
    fr.push(mk("crown", "body", 2, false, 0.5, cloud(fz0, fz1, 0, H, S.n + 2, S.amp, 1.1, true)));
    pl.push(mk("crown", "body", 2, false, 0.5, id === "hosta-clump" ? star(ex0, ex1, fz0, fz1, 7, 0.62, 0.2) : cloud(ex0, ex1, fz0, fz1, S.n, S.amp, 0.9)));
    pl.push(mk("heart", "detail", 3, false, 0.4, circle((ex0 + ex1) / 2, (fz0 + fz1) / 2, id === "hosta-clump" ? 0.08 : 0.17)));
    break;
  }
  case "bamboo": {
    const cx = [-0.82, -0.6, -0.28, 0.02, 0.32, 0.62];
    el.push(mk("crown", "body", 2, false, 0.5, cloud(ex0, ex1, yb, H, 11, 0.12, 0.5)));
    fr.push(mk("crown", "body", 2, false, 0.5, cloud(fz0, fz1, yb, H, 9, 0.12, 1.5)));
    pl.push(mk("crown", "body", 2, false, 0.5, cloud(ex0, ex1, fz0, fz1, 9, 0.12, 2.3)));
    const zs = [-0.35, 0.3, -0.1, 0.25, -0.3, 0.1];
    cx.forEach((px, i) => { el.push(mk(`culm-${i + 1}`, "detail", 3, false, 0.3, [[px, 0], [r4(px * 1.15), r4(yb + 0.3)]], false)); fr.push(mk(`culm-${i + 1}`, "detail", 3, false, 0.3, [[zs[i]!, 0], [r4(zs[i]! * 1.2), r4(yb + 0.3)]], false)); pl.push(mk(`culm-${i + 1}`, "body", 4, true, 0.25, circle(px, zs[i]!, 0.0145))); });
    break;
  }
  case "hedge": {
    el.push(mk("hedge", "body", 2, false, 0.5, [[ex0, 0], [ex1, 0], [ex1, H - 0.18], [ex1 - 0.1, H], [ex0 + 0.1, H], [ex0, H - 0.18]]));
    fr.push(mk("hedge", "body", 2, false, 0.5, [[fz0, 0], [fz1, 0], [fz1 - 0.04, H - 0.15], [fz1 - 0.12, H], [fz0 + 0.12, H], [fz0 + 0.04, H - 0.15]]));
    pl.push(mk("hedge", "body", 2, false, 0.5, rect(ex0, ex1, fz0, fz1)));
    pl.push(mk("axis", "detail", 3, false, 0.4, [[ex0, 0], [ex1, 0]], false));
    for (let i = 0; i < 6; i++) pl.push(mk(`stem-${i + 1}`, "body", 4, true, 0.25, circle(r4(ex0 + 0.25 + i * 0.5), 0, 0.03)));
    break;
  }
  case "panel": {
    el.push(mk("panel", "body", 2, false, 0.5, rect(ex0, ex1, 0, H)));
    fr.push(mk("panel", "body", 2, false, 0.5, rect(fz0, fz1, 0, H)));
    pl.push(mk("panel", "body", 2, false, 0.5, rect(ex0, ex1, fz0, fz1)));
    [-0.9, 0.9].forEach((px, i) => { el.push(mk(`post-${i + 1}`, "detail", 3, false, 0.3, rect(px - 0.045, px + 0.045, 0, H))); fr.push(mk(`post-${i + 1}`, "detail", 3, false, 0.3, rect(-0.045, 0.045, 0, H))); pl.push(mk(`post-${i + 1}`, "body", 4, true, 0.25, rect(px - 0.045, px + 0.045, -0.045, 0.045))); });
    break;
  }
}
const sizeEn = `${D.widthM} m wide, ${D.depthM} m deep and ${D.heightM} m high`, sizeDe = `${String(D.widthM).replace(".", ",")} m breit, ${String(D.depthM).replace(".", ",")} m tief und ${String(D.heightM).replace(".", ",")} m hoch`;
for (const sym of a.symbols) {
  if (sym.semantics === "true-extent") {
    sym.geometry = { elevation: el, front: fr, plan: pl };
    sym.dimensionalNote = { en: `A conventional simplification of the plant drawn at its true extent in model metres in all three views (${sizeEn}): the outline is the crown or body, the stem or stems are drawn at their real position and thickness. It is a symbol, never used silently for a pictorial scale.`, de: `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${sizeDe}): Der Umriss ist Krone oder Körper, Stamm oder Stämme sind in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.` };
  } else {
    const plan = sym.geometry.plan; const move = (s: any, dy: number) => { const c = structuredClone(s); if (c.kind === "path") c.d = transformPath(c.d, { sx: 1, tx: 0, sy: 1, ty: dy }); else if (c.kind === "ellipse") c.cy += dy; return c; };
    const circ = plan.find((s: any) => s.id === "sign-circle"); const rr = (() => { const m = circ.d.match(/-?\d*\.?\d+/g)!.map(Number); const ys = m.filter((_: number, i: number) => i % 2 === 1); return (Math.max(...ys) - Math.min(...ys)) / 2; })();
    const lift = rr * 1.9;
    const build = () => [...plan.map((s: any) => move(s, lift)), { id: "sign-stem", bone: root, role: "body", z: 1, filled: false, tone: 0.3, kind: "path", d: `M0 0L0 ${r4(lift - rr)}` }];
    sym.geometry = { elevation: build(), front: build(), plan };
    sym.dimensionalNote = { en: `A conventional symbol in paper millimetres, not to scale: a fixed ${r4(rr * 2)} mm tree sign, drawn as plan circle, side lollipop and front lollipop. Its size carries no dimension; the crown is round, so side and front are the same sign.`, de: `Ein konventionelles Symbol in Papiermillimetern, nicht maßstäblich: ein festes Baumzeichen von ${String(r4(rr * 2)).replace(".", ",")} mm, gezeichnet als Grundrisskreis sowie als Lutscher in Seiten- und Vorderansicht. Seine Größe trägt keine Abmessung; die Krone ist rund, daher sind Seiten- und Vorderansicht dasselbe Zeichen.` };
  }
}
if (write) save(x); else console.log(id, "yb", yb.toFixed(2), "ex", ex0, ex1, "fz", fz0, fz1);
