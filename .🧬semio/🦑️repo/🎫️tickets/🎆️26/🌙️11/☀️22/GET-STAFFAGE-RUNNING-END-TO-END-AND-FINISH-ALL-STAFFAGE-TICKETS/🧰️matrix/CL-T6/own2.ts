/** 🧮️ Bytes of the GLB of one pose of an asset or trial copy by mesh group (first owner of an accessor), finer than `own.ts`: the mesh name with its scale kept and every other number replaced by `#`, cut to `--depth` name parts (default 3). own2.ts <asset id | file.json> [--pose <id>] [--depth <n>] [--save <file.glb>]
 * @see ../CX-FLOW/own.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [target, ...rest] = process.argv.slice(2);
const option = (name: string): string | undefined => (rest.includes(name) ? rest[rest.indexOf(name) + 1] : undefined);
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const pose = option("--pose") ?? asset.presentation.defaultPose;
const depth = Number(option("--depth") ?? 3);
const bytes = await writeGlb(asset, { pose });
if (option("--save") !== undefined) writeFileSync(option("--save")!, bytes);
const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
const jsonBytes = view.getUint32(12, true);
const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonBytes)));
const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const bytesOf = (i: number): number => { const a = json.accessors[i]; const v = json.bufferViews[a.bufferView]; return v.byteStride ? v.byteStride * a.count : a.count * size[a.componentType]! * width[a.type]!; };
console.log(`glb ${bytes.length} bytes (${(bytes.length / 1048576).toFixed(2)} MiB, budget 8): pose ${pose}, json ${jsonBytes}, bin ${bytes.length - jsonBytes - 28}; meshes ${json.meshes.length}, nodes ${json.nodes.length}, materials ${json.materials.length}, accessors ${json.accessors.length}`);
console.log(`  json by key: ${Object.entries(json).map(([k, v]) => [k, JSON.stringify(v).length] as const).sort((a, b) => b[1] - a[1]).slice(0, 8).map(([k, n]) => `${k} ${n}`).join(", ")}`);
const seen = new Set<number>();
const groups: Record<string, { bytes: number; triangles: number; meshes: number; vertices: number; json: number }> = {};
for (const mesh of json.meshes) {
  const key = (mesh.name as string).split("-").slice(0, depth).map((part, at) => (at === 0 ? part : part.replace(/\d+/gu, "#"))).join("-");
  const g = (groups[key] ??= { bytes: 0, triangles: 0, meshes: 0, vertices: 0, json: 0 });
  g.meshes += 1;
  g.json += JSON.stringify(mesh).length;
  for (const p of mesh.primitives) {
    for (const a of [...Object.values(p.attributes as Record<string, number>), p.indices as number]) if (a !== undefined && !seen.has(a)) { seen.add(a); g.bytes += bytesOf(a); }
    if (p.mode === undefined || p.mode === 4) g.triangles += json.accessors[p.indices].count / 3;
    g.vertices += json.accessors[p.attributes.POSITION].count;
  }
}
for (const [key, g] of Object.entries(groups).sort((a, b) => b[1].bytes - a[1].bytes).slice(0, Number(option("--top") ?? 60))) console.log(`  ${key.padEnd(44)} ${String(g.bytes).padStart(9)} B, ${g.meshes} meshes, ${Math.round(g.triangles)} triangles, ${g.vertices} vertices, mesh json ${g.json}`);
const modes: Record<string, number> = {};
for (const [key, g] of Object.entries(groups)) { const m = /^s\d+-[a-z]+/u.exec(key)?.[0] ?? key; modes[m] = (modes[m] ?? 0) + g.bytes; }
console.log(`  by scene prefix: ${Object.entries(modes).sort((a, b) => b[1] - a[1]).map(([k, n]) => `${k} ${n}`).join(", ")}`);
