/** 🧑‍💼️ The 1:50 body of the round 3 seated office worker, every section stated by hand: a man of 1.78 m in a long-sleeve shirt, belted suit trousers and leather shoes (rest frames of the rig: seated, arms hanging; the pose brings the hands to the keyboard), a task chair after EN 1335-1, a compact desk on four steel legs and a 15.6 inch notebook.
 * Rows are `[s, cv, cw, rx, ry, e?]`: s along the bone, cv along its normal (trunk, neck and head: backwards; arms and shank: forwards; thigh: upwards), cw to the left, rx the lateral and ry the normal semi-axis. Furniture points are stated in world coordinates (x ahead, y up, z left) and carried into the frame of their bone by `local`.
 * @see ./r3-seated-office-worker-ref.ts
 * @see ./r3-humans-seated-office-worker.md */
import { Kit, aboutU, aboutV, aboutW, dome, quat, type P2, type Row, type V3 } from "./r3-seated-office-worker-kit.ts";
import { CHAIR, DESK, LAPTOP, STAR } from "./r3-seated-office-worker-ref.ts";
import { aim, local } from "./r3-seated-office-worker-frame.ts";

export const TONE = { skin: 0.74, hair: 0.2, shirt: 0.84, trousers: 0.3, belt: 0.14, shoe: 0.16, sole: 0.08, plastic: 0.2, fabric: 0.34, steel: 0.62, rubber: 0.1, board: 0.8, frame: 0.5, alu: 0.68, keys: 0.16, display: 0.08, dark: 0.07 } as const;
const rad = (d: number): number => (d * Math.PI) / 180;
const radial = (yaw: number, r: number, y: number): V3 => [CHAIR.columnX + r * Math.cos(rad(yaw)), y, r * Math.sin(rad(yaw))];

/* ---------------------------------------------------------------- chair */

