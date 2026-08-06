// 🔎️ Scratch probe: mirrors root 📜️script.ts's policyCrateEntryPath so every discovered rust package's
// resolved crate-root file can be checked to actually exist on disk (ticket 26/08/06/ROOT-POLICY-…).
import { existsSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { discoverPackages, loadTaxonomy } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🟦️discovery.ts";

const repoRoot = process.cwd();
const taxonomy = loadTaxonomy();

function entryPath(manifestDirRel: string, ownerRel: string): string {
  const manifestAbs = join(repoRoot, manifestDirRel, "Cargo.toml");
  if (existsSync(manifestAbs)) {
    const lines = readFileSync(manifestAbs, "utf8").split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      if (!/^\s*\[\[?(?:lib|bin)\]?\]\s*$/.test(lines[i]!)) continue;
      for (let j = i + 1; j < lines.length && !/^\s*\[/.test(lines[j]!); j++) {
        const declared = lines[j]!.match(/^\s*path\s*=\s*"([^"]+)"\s*$/)?.[1];
        if (declared) return relative(repoRoot, resolve(join(repoRoot, manifestDirRel), declared)).replaceAll("\\", "/");
      }
    }
  }
  for (const entry of taxonomy.ecosystems["🦀️rust"]?.entryFilenames ?? []) {
    if (existsSync(join(repoRoot, manifestDirRel, entry))) return `${manifestDirRel}/${entry}`;
    if (existsSync(join(repoRoot, ownerRel, entry))) return `${ownerRel}/${entry}`;
  }
  return `${manifestDirRel}/📦️lib.rs`;
}

let missing = 0;
for (const pkg of discoverPackages(repoRoot, taxonomy)) {
  if (pkg.lang !== "🦀️rust") continue;
  const resolved = entryPath(pkg.packageRel, pkg.ownerRel);
  const ok = existsSync(join(repoRoot, resolved));
  if (!ok) missing += 1;
  console.log(`${ok ? "OK " : "MISS"} ${pkg.role.padEnd(9)} ${resolved}`);
}
console.log(`\nmissing entry files: ${missing}`);
