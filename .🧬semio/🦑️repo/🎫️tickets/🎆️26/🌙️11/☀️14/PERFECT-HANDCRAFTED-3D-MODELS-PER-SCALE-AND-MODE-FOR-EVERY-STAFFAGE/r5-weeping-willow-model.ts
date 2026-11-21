/** 🌳️ Round-5 weeping willow, grown from the round-3 script (*Salix × sepulcralis* 'Chrysocoma'): every stem section, limb, branch, hanging tress, crown lobe and outlet is stated by hand per scale in world metres (x east, y up, z lateral) and written as bone-local solids to `volumes/vegetation/weeping-willow.json`; the root fixes (drooping outer limbs, curtain lengths, emitter anchors, 2D drawings as true views of each scale's body) go to the candidate document `🗑️generated/r3-weeping-willow/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-weeping-willow.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, SOURCES, entryOf, frames, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, along3, contains, dirOf, mul, r4, r5, radial, secPoint, solidOf, sub, surfacePoint, tangential, unit, type Lobe, type Loft, type Part, type Sec } from "./r3-weeping-willow-kit.ts";
import { draw } from "./r3-weeping-willow-draw.ts";

const { entry } = entryOf("vegetation/weeping-willow");
const document: Json = JSON.parse(readFileSync(entry.path, "utf8"));
const asset: Json = document.asset;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

const RIG: Record<string, { angleDeg: number; lengthM?: number }> = {
  "limb-w-c": { angleDeg: 71 },
  "limb-e-c": { angleDeg: -71 },
  "curt-w3": { angleDeg: 60, lengthM: 8.85 },
  "curt-e3": { angleDeg: -60, lengthM: 8.85 },
  "curt-w4": { angleDeg: 60, lengthM: 8.1 },
  "curt-e4": { angleDeg: -60, lengthM: 8.1 },
};
for (const [id, fix] of Object.entries(RIG)) {
  const b = bone(id);
  b.angleDeg = fix.angleDeg;
  if (fix.lengthM !== undefined) {
    b.lengthM = fix.lengthM;
    b.mass.comX = r4(fix.lengthM / 2);
  }
}
for (const pose of asset.poses) for (const load of pose.loads ?? []) if (RIG[load.bone]?.lengthM !== undefined) load.x = r4(RIG[load.bone]!.lengthM! / 2);

/** 🌬️ Wind poses and clip keys: the stiff stem and limbs give little (stem x 0.5, limbs and caps x 0.25 of the round-2 angles), and every curtain gets the absolute swing to the lee that its pose describes (its round-2 angle), so the pendulums are no longer turned down by the limbs they hang from. Applied once (the round-2 storm angle of `limb-w-a` is -4 degrees). */
const ancestors = (id: string): string[] => {
  const out: string[] = [];
  for (let b = bone(id).parent; b !== null; b = bone(b).parent) out.push(b);
  return out;
};
function windFix(bones: Record<string, { angleDeg: number }>): void {
  const swing: Record<string, number> = {};
  for (const [id, v] of Object.entries(bones)) {
    if (id.startsWith("curt-")) swing[id] = v.angleDeg;
    else if (id.startsWith("trunk-")) v.angleDeg = r4(v.angleDeg * 0.5);
    else v.angleDeg = r4(v.angleDeg * 0.25);
  }
  for (const [id, a] of Object.entries(swing)) bones[id]!.angleDeg = r4(a - ancestors(id).reduce((t, b) => t + (bones[b]?.angleDeg ?? 0), 0));
}
const storm = asset.poses.find((p: Json) => p.id === "storm");
if (storm.bones["limb-w-a"].angleDeg === -4) {
  for (const pose of asset.poses) if (pose.id !== "calm") windFix(pose.bones);
  for (const clip of asset.clips) for (const key of clip.keys) if (key.bones && Object.keys(key.bones).length > 0) windFix(key.bones);
  storm.description.en = storm.description.en.replace("and the limbs approach their joint limits.", "and the stiff limbs give by 1 to 2 degrees at each joint.");
  storm.description.de = storm.description.de.replace("die Äste nähern sich ihren Gelenkgrenzen.", "die steifen Äste geben an jedem Gelenk um 1 bis 2 Grad nach.");
}
const F: Record<string, Frame> = frames(asset);

const BARK = "willow-bark", LEAF = "willow-leaf", WOOD = "willow-wood";
const CX = -0.091, A = 8.106, B = 7.0, TOP = 11.1;
/** 🍀️ The plan of the crown is lobed (four broad and eight small lobes, swelling unevenly between the axes, where every scale has a tress on each axis); the lobing is normalised so that the crown still spans exactly the declared width (x) and depth (z). */
const lobing = (az: number): number => 1 + 0.04 * Math.cos(4 * az * D2R) + 0.02 * Math.cos(8 * az * D2R) + 0.12 * Math.sin(2 * az * D2R) ** 2 * Math.sin(3 * az * D2R + 0.7);
const R0 = (az: number, a: number, b: number): number => lobing(az) / Math.hypot(Math.cos(az * D2R) / a, Math.sin(az * D2R) / b);
const [KX, KZ] = (() => {
  let [kx, kz] = [1, 1];
  for (let it = 0; it < 6; it++) {
    let [mx, mz] = [0, 0];
    for (let az = 0; az < 360; az += 0.25) { const r = R0(az, A / kx, B / kz); mx = Math.max(mx, Math.abs(r * Math.cos(az * D2R))); mz = Math.max(mz, Math.abs(r * Math.sin(az * D2R))); }
    kx *= mx / A;
    kz *= mz / B;
  }
  return [kx, kz];
})();
const R = (az: number): number => R0(az, A / KX, B / KZ);
const side = (x: number): "w" | "e" => (x < CX ? "w" : "e");
const curtainOf = (p: V3): string => {
  const d = Math.abs(p[0] - CX);
  return `curt-${side(p[0])}${d < 2.4 ? 1 : d < 4.6 ? 2 : d < 6.2 ? 3 : 4}`;
};

/** 🪵️ A round or oval tube through stated centre points: rows [point, radius across, radius second axis?]; the across axis is world z for limbs, x for the stem. */
const tube = (id: string, b: string, material: string, tone: number, group: string, rows: [V3, number, number?][], across: V3 = [0, 0, 1], flags?: Json): Loft => ({
  kind: "loft", id, bone: b, material, tone, group, ...(flags ? { flags } : {}),
  secs: rows.map(([p, rx, ry]) => ({ p, rx, ry: ry ?? rx, x: across })),
});