/** 🪑️ Task chair: five-star polyamide base with twin-wheel castors, gas lift in its telescopic cover, steel synchro mechanism with paddle, plastic seat shell under an upholstered cushion with waterfall front, backrest pad on its shell and J-bar, two T-armrests. */
export function chair(k: Kit): void {
  k.rev("chair-hub", "chair-base", "pa-plastic", TONE.plastic, [[-0.036, 0], [-0.036, 0.036], [-0.026, 0.044], [0.016, 0.044], [0.03, 0.036], [0.036, 0.028], [0.036, 0]], [0, 1, 0], [0, 0, 0], "chair-plastic");
  for (const [id, yaw] of STAR) {
    const arm = `chair-arm-${id}`;
    const root = radial(yaw, 0.02, 0.1);
    const tip = radial(yaw, CHAIR.armReach + 0.006, 0.082);
    const dir: V3 = [tip[0] - root[0], tip[1] - root[1], tip[2] - root[2]];
    const len = Math.hypot(...dir);
    k.loft(arm, arm, "pa-plastic", TONE.plastic, [[0, 0.004, 0, 0.026, 0.024, 3], [0.08, 0.002, 0, 0.024, 0.021, 3], [0.18, 0, 0, 0.02, 0.017, 3], [len - 0.024, -0.001, 0, 0.0165, 0.0145, 3], [len - 0.006, -0.001, 0, 0.0155, 0.014, 3], [len, -0.001, 0, 0.01, 0.01, 3]], "chair-plastic", { translation: local(arm, root), rotation: aim(arm, dir) });
    const boss = local(arm, radial(yaw, CHAIR.armReach, 0.081));
    k.cap(`chair-socket-${id}`, arm, "pa-plastic", TONE.plastic, [boss[0], boss[1] - 0.012, boss[2]], [boss[0], boss[1] + 0.008, boss[2]], 0.0145, 0.0145, "chair-plastic");
    const castor = `chair-castor-${id}`;
    const axle: V3 = [-Math.sin(rad(yaw)), 0, Math.cos(rad(yaw))];
    const out: V3 = [Math.cos(rad(yaw)), 0, Math.sin(rad(yaw))];
    for (const side of [1, -1]) {
      const at: V3 = [axle[0] * side * 0.0115, 0, axle[2] * side * 0.0115];
      const w = side > 0 ? "a" : "b";
      k.rev(`chair-wheel-${id}-${w}`, castor, "pa-plastic", TONE.plastic + 0.08, [[-0.0075, 0], [-0.0075, 0.0216], [0.0075, 0.0216], [0.0075, 0]], axle, at, "chair-plastic");
      k.rev(`chair-tyre-${id}-${w}`, castor, "rubber", TONE.rubber, [[-0.008, 0.0212], [-0.008, 0.0232], [-0.0058, 0.025], [0.0058, 0.025], [0.008, 0.0232], [0.008, 0.0212], [-0.008, 0.0212]], axle, at, "castor-rubber");
    }
    k.ell(`chair-hood-${id}`, castor, "pa-plastic", TONE.plastic + 0.04, [out[0] * 0.006, 0.014, out[2] * 0.006], [0.03, 0.026, 0.012], "chair-plastic", { rotation: aboutV(-yaw) });
    k.cap(`chair-stem-${id}`, castor, "steel-painted", TONE.steel, [out[0] * CHAIR.castorTrail, 0.034, out[2] * CHAIR.castorTrail], [out[0] * CHAIR.castorTrail, 0.06, out[2] * CHAIR.castorTrail], 0.0055, 0.0055, "chair-steel");
  }
  k.loft("chair-column-cover", "chair-column-sleeve", "pa-plastic", TONE.plastic - 0.04, [[0.028, 0, 0, 0.036, 0.036], [0.105, 0, 0, 0.036, 0.036], [0.112, 0, 0, 0.033, 0.033], [0.19, 0, 0, 0.033, 0.033], [0.197, 0, 0, 0.03, 0.03], [0.24, 0, 0, 0.03, 0.03]], "chair-plastic");
  k.loft("chair-column", "chair-column", "steel-painted", TONE.steel + 0.1, [[0.2, 0, 0, 0.025, 0.025], [0.3, 0, 0, 0.025, 0.025]], "chair-steel");
  k.box("chair-mechanism", "chair-seat", "steel-painted", TONE.steel - 0.2, [-0.01, 0.018, 0], [0.25, 0.036, 0.19], 0.008, "chair-steel");
  k.bar("chair-paddle", "chair-seat", "steel-painted", TONE.steel - 0.25, [[0.06, 0.016, -0.09, 0.0055], [0.085, 0.012, -0.17, 0.0055], [0.1, 0.008, -0.215, 0.0075, 0.005]], "chair-steel");
  const plan: readonly (readonly [s: number, v: number, half: number, t: number])[] = [[-0.235, 0.07, 0.19, 0.008], [-0.226, 0.068, 0.222, 0.018], [-0.2, 0.066, 0.238, 0.021], [-0.1, 0.063, 0.245, 0.0205], [0.05, 0.064, 0.245, 0.0205], [0.17, 0.066, 0.241, 0.021], [0.205, 0.063, 0.235, 0.019], [0.225, 0.054, 0.222, 0.013], [0.235, 0.046, 0.2, 0.006]];
  k.loft("chair-seat-shell", "chair-seat", "pa-plastic", TONE.plastic, [[-0.222, 0.0405, 0, 0.21, 0.0045, 4], [-0.2, 0.0405, 0, 0.232, 0.0045, 4], [0.19, 0.0405, 0, 0.236, 0.0045, 4], [0.215, 0.0405, 0, 0.222, 0.0045, 4]], "chair-plastic");
  k.loft("chair-seat-cushion", "chair-seat", "fabric-upholstery", TONE.fabric, plan.map(([s, v, half, t]) => [s, v, 0, half, t, 3.2] as Row), "chair-fabric");
  const back: readonly (readonly [u: number, cv: number, half: number, t: number])[] = [[0.13, -0.004, 0.17, 0.012], [0.142, -0.008, 0.205, 0.024], [0.17, -0.014, 0.218, 0.028], [0.23, -0.016, 0.224, 0.029], [0.32, -0.01, 0.228, 0.027], [0.43, -0.002, 0.226, 0.025], [0.53, 0.004, 0.218, 0.024], [0.575, 0.006, 0.205, 0.02], [0.598, 0.007, 0.17, 0.01]];
  k.loft("chair-backrest-pad", "chair-backrest", "fabric-upholstery", TONE.fabric, back.map(([u, cv, half, t]) => [u, cv, 0, half, t, 3.4] as Row), "chair-fabric");
  k.loft("chair-backrest-shell", "chair-backrest", "pa-plastic", TONE.plastic, back.slice(1, -1).map(([u, cv, half, t]) => [u, cv + t + 0.002, 0, half - 0.004, 0.0048, 3.4] as Row), "chair-plastic");
  const strut: P2[] = [[0.003, -0.085], [0.003, 0.004], [0.012, 0.024], [0.03, 0.037], [0.07, 0.043], [0.27, 0.04], [0.29, 0.033], [0.27, 0.023], [0.08, 0.025], [0.045, 0.02], [0.028, 0.01], [0.022, -0.002], [0.022, -0.085]];
  k.ext("chair-backrest-strut", "chair-backrest", "steel-painted", TONE.steel - 0.3, "xy", strut, -0.032, 0.032, "chair-steel");
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    k.bar(`chair-armrest-post-${s}`, "chair-seat", "pa-plastic", TONE.plastic, [[-0.03, 0.02, side * 0.09, 0.022, 0.009], [-0.03, 0.022, side * 0.215, 0.022, 0.0095], [-0.03, 0.05, side * CHAIR.armrestZ, 0.022, 0.012], [-0.026, 0.13, side * CHAIR.armrestZ, 0.024, 0.012], [-0.02, 0.262, side * CHAIR.armrestZ, 0.026, 0.012]], "chair-plastic", { e: 3 });
    const top = CHAIR.armrestTop - CHAIR.seatY;
    k.loft(`chair-armrest-pad-${s}`, "chair-seat", "pa-plastic", TONE.plastic - 0.06, [[-0.13, top - 0.012, side * CHAIR.armrestZ, 0.03, 0.008, 3], [-0.122, top - 0.012, side * CHAIR.armrestZ, 0.037, 0.012, 3], [0.1, top - 0.012, side * CHAIR.armrestZ, 0.04, 0.012, 3], [0.122, top - 0.012, side * CHAIR.armrestZ, 0.036, 0.011, 3], [0.13, top - 0.013, side * CHAIR.armrestZ, 0.028, 0.008, 3]], "chair-plastic");
  }
}

