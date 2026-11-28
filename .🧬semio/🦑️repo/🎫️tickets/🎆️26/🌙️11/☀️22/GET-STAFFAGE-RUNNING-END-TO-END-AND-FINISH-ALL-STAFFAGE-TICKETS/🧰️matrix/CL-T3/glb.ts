/** ⚖️ The size of the pose GLBs of an asset and where the bytes sit: per pose the bytes against the budget of `🧊️model-catalog` (8 MB), and with `--scenes` the triangles of every scene and the accessor bytes owned by each scale and mode. glb.ts [--scenes] [--poses <id,id>] <asset id | file.json>… */
import { readFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { BUDGET, glbFacts } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const args = process.argv.slice(2);
const scenes = args.includes("--scenes");
const chosen = args.includes("--poses") ? args[args.indexOf("--poses") + 1]!.split(",") : undefined;
const targets = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--poses");
const entries = await assetPathsFromFiles();
const SIZE: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 }, COUNT: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
for (const target of targets) {
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset(entries.find((e) => e.folder === target)!)).asset;
  for (const pose of asset.poses.filter((p: { id: string; key?: boolean }) => (chosen === undefined ? p.key !== true : chosen.includes(p.id)))) {
    const { bytes, triangles } = await writeGlbDetailed(asset, { pose: pose.id });
    console.log(`${asset.id} pose:${pose.id} ${bytes.length} bytes ${(bytes.length / 1048576).toFixed(2)} MB ${triangles} triangles ${bytes.length > BUDGET.glbBytes ? "OVER" : `under by ${((BUDGET.glbBytes - bytes.length) / 1048576).toFixed(2)} MB`}`);
    if (!scenes) continue;
    const facts = glbFacts(bytes), json = facts.json, owners = new Map<number, Set<string>>(), kinds = new Map<number, string>(), sums = new Map<string, number>();
    console.log(`  scenes ${Object.entries(facts.scenes).map(([name, f]) => `${name}=${f.triangles}`).join(" ")}`);
    for (const mesh of json.meshes) for (const primitive of mesh.primitives) for (const [name, id] of [...Object.entries(primitive.attributes as Record<string, number>), ["INDEX", primitive.indices as number] as const]) {
      (owners.get(id) ?? owners.set(id, new Set()).get(id)!).add(String(mesh.name).split("-").slice(0, 2).join("-"));
      kinds.set(id, primitive.mode === 1 && name === "INDEX" ? "LINES" : name);
    }
    json.accessors.forEach((a: { count: number; type: string; componentType: number }, i: number) => {
      const own = [...(owners.get(i) ?? ["none"])], size = a.count * COUNT[a.type]! * SIZE[a.componentType]!;
      for (const key of [`owner ${own.length === 1 ? own[0] : "shared"}`, `kind ${kinds.get(i) ?? "other"}/${a.componentType}`]) sums.set(key, (sums.get(key) ?? 0) + size);
    });
    console.log(`  json ${new DataView(bytes.buffer, bytes.byteOffset).getUint32(12, true)} bytes; ${[...sums].sort().map(([k, n]) => `${k}=${n}`).join(" ")}`);
  }
}
