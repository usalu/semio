/** 🪆️ The coarser bodies of the round 3 seated office worker, each stated anew for its scale: 1:100 (smallest feature 15 mm: collar, cuffs, belt buckle, face details, castor stems and twin wheels, seat shell and glides merged; mitten hands with thumbs, one shoe form per foot bone), 1:200 (30 mm: one form per body segment, the head without features, hands as mittens, the chair as base, wheels, column, seat, backrest with its bar and two armrests, the desk as board, legs and rails, the notebook as base and lid) and 1:500 (75 mm: trunk, head with hair, limbs with hands, the chair as star, column, seat and backrest, the desk as board and legs, the notebook as one open wedge of base and lid).
 * Rows are `[s, cv, cw, rx, ry, e?]` as in the 1:50 body.
 * @see ./r3-seated-office-worker-s50.ts */
import { Kit, aboutV, aboutW, dome, type P2, type Row, type V3 } from "./r3-seated-office-worker-kit.ts";
import { CHAIR, DESK, LAPTOP, STAR } from "./r3-seated-office-worker-ref.ts";
import { aim, local } from "./r3-seated-office-worker-frame.ts";
import { FOOT_DROP, TONE } from "./r3-seated-office-worker-s50.ts";

const SIDES = [["l", 1], ["r", -1]] as const;
const rad = (d: number): number => (d * Math.PI) / 180;
const radial = (yaw: number, r: number, y: number): V3 => [CHAIR.columnX + r * Math.cos(rad(yaw)), y, r * Math.sin(rad(yaw))];
const side = (rows: readonly Row[], s: number): Row[] => rows.map(([a, cv, cw, rx, ry, e]) => (e === undefined ? [a, cv, s * cw, rx, ry] : [a, cv, s * cw, rx, ry, e]) as Row);

/* ---------------------------------------------------------------- furniture */