/* ---------------------------------------------------------------- desk and notebook */

/** 🗄️ Desk: 25 mm melamine board with ABS edge, four square steel tube legs with levelling glides, side and back rails under the board (the front stays open for the knees). */
export function desk(k: Kit): void {
  k.box("desk-top", "desk-top", "desk-board", TONE.board, [DESK.depth / 2, DESK.board / 2, 0], [DESK.depth, DESK.board, DESK.width], 0.003, "desk");
  const under = DESK.top - DESK.board;
  const lx = DESK.inset + DESK.leg / 2;
  const legW = DESK.width / 2 - lx;
  for (const [id, x, z] of [["fl", DESK.x0 + lx, legW], ["fr", DESK.x0 + lx, -legW], ["bl", DESK.x0 + DESK.depth - lx, legW], ["br", DESK.x0 + DESK.depth - lx, -legW]] as const) {
    const c = local("desk-leg", [x, (under + 0.012) / 2, z]);
    k.box(`desk-leg-${id}`, "desk-leg", "steel-painted", TONE.frame, c, [under - 0.012, DESK.leg, DESK.leg], 0.003, "desk");
    const g = local("desk-leg", [x, 0, z]);
    k.cap(`desk-glide-${id}`, "desk-leg", "steel-painted", TONE.dark, [0.004, g[1], g[2]], [0.012, g[1], g[2]], 0.012, 0.012, "desk");
  }
  for (const side of [1, -1]) k.box(`desk-rail-${side > 0 ? "l" : "r"}`, "desk-leg", "steel-painted", TONE.frame, local("desk-leg", [DESK.x0 + DESK.depth / 2, under - DESK.railH / 2, side * legW]), [DESK.railH, DESK.depth - 2 * lx - DESK.leg, DESK.railW], 0.002, "desk");
  k.box("desk-rail-back", "desk-leg", "steel-painted", TONE.frame, local("desk-leg", [DESK.x0 + DESK.depth - lx, under - DESK.railH / 2, 0]), [DESK.railH, DESK.railW, 2 * legW - DESK.leg], 0.002, "desk");
}

