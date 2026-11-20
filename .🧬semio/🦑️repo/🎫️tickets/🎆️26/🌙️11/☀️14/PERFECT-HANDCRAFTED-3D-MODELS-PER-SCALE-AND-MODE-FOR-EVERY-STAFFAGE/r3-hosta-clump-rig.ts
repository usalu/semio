/** 🦴️ Round 3, vegetation/hosta-clump: root fix of the rig. The rhizome crown with the root ball, four shoots (eyes) standing on it, and 25 leaves, each on its own petiole and blade bone from its insertion in its shoot (instead of nine bone pairs from one point 6 cm above the bed, each standing for two to three leaves, and a root bone of 0.9 m with its mass centre 0.45 m beside the plant). Poses and clip keys are stated per tier and solved for every leaf; blades never dip below the bed. Wind and rain loads per leaf with the earlier totals; transpiration anchors on the upper face of real blades.
 * @see ./r3-vegetation-hosta-clump.md */
import { join } from "node:path";
import { STAFFAGE, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, azElOf, dirOf, dot, mul, r4, sideOf, solveOffsets, sub, unit, upOf } from "./r3-agave-kit.ts";
import { ALL, SHOOTS, STATIONS, bladeId, cupDeg, leafNo, outerWavy, petioleId, shootCentre, wingPoint, type Built } from "./r3-hosta-clump-plant.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

const text = (en: string, de: string) => ({ en, de });
const ROOT = { x: 0, y: 0, angleDeg: 0 };
const SHOOT_AXES = { u: [0, 1, 0] as V3, v: [-1, 0, 0] as V3, w: [0, 0, 1] as V3 };
export const shootId = (n: number): string => `shoot-${n}`;
export const shootFrame = (i: number): Frame => ({ o: shootCentre(SHOOTS[i]!), ...SHOOT_AXES });

/** 🎚️ What a pose or key does to one leaf: change of the petiole chord elevation, bend of the blade at the junction, lean of the petiole towards +x (downwind), all in degrees. */
type Move = { dA: number; dB: number; lean: number };
const STILL: Move = { dA: 0, dB: 0, lean: 0 };

/** 🌦️ The four poses per tier. */
const POSE_MOVES: Record<string, (b: Built) => Move> = {
  "summer-full": () => STILL,
  "spring-emerging": (b) => ({ dA: Math.min(25, 86 - b.elA), dB: 38, lean: 0 }),
  "rain-bowed": (b) => ({ a: { dA: -1, dB: -3, lean: 0 }, b: { dA: -5, dB: -14, lean: 0 }, c: { dA: -4, dB: -14, lean: 0 } })[b.leaf.tier],
  "autumn-collapse": (b) => ({ a: { dA: -9, dB: -6, lean: 0 }, b: { dA: -26, dB: -24, lean: 0 }, c: { dA: -38, dB: -34, lean: 0 } })[b.leaf.tier],
};

/** 🌬️ The keys of the two clips: the 1.2 s flutter whose phase runs round the mound, and the 6 s gust that leans the petioles downwind and bows the lee blades at 2.6 s. */
const CLIP_MOVES: Record<string, (b: Built, t: number) => Move> = {
  "wind-flutter": (b, t) => ({ dA: 0.6 * Math.sin((2 * Math.PI * t) / 1.2 + b.leaf.az * D2R - 0.6), dB: 3.5 * Math.sin((2 * Math.PI * t) / 1.2 + b.leaf.az * D2R), lean: 0 }),
  gust: (b, t) => {
    const g = t <= 0 ? 0 : t <= 1.5 ? 0.6 : t <= 2.6 ? 1 : t <= 4 ? 0.35 : 0;
    return { dA: 0, dB: g * (-9 * Math.cos(b.leaf.az * D2R) + 5 * Math.sin(3 * b.leaf.az * D2R + t)), lean: 7 * g };
  },
};

