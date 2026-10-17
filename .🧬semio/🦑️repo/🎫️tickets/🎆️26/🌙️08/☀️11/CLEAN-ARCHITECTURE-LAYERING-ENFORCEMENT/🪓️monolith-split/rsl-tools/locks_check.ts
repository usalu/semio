// 🔎️ F-04(a) lock check: for every edge of rsl-edges.json and every Cargo.lock holding the consumer's path package, the dependency token is listed and its package block exists; lists the locks that hold neither. Read-only.
import {readFileSync} from "node:fs";
import {allLocks, read, blocksOf, nameOf, depsOf, pathBlock, isMember} from "./lib.ts";
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const raw = JSON.parse(readFileSync(process.argv[2] ?? `${G}/rsl-edges.json`, "utf8")) as {consumer: string; dep: string; dev: boolean}[];
const edges = [...new Map(raw.map(e => [`${e.consumer}>${e.dep}`, e])).values()];
let checked = 0; const bad: string[] = []; const locksTouched = new Set<string>();
for (const lock of allLocks()) {
  const text = read(lock);
  if (!edges.some(e => text.includes(`name = "${e.consumer}"\n`))) continue;
  const {blocks} = blocksOf(text);
  for (const e of edges) {
    const at = pathBlock(blocks, e.consumer);
    if (at < 0) continue;
    if (e.dev && !isMember(lock, e.consumer)) continue;
    checked++; locksTouched.add(lock);
    const deps = depsOf(blocks[at]!).map(d => d.split(" ")[0]);
    if (!deps.includes(e.dep)) bad.push(`${lock}: ${e.consumer} lacks ${e.dep}`);
    else if (!blocks.some(b => nameOf(b) === e.dep)) bad.push(`${lock}: ${e.dep} block missing`);
  }
}
console.log(`edges checked in locks: ${checked}; locks: ${locksTouched.size}; violations: ${bad.length}`);
for (const b of bad.slice(0, 30)) console.log("  " + b);