/** 🌲️ Stem rows [height y, radius along x, radius along z] on the stem axis, which leans 4 degrees west above 2.2 m. */
const stemAt = (y: number): V3 => [-0.0699 * Math.max(0, y - 2.2), y, 0];
const stem = (id: string, b: string, rows: [number, number, number][], tone = 0.28): Loft => tube(id, b, BARK, tone, "stem", rows.map(([y, rx, rz]) => [stemAt(y), rx, rz]), [1, 0, 0]);

/** 🦴️ Limb rows [u along the bone, bow away from the limb's upper side (negative = outwards/down), radius] in the rest frame of the bone. */
function limb(b: string, rows: [number, number, number][], id = `wood-${b}`, material = BARK, tone = 0.3): Loft {
  const f = F[b]!;
  const up: V3 = f.v[1] > 0 ? f.v : mul(f.v, -1);
  return tube(id, b, material, tone, "limb", rows.map(([u, bow, r]) => [add(add(f.o, mul(f.u, u)), mul(up, bow)), r]));
}

/** 🌿️ A secondary branch: from `start` towards azimuth `az` (degrees from +x towards +z), elevation turning from `e0` to `e1` over the length `L`, radius `r0` to `r1`, in `n` steps. */
function branch(id: string, b: string, start: V3, az: number, e0: number, e1: number, L: number, r0: number, r1: number, n: number): Loft {
  const rows: [V3, number][] = [[add(start, mul(dirOf(az, e0), -0.06)), r0 * 1.05]];
  let p = start;
  rows.push([p, r0]);
  for (let i = 1; i <= n; i++) {
    const s = (i - 0.5) / n;
    p = add(p, mul(dirOf(az, e0 + (e1 - e0) * s), L / n));
    rows.push([p, r0 + (r1 - r0) * (i / n)]);
  }
  return tube(id, b, BARK, 0.3, "branch", rows);
}

type Scale = 50 | 100 | 200 | 500;
type Model = { scale: Scale; parts: Part[]; decisions: { en: string; de: string }[] };
const SPURS = (scale: Scale): Loft[] => {
  const azs = scale === 50 ? [20, 95, 163, 236, 307] : [35, 160, 285];
  return azs.map((az, i) => {
    const rad = radial(az);
    const rows: [number, number, number][] = scale === 50 ? [[0.28, 0.33, 0.2], [0.55, 0.22, 0.17], [0.85, 0.12, 0.105], [1.15, 0.07, 0.06], [1.32, 0.05, 0.04]] : [[0.3, 0.33, 0.2], [0.7, 0.17, 0.14], [1.1, 0.08, 0.065], [1.3, 0.06, 0.05]];
    return tube(`root-spur-${i + 1}`, "root-plate", BARK, 0.25, "root", rows.map(([r, y, rr]) => [[r * rad[0], y, r * rad[2]], rr, rr * 0.75] as [V3, number, number]), [0, 1, 0]);
  });
};

function wood(scale: Scale): Loft[] {
  const fine = scale === 50;
  const out: Loft[] = [];
  out.push(stem("root-flare", "root-plate", scale === 500 ? [[0, 0.72, 0.72], [0.45, 0.47, 0.45]] : fine ? [[0, 0.8, 0.8], [0.08, 0.66, 0.66], [0.18, 0.56, 0.55], [0.3, 0.5, 0.49], [0.42, 0.48, 0.465]] : [[0, 0.78, 0.78], [0.14, 0.6, 0.6], [0.3, 0.5, 0.49], [0.44, 0.48, 0.465]], 0.25));
  if (scale !== 200 && scale !== 500) out.push(...SPURS(scale));
  out.push(stem("trunk-lower", "trunk-1", fine ? [[0.22, 0.5, 0.49], [0.6, 0.45, 0.44], [1.0, 0.43, 0.42], [1.4, 0.425, 0.41], [1.8, 0.425, 0.405], [2.28, 0.42, 0.4]] : scale === 500 ? [[0.3, 0.47, 0.46], [2.28, 0.42, 0.4]] : [[0.22, 0.5, 0.49], [0.8, 0.44, 0.43], [1.6, 0.425, 0.41], [2.28, 0.42, 0.4]]));
  out.push(stem("trunk-upper", "trunk-2", fine ? [[2.15, 0.42, 0.4], [2.6, 0.4, 0.385], [3.0, 0.42, 0.38], [3.35, 0.47, 0.36], [3.5, 0.43, 0.32]] : scale === 500 ? [[2.15, 0.42, 0.4], [3.52, 0.44, 0.33]] : [[2.15, 0.42, 0.4], [2.8, 0.41, 0.38], [3.35, 0.47, 0.36], [3.5, 0.43, 0.32]]));
  for (const s of ["w", "e"] as const) {
    if (fine) {
      out.push({ ...limb(`limb-${s}-a`, [[-0.5, -0.25, 0.27], [-0.2, -0.16, 0.25], [0.2, -0.08, 0.215], [0.6, -0.02, 0.2], [0.85, 0.01, 0.193]], `fork-${s}`, BARK, 0.29), group: "stem" });
      out.push(limb(`limb-${s}-a`, [[0.62, -0.02, 0.198], [1.0, 0.04, 0.19], [2.2, 0.08, 0.175], [3.4, 0.07, 0.158], [4.5, 0, 0.145], [4.62, -0.01, 0.14]]));
      out.push(limb(`limb-${s}-b`, [[-0.15, 0, 0.15], [0.5, 0.06, 0.142], [1.4, 0.12, 0.13], [2.3, 0.1, 0.118], [3.2, 0, 0.104], [3.3, -0.01, 0.102]]));
      out.push(limb(`limb-${s}-c`, [[-0.12, 0, 0.104], [0.8, 0.08, 0.092], [1.6, 0.1, 0.08], [2.4, 0.06, 0.066], [3.0, 0, 0.055], [3.25, -0.05, 0.042]]));
    } else if (scale === 500) {
      out.push(limb(`limb-${s}-a`, [[-0.4, -0.2, 0.26], [4.6, 0, 0.145]]));
      out.push(limb(`limb-${s}-b`, [[-0.12, 0, 0.15], [3.28, 0, 0.105]]));
      out.push(limb(`limb-${s}-c`, [[-0.12, 0, 0.105], [3.1, -0.02, 0.05]]));
    } else {
      out.push({ ...limb(`limb-${s}-a`, [[-0.5, -0.25, 0.27], [0.2, -0.08, 0.215], [0.85, 0.01, 0.193]], `fork-${s}`, BARK, 0.29), group: "stem" });
      out.push(limb(`limb-${s}-a`, [[0.62, -0.02, 0.198], [2.2, 0.08, 0.175], [4.6, 0, 0.143]]));
      out.push(limb(`limb-${s}-b`, [[-0.15, 0, 0.15], [1.4, 0.12, 0.13], [3.28, 0, 0.103]]));
      out.push(limb(`limb-${s}-c`, [[-0.12, 0, 0.104], [1.6, 0.1, 0.08], [3.2, -0.03, 0.05]]));
    }
  }
  if (scale !== 500) {
    const n = scale === 50 ? 7 : scale === 100 ? 4 : 3;
    const sets = scale === 200 ? [["w", [105, 255]], ["e", [75, 285]]] as const : [["w", [105, 148, 212, 255]], ["e", [32, 75, 285, 328]]] as const;
    for (const [s, azs] of sets) {
      const f = F[`limb-${s}-b`]!;
      const start = add(f.o, mul(f.u, 0.4));
      azs.forEach((az, i) => out.push(branch(`branch-${s}${i + 1}`, `limb-${s}-b`, i % 3 === 0 ? start : add(start, mul(f.u, 0.9)), az, i % 3 === 0 ? 30 : 38, -30, i % 3 === 0 ? 5.6 : 4.4, i % 3 === 0 ? 0.09 : 0.075, 0.022, n)));
    }
  }
  return out;
}

