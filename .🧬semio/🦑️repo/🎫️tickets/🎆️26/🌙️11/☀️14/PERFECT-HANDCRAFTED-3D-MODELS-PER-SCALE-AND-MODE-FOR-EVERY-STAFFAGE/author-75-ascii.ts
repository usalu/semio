import { entryOf, loadSource, merged, scaleMeshes, extentOf } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string, d: string) => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]!);
const spec = args[0]!;
const scale = Number(flag("--scale", "50"));
const list = flag("--list", "body") as "body" | "thermal" | "structure";
const az = Number(flag("--az", "-55")) * Math.PI / 180;
const el = Number(flag("--el", "28")) * Math.PI / 180;
const W = Number(flag("--w", "190"));
const mode = flag("--mode", "shade");
const only = flag("--only", "");
const clip = flag("--clip", "");
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const meshes = sm[list];
const solids = sm.solids[list];
const e: number[] = [Math.sin(az) * Math.cos(el), Math.sin(el), Math.cos(az) * Math.cos(el)];
const rr = [e[2]!, 0, -e[0]!];
const rl = Math.hypot(rr[0]!, rr[2]!);
const r = [rr[0]! / rl, 0, rr[2]! / rl];
const u = [e[1]! * r[2]! - e[2]! * r[1]!, e[2]! * r[0]! - e[0]! * r[2]!, e[0]! * r[1]! - e[1]! * r[0]!];
const dot = (a: number[], x: number, y: number, z: number) => a[0]! * x + a[1]! * y + a[2]! * z;
const sel = (i: number) => only === "" || only.split(",").some((p) => solids[i]!.id.startsWith(p));
let minX = 1e9, maxX = -1e9, minY = 1e9, maxY = -1e9;
const box = clip === "" ? null : clip.split(",").map(Number);
for (const [i, mesh] of meshes.entries()) { if (!sel(i)) continue; const w = mesh.world; for (let k = 0; k < w.length; k += 3) { const x = dot(r, w[k]!, w[k + 1]!, w[k + 2]!), y = dot(u, w[k]!, w[k + 1]!, w[k + 2]!); minX = Math.min(minX, x); maxX = Math.max(maxX, x); minY = Math.min(minY, y); maxY = Math.max(maxY, y); } }
if (box) { minX = box[0]!; maxX = box[1]!; minY = box[2]!; maxY = box[3]!; }
const sx = (W - 1) / (maxX - minX);
const H = Math.ceil((maxY - minY) * sx * 0.5) + 1;
const depth = new Float64Array(W * H).fill(-1e9);
const id = new Int32Array(W * H).fill(-1);
const shade = new Float64Array(W * H);
const light = [-0.4, 0.8, 0.5]; const ll = Math.hypot(...light);
for (const [si, mesh] of meshes.entries()) {
  if (!sel(si)) continue;
  const w = mesh.world; const n = w.length / 3;
  const X = new Float64Array(n), Y = new Float64Array(n), D = new Float64Array(n);
  for (let i = 0; i < n; i++) { X[i] = (dot(r, w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!) - minX) * sx; Y[i] = (maxY - dot(u, w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!)) * sx * 0.5; D[i] = dot(e, w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!); }
  for (let t = 0; t < mesh.indices.length; t += 3) {
    const a = mesh.indices[t]!, b = mesh.indices[t + 1]!, c = mesh.indices[t + 2]!;
    const ux = w[b * 3]! - w[a * 3]!, uy = w[b * 3 + 1]! - w[a * 3 + 1]!, uz = w[b * 3 + 2]! - w[a * 3 + 2]!;
    const vx = w[c * 3]! - w[a * 3]!, vy = w[c * 3 + 1]! - w[a * 3 + 1]!, vz = w[c * 3 + 2]! - w[a * 3 + 2]!;
    let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx; const nl = Math.hypot(nx, ny, nz) || 1; nx /= nl; ny /= nl; nz /= nl;
    const lam = Math.abs(nx * light[0]! + ny * light[1]! + nz * light[2]!) / ll;
    const x0 = Math.max(0, Math.floor(Math.min(X[a]!, X[b]!, X[c]!))), x1 = Math.min(W - 1, Math.ceil(Math.max(X[a]!, X[b]!, X[c]!)));
    const y0 = Math.max(0, Math.floor(Math.min(Y[a]!, Y[b]!, Y[c]!))), y1 = Math.min(H - 1, Math.ceil(Math.max(Y[a]!, Y[b]!, Y[c]!)));
    const den = (Y[b]! - Y[c]!) * (X[a]! - X[c]!) + (X[c]! - X[b]!) * (Y[a]! - Y[c]!);
    if (Math.abs(den) < 1e-12) continue;
    for (let py = y0; py <= y1; py++) for (let px = x0; px <= x1; px++) {
      const qx = px + 0.5, qy = py + 0.5;
      const l1 = ((Y[b]! - Y[c]!) * (qx - X[c]!) + (X[c]! - X[b]!) * (qy - Y[c]!)) / den;
      const l2 = ((Y[c]! - Y[a]!) * (qx - X[c]!) + (X[a]! - X[c]!) * (qy - Y[c]!)) / den;
      const l3 = 1 - l1 - l2;
      if (l1 < -0.02 || l2 < -0.02 || l3 < -0.02) continue;
      const d = l1 * D[a]! + l2 * D[b]! + l3 * D[c]!;
      if (d > depth[py * W + px]!) { depth[py * W + px] = d; id[py * W + px] = si; shade[py * W + px] = lam; }
    }
  }
}
const ramp = " .:-=+*#%@";
const ids = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const out: string[] = [];
for (let y = 0; y < H; y++) { let s = ""; for (let x = 0; x < W; x++) { const i = id[y * W + x]!; s += i < 0 ? " " : mode === "id" ? ids[i % ids.length]! : ramp[Math.min(9, 2 + Math.floor(shade[y * W + x]! * 8))]!; } out.push(s.trimEnd()); }
console.log(out.join("\n"));
if (mode === "id") { const seen = new Set<number>(); for (const i of id) if (i >= 0) seen.add(i); console.log([...seen].sort((a, b) => a - b).map((i) => `${ids[i % ids.length]}=${solids[i]!.id}`).join(" ")); }