/** 💻️ Notebook: aluminium base with keyboard field and touchpad, hinge barrel, lid with display; the base stands on the desk, the lid opens 110 degrees. */
export function laptop(k: Kit): void {
  const top = LAPTOP.hingeY - DESK.top - 0.019;
  k.box("laptop-base", "laptop", "laptop-aluminium", TONE.alu, [LAPTOP.depth / 2 - 0.008, top + 0.0095, 0], [LAPTOP.depth, 0.019, LAPTOP.width], 0.004, "laptop");
  k.box("laptop-keyboard", "laptop", "laptop-aluminium", TONE.keys, [0.072, top - 0.0004, 0], [0.104, 0.0012, 0.31], 0.0005, "laptop", { texture: true, role: "detail" });
  k.box("laptop-touchpad", "laptop", "laptop-aluminium", TONE.alu - 0.12, [0.172, top - 0.0004, 0], [0.072, 0.0012, 0.115], 0.0005, "laptop", { texture: true, role: "detail" });
  k.cap("laptop-hinge", "laptop", "laptop-aluminium", TONE.alu - 0.2, [-0.002, top + 0.002, -0.13], [-0.002, top + 0.002, 0.13], 0.0075, 0.0075, "laptop");
  k.box("laptop-lid", "screen", "laptop-aluminium", TONE.alu, [LAPTOP.lidLength / 2 + 0.002, -0.0015, 0], [LAPTOP.lidLength - 0.004, 0.0075, LAPTOP.width], 0.003, "laptop");
  k.box("laptop-display", "screen", "laptop-aluminium", TONE.display, [0.122, 0.0026, 0], [0.19, 0.0012, 0.336], 0.0005, "laptop", { texture: true, role: "detail" });
}

/* ---------------------------------------------------------------- trunk, neck and head */

