import {existsSync, readFileSync, readdirSync} from "node:fs";
import {dirname, join, relative} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const root = "/Users/Shared/semio", library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {openFactIndexV1} = await import(join(root, library, "🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const directory = join(root, ".cache/b/g/m/generations");
const gens = readdirSync(join(directory, "m")).map(f => JSON.parse(readFileSync(join(directory, "m", f), "utf8"))).sort((a, b) => a.generation - b.generation);
const manifest = gens[gens.length - 1];
const raw = brotliDecompressSync(readFileSync(join(directory, "x", manifest.factIndex + ".facts.br")), {maximumOutputLength: 2 ** 31} as never);
const index: any = openFactIndexV1(raw, {maximumBytes: raw.length});
const wanted = process.env.R29_PATHS ? new Set(JSON.parse(readFileSync(process.env.R29_PATHS, "utf8")) as string[]) : undefined;
const perKind = new Map<string, Map<string, Set<string>>>();
const holders = new Map<string, {kinds: Set<string>; owners: Set<string>}>();
for (const cls of index.classes) for (const id of cls.r) {
  const row = index.rows[id];
  const path = typeof row.path === "string" ? row.path : "";
  const rel = path.startsWith(root + "/") ? relative(root, path) : path;
  let entry = holders.get(rel);
  if (!entry) holders.set(rel, entry = {kinds: new Set(), owners: new Set()});
  entry.kinds.add(row.kind);
  for (const o of cls.o) if (!index.owners[o].startsWith("@derived:")) entry.owners.add(index.owners[o]);
  if (wanted?.has(rel)) { let k = perKind.get(rel); if (!k) perKind.set(rel, k = new Map()); let set = k.get(row.kind); if (!set) k.set(row.kind, set = new Set()); for (const o of cls.o) if (!index.owners[o].startsWith("@derived:")) set.add(index.owners[o]); }
}
const want = process.env.R29_PATHS ? new Set(JSON.parse(readFileSync(process.env.R29_PATHS, "utf8")) as string[]) : undefined;
const counts = [...holders].map(([path, e]) => [path, e.owners.size, [...e.kinds]] as const).filter(([, n]) => n >= 2).sort((a, b) => b[1] - a[1]);
const out: Record<string, {kinds: string[]; owners: string[]}> = {};
if (want) for (const path of want) { const e = holders.get(path); out[path] = e ? {kinds: [...e.kinds], owners: [...e.owners]} : {kinds: [], owners: []}; }
const kindCount: Record<string, number> = {};
for (const cls of index.classes) for (const id of cls.r) kindCount[index.rows[id].kind] = (kindCount[index.rows[id].kind] ?? 0) + 1;
console.log("generation", manifest.generation, "rows", index.rows.length, "kinds", JSON.stringify(kindCount), "paths", holders.size);
await Bun.write(process.env.R29_OUT!, JSON.stringify({perKind: Object.fromEntries([...perKind].map(([p, k]) => [p, Object.fromEntries([...k].map(([kind, set]) => [kind, set.size]))])), generation: manifest.generation, owners: [...new Set(index.owners as string[])].filter(o => !o.startsWith("@derived:")).length, holders: out, shared: counts.slice(0, 4000)}));
