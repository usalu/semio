import { readFileSync } from "node:fs";
for (const f of process.argv.slice(2)) {
  try { const j = JSON.parse(readFileSync(f, "utf8")); console.log(j.name, "targets:", Object.keys(j.targets ?? {}).join(" ").slice(0, 900)); }
  catch (e: any) { console.log("ERR", f, String(e.message).slice(0, 100)); }
}
