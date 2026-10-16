// 🔗️ Lock propagation for the orc slice: drops the deleted `semio-framework-pack-record-token` crate and the os-pack edges that the dedupe made unused, in every Cargo.lock (dry run unless --write), then drops packages that became unreachable.
import {allLocks, pathBlock, DRY, log, read, put, blocksOf, nameOf, depsOf, withDeps, cmp} from "./lib.ts";
const GONE = "semio-framework-pack-record-token";
const OS_PACK = "semio-framework-os-pack";
const REMOVED_EDGES = [GONE, "semio-framework-diagnostic", "semio-framework-pack-record-schema", "semio-framework-trace-memory-testing", "semio-framework-pack-intrinsic-media-laws"];
const reachable = (blocks: string[]): Set<string> => {
  const byName = new Map<string, string[]>();
  for (const b of blocks) byName.set(nameOf(b), [...(byName.get(nameOf(b)) ?? []), b]);
  const seen = new Set<string>();
  const queue = blocks.filter(b => !/\nsource = /u.test(b)).map(nameOf);
  while (queue.length) {
    const name = queue.pop()!;
    if (seen.has(name)) continue;
    seen.add(name);
    for (const b of byName.get(name) ?? []) for (const d of depsOf(b)) queue.push(d.split(" ")[0]!);
  }
  return seen;
};
const changed: string[] = [], orphans: string[] = [];
for (const lock of allLocks()) {
  const text = read(lock);
  if (!text.includes(`name = "${GONE}"\n`) && !text.includes(`name = "${OS_PACK}"\n`)) continue;
  const {header, blocks} = blocksOf(text);
  const before = reachable(blocks);
  let touched = false;
  const os = pathBlock(blocks, OS_PACK);
  let next = blocks;
  if (os >= 0) {
    const current = depsOf(blocks[os]!);
    const kept = current.filter(d => !REMOVED_EDGES.includes(d.split(" ")[0]!));
    if (kept.length !== current.length) { next = [...blocks]; next[os] = withDeps(blocks[os]!, kept); touched = true; }
  }
  const gone = pathBlock(next, GONE);
  if (gone >= 0) { next = next.filter((_, i) => i !== gone); touched = true; }
  if (!touched) continue;
  const after = reachable(next);
  const dead = next.filter(b => !after.has(nameOf(b)) && before.has(nameOf(b)));
  const deadNames = dead.map(b => nameOf(b));
  if (dead.length) { orphans.push(`${lock}: ${deadNames.join(", ")}`); next = next.filter(b => !deadNames.includes(nameOf(b))); }
  changed.push(lock);
  put(lock, `${header ? header + "\n" : ""}${next.map(b => b.trimEnd()).join("\n\n")}\n`);
}
log(`${DRY ? "(dry) " : ""}locks changed: ${changed.length}`);
for (const o of orphans) log("  orphans dropped: " + o);
