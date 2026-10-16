import { loadTaxonomy, semanticDirectoryKindId } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
const taxonomy = loadTaxonomy();
for (const path of process.argv.slice(2)) {
  let parent: string | undefined; const trail: string[] = [];
  for (const part of path.split("/")) {
    const id = semanticDirectoryKindId(part, taxonomy, { parentKindId: parent } as never) as string | null | undefined;
    trail.push(`${part}=${id ?? "UNRESOLVED"}`);
    parent = id ?? parent;
  }
  console.log((trail.at(-1)!.endsWith("UNRESOLVED") ? "UNRESOLVED " : "ok ") + trail.slice(-3).join(" > "));
}
