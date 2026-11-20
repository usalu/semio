/** 🛒️ The stroller at 1:50, stated by hand in the stroller-bone frame (u ahead, v up, origin on the push-bar axis) and centred on w = 0: central-joint aluminium chassis (U push bar with telescopic sleeves, joint hubs, rear legs on a through axle, front legs with cross tube, swivel housings and caster forks), seat frame with padded sling, side wings, footrest and bumper bar, two-part canopy on three bows, underseat basket, moulded spoked wheels with foam tyres.
 * Real reference: a central-joint all-terrain pushchair of the Bugaboo Cameleon class (rear wheels 0.30 m, swivel front wheels 0.15 m, width 0.60 m, push bar at 1.0 m).
 * @see ./r3-stroller-kit.ts */
import { B, C, E, R, T, X, mirror, qaxis, type Json, type V3 } from "./r3-stroller-kit.ts";

export const HINGE: [number, number] = [0.19, -0.225];
export const J: [number, number] = [0.36, -0.5];
const ALU = 0.8, DARK = 0.3, FAB = 0.45;
const tag = (s: Json, r: string): Json => ({ ...s, _r: r });
/** 🎗️ Closed outline of a strip of thickness `t` along a stated centre line (mitred corners). */
export function strip(pts: [number, number][], t: number): number[][] {
  const n = pts.length, off = (sign: number): number[][] => pts.map((p, i) => {
    const a = pts[Math.max(0, i - 1)]!, b = pts[Math.min(n - 1, i + 1)]!;
    const d1 = i > 0 ? [p[0] - a[0], p[1] - a[1]] : [b[0] - p[0], b[1] - p[1]], d2 = i < n - 1 ? [b[0] - p[0], b[1] - p[1]] : d1;
    const u1 = Math.hypot(d1[0]!, d1[1]!), u2 = Math.hypot(d2[0]!, d2[1]!);
    const nx = -(d1[1]! / u1 + d2[1]! / u2), ny = d1[0]! / u1 + d2[0]! / u2, l = Math.hypot(nx, ny), c = Math.max(0.5, (nx / l) * (-d1[1]! / u1) + (ny / l) * (d1[0]! / u1));
    return [p[0] + (sign * t * nx) / (2 * l * c), p[1] + (sign * t * ny) / (2 * l * c)];
  });
  return [...off(1), ...off(-1).reverse()];
}
const canopyLoop = (): number[][] => { const o = [[-0.195, 0.03], [-0.195, 0.2], [-0.17, 0.262], [-0.12, 0.284], [-0.06, 0.29], [0.06, 0.29], [0.12, 0.284], [0.17, 0.262], [0.195, 0.2], [0.195, 0.03]], i = [[0.187, 0.03], [0.187, 0.198], [0.164, 0.255], [0.117, 0.276], [0.06, 0.282], [-0.06, 0.282], [-0.117, 0.276], [-0.164, 0.255], [-0.187, 0.198], [-0.187, 0.03]]; return [...o, ...i, o[0]!]; };
const bow = (id: string, bone: string, c: [number, number], deg: number): Json => { const a = (deg * Math.PI) / 180, p = (rho: number, z: number): V3 => [c[0] + rho * Math.cos(a), c[1] + rho * Math.sin(a), z]; const half: V3[] = [p(0.01, 0.195), p(0.2, 0.195), p(0.258, 0.172), p(0.282, 0.12), p(0.288, 0.06)]; return T(id, bone, "aluminium", DARK, [...half, ...half.map((q) => [q[0], q[1], -q[2]] as V3).reverse()], 0.006); };

