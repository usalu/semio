/** 🦴 Root fixes of `humans/leaning-person` (round 3), stated as absolute values so the script can run any number of times: rig segment lengths after Winter 2009 for a 1.72 m man in 20 mm sneaker soles, anchors on the modelled body, every pose re-solved (legs by two-link geometry, arms by a small optimiser on the product's own chain), clips, dimensions and, when present, the redrawn 2D shapes and the representation decisions. Writes the candidate document `🗑️generated/r3-leaning-person/🔣️.json`; `--commit` writes the owned members into the catalog file (re-read right before).
 *
 * Usage: `bun r3-leaning-person-roots.ts [--fit] [--commit]` (`--fit` places each pose against the wall from the 1:50 body of the volumes source)
 * @see ./r3-humans-leaning-person.md */
import { fileURLToPath } from "node:url";
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { dump } from "./r3-leaning-person-fmt.ts";
import { CANDIDATE, OUT, SOURCE } from "./r3-leaning-person-io.ts";
import { PHONE, RIG } from "./r3-leaning-person-data.ts";

type Json = any;
type V3 = [number, number, number];
const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/leaning-person/🔣️.json";
const MANIFEST = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json";
const OWNED = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "poses", "clips", "thermal", "structure", "representations"];
const ORIGINAL = `${OUT}/original.json`;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const deg = (r: number): number => (r * 180) / Math.PI;
const rad = (d: number): number => (d * Math.PI) / 180;


mkdirSync(OUT, { recursive: true });
if (!existsSync(ORIGINAL)) writeFileSync(ORIGINAL, readFileSync(CATALOG, "utf8"));
const doc = JSON.parse(readFileSync(ORIGINAL, "utf8")) as Json;
const asset = doc.asset as Json;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);

