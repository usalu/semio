import { analyze } from "./ast.ts";
const path = process.argv[2];
const a = analyze(path);
console.log("imports", a.imports.length, "exportsFrom", a.exportsFrom.length, "decls", a.decls.length);
for (const e of a.exportsFrom) console.log("EXPF", e.start, JSON.stringify(e.spec), e.names.join(","), e.star ? "*" : "");
for (const d of a.decls) {
  const internal = [...d.deps].filter((n) => a.topNames.has(n) && !d.names.includes(n));
  console.log(`${d.startLine}-${d.endLine} ${d.kind}${d.exported ? " X" : ""} ${d.names.join(",")} -> ${internal.join(",")}`);
}
