import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { deflateSync } from "node:zlib";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { GENERATED, QUALITY_FOR, frames, loadAsset, resolveAsset, toWorld } from "./volume-lib.ts";

const args = process.argv.slice(2);
const slug = args[0]!;
const opt = (k: string) => { const i = args.indexOf(k); return i >= 0 ? args[i + 1] : undefined; };
const asset = loadAsset(resolveAsset(slug).path);
const volFile = opt("--volume");
const volume = volFile ? (JSON.parse(readFileSync(volFile, "utf8")).volume) : asset.volume;
const poseId = opt("--pose");
const clip = opt("--clip");
let frameOf: (b: string) => any = (b) => frames(asset)[b];
if (poseId || clip) {
  const q = clip ? { kind: "clip", clip, timeS: Number(opt("--t") ?? 0) } : { kind: "pose", pose: poseId };
  const state: any = evaluate(asset, q as any, { kind: "scale", scale: { denominator: 50 } } as any);
  const arr = (p: any) => [p.x, p.y, p.z];
  frameOf = (b) => { const f = state.bones[b].frame; return { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) }; };
}
const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const quality = QUALITY_FOR(maxDim);
type Tri = { a: number[]; b: number[]; c: number[]; col: number[] };
const tris: Tri[] = [];
const bones = asset.rig.bones.map((b: any) => b.id);
const hue = (i: number) => { const h = (i * 0.618034) % 1; const f = (n: number) => { const k = (n + h * 6) % 6; return Math.round(255 * (0.55 + 0.35 * (1 - Math.max(0, Math.min(k, 4 - k, 1))))); }; return [f(5), f(3), f(1)]; };
for (const s of volume.solids) {
  const g = tessellateSolid({ ...s, node: s.bone } as never, quality);
  const f = frameOf(s.bone);
  const w: number[][] = [];
  for (let i = 0; i < g.positions.length; i += 3) w.push(toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]));
  const col = hue(bones.indexOf(s.bone));
  for (let i = 0; i < g.indices.length; i += 3) tris.push({ a: w[g.indices[i]!]!, b: w[g.indices[i + 1]!]!, c: w[g.indices[i + 2]!]!, col });
}
let mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
for (const t of tris) for (const p of [t.a, t.b, t.c]) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, p[k]!); mx[k] = Math.max(mx[k]!, p[k]!); }
const ctr = mn.map((v, k) => (v + mx[k]!) / 2);
const R = Math.max(...mx.map((v, k) => v - mn[k]!)) * 0.62;
const W = 560, H = 420;
type Cam = { right: number[]; up: number[]; fwd: number[] };
const norm = (a: number[]) => { const l = Math.hypot(...a) || 1; return a.map((x) => x / l); };
const cross = (a: number[], b: number[]) => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
const dot = (a: number[], b: number[]) => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
function cam(dir: number[], up0: number[]): Cam { const fwd = norm(dir); const right = norm(cross(up0, fwd)); const up = cross(fwd, right); return { right, up, fwd }; }
const cams: Cam[] = [cam([0, 0, 1], [0, 1, 0]), cam([-1, 0, 0], [0, 1, 0]), cam([0, -1, 0.0001], [1, 0, 0]), cam([-0.7, -0.45, 0.8], [0, 1, 0])];
const img = new Uint8Array(W * 2 * H * 2 * 3).fill(255);
for (let c = 0; c < 4; c++) {
  const cm = cams[c]!;
  const ox = (c % 2) * W, oy = Math.floor(c / 2) * H;
  const depth = new Float32Array(W * H).fill(1e9);
  const scale = Math.min(W / (2 * R * (c === 2 ? 1 : 1)), H / (2 * R));
  const proj = (p: number[]) => { const d = [p[0]! - ctr[0]!, p[1]! - ctr[1]!, p[2]! - ctr[2]!]; return [W / 2 + dot(d, cm.right) * scale, H / 2 - dot(d, cm.up) * scale, dot(d, cm.fwd)]; };
  const light = norm([0.3, 0.8, -0.5].map((x, k) => x));
  for (const t of tris) {
    const n = norm(cross([t.b[0]! - t.a[0]!, t.b[1]! - t.a[1]!, t.b[2]! - t.a[2]!], [t.c[0]! - t.a[0]!, t.c[1]! - t.a[1]!, t.c[2]! - t.a[2]!]));
    const facing = dot(n, cm.fwd);
    const shade = 0.45 + 0.55 * Math.abs(dot(n, norm([cm.fwd[0]! * -1 + 0.4 * cm.up[0]! + 0.3 * cm.right[0]!, cm.fwd[1]! * -1 + 0.4 * cm.up[1]! + 0.3 * cm.right[1]!, cm.fwd[2]! * -1 + 0.4 * cm.up[2]! + 0.3 * cm.right[2]!])));
    void facing; void light;
    const P = [proj(t.a), proj(t.b), proj(t.c)] as number[][];
    const minX = Math.max(0, Math.floor(Math.min(P[0]![0]!, P[1]![0]!, P[2]![0]!))), maxX = Math.min(W - 1, Math.ceil(Math.max(P[0]![0]!, P[1]![0]!, P[2]![0]!)));
    const minY = Math.max(0, Math.floor(Math.min(P[0]![1]!, P[1]![1]!, P[2]![1]!))), maxY = Math.min(H - 1, Math.ceil(Math.max(P[0]![1]!, P[1]![1]!, P[2]![1]!)));
    const [p0, p1, p2] = P as [number[], number[], number[]];
    const den = (p1[1]! - p2[1]!) * (p0[0]! - p2[0]!) + (p2[0]! - p1[0]!) * (p0[1]! - p2[1]!);
    if (Math.abs(den) < 1e-9) continue;
    for (let y = minY; y <= maxY; y++) for (let x = minX; x <= maxX; x++) {
      const l0 = ((p1[1]! - p2[1]!) * (x + 0.5 - p2[0]!) + (p2[0]! - p1[0]!) * (y + 0.5 - p2[1]!)) / den;
      const l1 = ((p2[1]! - p0[1]!) * (x + 0.5 - p2[0]!) + (p0[0]! - p2[0]!) * (y + 0.5 - p2[1]!)) / den;
      const l2 = 1 - l0 - l1;
      if (l0 < -0.001 || l1 < -0.001 || l2 < -0.001) continue;
      const z = l0 * p0[2]! + l1 * p1[2]! + l2 * p2[2]!;
      if (z >= depth[y * W + x]!) continue;
      depth[y * W + x] = z;
      const o = ((oy + y) * W * 2 + ox + x) * 3;
      img[o] = Math.round(t.col[0]! * shade); img[o + 1] = Math.round(t.col[1]! * shade); img[o + 2] = Math.round(t.col[2]! * shade);
    }
  }
  for (let x = 0; x < W; x++) for (const y of [0, H - 1]) { const o = ((oy + y) * W * 2 + ox + x) * 3; img[o] = img[o + 1] = img[o + 2] = 120; }
  for (let y = 0; y < H; y++) for (const x of [0, W - 1]) { const o = ((oy + y) * W * 2 + ox + x) * 3; img[o] = img[o + 1] = img[o + 2] = 120; }
}
const crcT = new Uint32Array(256).map((_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
const crc = (b: Uint8Array) => { let c = 0xffffffff; for (const x of b) c = crcT[(c ^ x) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
const chunk = (type: string, data: Uint8Array) => { const out = new Uint8Array(12 + data.length); const dv = new DataView(out.buffer); dv.setUint32(0, data.length); out.set(new TextEncoder().encode(type), 4); out.set(data, 8); dv.setUint32(8 + data.length, crc(out.subarray(4, 8 + data.length))); return out; };
const IW = W * 2, IH = H * 2;
const raw = new Uint8Array(IH * (IW * 3 + 1));
for (let y = 0; y < IH; y++) { raw[y * (IW * 3 + 1)] = 0; raw.set(img.subarray(y * IW * 3, (y + 1) * IW * 3), y * (IW * 3 + 1) + 1); }
const ihdr = new Uint8Array(13); const dv = new DataView(ihdr.buffer); dv.setUint32(0, IW); dv.setUint32(4, IH); ihdr[8] = 8; ihdr[9] = 2;
const png = Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))]);
const name = opt("--out") ?? `${slug}.${poseId ?? clip ?? "rest"}.png`;
writeFileSync(join(GENERATED, "authoring-animals", name), png);
console.log(join(GENERATED, "authoring-animals", name));
