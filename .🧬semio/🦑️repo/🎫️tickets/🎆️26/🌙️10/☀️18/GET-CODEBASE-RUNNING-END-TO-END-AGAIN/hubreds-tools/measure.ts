import { readFileSync } from "node:fs";
import { dirname, relative, sep } from "node:path";
const [root, entry, prep] = process.argv.slice(2) as [string, string, string];
const pending = new Set([entry, prep]), seen = new Set<string>(), res: {source:string;specifier:string;selected:string}[] = [];
for (const path of pending) {
  if (seen.has(path)) continue; seen.add(path);
  if (!/\.[cm]?[jt]sx?$/.test(path)) continue;
  const bytes = readFileSync(path);
  const imports = new Bun.Transpiler({ loader: path.endsWith("tsx") ? "tsx" : path.endsWith("jsx") ? "jsx" : path.endsWith("ts") ? "ts" : "js" }).scanImports(bytes.toString("utf8").replace(/^#![^\r\n]*/, ""));
  for (const edge of imports) {
    if ((edge.kind !== "import-statement" && edge.kind !== "require-call") || edge.path.startsWith("node:") || edge.path.startsWith("bun:")) continue;
    const selected = Bun.resolveSync(edge.path, dirname(path));
    res.push({ source: path, specifier: edge.path, selected });
    if (relative(root, selected).split(sep).includes("node_modules")) continue;
    pending.add(selected);
  }
}
const text = JSON.stringify(res);
console.log(JSON.stringify({ edges: res.length, files: seen.size, chars: text.length, bytes: Buffer.byteLength(text) }));
