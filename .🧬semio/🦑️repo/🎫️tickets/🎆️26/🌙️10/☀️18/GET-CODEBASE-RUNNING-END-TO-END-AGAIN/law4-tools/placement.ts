import { existsSync, readFileSync, readdirSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const plugins = join(root, "✏️s/🔌️plugins");
const fixture = JSON.parse(readFileSync(join(plugins, "🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json"), "utf8"));
const parts = (file: string): string[] => file.split(/[\\/]/u);
const opaque = new Set(["node_modules", "target", "dist", ".git", ".nx", "🗑️generated", "🤖️generated"]);
const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() && !opaque.has(entry.name) ? walk(join(directory, entry.name)) : entry.isFile() ? [join(directory, entry.name)] : []);
const files = fixture.owners.flatMap((owner: string) => walk(join(plugins, owner)));
const accepted = (file: string): boolean => { const path = parts(file), tests = path.indexOf("🧪️tests"), examples = path.indexOf("🧫️fixtures"); return tests >= 0 && examples > tests && !path.slice(0, examples).some(part => ["🧬️schema", "🖼️assets"].includes(part)); };
const rel = (f: string) => relative(root, f);
console.log("== rule1: fixtures not beneath tests");
for (const file of files.filter((f: string) => parts(f).includes("🧫️fixtures"))) { let ok = false; try { ok = accepted(realpathSync(file)); } catch { ok = false; } if (!ok || file.length > 256) console.log("  ", rel(file), file.length > 256 ? "(len)" : ""); }
console.log("== rule2: include_str in rs");
for (const file of files.filter((f: string) => f.endsWith(".rs"))) {
  const source = readFileSync(file, "utf8");
  for (const match of source.matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
    const target = resolve(dirname(file), match[1]!);
    if (!existsSync(target)) { console.log("  MISSING", rel(file), "=>", match[1]); continue; }
    const coordinate = relative(plugins, realpathSync(target));
    if (!coordinate.startsWith("..") && !isAbsolute(coordinate) && fixture.owners.includes(parts(coordinate)[0]!) && !accepted(realpathSync(target))) console.log("  UNPLACED", rel(file), "=>", rel(target));
  }
}
console.log("== rule3: ts imports");
for (const file of files.filter((f: string) => f.endsWith(".ts") || f.endsWith(".tsx"))) {
  for (const match of readFileSync(file, "utf8").matchAll(/(?:from\s*|import\(\s*)["']([^"']*🧫️fixtures[^"']*)["']/gu)) {
    const target = resolve(dirname(file), match[1]!);
    const coordinate = relative(plugins, target);
    if (!coordinate.startsWith("..") && !isAbsolute(coordinate) && fixture.owners.includes(parts(coordinate)[0]!)) {
      const problems = [] as string[];
      if (!parts(file).includes("🧪️tests")) problems.push("notInTests");
      if (!match[1]!.startsWith(".")) problems.push("notRelative");
      if (!existsSync(target)) problems.push("missing"); else if (!accepted(realpathSync(target))) problems.push("unplaced");
      if (problems.length) console.log("  ", problems.join(","), rel(file), "=>", match[1]);
    }
  }
}
console.log("== rule4: manifest readers");
const repository = root;
for (const row of fixture.manifestReaders) {
  const file = join(repository, row.path), manifest = join(repository, row.manifest);
  if (!existsSync(manifest)) { console.log("  MANIFEST MISSING", row.manifest); continue; }
  if (!existsSync(file)) { console.log("  FILE MISSING", row.path); continue; }
  for (const match of readFileSync(file, "utf8").matchAll(/concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
    const target = resolve(dirname(manifest) + match[1]!);
    if (!existsSync(target)) console.log("  MISSING", row.path, "=>", target.replace(root + "/", ""));
    else if (!accepted(realpathSync(target))) console.log("  UNPLACED", row.path, "=>", target.replace(root + "/", ""));
  }
}
