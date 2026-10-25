import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { parse } from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const files = execSync("git ls-files -z -- '*Cargo.toml'", { cwd: "/Users/ueli/Documents/semio", maxBuffer: 1 << 28 }).toString().split("\0").filter(Boolean);
const re = /(^|[-_])(test|tests|testing|testkit)([-_]|$)/;
let hits = 0;
const shown: string[] = [];
const testOwned = (path: string): boolean => path.split("/").slice(0, -1).some((seg) => ["🧪️tests", "🧫️fixtures", "📚️examples", "🔮️oracles", "🔬️probes", "🏭️generator"].includes(seg)) || path.startsWith(".🧬semio/");
for (const f of files) {
  if (testOwned(f)) continue;
  let t: any; try { t = parse(readFileSync(`/Users/ueli/Documents/semio/${f}`, "utf8")); } catch { continue; }
  const tables: any[] = [t.dependencies, ...Object.values<any>(t.target ?? {}).map((x) => x.dependencies)].filter(Boolean);
  for (const table of tables) for (const [name, spec] of Object.entries<any>(table)) {
    const feats: string[] = typeof spec === "object" ? spec.features ?? [] : [];
    const bad = feats.filter((x) => re.test(x));
    if (bad.length) { hits += 1; if (shown.length < 12) shown.push(`${f.slice(-90)} -> ${name} ${bad.join(",")}`); }
  }
}
console.log("manifests with a test-class feature enabled in non-dev dependencies:", hits);
console.log(shown.join("\n"));
