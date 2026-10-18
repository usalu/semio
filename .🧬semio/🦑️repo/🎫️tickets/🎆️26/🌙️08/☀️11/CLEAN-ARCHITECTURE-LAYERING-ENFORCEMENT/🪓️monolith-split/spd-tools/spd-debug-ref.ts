/** 🔍️ Finds the node that blocks a pointer into a foreign document: `bun spd-debug-ref.ts <referring leaf> <document $id> <pointer>`. */
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SchemaCatalog, SourceStore, buildOwnerIndex, readDocument } = await import(`${base}/🗂️documents/🟦️.ts`);
const { Context, parserExpression, pointed } = await import(`${base}/🧾️parser-render/🟦️.ts`);
const [leafRel, id, pointer] = process.argv.slice(2) as [string, string, string];
const index = buildOwnerIndex(root, new SchemaCatalog(root));
const owner = index.get(id)!;
const sources = new SourceStore();
const target = `${root}/${leafRel}`;
const doc = readDocument(root, leafRel.replace(/🟦️\.ts$/u, "🔣️.json"));
const context = new Context("g", typeof doc?.$id === "string" ? doc.$id : undefined, index, target, [], sources, new Set(), true, doc);
const view = context.viewOf(owner, owner.load());
const isDict = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);
const render = (node: Record<string, unknown>) => parserExpression(node, "value", "${at}", view, 0);
const children = (node: Record<string, unknown>): Record<string, unknown>[] => {
  const found: Record<string, unknown>[] = [];
  if (isDict(node.properties)) for (const v of Object.values(node.properties)) if (isDict(v)) found.push(v);
  if (isDict(node.items)) found.push(node.items);
  for (const key of ["oneOf", "anyOf", "allOf"]) if (Array.isArray(node[key])) for (const v of node[key] as unknown[]) if (isDict(v)) found.push(v);
  return found;
};
const culprit = (node: Record<string, unknown>): Record<string, unknown> => {
  for (const child of children(node)) if (render(child) === undefined) return culprit(child);
  return node;
};
const node = pointed(pointer, owner.load())!;
console.log("renders:", render(node) !== undefined);
console.log(JSON.stringify(culprit(node)).slice(0, 600));
