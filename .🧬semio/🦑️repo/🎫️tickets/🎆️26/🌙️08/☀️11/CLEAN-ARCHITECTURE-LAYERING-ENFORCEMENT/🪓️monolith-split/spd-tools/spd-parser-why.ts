/** 🔍️ For every refused `parser <Export> <leaf>` row of a dump (not the `needs` cascade rows) finds the smallest schema node the parser renderer cannot express: `bun spd-parser-why.ts <dump.json> <out.json>`. */
import { readFileSync, writeFileSync } from "node:fs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SchemaCatalog, SourceStore, readDocument, exportsOf, buildOwnerIndex, facetOf, leafPath, helperPrefix } = await import(`${base}/🗂️documents/🟦️.ts`);
const { Context, parserExpression } = await import(`${base}/🧾️parser-render/🟦️.ts`);
const { existingHelper } = await import(`${base}/📖️declared-types/🟦️.ts`);
const catalog = new SchemaCatalog(root);
const index = buildOwnerIndex(root, catalog);
const sources = new SourceStore();
const rows: string[] = JSON.parse(readFileSync(process.argv[2]!, "utf8")).refused;
const out: [string, string, string, string][] = [];
const isDict = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);
for (const row of rows) {
  const m = /^parser (\S+) (\S+)( needs .*)?$/u.exec(row);
  if (m === null || m[3] !== undefined) continue;
  const [, name, leaf] = m;
  const directory = leaf!.slice(0, leaf!.lastIndexOf("/"));
  const docPath = `${directory}/🔣️.json`;
  const document = readDocument(root, docPath);
  if (document === undefined) { out.push([name!, leaf!, "no-document", ""]); continue; }
  const definition = exportsOf(document).get(name!);
  if (definition === undefined) { out.push([name!, leaf!, "no-export", ""]); continue; }
  const target = `${root}/${leaf}`;
  const source = sources.read(target);
  const helper = existingHelper(source) ?? helperPrefix(document.$id);
  const module = directory.slice(0, directory.lastIndexOf("/🧬️schema") + "/🧬️schema".length);
  const siblings = catalog.documents(module).map((other: string) => leafPath(root, module, facetOf(module, other), "🟦️typescript"));
  const context = new Context(helper, typeof document.$id === "string" ? document.$id : undefined, index, target, siblings, sources, new Set());
  context.self = name!;
  const render = (node: Record<string, unknown>): string | undefined => parserExpression(node, "value", "${at}", context, 0);
  const children = (node: Record<string, unknown>): Record<string, unknown>[] => {
    const found: Record<string, unknown>[] = [];
    if (isDict(node.properties)) for (const v of Object.values(node.properties)) if (isDict(v)) found.push(v);
    if (isDict(node.items)) found.push(node.items);
    if (Array.isArray(node.items)) for (const v of node.items) if (isDict(v)) found.push(v);
    if (isDict(node.additionalProperties)) found.push(node.additionalProperties);
    for (const key of ["oneOf", "anyOf", "allOf"]) if (Array.isArray(node[key])) for (const v of node[key] as unknown[]) if (isDict(v)) found.push(v);
    return found;
  };
  const culprit = (node: Record<string, unknown>): Record<string, unknown> => {
    for (const child of children(node)) if (render(child) === undefined) return culprit(child);
    return node;
  };
  const bad = culprit(definition);
  const keys = Object.keys(bad).filter((k) => !["description", "title", "default", "examples", "$comment"].includes(k)).sort().join(",");
  const detail = "$ref" in bad ? String(bad.$ref).slice(-70) : JSON.stringify(bad).slice(0, 160);
  out.push([name!, leaf!, keys, detail]);
}
writeFileSync(process.argv[3]!, JSON.stringify(out));
const counts = new Map<string, number>();
for (const [, , keys] of out) counts.set(keys, (counts.get(keys) ?? 0) + 1);
for (const [k, v] of [...counts].sort((a, b) => b[1] - a[1])) console.log(v, k);
