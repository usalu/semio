/** ✏️ Redraws the 2D drawings of `humans/courier-with-hand-truck` in the candidate document as the orthographic views of the handcrafted body of each scale (ruling E1). Trunk, head, legs and boots are outlines of their solids (bone-local in the side elevation, rest-world in front and plan); arms, forearms and gloves, which leave the sagittal plane, and every truck and carton part are `solid` shapes (revolved or extruded profiles), so the three views are one definition; wheels are stated directly. Ids, tones, regions, hatches and layers of the previous drawing are kept where a shape of the same id existed. Also sets `dimensions` to the extent of the 1:50 body in the default pose.
 * `bun r3-courier-draw2d.ts` reads and rewrites `🗑️generated/r3-courier-with-hand-truck/🔣️.json` and the author source of the volumes. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, SOURCES, localGeometry, qualityOfAsset, resolveList } from "./volumes-lib.ts";

type P = [number, number];
type V = "elevation" | "front" | "plan";
const FILE = join(GENERATED, "r3-courier-with-hand-truck", "🔣️.json");
const doc = JSON.parse(readFileSync(FILE, "utf8"));
const asset = doc.asset;
asset.volumes = JSON.parse(readFileSync(join(SOURCES, "humans", "courier-with-hand-truck.json"), "utf8")).volumes;
const q = qualityOfAsset(asset);
const stateOf = (bones: object, ground: string): any => evaluate({ ...asset, poses: [{ ...asset.poses[0], id: "x", bones, ground }] } as never, { kind: "pose", pose: "x" } as never);
const rest = stateOf({}, "authored");
const posed = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never) as any;
const r4 = (v: number): number => Math.round(v * 1e4) / 1e4;

/** 🕸️ Triangles of solids as 2D points: bone-local (u, v) for the elevation, rest-world (z, y) for the front, (x, z) for the plan. */
function triangles(solids: any[], view: V, state: any = rest): P[][] {
  const out: P[][] = [];
  for (const s of solids) {
    const g = localGeometry(s, q);
    const f = state.bones[s.bone].frame;
    const pt = (i: number): P => {
      const [a, b, c] = [g.positions[3 * i]!, g.positions[3 * i + 1]!, g.positions[3 * i + 2]!];
      if (view === "elevation") return [a, b];
      const w = (k: "x" | "y" | "z"): number => f.o[k] + a * f.u[k] + b * f.v[k] + c * f.w[k];
      return view === "front" ? [w("z"), w("y")] : [w("x"), w("z")];
    };
    for (let i = 0; i < g.indices.length; i += 3) out.push([pt(g.indices[i]!), pt(g.indices[i + 1]!), pt(g.indices[i + 2]!)]);
  }
  return out;
}