type ChairSpec = { arm: readonly Row[]; wheel: number; hood: boolean; pad: readonly (readonly [number, number, number, number])[]; back: readonly (readonly [number, number, number, number])[]; shell: number; strut: number; armrests: boolean; mechanism: boolean };
function chair(k: Kit, c: ChairSpec): void {
  k.rev("chair-hub", "chair-base", "pa-plastic", TONE.plastic, [[-0.036, 0], [-0.036, 0.04], [0.02, 0.044], [0.036, 0.03], [0.036, 0]], [0, 1, 0], [0, 0, 0], "chair-plastic");
  for (const [id, yaw] of STAR) {
    const arm = `chair-arm-${id}`;
    const root = radial(yaw, 0.02, 0.1);
    const tip = radial(yaw, CHAIR.armReach + 0.006, 0.082);
    const dir: V3 = [tip[0] - root[0], tip[1] - root[1], tip[2] - root[2]];
    const len = Math.hypot(...dir);
    k.loft(arm, arm, "pa-plastic", TONE.plastic, c.arm.map(([s, cv, cw, rx, ry, e]) => [s < 0 ? len + s : s, cv, cw, rx, ry, e ?? 3] as Row), "chair-plastic", { translation: local(arm, root), rotation: aim(arm, dir) });
    const castor = `chair-castor-${id}`;
    const axle: V3 = [-Math.sin(rad(yaw)), 0, Math.cos(rad(yaw))];
    const out: V3 = [Math.cos(rad(yaw)), 0, Math.sin(rad(yaw))];
    const half = c.wheel / 2;
    k.rev(`chair-wheel-${id}`, castor, "pa-plastic", TONE.plastic + 0.08, [[-half + 0.0005, 0], [-half + 0.0005, 0.0216], [half - 0.0005, 0.0216], [half - 0.0005, 0]], axle, [0, 0, 0], "chair-plastic");
    k.rev(`chair-tyre-${id}`, castor, "rubber", TONE.rubber, [[-half, 0.0212], [-half, 0.0232], [-half + 0.0022, 0.025], [half - 0.0022, 0.025], [half, 0.0232], [half, 0.0212], [-half, 0.0212]], axle, [0, 0, 0], "castor-rubber");
    if (c.hood) k.ell(`chair-hood-${id}`, castor, "pa-plastic", TONE.plastic + 0.04, [out[0] * 0.01, 0.03, out[2] * 0.01], [0.032, 0.034, 0.012], "chair-plastic", { rotation: aboutV(-yaw) });
  }
  k.loft("chair-column-cover", "chair-column-sleeve", "pa-plastic", TONE.plastic - 0.04, c.hood ? [[0.028, 0, 0, 0.036, 0.036], [0.11, 0, 0, 0.034, 0.034], [0.24, 0, 0, 0.03, 0.03]] : [[0.028, 0, 0, 0.036, 0.036], [0.24, 0, 0, 0.032, 0.032]], "chair-plastic");
  k.loft("chair-column", "chair-column", "steel-painted", TONE.steel + 0.1, [[0.2, 0, 0, Math.max(0.025, c.wheel / 2.4), Math.max(0.025, c.wheel / 2.4)], [0.3, 0, 0, Math.max(0.025, c.wheel / 2.4), Math.max(0.025, c.wheel / 2.4)]], "chair-steel");
  if (c.mechanism) k.box("chair-mechanism", "chair-seat", "steel-painted", TONE.steel - 0.2, [-0.01, 0.018, 0], [0.25, 0.036, 0.19], 0.008, "chair-steel");
  k.loft("chair-seat-cushion", "chair-seat", "fabric-upholstery", TONE.fabric, c.pad.map(([s, v, h, t]) => [s, v, 0, h, t, 3.2] as Row), "chair-fabric");
  k.loft("chair-backrest-pad", "chair-backrest", "fabric-upholstery", TONE.fabric, c.back.map(([u, cv, h, t]) => [u, cv, 0, h, t, 3.4] as Row), "chair-fabric");
  if (c.shell > 0) k.loft("chair-backrest-shell", "chair-backrest", "pa-plastic", TONE.plastic, c.back.slice(1, -1).map(([u, cv, h, t]) => [u, cv + t + c.shell / 2 - 0.002, 0, h - 0.004, c.shell / 2, 3.4] as Row), "chair-plastic");
  const t = c.strut;
  const strut: P2[] = [[0.003, -0.085], [0.003, 0.004], [0.03, 0.022 + t], [0.27, 0.022 + t], [0.27, 0.022], [0.04, 0.022], [0.003 + t, -0.002], [0.003 + t, -0.085]];
  k.ext("chair-backrest-strut", "chair-backrest", "steel-painted", TONE.steel - 0.3, "xy", strut, -0.032, 0.032, "chair-steel");
  if (c.armrests) for (const [s, sd] of SIDES) {
    const top = CHAIR.armrestTop - CHAIR.seatY;
    const w = Math.max(0.024, t);
    k.loft(`chair-armrest-post-${s}`, "chair-seat", "pa-plastic", TONE.plastic, [[0.02, 0.025, sd * CHAIR.armrestZ, w / 2 + 0.003, 0.026], [top - 0.01, 0.02, sd * CHAIR.armrestZ, w / 2 + 0.003, 0.026]], "chair-plastic", { rotation: aboutW(90), translation: [0, 0, 0] });
    k.loft(`chair-armrest-pad-${s}`, "chair-seat", "pa-plastic", TONE.plastic - 0.06, [[-0.13, top - 0.012, sd * CHAIR.armrestZ, 0.034, Math.max(0.012, t / 2), 3], [0.13, top - 0.012, sd * CHAIR.armrestZ, 0.034, Math.max(0.012, t / 2), 3]], "chair-plastic");
  }
}

