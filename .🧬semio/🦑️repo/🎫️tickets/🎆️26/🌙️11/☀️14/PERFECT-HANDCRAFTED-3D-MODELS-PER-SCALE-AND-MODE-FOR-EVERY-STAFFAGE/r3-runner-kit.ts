/** 🧰️ Small constructors of the round 3 runner with blade prosthesis: lofts from hand-stated station rows `[s, cv, cw, rx, ry, e?, tiltDeg?]` in the bone frame (s along the bone, cv along its normal, cw lateral; rx lateral and ry normal semi-axes; tilt turns the section about the lateral axis), lofts and strips stated in the sagittal rest plane of an inclined bone (shoe, blade), ellipsoids, capsules, boxes, rib rings, the registry of the thermal region of every body solid, and the nested layer shells of a region.
 * @see ./r3-runner-figure.ts */
export type Json = Record<string, any>;
export type Row = readonly [s: number, cv: number, cw: number, rx: number, ry: number, e?: number, tiltDeg?: number];
export type V3 = readonly [number, number, number];
/** 📐️ The rest frame of a bone in the sagittal plane: origin (x ahead, y up) and the angle of its axis from the horizontal, in degrees. */
export type Plane = { ox: number; oy: number; deg: number };

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const v3 = (p: V3): number[] => [r5(p[0]), r5(p[1]), r5(p[2])];
const S = Math.SQRT1_2;
/** 🔄️ The section rotation: rx along the lateral axis, ry along the bone normal turned by `deg` about the lateral axis. */
const turn = (deg: number): number[] => {
  const h = (deg * Math.PI) / 360;
  return [r5(Math.sin(h) * S), r5(-Math.cos(h) * S), r5(Math.sin(h) * S), r5(Math.cos(h) * S)];
};
/** 🗺️ A point of the sagittal rest plane in the frame of a bone: `[along, normal]`. */
export const local = (f: Plane, x: number, y: number): [number, number] => {
  const a = (f.deg * Math.PI) / 180;
  const [dx, dy] = [x - f.ox, y - f.oy];
  return [dx * Math.cos(a) + dy * Math.sin(a), -dx * Math.sin(a) + dy * Math.cos(a)];
};

/** 🗂️ One list under construction: the solids in order and the thermal region of each. */
export class Kit {
  readonly solids: Json[] = [];
  readonly region = new Map<string, string>();
  /** The convex pieces (bone-local points) of every strip, for the 2D outline of the drawing. */
  readonly convex = new Map<string, V3[][]>();
  private add(solid: Json, region?: string): Json {
    this.solids.push(solid);
    if (region !== undefined) this.region.set(solid.id, region);
    return solid;
  }
  loft(id: string, bone: string, material: string, tone: number, rows: readonly Row[], region?: string, role = "body", e = 2): Json {
    const sections = rows.map(([s, cv, cw, rx, ry, ex, tilt]) => ({ at: v3([s, cv, cw]), rx: r5(rx), ry: r5(ry), exponent: ex ?? e, rotation: turn(tilt ?? 0) }));
    return this.add({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true }, region);
  }
  /** 👟️ A loft whose stations `[x, y, cw, rx, ry, e?]` are stated in the sagittal rest plane (x ahead, y up) with upright sections, written into the frame of an inclined bone. */
  loftPlane(id: string, bone: string, f: Plane, material: string, tone: number, rows: readonly Row[], region?: string, role = "body", e = 2): Json {
    const sections = rows.map(([x, y, cw, rx, ry, ex]) => ({ at: v3([...local(f, x, y), cw]), rx: r5(rx), ry: r5(ry), exponent: ex ?? e, rotation: turn(-f.deg) }));
    return this.add({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true }, region);
  }
  /** 🌀️ A leaf spring: a band of the lateral width `w0..w1` along a centre line `[x, y, thickness]` stated in the sagittal rest plane, with a round end where `round` says so; an extrusion in the frame of its bone. */
  strip(id: string, bone: string, f: Plane, material: string, tone: number, line: readonly (readonly [number, number, number])[], w0: number, w1: number, round: { start?: boolean; end?: boolean }, region?: string, role = "body"): Json {
    const n = line.length;
    const side = (sign: number): [number, number][] => line.map(([x, y, t], i) => {
      const [a, b] = [line[Math.max(0, i - 1)]!, line[Math.min(n - 1, i + 1)]!];
      const len = Math.hypot(b[0] - a[0], b[1] - a[1]);
      return [x - (sign * (b[1] - a[1]) * t) / 2 / len, y + (sign * (b[0] - a[0]) * t) / 2 / len];
    });
    const cap = (i: number, from: [number, number]): [number, number][] => {
      const [x, y, t] = line[i]!;
      const a0 = Math.atan2(from[1] - y, from[0] - x);
      return [1, 2, 3].map((k) => [x + (t / 2) * Math.cos(a0 - (k * Math.PI) / 4), y + (t / 2) * Math.sin(a0 - (k * Math.PI) / 4)]);
    };
    const [up, down] = [side(1), side(-1).reverse()];
    const ring = [...up, ...(round.end ? cap(n - 1, up[n - 1]!) : []), ...down, ...(round.start ? cap(0, down[n - 1]!) : [])];
    const profile = ring.map(([x, y]) => local(f, x, y).map(r5));
    const at = (p: [number, number], w: number): V3 => [...local(f, p[0], p[1]), w] as unknown as V3;
    const lower = side(-1);
    this.convex.set(id, [...up.slice(1).map((_, i) => [up[i]!, up[i + 1]!, lower[i + 1]!, lower[i]!].flatMap((q) => [at(q, w0), at(q, w1)])), ...(round.end ? [[up[n - 1]!, ...cap(n - 1, up[n - 1]!), lower[n - 1]!].flatMap((q) => [at(q, w0), at(q, w1)])] : []), ...(round.start ? [[lower[0]!, ...cap(0, lower[0]!), up[0]!].flatMap((q) => [at(q, w0), at(q, w1)])] : [])]);
    return this.add({ id, kind: "extrude", bone, material, role, tone, profile, plane: "xy", from: r5(w0), to: r5(w1) }, region);
  }
  ell(id: string, bone: string, material: string, tone: number, at: V3, radii: V3, region?: string, role = "body", rotation?: readonly number[]): Json {
    return this.add({ id, kind: "ellipsoid", bone, material, role, tone, radii: v3(radii), transform: { translation: v3(at), ...(rotation === undefined ? {} : { rotation: [...rotation] }) } }, region);
  }
  cap(id: string, bone: string, material: string, tone: number, p0: V3, p1: V3, r0: number, r1: number, region?: string, role = "body"): Json {
    return this.add({ id, kind: "capsule", bone, material, role, tone, p0: v3(p0), p1: v3(p1), r0: r5(r0), r1: r5(r1) }, region);
  }
  box(id: string, bone: string, material: string, tone: number, at: V3, size: V3, corner: number, region?: string, role = "body", rotation?: readonly number[]): Json {
    return this.add({ id, kind: "box", bone, material, role, tone, size: v3(size), cornerRadius: corner, transform: { translation: v3(at), ...(rotation === undefined ? {} : { rotation: [...rotation] }) } }, region);
  }
  /** 🦴️ A rib hoop: a round bar of radius `bar` bent to an ellipse (half depth `d` along the bone normal, half width `w` lateral) about the bone axis at `s`, tilted by `tiltDeg` about the lateral axis (front lower for a negative tilt on a trunk bone). */
  ring(id: string, bone: string, material: string, tone: number, s: number, cv: number, d: number, w: number, bar: number, tiltDeg: number): Json {
    const profile = Array.from({ length: 9 }, (_, i) => [r5(bar * Math.cos((i % 8) * (Math.PI / 4))), r5(d + bar * Math.sin((i % 8) * (Math.PI / 4)))]);
    const h = (tiltDeg * Math.PI) / 360;
    return this.add({ id, kind: "revolve", bone, material, role: "body", tone, profile, axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: v3([s, cv, 0]), rotation: [0, 0, r5(Math.sin(h)), r5(Math.cos(h))] } });
  }
}