/* ------------------------------------------------------------------ materials: the skeleton of the structure list */
if (!asset.materials.some((m: Json) => m.id === "bone")) asset.materials.push({ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" });

/* ------------------------------------------------------------------ rig */
Object.assign(bone("torso"), { lengthM: RIG.torso });
bone("torso").mass.comX = r4(0.4565 * RIG.torso);
Object.assign(bone("head"), { x: RIG.torso, lengthM: RIG.head, title: { en: "Head and neck", de: "Kopf und Hals" } });
bone("head").mass.comX = r4(0.5 * RIG.head);
for (const s of ["l", "r"]) {
  Object.assign(bone(`upper-arm-${s}`), { lengthM: RIG.upperArm });
  bone(`upper-arm-${s}`).mass.comX = r4(0.5772 * RIG.upperArm);
  Object.assign(bone(`forearm-${s}`), { x: RIG.upperArm, lengthM: RIG.forearm });
  bone(`forearm-${s}`).mass.comX = r4(0.4574 * RIG.forearm);
  Object.assign(bone(`hand-${s}`), { x: RIG.forearm, lengthM: RIG.hand });
  bone(`hand-${s}`).mass.comX = r4(0.5 * RIG.hand);
  Object.assign(bone(`toe-${s}`), { y: -RIG.ankle });
  bone(`foot-${s}`).mass.comY = r4(-RIG.ankle / 2);
}

/* ------------------------------------------------------------------ anchors (positions on the modelled body, see the 1:50 model) */
export const ANCHORS: Record<string, [number, number]> = {
  "contact-l": [0.127, -RIG.ankle], "contact-r": [0.127, -RIG.ankle], "heel-l": [-0.03, -RIG.ankle], "heel-r": [-0.03, -RIG.ankle],
  mouth: [0.06, -0.1], nostrils: [0.083, -0.112], crown: [RIG.head, 0.012],
  "grip-l": [0.075, 0], "grip-r": [0.075, 0], "phone-ear": [PHONE.u, PHONE.v + 0.06],
  "wall-shoulder": [0.37, 0.142], "wall-seat": [-0.02, 0.125], lumbar: [0, 0],
};
for (const [id, [x, y]] of Object.entries(ANCHORS)) Object.assign(anchor(id), { x, y });
anchor("phone-ear").z = 0.0255;
for (const s of ["l", "r"]) {
  Object.assign(anchor(`contact-${s}`), { footprintLengthM: 0.11, footprintWidthM: 0.095 });
  Object.assign(anchor(`heel-${s}`), { footprintLengthM: 0.08, footprintWidthM: 0.07 });
}

/* ------------------------------------------------------------------ poses */
const cl = (x: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, x));
const chord = (k: number): number => Math.sqrt(RIG.thigh ** 2 + RIG.shank ** 2 + 2 * RIG.thigh * RIG.shank * Math.cos(rad(k)));
/** 🦵 Thigh, shank and foot offsets that put the ankle `dx` ahead of the hip and `h` below it with the sole level; the knee bends forwards. */
function leg(rootDeg: number, dx: number, h: number): Json {
  const d = Math.hypot(dx, h);
  const gamma = Math.acos(cl((RIG.thigh ** 2 + RIG.shank ** 2 - d * d) / (2 * RIG.thigh * RIG.shank), -1, 1));
  const alpha = Math.acos(cl((RIG.thigh ** 2 + d * d - RIG.shank ** 2) / (2 * RIG.thigh * d), -1, 1));
  const thighW = deg(Math.atan2(-h, dx) + alpha);
  const kappa = 180 - deg(gamma);
  const pelvisW = 90 + rootDeg;
  const t = thighW - (pelvisW - 180);
  const shankW = thighW - kappa;
  return { thigh: r2(t), shank: r2(-kappa), foot: r2(-(shankW + 90)) };
}
/** 🦿 Both legs: the left ankle `dxL` ahead of the hip with the knee bent `kneeL`; the right ankle `dxR` ahead at the same height. */
function legs(rootDeg: number, dxL: number, kneeL: number, dxR: number): Json {
  const h = Math.sqrt(chord(kneeL) ** 2 - dxL * dxL);
  const l = leg(rootDeg, dxL, h);
  const r = leg(rootDeg, dxR, h);
  return { "thigh-l": { angleDeg: l.thigh }, "shank-l": { angleDeg: l.shank }, "foot-l": { angleDeg: l.foot }, "thigh-r": { angleDeg: r.thigh }, "shank-r": { angleDeg: r.shank }, "foot-r": { angleDeg: r.foot }, "toe-l": { angleDeg: 0 }, "toe-r": { angleDeg: 0 } };
}

const light = (): Json => ({ ...asset, representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]: [string, Json]) => [k, { ...v, geometry: { elevation: [], front: [], plan: [] } }])) });
function frames(poseId: string): Record<string, Frame> {
  const s = evaluate(light() as never, { kind: "pose", pose: poseId } as never) as Json;
  const v = (p: Json): V3 => [p.x, p.y, p.z ?? 0];
  const out: Record<string, Frame> = {};
  for (const b of asset.rig.bones) out[b.id] = { o: v(s.bones[b.id].frame.o), u: v(s.bones[b.id].frame.u), v: v(s.bones[b.id].frame.v), w: v(s.bones[b.id].frame.w) };
  return out;
}
export const at = (f: Frame, p: readonly number[]): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + (p[2] ?? 0) * f.w[k]!) as V3;
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const len = (a: V3): number => Math.hypot(...a);
const nrm = (a: V3): V3 => { const l = len(a); return [a[0] / l, a[1] / l, a[2] / l]; };
const dotv = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;

