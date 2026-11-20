/** 🦴️ Round 3, vegetation/agave: root fix of the rig. Twenty-eight leaves, each on its own proximal and distal bone from its insertion on the stem (instead of ten bone pairs from one point standing for two to three leaves each), the spear on its own bone; poses and clip keys re-solved so that every leaf keeps the change of azimuth and elevation that the earlier poses declared for the leaf group nearest to it; wind loads per leaf with the same total drag; transpiration anchors on the upper face of real leaves; the root plate's mass centre at the stem axis.
 * @see ./r3-vegetation-agave.md */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { STAFFAGE, TICKET, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, azElOf, dirOf, dot, mul, r4, solveOffsets, table, upOf } from "./r3-agave-kit.ts";
import { KNEE, LEAVES, SPEAR, blade, leafFrames, leafId, regionOf, thickness, type Leaf } from "./r3-agave-plant.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

const text = (en: string, de: string) => ({ en, de });
const OLD_GROUPS = ["r1", "r2", "r3", "r4", "r5", "l1", "l2", "l3", "l4", "l5"];
const oldA = (g: string) => (g.endsWith("5") ? `leaf-${g}` : `leaf-${g}-a`);
const oldB = (g: string) => (g.endsWith("5") ? `leaf-${g}` : `leaf-${g}-b`);
const ROOT = { x: 0, y: 0, angleDeg: 0 };

/** 🍃️ Chord geometry of every leaf in the default pose. */
export type Chords = { leaf: Leaf; az: number; elA: number; elB: number; La: number; Lb: number; restA: number; restB: number; fa: Frame; fb: Frame };

export function chords(): Chords[] {
  return LEAVES.map((l) => {
    const b = blade(l);
    const { a: fa, b: fb, La, Lb } = leafFrames(b);
    const elA = azElOf(fa.u).el, elB = azElOf(fb.u).el;
    const restA = r4(Math.atan2(-fa.u[0], fa.u[1]) / D2R);
    return { leaf: l, az: l.az, elA, elB, La: r4(La), Lb: r4(Lb), restA, restB: r4(elB - elA), fa, fb };
  });
}

/** 🌬️ The change of azimuth and of the chord elevations that an old pose or key applies to each old leaf group against the default pose. */
function groupDeltas(old: Json, offsets: Json, calm: Record<string, { frame: Frame }>) {
  const now = solveChain(old.rig, offsets, ROOT);
  return OLD_GROUPS.map((g) => {
    const [a0, b0, a1, b1] = [azElOf(calm[oldA(g)]!.frame.u), azElOf(calm[oldB(g)]!.frame.u), azElOf(now[oldA(g)]!.frame.u), azElOf(now[oldB(g)]!.frame.u)];
    let dAz = a1.az - a0.az;
    while (dAz > 180) dAz -= 360;
    while (dAz < -180) dAz += 360;
    return { dir: calm[oldA(g)]!.frame.u, dAz, dA: a1.el - a0.el, dB: b1.el - a1.el - (b0.el - a0.el) };
  });
}

/** 🎯️ The deltas of a new leaf: inverse-square weighting over the old groups by the angle between the default-pose chord directions. */
function blend(groups: ReturnType<typeof groupDeltas>, d: V3) {
  let [sw, az, a, b] = [0, 0, 0, 0];
  for (const g of groups) {
    const ang = Math.acos(Math.max(-1, Math.min(1, dot(g.dir, d))));
    const w = 1 / (ang * ang + 0.01);
    [sw, az, a, b] = [sw + w, az + w * g.dAz, a + w * g.dA, b + w * g.dB];
  }
  return { dAz: az / sw, dA: a / sw, dB: b / sw };
}

/** 🌍️ Least height of a leaf tip above the bed in every pose and clip key: a leaf that the wind or the frost presses down comes to rest on the ground, it does not pass through it. */
const GROUND_CLEARANCE = 0.025;

const ANCHORS = {
  spreading: [0, 2, 4, 6, 9, 12],
  erect: [17, 19, 21, 23, 25, 26],
};