/** 🍂️ The section profile of the hanging part of a tress from the widest point of the crown (s = 0, y = Y0) to its tip (s = 1): [s, width factor, depth factor, outer radius factor]. */
type Profile = [number, number, number, number][];
const HANG_ROWS: Profile = [[0, 1, 1, 1], [0.35, 1, 0.95, 1], [0.65, 0.93, 0.86, 0.995], [0.85, 0.82, 0.72, 0.985], [0.95, 0.62, 0.5, 0.975], [1, 0.3, 0.22, 0.97]];
const HANG: Record<number, Profile> = { 50: HANG_ROWS, 100: HANG_ROWS.filter((_, i) => i !== 1), 200: HANG_ROWS.filter((_, i) => i !== 1 && i !== 3), 500: HANG_ROWS.filter((_, i) => i !== 1 && i !== 2 && i !== 3) };
const INNER_HANG: Profile = [[0, 0.55, 0.5, 1], [0.08, 0.95, 0.95, 1], [0.45, 1, 0.95, 1], [0.75, 0.86, 0.76, 1], [0.92, 0.66, 0.55, 1], [1, 0.32, 0.3, 1]];

/** ⛲️ The crown envelope: widest (the crown radius) at and below Y0, a dome above it to the top: radius fraction rho = sqrt(1 - t^P), t = (y - Y0) / (TOP - Y0). */
const Y0 = 4.5;
const P = (az: number): number => 2.6 + 0.45 * Math.cos(2 * az * D2R);
/** ⛰️ Mounds of foliage over the arching limbs: [x, z] in fractions of the crown radius, amplitude, spread; the two co-dominant limbs lift the crown east and west, smaller mounds over the four secondary branches, a shallow saddle over the fork. */
const MOUNDS: [number, number, number, number][] = [[-0.42, 0.04, 0.16, 0.3], [0.44, -0.06, 0.13, 0.28], [-0.08, 0.5, 0.07, 0.22], [0.12, -0.52, 0.09, 0.22], [-0.5, -0.45, 0.05, 0.2], [0.5, 0.42, 0.06, 0.2]];
const shapeRaw = (rho: number, az: number): number => {
  const base = Math.max(0, 1 - rho * rho) ** (1 / P(az));
  const [u, v] = [rho * Math.cos(az * D2R), rho * Math.sin(az * D2R)];
  return 0.93 * base + 0.55 * Math.sqrt(base) * MOUNDS.reduce((t, [mx, mz, a, sp]) => t + a * Math.exp(-((u - mx) ** 2 + (v - mz) ** 2) / (sp * sp)), 0);
};
const SHAPE_MAX = (() => { let m = 0; for (let r = 0; r <= 1; r += 0.01) for (let a = 0; a < 360; a += 3) m = Math.max(m, shapeRaw(r, a)); return m; })();
const yEnv = (rho: number, az: number): number => Y0 + ((TOP - Y0) * shapeRaw(rho, az)) / SHAPE_MAX;
const yAt = (rho: number, az: number): number => yEnv(rho, az);

/** 🎲️ Stated irregularity of the tresses, taken in turn: how far a tress lies inside the envelope (m) and its width factor. */
const SET_IN = [0, 0.26, 0.12, 0, 0.22, 0.3, 0, 0.18, 0.28, 0, 0.24, 0.1];
const HEM = [0.35, 0.75, 1.15, 1.75, 2.25, 1.6, 0.9, 0.55, 1.0, 0.7, 0.4, 0.62, 1.05, 0.5, 0.85, 1.3, 2.4, 2.75, 2.45, 1.4, 0.65, 1.1, 0.45, 0.8];
/** 〰️ A stated table over the azimuth (equal steps from 0 degrees), read with a smooth step between its values, so every scale samples the same ragged hem and the same set-in lobes. */
const around = (table: number[], az: number): number => {
  const x = ((((az % 360) + 360) % 360) / 360) * table.length;
  const i = Math.floor(x), f = x - i, e = f * f * (3 - 2 * f);
  return table[i % table.length]! * (1 - e) + table[(i + 1) % table.length]! * e;
};
const WIDTH = [1.0, 0.9, 1.12, 0.95, 1.06, 0.88, 1.1, 1.0, 0.92, 1.08, 0.97, 1.14, 0.9, 1.04, 0.94, 1.1, 0.98];

/** 🌊️ The meridian of the envelope at azimuth `az` from the radius fraction `rho0` out to the widest point at Y0, as points (radius in metres, height) spaced about `step` metres along it. */
function meridian(az: number, rho0: number, step: number): [number, number][] {
  const fine: [number, number][] = [];
  for (let i = 0; i <= 600; i++) {
    const s = i / 600, rho = rho0 + (1 - rho0) * (1 - (1 - s) ** 2.5);
    fine.push([R(az) * rho, yEnv(Math.min(1, rho), az)]);
  }
  const lens = [0];
  for (let i = 1; i < fine.length; i++) lens.push(lens[i - 1]! + Math.hypot(fine[i]![0] - fine[i - 1]![0], fine[i]![1] - fine[i - 1]![1]));
  const total = lens[lens.length - 1]!;
  const n = Math.max(2, Math.round(total / step));
  const out: [number, number][] = [];
  for (let k = 0; k <= n; k++) {
    const target = (total * k) / n;
    const found = lens.findIndex((l) => l >= target - 1e-9);
    const i = found < 0 ? fine.length - 1 : Math.max(1, found);
    const f = (target - lens[i - 1]!) / (lens[i]! - lens[i - 1]! || 1);
    out.push([fine[i - 1]![0] + (fine[i]![0] - fine[i - 1]![0]) * f, fine[i - 1]![1] + (fine[i]![1] - fine[i - 1]![1]) * f]);
  }
  return out;
}

