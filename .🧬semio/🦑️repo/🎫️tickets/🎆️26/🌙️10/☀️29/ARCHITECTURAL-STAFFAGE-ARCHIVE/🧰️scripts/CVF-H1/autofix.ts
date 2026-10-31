/** CVF-H1 autofix: measures the print suitability per scale over the named poses and nudges the geometry of the offending shapes locally (bump toward the neighbour to join, or away to clear)
 *  until no gap/sliver violation remains. Usage: autofix.ts --file f.json --scales 500,200 --iterations 30 [--dry] [--cap 0.15] [--skip a,b] */
import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { DEFAULT_PRINT_PROFILE, featureWidthMm, gapsBetween, whiteSliver } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { outlineRuns, prepare, sourceOf } = await import(`${S}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, scales: { type: "string", default: "500,200,100,50" }, iterations: { type: "string", default: "30" }, dry: { type: "boolean", default: false }, cap: { type: "string", default: "0.2" }, skip: { type: "string", default: "" }, join: { type: "string", default: "0.04" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const profile = DEFAULT_PRINT_PROFILE;
const skip = new Set(values.skip!.split(",").filter(Boolean));
type V = [number, number];
const fmt = (n: number): string => String(Math.round(n * 1e5) / 1e5);

type Seg = { c: string; p: V[] };
function parsePath(d: string): Seg[] {
  const tok = d.match(/[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?/g)!;
  const segs: Seg[] = [];
  let i = 0, cur: V = [0, 0], start: V = [0, 0], cmd = "";
  const num = (): number => Number(tok[i++]);
  while (i < tok.length) {
    if (/[A-Za-z]/.test(tok[i]!)) cmd = tok[i++]!;
    const rel = cmd === cmd.toLowerCase(), C = cmd.toUpperCase();
    const abs = (x: number, y: number): V => (rel ? [cur[0] + x, cur[1] + y] : [x, y]);
    if (C === "M") { const p = abs(num(), num()); segs.push({ c: "M", p: [p] }); cur = start = p; cmd = rel ? "l" : "L"; }
    else if (C === "L") { const p = abs(num(), num()); segs.push({ c: "L", p: [p] }); cur = p; }
    else if (C === "H") { const x = num(); const p: V = [rel ? cur[0] + x : x, cur[1]]; segs.push({ c: "L", p: [p] }); cur = p; }
    else if (C === "V") { const y = num(); const p: V = [cur[0], rel ? cur[1] + y : y]; segs.push({ c: "L", p: [p] }); cur = p; }
    else if (C === "C") { const a = abs(num(), num()), b = abs(num(), num()), p = abs(num(), num()); segs.push({ c: "C", p: [a, b, p] }); cur = p; }
    else if (C === "Q") { const a = abs(num(), num()), p = abs(num(), num()); segs.push({ c: "Q", p: [a, p] }); cur = p; }
    else if (C === "Z") { segs.push({ c: "Z", p: [] }); cur = start; }
    else throw new Error(`path command ${cmd} unsupported`);
  }
  return segs;
}
const writePath = (segs: Seg[]): string => segs.map((s) => (s.c === "Z" ? "Z" : `${s.c} ${s.p.map((q) => `${fmt(q[0])} ${fmt(q[1])}`).join(" ")}`)).join(" ");

/** local points of a shape (every control and anchor point) */
function pointsOf(sh: any): { get: () => V[]; move: (f: (p: V) => V) => void } {
  if (sh.kind === "path") {
    const segs = parsePath(sh.d);
    return { get: () => segs.flatMap((s) => s.p), move: (f) => { for (const s of segs) s.p = s.p.map(f); sh.d = writePath(segs); } };
  }
  if (sh.kind === "poly") return { get: () => sh.points as V[], move: (f) => { sh.points = (sh.points as V[]).map((p) => f([p[0], p[1]])).map((p) => [Math.round(p[0] * 1e5) / 1e5, Math.round(p[1] * 1e5) / 1e5]); } };
  return { get: () => [], move: () => {} };
}

function inv(m: readonly number[]): number[] { const d = m[0]! * m[3]! - m[1]! * m[2]!; const a = m[3]! / d, b = -m[1]! / d, c = -m[2]! / d, dd = m[0]! / d; return [a, b, c, dd, -(a * m[4]! + c * m[5]!), -(b * m[4]! + dd * m[5]!)]; }
const apply = (m: number[], p: V): V => [m[0]! * p[0] + m[2]! * p[1] + m[4]!, m[1]! * p[0] + m[3]! * p[1] + m[5]!];
const rotOnly = (m: number[], v: V): V => [m[0]! * v[0] + m[2]! * v[1], m[1]! * v[0] + m[3]! * v[1]];

function nearest(A: V[], B: V[]): { a: V; b: V; d: number } {
  const seg = (ring: V[]): [V, V][] => ring.slice(1).map((p, i) => [ring[i]!, p] as [V, V]);
  let best = { a: [0, 0] as V, b: [0, 0] as V, d: Infinity };
  const near = (p: V, q0: V, q1: V): V => { const dx = q1[0] - q0[0], dy = q1[1] - q0[1]; const l2 = dx * dx + dy * dy; const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p[0] - q0[0]) * dx + (p[1] - q0[1]) * dy) / l2)); return [q0[0] + t * dx, q0[1] + t * dy]; };
  for (const [s0, s1] of seg(A)) for (const p of [s0, s1]) for (const [t0, t1] of seg(B)) { const q = near(p, t0, t1); const d = Math.hypot(p[0] - q[0], p[1] - q[1]); if (d < best.d) best = { a: p, b: q, d }; }
  for (const [t0, t1] of seg(B)) for (const p of [t0, t1]) for (const [s0, s1] of seg(A)) { const q = near(p, s0, s1); const d = Math.hypot(p[0] - q[0], p[1] - q[1]); if (d < best.d) best = { a: q, b: p, d }; }
  return best;
}

type Hit = { scale: number; pose: string; kind: "gap" | "sliver" | "feature"; a: string; b: string; mm: number; at: V };

function measure(asset: any, d: number, only?: string[]): Hit[] {
  const hits: Hit[] = [];
  const k = 1000 / d;
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    const state = evaluate(structuredClone(asset), { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    for (const p of prepared) if (p.shape.role !== "ground" && p.closed && p.shape.filled) {
      const w = featureWidthMm(p.rings, profile.minFeatureMm);
      if (w < profile.minFeatureMm - 1e-9) hits.push({ scale: d, pose: pose.id, kind: "feature", a: p.shape.id, b: "", mm: w, at: [0, 0] });
    }
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), profile.minGapMm);
    if (sl !== null) {
      const at: V = [sl.at.x, sl.at.y];
      const cand = paper.map((s) => ({ id: s.id, d: nearest([at, at], s.rings[0].map((q: any) => [q.x, q.y] as V)).d })).sort((x, y) => x.d - y.d);
      const seen = new Set<string>(); const two: string[] = [];
      for (const c of cand) if (!seen.has(c.id)) { seen.add(c.id); two.push(c.id); if (two.length === 2) break; }
      hits.push({ scale: d, pose: pose.id, kind: "sliver", a: two[0] ?? "", b: two[1] ?? "", mm: sl.widthMm, at });
    }
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < profile.minGapMm - 1e-9) hits.push({ scale: d, pose: pose.id, kind: "gap", a: g.a, b: g.b, mm: g.gapMm, at: [g.at.x, g.at.y] });
  }
  return hits;
}

function runsOf(asset: any, d: number, poseId: string): { state: any; runs: Map<string, V[][]>; pens: Map<string, number> } {
  const state = evaluate(structuredClone(asset), { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: d } });
  const prepared = prepare(state, "elevation", sourceOf(state), 1000 / d);
  const runs = new Map<string, V[][]>(); const pens = new Map<string, number>();
  for (const { p, run } of outlineRuns(prepared, true)) { const arr = runs.get(p.shape.id) ?? []; arr.push(run.points.map((q: any) => [q.x, q.y] as V)); runs.set(p.shape.id, arr); pens.set(p.shape.id, p.weightMm); }
  return { state, runs, pens };
}

function bump(rep: any, asset: any, d: number, hit: Hit, id: string, towards: V, centre: V, amountMm: number): boolean {
  const sh = rep.geometry.elevation.find((s: any) => s.id === id);
  if (!sh || skip.has(id)) return false;
  const { state } = runsOf(asset, d, hit.pose);
  const k = 1000 / d;
  const m = inv(state.bones[sh.bone].world as number[]);
  const lc = apply(m, [centre[0] / k, -centre[1] / k]);
  const dirW: V = [towards[0] / k, -towards[1] / k];
  const len = Math.hypot(dirW[0], dirW[1]) || 1;
  const dirL = rotOnly(m, [dirW[0] / len, dirW[1] / len]);
  const amt = amountMm / k;
  const R = Math.max(0.6, 0.35 * Math.max(amountMm, 0.3) / 0.15) / k;
  const pts = pointsOf(sh);
  if (sh.kind === "ellipse") { sh.rx += amt; sh.ry += amt; return true; }
  if (sh.kind === "capsule") { sh.r0 += amt; sh.r1 += amt; return true; }
  if (pts.get().length === 0) return false;
  pts.move((p) => { const dist = Math.hypot(p[0] - lc[0], p[1] - lc[1]); if (dist >= R) return p; const w = Math.cos((dist / R) * Math.PI / 2) ** 2; return [p[0] + dirL[0] * amt * w, p[1] + dirL[1] * amt * w]; });
  return true;
}

const cap = Number(values.cap), joinMargin = Number(values.join);
let total = 0;
for (const d of values.scales!.split(",").map(Number)) {
  const rep = doc.asset.representations[String(d)];
  if (rep.status !== "authored") continue;
  const pens = rep.pens;
  for (let it = 0; it < Number(values.iterations); it++) {
    const hits = measure(doc.asset, d);
    const gaps = hits.filter((h) => h.kind !== "feature");
    const feats = hits.filter((h) => h.kind === "feature");
    const worst = new Map<string, Hit>();
    for (const h of gaps) { const key = [h.a, h.b].sort().join("|"); const o = worst.get(key); if (!o || h.mm < o.mm) worst.set(key, h); }
    console.log(`1:${d} iteration ${it}: ${worst.size} gap pairs, ${new Set(feats.map((f) => f.a)).size} feature shapes`);
    if (worst.size === 0) break;
    let moved = 0;
    for (const h of worst.values()) {
      if (!h.a || !h.b) continue;
      const { runs, pens: pw } = runsOf(doc.asset, d, h.pose);
      const A = (runs.get(h.a) ?? []).flat(), B = (runs.get(h.b) ?? []).flat();
      if (A.length < 2 || B.length < 2) continue;
      const n = nearest(A, B);
      const pa = pw.get(h.a) ?? 0.1, pb = pw.get(h.b) ?? 0.1;
      const ink = n.d - (pa + pb) / 2;
      const dirAB: V = [n.b[0] - n.a[0], n.b[1] - n.a[1]];
      const join = ink + joinMargin, open = profile.minGapMm - ink + joinMargin;
      const mid: V = [(n.a[0] + n.b[0]) / 2, (n.a[1] + n.b[1]) / 2];
      if (join <= open || h.kind === "sliver") {
        const amount = Math.min(join, cap);
        const half = amount / 2;
        const okA = bump(rep, doc.asset, d, h, h.a, dirAB, n.a, half);
        const okB = bump(rep, doc.asset, d, h, h.b, [-dirAB[0], -dirAB[1]], n.b, okA ? half : amount);
        if (!okA && okB) bump(rep, doc.asset, d, h, h.b, [-dirAB[0], -dirAB[1]], n.b, half);
        moved += okA || okB ? 1 : 0;
      } else {
        const amount = Math.min(open, cap);
        const okA = bump(rep, doc.asset, d, h, h.a, [-dirAB[0], -dirAB[1]], n.a, amount / 2);
        const okB = bump(rep, doc.asset, d, h, h.b, dirAB, n.b, amount / 2);
        moved += okA || okB ? 1 : 0;
      }
    }
    total += moved;
    if (moved === 0) break;
  }
  const rest = measure(doc.asset, d);
  const pairs = new Map<string, Hit>();
  for (const h of rest) { const key = h.kind + ":" + [h.a, h.b].sort().join("+"); const o = pairs.get(key); if (!o || h.mm < o.mm) pairs.set(key, h); }
  console.log(`1:${d} remaining ${pairs.size}`);
  for (const h of pairs.values()) console.log(`   ${h.kind} ${h.a}${h.b ? " + " + h.b : ""} ${h.mm.toFixed(3)} @${h.at.map((v) => v.toFixed(2))} pose ${h.pose}`);
}
if (!values.dry) writeFileSync(values.file!, JSON.stringify(doc, null, 1) + "\n");
console.log(`done, ${total} nudges, ${values.dry ? "dry run" : "written"}`);
