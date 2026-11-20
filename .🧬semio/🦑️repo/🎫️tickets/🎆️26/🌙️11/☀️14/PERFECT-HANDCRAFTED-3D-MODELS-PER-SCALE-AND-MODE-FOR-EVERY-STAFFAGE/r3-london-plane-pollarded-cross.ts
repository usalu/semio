/** 🔀 Round 3 london plane: overlays the body silhouettes of two scales in the three views (green both, red finer only, blue coarser only). Usage: bun r3-london-plane-pollarded-cross.ts <finer> <coarser> */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, meshesAt } from "./volumes-lib.ts";
import { Canvas } from "./volumes-raster.ts";
import { asset, build } from "./r3-london-plane-pollarded-model.ts";
const [a, b] = [process.argv[2] ?? "50", process.argv[3] ?? "100"];
const lists = build();
const volumes = Object.fromEntries(Object.entries(lists).map(([k, v]) => [k, { status: "authored", denominator: Number(k), ...v }]));
const W = 800, K = 100;
const canvas = new Canvas(W * 3, 950, [250, 250, 246]);
const masks = [a, b].map((s) => {
  const m = new Uint8Array(W * 3 * 950);
  for (const mesh of meshesAt({ ...asset, volumes }, Number(s), lists[s]!.body.filter((x) => x.slender !== true && x.texture !== true))) for (let t = 0; t < mesh.indices.length; t += 3) for (let v = 0; v < 3; v++) {
    const p = [0, 1, 2].map((k) => { const i = mesh.indices[t + k]! * 3; const q = [mesh.world[i]!, mesh.world[i + 1]!, mesh.world[i + 2]!]; return v === 0 ? [W / 2 + q[0]! * K, 920 - q[1]! * K] : v === 1 ? [W * 1.5 + q[2]! * K, 920 - q[1]! * K] : [W * 2.5 + q[0]! * K, 475 - q[2]! * K]; });
    const lo = Math.floor(Math.min(p[0]![1]!, p[1]![1]!, p[2]![1]!)), hi = Math.ceil(Math.max(p[0]![1]!, p[1]![1]!, p[2]![1]!));
    for (let y = Math.max(0, lo); y <= Math.min(949, hi); y++) {
      const xs: number[] = [];
      for (const [i, j] of [[0, 1], [1, 2], [2, 0]] as const) if ((p[i]![1]! <= y + 0.5) !== (p[j]![1]! <= y + 0.5)) xs.push(p[i]![0]! + ((y + 0.5 - p[i]![1]!) * (p[j]![0]! - p[i]![0]!)) / (p[j]![1]! - p[i]![1]!));
      if (xs.length === 2) for (let x = Math.round(Math.min(...xs)); x < Math.round(Math.max(...xs)); x++) m[y * W * 3 + x] = 1;
    }
  }
  return m;
});
for (let i = 0; i < masks[0]!.length; i++) { const f = masks[0]![i], c = masks[1]![i]; if (f || c) canvas.set(i % (W * 3), Math.floor(i / (W * 3)), f && c ? [150, 205, 160] : f ? [225, 70, 70] : [60, 110, 225]); }
canvas.text(8, 6, `1:${a} (red only) against 1:${b} (blue only): elevation, front, plan`, [20, 20, 20], 2);
const out = join(GENERATED, "r3-london-plane-pollarded", `cross.${a}-${b}.png`);
writeFileSync(out, canvas.png());
console.log(out);
