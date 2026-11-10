import { readdir, readFile } from "node:fs/promises";
import { join } from "node:path";
import { selectRustSourceV1 } from "D:/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
async function scan(dir) { for (const e of await readdir(dir,{withFileTypes:true})) { const f=join(dir,e.name); if(e.isDirectory()) await scan(f); else if(f.endsWith(".rs")) { try { selectRustSourceV1(await readFile(f,"utf8"),[]); } catch(e) { console.log(f, String(e)); } } } }
await scan("D:/semio/🧰️framework/🔨️modules/🌱️value/🗂️ordered");