/** 🖊️ The outer outline of projected triangles: filled on a raster of `cell`, traced along the pixel edges and reduced to the points that matter within `tol`. */
function outline(tris: P[][], cell: number, tol: number): P[] {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const t of tris) for (const p of t) [x0, y0, x1, y1] = [Math.min(x0, p[0]), Math.min(y0, p[1]), Math.max(x1, p[0]), Math.max(y1, p[1])];
  const W = Math.ceil((x1 - x0) / cell) + 4;
  const H = Math.ceil((y1 - y0) / cell) + 4;
  const m = new Uint8Array(W * H);
  const gx = (x: number): number => (x - x0) / cell + 2;
  const gy = (y: number): number => (y - y0) / cell + 2;
  for (const t of tris) {
    const [a, b, c] = t.map((p) => [gx(p[0]), gy(p[1])]) as [P, P, P];
    const area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    if (Math.abs(area) < 1e-12) continue;
    for (let j = Math.max(0, Math.floor(Math.min(a[1], b[1], c[1]))); j <= Math.min(H - 1, Math.ceil(Math.max(a[1], b[1], c[1]))); j++)
      for (let i = Math.max(0, Math.floor(Math.min(a[0], b[0], c[0]))); i <= Math.min(W - 1, Math.ceil(Math.max(a[0], b[0], c[0]))); i++) {
        const [px, py] = [i + 0.5, j + 0.5];
        const w0 = ((b[0] - a[0]) * (py - a[1]) - (b[1] - a[1]) * (px - a[0])) / area;
        const w1 = ((c[0] - b[0]) * (py - b[1]) - (c[1] - b[1]) * (px - b[0])) / area;
        const w2 = ((a[0] - c[0]) * (py - c[1]) - (a[1] - c[1]) * (px - c[0])) / area;
        if (w0 >= -0.02 && w1 >= -0.02 && w2 >= -0.02) m[j * W + i] = 1;
      }
  }
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= W || j >= H ? 0 : m[j * W + i]!);
  for (let again = true; again; ) {
    again = false;
    for (let j = 0; j < H - 1; j++)
      for (let i = 0; i < W - 1; i++) {
        const [p, q, r, t] = [m[j * W + i]!, m[j * W + i + 1]!, m[(j + 1) * W + i]!, m[(j + 1) * W + i + 1]!];
        if ((p && t && !q && !r) || (q && r && !p && !t)) {
          m[j * W + i] = m[j * W + i + 1] = m[(j + 1) * W + i] = m[(j + 1) * W + i + 1] = 1;
          again = true;
        }
      }
  }
  const next = new Map<string, [number, number]>();
  for (let j = 0; j < H; j++)
    for (let i = 0; i < W; i++) {
      if (!at(i, j)) continue;
      if (!at(i, j - 1)) next.set(`${i},${j}`, [i + 1, j]);
      if (!at(i + 1, j)) next.set(`${i + 1},${j}`, [i + 1, j + 1]);
      if (!at(i, j + 1)) next.set(`${i + 1},${j + 1}`, [i, j + 1]);
      if (!at(i - 1, j)) next.set(`${i},${j + 1}`, [i, j]);
    }
  let best: P[] = [];
  const seen = new Set<string>();
  for (const start of next.keys()) {
    if (seen.has(start)) continue;
    const loop: P[] = [];
    let key = start;
    while (!seen.has(key) && next.has(key)) {
      seen.add(key);
      const [i, j] = key.split(",").map(Number) as [number, number];
      loop.push([i, j]);
      const n = next.get(key)!;
      key = `${n[0]},${n[1]}`;
    }
    if (key !== start) throw new Error(`outline is not closed at ${key}`);
    if (loop.length > best.length) best = loop;
  }
  const world = best.map(([i, j]) => [x0 + (i - 2) * cell, y0 + (j - 2) * cell] as P);
  const smooth = world.map((_, i) => {
    const [a, b, c] = [world[(i + world.length - 1) % world.length]!, world[i]!, world[(i + 1) % world.length]!];
    return [(a[0] + 2 * b[0] + c[0]) / 4, (a[1] + 2 * b[1] + c[1]) / 4] as P;
  });
  const rdp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-9;
    let [far, idx] = [0, 0];
    pts.forEach((p, i) => {
      const d = Math.abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / len;
      if (d > far) [far, idx] = [d, i];
    });
    return far <= tol ? [a, b] : [...rdp(pts.slice(0, idx + 1)).slice(0, -1), ...rdp(pts.slice(idx))];
  };
  let split = 0;
  smooth.forEach((p, i) => {
    if (Math.hypot(p[0] - smooth[0]![0], p[1] - smooth[0]![1]) > Math.hypot(smooth[split]![0] - smooth[0]![0], smooth[split]![1] - smooth[0]![1])) split = i;
  });
  return [...rdp(smooth.slice(0, split + 1)).slice(0, -1), ...rdp([...smooth.slice(split), smooth[0]!]).slice(0, -1)].map((p) => [r4(p[0]), r4(p[1])] as P);
}

