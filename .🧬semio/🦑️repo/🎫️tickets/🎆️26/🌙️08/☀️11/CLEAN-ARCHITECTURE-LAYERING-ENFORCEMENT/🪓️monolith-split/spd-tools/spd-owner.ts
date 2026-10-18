/** 🔍️ Prints the owner the index holds for a document id: `bun spd-owner.ts <$id>`. */
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SchemaCatalog, buildOwnerIndex } = await import(`${base}/🗂️documents/🟦️.ts`);
const catalog = new SchemaCatalog(root);
const index = buildOwnerIndex(root, catalog);
const owner = index.get(process.argv[2]!);
console.log(owner === undefined ? "NO OWNER" : JSON.stringify({ ts: owner.ts, root: owner.root, shared: owner.shared, specifier: owner.specifier, exports: [...owner.exports] }));
console.log("extras", catalog.extraDocuments().length, "manifests", catalog.manifests.length);
