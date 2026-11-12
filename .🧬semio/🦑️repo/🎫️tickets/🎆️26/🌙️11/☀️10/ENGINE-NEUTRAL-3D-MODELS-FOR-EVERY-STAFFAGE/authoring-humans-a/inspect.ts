/** Inspection of an authored volume: per-solid world boxes at rest, pose evaluation (connectivity, side flips).
 * Usage: bun inspect.ts <slug> [--volume file] [--poses a,b|all] [--boxes] [--gap 0.02] */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { solveChain } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { frames, loadAsset, meshes, resolveAsset, toWorld, type V3 } from "../volume-lib.ts";
import { carry } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const args = process.argv.slice(2);
const slug = args[0]!;
const opt = (name: string): string | undefined => { const i = args.indexOf(name); return i < 0 ? undefined : args[i + 1]; };
const entry = resolveAsset(slug);
const asset = loadAsset(entry.path);
const vf = opt("--volume");
const volume = vf ? (JSON.parse(readFileSync(resolve(vf), "utf8")).volume) : asset.volume;
const gap = Number(opt("--gap") ?? 0.02);
const dims = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const list = meshes(asset, volume.solids, dims);
const byId = new Map(volume.solids.map((s: any) => [s.id, s]));
const rest = frames(asset);
const fmt = (v: number) => v.toFixed(3);

type Cloud = { id: string; bone: string; pts: V3[] };
function clouds(offsets: any, root: any): Cloud[] {
  const now: any = offsets === null ? null : solveChain(asset.rig, offsets, root);
  return list.map((m) => {
    const solid: any = byId.get(m.id);
    const bone: string = solid.bone;
    const raw: V3[] = [];
    const P = (i: number): V3 => [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!];
    const tri = m.indices;
    for (let t = 0; t < tri.length; t += 3) {
      const a = P(tri[t]!), b = P(tri[t + 1]!), c = P(tri[t + 2]!);
      for (const [p, q] of [[a, b], [b, c], [c, a]] as const) {
        const L = Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2]);
        const n = Math.max(1, Math.ceil(L / (gap * 0.5)));
        for (let k = 0; k < n; k++) raw.push([p[0] + (q[0] - p[0]) * k / n, p[1] + (q[1] - p[1]) * k / n, p[2] + (q[2] - p[2]) * k / n]);
      }
    }
    const take = Math.min(raw.length, 12000);
    const pts: V3[] = [];
    for (let t = 0; t < take; t++) {
      const p = take === raw.length ? raw[t]! : raw[Math.floor((t * 0.6180339887 % 1) * raw.length)]!;
      pts.push(now === null ? p : (carry(rest[bone]!, now[bone].frame, p) as V3));
    }
    return { id: m.id, bone, pts };
  });
}

function components(cs: Cloud[]): { groups: string[][]; } {
  const cell = gap;
  const grid = new Map<string, { s: number; p: V3 }[]>();
  const kk = (x: number, y: number, z: number) => `${x},${y},${z}`;
  cs.forEach((c, s) => { for (const p of c.pts) { const k = kk(Math.floor(p[0] / cell), Math.floor(p[1] / cell), Math.floor(p[2] / cell)); const a = grid.get(k); if (a) a.push({ s, p }); else grid.set(k, [{ s, p }]); } });
  const parent = cs.map((_, i) => i);
  const find = (x: number): number => (parent[x] === x ? x : (parent[x] = find(parent[x]!)));
  const g2 = gap * gap;
  cs.forEach((c, i) => {
    for (const p of c.pts) {
      const cx = Math.floor(p[0] / cell), cy = Math.floor(p[1] / cell), cz = Math.floor(p[2] / cell);
      for (let dx = -1; dx <= 1; dx++) for (let dy = -1; dy <= 1; dy++) for (let dz = -1; dz <= 1; dz++) {
        const es = grid.get(kk(cx + dx, cy + dy, cz + dz));
        if (!es) continue;
        for (const e of es) { if (find(e.s) === find(i)) continue; const d = (p[0] - e.p[0]) ** 2 + (p[1] - e.p[1]) ** 2 + (p[2] - e.p[2]) ** 2; if (d <= g2) parent[find(i)] = find(e.s); }
      }
    }
  });
  const g = new Map<number, string[]>();
  cs.forEach((c, i) => { const r = find(i); g.set(r, [...(g.get(r) ?? []), c.id]); });
  return { groups: [...g.values()].sort((a, b) => b.length - a.length) };
}

