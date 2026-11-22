/** 🦴 The skeleton and the crutch tubes of `humans/person-on-crutches`, stated by hand for each scale: skull (braincase, facial skeleton, zygomatic arches, mandible), seven cervical, twelve thoracic and five lumbar vertebrae with discs, arches, spinous and transverse processes, clavicles, scapulae with spine, acromion and glenoid, the pelvis (iliac blades, acetabula, pubic and ischial rami, symphysis, sacrum with coccyx) and, at the coarse scales, long bones no thinner than the smallest feature. Ribs, sternum, hands, feet and the fine long bones of the earlier list stay where they are right.
 *
 * Bone-local frames: u along the bone, v its sagittal normal (backwards for a bone that points up, ahead for one that points down), w to the left.
 * Usage: `bun rework-person-on-crutches-skeleton.ts` reads `rework-person-on-crutches-before.json` (the list before this pass) and writes `volumes/humans/person-on-crutches.json`.
 * @see ./rework-brief.md
 * @see ./rework/assets/humans--person-on-crutches.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { Json } from "./volumes-lib.ts";
import { add, capsule, cross, dome, ellipsoid, loft, mirrored, mul, openCatalog, part, revolve, sub, tube, tubeStations, unit, len, type St, type V3 } from "./rework-04-kit.ts";

const HERE = import.meta.dir;
const SOURCE0 = join(HERE, "rework-person-on-crutches-before.json");
const SOURCE = join(HERE, "volumes", "humans", "person-on-crutches.json");
const { asset: ASSET } = openCatalog("humans/person-on-crutches");
const boneLen = (id: string): number => (ASSET.rig.bones as Json[]).find((b) => b.id === id).lengthM as number;
const BONE = { material: "bone", tone: 0.62 };
const b = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, ...BONE });
/** ✍️ A slice across an upright bone: centre (u, v), half-width along w, half-depth along v. */
const st = (u: number, v: number, rw: number, rv: number, e = 2): St => ({ at: [u, v, 0], ex: [0, 0, 1], ey: [0, 1, 0], rx: rw, ry: rv, e });
/** 🪓 A blade through stations of a lower and an upper edge point: the thin plates of scapula and ilium. */
function blade(rows: readonly (readonly [V3, V3])[], thick: number, e = 2.4): St[] {
  const mid = rows.map(([lo, hi]) => mul(add(lo, hi), 0.5));
  return rows.map(([lo, hi], i) => {
    const ey = unit(sub(hi, lo));
    const tangent = sub(mid[Math.min(i + 1, mid.length - 1)]!, mid[Math.max(i - 1, 0)]!);
    return { at: mid[i]!, ex: unit(cross(ey, tangent)), ey, rx: thick / 2, ry: len(sub(hi, lo)) / 2, e };
  });
}

/** 🌀 The line of the vertebral bodies in the torso frame (u from the lumbosacral joint, v behind the bone): lumbar lordosis, thoracic kyphosis, the turn into the neck. */
const SPINE: readonly [number, number][] = [[0, 0.036], [0.04, 0.027], [0.08, 0.022], [0.132, 0.027], [0.201, 0.041], [0.28, 0.056], [0.35, 0.062], [0.421, 0.053], [0.474, 0.036], [0.502, 0.022]];
function spineV(u: number): number {
  for (let i = 0; i + 1 < SPINE.length; i++) {
    const [u0, v0] = SPINE[i]!;
    const [u1, v1] = SPINE[i + 1]!;
    if (u <= u1 || i + 2 === SPINE.length) return v0 + ((v1 - v0) * (u - u0)) / (u1 - u0);
  }
  return SPINE[0]![1];
}
const LUMBAR_TOP = [0.039, 0.078, 0.117, 0.157, 0.196];
type Vertebra = { name: string; lo: number; hi: number; rx: number; ry: number; spinousDrop: number; spinousLen: number; transverse: number };
/** 🧱 The seventeen vertebrae of the torso bone from L5 up to T1, the thoracic ones at the levels of their ribs, and C7 on top. */
function vertebrae(ribU: readonly number[]): Vertebra[] {
  const out: Vertebra[] = [];
  let lo = 0;
  LUMBAR_TOP.forEach((hi, i) => { out.push({ name: `l${5 - i}`, lo, hi, rx: 0.025 - 0.001 * i, ry: 0.017 - 0.0005 * i, spinousDrop: 0.006, spinousLen: 0.03, transverse: 0.04 }); lo = hi; });
  for (let k = 12; k >= 1; k--) {
    const hi = ribU[k - 1]! + 0.008;
    const t = (12 - k) / 11;
    out.push({ name: `t${k}`, lo, hi, rx: 0.02 - 0.006 * t, ry: 0.015 - 0.0055 * t, spinousDrop: k >= 9 ? 0.012 : k >= 5 ? 0.026 : 0.016, spinousLen: k >= 9 ? 0.026 : 0.024, transverse: 0.026 + 0.008 * t });
    lo = hi;
  }
  out.push({ name: "c7", lo, hi: 0.504, rx: 0.0135, ry: 0.009, spinousDrop: 0.004, spinousLen: 0.032, transverse: 0.03 });
  return out;
}

