/** 📋️ hr2: lists the package.json of every workspace entry of every scope (ignored generated ones included). Usage: bun wsmanifests.ts */
import { existsSync, readFileSync } from "node:fs";
import { join, normalize, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const out = new Set<string>();
for (const scope of [".", "✏️s", "🌎️hub", "♻️mit-bestand", "🏢️semio-tech", "🎓️teaching"]) {
  const manifest = join(ROOT, scope, "package.json");
  if (!existsSync(manifest)) continue;
  const doc = JSON.parse(readFileSync(manifest, "utf8"));
  for (const ws of doc.workspaces ?? []) {
    if (typeof ws !== "string" || /[*!]/u.test(ws)) continue;
    const candidate = join(normalize(join(ROOT, scope, ws)), "package.json");
    if (existsSync(candidate)) out.add(relative(ROOT, candidate));
  }
}
console.log([...out].sort().join("\n"));
