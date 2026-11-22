/** 🌬️ PW-08 shared physics and clip builders for trees: wind drag after EN 1991-1-4 on every foliage bone, quasi-static bending of the stem chain and the foliage bones, the four pose families and the three clip families every tree of this worker carries.
 *
 * @see ./asset.ts
 * @see https://eurocodes.jrc.ec.europa.eu/EN-Eurocodes/eurocode-1-actions-structures — EN 1991-1-4 wind actions */
import { FEATURE_M, RAD, add, clamp, frames, mul, r1, r2, r3, sub, t, tipOf, toLocal, unit, type BoneSpec, type Mesh, type Text, type V3 } from "./lib.ts";
import { centroid, projectedArea, sec, type Part, type Species } from "./asset.ts";

export const RHO_AIR = 1.25;
export const G = 9.80665;
export const vAt = (v10: number, h: number): number => v10 * Math.pow(Math.max(h, 2) / 10, 0.22);

/** 🍃️ Per foliage bone of the 1:50 body: its share of the frontal (front view, seen along the wind) and plan area of the whole crown, its centroid and height. */
export function foliageStats(sp: Species, porosity: number): { bone: string; frontalM2: number; planM2: number; c: V3; mesh: Mesh[] }[] {
  const parts = sp.scales[50].parts.filter((p) => p.kind === "foliage");
  const bones = [...new Set(parts.map((p) => p.bone))];
  const all = parts.map((p) => p.mesh);
  const unionFront = projectedArea(all, "front", 0.03);
  const unionPlan = projectedArea(all, "plan", 0.03);
  const own = bones.map((b) => {
    const ms = parts.filter((p) => p.bone === b).map((p) => p.mesh);
    return { bone: b, front: projectedArea(ms, "front", 0.03), plan: projectedArea(ms, "plan", 0.03), ms };
  });
  const sf = own.reduce((s, x) => s + x.front, 0);
  const sp2 = own.reduce((s, x) => s + x.plan, 0);
  return own.map((x) => {
    let v = 0;
    let c: V3 = [0, 0, 0];
    for (const m of x.ms) {
      const cm = centroid(m);
      const w = Math.abs(volumeOf(m));
      v += w;
      c = add(c, mul(cm, w));
    }
    return { bone: x.bone, frontalM2: (unionFront * (1 - porosity) * x.front) / sf, planM2: (unionPlan * x.plan) / sp2, c: mul(c, 1 / v), mesh: x.ms };
  });
}

const volumeOf = (m: Mesh): number => {
  let v = 0;
  for (const [a, b, c] of m.t) {
    const A = m.p[a]!, B = m.p[b]!, C = m.p[c]!;
    v += (A[0] * (B[1] * C[2] - B[2] * C[1]) - A[1] * (B[0] * C[2] - B[2] * C[0]) + A[2] * (B[0] * C[1] - B[1] * C[0])) / 6;
  }
  return v;
};

export type WindCase = { id: string; v10: number; cd: number; reference: string; source: string };

/** 🧲️ The wind loads of one case: a point load at the foliage centroid of every foliage bone and a line load on the bare lower stem. */
export function windLoads(sp: Species, stats: ReturnType<typeof foliageStats>, wc: WindCase, stem: { bone: string; toM: number; d: number; title: Text } | null, title: (bone: string) => Text): { loads: unknown[]; forces: { bone: string; at: V3; fx: number }[] } {
  const fr = frames(sp.bones);
  const loads: unknown[] = [];
  const forces: { bone: string; at: V3; fx: number }[] = [];
  for (const s of stats) {
    const v = vAt(wc.v10, s.c[1]);
    const fx = 0.5 * RHO_AIR * wc.cd * s.frontalM2 * v * v;
    const local = toLocal(fr[s.bone]!, s.c);
    loads.push({ id: `wind-${s.bone}`, kind: "wind", title: title(s.bone), bone: s.bone, x: r3(local[0]), y: r3(local[1]), fxN: r1(fx), fyN: 0, source: wc.source, reference: wc.reference });
    forces.push({ bone: s.bone, at: s.c, fx });
  }
  if (stem !== null) {
    const v = vAt(wc.v10, stem.toM / 2);
    const w = 0.5 * RHO_AIR * 1.0 * stem.d * v * v;
    loads.push({ id: `wind-${stem.bone}`, kind: "wind", title: stem.title, distributed: { bone: stem.bone, fromM: 0, toM: r2(stem.toM), wxNPerM: r2(w), wyNPerM: 0 }, fxN: 0, fyN: 0, source: "literature", reference: "en-1991-1-4" });
    const b = sp.bones.find((x) => x.id === stem.bone)!;
    forces.push({ bone: stem.bone, at: add(b.o, [0, stem.toM / 2, 0]), fx: w * stem.toM });
  }
  return { loads, forces };
}

