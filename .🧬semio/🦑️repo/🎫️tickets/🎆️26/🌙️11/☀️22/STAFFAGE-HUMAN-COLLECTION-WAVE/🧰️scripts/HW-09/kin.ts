/** 🦾️ HW-09 kinematics helpers over the library's `evaluate`: world points of bones and anchors in a pose, and a damped
 * least-squares inverse kinematics that moves chosen bone offsets so anchors or bone points reach world targets.
 * @see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/README.md (Rest placement of a bone, Lateral spread, Axial twist) */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

export type V3 = { x: number; y: number; z: number };
export type Offset = { angleDeg?: number; abductionDeg?: number; twistDeg?: number; dx?: number; dy?: number };
export type PoseBones = Record<string, Offset>;
export type Field = "angleDeg" | "abductionDeg" | "twistDeg" | "dx" | "dy";
export type Var = { bone: string; field: Field; min?: number; max?: number; reg?: number };
export type Point = { bone: string; local: [number, number, number] } | { anchor: string };
export type Target = { point: Point; at: Partial<V3>; weight?: number; meet?: Point };

export const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";

/** 📥️ The asset document and its asset of a human slug. */
export async function loadHuman(slug: string): Promise<{ doc: any; asset: any }> {
  const doc = await Bun.file(`${CATALOG}/👤️humans/${slug}/🔣️.json`).json();
  return { doc, asset: doc.asset };
}

/** 💾️ Writes a human asset document with the catalog's formatting. */
export async function saveHuman(slug: string, doc: any): Promise<void> {
  await Bun.write(`${CATALOG}/👤️humans/${slug}/🔣️.json`, JSON.stringify(doc, null, 2) + "\n");
}

const PROBE = "__hw09-probe";

/** 🧍️ The evaluated state of a bare configuration (bones, root, ground authored) without touching the asset's poses. */
export function stateOf(asset: any, bones: PoseBones, root: { x?: number; y?: number; angleDeg?: number } = {}): any {
  const probe = { id: PROBE, title: { en: "probe", de: "Probe" }, description: { en: "probe", de: "Probe" }, bones, root: { x: root.x ?? 0, y: root.y ?? 0, angleDeg: root.angleDeg ?? 0 }, ground: "authored", contacts: [], thermal: asset.poses[0]?.thermal ?? { regions: {}, emitters: {} }, loads: [] };
  const probed = { ...asset, poses: [...asset.poses.filter((p: any) => p.id !== PROBE), probe] };
  return evaluate(probed, { kind: "pose", pose: PROBE });
}

/** 📍️ The world position of a bone-local point or an anchor in an evaluated state. */
export function pointOf(state: any, p: Point): V3 {
  if ("anchor" in p) {
    const a = state.anchors[p.anchor];
    if (!a) throw new Error(`unknown anchor ${p.anchor}`);
    return { x: a.at.x, y: a.at.y, z: a.z };
  }
  const f = state.bones[p.bone]?.frame;
  if (!f) throw new Error(`unknown bone ${p.bone}`);
  const [a, b, c] = p.local;
  return { x: f.o.x + a * f.u.x + b * f.v.x + c * f.w.x, y: f.o.y + a * f.u.y + b * f.v.y + c * f.w.y, z: f.o.z + a * f.u.z + b * f.v.z + c * f.w.z };
}

const get = (bones: PoseBones, v: Var): number => bones[v.bone]?.[v.field] ?? 0;
const set = (bones: PoseBones, v: Var, x: number): void => {
  bones[v.bone] = { ...(bones[v.bone] ?? {}), [v.field]: x };
};

