/** ⚖️ Prove every re-pointed literal now resolves, through the owner's entry, to the identical file. */
import { readFileSync } from "node:fs";
import { dirname, join, normalize, resolve } from "node:path";
import { collect, ROOT } from "./work.ts";

const index = readFileSync(join(import.meta.dir, "orig/index.tsv"), "utf8").split("\n").filter(Boolean).map(line => line.split("\t") as [string, string]);
const originals = new Map<string, string>();
for (const [snap, file] of index) if (snap !== "NEW" && !file.endsWith("🧪️tests/🧫️fixtures/🟦️.ts")) originals.set(file, readFileSync(snap, "utf8"));
const { hits } = collect([...originals.keys()], { textOf: file => originals.get(file) });
const exported = new Map<string, Set<string>>();
const entryPaths = new Set(index.map(([, file]) => file).filter(file => file.endsWith("🧪️tests/🧫️fixtures/🟦️.ts")));
for (const entry of entryPaths) {
  const text = readFileSync(join(ROOT, entry), "utf8");
  const targets = new Set<string>();
  for (const match of text.matchAll(/"(\.{1,2}\/[^"]+)"/g)) targets.add(normalize(resolve(dirname(join(ROOT, entry)), match[1]!.replace(/\?raw$/, ""))).slice(ROOT.length + 1));
  exported.set(entry, targets);
}
let ok = 0;
const bad: string[] = [];
for (const hit of hits) {
  const entry = hit.owner + "/🧪️tests/🧫️fixtures/🟦️.ts";
  if (exported.get(entry)?.has(hit.target)) ok++;
  else bad.push(`${hit.file}\t${hit.target}`);
}
console.log(JSON.stringify({ originalLiteralsInSnapshots: hits.length, resolveThroughEntryToSameFile: ok, unresolved: bad.length }));
if (bad.length) console.log(bad.slice(0, 20).join("\n"));
