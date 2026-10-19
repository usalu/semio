import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { initParser, findWasm } from "../rs-sweep-tools/parse.ts";
import { Package, loadPackage } from "../rs-sweep-tools/cargo.ts";
import { loadOwners, ROOT } from "./resolve-owner.ts";
import { planManifests } from "./manifests.ts";
import { Moved } from "./transform.ts";

const apply = process.argv.includes("--apply");
const G = `${import.meta.dir}/../🗑️generated/value-repoint`;
const moved: Moved = JSON.parse(readFileSync(`${import.meta.dir}/moved.json`, "utf8"));
await initParser(findWasm(ROOT)!);
const libs = [...new Set<string>([...Object.values(moved.modules), ...Object.values(moved.functions)])];
const owners = loadOwners(libs);
const fileMap: Record<string, string[]> = JSON.parse(readFileSync(`${G}/filemap.json`, "utf8"));
const ledger: Record<string, string> = JSON.parse(readFileSync(`${G}/backup/ledger.json`, "utf8"));
const extra = existsSync(`${G}/macro-files.txt`) ? readFileSync(`${G}/macro-files.txt`, "utf8").split("\n").filter(Boolean).map((f) => resolve(ROOT, f)) : [];
const byPkg = new Map<string, { pkg: Package; libs: Set<string> }>();
const used = (src: string, lib: string) => new RegExp(`\\b${lib}\\b`).test(src);
let files = 0;
const lone: string[] = [];
for (const f of new Set([...Object.keys(ledger), ...extra])) {
  if (!existsSync(f)) continue;
  const now = readFileSync(f, "utf8");
  const before = ledger[f] ? readFileSync(`${G}/backup/${ledger[f]}`, "utf8") : "";
  const need = libs.filter((l) => used(now, l) && !used(before, l));
  if (!need.length) continue;
  files++;
  const dirs = fileMap[resolve(f)] ?? [];
  if (!dirs.length) lone.push(f.replace(ROOT + "/", ""));
  for (const d of dirs) {
    const p = loadPackage(d);
    if (!p) continue;
    const e = byPkg.get(p.dir) ?? { pkg: p, libs: new Set<string>() };
    for (const l of need) e.libs.add(l);
    byPkg.set(p.dir, e);
  }
}
const plan = planManifests(byPkg, owners, apply, G);
writeFileSync(`${G}/deps-from-ledger.json`, JSON.stringify({ files, packages: byPkg.size, plan, lone }, null, 1));
console.log(JSON.stringify({ apply, files, packages: byPkg.size, edited: plan.edited, wsDecls: plan.wsDecls, issues: plan.issues, lone }, null, 1));
writeFileSync(`${G}/pkg-libs.json`, JSON.stringify([...byPkg.values()].map((e) => ({ name: e.pkg.name, dir: e.pkg.dir.replace(ROOT + "/", ""), libs: [...e.libs].map((l) => l.replace("semio_framework_value_", "")) }))));