/** 🧷️ Blade points in the blade bone frame that must stay above the bed: margins, midrib underside, tip. */
const guard = (b: Built): V3[] => STATIONS.flatMap((s) => [1, -1].map((sg) => wingPoint(b, s, sg as 1 | -1, outerWavy(b.leaf, s), -0.004)).concat([add(b.mid.at(s), mul(b.mid.normal(s), -0.006))])).map((p) => { const d = sub(p, b.fb.o); return [dot(d, b.fb.u), dot(d, b.fb.v), dot(d, b.fb.w)] as V3; });
const GUARDS = ALL.map(guard);

/** 🧭️ The directions of a leaf under a move: petiole chord azimuth and elevation, blade chord elevation, the junction, and the lowest guarded point. */
function placed(b: Built, m: Move) {
  const d0 = dirOf(b.leaf.az, Math.max(4, b.elA + m.dA));
  const d = m.lean === 0 ? d0 : unit(add(d0, [Math.tan(m.lean * D2R), 0, 0]));
  const { az, el } = azElOf(d);
  const elB = el + (b.elB - b.elA) + m.dB;
  const j = add(b.p0, mul(dirOf(az, el), b.La));
  const [u, v, w] = [dirOf(az, elB), upOf(az, elB), sideOf(az)];
  const low = Math.min(...GUARDS[b.leaf.k]!.map((q) => j[1] + q[0] * u[1] + q[1] * v[1] + q[2] * w[1]));
  return { az, el, elB, low };
}

const BED = 0.004;

/** 🛑️ The move scaled back until the blade clears the bed (the default pose itself clears it). */
function clear(b: Built, m: Move): Move {
  if (placed(b, m).low >= BED) return m;
  let [lo, hi] = [0, 1];
  for (let i = 0; i < 30; i++) {
    const f = (lo + hi) / 2;
    if (placed(b, { dA: m.dA * f, dB: m.dB * f, lean: m.lean }).low >= BED) lo = f; else hi = f;
  }
  return { dA: m.dA * lo, dB: m.dB * lo, lean: m.lean };
}

const restA = (b: Built) => r4(Math.atan2(-b.fa.u[0], b.fa.u[1]) / D2R);
const restB = (b: Built) => r4(b.elB - b.elA);

function offsetsOf(move: (b: Built) => Move, used: Record<string, number>): Json {
  const out: Json = {};
  for (const b of ALL) {
    const m = clear(b, move(b));
    const p = placed(b, m);
    const o = solveOffsets(SHOOT_AXES, dirOf(p.az, p.el), upOf(p.az, p.el), restA(b));
    out[petioleId(b.leaf.k)] = o;
    used[petioleId(b.leaf.k)] = Math.max(used[petioleId(b.leaf.k)] ?? 0, Math.abs(o.angleDeg));
    const bend = r4(p.elB - p.el - restB(b));
    if (Math.abs(bend) >= 0.0001) out[bladeId(b.leaf.k)] = { angleDeg: bend };
    used[bladeId(b.leaf.k)] = Math.max(used[bladeId(b.leaf.k)] ?? 0, Math.abs(bend));
  }
  return out;
}

export const OUTLETS: readonly { id: string; k: number }[] = [
  { id: "tp-outer-1", k: 0 }, { id: "tp-outer-2", k: 2 }, { id: "tp-outer-3", k: 5 }, { id: "tp-outer-4", k: 7 },
  { id: "tp-inner-1", k: 11 }, { id: "tp-inner-2", k: 14 }, { id: "tp-inner-3", k: 17 }, { id: "tp-inner-4", k: 20 }, { id: "tp-inner-5", k: 23 },
];
export const OUTLET_S = 0.36, OUTLET_D = 0.48;