type Detail = { scale: 50 | 100 | 200 | 500; floor: number };
/** 🩻 The structural solids this pass states for one scale; `kept` are the solids of the earlier list that stay. */
function skeleton(d: Detail, kept: Json[], ribU: readonly number[]): Json[] {
  const out: Json[] = [];
  const fine = d.scale === 50;
  const mid = d.scale <= 100;
  const r = (x: number): number => Math.max(x, d.floor / 2);
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const both = (solid: Json, member: string): void => { m(solid, member); m(mirrored(solid), member.replace(/-l$/, "-r")); };

  /* vertebral column */
  const V = vertebrae(ribU);
  if (mid) {
    for (const x of V) {
      const [h, c] = [(x.hi - x.lo) * 0.39, (x.lo + x.hi) / 2];
      if (2 * h < d.floor) continue;
      m(tube(b(`vertebra-${x.name}`, "torso"), [[c - h, spineV(c - h), 0], [c, spineV(c), 0], [c + h, spineV(c + h), 0]], [[r(x.rx), r(x.ry)], [r(x.rx * 0.9), r(x.ry * 0.9)], [r(x.rx), r(x.ry)]]), "torso");
      if (fine) {
        const back = spineV(c) + x.ry + 0.008;
        m(capsule(b(`spinous-${x.name}`, "torso"), [c, back - 0.004, 0], [c - x.spinousDrop, back + x.spinousLen, 0], 0.0046, 0.0042), "torso");
        m(capsule(b(`transverse-${x.name}`, "torso"), [c, back - 0.003, -x.transverse], [c, back - 0.003, x.transverse], 0.0045, 0.0045), "torso");
      }
    }
    const levels = [0, ...V.map((x) => x.hi)];
    m(tube(b("intervertebral-discs", "torso"), levels.map((u) => [u, spineV(u), 0] as V3), levels.map((u, i) => { const x = V[Math.min(i, V.length - 1)]!; return [r(x.rx * 0.82), r(x.ry * 0.82)] as [number, number]; })), "torso");
    m(tube(b(fine ? "vertebral-arches" : "spinous-crest", "torso"), levels.map((u, i) => { const x = V[Math.min(i, V.length - 1)]!; return [u, spineV(u) + x.ry + (fine ? 0.005 : 0.012), 0] as V3; }), fine ? [0.0075] : [[0.0075, 0.014]]), "torso");
    const NECK: readonly [number, number][] = [[0, 0.02], [0.0175, 0.015], [0.035, 0.011], [0.0525, 0.008], [0.07, 0.006], [0.0875, 0.005], [0.107, 0.005]];
    if (fine) {
      for (let i = 0; i < 6; i++) {
        const [[u0, v0], [u1, v1]] = [NECK[i]!, NECK[i + 1]!];
        const [uc, vc, h] = [(u0 + u1) / 2, (v0 + v1) / 2, (u1 - u0) * 0.39];
        const wide = i === 5 ? 0.019 : i === 4 ? 0.0125 : 0.012;
        m(tube(b(`vertebra-c${6 - i}`, "neck"), [[uc - h, vc, 0], [uc, vc, 0], [uc + h, vc, 0]], [[wide, 0.008], [wide * 0.92, 0.0075], [wide, 0.008]]), "neck");
      }
      m(capsule(b("spinous-c2", "neck"), [0.079, 0.016, 0], [0.074, 0.036, 0], 0.0046, 0.0042), "neck");
      m(tube(b("cervical-discs", "neck"), NECK.map(([u, v]) => [u, v, 0] as V3), [[0.0095, 0.0065]]), "neck");
      m(tube(b("cervical-arches", "neck"), NECK.map(([u, v]) => [u, v + 0.013, 0] as V3), [[0.011, 0.0065]]), "neck");
    } else {
      m(tube(b("cervical-spine", "neck"), NECK.map(([u, v]) => [u, v + 0.004, 0] as V3), [[0.012, 0.0115]]), "neck");
    }
  } else {
    const levels = d.scale === 200 ? [-0.004, 0.04, 0.08, 0.132, 0.201, 0.28, 0.35, 0.421, 0.474, 0.504] : [-0.004, 0.08, 0.201, 0.35, 0.474, 0.504];
    m(tube(b("spine", "torso"), levels.map((u) => [u, spineV(Math.max(u, 0)), 0] as V3), levels.map((_, i) => [r(0.024 - 0.008 * (i / (levels.length - 1))), r(0.018 - 0.006 * (i / (levels.length - 1)))] as [number, number])), "torso");
    m(tube(b("cervical-spine", "neck"), [[-0.004, 0.022, 0], [0.05, 0.012, 0], [0.107, 0.008, 0]], [r(0.012)]), "neck");
  }

  /* rib cage of the coarse scales: the basket as one closed body, where single ribs are thinner than anything the scale shows */
  if (!mid) m(loft(b("ribcage", "torso"), dome(dome([st(0.2, 0.012, 0.1, 0.05, 2.2), st(0.26, -0.01, 0.15, 0.078, 2.2), st(0.34, -0.014, 0.151, 0.085, 2.2), st(0.42, -0.016, 0.105, 0.068, 2.2), st(0.474, -0.011, 0.062, 0.05, 2.2)], -1, 0.02), 1, 0.014)), "torso");
  if (d.scale === 100) m(loft(b("sternum", "torso"), [st(0.24, -0.091, 0.012, 0.0075), st(0.309, -0.099, 0.018, 0.0075), st(0.388, -0.089, 0.02, 0.0075), st(0.446, -0.07, 0.03, 0.0075), st(0.468, -0.062, 0.028, 0.0075)]), "torso");

  /* shoulder girdle */
  if (d.scale <= 200) {
    const rc = r(0.0068);
    both(tube(b("clavicle-l", "torso"), [[0.461, -0.056, 0.02], [0.466, -0.06, 0.065], [0.471, -0.04, 0.125], [0.47, -0.008, 0.17], [mid ? 0.468 : 0.46, 0.01, 0.19]], mid ? [[r(0.0075), r(0.008)], rc, rc, [r(0.0085), rc], [r(0.01), r(0.0055)]] : [rc]), "torso");
    both(loft(b("scapula-l", "torso"), dome(blade([[[0.3, 0.086, 0.09], [0.44, 0.082, 0.078]], [[0.292, 0.078, 0.108], [0.446, 0.062, 0.11]], [[0.34, 0.056, 0.134], [0.448, 0.04, 0.14]], [[0.4, 0.026, 0.158], [0.444, 0.022, 0.16]], [[0.412, 0.01, 0.17], [0.45, 0.008, 0.17]]], r(0.004) * 2), -1, 0.006)), "torso");
    if (mid) {
      both(tube(b("scapula-spine-l", "torso"), [[0.402, 0.092, 0.084], [0.428, 0.072, 0.13], [0.458, 0.04, 0.172], [0.468, 0.014, 0.196]], [r(0.006), r(0.0065), r(0.007), [r(0.0105), r(0.0055)]], [1, 0, 0]), "torso");
      both(ellipsoid(b("glenoid-l", "torso"), [r(0.019), r(0.013), r(0.0085)], [0.4316, 0.005, 0.17]), "torso");
    }
  } else {
    both(capsule(b("clavicle-l", "torso"), [0.455, -0.03, 0.02], [0.45, 0.004, 0.182], r(0.0075), r(0.0075)), "torso");
  }

  /* pelvis */
  if (d.scale <= 200) {
    const thick = r(0.005) * 2;
    both(loft(b("ilium-l", "pelvis"), dome(dome(blade([[[0.058, -0.05, 0.108], [0.088, -0.052, 0.12]], [[0.022, -0.03, 0.104], [0.106, -0.026, 0.136]], [[0.012, 0.004, 0.1], [0.116, 0.01, 0.142]], [[0.02, 0.036, 0.082], [0.112, 0.046, 0.12]], [[0.036, 0.058, 0.05], [0.1, 0.072, 0.078]], [[0.05, 0.074, 0.036], [0.08, 0.086, 0.046]]], thick), -1, 0.006), 1, 0.006)), "pelvis");
    both(ellipsoid(b("acetabulum-l", "pelvis"), [r(0.031), r(0.03), r(0.013)], [0.004, 0, 0.081]), "pelvis");
    both(tube(b("pubis-l", "pelvis"), [[-0.004, -0.02, 0.08], [-0.02, -0.046, 0.044], [-0.034, -0.058, 0.004]], [r(0.0105), r(0.009), r(0.0095)]), "pelvis");
    both(tube(b("ischium-l", "pelvis"), [[-0.018, 0.012, 0.082], [-0.05, 0.03, 0.072], [-0.08, 0.036, 0.062]], [r(0.013), r(0.012), r(0.015)]), "pelvis");
    if (mid) {
      both(tube(b("ischiopubic-ramus-l", "pelvis"), [[-0.084, 0.03, 0.058], [-0.074, -0.018, 0.032], [-0.052, -0.054, 0.008]], [r(0.008), r(0.0075), r(0.0075)]), "pelvis");
      m(ellipsoid(b("pubic-symphysis", "pelvis"), [r(0.02), r(0.01), r(0.008)], [-0.042, -0.057, 0]), "pelvis");
    }
    m(tube(b("sacrum", "pelvis"), [[0.09, 0.038, 0], [0.062, 0.058, 0], [0.032, 0.075, 0], [0.006, 0.084, 0], [-0.016, 0.084, 0], [-0.034, 0.077, 0]], [[0.052, r(0.019)], [0.046, r(0.017)], [0.036, r(0.014)], [0.024, r(0.011)], [r(0.012), r(0.008)], [r(0.007), r(0.006)]]), "pelvis");
  } else {
    m(loft(b("pelvis", "pelvis"), dome(dome([st(-0.07, 0.03, 0.075, 0.04, 2.3), st(0, 0.012, 0.118, 0.06, 2.3), st(0.06, 0.012, 0.14, 0.07, 2.3), st(0.105, 0.02, 0.138, 0.062, 2.3)], -1, 0.02), 1, 0.012)), "pelvis");
  }

  /* skull */
  const braincase = [st(0.014, 0.018, 0.053, 0.056), st(0.035, 0.002, 0.066, 0.082), st(0.065, -0.002, 0.07, 0.091), st(0.095, 0, 0.069, 0.087), st(0.125, 0.004, 0.059, 0.072), st(0.143, 0.006, 0.039, 0.048)];
  if (mid) {
    m(loft(b("cranium", "head"), dome(dome(braincase, -1, 0.012), 1, 0.008)), "head");
    m(loft(b("face-bones", "head"), dome([st(-0.042, -0.056, 0.03, 0.028, 2.3), st(-0.02, -0.055, 0.036, 0.03, 2.3), st(0.005, -0.05, 0.056, 0.034, 2.3), st(0.032, -0.05, 0.052, 0.034, 2.3)], -1, 0.006)), "head");
    both(capsule(b("mandible-ramus-l", "head"), [0.014, -0.008, 0.052], [-0.046, -0.01, 0.044], r(0.0075), r(0.0085)), "head");
    m(tube(b("mandible-body", "head"), [[-0.046, -0.01, 0.044], [-0.058, -0.042, 0.033], [-0.066, -0.066, 0.018], [-0.069, -0.074, 0], [-0.066, -0.066, -0.018], [-0.058, -0.042, -0.033], [-0.046, -0.01, -0.044]], [[r(0.006), r(0.011)], [r(0.006), r(0.0105)], [r(0.0065), r(0.011)], [r(0.0075), r(0.013)], [r(0.0065), r(0.011)], [r(0.006), r(0.0105)], [r(0.006), r(0.011)]], [1, 0, 0]), "head");
    if (fine) both(capsule(b("zygomatic-arch-l", "head"), [0.008, -0.07, 0.05], [0.012, -0.004, 0.064], 0.0055, 0.0048), "head");
  } else {
    m(loft(b("skull", "head"), dome(dome([st(-0.07, -0.05, 0.03, 0.028), st(-0.04, -0.035, 0.045, 0.05), st(0, -0.012, 0.06, 0.075), ...braincase.slice(1)], -1, 0.008), 1, 0.008)), "head");
  }

  /* forefoot of the fine scale: five metatarsals fanning from the tarsus to their heads, five toes on the toe bone; the first ray lies medially; a metatarsal head is carried into the frame of the toe bone, which rests 32.48 degrees off the foot bone */
  if (fine) {
    both(ellipsoid(b("tarsus-l", "foot-l"), [0.022, 0.015, 0.027], [0.05, -0.004, 0]), "foot-l");
    const rays: readonly [number, number, number, number][] = [[-0.03, 0.16, 0.058, 0.0072], [-0.014, 0.164, 0.05, 0.0055], [0.002, 0.16, 0.044, 0.0052], [0.017, 0.152, 0.038, 0.005], [0.031, 0.142, 0.03, 0.0048]];
    rays.forEach(([w, head, tip, rr], i) => {
      both(capsule(b(`metatarsal-${i + 1}-l`, "foot-l"), [0.062, -0.002, w * 0.6], [head, 0.034, w], rr, rr + 0.001), "foot-l");
      both(capsule(b(`toe-phalanges-${i + 1}-l`, "toe-l"), [(head - 0.17693) * 0.8436 + 0.034 * 0.537 - 0.002, (0.17693 - head) * 0.537 + 0.034 * 0.8436, w], [tip, 0.03, w * 1.02], rr + 0.0005, Math.max(rr - 0.0008, 0.004)), "toe-l");
    });
  }

  /* long bones of the coarse scales, no thinner than the smallest feature */
  if (!mid) {
    const [TL, SL, AL, FL] = [boneLen("thigh-l"), boneLen("shank-l"), boneLen("arm-l"), boneLen("forearm-l")];
    both(tube(b("femur-l", "thigh-l"), [[0, 0, 0], [0.04, -0.004, 0.056], [0.23, 0.012, 0.028], [TL + 0.004, 0.002, 0.002]], [r(0.02), r(0.017), r(0.015), r(0.024)]), "thigh-l");
    if (d.scale === 200) both(ellipsoid(b("femur-condyles-l", "thigh-l"), [0.022, 0.032, 0.042], [TL - 0.008, -0.004, 0]), "thigh-l");
    both(tube(b("tibia-l", "shank-l"), [[-0.004, 0, 0], [0.06, 0.008, -0.004], [0.29, 0.008, -0.004], [SL + 0.004, 0.002, -0.003]], [[r(0.036), r(0.026)], r(0.017), r(0.013), r(0.016)]), "shank-l");
    both(tube(b("humerus-l", "arm-l"), [[0, 0, -0.012], [0.12, 0.004, 0.007], [0.24, 0.003, 0.021], [AL + 0.004, 0, 0.029]], [r(0.022), r(0.0115), r(0.011), r(0.016)]), "arm-l");
    both(tube(b("radius-ulna-l", "forearm-l"), [[-0.01, -0.004, 0], [0.13, 0, 0.012], [FL + 0.004, 0, 0.021]], [[r(0.012), r(0.02)], [r(0.011), r(0.016)], [r(0.011), r(0.02)]]), "forearm-l");
  }

  /* what stays of the earlier list, with every radius at least half the smallest feature */
  const floored = (x: Json): Json => {
    const y = JSON.parse(JSON.stringify(x)) as Json;
    if (y.kind === "capsule") { y.r0 = r(y.r0); y.r1 = r(y.r1); }
    if (y.kind === "ellipsoid") y.radii = y.radii.map(r);
    if (y.kind === "loft") for (const s of y.sections) { s.rx = r(s.rx); s.ry = r(s.ry); }
    return y;
  };
  return [...out, ...kept.map(floored)];
}

