/** 🔍 Walks the static import closure of the given entries with Bun's resolver and lists every specifier that does not resolve.
 * Usage: `bun check-closure.ts <entry>...` (paths relative to the repo root). Relative and bare specifiers are both resolved from the importing file. */
import { readFileSync, existsSync, statSync } from "node:fs";
import { dirname, join, resolve, extname } from "node:path";

const root = "/Users/ueli/Documents/semio";
const entries = process.argv.slice(2).map((p) => resolve(root, p));
const seen = new Set<string>();
const failures = new Map<string, Set<string>>();
const queue = [...entries];
const parent = new Map<string, string>();
const loaderFor = (file: string) => ({ ".ts": "ts", ".tsx": "tsx", ".mts": "ts", ".cts": "ts", ".js": "js", ".mjs": "js", ".cjs": "js", ".jsx": "jsx" } as Record<string, "ts" | "tsx" | "js" | "jsx">)[extname(file)];
let walked = 0;
while (queue.length) {
  const file = queue.pop()!;
  if (seen.has(file)) continue;
  seen.add(file);
  const loader = loaderFor(file);
  if (!loader || file.includes("/node_modules/") || file.includes("/.cache/")) continue;
  let text: string;
  try { text = readFileSync(file, "utf8"); } catch { continue; }
  walked++;
  const imports: { path: string }[] = [];
  const body = text.replace(/\/\*[\s\S]*?\*\//g, "").split("\n").filter((line) => !/^\s*\/\//.test(line)).join("\n");
  for (const m of body.matchAll(/(?:\bfrom\s*|\bimport\s*\(?\s*|\brequire\s*\(\s*|\bimport\.meta\.resolve\s*\(\s*)(["'])([^"'\n]+)\1/g)) { if (/^(?:\.{1,2}\/|@[a-z0-9][\w.-]*\/|[a-z0-9][\w.-]*(?:\/|$))[^\s,;()\[\]{}|]*$/i.test(m[2]!) || m[2] === "..") imports.push({ path: m[2]! }); }
  for (const { path } of imports) {
    if (path.startsWith("node:") || path.startsWith("bun:") || path === "bun") continue;
    let resolved: string;
    try { resolved = Bun.resolveSync(path, dirname(file)); } catch {
      const set = failures.get(path) ?? failures.set(path, new Set()).get(path)!;
      set.add(file);
      continue;
    }
    if (resolved.includes("/node_modules/") || resolved.includes("/.cache/")) continue;
    if (!seen.has(resolved) && !(process.env.SKIP && resolved.includes(process.env.SKIP))) { queue.push(resolved); if (!parent.has(resolved)) parent.set(resolved, file); }
  }
}
if (process.env.TRACE) for (const f of seen) if (f.includes(process.env.TRACE)) { const chain = [f.slice(root.length + 1)]; for (let c = f, n = 0; parent.has(c) && n < 40; n++) { c = parent.get(c)!; chain.push(c.slice(root.length + 1)); } console.log("TRACE " + chain.map((x) => x.split("/").slice(-3).join("/")).join("\n   <- ")); }
console.log(`walked ${walked} first-party files from ${entries.length} entries`);
for (const [spec, files] of [...failures].sort()) console.log(`UNRESOLVED ${spec}\n${[...files].slice(0, 4).map((f) => `    ${f.slice(root.length + 1)}`).join("\n")}${files.size > 4 ? `\n    (+${files.size - 4} more)` : ""}`);
if (process.env.CHAIN) for (const files of failures.values()) for (const f of [...files].slice(0, 1)) { const chain = [f]; for (let c = resolve(root, f), n = 0; parent.has(c) && n < 30; n++) { c = parent.get(c)!; chain.push(c.slice(root.length + 1)); } console.log("  CHAIN " + chain.map((x) => x.slice(-60)).join(" <- ")); }
console.log(failures.size ? `${failures.size} unresolved specifier(s)` : "all specifiers resolve");
process.exit(failures.size ? 1 : 0);
