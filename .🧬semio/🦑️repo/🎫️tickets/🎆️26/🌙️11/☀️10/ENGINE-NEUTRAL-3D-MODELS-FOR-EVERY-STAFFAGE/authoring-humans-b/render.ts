/** 🖼️ Renders a volume (rest or posed) as a four-panel PNG sheet: elevation, front, plan and an oblique view, z-buffered flat shading, authored outlines drawn in red on the planar panels at rest.
 * Usage: bun render.ts <slug> [--volume file] [--pose id] [--out file.png] [--yaw 35] [--pitch 25] [--size 520] [--ids] */
import { readFileSync, writeFileSync } from "node:fs";
import { deflateSync } from "node:zlib";
import { join } from "node:path";
import { tessellateSolid } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, QUALITY_FOR, frames, loadAsset, resolveAsset, restState, toWorld, projectWorld, type V3 } from "../volume-lib.ts";

const argv = process.argv.slice(2);
const opt = (k: string, d?: string) => { const i = argv.indexOf(k); return i < 0 ? d : argv[i + 1]; };
const slug = argv[0]!;
const asset = loadAsset(resolveAsset(slug).path);
const vol = opt("--volume") ? (JSON.parse(readFileSync(opt("--volume")!, "utf8")).volume ?? JSON.parse(readFileSync(opt("--volume")!, "utf8"))) : asset.volume;
const pose = opt("--pose");
const size = Number(opt("--size", "520"));
const yaw = Number(opt("--yaw", "35")) * Math.PI / 180, pitch = Number(opt("--pitch", "25")) * Math.PI / 180;
const out = opt("--out", join(GENERATED, "authoring-humans-b", `${asset.id}${pose ? "." + pose : ""}.png`))!;

let fr: Record<string, any> = frames(asset);
if (pose) {
  const st: any = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } } as any);
  fr = {};
  for (const [id, b] of Object.entries<any>(st.bones)) fr[id] = { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] };
}
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
const q = QUALITY_FOR(Math.max(...dims));
type Tri = { p: V3[]; id: number };
const tris: Tri[] = [];
const ids: string[] = [];
vol.solids.forEach((s: any, n: number) => {
  ids.push(s.id);
  const g = tessellateSolid({ ...s, node: s.bone } as never, q);
  const f = fr[s.bone];
  const w: V3[] = [];
  for (let i = 0; i < g.positions.length; i += 3) w.push(toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]));
  for (let t = 0; t < g.indices.length; t += 3) tris.push({ p: [w[g.indices[t]!]!, w[g.indices[t + 1]!]!, w[g.indices[t + 2]!]!], id: n });
});
const palette = (n: number): [number, number, number] => { const h = (n * 137.508) % 360; const c = 0.55, x = c * (1 - Math.abs(((h / 60) % 2) - 1)); const [r, g, b] = h < 60 ? [c, x, 0] : h < 120 ? [x, c, 0] : h < 180 ? [0, c, x] : h < 240 ? [0, x, c] : h < 300 ? [x, 0, c] : [c, 0, x]; return [r + 0.35, g + 0.35, b + 0.35]; };