/** 🎯 Nelder-Mead over the named bone offsets of a pose, minimising `cost(frames)`. */
function solve(poseId: string, params: [string, string, number, number, number][], cost: (f: Record<string, Frame>) => number): void {
  const p = pose(poseId);
  const set = (x: number[]): void => params.forEach(([b, key, , lo, hi], i) => { p.bones[b] = { ...(p.bones[b] ?? {}), [key]: cl(x[i]!, lo, hi) }; });
  const f = (x: number[]): number => { set(x); return cost(frames(poseId)) + x.reduce((s, xi, i) => s + (xi < params[i]![3] || xi > params[i]![4] ? 1 : 0), 0); };
  const n = params.length;
  let simplex = [params.map((q) => q[2])];
  for (let i = 0; i < n; i++) simplex.push(simplex[0]!.map((x, j) => (j === i ? x + 12 : x)));
  let values = simplex.map(f);
  for (let it = 0; it < 900; it++) {
    const order = values.map((v, i) => [v, i] as const).sort((a, b) => a[0] - b[0]).map((q) => q[1]);
    simplex = order.map((i) => simplex[i]!);
    values = order.map((i) => values[i]!);
    if (values[n]! - values[0]! < 1e-10 && it > 50) break;
    const c = simplex[0]!.map((_, j) => simplex.slice(0, n).reduce((s, x) => s + x[j]!, 0) / n);
    const xr = c.map((cj, j) => cj + (cj - simplex[n]![j]!));
    const fr = f(xr);
    if (fr < values[0]!) {
      const xe = c.map((cj, j) => cj + 2 * (cj - simplex[n]![j]!));
      const fe = f(xe);
      [simplex[n], values[n]] = fe < fr ? [xe, fe] : [xr, fr];
    } else if (fr < values[n - 1]!) [simplex[n], values[n]] = [xr, fr];
    else {
      const xc = c.map((cj, j) => cj + 0.5 * (simplex[n]![j]! - cj));
      const fc = f(xc);
      if (fc < values[n]!) [simplex[n], values[n]] = [xc, fc];
      else { simplex = simplex.map((x) => x.map((xj, j) => simplex[0]![j]! + 0.5 * (xj - simplex[0]![j]!))); values = simplex.map(f); }
    }
  }
  const best = f(simplex[0]!);
  for (const [b, key] of params) p.bones[b][key] = r2(p.bones[b][key]);
  console.log(`[DEBUG] ${poseId}: cost ${best.toExponential(2)} ${params.map(([b, k]) => `${b}.${k}=${p.bones[b][k]}`).join(" ")}`);
}
/** 📍 A point of the trunk: `a` up the torso bone, `fwd` ahead of it, `lat` to the left. */
const T = (f: Record<string, Frame>, a: number, fwd: number, lat: number): V3 => at(f.torso!, [a, -fwd, lat]);
/** 🗿 A point of the head: `a` up the head bone from the neck base, `fwd` ahead, `lat` to the left. */
const Hd = (f: Record<string, Frame>, a: number, fwd: number, lat: number): V3 => at(f.head!, [a, -fwd, lat]);
const d2 = (a: V3, b: V3): number => dotv(sub(a, b), sub(a, b));

type PoseSpec = { root: number; x: number; torso: number; head: number; legs: [number, number, number]; arms: Json };
const SPECS: Record<string, PoseSpec> = {
  "leaning-shoulder-wall": { root: 11, x: 0.2, torso: 0, head: -32, legs: [0.085, 8, 0.05], arms: { "upper-arm-l": { angleDeg: -9, abductionDeg: 7, twistDeg: 0 }, "forearm-l": { angleDeg: 16 }, "hand-l": { angleDeg: 6 } } },
  "leaning-arms-folded": { root: 4, x: 0.16, torso: 0, head: -6, legs: [0.15, 7, 0.12], arms: {} },
  "phone-call": { root: 9, x: 0.19, torso: 0, head: -14, legs: [0.0, 14, 0.09], arms: {} },
  "pushing-off": { root: -2, x: 0.16, torso: -4, head: 5, legs: [-0.01, 12, -0.03], arms: { "upper-arm-l": { angleDeg: 10, abductionDeg: 6, twistDeg: 0 }, "forearm-l": { angleDeg: 18 }, "hand-l": { angleDeg: 4 }, "upper-arm-r": { angleDeg: 12, abductionDeg: -6, twistDeg: 0 }, "forearm-r": { angleDeg: 20 }, "hand-r": { angleDeg: 4 } } },
  "standing-away": { root: 0, x: 0.33, torso: 0, head: 0, legs: [-0.04, 4, -0.035], arms: { "upper-arm-l": { angleDeg: 2, abductionDeg: 6, twistDeg: 0 }, "forearm-l": { angleDeg: 9 }, "hand-l": { angleDeg: 4 }, "upper-arm-r": { angleDeg: 2, abductionDeg: -6, twistDeg: 0 }, "forearm-r": { angleDeg: 9 }, "hand-r": { angleDeg: 4 } } },
};
const FIT = existsSync(`${OUT}/fit.json`) ? (JSON.parse(readFileSync(`${OUT}/fit.json`, "utf8")) as Record<string, { x: number; root: number }>) : {};
for (const [id, s] of Object.entries(SPECS)) {
  const p = pose(id);
  const root = FIT[id]?.root ?? s.root;
  p.root = { x: FIT[id]?.x ?? s.x, angleDeg: root };
  const keep = p.bones;
  p.bones = { ...keep, torso: { angleDeg: s.torso }, head: { angleDeg: s.head }, ...s.arms, ...legs(root, ...s.legs) };
  for (const k of Object.keys(p.bones)) if (k.startsWith("wall")) delete p.bones[k];
}

