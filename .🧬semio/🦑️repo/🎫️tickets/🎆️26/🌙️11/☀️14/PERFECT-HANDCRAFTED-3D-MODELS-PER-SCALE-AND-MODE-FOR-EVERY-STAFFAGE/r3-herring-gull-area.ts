/** 📐️ Round 3 herring gull: mesh area of every body solid of one scale (triangle sum, nothing buried), to see which part makes a thermal region too large. Usage: bun r3-herring-gull-area.ts <scale> [pattern] */
import { readFileSync } from "node:fs";
import { localGeometry } from "./volumes-lib.ts";
const scale = process.argv[2] ?? "50";
const re = new RegExp(process.argv[3] ?? ".");
const vol = JSON.parse(readFileSync(new URL("./volumes/animals/herring-gull.json", import.meta.url), "utf8")).volumes[scale];
for (const s of vol.body) {
  if (!re.test(s.id)) continue;
  const g = localGeometry(s, { chordM: 0.0002, minSegments: 32, maxSegments: 96 });
  let a = 0;
  for (let t = 0; t < g.indices.length; t += 3) {
    const p = [0, 1, 2].map((k) => [0, 1, 2].map((c) => g.positions[g.indices[t + k]! * 3 + c]!));
    const u = [0, 1, 2].map((c) => p[1]![c]! - p[0]![c]!), v = [0, 1, 2].map((c) => p[2]![c]! - p[0]![c]!);
    a += 0.5 * Math.hypot(u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!);
  }
  console.log(s.id.padEnd(22), (a * 1e4).toFixed(2), "cm2");
}
