/** 🧒️ Small constructors of the round 3 running child: lofts from hand-stated station rows `[s, cv, cw, rx, ry, e?]` in the bone frame (s along the bone, cv along its normal, cw lateral; rx lateral and ry normal semi-axes), lofts whose sections stand upright in a tilted bone frame (the shoe on the sloping foot bone), bands swept along a hand-stated path (straps, spine), ellipsoids, capsules, boxes, rib rings, the registry that remembers the thermal region of every body solid, and the nested layer shells of a region.
 * @see ./r3-running-child-figure.ts */
export type Json = Record<string, any>;
export type Row = readonly [s: number, cv: number, cw: number, rx: number, ry: number, e?: number];
export type V3 = readonly [number, number, number];

export const ROT = [0, -0.70711, 0, 0.70711] as const;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const v3 = (p: V3): number[] => [r5(p[0]), r5(p[1]), r5(p[2])];
/** 🧭️ The section rotation whose plane normal is the bone axis turned by `deg` about the lateral axis (towards the bone normal), with the section x lateral. */
export function quat(deg: number): number[] {
  const h = (deg * Math.PI) / 360;
  const [zs, zc] = [Math.sin(h), Math.cos(h)];
  const [x, y, z, w] = ROT;
  return [zc * x - zs * y, zc * y + zs * x, zc * z + zs * w, zc * w - zs * z].map((q) => Math.round(q * 1e6) / 1e6);
}

/** 🗂️ One list under construction: the solids in order and the thermal region of each. */
export class Kit {
  readonly solids: Json[] = [];
  readonly region = new Map<string, string>();
  private add(solid: Json, region?: string): Json {
    this.solids.push(solid);
    if (region !== undefined) this.region.set(solid.id, region);
    return solid;
  }
  loft(id: string, bone: string, material: string, tone: number, rows: readonly Row[], region?: string, role = "body", e = 2): Json {
    const sections = rows.map(([s, cv, cw, rx, ry, ex]) => ({ at: v3([s, cv, cw]), rx: r5(rx), ry: r5(ry), exponent: ex ?? e, rotation: [...ROT] }));
    return this.add({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true }, region);
  }
  ell(id: string, bone: string, material: string, tone: number, at: V3, radii: V3, region?: string, role = "body", rotation?: readonly number[]): Json {
    return this.add({ id, kind: "ellipsoid", bone, material, role, tone, radii: v3(radii), transform: { translation: v3(at), ...(rotation === undefined ? {} : { rotation: [...rotation] }) } }, region);
  }
  cap(id: string, bone: string, material: string, tone: number, p0: V3, p1: V3, r0: number, r1: number, region?: string, role = "body"): Json {
    return this.add({ id, kind: "capsule", bone, material, role, tone, p0: v3(p0), p1: v3(p1), r0: r5(r0), r1: r5(r1) }, region);
  }
  box(id: string, bone: string, material: string, tone: number, at: V3, size: V3, corner: number, region?: string, role = "body"): Json {
    return this.add({ id, kind: "box", bone, material, role, tone, size: v3(size), cornerRadius: corner, transform: { translation: v3(at) } }, region);
  }
  /** 👟️ A loft whose rows are stated in axes turned by `deg` about the lateral axis against the bone (x ahead, y up for a foot bone that slopes down by `deg`): `[x, y, cw, rx, ry, e?]`; every section stands perpendicular to x. */
  turned(id: string, bone: string, material: string, tone: number, deg: number, rows: readonly Row[], region?: string, role = "body", e = 2): Json {
    const a = (deg * Math.PI) / 180;
    const [c, s] = [Math.cos(a), Math.sin(a)];
    const rotation = quat(deg);
    const sections = rows.map(([x, y, cw, rx, ry, ex]) => ({ at: v3([x * c - y * s, x * s + y * c, cw]), rx: r5(rx), ry: r5(ry), exponent: ex ?? e, rotation }));
    return this.add({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true }, region);
  }
  /** 🎗️ A band swept along a path of `[u, v, w]` points: every section stands perpendicular to the path in the sagittal plane, `rx` is its lateral half width and `ry` its half thickness; `sizes` overrides both per point. */
  sweep(id: string, bone: string, material: string, tone: number, path: readonly V3[], rx: number, ry: number, region?: string, role = "body", e = 2, sizes?: readonly (readonly [number, number])[]): Json {
    const sections = path.map((p, i) => {
      const [a, b] = [path[Math.max(0, i - 1)]!, path[Math.min(path.length - 1, i + 1)]!];
      const deg = (Math.atan2(b[1] - a[1], b[0] - a[0]) * 180) / Math.PI;
      return { at: v3(p), rx: r5(sizes?.[i]?.[0] ?? rx), ry: r5(sizes?.[i]?.[1] ?? ry), exponent: e, rotation: quat(deg) };
    });
    return this.add({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true }, region);
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
  if (!nested || layers.length === 0) return [{ id: `th-${solid.id}`, of: solid.id, region: region.id }];
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