function desk(k: Kit, rails: number): void {
  k.box("desk-top", "desk-top", "desk-board", TONE.board, [DESK.depth / 2, DESK.board / 2, 0], [DESK.depth, DESK.board, DESK.width], 0.003, "desk");
  const under = DESK.top - DESK.board;
  const lx = DESK.inset + DESK.leg / 2;
  const legW = DESK.width / 2 - lx;
  for (const [id, x, z] of [["fl", DESK.x0 + lx, legW], ["fr", DESK.x0 + lx, -legW], ["bl", DESK.x0 + DESK.depth - lx, legW], ["br", DESK.x0 + DESK.depth - lx, -legW]] as const) k.box(`desk-leg-${id}`, "desk-leg", "steel-painted", TONE.frame, local("desk-leg", [x, under / 2, z]), [under, DESK.leg, DESK.leg], 0.003, "desk");
  if (rails > 0) {
    for (const sd of [1, -1]) k.box(`desk-rail-${sd > 0 ? "l" : "r"}`, "desk-leg", "steel-painted", TONE.frame, local("desk-leg", [DESK.x0 + DESK.depth / 2, under - DESK.railH / 2, sd * (legW + DESK.leg / 2 - rails / 2)]), [DESK.railH, DESK.depth - 2 * lx - DESK.leg, rails], 0.002, "desk");
    k.box("desk-rail-back", "desk-leg", "steel-painted", TONE.frame, local("desk-leg", [DESK.x0 + DESK.depth - lx + DESK.leg / 2 - rails / 2, under - DESK.railH / 2, 0]), [DESK.railH, rails, 2 * legW - DESK.leg], 0.002, "desk");
  }
}

function laptop(k: Kit, base: number, lid: number): void {
  const top = LAPTOP.hingeY - DESK.top - 0.019;
  k.box("laptop-base", "laptop", "laptop-aluminium", TONE.alu, [LAPTOP.depth / 2 - 0.008, top + 0.019 - base / 2, 0], [LAPTOP.depth, base, LAPTOP.width], 0.004, "laptop");
  k.box("laptop-lid", "screen", "laptop-aluminium", TONE.alu, [LAPTOP.lidLength / 2 + 0.002, 0.0022 - lid / 2, 0], [LAPTOP.lidLength - 0.004, lid, LAPTOP.width], 0.003, "laptop");
}

/* ---------------------------------------------------------------- the sitter */

type Person = { seat: readonly Row[]; shirt: readonly Row[]; belt: boolean; neck: readonly Row[]; head: readonly Row[]; hair: readonly Row[]; nose: boolean; ears: boolean; arm: readonly Row[]; fore: readonly Row[]; hand: readonly Row[]; thumb: readonly (readonly number[])[] | null; thigh: readonly Row[]; shank: readonly Row[]; shoe: readonly Row[]; toe: readonly Row[] | null };
function person(k: Kit, p: Person): void {
  k.loft("trousers-seat", "pelvis", "wool-blend", TONE.trousers, p.seat, "trousers");
  if (p.belt) k.loft("belt", "pelvis", "leather-shoe", TONE.belt, [[0.104, -0.022, 0, 0.1655, 0.1225, 2.4], [0.136, -0.026, 0, 0.1635, 0.1205, 2.4]], "trousers");
  k.loft("shirt", "torso", "cotton-shirt", TONE.shirt, p.shirt, "shirt");
  k.loft("neck", "neck", "skin", TONE.skin - 0.04, p.neck, "face-skin");
  k.loft("head", "head", "skin", TONE.skin, p.head, "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, p.hair, "hair");
  if (p.nose) k.loft("nose", "head", "skin", TONE.skin, [[0.022, -0.098, 0, 0.0085, 0.008, 2], [-0.022, -0.112, 0, 0.0135, 0.011, 2], [-0.04, -0.1, 0, 0.0145, 0.008, 2]], "face-skin");
  if (p.ears) for (const [s, sd] of SIDES) k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.002, 0.012, sd * 0.072], [0.03, 0.017, 0.0078], "face-skin", { rotation: aboutW(-14) });
  for (const [s, sd] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "cotton-shirt", TONE.shirt - 0.03, side(p.arm, sd), "shirt");
    k.loft(`forearm-${s}`, `forearm-${s}`, "cotton-shirt", TONE.shirt - 0.03, p.fore, "shirt");
    const t = s === "l" ? -1 : 1;
    k.loft(`hand-${s}`, `hand-${s}`, "skin", TONE.skin, side(p.hand, -t), "hand-skin");
    if (p.thumb !== null) k.bar(`thumb-${s}`, `hand-${s}`, "skin", TONE.skin, p.thumb.map(([u, v, w, r]) => [u!, v!, t * w!, r!]), "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "wool-blend", TONE.trousers, side(p.thigh, sd), "trousers");
    k.loft(`shank-${s}`, `shank-${s}`, "wool-blend", TONE.trousers - 0.02, p.shank, "trousers");
    k.loft(`shoe-${s}`, `foot-${s}`, "leather-shoe", TONE.shoe, p.shoe, "shoes", { rotation: aboutW(FOOT_DROP) });
    if (p.toe !== null) k.loft(`shoe-toe-${s}`, `toe-${s}`, "leather-shoe", TONE.shoe + 0.03, p.toe, "shoes");
  }
}

