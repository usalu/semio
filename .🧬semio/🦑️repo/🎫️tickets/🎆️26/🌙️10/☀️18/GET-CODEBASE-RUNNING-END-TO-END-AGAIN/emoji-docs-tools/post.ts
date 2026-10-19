import { blocksOf } from "./docs.ts";
import { norm } from "./pick.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const applied = JSON.parse(await Bun.file(`${out}/applied-apply.json`).text());
let missing = 0, fixedBlocks = 0, dupFiles = 0, dupAdded = 0, gone = 0;
const remaining: Record<string, number> = {};
const trees: Record<string, number> = {};
const collisions: any[] = [];
for (const a of applied) {
  let text: string;
  try { text = await Bun.file(`${root}/${a.p}`).text(); } catch { gone++; continue; }
  const bl = blocksOf(a.p, text);
  const miss = bl.filter((b) => !b.startEmoji);
  missing += miss.length;
  for (const m of miss) { const k = m.skip ?? (/^[^\x00-\x7f]/u.test(m.first) ? "symbolLed" : "other"); remaining[k] = (remaining[k] ?? 0) + 1; }
  const tree = a.p.split("/").slice(0, 2).join("/");
  trees[tree] = (trees[tree] ?? 0) + a.n;
  fixedBlocks += a.n;
  const counts = new Map<string, number>();
  for (const b of bl) if (b.startEmoji) counts.set(norm(b.startEmoji), (counts.get(norm(b.startEmoji)) ?? 0) + 1);
  const added = new Set(a.edits.filter((e: any) => e.emoji).map((e: any) => norm(e.emoji)));
  let d = 0;
  for (const e of added as Set<string>) if ((counts.get(e) ?? 0) > 1) { d++; collisions.push({ p: a.p, emoji: e, n: counts.get(e) }); }
  if (d) { dupFiles++; dupAdded += d; }
}
console.log({ files: applied.length, gone, fixedBlocks, missingAfter: missing, remaining, dupFiles, dupAdded });
console.log(Object.entries(trees).sort((a, b) => b[1] - a[1]));
await Bun.write(`${out}/collisions.json`, JSON.stringify(collisions));
