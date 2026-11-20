/** 🧺️ Small constructors of the round 3 kneeling gardener: lofts from hand-stated station rows `[s, cv, cw, rx, ry, e?]` in the bone frame (s along the bone, cv along its normal, cw lateral; rx lateral and ry normal semi-axes), bent round bars along a stated path, ellipsoids, capsules, boxes, revolves, extrudes, rib rings, the registry that remembers the thermal region of every body solid, and the nested layer shells of a region.
 * @see ./r3-kneeling-gardener-figure.ts */
export type Json = Record<string, any>;
export type Row = readonly [s: number, cv: number, cw: number, rx: number, ry: number, e?: number];
export type V3 = readonly [number, number, number];
export type P2 = readonly [number, number];

export const ROT = [0, -0.70711, 0, 0.70711] as const;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const v3 = (p: V3): number[] => [r5(p[0]), r5(p[1]), r5(p[2])];

/** 🌀️ The unit quaternion of a turn by `deg` about the lateral axis w (u towards v). */
export const aboutW = (deg: number): number[] => [0, 0, r5(Math.sin((deg * Math.PI) / 360)), r5(Math.cos((deg * Math.PI) / 360))];
/** 🌪️ The unit quaternion of a turn by `deg` about the bone axis u. */
export const aboutU = (deg: number): number[] => [r5(Math.sin((deg * Math.PI) / 360)), 0, 0, r5(Math.cos((deg * Math.PI) / 360))];
/** 🧭️ The unit quaternion of a turn by `deg` about the normal v. */
export const aboutV = (deg: number): number[] => [0, r5(Math.sin((deg * Math.PI) / 360)), 0, r5(Math.cos((deg * Math.PI) / 360))];

/** ✖️ The product of two unit quaternions `[x, y, z, w]`: first `b`, then `a`. */
export function quat(a: readonly number[], b: readonly number[]): number[] {
  const [ax, ay, az, aw] = a as [number, number, number, number];
  const [bx, by, bz, bw] = b as [number, number, number, number];
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz].map(r5);
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
  /** 🪢️ A bent bar: sections perpendicular to the path through `points` `[u, v, w, rx, ry?]`. */
  bar(id: string, bone: string, material: string, tone: number, points: readonly (readonly number[])[], region?: string, role = "body"): Json {
    const sections = points.map(([u, v, w, rx, ry]) => ({ at: v3([u!, v!, w!]), rx: r5(rx!), ry: r5(ry ?? rx!) }));
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
  /** 🏺️ A profile of `[axial, radial]` points revolved about `axis` through `origin`; `lateral` stretches the section along w. */
  rev(id: string, bone: string, material: string, tone: number, profile: readonly P2[], axis: V3, origin: V3, lateral: number, region?: string, role = "body"): Json {
    return this.add({ id, kind: "revolve", bone, material, role, tone, profile: profile.map(([a, r]) => [r5(a), r5(r)]), axis: [...axis], origin: v3(origin), ...(lateral === 1 ? {} : { lateralScale: r5(lateral) }) }, region);
  }
  /** 🧇️ A polygon on the plane (v, w) pushed along u from `from` to `to`, scaled about its centroid by `taper`. */
  ext(id: string, bone: string, material: string, tone: number, profile: readonly P2[], from: number, to: number, taper: P2 | undefined, region?: string, role = "body"): Json {
    return this.add({ id, kind: "extrude", bone, material, role, tone, profile: profile.map(([a, b]) => [r5(a), r5(b)]), plane: "yz", from: r5(from), to: r5(to), ...(taper === undefined ? {} : { taperTo: [...taper] }) }, region);
  }
  /** 🦴️ A rib hoop: a round bar of radius `bar` bent to an ellipse (half depth `d` along the bone normal, half width `w` lateral) about the bone axis at `s`, tilted by `tiltDeg` about the lateral axis. */
  ring(id: string, bone: string, material: string, tone: number, s: number, cv: number, d: number, w: number, bar: number, tiltDeg: number): Json {
    const profile = Array.from({ length: 9 }, (_, i) => [r5(bar * Math.cos((i % 8) * (Math.PI / 4))), r5(d + bar * Math.sin((i % 8) * (Math.PI / 4)))]);
    return this.add({ id, kind: "revolve", bone, material, role: "body", tone, profile, axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: v3([s, cv, 0]), rotation: aboutW(tiltDeg) } });
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
  else if (s.kind === "revolve") s.profile = s.profile.map(([a, r]: number[]) => [a, r === 0 ? 0 : d(r)]);
  else if (s.kind === "box") {
    s.size = s.size.map((x: number) => r5(Math.max(x - 2 * t, 0.001)));
    s.cornerRadius = Math.max(0, Math.min((s.cornerRadius ?? 0) - t, 0.45 * Math.min(...s.size)));
  }
  return s;
}

/** 🧅️ The thermal solids of one body solid: a reference for a region without layers or at a scale that merges them, else one shell per layer, the outermost being the body surface itself and every inner one inset by the layers outside it. */
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
