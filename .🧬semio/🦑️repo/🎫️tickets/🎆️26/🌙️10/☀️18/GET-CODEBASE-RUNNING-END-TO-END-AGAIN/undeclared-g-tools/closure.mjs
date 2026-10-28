// undeclared-g closure: the target closure (dependsOn, transitive) of root targets from the sealed generation; prints one JSON row per target with cache flag, execution, inputs. Offline, read-only. RSS guard 500 MB.
// usage: bun closure.mjs <out.jsonl> <root project:target>...
import {readFileSync, writeFileSync, appendFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const generations = "/Users/ueli/Documents/semio/.cache/b/g/m/generations/";
const [out, ...roots] = process.argv.slice(2);
const cur = JSON.parse(readFileSync(generations + "current.json", "utf8"));
const manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
const cache = new Map();
const fragment = (name) => { if (!cache.has(name)) { const row = manifest.owners[name]; cache.set(name, row && row.kind === "project" ? JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + row.fragment + ".json.br")).toString("utf8")) : undefined); if (cache.size > 40) cache.delete(cache.keys().next().value); } return cache.get(name); };
const key = (project, target) => `${project}:${target}`;
const split = (text) => { const at = text.lastIndexOf(":"); return [text.slice(0, at), text.slice(at + 1)]; };
writeFileSync(out, "");
const seen = new Set(), queue = roots.map((root) => root);
const edgesOf = (project) => (fragment(project)?.edges ?? []).filter((edge) => edge.source === project).map((edge) => edge.target);
while (queue.length) {
  const id = queue.shift();
  if (seen.has(id)) continue;
  seen.add(id);
  const [project, target] = split(id);
  const definition = fragment(project)?.node?.targets?.[target];
  if (!definition) { appendFileSync(out, JSON.stringify({id, missing: true}) + "\n"); continue; }
  const next = [];
  for (const item of definition.dependsOn ?? []) {
    const spec = typeof item === "string" ? {target: item} : item;
    const text = spec.target;
    if (text.startsWith("^")) { for (const dependency of (spec.projects && Array.isArray(spec.projects)) ? spec.projects : edgesOf(project)) next.push(key(dependency, text.slice(1))); }
    else if (text.includes(":")) next.push(text);
    else next.push(key(project, text));
  }
  for (const dependency of next) queue.push(dependency);
  appendFileSync(out, JSON.stringify({id, project, target, cache: definition.cache ?? false, root: fragment(project).root, dependsOn: next, execution: definition.execution, configurations: definition.configurations, inputs: definition.inputs, outputs: definition.outputs, metadata: definition.metadata}) + "\n");
  if (process.memoryUsage().rss > 500 * 1048576) { console.error("RSS above 500 MB"); process.exit(3); }
}
console.error(`${seen.size} targets from ${roots.length} roots, generation ${cur.generation}`);
