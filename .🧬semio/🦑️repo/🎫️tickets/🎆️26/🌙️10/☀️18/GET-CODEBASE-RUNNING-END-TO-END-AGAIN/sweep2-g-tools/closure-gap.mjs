// sweep2-g: the REAL command-import parser of the inference (⚡️caching/📥️inference commandSourceImports) applied to every module of a key list: relative imports that resolve to a workspace file outside the list and outside the scheduling prefixes = what a fresh closure walk would add. usage: bun closure-gap.mjs <key-list> ; streams, RSS guard 500 MB.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, normalize} from "node:path";
const ROOT = "/Users/ueli/Documents/semio", L = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const {commandSourceImports} = await import(L + "/⚡️caching/📥️inference/🟨️.mjs");
const {buildSchedulingPrefixesV1} = await import(L + "/🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts");
const prefixes = buildSchedulingPrefixesV1();
const files = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean), set = new Set(files);
const gaps = new Map();
for (const file of files) {
  if (!/\.(mjs|ts|tsx|js|cjs)$/u.test(file) || /(🧪|🧫)/u.test(file)) continue;
  let imports; try { imports = commandSourceImports(join(ROOT, file), readFileSync(join(ROOT, file), "utf8")); } catch { continue; }
  for (const spec of imports) {
    if (!spec.startsWith(".")) continue;
    const target = normalize(join(dirname(file), spec));
    if (set.has(target) || !existsSync(join(ROOT, target)) || prefixes.some((prefix) => target.startsWith(prefix))) continue;
    (gaps.get(target) ?? gaps.set(target, []).get(target)).push(file);
  }
  if (process.memoryUsage().rss > 500e6) { console.error("RSS above 500 MB"); process.exit(3); }
}
for (const [target, importers] of [...gaps].sort()) console.log(target, "<=", importers[0].slice(-70), importers.length > 1 ? `(+${importers.length - 1})` : "");
console.error(`${gaps.size} gaps of ${files.length} files`);
