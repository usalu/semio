import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { inspectRustPathMounts } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { POLICY_SPLIT_SOURCE_ROOTS, POLICY_SKIP_DIRS } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const out: string[] = [];
const walk = (rel: string) => {
  for (const e of readdirSync(join(root, rel), { withFileTypes: true })) {
    const r = rel + "/" + e.name;
    if (e.isDirectory()) { if (!POLICY_SKIP_DIRS.has(e.name) && e.name !== "🧪️tests") walk(r); }
    else if (e.name.endsWith(".rs") && /⁠|🦀️\.rs$/.test(e.name)) {
      const t = readFileSync(join(root, r), "utf8");
      if (!t.includes("#[path")) continue;
      const mounts = inspectRustPathMounts(t).filter((m) => !m.path.includes("🧪️tests"));
      if (mounts.length) out.push(r.slice(1));
    }
  }
};
walk("/✏️s/🔌️plugins");
const reg = new Set(POLICY_SPLIT_SOURCE_ROOTS);
const unreg = out.filter((f) => !reg.has(f));
console.log("plugin files with path mounts", out.length, "unregistered", unreg.length);
for (const f of unreg) console.log("UNREG", f);
