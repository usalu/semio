/** 🌴️ Root fix of the date palm rig: sixteen fronds in four tiers on the crown (instead of eight from one point), their pose and clip offsets, wind loads and gust tracks, all derived from one hand-stated crown table and from the wind response the earlier poses already declared.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-date-palm.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { STAFFAGE, TICKET, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, cross, dirOf, dot, upOf } from "./r3-date-palm-kit.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }> };

/** 🌿️ One frond as stated by hand: tier, azimuth (degrees from +x towards -z, as the sheets print it), elevation of the proximal and distal chords, chord lengths, base height along the crown bone and base radius from the stem axis. */
export type Frond = { k: number; id: string; tier: "A" | "B" | "C" | "D"; az: number; elA: number; elB: number; La: number; Lb: number; baseU: number; baseR: number; side: "l" | "r"; n: number; restA: number; restB: number };

const TIERS = {
  A: { count: 5, elA: 5, elB: -42, La: 2.0, Lb: 1.9, baseU: 0.2, baseR: 0.26 },
  B: { count: 5, elA: 27, elB: -3, La: 2.0, Lb: 1.85, baseU: 0.34, baseR: 0.24 },
  C: { count: 4, elA: 55, elB: 37, La: 1.82, Lb: 1.66, baseU: 0.46, baseR: 0.2 },
  D: { count: 2, elA: 77, elB: 67, La: 1.86, Lb: 1.72, baseU: 0.56, baseR: 0.14 },
} as const;
const AZ0 = -5;
const GOLDEN = 137.508;
const CROWN_X = 3.12, CROWN_Z = 3.47;

/** 📐️ Length factors of the outer fronds, measured with `r3-date-palm-reach.ts` so that the crown fills `dimensions` (6.3 m along, 7.0 m across) without passing it. */
const FIT: Record<number, number> = {0: 0.914, 1: 0.9497, 2: 0.9373, 3: 0.9411, 4: 0.91, 5: 0.8825, 6: 0.895, 7: 0.8857, 8: 0.8796, 9: 0.8728};

/** 📐️ The reach of the lowest tier at an azimuth: the crown is a little longer across (z) than along (x), as `dimensions` declares. */
const reachScale = (az: number): number => {
  const [c, s] = [Math.cos(az * D2R), Math.sin(az * D2R)];
  return 1 / Math.hypot(c / CROWN_X, s / CROWN_Z) / CROWN_Z;
};

/** 🌴️ The sixteen fronds of the crown, oldest (lowest) first; ids by side (l = left, +z) in the order of their age. */
export function fronds(): Frond[] {
  const out: Omit<Frond, "id" | "side" | "n" | "restA" | "restB">[] = [];
  let k = 0;
  for (const tier of ["A", "B", "C", "D"] as const) {
    const t = TIERS[tier];
    for (let i = 0; i < t.count; i++, k++) {
      const az = ((((AZ0 + k * GOLDEN) % 360) + 540) % 360) - 180;
      const wobble = [0.6, -0.9, 0.3, -0.4, 0.8, -0.2, 0.5, -0.7][k % 8]!;
      const scale = (tier === "A" || tier === "B" ? reachScale(az) : 1) * (FIT[k] ?? 1);
      out.push({ k, tier, az: Math.round(az * 10) / 10, elA: t.elA + wobble * 2, elB: t.elB + wobble * 3, La: Math.round(t.La * scale * 1000) / 1000, Lb: Math.round(t.Lb * scale * 1000) / 1000, baseU: t.baseU + (i % 2) * 0.03, baseR: t.baseR });
    }
  }
  const count = { l: 0, r: 0 };
  return out.map((f) => {
    const side: "l" | "r" = Math.sin(f.az * D2R) <= 0 ? "l" : "r";
    const n = ++count[side];
    const d = dirOf(f.az, f.elA);
    const restA = Math.round((Math.atan2(-d[0], d[1]) / D2R) * 1e4) / 1e4;
    return { ...f, id: `fr-${side}${n}`, side, n, restA, restB: Math.round((f.elB - f.elA) * 1e4) / 1e4 };
  });
}