/** 🔧️ Rewrites rig, poses, clips, loads, anchors and emitter directions of `asset` in place. */
export function fixRig(asset: Json): string[] {
  const report: string[] = [];
  const used: Record<string, number> = {};
  const poses = (asset.poses as Json[]).map((p) => ({ ...p, bones: offsetsOf(POSE_MOVES[p.id]!, used) }));
  const clips = (asset.clips as Json[]).map((c) => ({ ...c, keys: (c.keys as Json[]).map((k) => ({ ...k, bones: offsetsOf((b) => CLIP_MOVES[c.id]!(b, k.t), used) })) }));
  const limit = (id: string, floor: number) => Math.min(180, Math.max(floor, Math.ceil((used[id]! + 10) / 5) * 5));
  const mass = (kg: number, comX: number) => ({ kg: r4(kg), comX: r4(comX), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "schmid-1991" });
  const bladeArea = ALL.reduce((s, b) => s + b.leaf.Lb * b.leaf.W, 0), petioleLength = ALL.reduce((s, b) => s + b.leaf.Lp, 0);
  const bones: Json[] = [{ id: "root-plate", parent: null, title: text("Rhizome crown and root ball", "Rhizomkrone mit Wurzelballen"), x: 0, y: 0, angleDeg: 0, lengthM: 0.12, joint: { type: "fixed" }, mass: { kg: 3, comX: 0, comY: 0.02, distribution: "uniform", source: "authored-estimate", reference: "schmid-1991" } }];
  SHOOTS.forEach((s, i) => {
    const c = shootCentre(s);
    bones.push({ id: shootId(s.n), parent: "root-plate", title: text(`Shoot ${s.n}: the bundle of clasping petiole bases`, `Trieb ${s.n}: das Bündel der umfassenden Blattstielbasen`), x: r4(c[0]), y: 0, z: r4(c[2]), angleDeg: 90, lengthM: s.h, joint: { type: "fixed" }, mass: mass(0.1, s.h / 2) });
  });
  for (const b of ALL) {
    const l = b.leaf, n = l.k + 1, c = shootCentre(SHOOTS[l.shoot]!);
    const tier = { a: text("outer, spreading", "außen, ausgebreitet"), b: text("rising", "aufsteigend"), c: text("heart, upright", "Herz, aufrecht") }[l.tier];
    bones.push({ id: petioleId(l.k), parent: shootId(SHOOTS[l.shoot]!.n), title: text(`Petiole ${n} (${tier.en})`, `Blattstiel ${n} (${tier.de})`), x: r4(b.p0[1]), y: r4(-(b.p0[0] - c[0])), z: r4(b.p0[2] - c[2]), angleDeg: restA(b), lengthM: r4(b.La), joint: { type: "revolute", minDeg: -limit(petioleId(l.k), 30), maxDeg: limit(petioleId(l.k), 30) }, mass: mass((0.65 * l.Lp) / petioleLength, b.La * 0.42) });
    bones.push({ id: bladeId(l.k), parent: petioleId(l.k), title: text(`Leaf blade ${n} (${tier.en})`, `Blattspreite ${n} (${tier.de})`), x: r4(b.La), y: 0, angleDeg: restB(b), lengthM: r4(b.Lb), joint: { type: "revolute", minDeg: -limit(bladeId(l.k), 40), maxDeg: limit(bladeId(l.k), 40) }, mass: mass((0.65 * l.Lb * l.W) / bladeArea, b.Lb * 0.42) });
  }
  asset.rig.bones = bones;
  report.push(`rig: root-plate, 4 shoots, 25 petioles and 25 blades (${bones.length} bones), ${bones.reduce((s, b) => s + b.mass.kg, 0).toFixed(3)} kg`);
  const windOf: Record<string, number> = { "summer-full": 0.909, "spring-emerging": 0.459 };
  asset.poses = poses.map((p: Json) => {
    const loads: Json[] = [];
    if (windOf[p.id] !== undefined) for (const b of ALL) loads.push({ id: `wind-${leafNo(b.leaf.k)}`, kind: "wind", title: text(`Wind drag on blade ${b.leaf.k + 1}`, `Winddruck auf Spreite ${b.leaf.k + 1}`), bone: bladeId(b.leaf.k), x: r4(b.Lb * 0.42), y: 0, fxN: r4((windOf[p.id]! * b.leaf.Lb * b.leaf.W) / bladeArea), fyN: 0, source: "authored-estimate", reference: "en-1991-1-4" });
    if (p.id === "rain-bowed") for (const b of ALL) loads.push({ id: `water-${leafNo(b.leaf.k)}`, kind: "imposed", title: text(`Rain water retained on blade ${b.leaf.k + 1}`, `Auf Spreite ${b.leaf.k + 1} haftendes Regenwasser`), bone: bladeId(b.leaf.k), x: r4(b.Lb * 0.42), y: 0, fxN: 0, fyN: -r4((3.528 * b.leaf.Lb * b.leaf.W) / bladeArea), source: "authored-estimate", reference: "en-1991-1-3" });
    return { ...p, loads };
  });
  asset.clips = clips;
  const outlets = OUTLETS.map((o) => {
    const b = ALL[o.k]!;
    const p = wingPoint(b, OUTLET_S, 1, OUTLET_D * outerWavy(b.leaf, OUTLET_S), 0.003);
    const d = sub(p, b.fb.o);
    return { id: o.id, bone: bladeId(o.k), x: r4(dot(d, b.fb.u)), y: r4(dot(d, b.fb.v)), z: r4(dot(d, b.fb.w)), kind: "emitter", title: text(`Transpiration of blade ${o.k + 1}`, `Transpiration von Spreite ${o.k + 1}`) };
  });
  asset.anchors = [{ ...(asset.anchors as Json[]).find((a) => a.id === "base-support"), footprintLengthM: 0.3, footprintWidthM: 0.3 }, ...outlets];
  for (const e of asset.thermal.emitters as Json[]) {
    const b = ALL[OUTLETS.find((o) => o.id === e.anchor)!.k]!;
    e.directionDeg = Math.round(Math.atan2(b.fb.v[1], b.fb.u[1]) / D2R);
  }
  report.push(`outlets: ${outlets.map((a) => `${a.id} on ${a.bone} at ${a.x}, ${a.y}, ${a.z}`).join("; ")}; cup at the outlet ${cupDeg(ALL[0]!.leaf, OUTLET_S).toFixed(1)} deg`);
  return report;
}

