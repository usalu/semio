/** 🧮️ Compares the cruiser's edges for the original rules against the AST scan. Usage: bun compare.ts <err.txt> <scan.json> */
import { readFileSync } from "node:fs";
const nfc = (s: string) => s.normalize("NFC");
const [errPath, scanPath] = process.argv.slice(2);
const rules: Record<string, string> = { "no-cross-package-relative-@semio-tech/framework-schema": "schema", "no-cross-package-relative-@semio-tech/schema-validator-ts": "validator", "no-cross-package-relative-@semio-tech/value-ts": "value" };
const cr = new Map<string, Set<string>>();
for (const line of readFileSync(errPath, "utf8").split("\n")) {
  const x = line.match(/^  error ([^:]+): (.*) → (.*)$/u);
  if (!x || !rules[x[1]]) continue;
  const s = cr.get(rules[x[1]]) ?? new Set(); s.add(nfc(x[2]) + "\t" + nfc(x[3])); cr.set(rules[x[1]], s);
}
const scan: any[] = JSON.parse(readFileSync(scanPath, "utf8"));
const sc = new Map<string, Set<string>>();
for (const e of scan) { const s = sc.get(e.owner) ?? new Set(); s.add(nfc(e.file) + "\t" + nfc(e.resolved)); sc.set(e.owner, s); }
for (const k of ["schema", "validator", "value"]) {
  const a = cr.get(k) ?? new Set(), b = sc.get(k) ?? new Set();
  const onlyCr = [...a].filter(x => !b.has(x)), onlyScan = [...b].filter(x => !a.has(x));
  console.log(k, { cruiser: a.size, scan: b.size, onlyCruiser: onlyCr.length, onlyScan: onlyScan.length });
  for (const x of onlyCr.slice(0, 8)) console.log("  CRUISER-ONLY", x.replace(/\t/u, " -> ").slice(0, 250));
  for (const x of onlyScan.slice(0, 8)) console.log("  SCAN-ONLY", x.replace(/\t/u, " -> ").slice(0, 250));
}
