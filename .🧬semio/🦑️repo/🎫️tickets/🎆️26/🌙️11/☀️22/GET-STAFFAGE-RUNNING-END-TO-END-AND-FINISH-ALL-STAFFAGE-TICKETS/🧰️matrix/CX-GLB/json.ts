import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString());
console.log("materials", j.materials.length, JSON.stringify(j.materials[0]), JSON.stringify(j.materials[100]));
const sig = new Map<string, number>(); for (const m of j.materials) { const {name, extras, ...r} = m; const k = JSON.stringify(r); sig.set(k, (sig.get(k)??0)+1); }
console.log("distinct material bodies", sig.size);
console.log("variants", j.extensions.KHR_materials_variants.variants.length);
console.log(JSON.stringify(j.meshes[0]).slice(0, 1500));
console.log(JSON.stringify(j.nodes.slice(0,3)));
console.log(JSON.stringify(j.accessors.slice(0,2)), JSON.stringify(j.bufferViews.slice(0,2)));
let mapChars = 0; for (const m of j.meshes) for (const p of m.primitives) mapChars += JSON.stringify(p.extensions ?? {}).length; console.log("variant mappings chars", mapChars);
const nm = new Set(j.nodes.map((n:any)=>n.name)).size; console.log("nodes", j.nodes.length, "distinct names", nm);