/* ---------------------------------------------------------------- 1:100 */

export function body100(k: Kit): void {
  chair(k, {
    arm: [[0, 0.004, 0, 0.026, 0.024], [0.16, 0, 0, 0.021, 0.018], [-0.02, -0.001, 0, 0.0165, 0.015], [-0.001, -0.001, 0, 0.012, 0.012]],
    wheel: 0.04, hood: true, shell: 0.015, strut: 0.018, armrests: true, mechanism: true,
    pad: [[-0.235, 0.064, 0.19, 0.012], [-0.215, 0.062, 0.232, 0.025], [-0.1, 0.061, 0.245, 0.025], [0.17, 0.063, 0.241, 0.025], [0.215, 0.058, 0.228, 0.02], [0.235, 0.05, 0.2, 0.01]],
    back: [[0.13, -0.004, 0.17, 0.012], [0.15, -0.01, 0.21, 0.026], [0.24, -0.016, 0.224, 0.029], [0.43, -0.002, 0.226, 0.025], [0.56, 0.005, 0.212, 0.022], [0.598, 0.007, 0.17, 0.01]],
  });
  desk(k, 0.02);
  laptop(k, 0.019, 0.015);
  person(k, {
    seat: [[-0.098, 0.03, 0, 0.14, 0.075, 3], [-0.07, 0.034, 0, 0.198, 0.104, 2.6], [-0.02, 0.025, 0, 0.207, 0.115, 2.5], [0.06, 0.003, 0, 0.18, 0.118, 2.4], [0.136, -0.024, 0, 0.162, 0.118, 2.4]],
    belt: true,
    shirt: [[-0.02, -0.05, 0, 0.144, 0.098, 2.4], [0.06, -0.06, 0, 0.155, 0.11, 2.4], [0.2, -0.048, 0, 0.158, 0.118, 2.4], [0.3, -0.038, 0, 0.17, 0.131, 2.4], [0.41, -0.022, 0, 0.196, 0.12, 2.7], [0.448, -0.018, 0, 0.2, 0.112, 2.7], [0.475, -0.006, 0, 0.18, 0.086, 2.6], [0.5, 0.004, 0, 0.12, 0.068, 2.3], [0.528, 0.004, 0, 0.074, 0.064, 2]],
    neck: [[-0.02, -0.006, 0, 0.06, 0.062], [0.125, -0.008, 0, 0.054, 0.06]],
    head: [[-0.106, -0.08, 0, 0.022, 0.017, 2.2], [-0.09, -0.064, 0, 0.046, 0.038, 2.3], [-0.05, -0.03, 0, 0.062, 0.074, 2.2], [0.0, -0.008, 0, 0.073, 0.097, 2.1], [0.055, 0.0, 0, 0.077, 0.099, 2.1], [0.105, 0.005, 0, 0.061, 0.076, 2], [0.131, 0.008, 0, 0.016, 0.02, 2]],
    hair: [[0.0, 0.05, 0, 0.074, 0.054, 2.2], [0.04, 0.024, 0, 0.085, 0.086, 2.1], [0.085, 0.007, 0, 0.083, 0.1, 2.1], [0.12, 0.007, 0, 0.059, 0.073, 2], [0.1405, 0.008, 0, 0.014, 0.018, 2]],
    nose: true, ears: true,
    arm: [...dome(-0.05, 0, 0, 0.004, 0.056, 0.058, 3), [0, 0, 0.004, 0.056, 0.058], [0.14, 0.004, 0.002, 0.047, 0.05], [0.33, 0, 0, 0.042, 0.042], ...dome(0.372, 0.33, 0, 0, 0.042, 0.042, 3).reverse()],
    fore: [...dome(-0.044, 0, 0, 0, 0.043, 0.042, 3), [0, 0, 0, 0.043, 0.042], [0.06, 0.002, 0, 0.045, 0.043], [0.2, 0, 0, 0.036, 0.031], [0.242, 0, 0, 0.037, 0.031], [0.262, 0, 0, 0.03, 0.02]],
    hand: [[-0.004, 0, 0, 0.03, 0.019, 2.4], [0.05, -0.002, 0.002, 0.044, 0.016, 2.6], [0.095, -0.003, 0, 0.044, 0.013, 2.6], [0.122, -0.016, 0, 0.042, 0.011, 2.6], [0.142, -0.038, 0, 0.038, 0.009, 2.4], [0.15, -0.054, 0, 0.032, 0.0078, 2.2]],
    thumb: [[0.02, -0.008, 0.026, 0.0125], [0.06, -0.026, 0.047, 0.0105], [0.098, -0.04, 0.046, 0.0085]],
    thigh: [...dome(-0.07, 0, 0, 0.01, 0.092, 0.084, 3), [0, -0.004, 0.01, 0.097, 0.088, 2.4], [0.16, -0.001, 0.005, 0.088, 0.08], [0.32, 0.005, 0, 0.069, 0.066], [0.46, 0.011, 0, 0.056, 0.06], ...dome(0.505, 0.46, 0.011, 0, 0.056, 0.06, 3).reverse()],
    shank: [...dome(-0.055, 0, 0.003, 0, 0.058, 0.064, 3), [0, 0.003, 0, 0.058, 0.064], [0.14, -0.004, 0, 0.061, 0.067], [0.33, 0, 0, 0.056, 0.062], [0.428, -0.004, 0, 0.056, 0.062]],
    shoe: [[-0.074, -0.05, 0, 0.026, 0.04, 2.8], [-0.064, -0.048, 0, 0.038, 0.044, 2.8], [-0.01, -0.041, 0, 0.044, 0.051, 2.8], [0.07, -0.06, 0, 0.049, 0.032, 2.8], [0.15, -0.0685, 0, 0.0525, 0.0235, 2.8]],
    toe: [[-0.012, 0.024, 0, 0.0525, 0.0235, 2.8], [0.045, 0.021, 0, 0.047, 0.021, 2.8], [0.074, 0.018, 0, 0.03, 0.017, 2.6], [0.082, 0.018, 0, 0.016, 0.0115, 2.2]],
  });
}

