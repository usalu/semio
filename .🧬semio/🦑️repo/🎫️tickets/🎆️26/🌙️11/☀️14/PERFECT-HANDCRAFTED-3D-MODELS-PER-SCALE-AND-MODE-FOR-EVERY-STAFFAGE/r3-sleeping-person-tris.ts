/** 🔺️ Triangle count of every body solid of every scale of the round 3 sleeping person at the fixed monotonic quality (chord 2 mm, 16 segments; C4), to find what keeps a coarser scale from dropping.
 * Usage: `bun r3-sleeping-person-tris.ts [document]`
 * @see ./volumes-lib.ts */
import { readFileSync } from "node:fs";
import { localGeometry } from "./volumes-lib.ts";
const asset = JSON.parse(readFileSync(process.argv[2] ?? "🗑️generated/r3-sleeping-person/🔣️.json", "utf8")).asset;
for (const scale of ["50", "100", "200", "500"]) {
  const rows = (asset.volumes[scale].body as any[]).map((s) => [s.id, localGeometry(s, { chordM: 0.002, minSegments: 16, maxSegments: 16 }).indices.length / 3] as const);
  console.log(scale, rows.reduce((n, r) => n + r[1], 0), rows.sort((a, b) => b[1] - a[1]).slice(0, 8).map((r) => r.join(":")).join(" "));
}
