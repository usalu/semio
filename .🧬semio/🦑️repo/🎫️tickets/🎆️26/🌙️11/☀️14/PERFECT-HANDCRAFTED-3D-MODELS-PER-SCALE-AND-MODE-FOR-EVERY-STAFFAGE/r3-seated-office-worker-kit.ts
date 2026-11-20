/** 🧰️ Small constructors of the round 3 seated office worker: lofts from hand-stated station rows `[s, cv, cw, rx, ry, e?]` in the bone frame (s along the bone, cv along its normal, cw lateral; rx the lateral and ry the normal semi-axis), round ends of limbs, bent round bars along a stated path, ellipsoids, capsules, boxes, revolves and extrudes, the registry that remembers the thermal region of every body solid, the nested layer shells of a region and the marks of structural parts.
 * @see ./r3-seated-office-worker-s50.ts */
export type Json = Record<string, any>;
export type Row = readonly [s: number, cv: number, cw: number, rx: number, ry: number, e?: number];
export type V3 = readonly [number, number, number];
export type P2 = readonly [number, number];

/** 🧭️ The section rotation that lays a loft section across the bone axis: rx along w, ry along v. */
export const ACROSS_U = [0, -0.70711, 0, 0.70711] as const;
/** 🧭️ The section rotation that lays a loft section across the bone normal v: rx along u, ry along w. */
export const ACROSS_V = [0.70711, 0, 0, 0.70711] as const;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const v3 = (p: V3): number[] => [r5(p[0]), r5(p[1]), r5(p[2])];

export const aboutW = (deg: number): number[] => [0, 0, r5(Math.sin((deg * Math.PI) / 360)), r5(Math.cos((deg * Math.PI) / 360))];
export const aboutU = (deg: number): number[] => [r5(Math.sin((deg * Math.PI) / 360)), 0, 0, r5(Math.cos((deg * Math.PI) / 360))];
export const aboutV = (deg: number): number[] => [0, r5(Math.sin((deg * Math.PI) / 360)), 0, r5(Math.cos((deg * Math.PI) / 360))];
/** ✖️ The product of two unit quaternions `[x, y, z, w]`: first `b`, then `a`. */
export function quat(a: readonly number[], b: readonly number[]): number[] {
  const [ax, ay, az, aw] = a as [number, number, number, number];
  const [bx, by, bz, bw] = b as [number, number, number, number];
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz].map(r5);
}

/** 🥚️ Rows that close a limb with a round end: from the full section at `s1` to a small one at `s0`, the radii following a quarter ellipse (`n` rows, the full section excluded). */
export function dome(s0: number, s1: number, cv: number, cw: number, rx: number, ry: number, n = 5, e?: number): Row[] {
  const out: Row[] = [];
  for (let i = 0; i < n; i++) {
    const t = [0.05, 0.2, 0.45, 0.7, 0.9][i + (5 - n)]!;
    const k = Math.sqrt(1 - (1 - t) ** 2);
    out.push(e === undefined ? [s0 + (s1 - s0) * t, cv, cw, rx * k, ry * k] : [s0 + (s1 - s0) * t, cv, cw, rx * k, ry * k, e]);
  }
  return out;
}
/** 🪞️ The rows of the other side: lateral offsets mirrored. */
export const mirror = (rows: readonly Row[]): Row[] => rows.map(([s, cv, cw, rx, ry, e]) => (e === undefined ? [s, cv, -cw, rx, ry] : [s, cv, -cw, rx, ry, e]));

export type Opts = { role?: string; e?: number; caps?: [boolean, boolean]; across?: readonly number[]; rotation?: readonly number[]; translation?: V3; slender?: boolean; texture?: boolean };