/* ---------------------------------------------------------------- 1:200 */

export function body200(k: Kit): void {
  chair(k, {
    arm: [[0, 0.004, 0, 0.026, 0.024], [-0.001, -0.002, 0, 0.018, 0.016]],
    wheel: 0.05, hood: false, shell: 0, strut: 0.03, armrests: true, mechanism: false,
    pad: [[-0.235, 0.05, 0.2, 0.02], [-0.215, 0.044, 0.235, 0.044], [0.17, 0.044, 0.243, 0.044], [0.235, 0.04, 0.21, 0.02]],
    back: [[0.13, 0.0, 0.18, 0.016], [0.16, -0.004, 0.22, 0.036], [0.43, 0.006, 0.226, 0.034], [0.598, 0.012, 0.18, 0.016]],
  });
  desk(k, 0.03);
  laptop(k, 0.03, 0.03);
  person(k, {
    seat: [[-0.098, 0.03, 0, 0.15, 0.08, 3], [-0.05, 0.03, 0, 0.196, 0.11, 2.5], [0.06, 0.003, 0, 0.18, 0.118, 2.4], [0.136, -0.024, 0, 0.162, 0.118, 2.4]],
    belt: false,
    shirt: [[-0.02, -0.05, 0, 0.144, 0.098, 2.4], [0.15, -0.054, 0, 0.156, 0.116, 2.4], [0.35, -0.032, 0, 0.18, 0.132, 2.5], [0.448, -0.012, 0, 0.198, 0.1, 2.7], [0.5, 0.004, 0, 0.12, 0.066, 2.2], [0.528, 0.004, 0, 0.074, 0.062, 2]],
    neck: [[-0.02, -0.006, 0, 0.06, 0.062], [0.125, -0.008, 0, 0.056, 0.06]],
    head: [[-0.106, -0.072, 0, 0.03, 0.026, 2.2], [-0.07, -0.042, 0, 0.058, 0.062, 2.2], [0.0, -0.01, 0, 0.074, 0.098, 2.1], [0.08, 0.002, 0, 0.073, 0.094, 2.1], [0.128, 0.008, 0, 0.024, 0.03, 2]],
    hair: [[0.0, 0.05, 0, 0.074, 0.055, 2.2], [0.06, 0.016, 0, 0.086, 0.098, 2.1], [0.115, 0.007, 0, 0.065, 0.078, 2], [0.1405, 0.008, 0, 0.02, 0.025, 2]],
    nose: false, ears: false,
    arm: [...dome(-0.05, 0, 0, 0.004, 0.056, 0.058, 2), [0, 0, 0.004, 0.056, 0.058], [0.33, 0, 0, 0.043, 0.043], ...dome(0.372, 0.33, 0, 0, 0.043, 0.043, 2).reverse()],
    fore: [...dome(-0.044, 0, 0, 0, 0.044, 0.043, 2), [0, 0, 0, 0.044, 0.043], [0.24, 0, 0, 0.036, 0.031], [0.262, 0, 0, 0.03, 0.022]],
    hand: [[-0.004, 0, 0, 0.032, 0.02, 2.4], [0.09, -0.004, 0, 0.046, 0.017, 2.6], [0.14, -0.04, 0, 0.04, 0.015, 2.4]],
    thumb: null,
    thigh: [...dome(-0.07, 0, 0, 0.01, 0.092, 0.084, 2), [0, -0.004, 0.01, 0.097, 0.088, 2.4], [0.25, 0.0, 0.004, 0.08, 0.074], [0.46, 0.011, 0, 0.057, 0.061], ...dome(0.505, 0.46, 0.011, 0, 0.057, 0.061, 2).reverse()],
    shank: [...dome(-0.055, 0, 0.003, 0, 0.058, 0.064, 2), [0, 0.003, 0, 0.058, 0.064], [0.18, -0.004, 0, 0.06, 0.066], [0.428, -0.004, 0, 0.056, 0.062]],
    shoe: [[-0.074, -0.05, 0, 0.03, 0.042, 2.8], [-0.01, -0.041, 0, 0.045, 0.051, 2.8], [0.15, -0.066, 0, 0.0525, 0.026, 2.8], [0.215, -0.07, 0, 0.042, 0.022, 2.8], [0.228, -0.071, 0, 0.022, 0.016, 2.4]],
    toe: null,
  });
}

