import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
const { readRustSourceClosureV1 } = await import(process.argv[2]);
const m = process.argv[3], entry = resolve(dirname(m), process.argv[4]);
const r = await readRustSourceClosureV1([entry], "/Users/ueli/Documents/semio", [], async p => existsSync(p) ? new Uint8Array(readFileSync(p)) : null, { manifestDirectory: dirname(m), generated: () => undefined });
for (const x of r) console.log(x.bytes, x.path);console.log(r.length);