export function pram50(): { stroller: Json[]; hood: Json[]; rear: Json[]; front: Json[] } {
  const s: Json[] = [], hood: Json[] = [], rear: Json[] = [], front: Json[] = [];
  const pair = (list: Json[], x: Json, r: string): void => { list.push(tag(x, r)); list.push(tag(mirror(x, x.id.replace(/-l(-|$)/, "-r$1"), x.bone), r)); };
  const side: V3[] = [[J[0], J[1], 0.245], [0.2, -0.278, 0.245], [0.035, -0.049, 0.245], [0.012, -0.017, 0.242], [0.003, -0.004, 0.232], [0, 0, 0.21]];
  s.push(tag(T("push-bar", "stroller", "aluminium", ALU, [...side, ...side.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.012), "stroller-frame"));
  s.push(tag(C("handle-grip", "stroller", "rubber", 0.25, [0, 0, -0.198], [0, 0, 0.198], 0.017), "stroller-frame"));
  pair(s, C("handle-sleeve-l", "stroller", "aluminium", ALU - 0.08, [J[0], J[1], 0.245], [0.167, -0.232, 0.245], 0.015), "stroller-frame");
  pair(s, C("handle-clamp-l", "stroller", "rubber", DARK, [0.172, -0.239, 0.245], [0.158, -0.22, 0.245], 0.02), "stroller-frame");
  pair(s, R("joint-hub-l", "stroller", "rubber", DARK, [[-0.02, 0], [-0.02, 0.036], [-0.014, 0.042], [0.014, 0.042], [0.02, 0.036], [0.02, 0]], [0, 0, 1], [J[0], J[1], 0.245]), "stroller-frame");
  pair(s, T("rear-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1], 0.245], [0.32, -0.67, 0.245], [0.28, -0.8408, 0.245]], 0.0125), "stroller-frame");
  s.push(tag(T("rear-axle", "stroller", "aluminium", ALU - 0.1, [[0.28, -0.8408, -0.262], [0.28, -0.8408, 0.262]], 0.011), "stroller-frame"));
  pair(s, T("front-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1], 0.245], [0.6, -0.635, 0.232], [0.8, -0.745, 0.215]], 0.0125), "stroller-frame");
  s.push(tag(T("front-cross", "stroller", "aluminium", ALU - 0.1, [[0.8, -0.745, -0.215], [0.8, -0.745, 0.215]], 0.011), "stroller-frame"));
  pair(s, C("swivel-l", "stroller", "rubber", DARK, [0.823, -0.738, 0.215], [0.83, -0.8, 0.215], 0.019), "stroller-frame");
  pair(s, B("fork-crown-l", "stroller", "rubber", DARK, [0.826, -0.818, 0.215], [0.034, 0.014, 0.068], 0.005), "stroller-frame");
  for (const [k, dz] of [["a", 0.028], ["b", -0.028]] as const) pair(s, C(`fork-prong-${k}-l`, "stroller", "rubber", DARK, [0.826, -0.82, 0.215 + dz], [0.8, -0.9204, 0.215 + dz], 0.0055), "stroller-frame");

  const frame: V3[] = [[0.146, -0.062, 0.185], [0.236, -0.5, 0.185], [0.243, -0.528, 0.185], [0.262, -0.538, 0.185], [0.44, -0.527, 0.185], [0.462, -0.535, 0.185], [0.548, -0.694, 0.185]];
  pair(s, T("seat-rail-l", "stroller", "aluminium", ALU, frame, 0.009), "stroller-frame");
  const top: V3[] = [[0.156, -0.11, 0.185], [0.148, -0.072, 0.181], [0.144, -0.055, 0.165], [0.143, -0.05, 0.13]];
  s.push(tag(T("seat-top-bow", "stroller", "aluminium", ALU, [...top, ...top.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.009), "stroller-frame"));
  pair(s, C("seat-mount-l", "stroller", "rubber", DARK, [0.36, -0.53, 0.187], [J[0], J[1], 0.226], 0.012), "stroller-frame");
  s.push(tag(B("footrest", "stroller", "rubber", DARK, [0.566, -0.7, 0], [0.075, 0.012, 0.37], 0.004), "stroller-frame"));
  const bb: V3[] = [[0.33, -0.416, 0.187], [0.4, -0.376, 0.187], [0.452, -0.362, 0.172], [0.474, -0.36, 0.14], [0.478, -0.36, 0.09]];
  s.push(tag(T("bumper-bar", "stroller", "rubber", 0.25, [...bb, ...bb.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.011), "stroller-frame"));
  s.push(tag(X("seat-sling", "stroller", "stroller-textile", 0.55, "xy", strip([[0.15, -0.07], [0.246, -0.538], [0.455, -0.525], [0.54, -0.685]], 0.012), -0.178, 0.178), "stroller-fabric"));
  pair(s, X("seat-wing-l", "stroller", "stroller-textile", 0.5, "xy", [[0.236, -0.52], [0.196, -0.3], [0.33, -0.4], [0.43, -0.522]], 0.172, 0.18), "stroller-fabric");

  s.push(tag(B("basket-floor", "stroller", "stroller-textile", FAB, [0.525, -0.826, 0], [0.428, 0.008, 0.388]), "stroller-fabric"));
  s.push(tag(B("basket-front", "stroller", "stroller-textile", FAB, [0.735, -0.765, 0], [0.008, 0.13, 0.388]), "stroller-fabric"));
  s.push(tag(B("basket-back", "stroller", "stroller-textile", FAB, [0.315, -0.75, 0], [0.008, 0.16, 0.388]), "stroller-fabric"));
  pair(s, X("basket-side-l", "stroller", "stroller-textile", FAB, "xy", [[0.311, -0.83], [0.739, -0.83], [0.739, -0.7], [0.311, -0.67]], 0.186, 0.194), "stroller-fabric");
  pair(s, C("basket-strap-a-l", "stroller", "stroller-textile", DARK, [0.315, -0.672, 0.19], [0.318, -0.672, 0.245], 0.005), "stroller-fabric");
  pair(s, C("basket-strap-b-l", "stroller", "stroller-textile", DARK, [0.725, -0.702, 0.19], [0.722, -0.702, 0.224], 0.005), "stroller-fabric");

  const fan = (id: string, bone: string, c: [number, number], from: number): Json => ({ ...R(id, bone, "stroller-textile", 0.4, canopyLoop(), [0, 0, 1], [0, 0, 0]), sweepRad: (40 * Math.PI) / 180, transform: { rotation: qaxis([0, 0, 1], from).map((x) => Math.round(x * 1e5) / 1e5), translation: [c[0], c[1], 0] } });
  s.push(tag(fan("canopy-rear", "stroller", HINGE, 65), "stroller-fabric"));
  s.push(tag(bow("canopy-bow-rear", "stroller", HINGE, 105), "stroller-frame"));
  s.push(tag(bow("canopy-bow-mid", "stroller", HINGE, 65), "stroller-frame"));
  pair(s, R("canopy-hinge-l", "stroller", "rubber", DARK, [[-0.008, 0], [-0.008, 0.018], [0.008, 0.018], [0.008, 0]], [0, 0, 1], [HINGE[0], HINGE[1], 0.195]), "stroller-frame");
  hood.push(tag(fan("canopy-front", "stroller-hood", [0, 0], 25), "stroller-fabric"));
  hood.push(tag(bow("canopy-bow-front", "stroller-hood", [0, 0], 25), "stroller-frame"));

  const wheel = (list: Json[], bone: string, n: string, z: number, tyre: number[][], rim: number[][], hub: number[][], spoke: [number, number, number, number], count: number): void => {
    list.push(tag(R(`tyre-${n}`, bone, "rubber", 0.12, [...tyre, tyre[0]!], [0, 0, 1], [0, 0, z]), "tyres"));
    list.push(tag(R(`rim-${n}`, bone, "rubber", 0.7, [...rim, rim[0]!], [0, 0, 1], [0, 0, z]), "tyres"));
    list.push(tag(R(`hub-${n}`, bone, "rubber", 0.5, hub, [0, 0, 1], [0, 0, z]), "tyres"));
    list.push(tag({ ...B(`spokes-${n}`, bone, "rubber", 0.6, [spoke[3], 0, z], [spoke[0], spoke[1], spoke[2]]), repeat: { count, rotationAxis: [0, 0, 1], rotationRad: Math.round(((2 * Math.PI) / count) * 1e6) / 1e6, origin: [0, 0, z] } }, "tyres"));
  };
  for (const [n, z] of [["l", 0.277], ["r", -0.277]] as const) wheel(rear, "wheel-rear", `rear-${n}`, z, [[-0.022, 0.128], [-0.022, 0.15], [-0.014, 0.158], [0, 0.1592], [0.014, 0.158], [0.022, 0.15], [0.022, 0.128]], [[-0.016, 0.112], [-0.016, 0.13], [0.016, 0.13], [0.016, 0.112]], [[-0.028, 0], [-0.028, 0.024], [0.028, 0.024], [0.028, 0]], [0.094, 0.016, 0.01, 0.068], 6);
  for (const [n, z] of [["l", 0.215], ["r", -0.215]] as const) wheel(front, "wheel-front", `front-${n}`, z, [[-0.016, 0.056], [-0.016, 0.073], [-0.009, 0.079], [0, 0.0796], [0.009, 0.079], [0.016, 0.073], [0.016, 0.056]], [[-0.012, 0.046], [-0.012, 0.058], [0.012, 0.058], [0.012, 0.046]], [[-0.02, 0], [-0.02, 0.014], [0.02, 0.014], [0.02, 0]], [0.036, 0.012, 0.008, 0.03], 5);
  return { stroller: s, hood, rear, front };
}