/** 🧮️ Offsets (angle from rest, abduction, twist) of a proximal frond bone whose world axis is `d` with its normal `up`, under a parent frame `P`. */
function solveOffsets(P: { u: V3; v: V3; w: V3 }, d: V3, up: V3, restDeg: number): { angleDeg: number; abductionDeg: number; twistDeg: number } {
  const [du, dv, dw] = [dot(d, P.u), dot(d, P.v), dot(d, P.w)];
  const phi = Math.asin(Math.max(-1, Math.min(1, dw)));
  const theta = Math.atan2(dv, du);
  const u1: V3 = [P.u[0] * Math.cos(theta) + P.v[0] * Math.sin(theta), P.u[1] * Math.cos(theta) + P.v[1] * Math.sin(theta), P.u[2] * Math.cos(theta) + P.v[2] * Math.sin(theta)];
  const v1: V3 = [-P.u[0] * Math.sin(theta) + P.v[0] * Math.cos(theta), -P.u[1] * Math.sin(theta) + P.v[1] * Math.cos(theta), -P.u[2] * Math.sin(theta) + P.v[2] * Math.cos(theta)];
  const w2: V3 = [-u1[0] * Math.sin(phi) + P.w[0] * Math.cos(phi), -u1[1] * Math.sin(phi) + P.w[1] * Math.cos(phi), -u1[2] * Math.sin(phi) + P.w[2] * Math.cos(phi)];
  const psi = Math.atan2(-dot(up, w2), dot(up, v1));
  let angle = theta / D2R - restDeg;
  while (angle > 180) angle -= 360;
  while (angle < -180) angle += 360;
  const r = (x: number) => Math.round(x * 1e4) / 1e4;
  return { angleDeg: r(angle), abductionDeg: r(phi / D2R), twistDeg: r(psi / D2R) };
}

const FROND_RE = /^fr-[lr]\d+[ab]$/;
const elOf = (u: V3): number => Math.asin(Math.max(-1, Math.min(1, u[1]))) / D2R;

/** 🌬️ The wind response the earlier poses and keys declared: mean change of the world elevation of the proximal and distal chords of the windward (l) and leeward (r) fronds against the calm pose. */
function windResponse(old: Json, offsets: Json, calm: Record<string, { frame: { u: V3 } }>): { al: number; ar: number; bl: number; br: number } {
  const now = solveChain(old.rig, offsets, { x: 0, y: 0, angleDeg: 0 });
  const mean = (side: string, seg: string) => {
    const ids = old.rig.bones.map((b: Json) => b.id as string).filter((id: string) => id.startsWith(`fr-${side}`) && id.endsWith(seg));
    return ids.reduce((s: number, id: string) => s + elOf(now[id]!.frame.u) - elOf(calm[id]!.frame.u), 0) / ids.length;
  };
  return { al: mean("l", "a"), ar: mean("r", "a"), bl: mean("l", "b"), br: mean("r", "b") };
}

const text = (en: string, de: string) => ({ en, de });

