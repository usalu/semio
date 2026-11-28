/** 🧱️ Which meshes hold the bytes of one pose GLB: every accessor is counted once, for the first mesh that uses it; prints the largest meshes with their primitives, the sum per scene prefix, the JSON chunk split into its top-level keys and the largest `extras`. meshes.ts <asset id | file.json> <pose> [top] */
import { readFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { glbFacts } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const [target, pose, top = "40"] = process.argv.slice(2);
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const { bytes } = await writeGlbDetailed(asset, { pose: pose! });
const json = glbFacts(bytes).json;
const SIZE: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 }, COUNT: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const seen = new Set<number>(), rows: { name: string; bytes: number; primitives: number; solids: string }[] = [], scenes = new Map<string, number>();
for (const mesh of json.meshes) {
  let own = 0;
  for (const primitive of mesh.primitives) for (const id of [...Object.values(primitive.attributes as Record<string, number>), primitive.indices as number]) {
    if (seen.has(id)) continue;
    seen.add(id);
    const a = json.accessors[id];
    own += a.count * COUNT[a.type]! * SIZE[a.componentType]!;
  }
  const solids = [...new Set(mesh.primitives.flatMap((p: { extras?: { solids?: { id: string }[] } }) => (p.extras?.solids ?? []).map((s) => s.id)))];
  rows.push({ name: String(mesh.name), bytes: own, primitives: mesh.primitives.length, solids: `${solids.length} solids ${solids.slice(0, 4).join(",")}` });
  const scene = String(mesh.name).split("-").slice(0, 2).join("-");
  scenes.set(scene, (scenes.get(scene) ?? 0) + own);
}
console.log(`${asset.id} pose:${pose} ${bytes.length} bytes, ${json.meshes.length} meshes, ${json.nodes.length} nodes, ${json.accessors.length} accessors`);
console.log(`first-owner bytes per scene: ${[...scenes].map(([k, n]) => `${k}=${n}`).join(" ")}`);
console.log(`json keys: ${Object.entries(json).map(([k, v]) => `${k}=${JSON.stringify(v).length}`).join(" ")}; extras of primitives ${json.meshes.reduce((n: number, m: { primitives: { extras?: unknown }[] }) => n + m.primitives.reduce((k, p) => k + JSON.stringify(p.extras ?? {}).length, 0), 0)}; extras of nodes ${json.nodes.reduce((n: number, node: { extras?: unknown }) => n + JSON.stringify(node.extras ?? {}).length, 0)}`);
for (const row of rows.sort((a, b) => b.bytes - a.bytes).slice(0, Number(top))) console.log(`  ${row.bytes} ${row.name} ${row.primitives} primitives, ${row.solids}`);
