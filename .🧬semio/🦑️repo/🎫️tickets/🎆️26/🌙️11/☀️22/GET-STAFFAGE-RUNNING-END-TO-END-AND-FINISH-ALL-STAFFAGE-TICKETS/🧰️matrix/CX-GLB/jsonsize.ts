import { readFileSync } from "node:fs";
const b = readFileSync(process.argv[2]!); const jl = b.readUInt32LE(12);
const text = b.subarray(20, 20 + jl).toString(); const j = JSON.parse(text);
const short = (x: number): number => { if (!Number.isFinite(x) || Number.isInteger(x)) return x; for (let p = 1; p <= 9; p++) { const s = Number(x.toPrecision(p)); if (Math.fround(s) === Math.fround(x)) return s; } return x; };
const walk = (v: any): any => typeof v === "number" ? short(v) : Array.isArray(v) ? v.map(walk) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, k === "extras" || k==="name" ? x : walk(x)])) : v;
const all = (v: any): any => typeof v === "number" ? short(v) : Array.isArray(v) ? v.map(all) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, all(x)])) : v;
console.log("orig", text.length, "float32-shortest (all)", JSON.stringify(all(j)).length);
for (const k of Object.keys(j)) console.log(k, JSON.stringify(j[k]).length, "->", JSON.stringify(all(j[k])).length);
console.log("[DEBUG] material extras", JSON.stringify(j.materials.map((m: any) => m.extras)).length, "names", JSON.stringify(j.materials.map((m: any) => m.name)).length);
