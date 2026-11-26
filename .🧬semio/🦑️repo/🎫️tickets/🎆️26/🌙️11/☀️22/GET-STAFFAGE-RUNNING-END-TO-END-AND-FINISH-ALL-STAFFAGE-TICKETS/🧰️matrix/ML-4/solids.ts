/** 📦️ The solids of a model as the suite `🧊️model-bounds` reads them (`modelOf`, `tessellateSolid` at 0.1 mm chord): id, node, tags and the box of the tessellated mesh in metres. Usage: solids.ts <asset id | file.json> <scales, comma separated> <views, comma separated> <id regexp> */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { modelOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";

const [which, scales, views, pattern] = process.argv.slice(2);
const root = process.env.S + "/🗂️catalog";
const file = which!.endsWith(".json") ? which! : readdirSync(root).map((c) => join(root, c, which!, "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const f = (n: number): string => n.toFixed(4);
for (const scale of scales!.split(",").map(Number)) for (const view of views!.split(",")) {
  const model = await modelOf(asset, { scale, view: view as never, poses: [], clips: [] });
  const hits = model.solids.filter((s: any) => new RegExp(pattern!).test(s.id));
  console.log(`== ${asset.id} 1:${scale} ${view}: ${hits.length} of ${model.solids.length} solids`);
  for (const solid of hits as any[]) {
    const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
    const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < mesh.positions.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, mesh.positions[i]!); hi[i % 3] = Math.max(hi[i % 3]!, mesh.positions[i]!); }
    console.log(`   ${solid.id} node ${solid.node} tags ${(solid.tags ?? []).join(",")} x ${f(lo[0]!)}..${f(hi[0]!)} y ${f(lo[1]!)}..${f(hi[1]!)} z ${f(lo[2]!)}..${f(hi[2]!)} tri ${mesh.indices.length / 3}`);
  }
}