/** 🏋️ Quasi-static bending: the offset angle of every bone (degrees, negative leans with +x forces, positive with +y down for snow) from the moment of all loads beyond its joint over the bending stiffness of its section, plus the rotation of the root plate on its anchorage stiffness `kRoot` (N·m/rad) added to the first stem bone. */
export function bend(sp: Species, forces: readonly { bone: string; at: V3; fx: number; fy?: number }[], E: number, kRoot: number, stemFirst: string, limit: (b: BoneSpec) => number): Record<string, { angleDeg: number }> {
  const byId = new Map(sp.bones.map((b) => [b.id, b]));
  const below = (bone: string, of: string): boolean => {
    let at: string | null = bone;
    while (at !== null) {
      if (at === of) return true;
      at = byId.get(at)!.parent;
    }
    return false;
  };
  const out: Record<string, { angleDeg: number }> = {};
  for (const b of sp.bones) {
    if (b.joint.type === "fixed") continue;
    let M = 0;
    for (const f of forces) if (below(f.bone, b.id)) {
      const d = sub(f.at, b.o);
      M += f.fx * d[1] - (f.fy ?? 0) * d[0];
    }
    const d = b.section?.d ?? 0.05;
    const EI = E * sec(d).i;
    let th = (M * b.length) / (2 * EI);
    if (b.id === stemFirst) {
      let Mb = 0;
      for (const f of forces) { const dd = sub(f.at, sp.bones[0]!.o); Mb += f.fx * dd[1] - (f.fy ?? 0) * dd[0]; }
      th += Mb / kRoot;
    }
    const deg = clamp(-th / RAD, -limit(b), limit(b));
    if (Math.abs(deg) >= 0.005) out[b.id] = { angleDeg: r2(deg) };
  }
  return out;
}

/** 🌗️ A thermal binding of every region and emitter. */
export const binding = (regions: Record<string, { exposedFraction: number; sunlitFraction: number; wettedness?: number }>, emitters: Record<string, number>) => ({ regions, emitters });

/** 🔁️ The periodic wind-sway clip about the `windy` pose: nine smooth keys over one period, every bone swinging by `amp` of its windy offset (plus `floorDeg`) with a phase lag that grows with height. */
export function swayClip(sp: Species, windy: Record<string, { angleDeg: number }>, periodS: number, amp: number, floorDeg: (b: BoneSpec) => number, title: Text, description: Text, height: number): unknown {
  const keys = [];
  for (let k = 0; k <= 8; k++) {
    const tk = (periodS * k) / 8;
    const bones: Record<string, { angleDeg: number }> = {};
    for (const b of sp.bones) {
      if (b.joint.type === "fixed") continue;
      const base = windy[b.id]?.angleDeg ?? 0;
      const lag = (0.9 * b.o[1]) / height + (b.parent !== null && sp.bones.find((x) => x.id === b.parent)!.joint.type !== "fixed" && !/trunk|stem|leader/.test(b.id) ? 0.6 : 0);
      const a = Math.abs(base) * amp + floorDeg(b);
      const v = base - a * Math.sin((2 * Math.PI * k) / 8 - lag);
      if (Math.abs(v) >= 0.005 || base !== 0) bones[b.id] = { angleDeg: r2(k === 8 ? base - a * Math.sin(-lag) : v) };
    }
    keys.push({ t: r3(tk), pose: "windy", ease: "smooth", bones });
  }
  return { id: "wind-sway", title, description, durationS: periodS, fps: 30, loop: true, rootMotion: "none", keys, contacts: [], phases: [], events: [], tracks: [] };
}

