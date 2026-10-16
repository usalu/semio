/** 📊️ Per-plugin before/after table: AST import counts (scan1 vs scan4) and cruiser sdk-only edges per batch. Usage: bun table.ts <generated-dir> */
import { readFileSync, existsSync } from "node:fs";
const G = process.argv[2];
const plugin = (f: string) => f.split("/")[2];
const count = (p: string) => { const m = new Map<string, number>(); for (const r of JSON.parse(readFileSync(p, "utf8"))) m.set(plugin(r.file), (m.get(plugin(r.file)) ?? 0) + 1); return m; };
const before = count(G + "/sdkb-scan1.json"), after = count(G + "/sdkb-scan4.json");
const cruiser = new Map<string, number>(), covered = new Set<string>(), dda2 = new Map<string, number>();
for (const l of readFileSync(G + "/dda2-depcruise-err.txt", "utf8").split("\n")) { const m = l.match(/^\s+error plugins-framework-sdk-only: (✏️s\/🔌️plugins\/[^/]+)\//); if (m) dda2.set(m[1].split("/")[2], (dda2.get(m[1].split("/")[2]) ?? 0) + 1); }
for (const b of ["b1", "b2", "b3", "b4", "b5", "b6", "b7"]) {
  const p = `${G}/sdkb-${b}-err.txt`;
  if (!existsSync(p)) continue;
  for (const l of readFileSync(p, "utf8").split("\n")) {
    const m = l.match(/^\s+error plugins-framework-sdk-only: (✏️s\/🔌️plugins\/[^/]+)\//);
    if (m) cruiser.set(m[1].split("/")[2], (cruiser.get(m[1].split("/")[2]) ?? 0) + 1);
  }
}
const names = [...new Set([...before.keys(), ...after.keys(), ...cruiser.keys(), ...dda2.keys()])].sort((a, b) => (before.get(b) ?? 0) - (before.get(a) ?? 0));
console.log("| Plugin | Cruiser edges before (audit 2) | AST imports before | AST imports after | Cruiser edges after (batch run) |\n|---|---:|---:|---:|---:|");
let tb = 0, ta = 0, tc = 0, td = 0;
for (const n of names) { console.log(`| ${n} | ${dda2.get(n) ?? 0} | ${before.get(n) ?? 0} | ${after.get(n) ?? 0} | ${cruiser.get(n) ?? 0} |`); td += dda2.get(n) ?? 0; tb += before.get(n) ?? 0; ta += after.get(n) ?? 0; tc += cruiser.get(n) ?? 0; }
console.log(`| **total** | ${td} | ${tb} | ${ta} | ${tc} |`);