/** 🔧️ Rewrites rig, poses, clips, loads, anchors and emitters of `asset` in place from the baseline `r3-agave-baseline.json`. */
export function fixRig(asset: Json): { chords: Chords[]; report: string[] } {
  const old = JSON.parse(readFileSync(join(TICKET, "r3-agave-baseline.json"), "utf8"));
  const C = chords();
  const report: string[] = [];
  const total = C.reduce((s, c) => s + c.leaf.L * c.leaf.W * (c.leaf.Tm + c.leaf.Tb / 6), 0);
  const LEAF_KG = 35.4;
  const [rootBone, caudexBone] = [old.rig.bones.find((b: Json) => b.id === "root-plate"), old.rig.bones.find((b: Json) => b.id === "caudex")];
  const bones: Json[] = [
    { ...rootBone, mass: { ...rootBone.mass, comX: 0, comY: 0.03 } },
    caudexBone,
  ];
  const limitA: Record<string, number> = {};
  const solvedFor = (offsets: Json) => {
    const groups = groupDeltas(old, offsets, solveChain(old.rig, old.poses.find((p: Json) => p.id === "summer-turgid").bones, ROOT));
    const out: Json = {};
    const caudex: Frame = { o: [0, 0, 0], u: [0, 1, 0], v: [-1, 0, 0], w: [0, 0, 1] };
    for (const c of C) {
      const g = blend(groups, c.fa.u);
      const [az, ea] = [c.az + g.dAz, c.elA + g.dA];
      const o = solveOffsets(caudex, dirOf(az, ea), upOf(az, ea), c.restA);
      out[`${leafId(c.leaf.k)}-a`] = o;
      limitA[leafId(c.leaf.k)] = Math.max(limitA[leafId(c.leaf.k)] ?? 0, Math.abs(o.angleDeg), Math.abs(o.abductionDeg));
      const tipY = (b: number) => c.leaf.h + c.La * Math.sin(ea * D2R) + c.Lb * Math.sin((ea + c.elB - c.elA + b) * D2R);
      let lifted = g.dB;
      for (let i = 0; i < 80 && tipY(lifted) < GROUND_CLEARANCE; i++) lifted += 0.25;
      const bend = r4(lifted);
      if (Math.abs(bend) >= 0.0001) out[`${leafId(c.leaf.k)}-b`] = { angleDeg: bend };
    }
    if (offsets["leaf-s"] !== undefined) out.spear = offsets["leaf-s"];
    return out;
  };
  const poses = old.poses.map((p: Json) => ({ ...p, bones: solvedFor(p.bones) }));
  const clips = old.clips.map((c: Json) => ({ ...c, keys: c.keys.map((k: Json) => ({ ...k, bones: solvedFor(k.bones ?? old.poses.find((p: Json) => p.id === k.pose).bones) })) }));
  for (const c of C) {
    const id = leafId(c.leaf.k);
    const n = c.leaf.k + 1;
    const tier = c.leaf.k < 8 ? text("lowest, spreading", "unterste, ausgebreitet") : c.leaf.k < 16 ? text("rising", "aufsteigend") : c.leaf.k < 23 ? text("ascending", "aufstrebend") : text("young, upright", "jung, aufrecht");
    const kg = (LEAF_KG * c.leaf.L * c.leaf.W * (c.leaf.Tm + c.leaf.Tb / 6)) / total;
    const lim = Math.min(90, Math.ceil((limitA[id]! + 10) / 5) * 5);
    bones.push({
      id: `${id}-a`, parent: "caudex",
      title: text(`Leaf ${n} (${tier.en}), base and lower blade`, `Blatt ${n} (${tier.de}), Basis und unteres Blatt`),
      x: r4(c.leaf.h), y: r4(-c.leaf.r * Math.cos(c.az * D2R)), z: r4(-c.leaf.r * Math.sin(c.az * D2R)), angleDeg: c.restA, lengthM: c.La,
      joint: { type: "revolute", minDeg: -lim, maxDeg: lim },
      mass: { kg: r4(kg * 0.68), comX: r4(c.La * 0.4), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "nobel-1988" },
    });
    bones.push({
      id: `${id}-b`, parent: `${id}-a`,
      title: text(`Leaf ${n} (${tier.en}), upper blade and spine`, `Blatt ${n} (${tier.de}), oberes Blatt und Endstachel`),
      x: c.La, y: 0, angleDeg: c.restB, lengthM: c.Lb,
      joint: { type: "revolute", minDeg: -35, maxDeg: 35 },
      mass: { kg: r4(kg * 0.32), comX: r4(c.Lb * 0.38), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "nobel-1988" },
    });
  }
  const oldSpear = old.rig.bones.find((b: Json) => b.id === "leaf-s");
  bones.push({ ...oldSpear, id: "spear", title: text("Spear: the cone of furled young leaves in the heart", "Herzspeer: der Kegel eingerollter junger Blätter im Herzen"), x: SPEAR.base, lengthM: SPEAR.length, mass: { ...oldSpear.mass, comX: r4(SPEAR.length * 0.35) } });
  asset.rig.bones = bones;
  report.push(`rig: 28 leaves x 2 bones + spear (${C.map((c) => `${leafId(c.leaf.k)} az ${c.az} el ${c.elA.toFixed(1)}/${c.elB.toFixed(1)} L ${c.La}+${c.Lb}`).join("; ")})`);
  asset.poses = poses.map((p: Json) => {
    const winds = (p.loads ?? []).filter((l: Json) => l.kind === "wind");
    if (winds.length === 0) return p;
    const sum = winds.reduce((s: number, l: Json) => s + l.fxN, 0);
    const per = Math.round((sum / 29) * 1000) / 1000;
    const sample = winds[0];
    const loads = [
      ...C.map((c) => ({ ...sample, id: `wind-${leafId(c.leaf.k)}`, title: text(`Wind drag on leaf ${c.leaf.k + 1}`, `Winddruck auf Blatt ${c.leaf.k + 1}`), bone: `${leafId(c.leaf.k)}-a`, x: r4(c.La * 0.8), y: 0, fxN: per })),
      { ...sample, id: "wind-spear", title: text("Wind drag on the spear", "Winddruck auf den Herzspeer"), bone: "spear", x: r4(SPEAR.length * 0.45), y: 0, fxN: per },
      ...(p.loads ?? []).filter((l: Json) => l.kind !== "wind"),
    ];
    report.push(`pose ${p.id}: 29 wind loads of ${per} N (was ${winds.length} of ${winds[0].fxN} N, total ${sum.toFixed(3)} N)`);
    return { ...p, loads };
  });
  asset.clips = clips;
  const anchorOf = (k: number, n: number, group: "spreading" | "erect") => {
    const c = C[k]!;
    const b = blade(c.leaf);
    const s = KNEE + (1 - KNEE) * 0.35;
    const p = add(b.at(s), mul(b.normal(s), thickness(c.leaf, s) * 0.27));
    const loc = [dot(sub3(p, c.fb.o), c.fb.u), dot(sub3(p, c.fb.o), c.fb.v)];
    return { id: `tp-${group}-${n}`, bone: `${leafId(k)}-b`, x: r4(loc[0]!), y: r4(loc[1]!), kind: "emitter", title: text(`Transpiration of leaf ${k + 1}, outlet ${n}`, `Transpiration von Blatt ${k + 1}, Auslass ${n}`) };
  };
  const emitterAnchors = [...ANCHORS.spreading.map((k, i) => anchorOf(k, i + 1, "spreading")), ...ANCHORS.erect.map((k, i) => anchorOf(k, i + 1, "erect")), { id: "tp-erect-7", bone: "spear", x: r4(SPEAR.length * 0.7), y: r4(table(SPEAR.profile, SPEAR.length * 0.7) + 0.002), kind: "emitter", title: text("Transpiration of the spear, outlet 7", "Transpiration des Herzspeers, Auslass 7") }];
  asset.anchors = [...asset.anchors.filter((a: Json) => a.kind !== "emitter"), ...emitterAnchors];
  for (const e of asset.thermal.emitters) {
    const first = emitterAnchors.find((a) => a.id === e.anchor)!;
    const f = first.bone === "spear" ? { u: [0, 1, 0] as V3, v: [-1, 0, 0] as V3 } : C[Number(first.bone.slice(5, 7)) - 1]!.fb;
    e.directionDeg = Math.round(Math.atan2(f.v[1], f.u[1]) / D2R);
  }
  report.push(`anchors: ${emitterAnchors.map((a) => `${a.id} on ${a.bone} at ${a.x}, ${a.y}`).join("; ")}; emitter directions ${asset.thermal.emitters.map((e: Json) => `${e.id} ${e.directionDeg}`).join(", ")}`);
  report.push(`regions: leaves 1-16 ${regionOf(0)}, 17-28 and spear ${regionOf(20)}`);
  return { chords: C, report };
}

const sub3 = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];

/** 🧾️ Checks the solved default pose against the stated chords: azimuth, elevations and the normal of every leaf. */
export function rigCheck(asset: Json, pose: string): string[] {
  const frames = solveChain(asset.rig, asset.poses.find((p: Json) => p.id === pose).bones, ROOT);
  return chords().map((c) => {
    const [fa, fb] = [frames[`${leafId(c.leaf.k)}-a`]!.frame, frames[`${leafId(c.leaf.k)}-b`]!.frame];
    const err = Math.max(...[0, 1, 2].map((i) => Math.abs(fa.o[i]! - c.fa.o[i]!) + Math.abs(fa.u[i]! - c.fa.u[i]!) + Math.abs(fa.v[i]! - c.fa.v[i]!) + Math.abs(fb.o[i]! - c.fb.o[i]!) + Math.abs(fb.u[i]! - c.fb.u[i]!) + Math.abs(fb.v[i]! - c.fb.v[i]!)));
    return `${leafId(c.leaf.k)} ${pose}: frame error ${err.toExponential(2)}`;
  });
}
