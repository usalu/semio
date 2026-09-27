#!/usr/bin/env bun
/** 📋️ R9: the smallest set of taxonomy-admissible input patterns (no wildcard within the first N segments, N = the deepest
 * opaque root) that covers every declared project manifest. Prints the patterns (JS-sorted) as JSON. */
const ROOT = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(await Bun.file(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`).text());
const depth = Math.max(...Object.values(taxonomy.pathExclusions as Record<string, { path: string }>).map((entry) => entry.path.replace(/\/$/u, "").split("/").length));
const { declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const patterns = new Set<string>();
for (const { path } of declaredProjectTargets(ROOT) as { path: string }[]) {
  const segments = path === "" ? [] : path.split("/");
  patterns.add(segments.length < depth ? [...segments, "📋️project.json"].join("/") : [...segments.slice(0, depth), "**", "📋️project.json"].join("/"));
}
const sorted = [...patterns].sort();
for (const pattern of sorted) if (!Array.from(new Bun.Glob(pattern).scanSync({ cwd: ROOT, onlyFiles: true })).length) throw new Error(`pattern matches nothing: ${pattern}`);
console.log(JSON.stringify({ depth, count: sorted.length, patterns: sorted }));
