// sweep2-g static-closure: like undeclared-g's (bun's own Transpiler, independent of the repository's closure inference) but a bare specifier that names a WORKSPACE package (`@semio-tech/framework/raster/video/svg-export`) is resolved through the workspace links and the package `exports` (Bun.resolveSync), so the facade modules and their imports join the seed. Registry packages and node: stay ambient. Optional parent map for chains. RSS guard 500 MB.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
export const WORKSPACE = "/Users/ueli/Documents/semio";
const loaders = {".ts": "ts", ".tsx": "tsx", ".mts": "ts", ".cts": "ts", ".js": "js", ".mjs": "js", ".cjs": "js", ".jsx": "jsx"};
export function staticClosure(entries, {dynamic = true, stop = () => false, parents} = {}) {
  const seen = new Set(), queue = entries.map((entry) => resolve(WORKSPACE, entry)), transpilers = {};
  while (queue.length) {
    const file = queue.pop();
    if (seen.has(file) || !existsSync(file)) continue;
    seen.add(file);
    if (process.memoryUsage().rss > 500 * 1048576) throw new Error("RSS above 500 MB");
    const loader = loaders[file.slice(file.lastIndexOf("."))];
    if (!loader || stop(relative(WORKSPACE, file))) continue;
    let imports;
    try { imports = (transpilers[loader] ??= new Bun.Transpiler({loader})).scanImports(readFileSync(file, "utf8").replace(/^#![^\n]*/u, "")); } catch { continue; }
    for (const row of imports) {
      if (row.kind === "dynamic-import" && !dynamic) continue;
      let target;
      if (row.path.startsWith(".") || row.path.startsWith("/")) target = resolve(dirname(file), row.path);
      else if (row.path.startsWith("@semio-tech/")) { try { target = Bun.resolveSync(row.path, dirname(file)); } catch { continue; } }
      else continue;
      if (parents && !parents.has(target)) parents.set(target, file);
      queue.push(target);
    }
  }
  return [...seen].map((file) => relative(WORKSPACE, file)).filter((path) => !path.startsWith("..") && !path.includes("node_modules/")).sort();
}
if (import.meta.main) {
  const [target, ...entries] = process.argv.slice(2), parents = new Map();
  const files = staticClosure(entries, {parents});
  if (!target || target === "-") for (const path of files) console.log(path);
  else { let at = resolve(WORKSPACE, target); const chain = [at]; while (parents.has(at)) { at = parents.get(at); chain.push(at); } console.log(chain.reverse().map((p) => relative(WORKSPACE, p).slice(-90)).join("\n -> ")); }
}
