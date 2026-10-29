import { readFileSync, existsSync } from "node:fs";
const { readRustSourceClosureV1 } = await import(process.argv[2]);
const entry = process.argv[3], root = process.argv[4];
const r = await readRustSourceClosureV1([entry], root, [], async p => existsSync(p) ? new Uint8Array(readFileSync(p)) : null, { manifestDirectory: process.argv[5], generated: () => undefined });
const total = r.reduce((s, x) => s + x.bytes, 0), active = r.reduce((s, x) => s + x.activeLines, 0);
console.log("files", r.length, "bytes", total, "activeLines", active);
for (const x of r.sort((a,b)=>b.bytes-a.bytes).slice(0,8)) console.log(x.bytes, x.activeLines, x.path.slice(-90));
