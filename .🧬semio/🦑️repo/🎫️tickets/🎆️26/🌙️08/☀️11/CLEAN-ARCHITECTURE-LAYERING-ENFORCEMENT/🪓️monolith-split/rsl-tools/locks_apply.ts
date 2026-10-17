// 🔗️ F-04(a) lock propagation: for every edge of rsl-edges.json adds the dependency token to the consumer's path package in every Cargo.lock that contains it (dry run unless --write). Reuses the orc/opd propagation (ensurePackage copies a missing producer block with its closure).
import {readFileSync} from "node:fs";
import {propagateEdges, DRY, log, Edge} from "./lib.ts";
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const file = process.argv.slice(2).find(a => !a.startsWith("--")) ?? `${G}/rsl-edges.json`;
const raw = JSON.parse(readFileSync(file, "utf8")) as {consumer: string; dep: string; dev: boolean}[];
const seen = new Set<string>();
const edges: Edge[] = raw.filter(e => { const k = `${e.consumer}>${e.dep}`; if (seen.has(k)) return false; seen.add(k); return true; }).map(e => ({consumer: e.consumer, dep: e.dep, dev: e.dev}));
log(`${DRY ? "(dry) " : ""}edges: ${edges.length}`);
const {changed, skipped} = propagateEdges(edges, []);
log(`locks changed: ${changed.length}`);
for (const c of changed) log("  changed " + c);
for (const s of skipped) log("  skipped " + s);
