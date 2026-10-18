import { readFileSync, readdirSync, existsSync, lstatSync, realpathSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join, relative, resolve, sep, isAbsolute } from "node:path";
const [moduleRootArg, out] = process.argv.slice(2);
const repo = "/Users/ueli/Documents/semio";
const tax = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const layout = { tests: tax.testsDirName, impl: "🐹️.go", opaque: [tax.testFixturesDirName, tax.packagesDirName], sources: tax.semanticDirectoryMemberKinds["members-of-members-of-members-of-modules"].memberNames };
function plan(moduleRoot) {
  const root = realpathSync(moduleRoot), src = {}, tst = {};
  const map = (owner, source, test) => {
    const id = createHash("sha256").update(relative(root, source).split(sep).join("/")).digest("hex").slice(0, 16);
    (test ? tst : src)[join(owner, `zz_semio_${id}${test ? "_test" : ""}.go`)] = source;
  };
  const walk = (dir) => {
    if (dir !== root && existsSync(join(dir, "go.mod"))) return;
    for (const e of readdirSync(dir, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      if (!e.isDirectory() || e.isSymbolicLink()) continue;
      const p = join(dir, e.name);
      if (layout.sources.includes(e.name)) { { const s = join(p, layout.impl); if (existsSync(s) && lstatSync(s).isFile()) map(dir, s, false); } continue; }
      if (e.name === layout.tests) {
        for (const c of readdirSync(p, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
          if (!c.isDirectory() || c.isSymbolicLink()) continue;
          const s = join(p, c.name, layout.impl); if (existsSync(s) && lstatSync(s).isFile()) map(dir, s, true);
        }
        continue;
      }
      if ([".git", "node_modules", "target", "vendor"].includes(e.name) || layout.opaque.includes(e.name)) continue;
      walk(p);
    }
  };
  walk(root);
  return { src, tst };
}
function replacements(root) {
  const gm = join(root, "go.mod"); const r = [];
  if (!existsSync(gm)) return r;
  for (const raw of readFileSync(gm, "utf8").split(/\r?\n/)) {
    const l = raw.replace(/\/\/.*$/, "").trim(); const m = l.match(/^replace\s+\S+\s+=>\s+(\S+)/);
    if (m && (m[1].startsWith(".") || isAbsolute(m[1])) && existsSync(join(resolve(root, m[1]), "go.mod"))) r.push(realpathSync(resolve(root, m[1])));
  }
  return r;
}
const root = realpathSync(moduleRootArg);
const primary = plan(root); const all = { ...primary.src }; const seen = new Set([root]);
const add = (o) => { for (const d of replacements(o)) { if (seen.has(d)) continue; seen.add(d); Object.assign(all, plan(d).src); add(d); } };
add(root);
Object.assign(all, primary.tst);
writeFileSync(out, JSON.stringify({ Replace: all }, null, 1));
console.log(Object.keys(primary.tst).length, "test files", Object.keys(primary.src).length, "primary src", Object.keys(all).length, "total");
