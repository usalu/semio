// usage: bun bycrate.ts FROM TO [filter-regex-on-crate]  -> distinct errors attributed to the crate target that failed
import { readdirSync, readFileSync } from "node:fs";
const W = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/cargo-fix/window";
const [from, to] = [Number(process.argv[2]), Number(process.argv[3])]; const flt = process.argv[4] ? new RegExp(process.argv[4]) : null;
const by = new Map<string, Map<string, number>>();
for (const f of readdirSync(W).sort()) { const m = /^(\d+)-.*\.errs$/.exec(f); if (!m || +m[1] < from || +m[1] > to) continue;
  let pend: string[] = [];
  for (const l of readFileSync(`${W}/${f}`, "utf8").split("\n")) {
    const c = /^error: could not compile `([^`]+)` \(([^)]*)\)/.exec(l);
    if (c) { const k = `${c[1]} (${c[2]})`; const mp = by.get(k) ?? new Map(); for (const e of pend) mp.set(e, (mp.get(e) ?? 0) + 1); by.set(k, mp); pend = []; continue; }
    if (/error(\[E\d+\])?:/.test(l) && !l.startsWith("error: script")) { const mm = l.match(/^(.*?\.rs):(\d+):\d+: (.*)$/); pend.push(mm ? `${mm[1].split("/").slice(-4).join("/")}:${mm[2]} | ${mm[3].slice(0, 200)}` : l.slice(0, 200)); } } }
for (const [k, mp] of [...by].sort((a, b) => a[0].localeCompare(b[0]))) { if (flt && !flt.test(k)) continue; console.log(`## ${k} (${mp.size} distinct)`); for (const [e] of [...mp].slice(0, Number(process.env.N ?? 12))) console.log("  ", e); }
