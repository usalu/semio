/** 📊️ JSON anatomy of a saved GLB by object kind: anat.ts <file.glb> */
import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const L = (v: unknown) => v === undefined ? 0 : JSON.stringify(v).length;
console.log("json", jl, "bin", b.length - jl - 28);
for (const k of Object.keys(j)) console.log(k.padEnd(14), String(L(j[k])).padStart(9), Array.isArray(j[k]) ? j[k].length : "");
const sum = (arr: any[], f: (x: any) => unknown) => arr.reduce((s, x) => s + L(f(x)), 0);
console.log("node names", sum(j.nodes, (n) => n.name), "node extras", sum(j.nodes, (n) => n.extras), "node trs", sum(j.nodes, (n) => [n.translation, n.rotation, n.scale]), "node children", sum(j.nodes, (n) => n.children), "mesh/skin refs", sum(j.nodes, (n) => [n.mesh, n.skin]));
console.log("mesh names", sum(j.meshes, (m) => m.name), "prim attrs", sum(j.meshes.flatMap((m: any) => m.primitives), (p) => p.attributes), "prim extras", sum(j.meshes.flatMap((m: any) => m.primitives), (p) => p.extras), "prim ext", sum(j.meshes.flatMap((m: any) => m.primitives), (p) => p.extensions), "prims", j.meshes.reduce((s: number, m: any) => s + m.primitives.length, 0));
console.log("acc bounds", sum(j.accessors, (a) => [a.min, a.max]), "acc other", sum(j.accessors, (a) => ({ ...a, min: 0, max: 0 })));
console.log("mat names", sum(j.materials, (m) => m.name), "mat extras", sum(j.materials, (m) => m.extras), "mat pbr", sum(j.materials, (m) => m.pbrMetallicRoughness));
console.log("scenes extras", sum(j.scenes, (s) => s.extras), "anim", L(j.animations), "skins", L(j.skins));
const dupNodes = new Map<string, number>(); for (const n of j.nodes) { const k = JSON.stringify({ ...n, name: 0, children: 0 }); dupNodes.set(k, (dupNodes.get(k) ?? 0) + 1); }
console.log("distinct nodes ignoring name/children", dupNodes.size);
const dupM = new Map<string, number>(); for (const m of j.meshes) { const k = JSON.stringify(m.primitives); dupM.set(k, (dupM.get(k) ?? 0) + 1); }
console.log("distinct meshes by prims", dupM.size, "of", j.meshes.length);
const dupA = new Map<string, number>(); for (const a of j.accessors) { const k = JSON.stringify(a); dupA.set(k, (dupA.get(k) ?? 0) + 1); }
console.log("distinct accessors", dupA.size, "of", j.accessors.length, "views", j.bufferViews.length);
const dupMat = new Map<string, number>(); for (const m of j.materials) { const k = JSON.stringify({ ...m, name: 0, extras: 0 }); dupMat.set(k, (dupMat.get(k) ?? 0) + 1); }
console.log("distinct material bodies", dupMat.size, "of", j.materials.length);
console.log("sample node", JSON.stringify(j.nodes[5]), "\nmesh", JSON.stringify(j.meshes[5]).slice(0, 700), "\nacc", JSON.stringify(j.accessors[5]), "view", JSON.stringify(j.bufferViews[5]), "\nmat", JSON.stringify(j.materials[2]));
