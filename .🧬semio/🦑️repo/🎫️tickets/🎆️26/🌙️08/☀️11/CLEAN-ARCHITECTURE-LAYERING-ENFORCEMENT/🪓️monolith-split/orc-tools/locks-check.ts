// 🔎️ Static lock check for the orc slice: every lock it touched parses, lists are sorted and unique, every token resolves (unresolved tokens compared with HEAD), the record-token package is gone, and the os-pack entries match the manifest.
import {readFileSync, existsSync} from "node:fs";
import {join} from "node:path";
import {blocksOf, nameOf, versionOf, depsOf, cmp, depPackages} from "./dd.ts";
const ROOT = "/Users/ueli/Documents/semio";
const PRE = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/orcpre/";
const touched = readFileSync(join(import.meta.dir, "touched.txt"), "utf8").split("\n").filter(p => p.endsWith("Cargo.lock"));
let problems = 0, checked = 0, unresolvedNow = 0, unresolvedHead = 0;
const bad = (m: string) => { problems++; console.log("PROBLEM", m); };
const unresolved = (text: string): number => {
  const {blocks} = blocksOf(text);
  const byName = new Map<string, string[]>();
  for (const b of blocks) byName.set(nameOf(b), [...(byName.get(nameOf(b)) ?? []), versionOf(b)]);
  let n = 0;
  for (const b of blocks) for (const d of depsOf(b)) if (!byName.has(d.split(" ")[0]!)) n++;
  return n;
};
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
  }
  if (byName.has("semio-framework-pack-record-token")) bad(`${lock}: record-token package remains`);
  for (const b of blocks) if (depsOf(b).some(d => d.split(" ")[0] === "semio-framework-pack-record-token")) bad(`${lock}: ${nameOf(b)} still lists record-token`);
  unresolvedNow += unresolved(text);
  const backup = PRE + lock;
  if (existsSync(backup)) unresolvedHead += unresolved(readFileSync(backup, "utf8"));
  checked++;
}
const manifest = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml";
const wantAll = depPackages(manifest, undefined, ["dependencies", "dev-dependencies"]);
const wantNormal = depPackages(manifest, undefined, ["dependencies"]);
for (const [lockPath, want] of [["Cargo.lock", wantAll], ["🧰️framework/Cargo.lock", wantAll]] as const) {
  const blocks = blocksOf(readFileSync(join(ROOT, lockPath), "utf8")).blocks;
  const os = blocks.find(b => nameOf(b) === "semio-framework-os-pack" && !/\nsource = /u.test(b));
  if (!os) { console.log(lockPath, "has no os-pack package"); continue; }
  const have = depsOf(os).map(d => d.split(" ")[0]!).sort(cmp);
  const missing = want.filter(w => !have.includes(w)), extra = have.filter(h => !want.includes(h));
  console.log(`${lockPath} os-pack vs manifest: missing`, missing, "extra", extra);
  if (missing.length || extra.length) problems++;
}
console.log(`locks checked ${checked}, unresolved tokens now ${unresolvedNow} vs pre-edit ${unresolvedHead}, problems ${problems}`);
