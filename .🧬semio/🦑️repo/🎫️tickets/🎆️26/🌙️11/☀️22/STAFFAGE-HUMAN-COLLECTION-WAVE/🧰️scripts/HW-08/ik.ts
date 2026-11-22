/** 🎯️ Small numerical inverse kinematics on the library's own `evaluate`: solves bone angles of a pose so that anchors or bone tips reach targets.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts */
import { evaluate, type J } from "./lib.ts";

export type Target = { kind: "tip"; bone: string; x?: number; y?: number; z?: number } | { kind: "anchor"; anchor: string; x?: number; y?: number };
export type Unknown = { bone: string; field: "angleDeg" | "abductionDeg" | "twistDeg" };

/** 📍️ Evaluates `bones` as a temporary pose of `a` with `contacts` (ground auto) and returns the state. */
export function stateOf(a: J, bones: J, contacts: string[], root?: J): J {
  const p = { id: "ik-probe", title: { en: "p", de: "p" }, description: { en: "p", de: "p" }, key: true, bones, ground: "auto", ...(root ? { root } : {}), contacts, thermal: a.poses[0]?.thermal ?? { regions: {}, emitters: {} }, loads: [] };
  const b = { ...a, poses: [...a.poses.filter((x: J) => x.id !== "ik-probe"), p] };
  return evaluate(b, { kind: "pose", pose: "ik-probe" });
}
function residual(s: J, targets: Target[]): number[] {
  const out: number[] = [];
  for (const t of targets) {
    const p = t.kind === "tip" ? { x: s.bones[t.bone].tip.x, y: s.bones[t.bone].tip.y, z: s.bones[t.bone].tip.z } : { x: s.anchors[t.anchor].at.x, y: s.anchors[t.anchor].at.y, z: s.anchors[t.anchor].z };
    for (const k of ["x", "y", "z"] as const) if ((t as J)[k] !== undefined) out.push(p[k] - (t as J)[k]);
  }
  return out;
}
/** 🔁️ Gauss-Newton with finite differences (degrees), damped; returns the solved bone offsets and the final residual norm. */
export function solve(a: J, bones: J, contacts: string[], unknowns: Unknown[], targets: Target[], iterations = 30, root?: J): { bones: J; error: number } {
  const cur = structuredClone(bones);
  const get = (u: Unknown) => cur[u.bone]?.[u.field] ?? 0;
  const set = (u: Unknown, v: number) => { cur[u.bone] = { ...(cur[u.bone] ?? {}), [u.field]: v }; };
  let res = residual(stateOf(a, cur, contacts, root), targets);
  for (let it = 0; it < iterations && Math.hypot(...res) > 2e-5; it++) {
    const jac: number[][] = res.map(() => []);
    for (const u of unknowns) {
      const v0 = get(u); set(u, v0 + 0.05);
      const r1 = residual(stateOf(a, cur, contacts, root), targets); set(u, v0);
      r1.forEach((x, i) => jac[i]!.push((x - res[i]!) / 0.05));
    }
    const n = unknowns.length;
    const jtj = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => jac.reduce((s, row) => s + row[i]! * row[j]!, 0) + (i === j ? 1e-6 : 0)));
    const jtr = Array.from({ length: n }, (_, i) => jac.reduce((s, row, k) => s + row[i]! * res[k]!, 0));
    const step = gauss(jtj, jtr.map((x) => -x));
    let scale = 1;
    for (let tries = 0; tries < 8; tries++) {
      const saved = unknowns.map(get);
      unknowns.forEach((u, i) => set(u, saved[i]! + Math.max(-20, Math.min(20, step[i]! * scale))));
      const r2 = residual(stateOf(a, cur, contacts, root), targets);
      if (Math.hypot(...r2) < Math.hypot(...res)) { res = r2; break; }
      unknowns.forEach((u, i) => set(u, saved[i]!)); scale /= 2;
    }
  }
  for (const u of unknowns) set(u, Math.round(get(u) * 1e4) / 1e4);
  return { bones: cur, error: Math.hypot(...residual(stateOf(a, cur, contacts, root), targets)) };
}
function gauss(m: number[][], b: number[]): number[] {
  const n = b.length; const A = m.map((row, i) => [...row, b[i]!]);
  for (let i = 0; i < n; i++) {
    let p = i; for (let k = i + 1; k < n; k++) if (Math.abs(A[k]![i]!) > Math.abs(A[p]![i]!)) p = k;
    [A[i], A[p]] = [A[p]!, A[i]!];
    for (let k = i + 1; k < n; k++) { const f = A[k]![i]! / A[i]![i]!; for (let j = i; j <= n; j++) A[k]![j]! -= f * A[i]![j]!; }
  }
  const x = new Array(n).fill(0);
  for (let i = n - 1; i >= 0; i--) { let s = A[i]![n]!; for (let j = i + 1; j < n; j++) s -= A[i]![j]! * x[j]; x[i] = s / A[i]![i]!; }
  return x;
}
