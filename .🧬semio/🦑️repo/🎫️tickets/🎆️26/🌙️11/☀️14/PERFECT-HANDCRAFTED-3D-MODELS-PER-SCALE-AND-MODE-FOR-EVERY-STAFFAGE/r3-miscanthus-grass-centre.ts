/** 🧵 Round 3, vegetation/miscanthus-grass: compares the product's centre line of slender solids of the volumes source with their own section centres (bone-local). Usage: bun r3-miscanthus-grass-centre.ts <scale> <solid id>... */
import { readFileSync } from "node:fs";
import { copyMesh } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { centreLineOf, clusterPoints } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts";
import { qualityOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
const [scale, ...ids] = process.argv.slice(2);
const asset = JSON.parse(readFileSync("🗑️generated/r3-miscanthus-grass/🔣️.json", "utf8")).asset;
for (const id of ids) {
  const solid = asset.volumes[scale!].body.find((s: { id: string }) => s.id === id);
  const mesh = copyMesh(solid, qualityOf(asset));
  const line = centreLineOf(mesh.positions, mesh.positions, mesh.indices);
  const at = solid.sections.map((s: { at: number[] }) => s.at);
  if (line === undefined) { console.log(id, "no centre line"); continue; }
  const pts = clusterPoints(line, mesh.positions, 0);
  const seg = (p: readonly number[], a: number[], b: number[]) => { const d = b.map((x, k) => x - a[k]!); const l2 = d.reduce((t, x) => t + x * x, 0); const f = Math.max(0, Math.min(1, p.reduce((t, x, k) => t + (x - a[k]!) * d[k]!, 0) / l2)); return Math.hypot(...p.map((x, k) => x - a[k]! - f * d[k]!)); };
  const dist = pts.map((p) => Math.min(...at.slice(1).map((b: number[], i: number) => seg(p, at[i], b))));
  const ends = [at[0], at[at.length - 1]].map((e: number[]) => Math.min(...pts.map((p) => Math.hypot(...p.map((x, k) => x - e[k]!)))));
  console.log(id, "clusters", pts.length, "edges", line.edges.length, "length", line.length.toFixed(3), "thickness", line.thickness.toFixed(4), "max off path mm", (Math.max(...dist) * 1000).toFixed(1), "mean", ((dist.reduce((t, x) => t + x, 0) / dist.length) * 1000).toFixed(1), "end gaps mm", ends.map((e) => (e * 1000).toFixed(0)).join("/"));
}