/** 🌿️ The tiers of a scale's crown: shoots start under the head at `rho` and fan out; a tier grows in over 0.1 of the radius, so the count of tresses side by side doubles where a new tier starts. */
type Tier = { rho: number; rows: [number, number][] };
const countAt = (tiers: Tier[], rho: number, k: number): number => tiers.reduce((n, t, i) => n + t.rows.length * (i === 0 ? 1 : Math.min(1, Math.max(0, (rho - t.rho) / 0.1)) ** k), 0);

/** ⛲️ One fountain tress: it starts under the head (or under the tresses of the tier above) at the radius fraction of its tier, follows the dome of the crown outwards and down to its widest point and hangs from there vertically to its ragged tip at `tip` with a sideways `drift`; `share` is its half width as a share of the spacing of the tresses side by side, `b` its half depth. Upper and hanging parts are separate solids where the upper part is long (bones `cap-*` and `curt-*`). */
function jet(id: string, scale: number, tiers: Tier[], ti: number, az: number, drift: number, share0: number, b: number, step: number, split: boolean, k: number): Loft[] {
  const rad = radial(az), tan = tangential(az);
  const tip = around(HEM, az) + (scale === 500 ? 0.1 : 0), setIn = around(SET_IN, az), share = share0 * WIDTH[(k * 5 + ti * 2) % WIDTH.length]!;
  const t = tiers[ti]!;
  const top = meridian(az, t.rho, step);
  const secs: Sec[] = [];
  const at = (rM: number, y: number): V3 => [CX + rM * rad[0], y, rM * rad[2]];
  top.forEach(([rM, y], i) => {
    const [pr, py] = top[Math.max(0, i - 1)]!, [nr, ny] = top[Math.min(top.length - 1, i + 1)]!;
    const dr = nr - pr, dy = ny - py, l = Math.hypot(dr, dy) || 1;
    const n: [number, number] = [-dy / l, dr / l];
    const rho = rM / R(az);
    const grow = ti === 0 ? Math.min(1, 0.6 + (0.4 * (rho - t.rho)) / 0.08) : Math.min(1, Math.max(0, (rho - t.rho) / 0.1));
    const a = Math.max(ti === 0 ? 0.32 : 0.06, (share * 2 * Math.PI * rM) / Math.max(1, countAt(tiers, rho, 1)) * (0.25 + 0.75 * grow));
    const depth = b * (0.55 + 0.45 * grow);
    const tuck = (ti === 0 ? 0.12 : 0.22) * (1 - grow);
    const off = depth + tuck;
    const nn: V3 = unit(add(mul(rad, n[0]), [0, n[1], 0]));
    secs.push({ p: sub(at(rM, y), mul(nn, off + setIn)), rx: depth, ry: a, x: nn });
  });
  const aHang = (share * 2 * Math.PI * R(az)) / countAt(tiers, 1.2, 1);
  for (const [s, wf, df, rf] of HANG[scale]!.slice(1)) {
    const y = Y0 - s * (Y0 - tip);
    const depth = b * df;
    secs.push({ p: add(at(R(az) * rf - depth - setIn, y), mul(tan, drift * s * s)), rx: depth, ry: aHang * wf, x: rad });
  }
  const hangBone = curtainOf(secs[top.length - 1]!.p);
  const tone = 0.5 + 0.04 * ti;
  const whole: Loft = { kind: "loft", id, bone: hangBone, material: LEAF, tone, group: "tress", secs };
  if (!split) return [whole];
  const j = Math.max(2, top.findIndex(([rM]) => rM / R(az) >= 0.9));
  return [
    { ...whole, id: `${id}-arch`, bone: `cap-${side(secs[0]!.p[0])}`, tone: tone + 0.06, group: "arch", secs: [...secs.slice(0, j), { ...secs[j]!, rx: secs[j]!.rx * 0.82, ry: secs[j]!.ry * 0.82 }] },
    { ...whole, id, secs: secs.slice(j - 1) },
  ];
}

/** 🎐️ An inner tress hanging from the underside of the crown at the radius fraction `ring`, or from a point `from` of the wood, to `tip`. */
function inner(id: string, az: number, ring: number, tip: number, a: number, b: number, drift: number, from?: V3): Loft {
  const rad = radial(az), tan = tangential(az);
  const top: V3 = from ?? [CX + R(az) * ring * rad[0], yAt(ring, az) - 0.55, R(az) * ring * rad[2]];
  const secs: Sec[] = INNER_HANG.map(([s, wf, df]) => ({ p: add(add(top, [0, -s * (top[1] - tip) + 0.04, 0]), mul(tan, drift * s * s)), rx: b * df, ry: a * wf, x: rad }));
  return { kind: "loft", id, bone: curtainOf(secs[2]!.p), material: LEAF, tone: 0.4, group: from ? "fringe" : "tress", secs };
}

