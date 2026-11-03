import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries, extentOf, toleranceM } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const only = process.argv[2] && process.argv[2] !== "-" ? process.argv[2] : undefined;
const outFile = process.argv[3] ?? "audit.json";
const VIEWS = ["elevation", "front", "plan"];
const cats = ["🪑️objects", "🛋️furniture"];
const f3 = (n: number) => +n.toFixed(3);
const ctr = (sh: any) => {
  const p = sh.rings.flat();
  if (!p.length) return null;
  return { x: p.reduce((t: number, q: any) => t + q.x, 0) / p.length, y: p.reduce((t: number, q: any) => t + q.y, 0) / p.length, minX: Math.min(...p.map((q: any) => q.x)), maxX: Math.max(...p.map((q: any) => q.x)), minY: Math.min(...p.map((q: any) => q.y)), maxY: Math.max(...p.map((q: any) => q.y)) };
};
const segDist = (px: number, py: number, a: any, b: any) => {
  const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy;
  const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((px - a.x) * dx + (py - a.y) * dy) / l));
  return Math.hypot(px - (a.x + t * dx), py - (a.y + t * dy));
};
const inside = (px: number, py: number, ring: any[]) => {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i].y > py) !== (ring[j].y > py) && px < ((ring[j].x - ring[i].x) * (py - ring[i].y)) / (ring[j].y - ring[i].y) + ring[i].x) c = !c;
  return c;
};
const nearest = (shapes: any[], px: number, py: number) => {
  let best = Infinity;
  for (const sh of shapes) for (const ring of sh.rings) {
    if (sh.closed && inside(px, py, ring)) return 0;
    for (let i = 0; i + 1 < ring.length; i++) best = Math.min(best, segDist(px, py, ring[i], ring[i + 1]));
    if (ring.length === 1) best = Math.min(best, Math.hypot(px - ring[0].x, py - ring[0].y));
  }
  return best;
};
const out: any = {};
for (const cat of cats) {
  const dir = `${L}/🗂️catalog/${cat}`;
  for (const slug of readdirSync(dir).sort()) {
    if (only && slug !== only) continue;
    const a = JSON.parse(readFileSync(join(dir, slug, "🔣️.json"), "utf8")).asset;
    const bones = new Set(a.rig.bones.map((b: any) => b.id));
    const rec: any = { cat, dims: a.dimensions, scales: {}, symbols: (a.symbols ?? []).map((s: any) => ({ id: s.id, views: VIEWS.filter((v) => (s.geometry[v] ?? []).length) })), poses: a.poses.map((p: any) => p.id), clips: a.clips.map((c: any) => c.id), anchors: a.anchors.map((x: any) => `${x.id}:${x.kind}`) };
    const use = JSON.stringify(a.poses) + JSON.stringify(a.clips);
    rec.abd = (use.match(/abductionDeg/g) ?? []).length;
    rec.twist = (use.match(/twistDeg/g) ?? []).length;
    rec.boneZ = a.rig.bones.filter((b: any) => b.z).map((b: any) => `${b.id}:${b.z}`).join(" ");
    const emitAnchors = new Set<string>((a.thermal?.emitters ?? []).flatMap((e: any) => [e.anchor, ...(e.anchors ?? [])]));
    rec.emitters = [...emitAnchors];
    for (const [k, r] of Object.entries<any>(a.representations)) {
      if (r.status !== "authored") { rec.scales[k] = { status: "unavailable", alt: r.alternatives, reason: r.reason?.en?.slice(0, 80) }; continue; }
      const s: any = { counts: {}, unbound: [], idmiss: [], mism: {}, dim: {}, motion: {}, anchorMiss: {}, range: r.validRange };
      for (const v of VIEWS) {
        const g = r.geometry[v] ?? [];
        s.counts[v] = g.length;
        for (const sh of g) if (!bones.has(sh.bone)) s.unbound.push(`${v}/${sh.id}:${sh.bone}`);
      }
      const idsOf = (v: string) => new Map((r.geometry[v] ?? []).filter((x: any) => x.role !== "ground").map((x: any) => [x.id, x.bone]));
      const e = idsOf("elevation");
      for (const v of ["front", "plan"]) {
        const m = idsOf(v);
        for (const [id, b] of e) { if (!m.has(id)) s.idmiss.push(`${v} lacks ${id}`); else if (m.get(id) !== b) s.idmiss.push(`${v}/${id} bone ${m.get(id)} != ${b}`); }
        for (const [id] of m) if (!e.has(id)) s.idmiss.push(`${v} extra ${id}`);
      }
      const tol = toleranceM(a);
      s.tol = f3(tol);
      let worst: any = { eh: 0, fx: 0, pl: 0 };
      let bad = 0, total = 0;
      const sel = { kind: "scale", scale: { denominator: r.denominator } };
      let rest: any;
      try { rest = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel); } catch { rest = undefined; }
      for (const { name, query } of queries(a)) {
        let st: any;
        try { st = evaluate(a, query, sel); } catch (err) { s.mism.evalFail = (s.mism.evalFail ?? 0) + 1; continue; }
        const ex = extentOf(st.shapes.elevation), fx = extentOf(st.shapes.front), px = extentOf(st.shapes.plan);
        if (!ex || !fx || !px) { s.mism.empty = (s.mism.empty ?? 0) + 1; continue; }
        total++;
        const d1 = Math.max(Math.abs(ex.minY - fx.minY), Math.abs(ex.maxY - fx.maxY));
        const d2 = Math.max(Math.abs(ex.minX - px.minX), Math.abs(ex.maxX - px.maxX));
        const d3 = Math.max(Math.abs(fx.minX - px.minY), Math.abs(fx.maxX - px.maxY));
        worst.eh = Math.max(worst.eh, d1); worst.fx = Math.max(worst.fx, d3); worst.pl = Math.max(worst.pl, d2);
        if (d1 > tol || d2 > tol || d3 > tol) { bad++; if (!s.mism.first) s.mism.first = `${name} h:${f3(d1)} x:${f3(d2)} z:${f3(d3)}`; }
        if (name === `pose ${a.presentation.defaultPose}`) {
          s.dim = { e: [ex.maxX - ex.minX, ex.maxY - ex.minY].map(f3), f: [fx.maxX - fx.minX, fx.maxY - fx.minY].map(f3), p: [px.maxX - px.minX, px.maxY - px.minY].map(f3), fz: [f3(fx.minX), f3(fx.maxX)], pz: [f3(px.minY), f3(px.maxY)] };
          for (const aid of emitAnchors) {
            const an = st.anchors[aid]; if (!an) continue;
            const wx = an.at.x, wy = an.at.y;
            const dE = nearest(st.shapes.elevation, wx, wy), dF = nearest(st.shapes.front, 0, wy), dP = nearest(st.shapes.plan, wx, 0);
            s.anchorMiss[aid] = { e: f3(dE), f: f3(dF), p: f3(dP) };
          }
        }
        if (rest) {
          for (const [id, bb] of Object.entries<any>(st.bones)) {
            const r0 = rest.bones[id];
            if (!r0) continue;
            const dx = Math.max(Math.abs(bb.tip.x - r0.tip.x), Math.abs(bb.origin.x - r0.origin.x));
            const dy = Math.max(Math.abs(bb.tip.y - r0.tip.y), Math.abs(bb.origin.y - r0.origin.y));
            if (dx < 0.02 && dy < 0.02 && Math.abs(bb.offset.abductionDeg) + Math.abs(bb.offset.twistDeg) < 1e-6) continue;
            const mv = (view: string, axis: "x" | "y") => {
              const cur = st.shapes[view].filter((q: any) => q.shape.bone === id), ref = rest.shapes[view].filter((q: any) => q.shape.bone === id);
              let m = 0;
              for (const c of cur) { const r1 = ref.find((q: any) => q.shape.id === c.shape.id); if (!r1) continue; const A = ctr(c), B = ctr(r1); if (!A || !B) continue; m = Math.max(m, Math.abs(A[axis] - B[axis]), Math.abs(A[axis === "x" ? "maxX" : "maxY"] - B[axis === "x" ? "maxX" : "maxY"]), Math.abs(A[axis === "x" ? "minX" : "minY"] - B[axis === "x" ? "minX" : "minY"])); }
              return { m, n: cur.length };
            };
            const eX = mv("elevation", "x"), eY = mv("elevation", "y"), pX = mv("plan", "x"), fY = mv("front", "y");
            const key = id;
            const slot = (s.motion[key] ??= { dx: 0, dy: 0, eX: 0, eY: 0, pX: 0, fY: 0, nE: eX.n, nP: pX.n, nF: fY.n, worstPlan: "", worstFront: "" });
            slot.dx = Math.max(slot.dx, dx); slot.dy = Math.max(slot.dy, dy);
            slot.eX = Math.max(slot.eX, eX.m); slot.eY = Math.max(slot.eY, eY.m); slot.pX = Math.max(slot.pX, pX.m); slot.fY = Math.max(slot.fY, fY.m);
            if (eX.m > 0.03 && pX.m < eX.m * 0.5 - 0.01 && !slot.worstPlan) slot.worstPlan = `${name}: elevX ${f3(eX.m)} planX ${f3(pX.m)}`;
            if (eY.m > 0.03 && fY.m < eY.m * 0.5 - 0.01 && !slot.worstFront) slot.worstFront = `${name}: elevY ${f3(eY.m)} frontY ${f3(fY.m)}`;
          }
        }
      }
      s.mism.bad = `${bad}/${total}`;
      s.mism.worst = Object.fromEntries(Object.entries(worst).map(([kk, v]: any) => [kk, f3(v)]));
      for (const key of Object.keys(s.motion)) { const m = s.motion[key]; for (const q of Object.keys(m)) if (typeof m[q] === "number") m[q] = f3(m[q]); if (!m.worstPlan && !m.worstFront) delete s.motion[key]; }
      rec.scales[k] = s;
    }
    out[slug] = rec;
    console.error(slug, "done");
  }
}
writeFileSync(outFile, JSON.stringify(out, null, 1));
