import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
const b = readFileSync(process.argv[2]!);
const jl = b.readUInt32LE(12);
const json = JSON.parse(b.subarray(20, 20 + jl).toString());
const bin = b.subarray(20 + jl + 8);
const acc = json.accessors, views = json.bufferViews;
const comp: any = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 }, n: any = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const bytesOf = (a: any) => { const v = views[a.bufferView]; return bin.subarray(v.byteOffset, v.byteOffset + v.byteLength); };
const attr = new Map<number, string>();
const scaleMode = new Map<number, Set<string>>();
for (const m of json.meshes) { const key = m.name.split("-").slice(0, 2).join("-"); for (const p of m.primitives) { for (const [k, v] of Object.entries(p.attributes)) attr.set(v as number, k); attr.set(p.indices, p.mode === 1 ? "IDX_lines" : "IDX_tri"); for (const id of [...Object.values(p.attributes) as number[], p.indices]) { (scaleMode.get(id) ?? scaleMode.set(id, new Set()).get(id)!).add(key); } } }
const bySm = new Map<string, number>(); const sizeOf = (a: any) => a.count * n[a.type] * comp[a.componentType];
acc.forEach((a: any, i: number) => { const ks = [...(scaleMode.get(i) ?? [])]; const k = ks.length === 1 ? ks[0]! : "shared:" + ks.length; bySm.set(k, (bySm.get(k) ?? 0) + sizeOf(a)); });
console.log("unique accessor bytes by owner", Object.fromEntries([...bySm].sort()));
const seen = new Map<string, number>(); let dupBytes = 0, dupCount = 0;
acc.forEach((a: any, i: number) => { const h = a.type + a.componentType + createHash("sha1").update(bytesOf(a)).digest("hex"); if (seen.has(h)) { dupBytes += sizeOf(a); dupCount++; } else seen.set(h, i); });
console.log("byte-identical accessors not shared", dupCount, dupBytes);
const hist = new Map<string, [number, number]>();
acc.forEach((a: any, i: number) => { const k = (attr.get(i) ?? "other") + "/" + a.componentType; const e = hist.get(k) ?? [0, 0]; e[0]++; e[1] += sizeOf(a); hist.set(k, e); });
console.log("kind/comp [count,bytes]", Object.fromEntries(hist));
const small = acc.filter((a: any) => sizeOf(a) < 1024).length; console.log("accessors <1KB", small, "of", acc.length);
let primCount = 0, extrasBytes = 0; for (const m of json.meshes) for (const p of m.primitives) { primCount++; extrasBytes += JSON.stringify(p.extras ?? {}).length; }
console.log("primitives", primCount, "extras.solids chars", extrasBytes, "nodes chars", JSON.stringify(json.nodes).length, "meshes chars", JSON.stringify(json.meshes).length, "accessors chars", JSON.stringify(json.accessors).length, "views", JSON.stringify(json.bufferViews).length, "materials", JSON.stringify(json.materials).length, "scenes", JSON.stringify(json.scenes).length, "ext", JSON.stringify(json.extensions).length);
// Subrange identity: position accessors identical to a slice of a larger position accessor
const pos = acc.map((a: any, i: number) => ({ a, i })).filter(({ i }: any) => attr.get(i) === "POSITION");
const totalPos = pos.reduce((s: number, { a }: any) => s + sizeOf(a), 0);
const f = (x: any) => new Float32Array(bytesOf(x).buffer.slice(bytesOf(x).byteOffset, bytesOf(x).byteOffset + bytesOf(x).byteLength));
// first-vertex index
const idx = new Map<string, any[]>();
for (const p of pos) { const arr = f(p.a); const key = arr.slice(0, 3).join(","); (idx.get(key) ?? idx.set(key, []).get(key)!).push({ ...p, arr }); }
let contained = 0, containedBytes = 0;
for (const p of pos) { const arr = f(p.a); const cands = idx.get(arr.slice(0,3).join(",")) ?? []; }
let sub = 0, subBytes = 0;
const all = pos.map((p: any) => ({ ...p, arr: f(p.a) }));
const firstTriple = new Map<string, {i:number; at:number; arr: Float32Array}[]>();
for (const p of all) for (let v = 0; v < p.arr.length; v += 3) { const k = p.arr[v] + "," + p.arr[v+1] + "," + p.arr[v+2]; (firstTriple.get(k) ?? firstTriple.set(k, []).get(k)!).push({ i: p.i, at: v, arr: p.arr }); }
for (const p of all) { const k = p.arr[0] + "," + p.arr[1] + "," + p.arr[2]; const hit = (firstTriple.get(k) ?? []).find((c) => c.i !== p.i && c.arr.length >= p.arr.length && p.arr.every((x: number, j: number) => c.arr[c.at + j] === x)); if (hit) { sub++; subBytes += sizeOf(p.a); } }
console.log("position accessors", pos.length, totalPos, "contained verbatim inside another accessor", sub, subBytes);