/** 🧮️ Crown tiers per scale: rows [azimuth, sideways drift of the tip]. */
const TIERS: Record<number, Tier[]> = {
  50: [
    { rho: 0.025, rows: [[3, 0.1], [31, -0.12], [62, 0.08], [88, -0.1], [121, 0.12], [149, -0.08], [178, 0.1], [209, -0.12], [242, 0.06], [268, -0.1], [301, 0.12], [332, -0.06]] },
    { rho: 0.5, rows: [[16, -0.1], [47, 0.12], [75, -0.08], [104, 0.1], [136, -0.12], [164, 0.08], [194, -0.1], [226, 0.12], [255, -0.06], [285, 0.1], [316, -0.12], [347, 0.08]] },
    { rho: 0.78, rows: [[9.5, 0.08], [23.5, -0.1], [39, 0.12], [54.5, -0.06], [68.5, 0.1], [81.5, -0.12], [96, 0.08], [112.5, -0.1], [128.5, 0.12], [142.5, -0.08], [156.5, 0.1], [171, -0.12],
      [186, 0.06], [201.5, -0.1], [217.5, 0.12], [234, -0.08], [248.5, 0.1], [261.5, -0.12], [276.5, 0.08], [293, -0.1], [308.5, 0.12], [324, -0.06], [339.5, 0.1], [355, -0.12]] },
  ],
  100: [
    { rho: 0.025, rows: [[5, 0.1], [50, -0.12], [95, 0.1], [140, -0.1], [185, 0.12], [230, -0.08], [275, 0.1], [320, -0.12]] },
    { rho: 0.5, rows: [[27.5, -0.1], [72.5, 0.12], [117.5, -0.08], [162.5, 0.1], [207.5, -0.12], [252.5, 0.08], [297.5, -0.1], [342.5, 0.12]] },
    { rho: 0.78, rows: [[16, 0.1], [39, -0.1], [61, 0.12], [84, -0.08], [106, 0.1], [129, -0.12], [151, 0.08], [174, -0.1],
      [196, 0.12], [219, -0.08], [241, 0.1], [264, -0.12], [286, 0.08], [309, -0.1], [331, 0.12], [354, -0.08]] },
  ],
  200: [
    { rho: 0.025, rows: [[0, 0], [60, 0], [120, 0], [180, 0], [240, 0], [300, 0]] },
    { rho: 0.5, rows: [[30, 0], [90, 0], [150, 0], [210, 0], [270, 0], [330, 0]] },
    { rho: 0.78, rows: [[15, 0], [45, 0], [75, 0], [105, 0], [135, 0], [165, 0], [195, 0], [225, 0], [255, 0], [285, 0], [315, 0], [345, 0]] },
  ],
  500: [{ rho: 0.025, rows: [[0, 0], [22.5, 0], [45, 0], [67.5, 0], [90, 0], [112.5, 0], [135, 0], [157.5, 0], [180, 0], [202.5, 0], [225, 0], [247.5, 0], [270, 0], [292.5, 0], [315, 0], [337.5, 0]] }],
};
const INNER_50: [number, number, number][] = [[6, 1.6, 0.1], [51, 1.3, 0.08], [96, 1.5, 0.12], [141, 1.2, 0.06], [186, 1.45, 0.1], [231, 1.25, 0.08], [276, 1.55, 0.12], [321, 1.35, 0.06], [28.5, 2.4, -0.1], [73.5, 2.1, -0.06], [118.5, 2.7, -0.08], [163.5, 1.9, -0.12], [208.5, 2.5, -0.06], [253.5, 2.2, -0.1], [298.5, 2.6, -0.08], [343.5, 2.0, -0.12]];
const INNER_100: [number, number, number][] = [[18, 1.8, 0.1], [90, 1.4, 0.1], [162, 1.6, 0.12], [234, 2.3, 0.08], [306, 2.6, 0.1], [54, 2.4, -0.1], [126, 2.5, -0.08], [198, 1.7, -0.1], [270, 1.45, -0.12], [342, 1.5, -0.08]];

function crown(scale: Scale, w: Loft[]): Part[] {
  const step = { 50: 0.55, 100: 0.9, 200: 1.5, 500: 2.4 }[scale];
  const b = { 50: 0.32, 100: 0.34, 200: 0.38, 500: 0.5 }[scale];
  const share = scale === 500 ? 0.62 : 0.68;
  const tiers = TIERS[scale]!;
  const out: Part[] = [];
  tiers.forEach((t, ti) => t.rows.forEach(([az, drift], k) => out.push(...jet(`tress-${"abc"[ti]}${String(k + 1).padStart(2, "0")}`, scale, tiers, ti, az, drift, share, b, step, scale !== 500 && t.rho < 0.7, k))));
  out.push({ kind: "lobe", id: "crown-tuft", bone: "cap-w", material: LEAF, tone: 0.62, group: "crown", c: [CX, yEnv(0, 0) - 0.2, 0], axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii: [0.6, 0.2, 0.6] });
  if (scale === 50 || scale === 100) (scale === 50 ? INNER_50 : INNER_100).forEach(([az, tip, drift], k) => out.push(inner(`inner-${String(k + 1).padStart(2, "0")}`, az, 0.66, tip, scale === 50 ? 0.75 : 0.9, 0.3, drift)));
  if (scale === 50) {
    const branches = w.filter((t) => t.group === "branch");
    const lb = (s: string) => w.find((t) => t.id === `wood-limb-${s}-b`)!;
    const from: [V3, number][] = [[along3(lb("w"), 0.6), 4.6], [along3(lb("e"), 0.6), 4.9], [along3(branches[0]!, 0.5), 4.4], [along3(branches[1]!, 0.55), 5.0], [along3(branches[2]!, 0.5), 4.7], [along3(branches[3]!, 0.55), 4.3]];
    from.forEach(([p, tip], k) => out.push(inner(`fringe-${String(k + 1).padStart(2, "0")}`, Math.atan2(p[2], p[0] - CX) / D2R, 0, tip, 0.45, 0.22, 0.06, add(p, [0, -0.03, 0]))));
  }
  return out;
}

/** 💨️ Outlet patches of the transpiration emitter: [anchor, part id or null for the crown top of that side]. */
const OUTLETS: [string, string][] = [["tp-curtain-1", "curt-w1"], ["tp-curtain-2", "curt-e1"], ["tp-curtain-3", "curt-w2"], ["tp-curtain-4", "curt-e2"], ["tp-curtain-5", "curt-w3"], ["tp-curtain-6", "curt-e3"], ["tp-curtain-7", "curt-w4"], ["tp-curtain-8", "curt-e4"], ["tp-curtain-9", "cap-w"], ["tp-curtain-10", "cap-e"]];

