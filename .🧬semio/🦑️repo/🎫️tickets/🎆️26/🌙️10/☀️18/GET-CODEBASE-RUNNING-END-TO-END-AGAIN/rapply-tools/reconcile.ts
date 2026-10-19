import { resolve } from "node:path";
import { existsSync } from "node:fs";
import { packageIndex } from "../repoapi-tools/packages.ts";
import { ROOT, readJson } from "../repoapi-tools/lib.ts";
const final = readJson<any>(resolve(import.meta.dir, "../repoapi-tools/exports-final.json"));
const index = packageIndex();
const norm = (t: unknown) => typeof t === "string" ? t.replace(/\/\/+/g, "/") : JSON.stringify(t);
const counts = { ok: 0, missing: [] as string[], differs: [] as string[], noPackage: [] as string[], glueMissing: [] as string[] };
const add = (pkg: string, sub: string, target: string, glue?: string) => {
  const live = index.byName.get(pkg);
  if (!live) { counts.noPackage.push(pkg); return; }
  if (!(sub in live.exports)) { counts.missing.push(`${pkg} ${sub} -> ${target}`); return; }
  if (norm(live.exports[sub]) !== norm(target)) { counts.differs.push(`${pkg} ${sub}: live ${JSON.stringify(live.exports[sub])} spec ${JSON.stringify(target)}`); return; }
  counts.ok++;
  const abs = resolve(ROOT, live.dir, norm(target));
  if (!existsSync(abs)) counts.glueMissing.push(`${pkg} ${sub} -> ${abs}`);
};
for (const [pj, m] of Object.entries<any>(final.exportsToAdd)) {
  const pkgJson = readJson<any>(resolve(ROOT, pj));
  for (const [sub, target] of Object.entries<string>(m)) add(pkgJson.name, sub, target);
}
console.log(JSON.stringify({ ok: counts.ok, missing: counts.missing, differs: counts.differs, noPackage: [...new Set(counts.noPackage)], glueMissing: counts.glueMissing }, null, 1));
