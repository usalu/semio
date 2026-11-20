/** 🧵️ Small solid-writing helpers of the round-3 author of `humans/parent-with-stroller`: every form is stated by hand as station rows or path points; the helpers only turn rows into schema solids, densify a stated row list with a monotone cubic (so a stated bulge stays where it was stated), mirror a left solid to the right and inset a loft for a layer shell.
 * Frames: bone-local u along the bone, v its sagittal normal, w lateral (left +). A loft row is `[s, rx, ry, cv, cw, e, tilt]`: station along u, lateral half extent, sagittal half extent, centre offsets, superellipse exponent, tilt of the section plane about w in degrees.
 * @see ./round3-brief.md */
export type Json = any;
export type V3 = [number, number, number];
export type Row = number[];
export type Quat = [number, number, number, number];
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const ROT: Quat = [0, -0.70711, 0, 0.70711];
export const qmul = (a: Quat, b: Quat): Quat => [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]];
export const qaxis = (axis: V3, deg: number): Quat => { const h = (deg * Math.PI) / 360, s = Math.sin(h); return [axis[0] * s, axis[1] * s, axis[2] * s, Math.cos(h)]; };
/** 🧭️ Section frame x = w, y = v, z = -u, tilted about w by `tilt` degrees and twisted about its own normal by `twist` degrees. */
export const secRot = (tilt = 0, twist = 0): Quat => { let q: Quat = ROT; if (twist !== 0) q = qmul(q, qaxis([0, 0, 1], twist)); if (tilt !== 0) q = qmul(qaxis([0, 0, 1], tilt), q); return q.map(r5) as Quat; };

/** 📈️ Monotone cubic (Fritsch-Carlson) through the stated rows, `k` extra rows per span; column 0 is the station. */
export function dense(rows: Row[], k: number): Row[] {
  if (k <= 0 || rows.length < 3) return rows;
  const n = rows.length, m = Math.max(...rows.map((r) => r.length));
  const col = (j: number): number[] => rows.map((r) => r[j] ?? (j === 5 ? 2 : 0));
  const s = col(0);
  const slopes = (y: number[]): number[] => {
    const h = s.slice(1).map((x, i) => x - s[i]!), d = y.slice(1).map((x, i) => (x - y[i]!) / h[i]!);
    const t = new Array(n).fill(0);
    t[0] = d[0]!; t[n - 1] = d[n - 2]!;
    for (let i = 1; i < n - 1; i++) if (d[i - 1]! * d[i]! > 0) { const w1 = 2 * h[i]! + h[i - 1]!, w2 = h[i]! + 2 * h[i - 1]!; t[i] = (w1 + w2) / (w1 / d[i - 1]! + w2 / d[i]!); }
    return t;
  };
  const cols = Array.from({ length: m }, (_, j) => col(j)), tans = cols.map(slopes);
  const out: Row[] = [];
  for (let i = 0; i < n - 1; i++) {
    out.push(rows[i]!);
    const h = s[i + 1]! - s[i]!;
    for (let q = 1; q <= k; q++) {
      const t = q / (k + 1), t2 = t * t, t3 = t2 * t;
      out.push(cols.map((y, j) => (j === 0 ? s[i]! + t * h : (2 * t3 - 3 * t2 + 1) * y[i]! + (t3 - 2 * t2 + t) * h * tans[j]![i]! + (-2 * t3 + 3 * t2) * y[i + 1]! + (t3 - t2) * h * tans[j]![i + 1]!)));
    }
  }
  out.push(rows[n - 1]!);
  return out;
}

/** 🥚️ Dome rows closing a loft beyond `row` over the height `h` (sign gives the direction along u): radii follow a quarter ellipse. */
export function dome(row: Row, h: number, n = 3): Row[] {
  const out: Row[] = [];
  for (let i = 1; i <= n; i++) { const a = ((i / n) * Math.PI) / 2 * (i === n ? 0.93 : 1), r = [...row]; r[0] = row[0]! + h * Math.sin(a); r[1] = row[1]! * Math.cos(a); r[2] = row[2]! * Math.cos(a); out.push(r); }
  return h < 0 ? out.reverse() : out;
}
export const capped = (rows: Row[], h0: number, h1: number, n = 3): Row[] => [...(h0 > 0 ? dome(rows[0]!, -h0, n) : []), ...rows, ...(h1 > 0 ? dome(rows[rows.length - 1]!, h1, n) : [])];