const arm = (side: "l" | "r", a: number, ab: number, tw: number, fa: number, ha: number, ft = 0): [string, string, number, number, number][] => [
  [`upper-arm-${side}`, "angleDeg", a, -60, 180], [`upper-arm-${side}`, "abductionDeg", ab, -90, 90], [`upper-arm-${side}`, "twistDeg", tw, -100, 100], [`forearm-${side}`, "angleDeg", fa, 0, 145], [`forearm-${side}`, "twistDeg", ft, -90, 90], [`hand-${side}`, "angleDeg", ha, -65, 75],
];
/** 📱 The phone centre, its long axis and its screen normal in the world. */
const phone = (f: Record<string, Frame>): { c: V3; axis: V3; screen: V3 } => ({ c: at(f["hand-r"]!, [PHONE.u, PHONE.v, PHONE.w]), axis: f["hand-r"]!.v, screen: f["hand-r"]!.w });
/** 👀 The midpoint between the eyes. */
const eyes = (f: Record<string, Frame>): V3 => Hd(f, 0.142, 0.072, 0);

solve("leaning-shoulder-wall", arm("r", 15, 8, 20, 100, 0, -60), (f) => {
  const e = eyes(f);
  const target: V3 = [e[0] + 0.32 * Math.cos(rad(-52)), e[1] + 0.32 * Math.sin(rad(-52)), -0.04];
  const ph = phone(f);
  const toEye = nrm(sub(e, ph.c));
  const away = nrm([toEye[1], -toEye[0], 0]);
  const elbow = at(f["forearm-r"]!, [0, 0, 0]);
  return d2(ph.c, target) + 0.004 * (1 - dotv(ph.screen, toEye)) + 0.002 * (1 - dotv(ph.axis, away)) + 1.0 * d2(elbow, T(f, 0.13, 0.03, -0.205));
});
solve("phone-call", arm("r", 50, -10, 20, 140, 0, 0), (f) => {
  const ear = Hd(f, 0.118, 0.0, -0.078);
  const mouth = Hd(f, 0.065, 0.1, -0.03);
  const ph = phone(f);
  const want: V3 = [ear[0] + 0.03, ear[1] - 0.03, ear[2] - 0.016];
  const axis = nrm(sub(ear, mouth));
  const medial = nrm(sub(Hd(f, 0.118, 0.0, 0), ear));
  const elbow = at(f["forearm-r"]!, [0, 0, 0]);
  return d2(ph.c, want) + 0.004 * (1 - dotv(ph.axis, axis)) + 0.004 * (1 - dotv(ph.screen, medial)) + 0.03 * d2(elbow, T(f, 0.26, 0.2, -0.22));
});
solve("phone-call", arm("l", 0, 25, 40, 95, -20, -60), (f) => {
  const wrist = at(f["hand-l"]!, [0, 0, 0]);
  const mid = at(f["hand-l"]!, [0.1, 0, 0]);
  const palmIn = dotv(f["hand-l"]!.w, f.torso!.v);
  return d2(wrist, T(f, -0.04, 0.122, 0.128)) + 2 * d2(mid, T(f, -0.07, 0.075, 0.05)) + 0.0005 * (1 - palmIn) + 0.2 * d2(at(f["forearm-l"]!, [0, 0, 0]), T(f, 0.06, -0.02, 0.235));
});
solve("leaning-arms-folded", arm("l", 30, 10, 60, 110, 10), (f) => {
  const wrist = at(f["hand-l"]!, [0, 0, 0]);
  const tip = at(f["hand-l"]!, [0.12, 0, 0]);
  return d2(wrist, T(f, 0.245, 0.175, -0.11)) + d2(tip, T(f, 0.27, 0.1, -0.215)) + 0.3 * d2(at(f["forearm-l"]!, [0, 0, 0]), T(f, 0.17, 0.1, 0.215));
});
solve("leaning-arms-folded", arm("r", 30, -10, -60, 110, 10), (f) => {
  const wrist = at(f["hand-r"]!, [0, 0, 0]);
  const tip = at(f["hand-r"]!, [0.12, 0, 0]);
  return d2(wrist, T(f, 0.2, 0.15, 0.1)) + d2(tip, T(f, 0.22, 0.07, 0.2)) + 0.3 * d2(at(f["forearm-r"]!, [0, 0, 0]), T(f, 0.15, 0.1, -0.215));
});

