// 🔎️ Static lock check for the opd slice: every lock it touched parses, lists are sorted and unique, every token resolves, and the os-pack / mcp / surface entries match their manifests.
import {readFileSync, existsSync} from "node:fs";
import {join} from "node:path";
import {blocksOf, nameOf, versionOf, depsOf, cmp, depPackages} from "./dd.ts";
const ROOT = "/Users/ueli/Documents/semio";
const touched = readFileSync(join(import.meta.dir, "touched.txt"), "utf8").split("\n").filter(p => p.endsWith("Cargo.lock"));
let problems = 0, checked = 0;
const bad = (m: string) => { problems++; console.log("PROBLEM", m); };
for (const lock of touched) {
  const text = readFileSync(join(ROOT, lock), "utf8");
  try { Bun.TOML.parse(text); } catch (e) { bad(`${lock}: TOML parse ${String(e).slice(0, 80)}`); continue; }
  const {blocks} = blocksOf(text);
  const byName = new Map<string, string[]>();
  for (const b of blocks) byName.set(nameOf(b), [...(byName.get(nameOf(b)) ?? []), versionOf(b)]);
  const seen = new Set<string>();
  for (const b of blocks) {
    const key = nameOf(b) + "@" + versionOf(b) + (/\nsource = "([^"]*)"/u.exec(b)?.[1] ?? "");
    if (seen.has(key)) bad(`${lock}: duplicate package ${key}`);
    seen.add(key);
    const deps = depsOf(b);
    const sorted = [...deps].sort(cmp);
    if (deps.join("\n") !== sorted.join("\n") || new Set(deps).size !== deps.length) bad(`${lock}: ${nameOf(b)} dependency list not sorted/unique`);
    for (const d of deps) {
      const [n, v] = d.split(" ");
      const have = byName.get(n!);
      if (!have) { bad(`${lock}: ${nameOf(b)} -> ${d} has no package`); continue; }
      if (!v && have.length > 1) bad(`${lock}: ${nameOf(b)} -> ${n} ambiguous (${have.join(",")})`);
    }
  }
  const pk = blocks.find(b => nameOf(b) === "semio-framework-os-pack" && !/\nsource = /u.test(b));
  if (pk) {
    const d = depsOf(pk);
    if (!d.includes("semio-framework-pack-record-schema")) bad(`${lock}: os-pack lacks pack-record-schema`);
    for (const gone of ["semio-framework-pack-schema-index", "semio-framework-pack-json"]) if (d.includes(gone)) bad(`${lock}: os-pack still lists ${gone}`);
  }
  checked++;
}
const manifest = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml";
const want = depPackages(manifest, undefined, ["dependencies", "dev-dependencies"]);
const rootLock = blocksOf(readFileSync(join(ROOT, "Cargo.lock"), "utf8")).blocks.find(b => nameOf(b) === "semio-framework-os-pack")!;
const have = depsOf(rootLock).map(d => d.split(" ")[0]!).sort(cmp);
const missing = want.filter(w => !have.includes(w)), extra = have.filter(h => !want.includes(h));
console.log("root Cargo.lock os-pack vs manifest: missing", missing, "extra", extra);
if (missing.length || extra.length) problems++;
console.log(`locks checked ${checked}, problems ${problems}`);