export const SEAT: Row[] = [[-0.098, 0.03, 0, 0.13, 0.07, 3], [-0.088, 0.03, 0, 0.17, 0.095, 2.8], [-0.06, 0.035, 0, 0.2, 0.105, 2.6], [-0.02, 0.025, 0, 0.207, 0.115, 2.5], [0.03, 0.012, 0, 0.198, 0.118, 2.4], [0.08, -0.004, 0, 0.172, 0.118, 2.4], [0.12, -0.022, 0, 0.163, 0.12, 2.4], [0.136, -0.024, 0, 0.161, 0.118, 2.4]];
export const SHIRT: Row[] = [[-0.02, -0.05, 0, 0.144, 0.098, 2.4], [0.02, -0.056, 0, 0.15, 0.106, 2.4], [0.06, -0.06, 0, 0.155, 0.11, 2.4], [0.12, -0.056, 0, 0.152, 0.112, 2.4], [0.2, -0.048, 0, 0.158, 0.118, 2.4], [0.28, -0.04, 0, 0.168, 0.13, 2.4], [0.35, -0.032, 0, 0.178, 0.132, 2.5], [0.41, -0.022, 0, 0.196, 0.12, 2.7], [0.448, -0.018, 0, 0.2, 0.112, 2.7], [0.47, -0.008, 0, 0.184, 0.09, 2.6], [0.49, 0.0, 0, 0.14, 0.074, 2.4], [0.505, 0.006, 0, 0.096, 0.062, 2.2], [0.518, 0.004, 0, 0.07, 0.056, 2]];
/** 👔️ The front of the shirt at height `s` on the centre line, or at lateral `w` (torso normal coordinate, negative is ahead). */
export function shirtFront(s: number, w = 0): number {
  for (let i = 1; i < SHIRT.length; i++) if (s <= SHIRT[i]![0]) {
    const [a, b] = [SHIRT[i - 1]!, SHIRT[i]!];
    const t = (s - a[0]) / (b[0] - a[0]);
    const [cv, rx, ry, e] = [a[1] + (b[1] - a[1]) * t, a[3] + (b[3] - a[3]) * t, a[4] + (b[4] - a[4]) * t, a[5]! + (b[5]! - a[5]!) * t];
    return cv - ry * (1 - Math.abs(w / rx) ** e) ** (1 / e);
  }
  return SHIRT[SHIRT.length - 1]![1] - SHIRT[SHIRT.length - 1]![4];
}
export const NECK: Row[] = [[-0.02, -0.006, 0, 0.06, 0.062], [0.03, -0.01, 0, 0.055, 0.058], [0.08, -0.012, 0, 0.053, 0.057], [0.125, -0.008, 0, 0.054, 0.06]];
export const HEAD: Row[] = [[-0.1, -0.082, 0, 0.012, 0.01, 2], [-0.0965, -0.078, 0, 0.026, 0.02, 2.2], [-0.087, -0.068, 0, 0.042, 0.032, 2.3], [-0.0705, -0.048, 0, 0.055, 0.054, 2.3], [-0.0465, -0.03, 0, 0.062, 0.074, 2.2], [-0.0167, -0.012, 0, 0.07, 0.092, 2.1], [0.02, -0.003, 0, 0.074, 0.1, 2.1], [0.055, 0.0, 0, 0.0755, 0.099, 2.1], [0.088, 0.003, 0, 0.071, 0.09, 2.1], [0.112, 0.006, 0, 0.055, 0.068, 2], [0.126, 0.008, 0, 0.032, 0.04, 2], [0.131, 0.008, 0, 0.012, 0.015, 2]];
export const HAIR: Row[] = [[-0.04, 0.035, 0, 0.03, 0.012, 2.2], [-0.015, 0.045, 0, 0.06, 0.041, 2.2], [0.02, 0.02, 0, 0.07, 0.085, 2.1], [0.045, 0.01, 0, 0.0785, 0.097, 2.1], [0.07, 0.005, 0, 0.0795, 0.096, 2.1], [0.095, 0.006, 0, 0.0725, 0.092, 2.1], [0.115, 0.007, 0, 0.059, 0.072, 2], [0.13, 0.008, 0, 0.036, 0.044, 2], [0.138, 0.008, 0, 0.012, 0.014, 2]];

export function trunk(k: Kit): void {
  k.loft("trousers-seat", "pelvis", "wool-blend", TONE.trousers, SEAT, "trousers");
  k.loft("belt", "pelvis", "leather-shoe", TONE.belt, [[0.104, -0.022, 0, 0.1655, 0.1225, 2.4], [0.134, -0.026, 0, 0.1635, 0.1205, 2.4]], "trousers");
  k.box("belt-buckle", "pelvis", "steel-painted", TONE.steel, [0.119, -0.146, 0], [0.034, 0.008, 0.05], 0.003, "trousers", { role: "detail" });
  k.loft("shirt", "torso", "cotton-shirt", TONE.shirt, SHIRT, "shirt");
  const collar = Array.from({ length: 11 }, (_, i) => {
    const t = (-150 + 30 * i) * (Math.PI / 180);
    const front = Math.abs(i - 5) / 5;
    return [0.024 - 0.022 * front ** 2, -0.01 + 0.067 * Math.cos(t) - (front === 1 ? 0.006 : 0), 0.066 * Math.sin(t) * (front === 1 ? 1.1 : 1), front === 1 ? 0.014 : 0.018, 0.0042];
  });
  k.bar("shirt-collar", "neck", "cotton-shirt", TONE.shirt + 0.06, collar, "shirt", { e: 3 });
  const stations = [0.49, 0.42, 0.34, 0.26, 0.18, 0.1, 0.02];
  k.bar("shirt-placket", "torso", "cotton-shirt", TONE.shirt + 0.06, stations.map((s) => [s, shirtFront(s) - 0.0006, 0, 0.0036, 0.016]), "shirt", { texture: true, role: "detail", e: 4 });
  [0.4, 0.31, 0.22, 0.13, 0.05].forEach((s, i) => k.ell(`shirt-button-${i + 1}`, "torso", "cotton-shirt", 0.3, [s, shirtFront(s) - 0.0035, 0], [0.0055, 0.0022, 0.0055], "shirt", { texture: true, role: "detail" }));
  const pocket = [0.05, 0.075, 0.1, 0.125].map((w) => [0.31, shirtFront(0.31, w) - 0.0005, w, 0.06, 0.0024]);
  k.bar("shirt-pocket", "torso", "cotton-shirt", TONE.shirt - 0.05, pocket, "shirt", { texture: true, role: "detail", e: 6 });
}

