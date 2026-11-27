/** 🔬️ Where the triangles and the bytes of an asset's 3D model go: per scene the solids and triangles by kind (and the five heaviest solids), and for the GLB of the default pose the bytes of the JSON chunk and of every kind of accessor. profile.ts <asset id | file.json> [--glb] */
import { readFileSync } from "node:fs";
import { tessellateSolid, scenesOf } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [target, ...flags] = process.argv.slice(2);
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")) : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const document = await modelOfAsset(asset, { poses: "all", clips: [] });
for (const scene of scenesOf(document)) {
  const scale = Number(/^s(\d+)-/u.exec(scene.id)![1]);
  const quality = qualityOf(asset, scale);
  const members = new Set(scene.nodes);
  const kinds: Record<string, { solids: number; triangles: number; vertices: number }> = {};
  const heavy: [number, string][] = [];
  let total = 0;
  for (const solid of document.solids.filter((s) => members.has(s.node))) {
    const g = tessellateSolid(solid, quality);
    const key = `${solid.kind}${solid.tags?.includes("glyph") ? "/glyph" : ""}${solid.tags?.includes("flow") ? "/flow" : ""}`;
    const k = (kinds[key] ??= { solids: 0, triangles: 0, vertices: 0 });
    k.solids += 1;
    k.triangles += g.indices.length / 3;
    k.vertices += g.positions.length / 3;
    total += g.indices.length / 3;
    heavy.push([g.indices.length / 3, `${solid.id}:${solid.kind}`]);
  }
  heavy.sort((a, b) => b[0] - a[0]);
  console.log(`${scene.id} ${total} triangles | ${Object.entries(kinds).map(([k, v]) => `${k} ${v.solids}/${v.triangles}t/${v.vertices}v`).join(", ")} | top ${heavy.slice(0, 4).map(([n, id]) => `${id}=${n}`).join(" ")}`);
}
if (flags.includes("--glb")) {
  const bytes = await writeGlb(asset, { pose: asset.presentation.defaultPose });
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const jsonBytes = view.getUint32(12, true);
  const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonBytes)));
  const use = new Map<number, string>();
  for (const mesh of json.meshes) for (const p of mesh.primitives) {
    for (const [name, accessor] of Object.entries(p.attributes as Record<string, number>)) use.set(accessor, name);
    use.set(p.indices, p.mode === 1 ? "line-indices" : "indices");
  }
  for (const skin of json.skins ?? []) use.set(skin.inverseBindMatrices, "inverse-bind");
  const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
  const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
  const sums: Record<string, { accessors: number; bytes: number; count: number }> = {};
  json.accessors.forEach((a: any, i: number) => {
    const key = `${use.get(i) ?? "other"}:${a.componentType}`;
    const s = (sums[key] ??= { accessors: 0, bytes: 0, count: 0 });
    s.accessors += 1;
    s.count += a.count;
    s.bytes += a.count * size[a.componentType]! * width[a.type]!;
  });
  console.log(`glb ${bytes.length} bytes: json ${jsonBytes}, bin ${bytes.length - jsonBytes - 28}; bufferViews ${json.bufferViews.length}, accessors ${json.accessors.length}, meshes ${json.meshes.length}, nodes ${json.nodes.length}, materials ${json.materials.length}`);
  for (const [key, s] of Object.entries(sums).sort((a, b) => b[1].bytes - a[1].bytes)) console.log(`  ${key}: ${s.accessors} accessors, ${s.count} elements, ${s.bytes} bytes (referenced; shared views count once per accessor)`);
  const top: Record<string, number> = {};
  for (const [k, v] of Object.entries(json)) top[k] = JSON.stringify(v).length;
  console.log(`  json by key: ${Object.entries(top).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([k, v]) => `${k} ${v}`).join(", ")}`);
  const extras = json.meshes.reduce((n: number, m: any) => n + m.primitives.reduce((k: number, p: any) => k + JSON.stringify(p.extras ?? {}).length, 0), 0);
  console.log(`  primitive extras ${extras}`);
}