/** 🧾️ The largest deviation between the solved bone frames of a pose and the stated leaves moved by that pose (origin in metres, axes as unit vectors). */
export function rigCheck(asset: Json): string[] {
  const out: string[] = [];
  const check = (name: string, offsets: Json, move: (b: Built) => Move) => {
    const frames = solveChain(asset.rig, offsets, ROOT);
    let err = 0;
    for (const b of ALL) {
      const p = placed(b, clear(b, move(b)));
      const [fa, fb] = [frames[petioleId(b.leaf.k)]!.frame, frames[bladeId(b.leaf.k)]!.frame];
      const j = add(b.p0, mul(dirOf(p.az, p.el), b.La));
      const want: [V3, V3][] = [[fa.o, b.p0], [fa.u, dirOf(p.az, p.el)], [fa.v, upOf(p.az, p.el)], [fb.o, j], [fb.u, dirOf(p.az, p.elB)], [fb.v, upOf(p.az, p.elB)], [fb.w, sideOf(p.az)]];
      for (const [got, exp] of want) err = Math.max(err, ...[0, 1, 2].map((i) => Math.abs(got[i]! - exp[i]!)));
    }
    out.push(`${name}: frame error ${err.toExponential(2)}`);
  };
  for (const p of asset.poses as Json[]) check(`pose ${p.id}`, p.bones, POSE_MOVES[p.id]!);
  for (const c of asset.clips as Json[]) for (const k of c.keys as Json[]) check(`clip ${c.id} t ${k.t}`, k.bones, (b) => CLIP_MOVES[c.id]!(b, k.t));
  return out;
}
