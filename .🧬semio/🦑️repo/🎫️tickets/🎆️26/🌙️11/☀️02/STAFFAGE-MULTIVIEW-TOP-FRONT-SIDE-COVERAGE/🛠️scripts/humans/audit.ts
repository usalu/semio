import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries, extentOf, toleranceM } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const only = process.argv[2];
const dir = `${L}/🗂️catalog/👤️humans`;
const out: any = {};
const VIEWS = ["elevation", "front", "plan"];
for (const slug of readdirSync(dir).sort()) {
  if (only && slug !== only) continue;
  const a = JSON.parse(readFileSync(join(dir, slug, "🔣️.json"), "utf8")).asset;
  const bones = new Set(a.rig.bones.map((b: any) => b.id));
  const rec: any = { dims: a.dimensions, scales: {}, symbols: (a.symbols ?? []).map((s: any) => ({ id: s.id, views: VIEWS.filter((v) => (s.geometry[v] ?? []).length) })), poses: a.poses.length, clips: a.clips.length, abd: 0, issues: [] };
  const abdUse = JSON.stringify(a.poses) + JSON.stringify(a.clips);
  rec.abd = (abdUse.match(/abductionDeg/g) ?? []).length;
  rec.twist = (abdUse.match(/twistDeg/g) ?? []).length;
  rec.boneZ = a.rig.bones.filter((b: any) => b.z).map((b: any) => `${b.id}:${b.z}`).join(" ");
  for (const [k, r] of Object.entries<any>(a.representations)) {
    if (r.status !== "authored") { rec.scales[k] = { status: "unavailable", alt: r.alternatives, reason: r.reason?.en }; continue; }
    const s: any = { counts: {}, unbound: [], idmiss: [], handed: [], mism: {}, dim: {} };
    for (const v of VIEWS) {
      const g = r.geometry[v] ?? [];
      s.counts[v] = g.length;
      for (const sh of g) if (!bones.has(sh.bone)) s.unbound.push(`${v}/${sh.id}:${sh.bone}`);
    }
    const boneSet = (v: string) => new Set(r.geometry[v].filter((x: any) => x.role !== "ground").map((x: any) => x.bone));
    const be = boneSet("elevation");
    for (const v of ["front", "plan"]) { const m = boneSet(v); const miss = [...be].filter((x) => !m.has(x)); if (miss.length) s.idmiss.push(`${v} lacks bones ${miss.join(",")}`); }
    const tol = toleranceM(a);
    let worst: any = { eh: 0, fx: 0, pl: 0 };
    let bad = 0, total = 0;
    const q = queries(a);
    for (const { name, query } of q) {
      let st: any;
      try { st = evaluate(a, query, { kind: "scale", scale: { denominator: r.denominator } }); } catch (err) { s.mism.evalFail = (s.mism.evalFail ?? 0) + 1; continue; }
      const ex = extentOf(st.shapes.elevation), fx = extentOf(st.shapes.front), px = extentOf(st.shapes.plan);
      if (!ex || !fx || !px) { s.mism.empty = (s.mism.empty ?? 0) + 1; continue; }
      total++;
      const d1 = Math.max(Math.abs(ex.minY - fx.minY), Math.abs(ex.maxY - fx.maxY));
      const d2 = Math.max(Math.abs(ex.minX - px.minX), Math.abs(ex.maxX - px.maxX));
      const d3 = Math.max(Math.abs(fx.minX - px.minY), Math.abs(fx.maxX - px.maxY));
      worst.eh = Math.max(worst.eh, d1); worst.fx = Math.max(worst.fx, d3); worst.pl = Math.max(worst.pl, d2);
      if (d1 > tol || d2 > tol || d3 > tol) { bad++; if (!s.mism.first) s.mism.first = `${name} h:${d1.toFixed(3)} x:${d2.toFixed(3)} z:${d3.toFixed(3)}`; }
      if (name === `pose ${a.presentation.defaultPose}`) {
        s.dim = { e: [ex.maxX - ex.minX, ex.maxY - ex.minY].map((n) => +n.toFixed(3)), f: [fx.maxX - fx.minX, fx.maxY - fx.minY].map((n) => +n.toFixed(3)), p: [px.maxX - px.minX, px.maxY - px.minY].map((n) => +n.toFixed(3)), fz: [+fx.minX.toFixed(3), +fx.maxX.toFixed(3)], pz: [+px.minY.toFixed(3), +px.maxY.toFixed(3)] };
        for (const sh of st.shapes.front) {
          const id = sh.shape.id; const mt = /(?:^|-)(l|r|left|right)(?:-|$)/.exec(id) ?? /-(l|r)\d*$/.exec(id);
          if (!mt) continue; const side = mt[1][0] === "l" ? 1 : -1;
          const pts = sh.rings.flat(); if (!pts.length) continue;
          const cx = pts.reduce((t: number, p: any) => t + p.x, 0) / pts.length;
          const pp = st.shapes.plan.find((x: any) => x.shape.id === id);
          const cy = pp ? pp.rings.flat().reduce((t: number, p: any) => t + p.y, 0) / Math.max(1, pp.rings.flat().length) : NaN;
          if (cx * side < -0.01 || cy * side < -0.01) s.handed.push(`${id} frontX=${cx.toFixed(3)} planY=${cy.toFixed(3)}`);
        }
      }
    }
    s.mism.bad = `${bad}/${total}`; s.mism.worst = Object.fromEntries(Object.entries(worst).map(([k, v]: any) => [k, +v.toFixed(3)]));
    s.tol = +tol.toFixed(3);
    rec.scales[k] = s;
  }
  out[slug] = rec;
  console.error(slug, "done");
}
writeFileSync(process.argv[3] ?? "audit.json", JSON.stringify(out, null, 1));
