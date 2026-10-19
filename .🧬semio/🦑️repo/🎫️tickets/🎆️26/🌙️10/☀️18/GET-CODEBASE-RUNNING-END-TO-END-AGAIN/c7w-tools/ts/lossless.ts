import { readFileSync } from "node:fs";
import { ts, parse } from "./ast.ts";
const norm = (t: string) => t.replace(/^export\s+/u, "").replace(/\bprotected\b/gu, "private");
function units(path: string, text: string, classMembers = false): string[] {
  const sf = parse(path, text);
  const out: string[] = [];
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) continue;
    if (classMembers && ts.isClassDeclaration(st)) {
      for (const m of st.members) out.push(norm(m.getText(sf)).replace(/^abstract\s+/u, ""));
      continue;
    }
    out.push(norm(st.getText(sf)));
  }
  return out;
}
const [origPath, listPath, mode] = process.argv.slice(2);
const orig = units(origPath, readFileSync(origPath, "utf8"), mode === "class");
const created: string[] = [];
for (const f of readFileSync(listPath, "utf8").split("\n").filter(Boolean)) created.push(...units(f, readFileSync(f.startsWith("/") ? f : "/Users/ueli/Documents/semio/" + f, "utf8"), mode === "class"));
const count = (xs: string[]) => { const m = new Map<string, number>(); for (const x of xs) m.set(x, (m.get(x) ?? 0) + 1); return m; };
const a = count(orig), b = count(created);
const missing = [...a].filter(([k, n]) => (b.get(k) ?? 0) < n);
const extra = [...b].filter(([k, n]) => (a.get(k) ?? 0) < n);
console.log(`${origPath.split("/").pop()}: orig units ${orig.length}, new units ${created.length}, missing ${missing.length}, extra ${extra.length}`);
for (const [k] of missing.slice(0, 3)) console.log("  MISSING:", k.slice(0, 140).replace(/\n/g, " "));
for (const [k] of extra.slice(0, 3)) console.log("  EXTRA:", k.slice(0, 140).replace(/\n/g, " "));
if (process.argv[5] === "diff") {
  import("node:child_process").then(({ spawnSync }) => {
    const { writeFileSync } = require("node:fs");
    for (let i = 0; i < Math.min(missing.length, extra.length); i++) {
      writeFileSync("/tmp/ll-a.txt", missing[i][0]); writeFileSync("/tmp/ll-b.txt", extra[i][0]);
      const r = spawnSync("diff", ["-u", "/tmp/ll-a.txt", "/tmp/ll-b.txt"], { encoding: "utf8" });
      console.log(r.stdout.split("\n").filter((l) => /^[+-][^+-]/.test(l)).map((l) => l.slice(0, 260)).join("\n"));
    }
  });
}