function outlets(scale: Scale, parts: Part[]): { solids: Json[]; anchors: Record<string, V3> } {
  const half = { 50: 0.01, 100: 0.01, 200: 0.02, 500: 0.04 }[scale];
  const solids: Json[] = [];
  const anchors: Record<string, V3> = {};
  OUTLETS.forEach(([anchor, b], i) => {
    let at: V3, n: V3, t: V3;
    if (b.startsWith("cap-")) {
      const arches = parts.filter((p): p is Loft => p.kind === "loft" && p.bone === b && p.group === "arch");
      const l = arches.sort((p, q) => q.secs[2]!.p[1] - p.secs[2]!.p[1])[0] ?? parts.find((p): p is Loft => p.kind === "loft" && p.group === "tress" && side(p.secs[0]!.p[0]) === b.slice(4))!;
      const k = Math.min(2, l.secs.length - 1);
      at = secPoint(l, k, 0);
      n = unit(sub(at, l.secs[k]!.p));
      t = unit(sub(l.secs[Math.min(l.secs.length - 1, k + 1)]!.p, l.secs[k]!.p));
    } else {
      const outer = parts.filter((p): p is Loft => p.kind === "loft" && p.group === "tress" && !p.id.includes("inner"));
      const mine = outer.filter((p) => p.bone === b);
      const pivot = F[b]!.o[0];
      const l = mine.length > 0 ? mine.sort((p, q) => Math.abs(q.secs[2]!.p[2]) - Math.abs(p.secs[2]!.p[2]))[0]! : outer.filter((p) => side(p.secs[2]!.p[0]) === side(pivot)).sort((p, q) => Math.abs(p.secs[2]!.p[0] - pivot) - Math.abs(q.secs[2]!.p[0] - pivot))[i % 2]!;
      const k = Math.floor(l.secs.length / 2) - 1;
      at = secPoint(l, k, 0);
      n = unit(sub(at, l.secs[k]!.p));
      t = [0, 1, 0];
    }
    const third = unit([n[1] * t[2] - n[2] * t[1], n[2] * t[0] - n[0] * t[2], n[0] * t[1] - n[1] * t[0]]);
    const tt = unit([third[1] * n[2] - third[2] * n[1], third[2] * n[0] - third[0] * n[2], third[0] * n[1] - third[1] * n[0]]);
    const lens: Lobe = { kind: "lobe", id: `emit-transpiration-curtain-${i + 1}`, bone: b, material: LEAF, tone: 0.5, group: "outlet", c: add(at, mul(n, -half * 1.2)), axes: [n, tt, third], radii: [half, b.startsWith("cap-") ? 0.35 : 0.45, b.startsWith("cap-") ? 0.18 : 0.22] };
    solids.push({ ...solidOf(lens, F), region: "leaf-curtain", tags: ["transpiration-curtain", "emitter"] });
    anchors[anchor] = add(at, mul(n, 0.04));
  });
  return { solids, anchors };
}

const regionOf = (p: Part): string => (p.group === "root" || p.group === "stem" ? "trunk-bark" : p.group === "limb" || p.group === "branch" ? "limb-bark" : "leaf-curtain");

/** ✂️ The section of a tube where it crosses the bone-local position `u` of frame `f` (interpolated between its stated sections). */
function cut(t: Loft, f: Frame, u: number): Sec {
  const us = t.secs.map((s) => toLocal(f, s.p)[0]);
  for (let i = 0; i + 1 < us.length; i++)
    if ((us[i]! - u) * (us[i + 1]! - u) <= 0 && us[i] !== us[i + 1]) {
      const k = (u - us[i]!) / (us[i + 1]! - us[i]!);
      const [a, b] = [t.secs[i]!, t.secs[i + 1]!];
      return { p: add(a.p, mul(sub(b.p, a.p), k)), rx: a.rx + (b.rx - a.rx) * k, ry: a.ry + (b.ry - a.ry) * k, x: a.x };
    }
  throw new Error(`${t.id} does not cross u = ${u}`);
}

/** 🏗️ The structural list of a scale: wood members under the bark along the limb axes, curtain and cap pendulums along their bones, joints, the root-plate support, masses. */
function structure(scale: Scale, w: Loft[]): Json[] {
  const out: Json[] = [];
  const bark = { 50: 0.025, 100: 0.025, 200: 0.03, 500: 0.03 }[scale];
  const memberOf: Record<string, string> = { "root-flare": "root-plate", "trunk-lower": "trunk-1", "trunk-upper": "trunk-2" };
  for (let t of w) {
    if (t.group === "branch") {
      if (scale === 50 || scale === 100) out.push({ ...solidOf({ ...t, id: `member-${t.id}`, material: WOOD, tone: 0.45, group: "member", secs: t.secs.slice(1).map((x) => ({ ...x, rx: Math.max(0.02, x.rx - 0.012), ry: Math.max(0.02, x.ry - 0.012) })) }, F), member: t.bone, part: "member" });
      continue;
    }
    const b = memberOf[t.id] ?? (t.id.startsWith("wood-limb-") ? t.id.slice(5) : null);
    if (b === null) continue;
    const f = F[b]!, L = bone(b).lengthM as number;
    const fork = w.find((x) => x.id === `fork-${b.slice(5, 6)}`);
    if (b.endsWith("-a") && fork) t = { ...t, secs: [...fork.secs.filter((x) => toLocal(f, x.p)[0] < 0.6), ...t.secs] };
    const inBone = [cut(t, f, 0), ...t.secs.filter((s) => { const u = toLocal(f, s.p)[0]; return u > 0.05 && u < L - 0.05; }), cut(t, f, L)];
    const keep = scale === 500 || scale === 200 ? [inBone[0]!, inBone[inBone.length - 1]!] : inBone;
    const part: Loft = { ...t, id: `member-${b}`, material: WOOD, tone: 0.45, group: "member", secs: keep.map((s) => ({ ...s, rx: Math.max(0.04, s.rx - bark), ry: Math.max(0.04, s.ry - bark) })) };
    out.push({ ...solidOf(part, F), member: b, part: "member" });
  }
  const rod = { 50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04 }[scale];
  for (const b of asset.rig.bones as Json[]) {
    if (!b.id.startsWith("curt-") && !b.id.startsWith("cap-")) continue;
    const f = F[b.id]!;
    const reach = b.id.startsWith("curt-") ? Math.min(b.lengthM, f.o[1] - 1.7) : b.lengthM;
    const part: Loft = { kind: "loft", id: `member-${b.id}`, bone: b.id, material: BARK, tone: 0.45, group: "member", secs: [0, reach].map((u: number) => ({ p: add(f.o, mul(f.u, u)), rx: rod, ry: rod, x: [0, 0, 1] as V3 })) };
    out.push({ ...solidOf(part, F), member: b.id, part: "member" });
  }
  const loc = (b: string, p: V3) => toLocal(F[b]!, p).map(r5);
  if (scale === 50 || scale === 100)
    for (const b of asset.rig.bones as Json[]) {
      if (b.parent === null || (scale === 100 && (b.id.startsWith("curt-") || b.id.startsWith("cap-")))) continue;
      const r = b.section ? Math.sqrt(b.section.areaM2 / Math.PI) - bark : rod * 1.6;
      out.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: WOOD, role: "detail", tone: 0.35, radii: [Math.min(r, F[b.id]!.o[1] - 0.02), r, r].map(r5), transform: { translation: [0, 0, 0] }, member: b.id, part: "joint" });
    }
  out.push({ id: "support-base-support", kind: "loft", bone: "root-plate", material: WOOD, role: "detail", tone: 0.25, sections: [0, Math.max(0.05, (0.15 * scale) / 1000 + 0.005)].map((u) => ({ at: [u, 0, 0], rx: scale === 500 ? 0.68 : 0.76, ry: scale === 500 ? 0.68 : 0.76, rotation: [0.5, 0.5, 0.5, 0.5] })), capStart: true, capEnd: true, member: "root-plate", part: "support", anchor: "base-support" });
  if (scale !== 500)
    for (const b of asset.rig.bones as Json[]) {
      const pendulum = b.id.startsWith("curt-") || b.id.startsWith("cap-");
      if (scale === 200 && !b.id.startsWith("curt-")) continue;
      if (scale === 100 && !pendulum) continue;
      const r = Math.max((0.15 * scale) / 1000, 0.06 * Math.cbrt(b.mass.kg / 35));
      out.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: pendulum ? LEAF : WOOD, role: "detail", tone: 0.6, radii: [r, r, r].map(r5), transform: { translation: [b.mass.comX, b.mass.comY ?? 0, 0] }, member: b.id, part: "mass" });
    }
  return out;
}

