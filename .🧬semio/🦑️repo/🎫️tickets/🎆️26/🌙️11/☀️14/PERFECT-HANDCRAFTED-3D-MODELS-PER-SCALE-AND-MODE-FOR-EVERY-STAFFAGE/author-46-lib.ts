/** 🔧 Author 46 shared helpers for the four animal drivers: plume disc on the back, thermal binding of the nose jets, material fixes. */
import { type Ctx, type Json } from "./kit-lib.ts";

/** Upper surface (bone-local v) of a loft along u at u. */
export function topAt(solid: Json, u: number): number {
  const s = solid.sections as { at: number[]; rx: number }[];
  for (let i = 0; i + 1 < s.length; i++) {
    const a = s[i]!;
    const b = s[i + 1]!;
    if (u >= a.at[0]! && u <= b.at[0]!) {
      const t = (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1);
      return a.at[1]! + (b.at[1]! - a.at[1]!) * t + (a.rx + (b.rx - a.rx) * t);
    }
  }
  const e = u < s[0]!.at[0]! ? s[0]! : s[s.length - 1]!;
  return e.at[1]! + e.rx;
}

/** Plume emitter as a thin horizontal disc lying on the back: thin axis vertical, centre on the back line of the bone's body loft, half width limited to the body half width. */
export function fixPlume(c: Ctx, thermal: Json[], body: Json[], halfWidth: number): void {
  for (const t of thermal) {
    if (!(t.tags ?? []).includes("plume-back") || t.kind !== "ellipsoid") continue;
    const f = c.state.frames[t.bone]!;
    const theta = Math.atan2(f.u[1]!, f.u[0]!);
    const a = (Math.PI / 2 - theta) / 2;
    const loft = body.find((s) => s.bone === t.bone && s.kind === "loft" && s.role === "body")!;
    const u = t.transform.translation[0]!;
    const r = t.radii as number[];
    t.transform = { translation: [u, +(topAt(loft, u) + 0.4 * r[0]!).toFixed(5), 0], rotation: [0, 0, +Math.sin(a).toFixed(5), +Math.cos(a).toFixed(5)] };
    t.radii = [r[0]!, r[1]!, Math.min(r[2]!, halfWidth)];
    delete t.region;
  }
}

/** Members of bones that no body solid draws at a scale are shifted back along their bone until the capsule ends within 90 % of the bone length (they would otherwise stick out of the neighbouring body). */
export function shiftUndrawn(c: Ctx, structure: Json[], frac: Record<string, number> = { ear: 0.9, tail: 0.9 }): Json[] {
  const bones: Json[] = c.raw.rig.bones;
  for (const m of structure) {
    if (m.part !== "member" || m.kind !== "capsule" || (c.vol.body as Json[]).some((b) => b.bone === m.bone)) continue;
    const len: number = bones.find((b) => b.id === m.bone)!.lengthM;
    const k = frac[(m.bone as string).replace(/-.*$/, "")] ?? 0.9;
    const shift = Math.max(0, m.p1[0] + m.r1 - k * len);
    m.p0 = [m.p0[0] - shift, m.p0[1], m.p0[2]];
    m.p1 = [m.p1[0] - shift, m.p1[1], m.p1[2]];
  }
  return structure;
}

/** Nostril and mouth jets are capsules whose end must not leave the head loft: both ends are clamped to the tip of the loft along the bone axis, minus the jet radius. */
export function clampJets(thermal: Json[], body: Json[]): void {
  for (const t of thermal) {
    if (t.kind !== "capsule" || !(t.tags ?? []).some((g: string) => /^breath-/.test(g))) continue;
    const loft = body.find((s) => s.bone === t.bone && s.kind === "loft" && s.role === "body");
    if (loft === undefined) continue;
    const secs = loft.sections as { at: number[] }[];
    const tip = Math.max(...secs.map((q) => q.at[0]!)) - 2.5 * Math.max(t.r0, t.r1) - 0.01;
    t.p0 = [Math.min(t.p0[0], tip), t.p0[1], t.p0[2]];
    t.p1 = [Math.min(t.p1[0], tip), t.p1[1], t.p1[2]];
  }
}

/** Centre of the body loft of a bone (bone-local v, w) at u, linear between sections, clamped to the ends. */
export function centreAt(solid: Json, u: number): [number, number] {
  const s = solid.sections as { at: number[] }[];
  if (u <= s[0]!.at[0]!) return [s[0]!.at[1]!, s[0]!.at[2]!];
  for (let i = 0; i + 1 < s.length; i++) {
    const a = s[i]!;
    const b = s[i + 1]!;
    if (u <= b.at[0]!) {
      const k = (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1);
      return [a.at[1]! + (b.at[1]! - a.at[1]!) * k, a.at[2]! + (b.at[2]! - a.at[2]!) * k];
    }
  }
  const e = s[s.length - 1]!;
  return [e.at[1]!, e.at[2]!];
}

/** Members, joints and mass ellipsoids of a bone with a body loft are centred in the loft section (v and w of the section centre) instead of lying on the bone axis. */
export function centreOnLofts(c: Ctx, structure: Json[], skip: RegExp = /^ear-/): Json[] {
  for (const m of structure) {
    const loft = (c.vol.body as Json[]).find((b) => b.bone === m.bone && b.kind === "loft" && b.role === "body");
    if (loft === undefined || skip.test(m.bone)) continue;
    const secs = loft.sections as { at: number[]; rx: number; ry: number }[];
    const lo = secs[0]!.at[0]!;
    const hi = secs[secs.length - 1]!.at[0]!;
    const half = (u: number): number => {
      let i = 0;
      while (i + 2 < secs.length && secs[i + 1]!.at[0]! < u) i++;
      const a = secs[i]!;
      const b = secs[i + 1]!;
      const k = Math.min(1, Math.max(0, (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1)));
      return Math.min(a.rx + (b.rx - a.rx) * k, a.ry + (b.ry - a.ry) * k);
    };
    const floor = 0.51 * c.feature;
    if (m.kind === "capsule") {
      m.p0 = [Math.max(m.p0[0], lo + m.r0), m.p0[1], m.p0[2]];
      m.p1 = [Math.max(Math.min(m.p1[0], hi - m.r1), m.p0[0]), m.p1[1], m.p1[2]];
      m.r0 = Math.max(floor, Math.min(m.r0, 0.85 * half(m.p0[0])));
      m.r1 = Math.max(floor, Math.min(m.r1, 0.85 * half(m.p1[0])));
      const [v0, w0] = centreAt(loft, m.p0[0]);
      const [v1, w1] = centreAt(loft, m.p1[0]);
      m.p0 = [m.p0[0], +v0.toFixed(5), +w0.toFixed(5)];
      m.p1 = [m.p1[0], +v1.toFixed(5), +w1.toFixed(5)];
    } else if (m.kind === "ellipsoid") {
      const tr = m.transform.translation as number[];
      const [v, w] = centreAt(loft, tr[0]!);
      m.radii = (m.radii as number[]).map((r) => Math.max(floor, Math.min(r, 0.85 * half(tr[0]!))));
      m.transform = { ...m.transform, translation: [tr[0], +v.toFixed(5), +w.toFixed(5)] };
    }
  }
  return structure;
}
