// undeclared-g static-closure: the workspace files a set of entry modules load by STATIC (and literal dynamic) relative imports, scanned with bun's own Transpiler (an implementation independent of the repository's closure inference). Bare/node: specifiers are ambient. Streams: one visited set, files read once, RSS guard 500 MB.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
export const WORKSPACE = "/Users/ueli/Documents/semio";
const loaders = {".ts": "ts", ".tsx": "tsx", ".mts": "ts", ".cts": "ts", ".js": "js", ".mjs": "js", ".cjs": "js", ".jsx": "jsx"};
export function staticClosure(entries, {dynamic = true, stop = () => false} = {}) {
  const seen = new Set(), queue = entries.map((entry) => resolve(WORKSPACE, entry)), transpilers = {};
  while (queue.length) {
    const file = queue.pop();
    if (seen.has(file) || !existsSync(file)) continue;
    seen.add(file);
    if (process.memoryUsage().rss > 500 * 1048576) throw new Error("RSS above 500 MB");
    const extension = file.slice(file.lastIndexOf("."));
    const loader = loaders[extension];
    if (!loader || stop(relative(WORKSPACE, file))) continue;
    let imports;
    try { imports = (transpilers[loader] ??= new Bun.Transpiler({loader})).scanImports(readFileSync(file, "utf8").replace(/^#![^\n]*/u, "")); } catch { continue; }
    for (const row of imports) {
      if (!row.path.startsWith(".") && !row.path.startsWith("/")) continue;
      if (row.kind === "dynamic-import" && !dynamic) continue;
      queue.push(resolve(dirname(file), row.path));
    }
  }
  return [...seen].map((file) => relative(WORKSPACE, file)).filter((path) => !path.startsWith("..") && !path.includes("node_modules/")).sort();
}
if (import.meta.main) for (const path of staticClosure(process.argv.slice(2))) console.log(path);
