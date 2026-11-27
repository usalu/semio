import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString());
const unlit = (m: number) => j.materials[m].extensions?.KHR_materials_unlit !== undefined;
console.log("unlit materials", j.materials.filter((_: any, i: number) => unlit(i)).length, "of", j.materials.length, "alpha blend", j.materials.filter((m: any) => m.alphaMode === "BLEND").length);
const nb = (i: number) => j.accessors[i].count * 12;
const seen = new Map<number, boolean>(); // normal accessor -> all uses unlit
for (const m of j.meshes) for (const p of m.primitives) { if (p.attributes.NORMAL === undefined) continue; const mats = [p.material, ...(p.extensions?.KHR_materials_variants?.mappings ?? []).map((x: any) => x.material)]; const allUnlit = mats.every(unlit); seen.set(p.attributes.NORMAL, (seen.get(p.attributes.NORMAL) ?? true) && allUnlit); }
let tot = 0, dead = 0; for (const [a, u] of seen) { tot += nb(a); if (u) dead += nb(a); }
console.log("normal bytes", tot, "never read (all materials unlit)", dead);
const kinds = new Map<string, number>(); for (const m of j.meshes) for (const p of m.primitives) { const k = (unlit(p.material) ? "U" : "L") + ":" + (p.extensions?.KHR_materials_variants?.mappings ?? []).map((x: any) => unlit(x.material) ? "U" : "L").join(""); kinds.set(k, (kinds.get(k) ?? 0) + 1); }
console.log(Object.fromEntries(kinds));