const circle = (cx: number, cy: number, r: number, n = 12): P[] => Array.from({ length: n }, (_, i) => [r4(cx + r * Math.cos((2 * Math.PI * i) / n)), r4(cy + r * Math.sin((2 * Math.PI * i) / n))] as P);
const ellipsePts = (cx: number, cy: number, rx: number, ry: number, n = 16): P[] => Array.from({ length: n }, (_, i) => [r4(cx + rx * Math.cos((2 * Math.PI * i) / n)), r4(cy + ry * Math.sin((2 * Math.PI * i) / n))] as P);
const GLOVE: Record<number, P[]> = {
  50: [[-0.028, 0.026], [0.0, 0.024], [0.03, 0.026], [0.07, 0.024], [0.098, 0.018], [0.112, 0.004], [0.119, -0.02], [0.116, -0.042], [0.104, -0.062], [0.086, -0.07], [0.066, -0.068], [0.05, -0.056], [0.04, -0.036], [0.022, -0.022], [0.0, -0.024], [-0.028, -0.026]],
  100: [[-0.02, 0.025], [0.03, 0.025], [0.075, 0.022], [0.108, 0.004], [0.123, -0.03], [0.116, -0.052], [0.09, -0.07], [0.066, -0.066], [0.05, -0.04], [0.03, -0.02], [-0.02, -0.025]],
  200: ellipsePts(0.068, -0.016, 0.066, 0.042, 12),
  500: ellipsePts(0.066, -0.014, 0.066, 0.042, 8),
};

