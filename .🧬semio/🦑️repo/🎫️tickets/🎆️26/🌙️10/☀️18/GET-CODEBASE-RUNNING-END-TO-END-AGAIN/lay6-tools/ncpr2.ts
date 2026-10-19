import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const ROOT = "/Users/ueli/Documents/semio/";
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const nearest = (p: string): string | null => { let d = dirname(p); while (d.length > 0 && d !== ".") { if (existsSync(ROOT + d + "/package.json")) return d; d = dirname(d); } return existsSync(ROOT + "package.json") ? "" : null; };
const deps = (d: string): Set<string> => { const j = JSON.parse(readFileSync(ROOT + (d ? d + "/" : "") + "package.json", "utf8")); return new Set([...Object.keys(j.dependencies ?? {}), ...Object.keys(j.devDependencies ?? {}), ...Object.keys(j.peerDependencies ?? {})]); };
const v = (r.summary.violations as any[]).filter((x) => x.rule.name.startsWith("no-cross-package-relative-") && !x.from.startsWith(rend) && x.to.startsWith(OS));
const pkg = (x: any) => x.rule.name.replace("no-cross-package-relative-", "");
const s = (p: string) => p.replace(OS, "");
const rows: Record<string, number> = {};
for (const x of v) {
  const n = nearest(x.from); const listed = n !== null && deps(n).has(pkg(x));
  const key = `${pkg(x)} | from-pkg=${n === null ? "?" : s(n + "/") || "(os-root)"} | listed=${listed}`;
  rows[key] = (rows[key] ?? 0) + 1;
}
for (const [k, c] of Object.entries(rows).sort((a, b) => b[1] - a[1])) console.log(c, k);