/** 🎯️ Damped least squares (Levenberg–Marquardt with numeric Jacobian) over the variables; returns the solved bones and the residual in metres. */
export function solve(asset: any, start: PoseBones, vars: Var[], targets: Target[], root: { x?: number; y?: number; angleDeg?: number } = {}, iterations = 80): { bones: PoseBones; residual: number; worst: number } {
  const bones: PoseBones = JSON.parse(JSON.stringify(start));
  const x0 = vars.map((v) => get(bones, v));
  const residuals = (b: PoseBones): number[] => {
    const s = stateOf(asset, b, root);
    const out: number[] = [];
    for (const t of targets) {
      const p = pointOf(s, t.point);
      const w = t.weight ?? 1;
      const at = t.meet ? pointOf(s, t.meet) : t.at;
      for (const k of ["x", "y", "z"] as const) if (at[k] !== undefined) out.push(w * (p[k] - at[k]!));
    }
    vars.forEach((v, i) => out.push((v.reg ?? 0) * (get(b, v) - x0[i]!) * (v.field === "dx" || v.field === "dy" ? 1 : Math.PI / 180)));
    return out;
  };
  const clamp = (v: Var, x: number): number => Math.min(v.max ?? Infinity, Math.max(v.min ?? -Infinity, x));
  let r = residuals(bones);
  let cost = r.reduce((s, e) => s + e * e, 0);
  let lambda = 1e-3;
  for (let it = 0; it < iterations && cost > 1e-12; it++) {
    const J: number[][] = vars.map((v) => {
      const h = v.field === "dx" || v.field === "dy" ? 1e-5 : 1e-3;
      const b = JSON.parse(JSON.stringify(bones));
      set(b, v, get(bones, v) + h);
      const r2 = residuals(b);
      return r2.map((e, k) => (e - r[k]!) / h);
    });
    const n = vars.length;
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => J[i]!.reduce((s, e, k) => s + e * J[j]![k]!, 0)));
    const g = Array.from({ length: n }, (_, i) => J[i]!.reduce((s, e, k) => s + e * r[k]!, 0));
    let improved = false;
    for (let tries = 0; tries < 8 && !improved; tries++) {
      const M = A.map((row, i) => row.map((e, j) => (i === j ? e * (1 + lambda) + 1e-12 : e)));
      const step = gauss(M, g.map((e) => -e));
      const b = JSON.parse(JSON.stringify(bones));
      vars.forEach((v, i) => set(b, v, clamp(v, get(bones, v) + step[i]!)));
      const r2 = residuals(b);
      const c2 = r2.reduce((s, e) => s + e * e, 0);
      if (c2 < cost) {
        Object.assign(bones, b);
        r = r2;
        cost = c2;
        lambda = Math.max(lambda / 3, 1e-7);
        improved = true;
      } else lambda *= 8;
    }
    if (!improved) break;
  }
  const s = stateOf(asset, bones, root);
  let worst = 0;
  for (const t of targets) {
    const p = pointOf(s, t.point);
    const at = t.meet ? pointOf(s, t.meet) : t.at;
    const d = Math.hypot(...(["x", "y", "z"] as const).filter((k) => at[k] !== undefined).map((k) => p[k] - at[k]!));
    worst = Math.max(worst, d);
  }
  return { bones, residual: Math.sqrt(cost), worst };
}

function gauss(A: number[][], b: number[]): number[] {
  const n = b.length;
  const M = A.map((row, i) => [...row, b[i]!]);
  for (let c = 0; c < n; c++) {
    let p = c;
    for (let r = c + 1; r < n; r++) if (Math.abs(M[r]![c]!) > Math.abs(M[p]![c]!)) p = r;
    [M[c], M[p]] = [M[p]!, M[c]!];
    const d = M[c]![c]! || 1e-12;
    for (let r = 0; r < n; r++) {
      if (r === c) continue;
      const f = M[r]![c]! / d;
      for (let k = c; k <= n; k++) M[r]![k]! -= f * M[c]![k]!;
    }
  }
  return M.map((row, i) => row[n]! / (row[i]! || 1e-12));
}

/** 🔢️ Rounds every offset to a fixed number of decimals and drops zero fields. */
export function tidy(bones: PoseBones, digits = 4): PoseBones {
  const out: PoseBones = {};
  for (const [id, o] of Object.entries(bones)) {
    const e: Offset = {};
    for (const [k, v] of Object.entries(o)) if (v !== undefined && Math.abs(v) > 1e-9) (e as any)[k] = Number(v.toFixed(k === "dx" || k === "dy" ? 5 : digits));
    out[id] = Object.keys(e).length ? e : { angleDeg: 0 };
  }
  return out;
}