export function head(k: Kit): void {
  k.loft("neck", "neck", "skin", TONE.skin - 0.04, NECK, "face-skin");
  k.loft("head", "head", "skin", TONE.skin, HEAD, "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, HAIR, "hair");
  k.loft("nose", "head", "skin", TONE.skin, [[0.022, -0.1, 0, 0.007, 0.006, 2], [0.0, -0.11, 0, 0.0095, 0.009, 2], [-0.022, -0.116, 0, 0.0135, 0.0115, 2], [-0.034, -0.11, 0, 0.0165, 0.0125, 2], [-0.04, -0.1, 0, 0.0145, 0.01, 2]], "face-skin");
  k.ell("mouth", "head", "skin", TONE.skin - 0.28, [-0.06, -0.1, 0], [0.004, 0.005, 0.022], "face-skin", { role: "detail" });
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.002, 0.012, side * 0.074], [0.031, 0.018, 0.009], "face-skin", { rotation: quat(aboutU(side * -12), aboutW(-14)) });
    k.ell(`eye-${s}`, "head", "skin", TONE.dark + 0.08, [0.018, -0.087, side * 0.032], [0.0055, 0.006, 0.0125], "face-skin", { role: "detail" });
    k.bar(`brow-${s}`, "head", "hair", TONE.hair, [[0.034, -0.094, side * 0.013, 0.0042], [0.038, -0.092, side * 0.033, 0.0048], [0.033, -0.083, side * 0.053, 0.0038]], "hair", { role: "detail" });
  }
}

/* ---------------------------------------------------------------- limbs */