/** 💨️ The single gust: load tracks from 5 % to 2.75 times the fresh-breeze load at 2.5 s (peak velocity pressure with Iv 0.25) and back, and bone keys scaled the same way. */
export function gustClip(windy: Record<string, { angleDeg: number }>, loadIds: readonly string[], title: Text, description: Text): unknown {
  const prof: [number, number][] = [[0, 0.05], [1.2, 0.9], [2.5, 2.75], [3.6, 1.6], [5, 0.7], [6.5, 0.25], [8, 0.05]];
  const keys = prof.map(([tk, f]) => ({ t: tk, pose: "windy", ease: "smooth", bones: Object.fromEntries(Object.entries(windy).map(([b, o]) => [b, { angleDeg: r2(o.angleDeg * f) }])) }));
  const tracks = loadIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: prof.map(([tk, f]) => ({ t: tk, v: f })) }));
  return {
    id: "gust", title, description, durationS: 8, fps: 30, loop: false, rootMotion: "none", keys, contacts: [],
    phases: [{ id: "rise", fromS: 0, toS: 2.5, title: t("Gust rising", "Böe steigt an") }, { id: "decay", fromS: 2.5, toS: 8, title: t("Gust decaying", "Böe klingt ab") }],
    events: [{ t: 2.5, kind: "load-on", title: t("Peak of the gust", "Spitze der Böe") }],
    tracks,
  };
}

/** ☀️ The 24 h transpiration day compressed to 24 s: emitter strengths and sunlit fractions follow the sun, keys hold the calm pose. */
export function diurnalClip(emitters: Record<string, [number, number]>, sunlit: Record<string, number>, title: Text, description: Text): unknown {
  const sun = (h: number): number => (h <= 6 || h >= 18 ? 0 : Math.sin(((h - 6) / 12) * Math.PI));
  const hours = [0, 5, 7, 9, 12, 15, 18, 20, 24];
  const tracks: unknown[] = [];
  for (const [id, [night, peak]] of Object.entries(emitters)) tracks.push({ target: `emitter:${id}`, ease: "smooth", keys: hours.map((h) => ({ t: h, v: r3(night + (peak - night) * Math.pow(sun(h), 0.8)) })) });
  for (const [id, noon] of Object.entries(sunlit)) tracks.push({ target: `region:${id}:sunlitFraction`, ease: "smooth", keys: [0, 6, 8, 10, 12, 14, 16, 18, 24].map((h) => ({ t: h, v: r3(noon * sun(h)) })) });
  return {
    id: "diurnal-transpiration", title, description, durationS: 24, fps: 10, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "calm", ease: "smooth" }, { t: 12, pose: "calm", ease: "smooth" }, { t: 24, pose: "calm", ease: "smooth" }], contacts: [],
    phases: [{ id: "night", fromS: 0, toS: 6, title: t("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: t("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: t("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: t("Evening and night", "Abend und Nacht") }],
    events: [{ t: 6, kind: "thermal", title: t("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: t("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration") }, { t: 18, kind: "thermal", title: t("Sunset", "Sonnenuntergang") }],
    tracks,
  };
}

/** 🧭️ The world direction of an emitter anchor's bone vertical, as `directionDeg` in that bone's frame. */
export const verticalIn = (b: BoneSpec): number => r1(90 - b.angle);

export { FEATURE_M, unit, tipOf, type Part };
