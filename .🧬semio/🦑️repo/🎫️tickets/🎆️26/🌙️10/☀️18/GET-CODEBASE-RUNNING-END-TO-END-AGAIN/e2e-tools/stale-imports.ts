/** 🩹 Finds (and with `--apply` repairs) relative imports of the retired path `🧰️framework/📦️packages/🟦️typescript/🛍️products/🦑️repo/`
 * whose owner now lives at `🧰️framework/🛍️products/🦑️repo/`. Fails closed: a specifier is rewritten only when the new target is
 * an existing file; everything else is listed. Usage: `bun stale-imports.ts [--apply] [<path-prefix>…]` (default: whole repository). */
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const root = "/Users/ueli/Documents/semio";
const stale = "🧰️framework/📦️packages/🟦️typescript/🛍️products/🦑️repo/";
const fresh = "🧰️framework/🛍️products/🦑️repo/";
const apply = process.argv.includes("--apply");
const prefixes = process.argv.slice(2).filter((argument) => !argument.startsWith("--"));
const isFile = (path: string) => existsSync(path) && statSync(path).isFile();
const resolves = (from: string, specifier: string) => [specifier, `${specifier}.ts`, `${specifier}.tsx`, `${specifier}/🟦️.ts`].some((candidate) => isFile(resolve(root, dirname(from), candidate)));
const files = execFileSync("rg", ["-l", "--no-messages", "-F", stale, "-g", "*.ts", "-g", "*.tsx", "-g", "*.mjs", "-g", "!node_modules", "-g", "!.🧬semio", "-g", "!dist", "-g", "!.cache", "."], { cwd: root, maxBuffer: 1 << 26 }).toString().split("\n").filter(Boolean).map((file) => file.replace(/^\.\//, "")).filter((file) => prefixes.length === 0 || prefixes.some((prefix) => file.startsWith(prefix)));
let rewritten = 0;
const remaining: string[] = [];
for (const file of files) {
  const text = readFileSync(resolve(root, file), "utf8");
  const next = text.replace(/"(\.[^"\n]*)"/g, (whole, specifier: string) => {
    if (!specifier.includes(stale)) return whole;
    const replaced = specifier.replace(stale, fresh);
    if (resolves(file, replaced)) { rewritten++; return `"${replaced}"`; }
    remaining.push(`${file} -> ${specifier.slice(specifier.indexOf("🧰️framework"))}`);
    return whole;
  });
  if (apply && next !== text) writeFileSync(resolve(root, file), next);
}
console.log(`${files.length} files; ${apply ? "rewrote" : "would rewrite"} ${rewritten} specifiers; ${remaining.length} stay unresolved`);
for (const line of remaining) console.log(line);