const DECISIONS: Record<Scale, { en: string; de: string }[]> = {
  50: [
    { en: "WILLOW 1:50: the habit of a golden weeping willow, stated section by section. Root flare with five surface roots, a short stout stem (0.85 m) swelling into the fork at 3.5 m with its two bark collars, two co-dominant limbs that rise at 19 degrees from the vertical, arch to 49 degrees and droop 30 degrees below the horizontal at their ends, and eight secondary branches that radiate from the outer limbs to every quarter and carry the crown in its depth (14 m). The crown is a fountain over a lobed plan, its top raised into a broad mound over each co-dominant limb, smaller mounds over the secondary branches and a shallow saddle over the fork: 48 tresses of hanging branchlets in three tiers (12 from the apex, 12 from half the radius, 24 from the shoulder) follow the rounded dome outwards and down to its widest point at 4.5 m and hang from there vertically, set in by up to 0.3 m against each other, to tapering, ragged tips 0.35 to 1.6 m above the ground, lifted to 2.2 to 2.75 m in two windows (south-west and north-east) where the stem and the room under the crown show, as where people walk under a real willow; the arching upper part of each first- and second-tier tress is its own solid on the crown-cap bones, the hanging part belongs to its curtain bone. Under the dome the space round the stem stays an open room: 16 shorter inner tresses and six fringes from the branches hang into it.", de: "TRAUERWEIDE 1:50: der Wuchs einer Gold-Trauerweide, Schnitt für Schnitt festgelegt. Wurzelanlauf mit fünf Oberflächenwurzeln, ein kurzer kräftiger Stamm (0,85 m), der mit seinen zwei Rindenwülsten in die Gabel bei 3,5 m anschwillt, zwei gleichrangige Stämmlinge, die mit 19 Grad zur Senkrechten aufsteigen, sich auf 49 Grad neigen und an den Enden 30 Grad unter die Waagrechte hängen, sowie acht Seitenäste, die von den äußeren Stämmlingen in jedes Viertel ausstrahlen und die Krone in ihrer Tiefe (14 m) tragen. Die Krone ist ein Springbrunnen über einem gelappten Grundriss, ihr Scheitel über jedem Stämmling zu einem breiten Hügel gehoben, kleinere Hügel über den Seitenästen und ein flacher Sattel über der Gabel: 48 Strähnen aus Hängezweigen in drei Rängen (12 vom Scheitel, 12 ab dem halben Radius, 24 ab der Schulter) folgen der gerundeten Kuppel nach außen und unten bis zu ihrer größten Breite bei 4,5 m und hängen von dort lotrecht, gegeneinander bis 0,3 m versetzt, bis zu spitz zulaufenden, ausgefransten Enden 0,35 bis 1,6 m über dem Boden, in zwei Fenstern (Südwest und Nordost) auf 2,2 bis 2,75 m angehoben, wo Stamm und Raum unter der Krone sichtbar werden, wie dort, wo man unter einer echten Weide hindurchgeht; der bogige obere Teil jeder Strähne des ersten und zweiten Rangs ist ein eigener Körper an den Kronenkappen-Knochen, der hängende Teil gehört zu seinem Vorhangknochen. Unter der Kuppel bleibt um den Stamm ein offener Raum: 16 kürzere innere Strähnen und sechs Fransen von den Seitenästen hängen hinein." },
    { en: "Thermal: every body solid by reference in its region (stem, fork collars, flare and surface roots in `trunk-bark`, limbs and branches in `limb-bark`, all foliage in `leaf-curtain`, an exchange area); the ground face of the flare is a contact face. The transpiration emitter leaves through ten flush outlet lenses in the outer faces of eight hanging tresses (one per curtain bone) and of two first-tier arches (one per crown cap).", de: "Thermik: jeder Körper per Verweis in seiner Region (Stamm, Gabelwülste, Anlauf und Oberflächenwurzeln in `trunk-bark`, Äste und Seitenäste in `limb-bark`, alles Laub in `leaf-curtain`, eine Austauschfläche); die Bodenfläche des Anlaufs ist Kontaktfläche. Der Transpirationsemitter tritt durch zehn bündige Auslasslinsen in den Außenflächen von acht hängenden Strähnen (eine je Vorhangknochen) und zwei Bögen des ersten Rangs (einer je Kronenkappe) aus." },
    { en: "Structure: green wood under 2.5 cm of bark along the stem and the six limb bones (the rig sections), the eight secondary branches as tapered wood members of their limb bone, the eight curtains as pendulum rods from their pivot down to 1.7 m and the two caps as rods along their bones, joints at every bone joint (the stem joint flattened above the ground), a root-plate disc of 1.5 m as the fixed support and a mass at the centre of mass of every bone.", de: "Tragwerk: grünes Holz unter 2,5 cm Rinde entlang Stamm und den sechs Astknochen (Querschnitte des Rigs), die acht Seitenäste als verjüngte Holzglieder ihres Astknochens, die acht Vorhänge als Pendelstäbe vom Drehpunkt bis 1,7 m und die zwei Kappen als Stäbe entlang ihrer Knochen, Gelenke an jedem Knochengelenk (das Stammgelenk über dem Boden abgeflacht), eine Wurzeltellerscheibe von 1,5 m als Einspannung und eine Masse im Schwerpunkt jedes Knochens." },
  ],
  100: [
    { en: "WILLOW 1:100, modelled again for 15 mm: three surface roots instead of five, stem, collars and limbs with fewer sections, the eight secondary branches in four steps; the fountain has 32 tresses over the same lobed plan, mounds and windows (8 from the apex, 8 from half the radius, 16 from the shoulder) with coarser steps along the dome and the same ragged hem, 10 inner tresses, no fringes (merged into the room's shade).", de: "TRAUERWEIDE 1:100, für 15 mm neu modelliert: drei statt fünf Oberflächenwurzeln, Stamm, Wülste und Äste mit weniger Schnitten, die acht Seitenäste in vier Schritten; der Springbrunnen hat 32 Strähnen über demselben gelappten Grundriss mit denselben Hügeln und Fenstern (8 vom Scheitel, 8 ab dem halben Radius, 16 ab der Schulter) mit gröberen Schritten über die Kuppel und demselben ausgefransten Saum, 10 innere Strähnen, keine Fransen (im Schatten des Raums aufgegangen)." },
    { en: "Thermal: references as at 1:50 by region, the ground face of the flare is a contact face; ten outlet lenses in the faces of hanging tresses and arches.", de: "Thermik: Verweise wie bei 1:50 je Region, die Bodenfläche des Anlaufs ist Kontaktfläche; zehn Auslasslinsen in den Flächen hängender Strähnen und Bögen." },
    { en: "Structure: wood members with fewer sections including the eight secondary branches, curtain and cap pendulum rods, joints only at the wood joints, the root-plate support and the ten pendulum masses (the wood masses are merged into their members).", de: "Tragwerk: Holzglieder mit weniger Schnitten samt den acht Seitenästen, Pendelstäbe der Vorhänge und Kappen, Gelenke nur an den Holzgelenken, die Wurzeltellerauflage und die zehn Pendelmassen (die Holzmassen gehen in ihren Gliedern auf)." },
  ],
  200: [
    { en: "WILLOW 1:200, modelled again for 30 mm: flare without surface roots, stem, collars and limbs in two to three sections, the four secondary branches in three steps; the fountain has 24 broad tresses (6, 6 and 12 per tier) in long steps, no inner tresses (they cannot be seen at this scale).", de: "TRAUERWEIDE 1:200, für 30 mm neu modelliert: Anlauf ohne Oberflächenwurzeln, Stamm, Wülste und Äste mit zwei bis drei Schnitten, die vier Seitenäste in drei Schritten; der Springbrunnen hat 24 breite Strähnen (6, 6 und 12 je Rang) in langen Schritten, keine inneren Strähnen (in diesem Maßstab nicht zu sehen)." },
    { en: "Thermal: references by region and ten outlet lenses; only the total area is compared at this scale.", de: "Thermik: Verweise je Region und zehn Auslasslinsen; in diesem Maßstab wird nur die Gesamtfläche verglichen." },
    { en: "Structure: wood members as prisms between their end sections, pendulum rods, the root-plate support and the eight curtain masses; joints are merged into the members.", de: "Tragwerk: Holzglieder als Prismen zwischen ihren Endschnitten, Pendelstäbe, die Wurzeltellerauflage und die acht Vorhangmassen; Gelenke gehen in den Gliedern auf." },
  ],
  500: [
    { en: "WILLOW 1:500, modelled again for 75 mm: flare, stem and limbs as single tapered tubes, the crown as one tier of 16 broad tresses from the apex over the mounded dome to the ragged hem with its two lifted windows, each one solid; the secondary branches are not modelled (inside the crown, below what the scale can show).", de: "TRAUERWEIDE 1:500, für 75 mm neu modelliert: Anlauf, Stamm und Äste als einzelne verjüngte Röhren, die Krone als ein Rang aus 16 breiten Strähnen vom Scheitel über die gehügelte Kuppel bis zum ausgefransten Saum mit seinen zwei angehobenen Fenstern, jede ein Körper; die Seitenäste sind nicht modelliert (innerhalb der Krone, unterhalb dessen, was der Maßstab zeigen kann)." },
    { en: "Thermal: references by region and ten outlet lenses as thick as the scale needs; only the total area is compared.", de: "Thermik: Verweise je Region und zehn Auslasslinsen so dick wie der Maßstab verlangt; nur die Gesamtfläche wird verglichen." },
    { en: "Structure: wood members as prisms of their end sections, the pendulum rods of the curtains and caps and the root-plate support; joints and masses are merged into the members.", de: "Tragwerk: Holzglieder als Prismen ihrer Endschnitte, die Pendelstäbe der Vorhänge und Kappen und die Wurzeltellerauflage; Gelenke und Massen gehen in den Gliedern auf." },
  ],
};

