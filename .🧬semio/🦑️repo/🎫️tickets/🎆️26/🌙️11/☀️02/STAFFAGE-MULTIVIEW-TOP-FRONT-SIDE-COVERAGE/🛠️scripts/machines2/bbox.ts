import { load } from "../machines/lib.ts";
const id = process.env.ID!, band = process.env.BAND ?? "50", re = new RegExp(process.env.RE ?? ".");
const d = load(id).asset;
const bb = (s: any) => {
  let pts: number[][] = [];
  if (s.points) pts = s.points;
  else if (s.kind === "rect") pts = [[s.x, s.y], [s.x + s.w, s.y + s.h]];
  else if (s.kind === "ellipse") pts = [[s.cx - s.rx, s.cy - s.ry], [s.cx + s.rx, s.cy + s.ry]];
  else if (s.d) { const n = (s.d.match(/-?\d*\.?\d+/g) ?? []).map(Number); for (let i = 0; i + 1 < n.length; i += 2) pts.push([n[i], n[i + 1]]); }
  if (!pts.length) return "?";
  const xs = pts.map(p => p[0]), ys = pts.map(p => p[1]);
  const f = (v: number) => v.toFixed(3);
  return `[${f(Math.min(...xs))},${f(Math.max(...xs))}] x [${f(Math.min(...ys))},${f(Math.max(...ys))}]`;
};
for (const v of ["elevation", "front", "plan"]) for (const s of d.representations[band].geometry[v]) if (re.test(s.id) || re.test(s.region ?? "")) console.log(v.padEnd(9), s.id.padEnd(22), (s.bone ?? "").padEnd(12), s.kind, s.solid ? `solid:${s.solid.kind}${s.solid.z0 !== undefined ? ` z[${s.solid.z0},${s.solid.z1}]` : ""}` : "", bb(s));
