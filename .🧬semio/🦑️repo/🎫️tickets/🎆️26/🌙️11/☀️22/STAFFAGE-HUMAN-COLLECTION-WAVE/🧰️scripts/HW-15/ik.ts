/** 🎯️ Arm inverse kinematics on the library's rig solver: finds shoulder, elbow and wrist offsets that put the wrist at a target, the hand along a direction and the palm towards a normal, by coordinate descent within joint limits.
 * @see ./lib.ts
 */
import { frames, v3, dot, sub, norm, unit, type V3 } from "./lib.ts";

export type ArmTarget = { wrist: V3; fingers?: V3; palm?: V3; hold?: Record<string, any> };
type Var = { bone: string; key: "angleDeg" | "abductionDeg" | "twistDeg"; min: number; max: number };

/** 🦾️ The offsets of `arm-<s>`, `forearm-<s>` and `hand-<s>` (s = l or r) that best reach the target; `base` holds the rest of the pose. Palm normal of the right hand is +w, of the left hand -w. */
export function solveArm(asset: any, base: Record<string, any>, side: "l" | "r", target: ArmTarget, start?: Record<string, any>, armBone = "arm"): { offsets: Record<string, any>; error: { wristM: number; fingersDeg: number; palmDeg: number } } {
  const m = side === "r" ? 1 : -1;
  const vars: Var[] = [
    { bone: `${armBone}-${side}`, key: "angleDeg", min: -40, max: 160 },
    { bone: `${armBone}-${side}`, key: "abductionDeg", min: -95, max: 95 },
    { bone: `${armBone}-${side}`, key: "twistDeg", min: -110, max: 110 },
    { bone: `forearm-${side}`, key: "angleDeg", min: 0, max: 145 },
    { bone: `forearm-${side}`, key: "twistDeg", min: -80, max: 80 },
    { bone: `hand-${side}`, key: "angleDeg", min: -55, max: 55 },
    { bone: `hand-${side}`, key: "abductionDeg", min: -30, max: 30 },
  ];
  const x = vars.map((v) => start?.[v.bone]?.[v.key] ?? base[v.bone]?.[v.key] ?? 0);
  const build = (x: number[]) => {
    const o: Record<string, any> = { ...base };
    for (const b of new Set(vars.map((v) => v.bone))) o[b] = { ...(base[b] ?? {}) };
    vars.forEach((v, i) => (o[v.bone][v.key] = Math.round(x[i]! * 10) / 10));
    return o;
  };
  const measure = (x: number[]) => {
    const f = frames(asset, build(x))[`hand-${side}`].frame;
    const w = norm(sub(v3(f.o), target.wrist));
    const fd = target.fingers ? Math.acos(Math.max(-1, Math.min(1, dot(v3(f.u), unit(target.fingers))))) : 0;
    const pd = target.palm ? Math.acos(Math.max(-1, Math.min(1, dot(v3(f.w).map((q) => q * m) as V3, unit(target.palm))))) : 0;
    return { w, fd, pd, cost: (w / 0.01) ** 2 + (fd / 0.08) ** 2 + (pd / 0.15) ** 2 };
  };
  let best = measure(x).cost;
  for (let step = 16; step >= 0.1; step /= 2) {
    let improved = true;
    while (improved) {
      improved = false;
      for (let i = 0; i < x.length; i++) for (const d of [step, -step]) {
        const y = [...x];
        y[i] = Math.max(vars[i]!.min, Math.min(vars[i]!.max, y[i]! + d));
        const c = measure(y).cost;
        if (c < best - 1e-9) { best = c; x.splice(0, x.length, ...y); improved = true; }
      }
    }
  }
  const e = measure(x);
  const offsets = build(x);
  const out: Record<string, any> = {};
  for (const b of new Set(vars.map((v) => v.bone))) out[b] = offsets[b];
  return { offsets: out, error: { wristM: e.w, fingersDeg: e.fd * 180 / Math.PI, palmDeg: e.pd * 180 / Math.PI } };
}
