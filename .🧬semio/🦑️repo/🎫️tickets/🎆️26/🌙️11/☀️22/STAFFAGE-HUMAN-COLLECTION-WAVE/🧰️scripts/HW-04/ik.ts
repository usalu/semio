/** 🤲️ Pose solver on the product's own chain (`solveChain`, ground `auto` on the listed contacts): damped least squares with a numeric Jacobian over chosen bone offsets, clamped to the joint limits, for point, coincidence and direction targets.
 *  bun ik.ts <spec.json> <out.json>; spec = { asset, problems: [{ id, contacts, base, vars: [[bone, key]], targets: [...] }] } */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { solveChain } = await import(join(S, "🔨️modules/🦴️rig/🟦️.ts"));
type V = [number, number, number];
type Off = Record<string, { angleDeg?: number; abductionDeg?: number; twistDeg?: number }>;
const [specPath, outPath] = process.argv.slice(2);
const spec = await Bun.file(specPath!).json();
const asset = (await Bun.file(spec.asset).json()).asset;
const rig = asset.rig;
const bones = new Map<string, any>(rig.bones.map((b: any) => [b.id, b]));
const anchors = new Map<string, any>(asset.anchors.map((a: any) => [a.id, a]));
const full = (o: Off) => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, { angleDeg: v.angleDeg ?? 0, abductionDeg: v.abductionDeg ?? 0, twistDeg: v.twistDeg ?? 0, dx: 0, dy: 0 }]));
const pt = (f: any, p: number[]): V => [0, 1, 2].map((k) => f.o[k] + p[0]! * f.u[k] + p[1]! * f.v[k] + (p[2] ?? 0) * f.w[k]) as V;
function frames(off: Off, contacts: string[]) {
  const w = solveChain(rig, full(off), { x: 0, y: 0, angleDeg: 0 });
  let low = Infinity;
  for (const c of contacts) { const a = anchors.get(c); low = Math.min(low, pt(w[a.bone].frame, [a.x, a.y, a.z ?? 0])[1]); }
  if (!Number.isFinite(low)) low = 0;
  const g: Record<string, any> = {};
  for (const [id, s] of Object.entries<any>(w)) g[id] = { o: [s.frame.o[0], s.frame.o[1] - low, s.frame.o[2]], u: s.frame.u, v: s.frame.v, w: s.frame.w };
  return g;
}
function limits(bone: string, key: string): [number, number] {
  const j = bones.get(bone).joint ?? {};
  if (key === "angleDeg") return [j.minDeg ?? -180, j.maxDeg ?? 180];
  if (key === "twistDeg") return [j.twistMinDeg ?? -180, j.twistMaxDeg ?? 180];
  return [-180, 180];
}
function solveLinear(a: number[][], b: number[]): number[] {
  const n = b.length, m = a.map((r, i) => [...r, b[i]!]);
  for (let c = 0; c < n; c++) {
    let p = c; for (let r = c + 1; r < n; r++) if (Math.abs(m[r]![c]!) > Math.abs(m[p]![c]!)) p = r;
    [m[c], m[p]] = [m[p]!, m[c]!];
    const d = Math.abs(m[c]![c]!) < 1e-12 ? 1e-12 : m[c]![c]!;
    for (let r = 0; r < n; r++) if (r !== c) { const f = m[r]![c]! / d; for (let k = c; k <= n; k++) m[r]![k]! -= f * m[c]![k]!; }
  }
  return m.map((r, i) => r[n]! / (Math.abs(r[i]!) < 1e-12 ? 1e-12 : r[i]!));
}
const out: any[] = [];
for (const pr of spec.problems) {
  const vars: [string, string, number, number][] = pr.vars.map(([b, k, lo, hi]: any) => { const [a, z] = limits(b, k); return [b, k, Math.max(a, lo ?? a), Math.min(z, hi ?? z)]; });
  const base: Off = structuredClone(pr.base);
  const get = (o: Off, b: string, k: string) => (o[b] as any)?.[k] ?? 0;
  const apply = (q: number[]): Off => { const o: Off = structuredClone(base); vars.forEach(([b, k], i) => { o[b] = { ...(o[b] ?? {}), [k]: q[i] }; }); return o; };
  const q0 = vars.map(([b, k]) => get(base, b, k));
  const resid = (q: number[]): number[] => {
    const f = frames(apply(q), pr.contacts);
    const r: number[] = [];
    for (const t of pr.targets) {
      const w = t.w ?? 1;
      if (t.type === "point") { const p = pt(f[t.bone], t.p); for (let k = 0; k < 3; k++) if (t.world[k] !== null) r.push(w * (p[k] - t.world[k])); }
      else if (t.type === "coincide") { const a = pt(f[t.a.bone], t.a.p), b = pt(f[t.b.bone], t.b.p); for (let k = 0; k < 3; k++) r.push(w * (a[k] - b[k])); }
      else if (t.type === "dir") { const ax = f[t.bone][t.axis ?? "u"]; const n = Math.hypot(...t.world); for (let k = 0; k < 3; k++) r.push(w * (ax[k] - t.world[k] / n)); }
      else if (t.type === "align") { const x = f[t.a.bone][t.a.axis], y = f[t.b.bone][t.b.axis]; for (let k = 0; k < 3; k++) r.push(w * (x[k] - (t.sign ?? 1) * y[k])); }
      else if (t.type === "reg") vars.forEach(([b, k], i) => { if (!t.bones || t.bones.includes(b)) r.push(w * (q[i]! - q0[i]!) / 57.3); });
    }
    return r;
  };
  const descend = (start: number[]): [number[], number[], number] => {
    let q = start.map((v, i) => Math.min(Math.max(v, vars[i]![2]), vars[i]![3]));
    let stall = 0, lambda = 1e-6, r = resid(q), cost = r.reduce((s, x) => s + x * x, 0);
    for (let it = 0; it < 400 && cost > 1e-14; it++) {
      const J = q.map((_, j) => { const p = [...q]; p[j]! += 0.01; return resid(p).map((x, i) => (x - r[i]!) / 0.01); });
      const n = q.length;
      const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => r.reduce((s, _x, k) => s + J[i]![k]! * J[j]![k]!, 0) + (i === j ? lambda : 0)));
      const g = Array.from({ length: n }, (_, i) => -r.reduce((s, x, k) => s + J[i]![k]! * x, 0));
      const raw = solveLinear(A, g); const big = Math.max(...raw.map(Math.abs)); const step = raw.map((x) => (big > 15 ? (x * 15) / big : x));
      const qn = q.map((v, i) => Math.min(Math.max(v + step[i]!, vars[i]![2]), vars[i]![3]));
      const rn = resid(qn), cn = rn.reduce((s, x) => s + x * x, 0);
      if (cn < cost) { const gain = cost - cn; q = qn; r = rn; cost = cn; if (gain < 1e-5 * cost + 1e-14) stall++; else stall = 0; if (stall > 8) break; lambda = Math.max(lambda / 3, 1e-12); } else lambda *= 4;
      if (lambda > 1e3) break;
    }
    return [q, r, cost];
  };
  let seed = 12345;
  const rnd = () => { seed = (seed * 1103515245 + 12345) % 2147483648; return seed / 2147483648; };
  let [q, r, cost] = descend(q0);
  for (let k = 0; k < (pr.restarts ?? 8) && cost > (pr.good ?? 2e-5); k++) {
    const start = q0.map((v, i) => { const [lo, hi] = [vars[i]![2], vars[i]![3]]; return Math.min(hi, Math.max(lo, v + (rnd() - 0.5) * Math.min(160, hi - lo))); });
    const [q2, r2, c2] = descend(start);
    if (c2 < cost) [q, r, cost] = [q2, r2, c2];
  }
  const o = apply(q.map((v) => Math.round(v * 1000) / 1000));
  const f = frames(o, pr.contacts);
  const report = pr.targets.filter((t: any) => t.type !== "reg").map((t: any) => {
    if (t.type === "point") { const p = pt(f[t.bone], t.p); return { t: t.type, bone: t.bone, err: Math.hypot(...[0, 1, 2].map((k) => (t.world[k] === null ? 0 : p[k] - t.world[k]))), at: p }; }
    if (t.type === "coincide") { const a = pt(f[t.a.bone], t.a.p), b = pt(f[t.b.bone], t.b.p); return { t: t.type, a: t.a.bone, b: t.b.bone, err: Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]), at: a }; }
    if (t.type === "align") { const x = f[t.a.bone][t.a.axis], y = f[t.b.bone][t.b.axis]; return { t: t.type, a: t.a.bone, err: Math.hypot(...[0, 1, 2].map((k) => x[k] - (t.sign ?? 1) * y[k])) }; }
    const ax = f[t.bone][t.axis ?? "u"]; const n = Math.hypot(...t.world); return { t: t.type, bone: t.bone, err: Math.hypot(...[0, 1, 2].map((k) => ax[k] - t.world[k] / n)) };
  });
  const atLimit = vars.filter(([b, k, lo, hi], i) => Math.abs(q[i]! - lo) < 0.01 || Math.abs(q[i]! - hi) < 0.01).map(([b, k]) => `${b}.${k}`);
  const probes = Object.fromEntries((pr.probes ?? []).map((p: any) => [p.id, pt(f[p.bone], p.p).map((x) => Math.round(x * 10000) / 10000)]));
  out.push({ id: pr.id, bones: o, cost, report, atLimit, probes });
  console.log(`${pr.id}: cost ${cost.toExponential(2)} ${report.map((x: any) => `${x.t}:${x.bone ?? x.a}:${(x.err * 1000).toFixed(1)}mm`).join(" ")}${atLimit.length ? " LIMIT " + atLimit.join(",") : ""}`);
}
await Bun.write(outPath!, JSON.stringify(out, null, 1));
