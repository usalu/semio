import { readFileSync } from "node:fs";
import { loadTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts";
import { matchDirectoryKind } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" });
const listPath = process.argv[2]!;
const paths = readFileSync(listPath, "utf8").split("\n").filter(Boolean);
const cache = new Map<string, { kind: string | null; ambiguous: string[] }>();
const chainOf = (path: string): { kinds: (string | null)[]; last: { kind: string | null; ambiguous: string[] } } => {
  const segments = path.split("/");
  const kinds: (string | null)[] = [];
  let last = { kind: null as string | null, ambiguous: [] as string[] };
  for (let index = 0; index < segments.length; index += 1) {
    const prefix = segments.slice(0, index + 1).join("/");
    const hit = cache.get(prefix);
    if (hit) { kinds.push(hit.kind); last = hit; continue; }
    const parent = index === 0 ? undefined : (kinds[index - 1] ?? undefined);
    const ancestors = kinds.slice(0, Math.max(0, index - 1)).reverse().filter((value): value is string => value !== null);
    const match = matchDirectoryKind(segments[index]!, taxonomy, parent, ancestors);
    last = { kind: match.kind?.id ?? null, ambiguous: [...match.ambiguous] };
    cache.set(prefix, last);
    kinds.push(last.kind);
  }
  return { kinds, last };
};
let bad = 0;
for (const path of paths) {
  const { last } = chainOf(path);
  const leaf = path.split("/").at(-1)!;
  const status = last.kind ? "ok" : last.ambiguous.length ? "AMBIGUOUS" : "UNRESOLVED";
  if (!last.kind) bad += 1;
  console.log(`${status}\t${last.kind ?? last.ambiguous.join("|")}\t${leaf}\t${path.split("/").slice(-3, -1).join("/")}`);
}
console.log(`checked=${paths.length} unresolved=${bad}`);
