import {statSync} from "node:fs";
const m = await import(process.argv[2]);
const rows = m.loadedImplementationRowsV1;
const out = m.classifyImplementationRowsV1(rows);
for (const r of out) {
  if (r.kind === "stamp") { console.log("STAMP", r.path, JSON.stringify(r.fact).slice(0,60)); continue; }
  const age = Math.round((Date.now()-statSync(r.path).mtimeMs)/60000);
  console.log("CONTENT", age+"min", r.path.replace(/^.*📚️library\//, ""));
}
process.exit(0);
