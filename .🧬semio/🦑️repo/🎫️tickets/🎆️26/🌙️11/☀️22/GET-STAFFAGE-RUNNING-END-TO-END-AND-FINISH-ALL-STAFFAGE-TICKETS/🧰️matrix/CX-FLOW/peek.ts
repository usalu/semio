import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12); const json = JSON.parse(b.subarray(20, 20 + jl).toString());
console.log(json.scenes.map((s: any) => s.name));
const names = json.nodes.map((n: any) => n.name); console.log(names.slice(0, 5), names.filter((n: string) => /flow/.test(n)).slice(0, 10), names.filter((n: string) => /flow/.test(n)).length);
const ids = new Map<string, number>(); const tris = new Map<string, number>();
for (const m of json.meshes) for (const p of m.primitives) for (const s of p.extras?.solids ?? []) { const k = String(s.id).replace(/\d+/g, "#"); ids.set(k, (ids.get(k) ?? 0) + 1); tris.set(k, (tris.get(k) ?? 0) + s.count / 3); }
console.log([...ids].sort((a, b) => b[1] - a[1]).slice(0, 30).map(([k, n]) => `${k} n=${n} tri=${tris.get(k)}`));
console.log(JSON.stringify(json.meshes[5]).slice(0, 800)); console.log(JSON.stringify(json.nodes[40])); console.log(JSON.stringify(json.materials[3]));