type Shape = Record<string, any>;
for (const scale of [50, 100, 200, 500]) {
  const rep = asset.representations[String(scale)];
  const old: Record<V, Map<string, Shape>> = { elevation: new Map(), front: new Map(), plan: new Map() };
  for (const v of ["elevation", "front", "plan"] as V[]) for (const s of rep.geometry[v]) old[v].set(s.id, s);
  const body: any[] = resolveList(asset.volumes[String(scale)], "body");
  const by = (pick: (id: string) => boolean): any[] => body.filter((s) => pick(s.id));
  const cell = scale <= 100 ? 0.001 : 0.002;
  const tol = { 50: 0.0012, 100: 0.0025, 200: 0.005, 500: 0.010 }[scale]!;
  const views: Record<V, Shape[]> = { elevation: [], front: [], plan: [] };
  const META: Record<string, [string, number]> = { jacket: ["jacket", 0.62], trousers: ["trousers", 0.4], boots: ["boots", 0.25], gloves: ["gloves", 0.36], cap: ["cap", 0.3], skin: ["face-skin", 0.7], truck: ["hand-truck", 0.35], tyres: ["tyres", 0.18], parcels: ["parcels", 0.52] };
  const put = (view: V, id: string, bone: string, kind: keyof typeof META, z: number, geometry: Shape, extra: Shape = {}): void => {
    const o = old[view].get(id);
    const [region, tone] = META[kind]!;
    const shape: Shape = { id, bone, role: "body", z: o?.bone === bone ? o.z : z, filled: true, tone, region, ...geometry, ...extra };
    if (o?.hatch !== undefined && shape.filled) shape.hatch = o.hatch;
    views[view].push(shape);
  };
  const poly = (points: P[]): Shape => ({ kind: "poly", points, closed: true });
  const flat = (view: V, solids: any[]): Shape => poly(outline(triangles(solids, view), cell, tol));
  const Z = { elevation: { l: 1, r: 3 }, front: { l: 1, r: 1 }, plan: { l: 1, r: 1 } };

  const torso = by((i) => i === "jacket" || i === "jacket-collar");
  for (const v of ["elevation", "front", "plan"] as V[]) {
    const seat = by((i) => i === "trousers-seat");
    if (seat.length > 0) put(v, v === "plan" ? "waist" : "pelvis", "pelvis", "trousers", v === "plan" ? 3.9 : 3, flat(v, seat));
    put(v, v === "elevation" ? "jacket" : v === "front" ? "torso" : "trunk", "torso", "jacket", v === "plan" ? 3.95 : 4, flat(v, torso));
    by((i) => i.startsWith("reflective-band")).forEach((b, n) => v !== "plan" && put(v, `reflective-band-${n}`, "torso", "jacket", 4.12, flat(v, [b]), { tone: 0.9 }));
    put(v, "neck", "neck", "skin", v === "plan" ? 5 : 3.8, flat(v, by((i) => i === "neck")));
    put(v, "head", "head", "skin", v === "elevation" ? 5 : 6, flat(v, by((i) => ["head", "chin", "brow", ...(v === "front" ? [] : ["nose"])].includes(i))));
    const hair = by((i) => i === "hair-nape");
    if (hair.length > 0 && v !== "front") put(v, "hair", "head", "cap", v === "elevation" ? 5.05 : 6.02, flat(v, hair), { tone: 0.25 });
    if (v === "front" && by((i) => i === "nose").length > 0) put(v, "nose", "head", "skin", 6.3, flat(v, by((i) => i === "nose")), { role: "detail", filled: false });
    for (const s of ["l", "r"]) {
      const ear = by((i) => i === `ear-${s}`);
      if (ear.length > 0 && !(v === "elevation" && s === "l")) put(v, v === "elevation" ? "ear" : `ear-${s}`, "head", "skin", v === "elevation" ? 5.2 : v === "front" ? 5.9 : 6.2, flat(v, ear), { role: "detail", tone: 0.66 });
      const eye = by((i) => i === `eye-${s}`);
      if (eye.length > 0 && v !== "plan" && !(v === "elevation" && s === "l")) put(v, v === "elevation" ? "eye" : `eye-${s}`, "head", "skin", v === "elevation" ? 5.3 : 6.3, flat(v, eye), { role: "detail", tone: 0.2 });
    }
    put(v, v === "plan" ? "cap" : "cap", "head", "cap", v === "elevation" ? 5.5 : v === "front" ? 6.6 : 6.05, flat(v, by((i) => i === "cap" || i === "cap-cuff")));
    const brim = by((i) => i === "cap-brim");
    if (brim.length > 0) put(v, "cap-brim", "head", "cap", v === "elevation" ? 5.4 : v === "front" ? 6.65 : 6.45, flat(v, brim), { tone: 0.22 });
    for (const s of ["l", "r"] as const) {
      const sz = s === "l" ? 0.185 : -0.185;
      const zArm = v === "elevation" ? (s === "l" ? 0.5 : 6) : v === "front" ? 4.5 : 3;
      for (const [solidId, ids, bone] of [[`sleeve-${s}`, { elevation: `sleeve-${s}`, front: `upper-arm-${s}`, plan: `upper-arm-${s}` }, `arm-${s}`], [`forearm-${s}`, { elevation: `forearm-${s}`, front: `forearm-${s}`, plan: `forearm-${s}` }, `forearm-${s}`], [`cuff-strap-${s}`, { elevation: `cuff-strap-${s}`, front: `cuff-strap-${s}`, plan: `cuff-strap-${s}` }, `forearm-${s}`]] as [string, Record<V, string>, string][]) {
        const solid = by((i) => i === solidId)[0];
        if (solid === undefined) continue;
        const sec = [...solid.sections].sort((a: any, b: any) => a.at[0] - b.at[0]);
        const profile: P[] = [[r4(sec[0].at[0]), 0], ...sec.map((x: any) => [r4(x.at[0]), r4(x.ry)] as P), [r4(sec[sec.length - 1].at[0]), 0]];
        const lateral = r4(sec.reduce((n: number, x: any) => n + x.rx / x.ry, 0) / sec.length);
        put(v, ids[v], bone, "jacket", zArm + (bone.startsWith("forearm") ? 0.05 : 0) + (solidId.startsWith("cuff") ? 0.03 : 0), poly(profile), { solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, mirror: true, lateralScale: lateral }, ...(solidId.startsWith("cuff") ? { tone: 0.3 } : {}) });
      }
      put(v, v === "elevation" ? `glove-${s}` : `hand-${s}`, `hand-${s}`, "gloves", zArm + 0.2, poly(GLOVE[scale]!), { solid: { kind: "extruded", sweep: "w", z0: r4(sz - (scale === 500 ? 0.05 : 0.048)), z1: r4(sz + (scale === 500 ? 0.05 : 0.048)) } });
      const legZ = Z[v][s];
      const thigh = by((i) => i === `thigh-${s}`);
      const shank = by((i) => i === `shank-${s}`);
      const hz = s === "l" ? 0.095 : -0.095;
      if (v === "plan") {
        put(v, `thigh-${s}`, `thigh-${s}`, "trousers", 1.05, { kind: "capsule", x0: 0, y0: r4(hz - Math.sign(hz) * 0.005), x1: 0, y1: r4(hz - Math.sign(hz) * 0.006), r0: 0.094, r1: 0.066, followTip: true });
        put(v, `shank-${s}`, `shank-${s}`, "trousers", 1.1, { kind: "capsule", x0: 0, y0: r4(hz - Math.sign(hz) * 0.005), x1: 0, y1: hz, r0: 0.064, r1: 0.058, followTip: true });
      } else {
        put(v, `thigh-${s}`, `thigh-${s}`, "trousers", legZ, flat(v, thigh));
        put(v, `shank-${s}`, `shank-${s}`, "trousers", legZ + 0.02, flat(v, shank));
        const pocket = by((i) => i === `cargo-pocket-${s}`);
        if (pocket.length > 0 && !(v === "elevation" && s === "l")) put(v, `cargo-pocket-${s}`, `thigh-${s}`, "trousers", legZ + 0.03, flat(v, pocket), { role: "detail", filled: false });
        const shaft = by((i) => i === `boot-shaft-${s}`);
        if (shaft.length > 0) put(v, `boot-shaft-${s}`, `shank-${s}`, "boots", legZ + 0.01, flat(v, shaft));
      }
      const upper = by((i) => i === `boot-upper-${s}` || i === `boot-${s}`);
      const sole = by((i) => i === `boot-sole-${s}`);
      const toe = by((i) => i === `boot-toe-${s}`);
      const toeSole = by((i) => i === `boot-toe-sole-${s}`);
      if (v === "elevation") {
        put(v, `boot-${s}`, `foot-${s}`, "boots", legZ + 0.1, flat(v, upper));
        put(v, `boot-toe-${s}`, `toe-${s}`, "boots", legZ + 0.1, flat(v, toe));
        if (sole.length > 0) put(v, `boot-sole-${s}`, `foot-${s}`, "boots", legZ + 0.15, flat(v, sole), { tone: 0.12 });
        if (toeSole.length > 0) put(v, `boot-toe-sole-${s}`, `toe-${s}`, "boots", legZ + 0.15, flat(v, toeSole), { tone: 0.12 });
      } else {
        put(v, `foot-${s}`, `foot-${s}`, "boots", 1.2, flat(v, [...upper, ...sole]));
        put(v, `toe-${s}`, `toe-${s}`, "boots", 1.22, flat(v, [...toe, ...toeSole]));
      }
    }
    const tz = v === "elevation" ? 2.5 : 6.5;
    const ext = (id: string, bone: string, kind: keyof typeof META, z: number, pts: P[], z0: number, z1: number, extra: Shape = {}): void => put(v, id, bone, kind, z, poly(pts), { solid: { kind: "extruded", sweep: "w", z0, z1 }, ...extra });
    const top = by((i) => i.startsWith("bow")).length > 0 ? 1.02 : 1.0065;
    const inner = scale === 500 ? -0.1175 : -0.1265;
    const half = scale === 500 ? 0.018 : 0.0135;
    const strip: P[] = [[-0.118, -0.1535], [top, -0.1535], [top, inner], [-0.118, inner]];
    if (v === "elevation") ext("rail", "hand-truck", "truck", tz, strip, -0.16, 0.16);
    else for (const [s, sg] of [["l", 1], ["r", -1]] as [string, number][]) ext(`rail-${s}`, "hand-truck", "truck", tz, strip, r4(sg * 0.1465 - half), r4(sg * 0.1465 + half));
    const handle = by((i) => i === "handle")[0];
    if (handle !== undefined) ext("handle", "hand-truck", "truck", tz + 0.02, circle(1.0065, -0.14, handle.r0), -0.1465, 0.1465, { tone: 0.2 });
    by((i) => i.startsWith("brace")).forEach((b) => ext(b.id, "hand-truck", "truck", tz - 0.05, circle(b.p0[0], b.p0[1], b.r0, 8), -0.1465, 0.1465, { role: "detail" }));
    by((i) => i.startsWith("strut")).forEach((b) => {
      if (v === "elevation" && b.id.endsWith("-l")) return;
      const f = rest.bones["hand-truck"].frame;
      const at = (p: number[]): P => {
        const w = (k: "x" | "y" | "z"): number => f.o[k] + p[0]! * f.u[k] + p[1]! * f.v[k] + p[2]! * f.w[k];
        return v === "elevation" ? [r4(p[0]!), r4(p[1]!)] : v === "front" ? [r4(w("z")), r4(w("y"))] : [r4(w("x")), r4(w("z"))];
      };
      put(v, v === "elevation" ? b.id.slice(0, -2) : b.id, "hand-truck", "truck", tz - 0.05, { kind: "poly", points: [at(b.p0), at(b.p1)], closed: false }, { role: "detail", filled: false });
    });
    const plate = by((i) => i === "plate")[0];
    if (plate !== undefined) ext("plate", "hand-truck", "truck", v === "elevation" ? 2.4 : 6.9, plate.profile.map((p: P) => [p[0], p[1]] as P), plate.from, plate.to);
    if (scale !== 500) ext("axle", "truck-lift", "truck", tz + 0.06, circle(0, 0, 0.01, 8), -0.289, 0.289, { role: "detail", tone: 0.2 });
    const hub = by((i) => i === "rim").length > 0;
    if (v === "elevation") {
      put(v, "tyre", "truck-lift", "tyres", 2.6, { kind: "ellipse", cx: 0, cy: 0, rx: 0.13, ry: 0.13 });
      if (hub) put(v, "hub", "truck-lift", "tyres", 2.65, { kind: "ellipse", cx: 0, cy: 0, rx: 0.059, ry: 0.059 }, { role: "detail", tone: 0.5, region: "hand-truck" });
    } else
      for (const [s, sg] of [["l", 1], ["r", -1]] as [string, number][]) {
        const wide = scale === 500 ? 0.1 : 0.085;
        const a = sg > 0 ? 0.285 - wide : -0.285;
        put(v, `tyre-${s}`, "truck-lift", "tyres", 6.6, v === "front" ? { kind: "rect", x: r4(a), y: 0, w: wide, h: 0.26, r: scale <= 100 ? 0.025 : 0.01 } : { kind: "rect", x: 0.82, y: r4(a), w: 0.26, h: wide, r: scale <= 100 ? 0.025 : 0.01 });
      }
    if (scale === 500) {
      const boxes = by((i) => i.startsWith("parcel-")).map((p) => ({ u0: p.transform.translation[0] - p.size[0] / 2, u1: p.transform.translation[0] + p.size[0] / 2, v0: p.transform.translation[1] - p.size[1] / 2, v1: p.transform.translation[1] + p.size[1] / 2, w: p.size[2] / 2 })).sort((a, b) => a.u0 - b.u0);
      const back = Math.max(...boxes.map((b) => b.v1));
      const stack: P[] = [[r4(boxes[0]!.u0), r4(back)], ...boxes.flatMap((b) => [[r4(b.u0), r4(b.v0)], [r4(b.u1), r4(b.v0)]] as P[]), [r4(boxes[boxes.length - 1]!.u1), r4(back)]];
      ext("parcel-stack", "parcels", "parcels", v === "elevation" ? 2 : 7, stack, -r4(boxes.reduce((n, b) => n + b.w, 0) / boxes.length), r4(boxes.reduce((n, b) => n + b.w, 0) / boxes.length));
    } else for (const p of by((i) => i.startsWith("parcel-"))) {
      const [u, w, c] = [p.transform.translation[0], p.transform.translation[1], p.size];
      ext(p.id, "parcels", "parcels", v === "elevation" ? 2 : 7, [[r4(u - c[0] / 2), r4(w - c[1] / 2)], [r4(u + c[0] / 2), r4(w - c[1] / 2)], [r4(u + c[0] / 2), r4(w + c[1] / 2)], [r4(u - c[0] / 2), r4(w + c[1] / 2)]], -c[2] / 2, c[2] / 2);
      if (scale === 50) ext(`tape-${p.id.slice(-1)}`, "parcels", "parcels", v === "elevation" ? 2.1 : 7.1, [[r4(u - c[0] / 2), r4(w - c[1] / 2 - 0.002)], [r4(u + c[0] / 2 + 0.002), r4(w - c[1] / 2 - 0.002)], [r4(u + c[0] / 2 + 0.002), r4(w + c[1] / 2)], [r4(u + c[0] / 2), r4(w + c[1] / 2)], [r4(u + c[0] / 2), r4(w - c[1] / 2)], [r4(u - c[0] / 2), r4(w - c[1] / 2)]], -0.025, 0.025, { role: "detail", tone: 0.3 });
    }
  }
  if (scale <= 100) {
    const line = (view: V, id: string, bone: string, kind: keyof typeof META, z: number, points: P[], role = "detail"): void => put(view, id, bone, kind, z, { kind: "poly", points, closed: false }, { role, filled: false });
    line("elevation", "jacket-zip", "torso", "jacket", 4.1, [[-0.125, -0.141], [-0.07, -0.137], [0.04, -0.122], [0.16, -0.128], [0.27, -0.14], [0.35, -0.13], [0.435, -0.092], [0.47, -0.068], [0.52, -0.071]]);
    line("front", "jacket-zip", "torso", "jacket", 4.1, [[0, 0.937], [0, 1.578]]);
    line("elevation", "collar", "torso", "jacket", 4.1, [[0.47, -0.068], [0.475, 0.0], [0.47, 0.088]]);
    line("front", "collar-line", "torso", "jacket", 4.1, [[-0.084, 1.53], [0, 1.522], [0.084, 1.53]]);
    line("elevation", "mouth", "head", "skin", 5.3, [[-0.019, -0.103], [-0.017, -0.082]], scale === 50 ? "texture" : "detail");
    line("front", "mouth", "head", "skin", 6.3, [[-0.024, 1.62], [0.024, 1.62]]);
    if (scale === 50) {
      for (const s of ["l", "r"]) [0.062, 0.084, 0.106].forEach((dx, n) => line("elevation", `boot-lace-${s}-${n}`, `foot-${s}`, "boots", (s === "l" ? 1 : 3) + 0.2, [[r4(dx * 0.915 - 0.03 * 0.404), r4(dx * 0.404 + 0.03 * 0.915)], [r4((dx + 0.012) * 0.915 - 0.006 * 0.404), r4((dx + 0.012) * 0.404 + 0.006 * 0.915)]], "texture"));
      line("elevation", "cap-seam", "head", "cap", 5.6, [[0.088, -0.114], [0.09, -0.01], [0.088, 0.096]], "texture");
    }
  }
  for (const v of ["elevation", "front", "plan"] as V[]) rep.geometry[v] = views[v].sort((a, b) => a.z - b.z);
  Object.assign(rep.decisions, JSON.parse(readFileSync(join(import.meta.dir, "r3-courier-decisions2d.json"), "utf8"))[String(scale)]);
}

let [x0, x1, y1, z1] = [Infinity, -Infinity, -Infinity, -Infinity];
for (const s of resolveList(asset.volumes["50"], "body") as any[]) {
  const g = localGeometry(s, q);
  const f = posed.bones[s.bone].frame;
  for (let i = 0; i < g.positions.length; i += 3) {
    const w = (k: "x" | "y" | "z"): number => f.o[k] + g.positions[i]! * f.u[k] + g.positions[i + 1]! * f.v[k] + g.positions[i + 2]! * f.w[k];
    [x0, x1, y1, z1] = [Math.min(x0, w("x")), Math.max(x1, w("x")), Math.max(y1, w("y")), Math.max(z1, Math.abs(w("z")))];
  }
}
asset.dimensions = { ...asset.dimensions, heightM: Math.round(y1 * 100) / 100, widthM: Math.round((x1 - x0) * 1000) / 1000, depthM: Math.round(2 * z1 * 1000) / 1000 };
console.log("dimensions", JSON.stringify(asset.dimensions), "x", x0.toFixed(3), x1.toFixed(3));
writeFileSync(FILE, JSON.stringify(doc));
