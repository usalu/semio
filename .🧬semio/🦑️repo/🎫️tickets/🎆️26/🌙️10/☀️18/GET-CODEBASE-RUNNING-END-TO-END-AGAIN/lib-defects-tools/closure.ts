import { dirname, join, relative, resolve } from "node:path";
import { existsSync, readFileSync } from "node:fs";

/** 🔗 Prints the repository-relative local import closure of the entry files given as arguments, plus each file's sibling schema directory. */
const root = resolve(process.argv[2]!);
const seen = new Set<string>();
const pending = process.argv.slice(3);
while (pending.length) {
  const path = pending.pop()!;
  if (seen.has(path)) continue;
  seen.add(path);
  if (!/\.(ts|mjs|js|cjs|tsx)$/u.test(path)) continue;
  let text = "";
  try { text = readFileSync(join(root, path), "utf8"); } catch { continue; }
  for (const match of text.matchAll(/(?:from|import\(|import)\s*["']([^"']+)["']/gu)) {
    const specifier = match[1]!;
    if (specifier.startsWith("node:") || specifier.startsWith("bun:")) continue;
    try {
      const resolved = relative(root, Bun.resolveSync(specifier, dirname(join(root, path)))).replaceAll("\\", "/");
      if (!resolved.startsWith("..") && !resolved.split("/").includes("node_modules")) pending.push(resolved);
    } catch {}
  }
}
const schemas = new Set<string>();
for (const path of seen) {
  const schema = (dirname(path) === "." ? "" : dirname(path) + "/") + "🧬️schema";
  if (existsSync(join(root, schema))) schemas.add(schema);
}
console.log(JSON.stringify({ files: [...seen].filter((path) => existsSync(join(root, path))), schemas: [...schemas] }));
