import { readFileSync, existsSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
/** 🧮️ Lists the non-test Rust source closure files with byte sizes. Usage: window-config-closure.ts <root> <lib.rs>. */
const root = resolve(process.argv[2]!);
const files: any[] = await readRustSourceClosureV1([resolve(process.argv[3]!)], root, [], async p => existsSync(p) && statSync(p).isFile() ? new Uint8Array(readFileSync(p)) : null);
for (const f of files) console.log((f.bytes ?? f.content?.byteLength), f.path);