type Proj = (p: V3) => [number, number, number];
const cy = Math.cos(yaw), sy = Math.sin(yaw), cp = Math.cos(pitch), sp = Math.sin(pitch);
const views: { name: string; proj: Proj; outlines?: "elevation" | "front" | "plan" }[] = [
  { name: "elevation", proj: (p) => [p[0], p[1], -p[2]], outlines: "elevation" },
  { name: "front", proj: (p) => [p[2], p[1], p[0]], outlines: "front" },
  { name: "plan", proj: (p) => [p[0], p[2], p[1]], outlines: "plan" },
  { name: "oblique", proj: (p) => { const x = p[0] * cy - p[2] * sy, z = p[0] * sy + p[2] * cy; return [x, p[1] * cp - z * sp, p[1] * sp + z * cp]; } },
];
const W = size * 2, H = size * 2;
const img = new Uint8Array(W * H * 3).fill(250);
const state = pose ? null : restState(asset, 50);
views.forEach((view, k) => {
  const ox = (k % 2) * size, oy = Math.floor(k / 2) * size;
  const P = tris.map((t) => ({ ...t, q: t.p.map(view.proj) }));
  let ext = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const t of P) for (const a of t.q) { ext.minX = Math.min(ext.minX, a[0]); ext.maxX = Math.max(ext.maxX, a[0]); ext.minY = Math.min(ext.minY, a[1]); ext.maxY = Math.max(ext.maxY, a[1]); }
  const sc = (size - 30) / Math.max(ext.maxX - ext.minX, ext.maxY - ext.minY);
  const tx = (x: number) => ox + 15 + (x - ext.minX) * sc, ty = (y: number) => oy + size - 15 - (y - ext.minY) * sc;
  const zb = new Float32Array(size * size).fill(-1e9);
  for (const t of P) {
    const a = t.q;
    const pts = a.map((v) => [tx(v[0]) - ox, ty(v[1]) - oy, v[2]]);
    const ux = [a[1]![0] - a[0]![0], a[1]![1] - a[0]![1], a[1]![2] - a[0]![2]], vx = [a[2]![0] - a[0]![0], a[2]![1] - a[0]![1], a[2]![2] - a[0]![2]];
    let nx = ux[1]! * vx[2]! - ux[2]! * vx[1]!, ny = ux[2]! * vx[0]! - ux[0]! * vx[2]!, nz = ux[0]! * vx[1]! - ux[1]! * vx[0]!;
    const nl = Math.hypot(nx, ny, nz) || 1;
    const shade = 0.45 + 0.55 * Math.abs(nz / nl);
    const col = palette(t.id).map((c) => Math.min(255, Math.round(c * shade * 255)));
    const minx = Math.max(0, Math.floor(Math.min(...pts.map((p) => p[0]!)))), maxx = Math.min(size - 1, Math.ceil(Math.max(...pts.map((p) => p[0]!))));
    const miny = Math.max(0, Math.floor(Math.min(...pts.map((p) => p[1]!)))), maxy = Math.min(size - 1, Math.ceil(Math.max(...pts.map((p) => p[1]!))));
    const [p0, p1, p2] = pts as [number[], number[], number[]];
    const den = (p1[1]! - p2[1]!) * (p0[0]! - p2[0]!) + (p2[0]! - p1[0]!) * (p0[1]! - p2[1]!);
    if (Math.abs(den) < 1e-12) continue;
    for (let y = miny; y <= maxy; y++) for (let x = minx; x <= maxx; x++) {
      const l0 = ((p1[1]! - p2[1]!) * (x + 0.5 - p2[0]!) + (p2[0]! - p1[0]!) * (y + 0.5 - p2[1]!)) / den;
      const l1 = ((p2[1]! - p0[1]!) * (x + 0.5 - p2[0]!) + (p0[0]! - p2[0]!) * (y + 0.5 - p2[1]!)) / den;
      const l2 = 1 - l0 - l1;
      if (l0 < -1e-6 || l1 < -1e-6 || l2 < -1e-6) continue;
      const z = l0 * p0[2]! + l1 * p1[2]! + l2 * p2[2]!;
      if (z <= zb[y * size + x]!) continue;
      zb[y * size + x] = z;
      const o = ((oy + y) * W + ox + x) * 3;
      img[o] = col[0]!; img[o + 1] = col[1]!; img[o + 2] = col[2]!;
    }
  }
  const dot = (x: number, y: number) => { const X = Math.round(x), Y = Math.round(y); if (X < ox || Y < oy || X >= ox + size || Y >= oy + size) return; const o = (Y * W + X) * 3; img[o] = 220; img[o + 1] = 20; img[o + 2] = 20; };
  if (view.outlines && state) for (const e of (state as any)[view.outlines]) {
    if (!e.closed || !["body", "detail", "glazing"].includes(e.shape.role)) continue;
    for (const ring of e.rings) for (let i = 0; i < ring.length; i++) {
      const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
      const pa = view.proj(view.outlines === "elevation" ? [a.x, a.y, 0] : view.outlines === "front" ? [0, a.y, a.x] : [a.x, 0, a.y]);
      const pb = view.proj(view.outlines === "elevation" ? [b.x, b.y, 0] : view.outlines === "front" ? [0, b.y, b.x] : [b.x, 0, b.y]);
      const n = Math.max(1, Math.ceil(Math.hypot(pb[0] - pa[0], pb[1] - pa[1]) * sc));
      for (let s = 0; s <= n; s++) dot(tx(pa[0] + (pb[0] - pa[0]) * s / n), ty(pa[1] + (pb[1] - pa[1]) * s / n));
    }
  }
});
const crcT = new Uint32Array(256).map((_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
const crc = (b: Uint8Array) => { let c = 0xffffffff; for (const x of b) c = crcT[(c ^ x) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
const chunk = (type: string, data: Uint8Array) => { const b = new Uint8Array(12 + data.length); const dv = new DataView(b.buffer); dv.setUint32(0, data.length); for (let i = 0; i < 4; i++) b[4 + i] = type.charCodeAt(i); b.set(data, 8); dv.setUint32(8 + data.length, crc(b.subarray(4, 8 + data.length))); return b; };
const raw = new Uint8Array(H * (W * 3 + 1));
for (let y = 0; y < H; y++) { raw[y * (W * 3 + 1)] = 0; raw.set(img.subarray(y * W * 3, (y + 1) * W * 3), y * (W * 3 + 1) + 1); }
const ihdr = new Uint8Array(13); const dv = new DataView(ihdr.buffer); dv.setUint32(0, W); dv.setUint32(4, H); ihdr[8] = 8; ihdr[9] = 2;
const parts = [new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))];
writeFileSync(out, Buffer.concat(parts));
console.log(out, tris.length, "triangles");
