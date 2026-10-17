// 🔬 sh7: does the parser projection see the types of a split TypeScript leaf? Compares what the projection reads from the root file alone (`SourceStore.read`), from `SourceStore.declarations`, and from the unsplit monolith at HEAD, for the architect schema and diff leaves.
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { declares, declaredOptionalFields, declaredTypes, aliasText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📖️declared-types/🟦️.ts";
import { SourceStore, exportsOf, readDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const schema = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const git = (spec: string): string => spawnSync("git", ["-c", "core.quotepath=false", "show", spec], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
const probe = (a: string, b: string, name: string): number =>
  Number(declares("🟦️typescript", a, name) !== declares("🟦️typescript", b, name)) +
  Number(JSON.stringify([...(declaredOptionalFields(a, name) ?? [])]) !== JSON.stringify([...(declaredOptionalFields(b, name) ?? [])])) +
  Number(JSON.stringify([...declaredTypes(a, name)]) !== JSON.stringify([...declaredTypes(b, name)])) +
  Number(aliasText(a, name) !== aliasText(b, name));
let rootOnlyBad = 0, newBad = 0;
for (const facet of ["", "🔺️diff"]) {
  const rel = join(schema, facet);
  const leaf = join(root, rel, "🟦️.ts");
  const document = readDocument(root, join(rel, "🔣️.json"));
  if (document === undefined) continue;
  const names = [...exportsOf(document).keys()];
  const store = new SourceStore();
  const only = store.read(leaf), seen = store.declarations(leaf), head = git(`HEAD:${join(rel, "🟦️.ts")}`);
  let a = 0, b = 0;
  for (const name of names) { a += probe(only, head, name) > 0 ? 1 : 0; b += probe(seen, head, name) > 0 ? 1 : 0; }
  console.log(`facet=${facet || "(module)"} exports=${names.length} names-where-root-only-differs-from-monolith=${a} names-where-declarations-differs-from-monolith=${b}`);
  rootOnlyBad += a; newBad += b;
}
console.log(`SUMMARY root-only=${rootOnlyBad} declarations=${newBad}`);
process.exit(newBad === 0 && rootOnlyBad > 0 ? 0 : 1);