/** 🔧️ Rewrites the rig, poses, clips, loads and gust tracks of the asset in place; returns the fronds and a report. */
export function fixRig(asset: Json): { fronds: Frond[]; report: string[] } {
  const old = JSON.parse(readFileSync(join(TICKET, "r3-date-palm-baseline.json"), "utf8"));
  asset.rig.bones = old.rig.bones.map((b: Json) => asset.rig.bones.find((x: Json) => x.id === b.id) ?? b).filter((b: Json) => !FROND_RE.test(b.id) || old.rig.bones.some((x: Json) => x.id === b.id));
  const calmOld = old.poses.find((p: Json) => p.id === "calm");
  const calmFrames = solveChain(old.rig, calmOld.bones, { x: 0, y: 0, angleDeg: 0 });
  const F = fronds();
  const report: string[] = [];
  const keep = asset.rig.bones.filter((b: Json) => !FROND_RE.test(b.id));
  const crownAt = keep.findIndex((b: Json) => b.id === "crown");
  const newBones: Json[] = [];
  for (const f of F) {
    const side = f.side === "l" ? text("left", "links") : text("right", "rechts");
    const tierName = { A: text("lowest tier", "unterster Kranz"), B: text("second tier", "zweiter Kranz"), C: text("third tier", "dritter Kranz"), D: text("youngest tier", "jüngster Kranz") }[f.tier];
    const [ca, sa] = [Math.cos(f.az * D2R), Math.sin(f.az * D2R)];
    newBones.push({
      id: `${f.id}a`, parent: "crown",
      title: text(`Frond ${f.n} ${side.en} (${tierName.en}), petiole and lower blade`, `Wedel ${f.n} ${side.de} (${tierName.de}), Blattstiel und unteres Blatt`),
      x: f.baseU, y: Math.round(-f.baseR * ca * 1e4) / 1e4, z: Math.round(-f.baseR * sa * 1e4) / 1e4, angleDeg: f.restA, lengthM: f.La,
      joint: { type: "revolute", minDeg: -45, maxDeg: 45 },
      mass: { kg: 3.5, comX: Math.round(f.La * 0.42 * 1e4) / 1e4, comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" },
    });
    newBones.push({
      id: `${f.id}b`, parent: `${f.id}a`,
      title: text(`Frond ${f.n} ${side.en} (${tierName.en}), upper blade`, `Wedel ${f.n} ${side.de} (${tierName.de}), oberes Blatt`),
      x: f.La, y: 0, angleDeg: f.restB, lengthM: f.Lb,
      joint: { type: "revolute", minDeg: -30, maxDeg: 30 },
      mass: { kg: 2.5, comX: Math.round(f.Lb * 0.45 * 1e4) / 1e4, comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" },
    });
  }
  asset.rig.bones = [...keep.slice(0, crownAt + 1), ...newBones, ...keep.slice(crownAt + 1)];
  report.push(`rig: ${F.length} fronds (${F.map((f) => `${f.id} ${f.tier} az ${f.az} el ${f.elA.toFixed(1)}/${f.elB.toFixed(1)} L ${f.La}+${f.Lb}`).join("; ")})`);
  const offsetsFor = (base: Json): Json => {
    const kept = Object.fromEntries(Object.entries(base).filter(([id]) => !FROND_RE.test(id)));
    const wind = windResponse(old, base, calmFrames);
    const crown = solveChain(asset.rig, kept, { x: 0, y: 0, angleDeg: 0 })["crown"]!.frame;
    const out: Json = { ...kept };
    for (const f of F) {
      const t = (1 - Math.cos(f.az * D2R)) / 2;
      const da = t * wind.al + (1 - t) * wind.ar;
      const db = t * wind.bl + (1 - t) * wind.br;
      const [ea, eb] = [f.elA + da, f.elB + db];
      out[`${f.id}a`] = solveOffsets(crown, dirOf(f.az, ea), upOf(f.az, ea), f.restA);
      const bend = Math.round((eb - ea - (f.elB - f.elA)) * 1e4) / 1e4;
      if (bend !== 0) out[`${f.id}b`] = { angleDeg: bend };
    }
    return out;
  };
  for (const pose of asset.poses) pose.bones = offsetsFor(old.poses.find((p: Json) => p.id === pose.id).bones);
  for (const clip of asset.clips) {
    const base = old.clips.find((c: Json) => c.id === clip.id);
    clip.keys = base.keys.map((key: Json) => ({ ...key, bones: offsetsFor(key.bones ?? old.poses.find((p: Json) => p.id === key.pose).bones) }));
    clip.tracks = base.tracks;
  }
  for (const pose of asset.poses) {
    pose.loads = old.poses.find((p: Json) => p.id === pose.id).loads;
    const sample = pose.loads.find((l: Json) => FROND_RE.test(l.bone ?? ""));
    const rest = pose.loads.filter((l: Json) => !FROND_RE.test(l.bone ?? ""));
    const per = Math.round((sample.fxN / 2) * 10) / 10;
    pose.loads = [...F.map((f) => ({ ...sample, id: `wind-${f.id}b`, title: text(`Wind drag on frond ${f.n} ${f.side === "l" ? "left" : "right"}, upper blade`, `Winddruck auf Wedel ${f.n} ${f.side === "l" ? "links" : "rechts"}, oberes Blatt`), bone: `${f.id}b`, x: Math.round(f.Lb * 0.45 * 1e4) / 1e4, fxN: per })), ...rest];
    report.push(`pose ${pose.id}: ${F.length} frond loads of ${per} N (was ${sample.fxN} N on 8 fronds)`);
  }
  for (const clip of asset.clips) {
    const tracks: Json[] = clip.tracks ?? [];
    const frondTrack = tracks.find((t) => /^load:wind-fr-/.test(t.target));
    if (frondTrack === undefined) continue;
    const others = tracks.filter((t) => !/^load:wind-fr-/.test(t.target));
    const at = tracks.indexOf(frondTrack);
    const made = F.map((f) => ({ ...frondTrack, target: `load:wind-${f.id}b` }));
    clip.tracks = [...others.slice(0, at), ...made, ...others.slice(at)];
  }
  return { fronds: F, report };
}

/** 🧾️ Checks the solved crown: world azimuth and elevation of every frond chord in a pose, against the table. */
export function crownCheck(asset: Json, pose: string): string[] {
  const frames = solveChain(asset.rig, asset.poses.find((p: Json) => p.id === pose).bones, { x: 0, y: 0, angleDeg: 0 });
  return fronds().map((f) => {
    const [a, b] = [frames[`${f.id}a`]!.frame, frames[`${f.id}b`]!.frame];
    const az = (u: V3) => (Math.atan2(-u[2], u[0]) / D2R).toFixed(1);
    const wy = a.w[1].toFixed(3);
    return `${f.id} ${pose}: a az ${az(a.u)} el ${elOf(a.u).toFixed(1)} (table ${f.az} / ${f.elA.toFixed(1)}), b el ${elOf(b.u).toFixed(1)} (table ${f.elB.toFixed(1)}), w_y ${wy}, v_y ${a.v[1].toFixed(2)}, |u x v - w| ${Math.hypot(...cross(a.u, a.v).map((x, i) => x - a.w[i]!)).toFixed(4)}`;
  });
}
