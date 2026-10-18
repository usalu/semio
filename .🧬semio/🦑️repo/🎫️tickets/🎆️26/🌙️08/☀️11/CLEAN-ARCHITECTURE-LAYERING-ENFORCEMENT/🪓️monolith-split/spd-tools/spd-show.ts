/** 🔍️ Prints the three renderings of one export: `bun spd-show.ts <schema document path> <ExportName>`. */
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { readDocument, exportsOf } = await import(`${base}/🗂️documents/🟦️.ts`);
const { renderGraphql, renderProto, renderTsType } = await import(`${base}/📐️type-render/🟦️.ts`);
const document = readDocument(root, process.argv[2]!)!;
const exports = exportsOf(document);
const definition = exports.get(process.argv[3]!)!;
console.log("--- graphql\n" + renderGraphql(process.argv[3]!, definition, (n: string) => exports.has(n)));
console.log("--- proto\n" + renderProto(process.argv[3]!, definition, (n: string) => exports.has(n)));
console.log("--- ts\n" + renderTsType(process.argv[3]!, definition));