/** 🫧️ A copy of a solid inset by `t` on every side (a thermal layer surface below the outer one). */
export function inset(solid: Json, t: number, id: string): Json {
  const s = JSON.parse(JSON.stringify(solid)) as Json;
  s.id = id;
  const d = (x: number): number => r5(x > 2.2 * t ? x - t : Math.max(x * 0.45, 0.0006));
  if (s.kind === "loft") for (const q of s.sections) [q.rx, q.ry] = [d(q.rx), d(q.ry)];
  else if (s.kind === "ellipsoid") s.radii = s.radii.map(d);
  else if (s.kind === "capsule") [s.r0, s.r1] = [d(s.r0), d(s.r1)];
  else if (s.kind === "box") {
    s.size = s.size.map((x: number) => r5(Math.max(x - 2 * t, 0.001)));
    s.cornerRadius = Math.max(0, Math.min((s.cornerRadius ?? 0) - t, 0.45 * Math.min(...s.size)));
  }
  return s;
}

/** 🧅️ The thermal solids of one body solid: a reference for a region without layers or at a scale that merges them, else one shell per layer, innermost first in the region, the outermost being the body surface itself. */
export function thermalOf(solid: Json, region: Json, nested: boolean): Json[] {
  const layers: Json[] = region.layers ?? [];
  if (!nested || layers.length < 2) return [{ id: `th-${solid.id}`, of: solid.id, region: region.id, ...(nested && layers.length === 1 ? { layer: layers[0].id } : {}) }];
  const out: Json[] = [];
  let cum = 0;
  for (let i = layers.length - 1; i >= 0; i--) {
    const l = layers[i]!;
    if (i === layers.length - 1) out.push({ id: `th-${solid.id}-${l.id}`, of: solid.id, region: region.id, layer: l.id });
    else out.push({ ...inset(solid, cum, `th-${solid.id}-${l.id}`), role: "body", region: region.id, layer: l.id });
    cum += l.thicknessM;
  }
  return out;
}

/** 🏷️ Marks a list of solids as structural parts of a member bone. */
export const parts = (solids: readonly Json[], member: string, part: "member" | "joint" | "mass"): Json[] => solids.map((s) => ({ ...s, member, part }));