/** 🧱 The wall bones follow the root: world-aligned frame, face at x = 0, foot on the ground. */
for (const p of asset.poses) {
  const f = frames(p.id);
  p.bones["wall-frame"] = { angleDeg: r4(-p.root.angleDeg) };
  p.bones["wall-x"] = { dx: r4(-f.pelvis!.o[0]) };
  p.bones.wall = { dy: r4(-f.pelvis!.o[1]) };
}

/** 🧲 `--fit`: root x (and the trunk angle of the folded pose) from the 1:50 body: the body's rearmost point on the wall plane x = 0, or 0.2 m off it standing away. */
if (process.argv.includes("--fit") && existsSync(SOURCE)) {
  const body = (JSON.parse(readFileSync(SOURCE, "utf8")).volumes["50"].body as Json[]).filter((s) => s.bone !== "wall");
  const minX = (poseId: string, band: [number, number] = [-9, 9]): number => {
    const f = rigAt(light(), 50, { kind: "pose", pose: poseId }).frames;
    const hy = f.pelvis!.o[1];
    let m = Infinity;
    for (const mesh of meshesIn(asset, f, body)) for (let i = 0; i < mesh.world.length; i += 3) { const y = mesh.world[i + 1]! - hy; if (y >= band[0] && y <= band[1]) m = Math.min(m, mesh.world[i]!); }
    return m;
  };
  const fit: Record<string, { x: number; root: number }> = {};
  const gap: Record<string, number> = { "leaning-shoulder-wall": 0, "leaning-arms-folded": 0, "phone-call": 0, "pushing-off": 0.002, "standing-away": 0.2 };
  for (const p of asset.poses) {
    if (p.id === "leaning-arms-folded") {
      let lo = -4, hi = 9;
      for (let i = 0; i < 14; i++) {
        const mid = (lo + hi) / 2;
        p.root.angleDeg = mid;
        Object.assign(p.bones, legs(mid, ...SPECS[p.id]!.legs));
        const seat = minX(p.id, [-0.15, 0.05]);
        const back = minX(p.id, [0.3, 0.7]);
        if (process.env.DBG) console.log(`[DEBUG] folded ${mid.toFixed(2)} seat ${seat.toFixed(4)} back ${back.toFixed(4)}`);
        if (back < seat) hi = mid; else lo = mid;
      }
    }
    p.root.x = r4(p.root.x - minX(p.id) + gap[p.id]!);
    fit[p.id] = { x: p.root.x, root: r2(p.root.angleDeg) };
  }
  writeFileSync(`${OUT}/fit.json`, JSON.stringify(fit, null, 1));
  console.log("[DEBUG] fit", JSON.stringify(fit));
  for (const p of asset.poses) { const f = frames(p.id); p.bones["wall-frame"] = { angleDeg: r4(-p.root.angleDeg) }; p.bones["wall-x"] = { dx: r4(-f.pelvis!.o[0]) }; p.bones.wall = { dy: r4(-f.pelvis!.o[1]) }; }
}

