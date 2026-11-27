import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const acc = j.accessors, views = j.bufferViews;
const f32 = (i: number) => { const a = acc[i], v = views[a.bufferView]; return new Float32Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + v.byteLength)); };
// owner groups per position accessor: set of scale-mode keys
const owners = new Map<number, Set<string>>();
for (const m of j.meshes) { const key = m.name.split("-").slice(0,2).join("-"); for (const p of m.primitives) { const s = owners.get(p.attributes.POSITION) ?? owners.set(p.attributes.POSITION, new Set()).get(p.attributes.POSITION)!; s.add(key); } }
// vertex multimap: key "x,y,z" per scale -> which modes have it in different accessors
const byScale = new Map<string, Map<string, Set<number>>>();
for (const [pi, ks] of owners) { const P = f32(pi); for (const k of ks) { const [scale] = k.split("-"); const sm = byScale.get(scale!) ?? byScale.set(scale!, new Map()).get(scale!)!; for (let v = 0; v < P.length; v += 3) { const key = P[v]+","+P[v+1]+","+P[v+2]; (sm.get(key) ?? sm.set(key, new Set()).get(key)!).add(pi); } } }
let uniqueAcc = 0, vertsInUnshared = 0, vertsAlsoElsewhere = 0, bytesDup = 0;
for (const [pi, ks] of owners) { if (ks.size !== 1) continue; const k = [...ks][0]!; const [scale] = k.split("-"); const sm = byScale.get(scale!)!; const P = f32(pi); uniqueAcc++; for (let v = 0; v < P.length; v += 3) { vertsInUnshared++; const set = sm.get(P[v]+","+P[v+1]+","+P[v+2])!; if (set.size > 1) { vertsAlsoElsewhere++; } } }
console.log("unshared pos accessors", uniqueAcc, "verts", vertsInUnshared, "of which vertex position also in another accessor of same scale", vertsAlsoElsewhere, "approx bytes (pos+normal 24B)", vertsAlsoElsewhere * 24);
