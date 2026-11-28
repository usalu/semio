import { readFileSync } from "node:fs";
for (const file of process.argv.slice(2)) {
const b = readFileSync(file); const jl = b.readUInt32LE(12);
const j = JSON.parse(b.subarray(20, 20 + jl).toString()); const bin = b.subarray(20 + jl + 8);
const seen = new Set<number>(); let bytes = 0, save8 = 0, n8 = 0, n = 0, lineBytes = 0, lineSave = 0;
const posCounts = new Map<number, number>();
for (const m of j.meshes) for (const p of m.primitives) { if (seen.has(p.indices)) continue; seen.add(p.indices); const a = j.accessors[p.indices]; const v = j.bufferViews[a.bufferView]; const arr = new Uint16Array(bin.buffer.slice(bin.byteOffset + v.byteOffset, bin.byteOffset + v.byteOffset + a.count * 2)); let max = 0; for (const x of arr) if (x > max) max = x; const sz = a.count * 2; bytes += sz; n++; if (p.mode === 1) lineBytes += sz; if (max < 255) { save8 += a.count; n8++; if (p.mode === 1) lineSave += a.count; } }
console.log(file.split("/").pop(), "index bytes", bytes, "accessors", n, "fit uint8:", n8, "saving", save8, "(lines", lineSave, "of", lineBytes, ")");
}
