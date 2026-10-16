/** ✅️ Checks every import of a workspace package specifier: the subpath is an authored export key and TypeScript's bundler resolution lands on exactly that key's target. Usage: bun spec-check.ts <policy.json> [package-name-substring] */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { loadPolicy, listSourceFiles, nfc } from "./model.ts";
const policy = loadPolicy(process.argv[2]);
const only = process.argv[3];
const pkgs = policy.packages.filter(p => p.exportTargets.length && (!only || p.name.includes(only))).sort((a, b) => b.name.length - a.name.length);
const names = pkgs.map(p => p.name);
let total = 0, ok = 0;
const bad: string[] = [];
const perPackage: Record<string, number> = {};
for (const f of listSourceFiles()) {
  let text: string;
  try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  if (!names.some(n => text.includes(n))) continue;
  for (const r of scanSource(f, text)) {
    const pkg = pkgs.find(p => r.spec === p.name || r.spec.startsWith(p.name + "/"));
    if (!pkg) continue;
    total++;
    perPackage[pkg.name] = (perPackage[pkg.name] ?? 0) + 1;
    const subpath = r.spec === pkg.name ? "." : "." + r.spec.slice(pkg.name.length);
    const rows = pkg.exportTargets.filter(row => row.subpath === subpath && row.target);
    if (!rows.length) { bad.push(`${f}:${r.line} ${r.spec}: not an authored export key`); continue; }
    const want = rows.map(row => nfc(pkg.owner + "/" + row.target!.slice(2)));
    if (!r.resolved || !want.includes(nfc(r.resolved))) { bad.push(`${f}:${r.line} ${r.spec}: resolved ${r.resolved} but export target is ${want[0]}`); continue; }
    ok++;
  }
}
console.log({ imports: total, ok, bad: bad.length });
for (const b of bad.slice(0, 40)) console.log("BAD", b.slice(0, 300));
process.exit(bad.length ? 1 : 0);
