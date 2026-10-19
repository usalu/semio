import { readFileSync, existsSync } from "node:fs";
import { dirname } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const nearest = (file: string): string | null => { let d = dirname(ROOT + file); while (d.length > ROOT.length - 1) { if (existsSync(d + "/package.json")) return d + "/package.json"; d = dirname(d); } return null; };
const gaps = new Map<string, Set<string>>();
const files = new Set<string>();
for (const f of plan.fixes) {
  files.add(f.from);
  const pj = nearest(f.from); if (!pj) continue;
  const j = JSON.parse(readFileSync(pj, "utf8"));
  const deps = { ...j.dependencies, ...j.devDependencies, ...j.peerDependencies };
  if (!(f.pkg in deps)) { const k = pj.slice(ROOT.length); (gaps.get(k) ?? gaps.set(k, new Set()).get(k)!).add(f.pkg); }
}
console.log("fixed files", files.size);
for (const [k, v] of gaps) console.log(k.replace(/^🧰️framework\/🛍️products\/💻️os\//, "OS/"), "::", [...v].join(", "));