export const SCALES: Scale[] = [50, 100, 200, 500];
export function models(): Model[] {
  return SCALES.map((scale) => {
    const w = wood(scale);
    return { scale, parts: [...w, ...crown(scale, w)], decisions: DECISIONS[scale] };
  });
}

if (import.meta.main) {
  const all = models();
  const volumes: Json = {};
  const anchorsAll: Record<number, Record<string, V3>> = {};
  for (const m of all) {
    const body = m.parts.map((p) => solidOf(p, F));
    const thermal: Json[] = m.parts.map((p) => ({ id: `th-${p.id}`, of: p.id, region: regionOf(p), ...(p.id === "root-flare" ? { sectionFaces: ["-u"] } : {}) }));
    const o = outlets(m.scale, m.parts);
    anchorsAll[m.scale] = o.anchors;
    volumes[String(m.scale)] = { status: "authored", denominator: m.scale, body, thermal: [...thermal, ...o.solids], structure: structure(m.scale, m.parts.filter((p): p is Loft => p.kind === "loft" && ["root", "stem", "limb", "branch"].includes(p.group))), decisions: m.decisions };
  }
  for (const a of asset.anchors as Json[]) {
    const p = anchorsAll[50]![a.id];
    if (!p) continue;
    const l = toLocal(F[a.bone]!, p);
    a.x = r4(l[0]);
    a.y = r4(l[1]);
    a.z = r4(l[2]);
  }
  const notes = draw(asset, F, all);
  mkdirSync(join(GENERATED, "r5-weeping-willow"), { recursive: true });
  writeFileSync(join(SOURCES, "vegetation", "weeping-willow.json"), JSON.stringify({ volumes }, null, 2) + "\n");
  writeFileSync(join(GENERATED, "r5-weeping-willow", "🔣️.json"), JSON.stringify({ ...document, asset: { ...asset, volumes } }, null, 2) + "\n");
  for (const m of all) console.log(`1:${m.scale}: body ${volumes[m.scale].body.length}, thermal ${volumes[m.scale].thermal.length}, structure ${volumes[m.scale].structure.length}`);
  for (const n of notes) console.log(n);
}

export { F, asset, contains };
