import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, posix, relative, resolve } from "node:path";
/**
 * usage: bun unfacade.ts <files-or-listfile...> [--apply]
 * Re-points every relative import/export specifier of the named files that resolves to a pure re-export facade
 * (`export * from "<rel>"` / `export { a, b } from "<rel>"` without renames, plus comments) onto the facade's own source,
 * so moved implementation never imports through the package facade that re-exports it.
 */
const root = "/Users/ueli/Documents/semio/";
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const files = args.filter((arg) => arg !== "--apply").flatMap((arg) => arg.endsWith(".txt") ? readFileSync(arg, "utf8").split("\n").filter(Boolean) : [arg]);
const abs = (path: string): string => resolve(root, path);
const facadeSource = (file: string): string | null => {
  let text: string;
  try { if (!statSync(abs(file)).isFile()) return null; text = readFileSync(abs(file), "utf8"); } catch { return null; }
  const stripped = text.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/^\s*\/\/.*$/gmu, "").trim();
  if (!stripped) return null;
  const statements = stripped.split(/;\s*(?:\n|$)/u).map((row) => row.trim()).filter(Boolean);
  const sources = new Set<string>();
  for (const row of statements) {
    const match = /^export\s+(?:type\s+)?(?:\*|\{[^}]*\})\s*from\s*["']([^"']+)["']$/su.exec(row);
    if (!match || /\bas\b/u.test(row)) return null;
    sources.add(match[1]!);
  }
  if (sources.size !== 1) return null;
  const spec = [...sources][0]!;
  return spec.startsWith(".") ? posix.normalize(posix.join(dirname(file), spec)) : null;
};
const specifier = /(\bfrom\s*|\bimport\s*\(\s*|\bimport\s+)(["'])(\.{1,2}\/[^"']+)\2/gu;
let total = 0;
for (const file of files) {
  const text = readFileSync(abs(file), "utf8");
  let changed = 0;
  const out = text.replace(specifier, (whole, head: string, quote: string, spec: string) => {
    let target = posix.normalize(posix.join(dirname(file), spec)), hops = 0;
    for (; hops < 4; hops++) { const next = facadeSource(target); if (!next) break; target = next; }
    if (hops === 0) return whole;
    let replacement = posix.relative(dirname(file), target);
    if (!replacement.startsWith(".")) replacement = `./${replacement}`;
    if (replacement === spec) return whole;
    changed += 1;
    return `${head}${quote}${replacement}${quote}`;
  });
  if (changed > 0) { total += changed; console.log(`${changed}\t${file.replace(/^🧰️framework\/🛍️products\//, "P/")}`); if (apply) writeFileSync(abs(file), out); }
}
console.log(`specifiers re-pointed: ${total}${apply ? " (applied)" : " (dry)"}`);
void existsSync; void relative;