export type Opt = { role?: string; k?: number; e?: number; extra?: Json };
const base = (id: string, kind: string, bone: string, material: string, tone: number, o: Opt = {}): Json => ({ id, kind, bone, material, role: o.role ?? "body", tone });
/** 🧶️ Loft from stated rows along u. */
export function L(id: string, bone: string, material: string, tone: number, rows: Row[], o: Opt = {}): Json {
  const rr = dense(rows, o.k ?? 0);
  return { ...base(id, "loft", bone, material, tone, o), sections: rr.map((r) => ({ at: [r5(r[0]!), r5(r[3] ?? 0), r5(r[4] ?? 0)], rx: r5(Math.max(r[1]!, 0.0006)), ry: r5(Math.max(r[2]!, 0.0006)), exponent: r5(r[5] ?? o.e ?? 2), rotation: secRot(r[6] ?? 0) })), capStart: true, capEnd: true, ...(o.extra ?? {}) };
}
/** 🪈️ Round tube through stated path points (sections perpendicular to the path). */
export function T(id: string, bone: string, material: string, tone: number, pts: V3[], r: number | number[], o: Opt = {}): Json {
  return { ...base(id, "loft", bone, material, tone, o), sections: pts.map((p, i) => ({ at: p.map(r5), rx: r5(typeof r === "number" ? r : r[i]!), ry: r5(typeof r === "number" ? r : r[i]!), exponent: 2 })), capStart: true, capEnd: true, ...(o.extra ?? {}) };
}
/** ➰️ Path loft with a flat section: the path lies in the (u, v) plane at the lateral position of each point, `rw` lateral half extent, `rt` half thickness in the plane; rows `[u, v, w, rw, rt, e]`. */
export function P(id: string, bone: string, material: string, tone: number, rows: Row[], o: Opt = {}): Json {
  const n = rows.length;
  return { ...base(id, "loft", bone, material, tone, o), sections: rows.map((r, i) => { const a = rows[Math.max(0, i - 1)]!, b = rows[Math.min(n - 1, i + 1)]!; const phi = (Math.atan2(b[1]! - a[1]!, b[0]! - a[0]!) * 180) / Math.PI; return { at: [r5(r[0]!), r5(r[1]!), r5(r[2]!)], rx: r5(r[3]!), ry: r5(r[4]!), exponent: r5(r[5] ?? o.e ?? 2), rotation: secRot(phi) }; }), capStart: true, capEnd: true, ...(o.extra ?? {}) };
}
export const C = (id: string, bone: string, material: string, tone: number, p0: V3, p1: V3, r0: number, r1 = r0, o: Opt = {}): Json => ({ ...base(id, "capsule", bone, material, tone, o), p0: p0.map(r5), p1: p1.map(r5), r0: r5(r0), r1: r5(r1), ...(o.extra ?? {}) });
export const E = (id: string, bone: string, material: string, tone: number, at: V3, radii: V3, o: Opt & { rot?: Quat } = {}): Json => ({ ...base(id, "ellipsoid", bone, material, tone, o), radii: radii.map(r5), transform: { translation: at.map(r5), ...(o.rot ? { rotation: o.rot.map(r5) } : {}) }, ...(o.extra ?? {}) });
export const B = (id: string, bone: string, material: string, tone: number, at: V3, size: V3, corner = 0, o: Opt & { rot?: Quat } = {}): Json => ({ ...base(id, "box", bone, material, tone, o), size: size.map(r5), ...(corner > 0 ? { cornerRadius: r5(corner) } : {}), transform: { translation: at.map(r5), ...(o.rot ? { rotation: o.rot.map(r5) } : {}) }, ...(o.extra ?? {}) });
/** 🌀️ Revolve of a profile `[axial, radial]` about `axis` through `origin`; a closed loop repeats its first point. */
export const R = (id: string, bone: string, material: string, tone: number, profile: number[][], axis: V3, origin: V3, o: Opt = {}): Json => ({ ...base(id, "revolve", bone, material, tone, o), profile: profile.map((p) => p.map(r5)), axis, origin: origin.map(r5), ...(o.extra ?? {}) });
/** 🧱️ Extrude: plane xy = profile (u, v) pushed along w; yz = (v, w) along u; zx = (w, u) along v. */
export const X = (id: string, bone: string, material: string, tone: number, plane: "xy" | "yz" | "zx", profile: number[][], from: number, to: number, o: Opt = {}): Json => ({ ...base(id, "extrude", bone, material, tone, o), profile: profile.map((p) => p.map(r5)), plane, from: r5(from), to: r5(to), ...(o.extra ?? {}) });