const REPLACED = /^(spine|sternum|clavicle|scapula|ilium|pubis|ischium|mandible|sacrum|cranium|face-bones|cervical-spine|skull|shoulder-girdle|pelvic-ring|pelvic-girdle)(-|$)/;
const COARSE_LIMBS = /^(costal|femur|patella|tibia|malleolus|fibula|humerus|ulna|radius)(-|$)/;
/** 🩼 The two telescoping aluminium tubes of a forearm crutch as turned parts: 22 mm outer tube from the handle joint down to the clip, 19 mm inner tube down into the rubber tip. */
function crutchTubes(old: Json, side: 1 | -1): Json {
  const axis: V3 = [0.99631, 0, 0.08579 * side];
  const common = { id: old.id, bone: old.bone, material: old.material, tone: old.tone, role: old.role };
  const extra = old.slender ? { slender: true } : {};
  return old.id.startsWith("crutch-tube") ? revolve(common, [[-0.013, 0], [-0.013, 0.011], [0.467, 0.011], [0.467, 0]], [0, 0, 0], axis, extra) : revolve(common, [[0.44, 0], [0.44, 0.0095], [0.854, 0.0095], [0.854, 0]], [0, 0, 0], axis, extra);
}

const source0 = JSON.parse(readFileSync(SOURCE0, "utf8")) as { volumes: Record<string, Json> };
const ribU = Array.from({ length: 12 }, (_, i) => (source0.volumes["50"]!.structure as Json[]).find((x) => x.id === `costal-${i + 1}-l`).sections[0].at[0] as number);
for (const [scale, floor] of [[50, 0.0075], [100, 0.015], [200, 0.03], [500, 0.075]] as const) {
  const v = source0.volumes[String(scale)]!;
  const kept = (v.structure as Json[]).filter((x) => !REPLACED.test(x.id) && (scale <= 100 || !COARSE_LIMBS.test(x.id)) && (scale !== 50 || !/^(metatarsals|phalanges)-[lr]$/.test(x.id)));
  for (const x of kept) {
    if (/^patella-/.test(x.id)) x.transform.translation[1] = 0.036;
    if (/^fibula-/.test(x.id)) { x.p0 = [0.03, -0.018, x.p0[2] > 0 ? 0.03 : -0.03]; x.r0 = 0.009; }
  }
  v.structure = skeleton({ scale, floor }, kept, ribU);
  v.body = (v.body as Json[]).map((x) => {
    if (x.kind === "polyhedron" && /^crutch-(tube|lower)-[lr]$/.test(x.id)) return crutchTubes(x, x.id.endsWith("-l") ? 1 : -1);
    if (x.id === "lace-r2") return { ...x, p0: [x.p0[0], x.p0[1] - 0.0046, x.p0[2]], p1: [x.p1[0], x.p1[1] - 0.0046, x.p1[2]] };
    if (x.id === "lace-r3") return { ...x, p0: [x.p0[0], x.p0[1] - 0.006, x.p0[2]], p1: [x.p1[0], x.p1[1] - 0.006, x.p1[2]] };
    return x;
  });
  if (scale >= 100) {
    const gone = scale >= 200 ? /^crutch-(detent|clip|bracket)-[lr]$/ : /^crutch-detent-[lr]$/;
    v.body = (v.body as Json[]).filter((x) => !gone.test(x.id));
    v.thermal = (v.thermal as Json[]).filter((x) => !(x.of && gone.test(x.of)));
    v.structure = (v.structure as Json[]).filter((x) => !(x.of && gone.test(x.of)));
  }
  const say = (index: number, en: string, de: string): void => { (v.decisions as Json[])[index] = { en, de }; };
  if (scale === 50) {
    say(2, "Each forearm crutch is built as the real product: cuff arm and shaft tube of 22 mm aluminium as turned parts, bracket, the telescopic 19 mm lower tube with its clamp collar and spring button, rubber handle, ferrule and an open polymer cuff below the elbow, rigid in the gripping hand; the 5 mm adjustment holes are below the 7.5 mm feature and left out.", "Jede Unterarmgehstütze ist wie das reale Produkt gebaut: Manschettenarm und Schaftrohr aus 22-mm-Aluminium als Drehteile, Halter, das ausziehbare 19-mm-Unterrohr mit Klemmring und Federknopf, Gummigriff, Gummipuffer und eine offene Kunststoffmanschette unter dem Ellbogen, starr in der greifenden Hand; die 5-mm-Verstelllöcher liegen unter der kleinsten Einzelheit von 7,5 mm und entfallen.");
    say(4, "Structure: the anatomical skeleton. Skull of braincase, facial skeleton, zygomatic arches and a mandible of two rami and a curved body; seven cervical, twelve thoracic and five lumbar vertebral bodies on their discs along lordosis and kyphosis, each with arch, spinous and transverse processes, the thoracic ones at the levels of their ribs; twelve pairs of ribs with a costal margin and two floating pairs, the sternum; S-curved clavicles from the sternum to the acromion, scapulae as blades on the ribs with spine, acromion and glenoid at the head of the humerus; a pelvis of two iliac blades from the anterior to the posterior superior spine, acetabula, pubic and ischial rami around the obturator openings, symphysis, sacrum and coccyx; long bones with their heads and condyles, carpals, finger and foot bones; both crutch frames as members, joints at the rig joints, masses at the segment centres after de Leva.", "Tragwerk: das anatomische Skelett. Schädel aus Hirnschädel, Gesichtsschädel, Jochbögen und einem Unterkiefer aus zwei Ästen und gebogenem Körper; sieben Hals-, zwölf Brust- und fünf Lendenwirbelkörper auf ihren Bandscheiben entlang von Lordose und Kyphose, jeder mit Bogen, Dorn- und Querfortsätzen, die Brustwirbel auf der Höhe ihrer Rippen; zwölf Rippenpaare mit Rippenbogen und zwei freien Paaren, das Brustbein; S-förmige Schlüsselbeine vom Brustbein zum Schulterdach, Schulterblätter als Platten auf den Rippen mit Gräte, Schulterdach und Gelenkpfanne am Oberarmkopf; ein Becken aus zwei Darmbeinschaufeln vom vorderen zum hinteren oberen Darmbeinstachel, Hüftpfannen, Scham- und Sitzbeinästen um die Hüftlöcher, Schambeinfuge, Kreuz- und Steißbein; Röhrenknochen mit Köpfen und Gelenkrollen, Handwurzel-, Finger- und Fußknochen; beide Krückengestelle als Stäbe, Gelenke an den Gelenken des Skeletts, Massen in den Segmentschwerpunkten nach de Leva.");
  }
  if (scale === 100) {
    say(1, "The boot keeps shell, sole and three straps, the sneaker sole and upper; each crutch is the 22 mm shaft tube, the 19 mm lower tube with its clamp collar, bracket, handle, ferrule and cuff; the spring button is below the feature and left out.", "Die Orthese behält Schale, Sohle und drei Riemen, der Turnschuh Sohle und Schaft; jede Krücke besteht aus dem 22-mm-Schaftrohr, dem 19-mm-Unterrohr mit Klemmring, Halter, Griff, Puffer und Manschette; der Federknopf liegt unter der kleinsten Einzelheit und entfällt.");
    say(2, "Thermal: every body solid in its region and outer layer, face and neck as open skins of the visible part. Structure: braincase, facial skeleton and mandible; the lumbar and thoracic vertebral bodies on one disc column with the spinous processes merged into one crest, the neck as one column; six paired rib courses and a flat sternum; curved clavicles, scapula blades with spine and glenoid; iliac blades, acetabula, pubic and ischial rami, symphysis and sacrum; long bones, simplified hands and feet, crutch frames, joints and segment masses.", "Thermik: jeder Körper in seiner Region und äußeren Lage, Gesicht und Hals als offene Häute des sichtbaren Teils. Tragwerk: Hirnschädel, Gesichtsschädel und Unterkiefer; die Lenden- und Brustwirbelkörper auf einer Bandscheibensäule, die Dornfortsätze zu einem Kamm zusammengefasst, der Hals als eine Säule; sechs beidseitige Rippenzüge und ein flaches Brustbein; gebogene Schlüsselbeine, Schulterblattplatten mit Gräte und Gelenkpfanne; Darmbeinschaufeln, Hüftpfannen, Scham- und Sitzbeinäste, Schambeinfuge und Kreuzbein; Röhrenknochen, vereinfachte Hände und Füße, Krückengestelle, Gelenke und Segmentmassen.");
    say(3, "Blades and thin bones are drawn 15 mm thick so that they stay readable; fibula, radius and ulna keep their place beside their neighbours at that thickness. The joints turn in the same skeleton at every display scale.", "Platten und dünne Knochen sind 15 mm stark gezeichnet, damit sie lesbar bleiben; Wadenbein, Speiche und Elle behalten in dieser Stärke ihren Platz neben ihren Nachbarn. Die Gelenke drehen in allen Darstellungsmaßstäben im selben Skelett.");
  }
  if (scale === 200) {
    say(1, "Thermal follows the body solid by solid. Structure: one skull, the spine as one curved column, the rib cage as one closed basket, clavicles and scapula blades, a pelvis of iliac blades, acetabula, pubic and ischial bars and sacrum, one bone each for thigh, shank, upper arm and forearm, hand and foot bones as one member each, the crutch rods, joints and the four main masses.", "Die Thermik folgt dem Körper Stück für Stück. Tragwerk: ein Schädel, die Wirbelsäule als eine gekrümmte Säule, der Brustkorb als ein geschlossener Korb, Schlüsselbeine und Schulterblattplatten, ein Becken aus Darmbeinschaufeln, Hüftpfannen, Scham- und Sitzbeinstäben und Kreuzbein, je ein Knochen für Oberschenkel, Unterschenkel, Oberarm und Unterarm, Hand- und Fußknochen als je ein Stab, die Krückenstangen, Gelenke und die vier Hauptmassen.");
    say(2, "No bone is thinner than the 30 mm feature: radius and ulna are one forearm bone, the fibula lies in the shank bone, single ribs in the basket. The joints turn in the same skeleton at every display scale.", "Kein Knochen ist dünner als die kleinste Einzelheit von 30 mm: Speiche und Elle sind ein Unterarmknochen, das Wadenbein liegt im Unterschenkelknochen, die einzelnen Rippen im Korb. Die Gelenke drehen in allen Darstellungsmaßstäben im selben Skelett.");
  }
  if (scale === 500) {
    say(1, "Thermal keeps every region: grips and ferrules follow the visible rubber components. Structure: skull, the spine as one column, the rib cage as one basket, two clavicle bars to the shoulder joints, the pelvis as one basin, one bone each for thigh, shank, upper arm and forearm, one member each for hand and foot bones, the crutch rods, the joints of the limbs and the trunk masses.", "Die Thermik behält jede Region: Griffe und Puffer folgen den sichtbaren Gummibauteilen. Tragwerk: Schädel, die Wirbelsäule als eine Säule, der Brustkorb als ein Korb, zwei Schlüsselbeinstäbe zu den Schultergelenken, das Becken als eine Schale, je ein Knochen für Oberschenkel, Unterschenkel, Oberarm und Unterarm, je ein Stab für Hand- und Fußknochen, die Krückenstangen, die Gelenke der Glieder und die Rumpfmassen.");
    say(2, "No bone is thinner than the 75 mm feature, so the skeleton reads as a jointed mannequin whose bars lie on the true joint centres. The joints turn in the same skeleton at every display scale.", "Kein Knochen ist dünner als die kleinste Einzelheit von 75 mm, das Skelett liest sich daher als Gliederpuppe, deren Stäbe auf den wahren Gelenkmitten liegen. Die Gelenke drehen in allen Darstellungsmaßstäben im selben Skelett.");
  }
  console.log(scale, ["body", "thermal", "structure"].map((l) => `${l} ${(v[l] as Json[]).length}`).join(", "));
}
writeFileSync(SOURCE, `${JSON.stringify(source0)}\n`);
