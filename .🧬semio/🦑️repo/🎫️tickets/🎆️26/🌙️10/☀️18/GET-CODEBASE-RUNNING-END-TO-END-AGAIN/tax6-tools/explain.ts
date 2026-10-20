// 🔎️ Explains how the live engine classifies one tracked path: file kind, role context, directory kind of every ancestor and the semantic-stem match. usage: bun explain.ts <repo-relative path>...
import {readdirSync} from "node:fs";
import {join} from "node:path";
const root = "/Users/ueli/Documents/semio";
const norm = join(root, ...["🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🧹️normalization"].map(() => "")) ;
const pick = (dir: string, lead: string): string => join(dir, readdirSync(dir).find((name) => name.startsWith(lead))!);
const L = pick(pick(pick(pick(pick(root, "🧰"), "🛍"), "🦑"), "🔨"), "📚");
const N = pick(L, "🧹");
const {loadTaxonomy} = await import(join(pick(N, "🔣"), "🟦️.ts"));
const {matchDirectoryKind, resolveFileKind} = await import(join(pick(N, "🏷"), "🟦️.ts"));
const {canonicalDirectory} = await import(join(pick(N, "📏"), "🟦️.ts"));
const taxonomy = loadTaxonomy({repoRoot: root, taxonomyPath: join(L, readdirSync(L).find((n) => n.startsWith("🔣") && n.endsWith("taxonomy.json"))!)});
for (const path of process.argv.slice(2)) {
  const parts = path.split("/");
  const kinds = new Map<string, string>();
  let canonical = "", parentKind: string | undefined;
  const ancestors: string[] = [];
  const dirs: string[] = [];
  for (let i = 0; i < parts.length - 1; i++) {
    const p = parts.slice(0, i + 1).join("/");
    const c = canonicalDirectory(p, canonical, parentKind, [...ancestors], taxonomy);
    canonical = c.path;
    if (c.kindId) { kinds.set(p, c.kindId); ancestors.push(c.kindId); parentKind = c.kindId; } else parentKind = undefined;
    if (i >= parts.length - 4) dirs.push(`${parts[i]} => ${c.kindId ?? "UNRESOLVED"}${c.violations.length ? " !" + c.violations.map((v: any) => v.code).join(",") : ""}`);
  }
  const resolved = resolveFileKind(path, taxonomy, parentKind, ancestors);
  console.log(`# ${path}\n  last dirs: ${dirs.join(" | ")}\n  fileKind=${resolved.kind?.id ?? null} role=${resolved.kind?.role} ext=${resolved.extension} stem=${JSON.stringify(resolved.stem)} ambiguous=${resolved.ambiguous.join(",")}`);
  const stem = resolved.stem;
  const ctx = resolved.kind?.role === "asset" ? "assets" : resolved.kind?.role === "test" ? "tests" : parentKind;
  for (const roleContext of new Set([ctx, parentKind])) {
    const m = matchDirectoryKind(stem, taxonomy, roleContext);
    console.log(`  stem match ctx=${roleContext}: ${m.kind?.id ?? "NONE"} ambiguous=${m.ambiguous.join(",")}`);
  }
}
