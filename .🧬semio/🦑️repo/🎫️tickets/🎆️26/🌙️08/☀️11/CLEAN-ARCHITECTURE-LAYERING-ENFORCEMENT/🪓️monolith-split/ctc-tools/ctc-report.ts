// 📋️ Groups scanner hits by owner crate and declaration for triage, labelled with the rollout slice of owner and consumer crates.
import {readFileSync} from "node:fs";
import {sliceOf} from "./ctc-slices.ts";
const prefix = process.argv[2]!;
const mode = process.argv[3] ?? "bad";
const hits = JSON.parse(readFileSync(prefix + "-hits.json", "utf8")) as any[];
const triageArg = process.argv.find(a => a.startsWith("--triage="));
const triaged = new Set<string>(triageArg ? readFileSync(triageArg.slice(9), "utf8").split("\n").filter(Boolean).map(l => { const [o, i] = l.split("\t"); return `${o}\t${i}`; }) : []);
const sel = hits.filter(h => (mode === "bad" ? !h.sanctioned : h.sanctioned)).filter(h => !triaged.has(`${h.owner}\t${h.item}`));
type G = {owner: string; ownerSlice: string; item: string; decl: string; gate: string; classes: Set<string>; consumers: Map<string, {slice: string; n: number; files: Set<string>}>; n: number};
const groups = new Map<string, G>();
for (const h of sel) {
  const k = `${h.owner}\t${h.item}\t${h.declFile}:${h.declLine}`;
  let g = groups.get(k);
  if (!g) groups.set(k, g = {owner: h.owner, ownerSlice: sliceOf(h.ownerCrate) ?? "-", item: h.item, decl: `${h.declFile}:${h.declLine}`, gate: h.gateRaw, classes: new Set(), consumers: new Map(), n: 0});
  g.classes.add(h.class);
  g.n++;
  let c = g.consumers.get(h.consumerCrate);
  if (!c) g.consumers.set(h.consumerCrate, c = {slice: sliceOf(h.consumer) ?? "-", n: 0, files: new Set()});
  c.n++;
  c.files.add(h.file);
}
const out = [...groups.values()].sort((a, b) => (a.owner + a.item).localeCompare(b.owner + b.item));
for (const g of out) console.log([g.owner, g.ownerSlice, g.item, [...g.classes].join(","), g.n, [...g.consumers].map(([n, c]) => `${n}@${c.slice}`).join(","), g.decl, g.gate].join("\t"));
console.error(`groups ${out.length} hits ${sel.length}`);
