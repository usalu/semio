/** 📦️ The GLB of the default pose of an asset or a trial copy as the live writer gives it, against the budget of 8 MiB of `🧊️model-catalog`: the bytes, the JSON chunk and the binary chunk, the bytes by kind of accessor, the JSON by key, and per scene the meshes, triangles and the bytes of its accessors (an accessor shared by several scenes counts for the first). glb.ts <asset id | file.json> [--pose <id>]
 * @see ../CL-3D/profile.ts */
import { readFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [target, ...rest] = process.argv.slice(2);
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const pose = rest.includes("--pose") ? rest[rest.indexOf("--pose") + 1]! : asset.presentation.defaultPose;
const bytes = await writeGlb(asset, { pose });
const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
const jsonBytes = view.getUint32(12, true);
const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonBytes)));
const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const bytesOf = (index: number): number => { const a = json.accessors[index]; return a.count * size[a.componentType]! * width[a.type]!; };
console.log(`glb ${bytes.length} bytes (${(bytes.length / 1048576).toFixed(2)} MiB, budget 8): pose ${pose}, json ${jsonBytes}, bin ${bytes.length - jsonBytes - 28}; meshes ${json.meshes.length}, nodes ${json.nodes.length}, materials ${json.materials.length}, accessors ${json.accessors.length}`);
const kinds: Record<string, { accessors: Set<number>; bytes: number }> = {};
for (const mesh of json.meshes) for (const p of mesh.primitives) for (const [name, index] of [...Object.entries(p.attributes as Record<string, number>), [p.mode === 1 ? "line-indices" : "indices", p.indices as number] as const]) {
  const k = (kinds[name] ??= { accessors: new Set(), bytes: 0 });
  if (!k.accessors.has(index)) { k.accessors.add(index); k.bytes += bytesOf(index); }
}
console.log(`  bin by kind: ${Object.entries(kinds).map(([k, v]) => `${k} ${v.bytes}`).join(", ")}`);
console.log(`  json by key: ${Object.entries(json).map(([k, v]) => [k, JSON.stringify(v).length] as const).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([k, n]) => `${k} ${n}`).join(", ")}`);
const seen = new Set<number>();
for (const scene of json.scenes) {
  let meshes = 0, triangles = 0, own = 0;
  const walk = (index: number): void => {
    const node = json.nodes[index];
    if (node.mesh !== undefined) {
      meshes += 1;
      for (const p of json.meshes[node.mesh].primitives) {
        if (p.mode === undefined || p.mode === 4) triangles += json.accessors[p.indices].count / 3;
        for (const index of [...Object.values(p.attributes as Record<string, number>), p.indices as number]) if (!seen.has(index)) { seen.add(index); own += bytesOf(index); }
      }
    }
    for (const child of node.children ?? []) walk(child);
  };
  for (const root of scene.nodes) walk(root);
  console.log(`  ${scene.name}: ${meshes} meshes, ${triangles} triangles, ${own} bytes of its own accessors`);
}
