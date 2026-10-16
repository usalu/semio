import { readFileSync } from "node:fs";
import { docstringEmojiReuseOfText, docstringHitsOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
// argv: snapshotRoot liveRoot child1 child2 ...
const [snap, root, ...kids] = process.argv.slice(2);
const stat = (p: string) => { const t = readFileSync(p, "utf8"); return { reuse: docstringEmojiReuseOfText(p, t), hits: docstringHitsOfText(p, t) }; };
const o = stat(snap!);
const oReuseGroups = o.reuse.length, oReuseLines = o.reuse.reduce((n, r) => n + r.lines.length, 0);
let nGroups = 0, nLines = 0, nHits = 0;
const perFile: Record<string, [number, number]> = {};
for (const p of [root!, ...kids]) { const s = stat(p); nGroups += s.reuse.length; nLines += s.reuse.reduce((n, r) => n + r.lines.length, 0); nHits += s.hits.length; perFile[p.split("/").slice(-2).join("/")] = [s.reuse.length, s.hits.length]; }
console.log(JSON.stringify({ original: { reuseGroups: oReuseGroups, reuseOpeners: oReuseLines, hits: o.hits.length }, now: { reuseGroups: nGroups, reuseOpeners: nLines, hits: nHits }, perFile }));