const SIDES = [["l", 1], ["r", -1]] as const;
export const SHOULDER_CAP: Row[] = [[-0.05, 0, -0.03, 0.012, 0.03], [-0.044, 0, -0.022, 0.028, 0.042], [-0.034, 0, -0.012, 0.042, 0.052], [-0.02, 0, -0.004, 0.051, 0.056], [-0.006, 0, 0.002, 0.055, 0.058]];
export const UPPER_ARM: Row[] = [...SHOULDER_CAP, [0, 0, 0.004, 0.056, 0.058], [0.05, 0.002, 0.004, 0.054, 0.056], [0.12, 0.004, 0.002, 0.048, 0.051], [0.2, 0.004, 0, 0.044, 0.048], [0.27, 0.002, 0, 0.042, 0.045], [0.33, 0, 0, 0.042, 0.042], ...dome(0.372, 0.33, 0, 0, 0.042, 0.042).reverse()];
export const FOREARM: Row[] = [...dome(-0.04, -0.004, 0, 0, 0.036, 0.035), [0, 0, 0, 0.043, 0.042], [0.05, 0.002, 0, 0.045, 0.043], [0.12, 0.001, 0, 0.04, 0.036], [0.19, 0, 0, 0.035, 0.03], [0.205, 0, 0, 0.034, 0.029]];
export const CUFF: Row[] = [[0.195, 0, 0, 0.035, 0.03, 2.4], [0.2, 0, 0, 0.038, 0.032, 2.4], [0.238, 0, 0, 0.038, 0.032, 2.4], [0.243, 0, 0, 0.035, 0.029, 2.4]];
export const WRIST: Row[] = [[0.232, 0, 0, 0.0305, 0.021], [0.255, 0, 0, 0.0295, 0.0195], [0.272, 0, 0, 0.029, 0.019]];
export const THIGH: Row[] = [...dome(-0.07, 0, -0.004, 0.01, 0.092, 0.078), [0, -0.004, 0.01, 0.097, 0.08, 2.4], [0.08, -0.004, 0.007, 0.093, 0.0795, 2.3], [0.18, -0.001, 0.004, 0.086, 0.076], [0.28, 0.003, 0.0, 0.075, 0.069], [0.36, 0.007, 0, 0.064, 0.063], [0.42, 0.011, 0, 0.0585, 0.06], [0.45, 0.011, 0, 0.057, 0.059], ...dome(0.49, 0.45, 0.011, 0, 0.057, 0.059).reverse()];
export const SHANK: Row[] = [...dome(-0.045, 0, 0.003, 0, 0.048, 0.052), [0, 0.003, 0, 0.052, 0.057], [0.06, 0.0, 0, 0.058, 0.064], [0.14, -0.004, 0, 0.061, 0.067], [0.24, -0.004, 0, 0.058, 0.064], [0.33, 0.0, 0, 0.056, 0.062], [0.405, 0.002, 0, 0.057, 0.063], [0.418, -0.002, 0, 0.058, 0.064], [0.428, -0.004, 0, 0.055, 0.061]];
/** 👞️ The shoe in a level frame at the ankle (s ahead, cv up; sole at cv -0.092): heel counter, quarters, instep and vamp to the ball, where the toe bone takes over. */
export const SHOE: Row[] = [[-0.074, -0.046, 0, 0.026, 0.036, 2.6], [-0.068, -0.044, 0, 0.036, 0.046, 2.6], [-0.045, -0.038, 0, 0.042, 0.056, 2.6], [-0.01, -0.038, 0, 0.044, 0.056, 2.6], [0.03, -0.05, 0, 0.046, 0.042, 2.6], [0.075, -0.06, 0, 0.049, 0.032, 2.6], [0.12, -0.066, 0, 0.051, 0.026, 2.6], [0.15, -0.068, 0, 0.0515, 0.024, 2.6]];
export const TOE: Row[] = [[-0.012, 0.025, 0, 0.0515, 0.022, 2.6], [0.03, 0.023, 0, 0.049, 0.02, 2.6], [0.058, 0.021, 0, 0.042, 0.017, 2.6], [0.074, 0.019, 0, 0.03, 0.013, 2.4], [0.082, 0.018, 0, 0.016, 0.009, 2.2]];
export const FOOT_DROP = (Math.atan2(0.092, 0.14) * 180) / Math.PI;

/** ✋️ One finger as a bent bar in the hand frame (u to the finger tips, v out of the back of the hand, w across), flexing towards the palm (-v): knuckle, joints and tip from the flexion of the three joints in degrees. */
export function fingerPath(w: number, scale: number, r: number, flex: readonly [number, number, number], knuckle = 0.096): number[][] {
  const lengths = [0.045, 0.027, 0.022].map((l) => l * scale);
  let [u, p, angle] = [knuckle, -0.002, 0];
  const out: number[][] = [[u - 0.014, p + 0.001, w * 0.9, r * 1.08], [u, p, w, r * 1.1]];
  lengths.forEach((l, i) => {
    const half = (flex[i]! * Math.PI) / 360;
    const corner = Math.min(0.008, l * 0.3);
    angle -= half;
    u += corner * Math.cos(angle);
    p += corner * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.06 * i)]);
    angle -= half;
    u += (l - corner) * Math.cos(angle);
    p += (l - corner) * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.06 * (i + 1)) * (i === 2 ? 0.85 : 1)]);
  });
  return out;
}
/** 🤏️ Finger flexion (index to little finger) and the thumb path `[u, v, towards the thumb side, radius]` of the typing hand. */
export const TYPING = { flex: [[18, 46, 26], [16, 44, 26], [18, 46, 26], [22, 48, 26]], thumb: [[0.018, -0.008, 0.026, 0.0125], [0.045, -0.02, 0.044, 0.0115], [0.072, -0.03, 0.05, 0.0102], [0.098, -0.04, 0.046, 0.009]] } as const;
const FINGERS: readonly (readonly [w: number, scale: number, r: number])[] = [[0.029, 0.94, 0.0088], [0.0095, 1, 0.009], [-0.0105, 0.95, 0.0085], [-0.029, 0.8, 0.0076]];