/** 🗂️ One list under construction: the solids in order and the thermal region of each. */
export class Kit {
  readonly solids: Json[] = [];
  readonly region = new Map<string, string>();
  private add(solid: Json, region: string | undefined, o: Opts): Json {
    if (this.solids.some((s) => s.id === solid.id)) throw new Error(`duplicate solid ${solid.id}`);
    if (o.rotation !== undefined || o.translation !== undefined) solid.transform = { ...(solid.transform ?? {}), ...(o.translation === undefined ? {} : { translation: v3(o.translation) }), ...(o.rotation === undefined ? {} : { rotation: [...o.rotation] }) };
    if (o.slender === true) solid.slender = true;
    if (o.texture === true) solid.texture = true;
    this.solids.push(solid);
    if (region !== undefined) this.region.set(solid.id, region);
    return solid;
  }
  /** 🫙️ A loft of sections across the bone axis (or across v with `across: ACROSS_V`, rows then `[s along v, cu, cw, rx along u, ry along w]`). */
  loft(id: string, bone: string, material: string, tone: number, rows: readonly Row[], region?: string, o: Opts = {}): Json {
    const across = o.across ?? ACROSS_U;
    const sections = rows.map(([s, c1, c2, rx, ry, ex]) => ({ at: across === ACROSS_V ? v3([c1, s, c2]) : v3([s, c1, c2]), rx: r5(rx), ry: r5(ry), exponent: ex ?? o.e ?? 2, rotation: [...across] }));
    return this.add({ id, kind: "loft", bone, material, role: o.role ?? "body", tone, sections, capStart: o.caps?.[0] ?? true, capEnd: o.caps?.[1] ?? true }, region, o);
  }
  /** 🪢️ A bent bar: sections perpendicular to the path through `points` `[u, v, w, rx, ry?]`. */
  bar(id: string, bone: string, material: string, tone: number, points: readonly (readonly number[])[], region?: string, o: Opts = {}): Json {
    const sections = points.map(([u, v, w, rx, ry]) => ({ at: v3([u!, v!, w!]), rx: r5(rx!), ry: r5(ry ?? rx!), ...(o.e === undefined ? {} : { exponent: o.e }) }));
    return this.add({ id, kind: "loft", bone, material, role: o.role ?? "body", tone, sections, capStart: true, capEnd: true }, region, o);
  }
  ell(id: string, bone: string, material: string, tone: number, at: V3, radii: V3, region?: string, o: Opts = {}): Json {
    return this.add({ id, kind: "ellipsoid", bone, material, role: o.role ?? "body", tone, radii: v3(radii), transform: { translation: v3(at) } }, region, { ...o, translation: undefined });
  }
  cap(id: string, bone: string, material: string, tone: number, p0: V3, p1: V3, r0: number, r1: number, region?: string, o: Opts = {}): Json {
    return this.add({ id, kind: "capsule", bone, material, role: o.role ?? "body", tone, p0: v3(p0), p1: v3(p1), r0: r5(r0), r1: r5(r1) }, region, o);
  }
  box(id: string, bone: string, material: string, tone: number, at: V3, size: V3, corner: number, region?: string, o: Opts = {}): Json {
    return this.add({ id, kind: "box", bone, material, role: o.role ?? "body", tone, size: v3(size), cornerRadius: corner, transform: { translation: v3(at) } }, region, { ...o, translation: undefined });
  }
  /** 🏺️ A profile of `[axial, radial]` points revolved about `axis` through `origin`. */
  rev(id: string, bone: string, material: string, tone: number, profile: readonly P2[], axis: V3, origin: V3, region?: string, o: Opts & { lateral?: number } = {}): Json {
    return this.add({ id, kind: "revolve", bone, material, role: o.role ?? "body", tone, profile: profile.map(([a, r]) => [r5(a), r5(r)]), axis: [...axis], origin: v3(origin), ...(o.lateral === undefined ? {} : { lateralScale: r5(o.lateral) }) }, region, o);
  }
  /** 🧇️ A polygon on a coordinate plane pushed along the third axis from `from` to `to`. */
  ext(id: string, bone: string, material: string, tone: number, plane: "xy" | "yz" | "zx", profile: readonly P2[], from: number, to: number, region?: string, o: Opts & { holes?: readonly (readonly P2[])[]; taper?: P2 } = {}): Json {
    return this.add({ id, kind: "extrude", bone, material, role: o.role ?? "body", tone, profile: profile.map(([a, b]) => [r5(a), r5(b)]), ...(o.holes === undefined ? {} : { holes: o.holes.map((h) => h.map(([a, b]) => [r5(a), r5(b)])) }), plane, from: r5(from), to: r5(to), ...(o.taper === undefined ? {} : { taperTo: [...o.taper] }) }, region, o);
  }
  /** 🦴️ A hoop: a round bar of radius `bar` bent to an ellipse (half depth `d` along v, half width `w` lateral) about the bone axis at `s`, tilted by `tiltDeg` about the lateral axis. */
  ring(id: string, bone: string, material: string, tone: number, s: number, cv: number, d: number, w: number, bar: number, tiltDeg: number, region?: string): Json {
    const profile = Array.from({ length: 9 }, (_, i) => [r5(bar * Math.cos((i % 8) * (Math.PI / 4))), r5(d + bar * Math.sin((i % 8) * (Math.PI / 4)))]);
    return this.add({ id, kind: "revolve", bone, material, role: "body", tone, profile, axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: v3([s, cv, 0]), rotation: aboutW(tiltDeg) } }, region, {});
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
  else if (s.kind === "revolve") s.profile = s.profile.map(([a, r]: number[]) => [a, r === 0 ? 0 : d(r!)]);
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
  return out.reverse();
}

/** 🏷️ Marks a list of solids as structural parts of a member bone. */
export const parts = (solids: readonly Json[], member: string, part: "member" | "joint" | "mass"): Json[] => solids.map((s) => ({ ...s, member, part }));