/* ------------------------------------------------------------------ texts of poses and clips */
pose("leaning-shoulder-wall").description = {
  en: "The body rests 11 degrees backwards with the upper back on the wall and the feet 0.3 m ahead of it, knees almost straight; the right hand holds the phone at the waist and the head bends down to look at the screen. The wall presses horizontally and the feet resist by friction.",
  de: "Der Körper ruht 11 Grad nach hinten geneigt mit dem oberen Rücken an der Wand und den Füßen 0,3 m davor, die Knie fast gestreckt; die rechte Hand hält das Telefon auf Taillenhöhe und der Kopf neigt sich zum Bildschirm. Die Wand drückt horizontal, die Füße halten durch Reibung.",
};
pose("leaning-arms-folded").description = { en: "Back almost flat against the wall with the upper back and the seat touching, forearms crossed in front of the chest, each hand on the other upper arm.", de: "Rücken fast flach an der Wand mit Kontakt an oberem Rücken und Gesäß, die Unterarme vor der Brust gekreuzt, jede Hand am anderen Oberarm." };
pose("phone-call").description = { en: "Leaning on the wall with the left hand in the front pocket of the hoodie and the phone held to the right ear, head bent a little towards the phone, weight on the straight right leg.", de: "An die Wand gelehnt, die linke Hand in der Bauchtasche des Pullovers und das Telefon am rechten Ohr, der Kopf leicht zum Telefon geneigt, das Gewicht auf dem gestreckten rechten Bein." };
pose("pushing-off").description = { en: "The feet stand under the body, the trunk tips forwards and the upper back just leaves the wall; the wall reaction drops to zero.", de: "Die Füße stehen unter dem Körper, der Rumpf kippt nach vorn und der obere Rücken löst sich eben von der Wand; die Wandreaktion sinkt auf null." };
pose("standing-away").description = { en: "Upright with the back 0.2 m in front of the wall, knees straight, weight over both feet.", de: "Aufrecht mit dem Rücken 0,2 m vor der Wand, Knie gestreckt, das Gewicht über beiden Füßen." };

const idle = asset.clips.find((c: Json) => c.id === "phone-call-idle");
const ua = pose("phone-call").bones["upper-arm-r"].angleDeg;
const ul = pose("phone-call").bones["upper-arm-l"].angleDeg;
idle.keys[1].bones = { head: { angleDeg: -9 }, "upper-arm-r": { angleDeg: r2(ua + 3) }, torso: { angleDeg: 1.5 } };
idle.keys[2].bones = { head: { angleDeg: -19 }, "upper-arm-l": { angleDeg: r2(ul + 3) }, torso: { angleDeg: -0.5 } };
idle.keys[3].bones = { head: { angleDeg: -11 }, "upper-arm-r": { angleDeg: r2(ua - 3) } };
idle.description = { en: "Talking on the phone: slow weight shifts, the head nods and the arm in the pocket shrugs, while the upper back stays on the wall and breathing alternates between talking and listening.", de: "Telefonieren: langsame Gewichtsverlagerungen, der Kopf nickt und der Arm in der Tasche zuckt, während der obere Rücken an der Wand bleibt und die Atmung zwischen Sprechen und Zuhören wechselt." };
const push = asset.clips.find((c: Json) => c.id === "push-off");
push.description = { en: "The person sets the feet under the body, the upper back leaves the wall at 1.4 s and the wall reaction vanishes; at the end the person stands upright 0.2 m in front of the wall.", de: "Die Person setzt die Füße unter den Körper, bei 1,4 s löst sich der obere Rücken von der Wand und die Wandreaktion verschwindet; am Ende steht die Person aufrecht 0,2 m vor der Wand." };
push.events[0].title = { en: "Upper back leaves the wall", de: "Oberer Rücken löst sich von der Wand" };
anchor("wall-shoulder").title = { en: "Upper back on the wall", de: "Oberer Rücken an der Wand" };

