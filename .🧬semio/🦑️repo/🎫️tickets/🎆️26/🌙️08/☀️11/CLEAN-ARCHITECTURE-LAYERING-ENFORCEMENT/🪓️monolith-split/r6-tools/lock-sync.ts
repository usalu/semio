// 🔒️ Adds the carved crates of one artifact facade to every other lockfile that lists the facade (path crates of non-member workspaces carry normal dependencies only).
import {readFileSync, writeFileSync, mkdirSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {patchCargoLock} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [facadeManifest, facadeName, backup, ...locks] = process.argv.slice(2);
const facade = Bun.TOML.parse(readFileSync(join(ROOT, facadeManifest!), "utf8")) as any;
const names = Object.keys(facade.dependencies ?? {}).filter((n) => n.startsWith(`${facadeName}-`));
const manifestOf = new Map<string, any>();
const dirs: string[] = [];
const wsDeps = (Bun.TOML.parse(readFileSync(join(ROOT, dirname(dirname(dirname(facadeManifest!))), "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const carved: {name: string; dependsOn: string[]}[] = [];
const seen = new Set<string>();
const visit = (name: string, dir: string) => {
  if (seen.has(name)) return; seen.add(name);
  const m = Bun.TOML.parse(readFileSync(join(dir, "Cargo.toml"), "utf8")) as any;
  const deps: string[] = [];
  for (const [dep, spec] of Object.entries<any>(m.dependencies ?? {})) {
    if (!dep.startsWith(`${facadeName}-`)) continue;
    const path = spec.path ? resolve(dir, spec.path) : resolve(dir, m.package.workspace ?? "", wsDeps[dep]?.path ?? "");
    deps.push(dep); visit(dep, path);
  }
  carved.push({name, dependsOn: deps});
};
const facadeDir = join(ROOT, dirname(facadeManifest!));
for (const [dep, spec] of Object.entries<any>(facade.dependencies ?? {})) if (dep.startsWith(`${facadeName}-`)) visit(dep, resolve(facadeDir, spec.path));
console.log("carved", carved.length);
for (const lock of locks) {
  const abs = join(ROOT, lock);
  const text = readFileSync(abs, "utf8");
  const b = join(ROOT, backup!, lock);
  if (!existsSync(b)) { mkdirSync(dirname(b), {recursive: true}); writeFileSync(b, text); }
  writeFileSync(abs, patchCargoLock(text, facadeName!, carved));
  console.log("patched", lock);
}
