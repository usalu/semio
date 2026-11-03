// Coordinate helpers for the two A1 assets (scratch scripts: they compute coordinates, every shape is designed by hand in worker.mjs / sleeper.mjs).
import { readFileSync } from "node:fs";

export const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const r4 = (n) => Math.round(n * 1e4) / 1e4;
export const rad = Math.PI / 180;
export const clone = (o) => JSON.parse(JSON.stringify(o));
export const txt = (en, de) => ({ en, de });
export const load = (rel) => JSON.parse(readFileSync(`${S}/🗂️catalog/${rel}/🔣️.json`, "utf8")).asset;

export const scalePath = (d, k) => d.replace(/-?\d*\.?\d+(?:e-?\d+)?/g, (m) => String(r4(parseFloat(m) * k)));

export function scaleShape(s, k) {
  const o = clone(s);
  if (o.points) o.points = o.points.map(([x, y]) => [r4(x * k), r4(y * k)]);
  if (o.d) o.d = scalePath(o.d, k);
  for (const key of ["cx", "cy", "rx", "ry", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1"]) if (typeof o[key] === "number") o[key] = r4(o[key] * k);
  return o;
}

export function fk(bones, offsets = {}) {
  const by = new Map(bones.map((b) => [b.id, b]));
  const out = new Map();
  const walk = (id) => {
    if (out.has(id)) return out.get(id);
    const b = by.get(id);
    const o = offsets[id] ?? {};
    const P = b.parent ? walk(b.parent) : { x: 0, y: 0, a: 0 };
    const lx = b.x + (o.dx ?? 0), ly = b.y + (o.dy ?? 0);
    const c = Math.cos(P.a * rad), s = Math.sin(P.a * rad);
    const t = { x: P.x + c * lx - s * ly, y: P.y + s * lx + c * ly, a: P.a + b.angleDeg + (o.angleDeg ?? 0) };
    out.set(id, t);
    return t;
  };
  for (const b of bones) walk(b.id);
  return out;
}

export function toWorld(t, [x, y]) {
  const c = Math.cos(t.a * rad), s = Math.sin(t.a * rad);
  return [t.x + c * x - s * y, t.y + s * x + c * y];
}

export function toLocal(t, [x, y]) {
  const c = Math.cos(t.a * rad), s = Math.sin(t.a * rad);
  const dx = x - t.x, dy = y - t.y;
  return [c * dx + s * dy, -s * dx + c * dy];
}

/** Two-link planar solution: returns world angles (deg) of upper arm and forearm so that the second link ends at target; `side` picks the elbow side. */
export function ik2(shoulder, target, l1, l2, side) {
  const dx = target[0] - shoulder[0], dy = target[1] - shoulder[1];
  let d = Math.hypot(dx, dy);
  d = Math.min(d, l1 + l2 - 1e-6);
  const base = Math.atan2(dy, dx);
  const cosA = (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d);
  const A = Math.acos(Math.max(-1, Math.min(1, cosA)));
  const th1 = base + side * A;
  const ex = shoulder[0] + l1 * Math.cos(th1), ey = shoulder[1] + l1 * Math.sin(th1);
  const th2 = Math.atan2(target[1] - ey, target[0] - ex);
  return { a1: th1 / rad, a2: th2 / rad, elbow: [ex, ey] };
}

export const wrap = (a) => ((((a + 180) % 360) + 360) % 360) - 180;

/** Closed Catmull-Rom spline through points as an SVG cubic path in the same coordinates. */
export function smooth(pts, closed = true, t = 0.5) {
  const n = pts.length;
  const P = (i) => pts[closed ? (i + n) % n : Math.max(0, Math.min(n - 1, i))];
  const f = (v) => String(r4(v));
  let d = `M ${f(pts[0][0])} ${f(pts[0][1])}`;
  const segs = closed ? n : n - 1;
  for (let i = 0; i < segs; i++) {
    const p0 = P(i - 1), p1 = P(i), p2 = P(i + 1), p3 = P(i + 2);
    const c1 = [p1[0] + ((p2[0] - p0[0]) * t) / 3, p1[1] + ((p2[1] - p0[1]) * t) / 3];
    const c2 = [p2[0] - ((p3[0] - p1[0]) * t) / 3, p2[1] - ((p3[1] - p1[1]) * t) / 3];
    d += ` C ${f(c1[0])} ${f(c1[1])} ${f(c2[0])} ${f(c2[1])} ${f(p2[0])} ${f(p2[1])}`;
  }
  return closed ? d + " Z" : d;
}

/** Polygon with every corner cut by radius r and closed through a quadratic curve. */
export function rpoly(pts, r) {
  const n = pts.length;
  const f = (v) => String(r4(v));
  const cut = (a, b, rr) => {
    const L = Math.hypot(b[0] - a[0], b[1] - a[1]);
    const u = Math.min(rr, L / 2) / L;
    return [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u];
  };
  let d = "";
  for (let i = 0; i < n; i++) {
    const p = pts[i], prev = pts[(i - 1 + n) % n], next = pts[(i + 1) % n];
    const rr = Array.isArray(r) ? r[i] : r;
    const a = cut(p, prev, rr), b = cut(p, next, rr);
    d += `${i === 0 ? "M" : "L"} ${f(a[0])} ${f(a[1])} Q ${f(p[0])} ${f(p[1])} ${f(b[0])} ${f(b[1])} `;
  }
  return d + "Z";
}

export const mapPts = (t, pts) => pts.map((p) => toLocal(t, p).map(r4));

/** Maps an absolute M/L/C/Q/Z path from world coordinates into a bone frame. */
export function mapPath(t, d) {
  const tokens = d.match(/[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?/g);
  const out = [];
  let nums = [];
  const flush = () => {
    for (let i = 0; i < nums.length; i += 2) out.push(...toLocal(t, [nums[i], nums[i + 1]]).map(r4));
    nums = [];
  };
  for (const k of tokens) {
    if (/[MLCQZ]/.test(k)) {
      flush();
      out.push(k);
    } else nums.push(parseFloat(k));
  }
  flush();
  return out.join(" ").replace(/ Z/, " Z");
}

export function shape(id, bone, role, z, tone, geom, extra = {}) {
  return { id, bone, role, z, filled: extra.filled ?? role === "body", tone, ...geom, ...Object.fromEntries(Object.entries(extra).filter(([k]) => k !== "filled")) };
}
export const poly = (points, closed = true) => ({ kind: "poly", points: points.map(([x, y]) => [r4(x), r4(y)]), closed });
export const path = (d) => ({ kind: "path", d });
export const ell = (cx, cy, rx, ry, rotationDeg) => ({ kind: "ellipse", cx: r4(cx), cy: r4(cy), rx: r4(rx), ry: r4(ry), ...(rotationDeg === undefined ? {} : { rotationDeg }) });
export const cap = (x0, y0, x1, y1, r0, r1) => ({ kind: "capsule", x0: r4(x0), y0: r4(y0), x1: r4(x1), y1: r4(y1), r0: r4(r0), r1: r4(r1) });
export const rect = (x, y, w, h, rr) => ({ kind: "rect", x: r4(x), y: r4(y), w: r4(w), h: r4(h), ...(rr === undefined ? {} : { r: r4(rr) }) });
