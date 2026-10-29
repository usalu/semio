import { readFileSync } from "node:fs";
const { selectRustSourceV1 } = await import(process.argv[2]);
for (const f of process.argv.slice(3)) { try { selectRustSourceV1(readFileSync(f,"utf8"),[]); console.log("ok",f.slice(-40)); } catch(e){ console.log("FAIL",f.slice(-60),e.message); } }
