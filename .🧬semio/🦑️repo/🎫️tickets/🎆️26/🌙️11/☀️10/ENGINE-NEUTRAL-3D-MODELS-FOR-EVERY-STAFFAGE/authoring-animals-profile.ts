import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, frames, loadAsset, resolveAsset, restState, toLocal, type P2 } from "./volume-lib.ts";

const slug = process.argv[2]!;
const asset = loadAsset(resolveAsset(slug).path);
const f = frames(asset);
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
const out: string[] = [];
const st = restState(asset, 50);

function cutV(ring: P2[], bone: string, u: number): [number, number] | null {
  const fr = f[bone]!;
  const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
  let lo = Infinity, hi = -Infinity;
  for (let i = 0; i < loc.length; i++) {
    const a = loc[i]!, b = loc[(i + 1) % loc.length]!;
    if ((a[0] - u) * (b[0] - u) <= 0 && a[0] !== b[0]) {
      const t = (u - a[0]) / (b[0] - a[0]);
      const v = a[1] + t * (b[1] - a[1]);
      lo = Math.min(lo, v); hi = Math.max(hi, v);
    }
  }
  return lo === Infinity ? null : [lo, hi];
}

out.push(`# ${asset.id} dims ${JSON.stringify(asset.dimensions)}`);
out.push("## elevation, bone-local: id bone urange | u:vmin..vmax stations");
for (const e of st.elevation) {
  const sh = e.shape;
  if (sh.role === "texture" || sh.role === "ground") continue;
  const fr = f[sh.bone]!;
  const pts = e.rings.flat();
  if (!pts.length) continue;
  const loc = pts.map((p) => toLocal(fr, [p.x, p.y, 0]));
  const u0 = Math.min(...loc.map((p) => p[0])), u1 = Math.max(...loc.map((p) => p[0]));
  const vmin = Math.min(...loc.map((p) => p[1])), vmax = Math.max(...loc.map((p) => p[1]));
  let line = `${sh.id} | ${sh.bone} | u ${r(u0)}..${r(u1)} v ${r(vmin)}..${r(vmax)} ${e.closed ? "" : "OPEN "}|`;
  if (e.closed) for (let k = 0; k <= 6; k++) {
    const u = u0 + ((u1 - u0) * k) / 6;
    const c = cutV(e.rings[0]!, sh.bone, Math.min(u1 - 1e-6, Math.max(u0 + 1e-6, u)));
    line += ` ${r(u)}:${c ? r(c[0]) + ".." + r(c[1]) : "-"}`;
  } else line += " " + loc.map((p) => `(${r(p[0])},${r(p[1])})`).join(" ");
  out.push(line);
}
for (const view of ["front", "plan"] as const) {
  out.push(`## ${view}: id bone | bone-local (u,v,w) of bbox via world | world x/z or y/z`);
  for (const e of st[view]) {
    const sh = e.shape;
    if (sh.role === "texture" || sh.role === "ground") continue;
    const pts = e.rings.flat();
    if (!pts.length) continue;
    const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
    const a = view === "plan" ? "x" : "z", b = view === "plan" ? "z" : "y";
    out.push(`${sh.id} | ${sh.bone} | ${view === "plan" ? `x ${r(Math.min(...xs))}..${r(Math.max(...xs))} z ${r(Math.min(...ys))}..${r(Math.max(...ys))}` : `z ${r(Math.min(...xs))}..${r(Math.max(...xs))} y ${r(Math.min(...ys))}..${r(Math.max(...ys))}`}`);
  }
}
writeFileSync(join(GENERATED, "authoring-animals", `${asset.id}.profile.txt`), out.join("\n") + "\n");
