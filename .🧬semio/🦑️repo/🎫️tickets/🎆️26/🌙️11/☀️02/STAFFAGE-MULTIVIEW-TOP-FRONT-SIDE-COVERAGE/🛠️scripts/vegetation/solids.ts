import { readFileSync } from "node:fs";
import { L, load, save, stateAt, three, transformShape } from "./lib.ts";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const id = process.argv[2]!; const write = process.argv[3] === "write";
const x = load(id); const a = x.asset;
const SPEC: Record<string, { plate: number; ribs: RegExp; rib0: number; rib1: number; round: RegExp }> = {
  "agave": { plate: 0.004, ribs: /^rib-/, rib0: 0.02, rib1: 0.002, round: /^rib-/ },
  "date-palm": { plate: 0.002, ribs: /rachis/, rib0: 0.03, rib1: 0.0075, round: /rachis/ },
  "hosta-clump": { plate: 0.0015, ribs: /^(midrib|petiole)-/, rib0: 0, rib1: 0, round: /^(midrib|petiole)-/ },
};
const S = SPEC[id]!;
const flagged = JSON.parse(readFileSync(process.argv[4]!, "utf8")).assets[`vegetation/${id}`].issues.filter((i: any) => i.code === "lateral-ambiguous").flatMap((i: any) => [...i.message.matchAll(/on bone "([^"]+)"/g)].map((m) => m[1] as string));
const bones = new Set<string>(flagged);
const boneOf = (b: string) => a.rig.bones.find((q: any) => q.id === b);
const r4 = (v: number) => Math.round(v * 10000) / 10000;
let n = 0;
if (id === "agave") { // fixpoly
for (const den of ["50", "100", "200", "500"]) for (const s of a.representations[den]?.geometry?.elevation ?? []) if (s.kind === "poly" && s.id === "leaf-R1-b" && den === "100" && s.points[0][0] === -0.1223) s.points = s.points.slice(1, -1);
}
for (const den of ["50", "100", "200", "500"]) {
  const rep = a.representations[den]; if (rep.status !== "authored") continue; const g = rep.geometry;
  for (const bone of bones) {
    const roundIds = new Map<string, any>();
    for (const view of ["elevation", "front", "plan"]) for (const q of g[view]) if (q.bone === bone && S.round.test(q.id) && !roundIds.has(q.id)) roundIds.set(q.id, q);
    const elev = g.elevation.filter((s: any) => s.bone === bone && !S.round.test(s.id));
    if (!elev.length && !roundIds.size) continue;
    if (id === "agave" && !roundIds.size && elev.length) roundIds.set(`rib-${bone.slice(5)}`, { id: `rib-${bone.slice(5)}`, role: "detail", z: 5, tone: 0.3, region: elev[0].region });
    const bl = boneOf(bone).lengthM;
    const round = (s: any, view: string) => {
      const front = g.front.find((q: any) => q.bone === bone && S.round.test(q.id) && q.kind === "poly");
      let r0 = S.rib0, r1 = S.rib1;
      if (!r0) { const pts = front?.points; if (pts) { const hw = (pts: number[][]) => { const xs = pts.map((p) => p[0]); const lo = Math.min(...xs), hi = Math.max(...xs); const at = (xx: number) => Math.max(...pts.filter((p) => Math.abs(p[0] - xx) < (hi - lo) * 0.2).map((p) => Math.abs(p[1]))); return [at(lo), at(hi)]; }; [r0, r1] = hw(pts); } else { r0 = 0.003; r1 = 0.0015; } }
      if (/^midrib/.test(s.id)) { r0 = 0.003; r1 = 0.0008; }
      const thin = Math.min(r0, r1) * 2 < 0.02;
      return { id: s.id, bone, role: s.role, z: s.z, filled: !thin, tone: s.tone, ...(s.region ? { region: s.region } : {}), ...(s.penMm ? { penMm: s.penMm } : {}), kind: "poly", points: [[0, 0], [0, r4(r0)], [r4(bl), r4(r1)], [r4(bl), 0]], closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, mirror: true } };
    };
    const plate = (s: any) => ({ ...structuredClone(s), ...(s.kind === "poly" ? { points: s.points.filter((p: number[], i: number, all: number[][]) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]) } : {}), solid: { kind: "extruded", z0: -S.plate / 2, z1: S.plate / 2 } });
    const defs = [...[...roundIds.values()].map((s: any) => round(s, "elevation")), ...elev.map((s: any) => plate(s))];
    for (const view of ["elevation", "front", "plan"]) {
      g[view] = [...g[view].filter((q: any) => q.bone !== bone), ...structuredClone(defs)];
      n += defs.length;
    }
  }
}
const FIT: Record<string, RegExp> = { "date-palm": /^fr-[lr]1[ab]$/, "hosta-clump": /^blade-/ };
const scaleBones = (f: number) => {
  for (const b of a.rig.bones) if (FIT[id]?.test(b.id)) { b.lengthM = r4(b.lengthM * f); if (b.mass?.comX !== undefined) b.mass.comX = r4(b.mass.comX * f); }
  for (const b of a.rig.bones) if (b.parent && FIT[id]?.test(b.parent)) b.x = r4(b.x * f);
  for (const an of a.anchors) if (FIT[id]?.test(an.bone)) an.x = r4(an.x * f);
  for (const den of ["50", "100", "200", "500"]) { const rep = a.representations[den]; if (rep.status !== "authored") continue; for (const v of ["elevation", "front", "plan"]) for (const s of rep.geometry[v]) if (FIT[id]?.test(s.bone)) transformShape(s, { sx: f, tx: 0, sy: 1, ty: 0 }); }
};
const fitErr = () => { const D2 = a.dimensions; const st = stateAt(a, 100); const e = three.extentOf(st.shapes.elevation), fr = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan); return Math.abs(e.maxX - e.minX - D2.widthM) / D2.widthM + Math.abs(fr.maxX - fr.minX - D2.depthM) / D2.depthM + Math.abs(p.maxX - p.minX - D2.widthM) / D2.widthM + Math.abs(p.maxY - p.minY - D2.depthM) / D2.depthM + 2 * Math.abs(e.maxY - e.minY - D2.heightM) / D2.heightM; };
if (FIT[id] && process.env.FIT) {
  let best = 1, be = fitErr(); const snapshot = JSON.stringify(a); let applied = 1;
  for (let k = 1; k <= 14; k++) { const f = 1 + k * 0.005; const keep = JSON.parse(snapshot); Object.assign(a, keep); scaleBones(f); const er = fitErr(); console.log("fit", f.toFixed(3), er.toFixed(4)); if (er < be) { be = er; best = f; } }
  Object.assign(a, JSON.parse(snapshot)); scaleBones(best); console.log("fit best", best, be.toFixed(4)); void applied;
}
const known = new Map([[a.id, a]]);
const issues = validateAsset(a, { assets: known, environments: new Set<string>() }).filter((i: any) => /lateral|extent|solid|print|thin/i.test(i.code)).map((i: any) => `${i.code}: ${i.message.en.slice(0, 200)}`);
const D = a.dimensions;
for (const den of [50, 100, 200, 500]) { if (a.representations[String(den)].status !== "authored") continue; const st = stateAt(a, den); const e = three.extentOf(st.shapes.elevation), f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan); console.log(id, den, `el ${(e.maxX - e.minX).toFixed(3)}x${(e.maxY - e.minY).toFixed(3)} fr ${(f.maxX - f.minX).toFixed(3)}x${(f.maxY - f.minY).toFixed(3)} pl ${(p.maxX - p.minX).toFixed(3)}x${(p.maxY - p.minY).toFixed(3)} | dims ${D.widthM}x${D.depthM}x${D.heightM}`); }
console.log(id, "shapes written", n, "validator issues", issues.length); issues.slice(0, 8).forEach((m: string) => console.log("  ", m));
if (write) save(x);
if (process.env.DBG) { const { boxes } = await import("./lib.ts"); for (const den of [100, 200]) { const bx = boxes(stateAt(a, den)).plan; console.log(den, Object.entries<any>(bx).map(([b, v]) => [b, v[0], v[1]]).sort((p: any, q: any) => p[1] - q[1]).slice(0, 2).map((r: any) => `${r[0]} ${r[1].toFixed(2)}`).join(" "), "|", Object.entries<any>(bx).sort((p: any, q: any) => q[1][1] - p[1][1]).slice(0, 2).map(([b, v]: any) => `${b} ${v[1].toFixed(2)}`).join(" ")); } }
if (process.env.DBG2) { const st = stateAt(a, 100); for (const s of st.shapes.plan) if (s.shape.bone === process.env.DBG2) console.log(s.shape.id, s.rings.length, JSON.stringify(s.rings[0]?.slice(0, 3)), JSON.stringify(s.shape).slice(0, 200)); const el = a.representations["100"].geometry.elevation.find((s: any) => s.bone === process.env.DBG2); console.log(JSON.stringify(el).slice(0, 400)); }
if (process.env.EMPTY) for (const den of [50, 100, 200, 500]) { if (a.representations[String(den)].status !== "authored") continue; const st = stateAt(a, den); for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) if (s.shape.solid && s.rings.length === 0) console.log("EMPTY", den, v, s.shape.id, s.shape.solid.kind); }
if (process.env.THICK) { for (const t of [0.0005, 0.001, 0.004, 0.01, 0.03]) { for (const v of ["elevation", "front", "plan"]) for (const s of a.representations["100"].geometry[v]) if (s.bone === "leaf-R1-b" && s.solid) { s.solid.z0 = -t / 2; s.solid.z1 = t / 2; } const st = stateAt(a, 100); console.log("thickness", t, "plan rings", st.shapes.plan.filter((s: any) => s.shape.bone === "leaf-R1-b").map((s: any) => s.rings.length + ":" + (s.rings[0]?.length ?? 0)).join(","), "front", st.shapes.front.filter((s: any) => s.shape.bone === "leaf-R1-b").map((s: any) => s.rings.length).join(",")); } }
if (process.env.SWAP) { const g = a.representations["100"].geometry; const src = a.representations["200"].geometry.elevation.find((s: any) => s.id === "leaf-R1-b"); for (const v of ["elevation", "front", "plan"]) for (const s of g[v]) if (s.bone === "leaf-R1-b") { s.points = src.points.filter((p: number[], i: number, all: number[][]) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]); } const st = stateAt(a, 100); console.log("swap plan rings", st.shapes.plan.filter((s: any) => s.shape.bone === "leaf-R1-b").map((s: any) => s.rings.length)); console.log(JSON.stringify(g.plan.filter((s: any) => s.bone === "leaf-R1-b").map((s: any) => [s.id, s.kind, s.solid]))); }
if (process.env.VAR) { const g = a.representations["100"].geometry; const orig = a.representations["100"].geometry.elevation.find((s: any) => s.id === "leaf-R1-b").points.filter((p: number[], i: number, all: number[][]) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]); console.log("npts", orig.length, JSON.stringify(orig.slice(0, 2)), JSON.stringify(orig.slice(-3)));
  const variants: Record<string, number[][]> = { noKnee: orig.slice(1, -1), drop2: orig.slice(0, -2), dropFirst: orig.slice(1) };
  for (const [name, pts] of Object.entries(variants)) { for (const v of ["elevation", "front", "plan"]) for (const s of g[v]) if (s.bone === "leaf-R1-b") s.points = pts; const st = stateAt(a, 100); console.log(name, st.shapes.plan.filter((s: any) => s.shape.bone === "leaf-R1-b").map((s: any) => s.rings.length)); } }
