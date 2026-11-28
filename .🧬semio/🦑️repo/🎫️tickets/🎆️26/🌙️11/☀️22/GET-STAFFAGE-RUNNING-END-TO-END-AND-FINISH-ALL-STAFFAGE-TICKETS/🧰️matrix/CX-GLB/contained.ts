import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const f32 = (i: number) => { const a = j.accessors[i], v = j.bufferViews[a.bufferView]; return new Float32Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + v.byteLength)); };
const who = new Map<number, string[]>(); for (const m of j.meshes) for (const p of m.primitives) (who.get(p.attributes.POSITION) ?? who.set(p.attributes.POSITION, []).get(p.attributes.POSITION)!).push(m.name + "/" + (p.mode === 4 ? "T" : "L") + (p.extras?.solids?.map((s: any) => s.id).join("+") ?? ""));
const ids = [...who.keys()]; const arrs = new Map(ids.map((i) => [i, f32(i)]));
let shown = 0;
for (const i of ids) { const a = arrs.get(i)!; if (a.length < 300) continue; for (const k of ids) { if (k === i) continue; const c = arrs.get(k)!; if (c.length <= a.length) continue; for (let at = 0; at + a.length <= c.length; at += 3) { if (c[at] !== a[0] || c[at+1] !== a[1] || c[at+2] !== a[2]) continue; let ok = true; for (let q = 0; q < a.length; q++) if (c[at + q] !== a[q]) { ok = false; break; } if (ok && shown++ < 12) { console.log(`acc ${i} (${a.length/3} verts) [${who.get(i)!.slice(0,2).join(",").slice(0,90)}] inside acc ${k} (${c.length/3} verts) at vertex ${at/3} [${who.get(k)!.slice(0,2).join(",").slice(0,90)}]`); } if (ok) break; } } }