/* ------------------------------------------------------------------ texts, dimensions, provenance */
asset.summary = {
  en: "A 1.72 m man in hoodie, jeans, beanie and sneakers leaning with the upper back against a masonry wall while looking at or talking on a phone: wall reaction and friction at the feet, heat conduction into the wall, speech breathing and a push-off to free standing; four individually drawn scales.",
  de: "Ein 1,72 m großer Mann in Kapuzenpullover, Jeans, Mütze und Turnschuhen, der mit dem oberen Rücken an einer gemauerten Wand lehnt und aufs Telefon schaut oder telefoniert: Wandreaktion und Reibung an den Füßen, Wärmeleitung in die Wand, Sprechatmung und Abstoßen zum freien Stehen; vier einzeln gezeichnete Maßstäbe.",
};
const refs = asset.provenance.references as Json[];
if (!refs.some((r) => r.id === "gordon-2014")) refs.push({ id: "gordon-2014", citation: "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel (ANSUR II): Methods and Summary Statistics. NATICK/TR-15/007 (head, hand, foot, chest, neck and limb girths and breadths)." });
asset.provenance.notes = {
  en: "Round 3 (2026-11-19): rig after Winter 2009 for 1.72 m barefoot in 20 mm sneaker soles (torso to C7 0.4755 m, head and neck 0.249 m, upper arm 0.318 m, forearm 0.252 m, hand 0.186 m); every pose re-solved on that chain; body, thermal model and skeleton handcrafted per scale; 2D drawings redrawn as views of the 3D body.",
  de: "Runde 3 (2026-11-19): Skelett nach Winter 2009 für 1,72 m barfuß in 20 mm Turnschuhsohlen (Rumpf bis C7 0,4755 m, Kopf mit Hals 0,249 m, Oberarm 0,318 m, Unterarm 0,252 m, Hand 0,186 m); jede Haltung auf dieser Kette neu gelöst; Körper, Wärmemodell und Skelett je Maßstab von Hand modelliert; 2D-Zeichnungen als Ansichten des 3D-Körpers neu gezeichnet.",
};
const DIMS = fileURLToPath(new URL("./r3-leaning-person-dimensions.json", import.meta.url));
if (existsSync(DIMS)) Object.assign(asset.dimensions, JSON.parse(readFileSync(DIMS, "utf8")));
asset.structure.assumptions[0] = { en: "The wall contacts are unilateral supports with normal 0 degrees (horizontal reaction, friction 0.4) at the upper back and the seat; the equal-stiffness assumption distributes the horizontal force between wall and feet.", de: "Die Wandkontakte sind einseitige Auflager mit Normale 0 Grad (horizontale Reaktion, Reibung 0,4) am oberen Rücken und am Gesäß; die Gleichsteifigkeitsannahme verteilt die horizontale Kraft auf Wand und Füße." };

const SHAPES = `${OUT}/shapes.json`;
if (existsSync(SHAPES)) {
  const g = JSON.parse(readFileSync(SHAPES, "utf8")) as Json;
  for (const [scale, views] of Object.entries(g)) Object.assign(asset.representations[scale].geometry, views);
}
const TEXTS = fileURLToPath(new URL("./r3-leaning-person-texts.json", import.meta.url));
if (existsSync(TEXTS)) {
  const t = JSON.parse(readFileSync(TEXTS, "utf8")) as Json;
  for (const [scale, d] of Object.entries(t.representations ?? {})) asset.representations[scale].decisions = d;
}
if (process.argv.includes("--with-volumes") && existsSync(SOURCE)) asset.volumes = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
writeFileSync(CANDIDATE, `${dump(doc)}\n`);
console.log("candidate", CANDIDATE);

if (process.argv.includes("--commit")) {
  const live = JSON.parse(readFileSync(CATALOG, "utf8")) as Json;
  for (const k of OWNED) live.asset[k] = asset[k];
  writeFileSync(CATALOG, `${dump(live)}\n`);
  const man = readFileSync(MANIFEST, "utf8");
  const m = JSON.parse(man) as Json;
  const e = m.entries.find((x: Json) => x.id === "leaning-person" && x.category === "humans");
  if (e.heightM !== asset.dimensions.heightM || e.widthM !== asset.dimensions.widthM) {
    const fresh = readFileSync(MANIFEST, "utf8");
    const re = /("id": "leaning-person",\s*"category": "humans",\s*"title": \{[^{}]*\},\s*"heightM": )([0-9.]+)(,\s*"widthM": )([0-9.]+)/;
    if (!re.test(fresh)) throw new Error("manifest entry not found");
    writeFileSync(MANIFEST, fresh.replace(re, `$1${asset.dimensions.heightM}$3${asset.dimensions.widthM}`));
  }
  console.log("committed", CATALOG);
}
