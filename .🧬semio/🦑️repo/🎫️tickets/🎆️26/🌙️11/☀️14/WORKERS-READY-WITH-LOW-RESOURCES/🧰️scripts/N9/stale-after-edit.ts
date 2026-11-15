import {existsSync, readFileSync} from "node:fs";
import {dirname, join, relative} from "node:path";
import {brotliDecompressSync} from "node:zlib";

const findRoot = (directory: string): string => (existsSync(join(directory, "AGENTS.md")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd() : findRoot(dirname(directory)));
const root = process.env.SEMIO_WORKSPACE_ROOT ?? findRoot(import.meta.dir), library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {openFactIndexV1} = await import(join(root, library, "🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const directory = join(root, ".cache/b/g/m/generations");
const pointer = JSON.parse(readFileSync(join(directory, "current.json"), "utf8")), manifest = JSON.parse(readFileSync(join(directory, "m", pointer.manifest + ".json"), "utf8"));
const raw = brotliDecompressSync(readFileSync(join(directory, "x", manifest.factIndex + ".facts.br")), {maximumOutputLength: 2 ** 31} as never);
const index: any = openFactIndexV1(raw, {maximumBytes: raw.length});
const covered = (path: string) => /🎯️precision\/(?:🟦️\.ts|🧬️schema\/)/u.test(path) || (path.includes("🔌️managers/") && !path.endsWith("🔣️owned.json"));
const staysData = (path: string) => path.endsWith("🎯️precision/🔣️.json");
const providers = (path: string) => path.endsWith("🔌️providers/📋️project.json");
const rowOwners = new Map<number, Set<number>>();
for (const cls of index.classes) for (const id of cls.r) {
  const row = index.rows[id];
  if (row.kind !== "file" && row.kind !== "digest") continue;
  const path = relative(root, row.path);
  if (!path.startsWith(library) || !(covered(path) || staysData(path) || providers(path))) continue;
  let set = rowOwners.get(id);
  if (!set) rowOwners.set(id, set = new Set());
  for (const o of cls.o) if (!index.owners[o].startsWith("@derived:")) set.add(o);
}
const rows = [...rowOwners].map(([id, set]) => ({path: relative(root, index.rows[id].path).replace(library, "L/"), kind: index.rows[id].kind, holders: set.size, owners: set})).sort((a, b) => b.holders - a.holders);
const projects = new Set<number>();
for (const o of index.owners.keys()) if (!index.owners[o].startsWith("@derived:")) projects.add(o);
const staleBefore = (paths: Set<string>) => { const hit = new Set<number>(); for (const row of rows) if (paths.has(row.path)) for (const o of row.owners) hit.add(o); return hit.size; };
const staleAfterBodyOnly = (paths: Set<string>) => { const hit = new Set<number>(); for (const row of rows) if (paths.has(row.path) && staysData(row.path)) for (const o of row.owners) hit.add(o); return hit.size; };
console.log(`generation ${pointer.generation}, owners ${projects.size}, rows ${rows.length}`);
console.log("holders kind  before(any edit)  after(body-only)  path");
for (const row of rows) console.log(String(row.holders).padStart(7), row.kind.padEnd(6), String(staleBefore(new Set([row.path]))).padStart(10), String(staleAfterBodyOnly(new Set([row.path]))).padStart(14), "  ", row.path.slice(0, 110));
const all = new Set(rows.map((row) => row.path));
console.log(`touching one comment in every one of the ${all.size} rows at once: before ${staleBefore(all)}, after ${staleAfterBodyOnly(all)} (only the DATA rules file stays a content fact)`);
const bodyCode = new Set(rows.filter((row) => covered(row.path)).map((row) => row.path));
console.log(`touching a comment in the ${bodyCode.size} code/schema rows the stamps cover: before ${staleBefore(bodyCode)}, after 0`);
