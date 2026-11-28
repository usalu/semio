/** 📊️ JSON chunk anatomy of a saved GLB: jsonparts.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString());
const L = (v: unknown) => JSON.stringify(v).length;
console.log("json", jl, "materials", json.materials.length, L(json.materials), "meshes", json.meshes.length, "prims", json.meshes.reduce((s: number, m: any) => s + m.primitives.length, 0), "nodes", json.nodes.length, "accessors", json.accessors.length, "views", json.bufferViews.length);
let ex = 0, var_ = 0, attrs = 0, names = 0, tags = 0;
for (const m of json.meshes) { names += m.name.length; for (const p of m.primitives) { ex += L(p.extras ?? {}); var_ += L(p.extensions ?? {}); attrs += L(p.attributes) + L(p.indices) } }
console.log({ solidsExtras: ex, variantMappings: var_, attrs, meshNames: names });
console.log("node extras", json.nodes.reduce((s: number, n: any) => s + L(n.extras ?? {}), 0), "node names", json.nodes.reduce((s: number, n: any) => s + n.name.length, 0), "node TRS", json.nodes.reduce((s: number, n: any) => s + L(n.translation ?? 0) + L(n.rotation ?? 0) + L(n.scale ?? 0), 0));
console.log("mat extras", json.materials.reduce((s: number, m: any) => s + L(m.extras ?? {}), 0), "mat pbr", json.materials.reduce((s: number, m: any) => s + L(m.pbrMetallicRoughness), 0));
const dup = new Map<string, number>(); for (const m of json.materials) { const k = JSON.stringify({ ...m, name: 0, extras: 0 }); dup.set(k, (dup.get(k) ?? 0) + 1); }
console.log("distinct material bodies", dup.size, "of", json.materials.length);
console.log("accessor sample", JSON.stringify(json.accessors[3]), "view sample", JSON.stringify(json.bufferViews[3]));