const centre = (c: Cloud): V3 => { const m: V3 = [0, 0, 0]; for (const p of c.pts) for (let k = 0; k < 3; k++) m[k]! += p[k]! / c.pts.length; return m; };
const exact = (id: string) => { const m = list.find((x) => x.id === id)!; const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9]; for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); } return { lo, hi }; };
const box = (c: Cloud) => { const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9]; for (const p of c.pts) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); } return { lo, hi }; };

const base = clouds(null, null);
if (args.includes("--boxes")) {
  for (const c of base) { const b = exact(c.id); const s: any = byId.get(c.id); console.log(`${c.id.padEnd(22)} ${String(s.bone).padEnd(12)} ${s.kind.padEnd(9)} x[${fmt(b.lo[0])},${fmt(b.hi[0])}] y[${fmt(b.lo[1])},${fmt(b.hi[1])}] z[${fmt(b.lo[2])},${fmt(b.hi[2])}]`); }
}
const restGroups = components(base).groups;
console.log(`rest: ${restGroups.length} connected group(s) at gap ${gap}: ${restGroups.map((g) => g.length).join(",")}`);
if (restGroups.length > 1) for (const g of restGroups.slice(1)) console.log("  detached at rest:", g.join(" "));
const restComponent = new Map<string, number>();
restGroups.forEach((g, i) => g.forEach((id) => restComponent.set(id, i)));
const pair = opt("--pair")?.split(",");
const minDist = (cs: Cloud[]) => { const a = cs.find((c) => c.id === pair![0])!, b = cs.find((c) => c.id === pair![1])!; let m = 1e9; for (const p of a.pts) for (const q of b.pts) m = Math.min(m, Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2])); return m; };
const want = opt("--poses");
const poses = asset.poses.filter((p: any) => want === "all" ? true : want === undefined ? !/-\d+$/.test(p.id) : want.split(",").includes(p.id));
for (const pose of poses) {
  const root = { x: pose.root?.x ?? 0, y: pose.root?.y ?? 0, angleDeg: pose.root?.angleDeg ?? 0 };
  const cs = clouds(pose.bones, root);
  const groups = components(cs).groups;
  const flips: string[] = [];
  base.forEach((b, i) => { const a = centre(b)[2], n = centre(cs[i]!)[2]; if (Math.abs(a) > 0.03 && Math.abs(n) > 0.03 && Math.sign(a) !== Math.sign(n)) flips.push(b.id); });
  const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9];
  for (const c of cs) { const b = box(c); for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, b.lo[k]!); hi[k] = Math.max(hi[k]!, b.hi[k]!); } }
  const extra = groups.length - restGroups.length;
  if (pair) console.log(`   distance ${pair.join("-")}: ${minDist(cs).toFixed(3)}`);
  console.log(`pose ${pose.id}: ${groups.length} group(s) [${groups.map((g) => g.length).join(",")}] bbox ${fmt(hi[0] - lo[0])} x ${fmt(hi[1] - lo[1])} x ${fmt(hi[2] - lo[2])} lowest y ${fmt(lo[1])}${extra > 0 ? "  NEW DETACHMENTS" : ""}${flips.length ? "  SIDE FLIPS: " + flips.join(",") : ""}`);
  if (extra > 0) for (const g of groups.slice(1)) console.log("   detached:", g.join(" "));
}
