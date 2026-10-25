import { readFileSync, writeFileSync } from "node:fs";
import { dirname, normalize, join, relative } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const targets = new Set(process.argv.slice(3));
const plans = new Map<string, { glue: string; target: string; values: Set<string>; types: Set<string> }>();
for (const name of ["edges0.plan-applied.json", "e1.plan-applied.json"]) {
  const d = JSON.parse(readFileSync(join(dir, name), "utf8"));
  for (const p of d.plans) if (p.extend && targets.has(p.glue)) {
    const row = plans.get(p.glue) ?? { glue: p.glue, target: p.target, values: new Set(), types: new Set() };
    p.values.forEach((n: string) => row.values.add(n)); p.types.forEach((n: string) => row.types.add(n)); plans.set(p.glue, row);
  }
}
for (const p of plans.values()) {
  let text = spawnSync("git", ["show", ":" + p.glue], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }).stdout;
  const specRaw = relative(dirname(join(root, p.glue)), join(root, p.target)).split("\\").join("/");
  const spec = specRaw.startsWith(".") ? specRaw : "./" + specRaw;
  const have = (typeOnly: boolean) => { const sf = ts.createSourceFile(p.glue, text, ts.ScriptTarget.Latest, true); const out = new Set<string>(); for (const st of sf.statements) if (ts.isExportDeclaration(st) && st.moduleSpecifier && (st.moduleSpecifier as ts.StringLiteral).text === spec && st.isTypeOnly === typeOnly && st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) out.add(e.name.text); return out; };
  const merge = (names: string[], typeOnly: boolean) => {
    if (!names.length) return;
    const sf = ts.createSourceFile(p.glue, text, ts.ScriptTarget.Latest, true);
    for (const st of sf.statements) {
      if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || (st.moduleSpecifier as ts.StringLiteral).text !== spec || st.isTypeOnly !== typeOnly || !st.exportClause || !ts.isNamedExports(st.exportClause) || st.attributes) continue;
      const all = [...new Set([...st.exportClause.elements.map((e) => e.getText(sf)), ...names])].sort();
      text = text.slice(0, st.getStart(sf)) + `export ${typeOnly ? "type " : ""}{ ${all.join(", ")} } from "${spec}";` + text.slice(st.getEnd());
      return;
    }
    text = text.replace(/\n*$/u, "\n") + `export ${typeOnly ? "type " : ""}{ ${names.join(", ")} } from "${spec}";\n`;
  };
  const hv = have(false), ht = have(true);
  merge([...p.values].filter((n) => !hv.has(n)).sort(), false);
  merge([...p.types].filter((n) => !ht.has(n)).sort(), true);
  writeFileSync(join(root, p.glue), text);
  console.log("repaired", p.glue.slice(-70), text.split("\n").length);
}
