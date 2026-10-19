import {readdirSync, readFileSync} from "node:fs";
import {join, relative} from "node:path";
const root = "/Users/ueli/Documents/semio";
const T = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`;
const {testTaxonomy} = await import(`${T}/🗂️taxonomy/🟦️.ts`);
const {inspectTestLayoutSources} = await import(`${T}/🗺️test-layout/🟦️.ts`);
const taxonomy = testTaxonomy(root);
const sources: {path: string; source: string}[] = [];
const directories: string[] = [];
const walk = (abs: string): void => {
  for (const e of readdirSync(abs, {withFileTypes: true})) {
    const p = join(abs, e.name), rel = relative(root, p);
    if (["node_modules", "target", ".git", "dist"].includes(e.name)) continue;
    if (e.isDirectory()) { directories.push(rel); walk(p); }
    else if (e.isFile()) sources.push({path: rel, source: /\.(go|rs|ts|feature)$/u.test(e.name) ? readFileSync(p, "utf8") : ""});
  }
};
for (const base of process.argv.slice(2)) walk(base);
const findings = inspectTestLayoutSources(taxonomy, sources, directories);
const byCode = new Map<string, number>();
for (const f of findings) byCode.set(f.code, (byCode.get(f.code) ?? 0) + 1);
console.log(JSON.stringify({files: sources.length, findings: findings.length, byCode: Object.fromEntries(byCode)}));
for (const f of findings.slice(0, Number(process.env.SHOW ?? 15))) console.log(f.code, f.path, f.line ?? "");
