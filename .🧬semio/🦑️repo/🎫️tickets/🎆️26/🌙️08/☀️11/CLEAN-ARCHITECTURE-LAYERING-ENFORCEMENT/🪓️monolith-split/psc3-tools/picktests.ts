/** 🧪️ Picks the tests that exercise the touched importers: the touched test files plus the test entries of the closest enclosing module directory of every touched file. Usage: bun picktests.ts <report.json> <policy.json> <out.txt> [max] [--extra <dir>]... */
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { ROOT } from "../sdkb-tools/lib.ts";
import { nfc } from "./model.ts";
const argv = process.argv.slice(2);
const extra = argv.flatMap((a, i) => (a === "--extra" ? [nfc(argv[i + 1])] : []));
const [reportPath, , out, max = "400"] = argv.filter((a, i) => a !== "--extra" && argv[i - 1] !== "--extra");
const report = JSON.parse(readFileSync(reportPath, "utf8"));
const perModule = Number(process.env.PER_MODULE ?? "3");
const touched: string[] = Object.keys(report.files).map(nfc);
const run = (args: string[]) => execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: ROOT, maxBuffer: 1 << 29, stdio: ["ignore", "pipe", "ignore"] }).toString().split("\n").filter(Boolean).map(nfc);
const entry = /\/🧪️tests?\/(?:[^/]+\/)*🟦️\.tsx?$|\.test\.tsx?$/u;
const roots = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const all = [...run(["ls-files", "--", ...roots]), ...run(["ls-files", "--others", "--exclude-standard", "--", ...roots])].filter(f => entry.test(f) && !f.includes("/🧫️fixtures/") && !f.includes("/node_modules/"));
const byModule = new Map<string, string[]>();
for (const t of all) { const m = t.slice(0, t.indexOf("/🧪️test")); const l = byModule.get(m) ?? []; l.push(t); byModule.set(m, l); }
const tests = new Set<string>(touched.filter(f => entry.test(f) && !f.includes("/🧫️fixtures/")));
for (const f of touched) {
  let d = f.slice(0, f.lastIndexOf("/"));
  while (d) { if (byModule.has(d)) { for (const t of byModule.get(d)!.slice(0, perModule)) tests.add(t); break; } const i = d.lastIndexOf("/"); if (i < 0) break; d = d.slice(0, i); }
}
for (const e of extra) for (const [m, l] of byModule) if (m === e || m.startsWith(e + "/")) for (const t of l) tests.add(t);
const list = [...tests].sort();
writeFileSync(out, list.slice(0, Number(max)).join("\n") + "\n");
console.log(`touched=${touched.length} tests=${list.length} written=${Math.min(list.length, Number(max))}`);