/** 🖐️ A typing hand, palm down (-v), thumb on the inner side (-w on the left hand, +w on the right). */
export function hand(k: Kit, s: "l" | "r"): void {
  const t = s === "l" ? -1 : 1;
  const bone = `hand-${s}`;
  k.loft(`palm-${s}`, bone, "skin", TONE.skin, [[-0.006, 0, 0, 0.029, 0.019, 2.4], [0.03, -0.001, t * 0.003, 0.039, 0.0165, 2.6], [0.066, -0.002, t * 0.001, 0.044, 0.0145, 2.8], [0.092, -0.002, -t * 0.001, 0.043, 0.0125, 2.8], [0.103, -0.002, -t * 0.001, 0.04, 0.009, 2.5]], "hand-skin");
  k.ell(`thenar-${s}`, bone, "skin", TONE.skin, [0.038, -0.011, t * 0.027], [0.03, 0.014, 0.017], "hand-skin");
  FINGERS.forEach(([w, sc, r], i) => k.bar(`finger-${i + 1}-${s}`, bone, "skin", TONE.skin - 0.02 * (i % 2), fingerPath(t * w, sc, r, TYPING.flex[i] as [number, number, number]), "hand-skin"));
  k.bar(`thumb-${s}`, bone, "skin", TONE.skin, TYPING.thumb.map(([u, p, w, r]) => [u, p, t * w, r]), "hand-skin");
}

export function limbs(k: Kit): void {
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "cotton-shirt", TONE.shirt - 0.03, UPPER_ARM.map(([a, cv, cw, rx, ry]) => [a, cv, side * cw, rx, ry] as Row), "shirt");
    k.loft(`forearm-${s}`, `forearm-${s}`, "cotton-shirt", TONE.shirt - 0.03, FOREARM, "shirt");
    k.loft(`cuff-${s}`, `forearm-${s}`, "cotton-shirt", TONE.shirt + 0.04, CUFF, "shirt");
    k.loft(`wrist-${s}`, `forearm-${s}`, "skin", TONE.skin, WRIST, "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "wool-blend", TONE.trousers, THIGH.map(([a, cv, cw, rx, ry, e]) => (e === undefined ? [a, cv, side * cw, rx, ry] : [a, cv, side * cw, rx, ry, e]) as Row), "trousers");
    k.loft(`shank-${s}`, `shank-${s}`, "wool-blend", TONE.trousers - 0.02, SHANK, "trousers");
    const level = { rotation: aboutW(FOOT_DROP) };
    k.loft(`shoe-${s}`, `foot-${s}`, "leather-shoe", TONE.shoe, SHOE, "shoes", level);
    k.loft(`shoe-heel-${s}`, `foot-${s}`, "leather-shoe", TONE.sole, [[-0.074, -0.0795, 0, 0.026, 0.0125, 4], [-0.066, -0.0795, 0, 0.036, 0.0125, 4], [-0.035, -0.0795, 0, 0.041, 0.0125, 4], [-0.005, -0.0795, 0, 0.041, 0.0125, 4]], "shoes", level);
    k.loft(`shoe-sole-${s}`, `foot-${s}`, "leather-shoe", TONE.sole, [[0.0, -0.0855, 0, 0.04, 0.0055, 4], [0.06, -0.087, 0, 0.047, 0.005, 4], [0.15, -0.0875, 0, 0.0525, 0.0045, 4]], "shoes", level);
    k.loft(`shoe-toe-${s}`, `toe-${s}`, "leather-shoe", TONE.shoe + 0.03, TOE, "shoes");
    k.loft(`shoe-toe-sole-${s}`, `toe-${s}`, "leather-shoe", TONE.sole, [[-0.012, 0.0045, 0, 0.0525, 0.0045, 4], [0.04, 0.0045, 0, 0.05, 0.0045, 4], [0.07, 0.005, 0, 0.038, 0.0045, 4], [0.084, 0.006, 0, 0.02, 0.0042, 3]], "shoes");
    hand(k, s);
  }
}

/** 🧱️ The whole 1:50 body. */
export function body50(k: Kit): void {
  chair(k);
  desk(k);
  laptop(k);
  trunk(k);
  head(k);
  limbs(k);
}
