// usage: bun agg.ts FROM TO  -> lists crates that fail to compile (lib vs test) and distinct non-test errors
import { readdirSync, readFileSync } from "node:fs";
const W = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/cargo-fix/window";
const [from, to] = [Number(process.argv[2]), Number(process.argv[3])];
const lines: string[] = [];
for (const f of readdirSync(W)) { const m = /^(\d+)-.*\.errs$/.exec(f); if (m && +m[1] >= from && +m[1] <= to) lines.push(...readFileSync(`${W}/${f}`, "utf8").split("\n")); }
const crates = new Map<string, number>();
for (const l of lines) { const m = /^error: could not compile `([^`]+)` \(([^)]*)\)/.exec(l); if (m) crates.set(`${m[1]} (${m[2]})`, (crates.get(`${m[1]} (${m[2]})`) ?? 0) + 1); }
console.log("== failing crate targets (steps hit)"); for (const [k, n] of [...crates].sort((a, b) => b[1] - a[1])) console.log(n, k);
const errs = new Map<string, number>();
for (const l of lines) { if (!/error(\[E\d+\])?:/.test(l) || l.startsWith("error: script") || l.startsWith("error: could not compile")) continue; const mm = l.match(/^(.*?\.rs):(\d+):\d+: (.*)$/); const key = mm ? `${mm[1].split("/").slice(-4).join("/")}:${mm[2]} | ${mm[3].slice(0, 190)}` : l.slice(0, 190); errs.set(key, (errs.get(key) ?? 0) + 1); }
console.log("== distinct errors", errs.size); for (const [k, n] of errs) console.log(n, k);
