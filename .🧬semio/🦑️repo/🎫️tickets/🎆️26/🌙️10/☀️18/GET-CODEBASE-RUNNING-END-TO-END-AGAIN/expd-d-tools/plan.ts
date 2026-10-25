import { readFileSync, existsSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const inv = JSON.parse(readFileSync(join(dir, "inv0.json"), "utf8"));
const packages: any[] = JSON.parse(readFileSync(join(dir, "packages.json"), "utf8"));
const byName = new Map(packages.map((p) => [p.name, p]));
type Cover = { pkg: string; key: string; facade: string };
const coverIndex = new Map<string, Cover[]>();
const facadeFiles = new Map<string, Cover>();
for (const pkg of packages) for (const row of pkg.exportTargets) {
  if (row.target === null) continue;
  const facade = normalize(join(pkg.owner, row.target));
  facadeFiles.set(facade, { pkg: pkg.name, key: row.subpath, facade });
  if (!existsSync(join(root, facade)) || !/\.[cm]?tsx?$/u.test(facade)) continue;
  const sf = ts.createSourceFile(facade, readFileSync(join(root, facade), "utf8"), ts.ScriptTarget.Latest, true);
  for (const st of sf.statements) {
    const spec = (ts.isExportDeclaration(st) || ts.isImportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : null;
    if (spec?.startsWith(".")) { const t = normalize(join(dirname(facade), spec)); coverIndex.set(t, [...(coverIndex.get(t) ?? []), { pkg: pkg.name, key: row.subpath, facade }]); }
  }
}
const targets = new Map<string, { rules: Set<string>; pairs: number; froms: Set<string> }>();
for (const p of inv.pairs) { const t = targets.get(p.to) ?? { rules: new Set(), pairs: 0, froms: new Set() }; p.rules.forEach((r: string) => t.rules.add(r)); t.pairs++; t.froms.add(p.from); targets.set(p.to, t); }
const rows = [...targets].map(([to, t]) => {
  const isFacade = facadeFiles.has(to);
  const cov = coverIndex.get(to) ?? [];
  return { to, n: t.pairs, rules: [...t.rules].join(","), isFacade, facadeKey: isFacade ? facadeFiles.get(to)!.key : "", covered: cov.map((c) => c.pkg + c.key.slice(1)).join(" ") };
}).sort((a, b) => b.n - a.n);
let cov = 0, unc = 0, fac = 0, covn = 0, uncn = 0;
for (const r of rows) { if (r.isFacade) fac++; else if (r.covered) { cov++; covn += r.n; } else { unc++; uncn += r.n; } }
console.log("targets", rows.length, "facade-unkeyed", fac, "covered", cov, "pairs", covn, "uncovered", unc, "pairs", uncn);
for (const r of rows.filter((r) => r.covered && !r.isFacade)) console.log("COV", r.n, r.to.replace(/^🧰️framework\/(🔨️modules\/)?/u, ""), "=>", r.covered);
if (process.argv[3] === "uncovered") {
  const owner = (rules: string) => rules;
  for (const r of rows.filter((r) => !r.covered && !r.isFacade)) {
    const refs = inv.refs.filter((x: any) => x.to === r.to);
    const names = new Set<string>(); let flags = new Set<string>();
    for (const x of refs) { x.names.names?.forEach((n: any) => names.add(n.imported)); if (x.names.def) flags.add("default"); if (x.names.ns) flags.add("ns"); if (x.names.sideEffect) flags.add("side"); if (x.form !== "import" && x.form !== "export") flags.add(x.form); }
    console.log(r.n, r.rules.replace(/@semio-tech\//gu, ""), r.to.replace(/^🧰️framework\/(🔨️modules\/)?/u, ""), "|", [...flags].join("+"), [...names].slice(0, 8).join(","));
  }
}
