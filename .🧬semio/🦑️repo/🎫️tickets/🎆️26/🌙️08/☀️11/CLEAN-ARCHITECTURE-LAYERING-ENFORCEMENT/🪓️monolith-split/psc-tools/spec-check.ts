/** ✅️ Checks every import of the three package specifiers: the subpath is an authored export key and TypeScript's bundler resolution lands on exactly that export's target (the invariants of the layering graph check). Usage: bun spec-check.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { loadPackages, listSourceFiles, nfc, PKG_NAME } from "./model.ts";
const pkgs = loadPackages().filter(p => Object.values(PKG_NAME).includes(p.name));
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
console.log({ imports: total, ok, bad: bad.length, perPackage });
for (const b of bad.slice(0, 30)) console.log("BAD", b.slice(0, 300));
process.exit(bad.length ? 1 : 0);