/** 🪞️ The solid mirrored in the plane w = `w0` (left to right), with a new id and bone. */
export function mirror(s: Json, id: string, bone: string, w0 = 0): Json {
  const n = JSON.parse(JSON.stringify(s));
  n.id = id; n.bone = bone;
  const mz = (p: number[]): void => { p[2] = r5(2 * w0 - p[2]!); };
  const mq = (q: number[]): number[] => [-q[0]!, -q[1]!, q[2]!, q[3]!].map(r5);
  if (n.sections) for (const q of n.sections) { mz(q.at); if (q.rotation) q.rotation = mq(q.rotation); }
  if (n.p0) { mz(n.p0); mz(n.p1); }
  if (n.kind === "revolve") { mz(n.origin); }
  if (n.kind === "extrude") { if (n.plane === "xy") { const f = n.from; n.from = r5(2 * w0 - n.to); n.to = r5(2 * w0 - f); } else if (n.plane === "yz") n.profile = n.profile.map((p: number[]) => [p[0], r5(2 * w0 - p[1]!)]).reverse(); else n.profile = n.profile.map((p: number[]) => [r5(2 * w0 - p[0]!), p[1]]).reverse(); }
  if (n.transform?.translation) mz(n.transform.translation); else if (n.kind === "box" || n.kind === "ellipsoid") n.transform = { translation: [0, 0, r5(2 * w0)] };
  if (n.transform?.rotation) n.transform.rotation = mq(n.transform.rotation);
  if (n.repeat?.origin) mz(n.repeat.origin);
  if (n.repeat?.translation) n.repeat.translation[2] = -n.repeat.translation[2];
  return n;
}
/** ↔️ Moves a solid laterally by `dw` (the stroller hangs from the right hand: its bones sit at z = -0.1925). */
export function shiftW(s: Json, dw: number): Json {
  const add = (p: number[]): void => { p[2] = r5(p[2]! + dw); };
  if (s.sections) for (const q of s.sections) add(q.at);
  if (s.p0) { add(s.p0); add(s.p1); }
  if (s.kind === "revolve") add(s.origin);
  if (s.kind === "extrude") { if (s.plane === "xy") { s.from = r5(s.from + dw); s.to = r5(s.to + dw); } else if (s.plane === "yz") s.profile = s.profile.map((p: number[]) => [p[0], r5(p[1]! + dw)]); else s.profile = s.profile.map((p: number[]) => [r5(p[0]! + dw), p[1]]); }
  if (s.kind === "box" || s.kind === "ellipsoid") { s.transform ??= { translation: [0, 0, 0] }; s.transform.translation ??= [0, 0, 0]; add(s.transform.translation); }
  if (s.repeat) { s.repeat.origin ??= [0, 0, 0]; add(s.repeat.origin); }
  return s;
}
/** 🧅️ A loft inset by `t` on both section axes (a layer shell under a garment); thin sections keep 45 % of their radius. */
export function inset(s: Json, t: number, id: string): Json {
  const n = JSON.parse(JSON.stringify(s));
  n.id = id;
  for (const q of n.sections) for (const k of ["rx", "ry"]) q[k] = r5(q[k] > 2.2 * t ? q[k] - t : Math.max(q[k] * 0.45, 0.0006));
  return n;
}
/** 🔵️ Points of a circular arc in the (u, v) plane at lateral position `w`: centre, radius, from and to angle in degrees. */
export const arc = (c: [number, number], r: number, a0: number, a1: number, n: number, w = 0): V3[] => Array.from({ length: n + 1 }, (_, i) => { const a = ((a0 + ((a1 - a0) * i) / n) * Math.PI) / 180; return [c[0] + r * Math.cos(a), c[1] + r * Math.sin(a), w] as V3; });
export const th = (id: string, of: string, region: string, layer?: string, extra: Json = {}): Json => ({ id, of, region, ...(layer ? { layer } : {}), ...extra });
export const st = (s: Json, member: string, part: string, extra: Json = {}): Json => ({ ...s, member, part, ...extra });