/* ---------------------------------------------------------------- 1:500 */

export function body500(k: Kit): void {
  k.rev("chair-hub", "chair-base", "pa-plastic", TONE.plastic, [[-0.04, 0], [-0.04, 0.045], [0.04, 0.045], [0.04, 0]], [0, 1, 0], [0, 0, 0], "chair-plastic");
  for (const [id, yaw] of STAR) {
    const arm = `chair-arm-${id}`;
    const root = radial(yaw, 0.02, 0.085);
    const tip = radial(yaw, CHAIR.armReach, 0.06);
    const dir: V3 = [tip[0] - root[0], tip[1] - root[1], tip[2] - root[2]];
    k.loft(arm, arm, "pa-plastic", TONE.plastic, [[0, 0, 0, 0.0385, 0.0385, 3], [Math.hypot(...dir), 0, 0, 0.0385, 0.0385, 3]], "chair-plastic", { translation: local(arm, root), rotation: aim(arm, dir) });
    const castor = `chair-castor-${id}`;
    k.ell(`chair-wheel-${id}`, castor, "rubber", TONE.rubber, [0, 0.0, 0], [0.038, 0.025, 0.038], "castor-rubber");
  }
  k.loft("chair-column", "chair-column", "steel-painted", TONE.steel, [[0.02, 0, 0, 0.038, 0.038], [0.3, 0, 0, 0.038, 0.038]], "chair-steel");
  k.loft("chair-seat-cushion", "chair-seat", "fabric-upholstery", TONE.fabric, [[-0.235, 0.045, 0, 0.215, 0.038, 3], [0.235, 0.045, 0, 0.215, 0.038, 3]], "chair-fabric");
  k.loft("chair-backrest-pad", "chair-backrest", "fabric-upholstery", TONE.fabric, [[0.11, 0.0, 0, 0.215, 0.038, 3], [0.6, 0.008, 0, 0.2, 0.038, 3]], "chair-fabric");
  desk(k, 0);
  laptop(k, 0.03, 0.03);
  person(k, {
    seat: [[-0.098, 0.03, 0, 0.17, 0.09, 3], [0.03, 0.012, 0, 0.196, 0.118, 2.4], [0.136, -0.024, 0, 0.162, 0.118, 2.4]],
    belt: false,
    shirt: [[-0.02, -0.05, 0, 0.15, 0.104, 2.4], [0.3, -0.04, 0, 0.17, 0.13, 2.4], [0.45, -0.012, 0, 0.198, 0.1, 2.8], [0.528, 0.004, 0, 0.09, 0.066, 2]],
    neck: [[-0.02, -0.006, 0, 0.06, 0.062], [0.125, -0.008, 0, 0.058, 0.062]],
    head: [[-0.106, -0.06, 0, 0.0475, 0.0475, 2.2], [0.0, -0.01, 0, 0.075, 0.098, 2.1], [0.075, 0.0, 0, 0.074, 0.094, 2.1]],
    hair: [[0.03, 0.03, 0, 0.083, 0.08, 2.1], [0.09, 0.008, 0, 0.083, 0.1, 2.1], [0.1405, 0.008, 0, 0.0375, 0.0375, 2]],
    nose: false, ears: false,
    arm: [[-0.03, 0, 0.004, 0.05, 0.05], [0, 0, 0.004, 0.054, 0.056], [0.33, 0, 0, 0.0425, 0.043], [0.36, 0, 0, 0.0375, 0.0375]],
    fore: [[-0.03, 0, 0, 0.04, 0.04], [0, 0, 0, 0.044, 0.043], [0.262, 0, 0, 0.0375, 0.0375]],
    hand: [[0.0, -0.008, 0, 0.0375, 0.0375, 2.4], [0.13, -0.03, 0, 0.0375, 0.0375, 2.4]],
    thumb: null,
    thigh: [[-0.06, 0, 0.008, 0.088, 0.08], [0, -0.004, 0.01, 0.097, 0.088, 2.4], [0.46, 0.011, 0, 0.058, 0.062], [0.5, 0.011, 0, 0.0375, 0.0375]],
    shank: [[-0.04, 0.003, 0, 0.0375, 0.0375], [0, 0.003, 0, 0.058, 0.064], [0.428, -0.004, 0, 0.056, 0.062]],
    shoe: [[-0.074, -0.054, 0, 0.0375, 0.0375, 2.8], [0.0, -0.044, 0, 0.046, 0.048, 2.8], [0.21, -0.054, 0, 0.0375, 0.0375, 2.8]],
    toe: null,
  });
}
