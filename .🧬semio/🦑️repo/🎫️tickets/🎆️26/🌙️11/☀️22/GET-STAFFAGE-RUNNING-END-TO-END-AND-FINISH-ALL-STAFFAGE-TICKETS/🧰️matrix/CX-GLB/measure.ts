import { writeFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { createHash } from "node:crypto";
const [id, poseArg, out] = process.argv.slice(2);
const entries = await assetPathsFromFiles();
const entry = entries.find((e) => e.folder === id)!;
const { asset } = await readStreamedAsset(entry);
const pose = poseArg || asset.poses.find((p: any) => p.key !== true)!.id;
const t = Date.now();
const r = await writeGlbDetailed(asset, { pose });
console.log(id, pose, "bytes", r.bytes.length, "tri", r.triangles, "ms", Date.now() - t);
if (out) writeFileSync(out, r.bytes);
const dv = new DataView(r.bytes.buffer, r.bytes.byteOffset);
const jl = dv.getUint32(12, true);
const json = JSON.parse(new TextDecoder().decode(r.bytes.subarray(20, 20 + jl)));
const acc = json.accessors, views = json.bufferViews;
const kind = new Map<string, number>();
const comp: any = { 5121: 1, 5123: 2, 5125: 4, 5126: 4 }, n: any = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const attrOf = new Map<number, string>();
for (const m of json.meshes) for (const p of m.primitives) { for (const [k, v] of Object.entries(p.attributes)) attrOf.set(v as number, k); attrOf.set(p.indices, p.mode === 1 ? "IDX_lines" : "IDX_tri"); }
acc.forEach((a: any, i: number) => { const k = attrOf.get(i) ?? "other"; kind.set(k, (kind.get(k) ?? 0) + a.count * n[a.type] * comp[a.componentType]); });
console.log("by kind", Object.fromEntries(kind), "accessors", acc.length, "meshes", json.meshes.length, "scenes", json.scenes.length, "jsonBytes", jl);
const per = json.meshes.map((m: any) => ({ name: m.name, prims: m.primitives.length, verts: m.primitives.reduce((s: number, p: any) => s + acc[p.attributes.POSITION].count, 0), idx: m.primitives.reduce((s: number, p: any) => s + acc[p.indices].count, 0), mode: m.primitives.map((p: any)=>p.mode).join("") }));
console.log(JSON.stringify(per.slice(0, 40)));
console.log("meshes", per.length, "skins", json.skins?.length, "nodes", json.nodes.length);
const hist = new Map<string, number>(); for (const a of acc) { const k = attrOf.get(acc.indexOf(a)) ?? "o"; }
