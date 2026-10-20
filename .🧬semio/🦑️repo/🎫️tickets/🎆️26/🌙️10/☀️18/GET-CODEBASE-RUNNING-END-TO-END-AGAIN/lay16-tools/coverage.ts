/**
 * 🧮 For each remaining boundary edge: is its target covered by an entry of the admitted package in the staged batch (exports-stage), by another package only, or not at all?
 *
 *   bun coverage.ts <scan.tsv> <rule-regex> <package> <batch.json>
 */
import { readFileSync } from "node:fs";
const [tsv, rulePat, pkg, batchPath] = [process.argv[2]!, new RegExp(process.argv[3]!), process.argv[4]!, process.argv[5]!];
const batch = JSON.parse(readFileSync(batchPath, "utf8")).entries as any[];
const byTarget = new Map<string, Set<string>>();
for (const e of batch) if (e.op === "add" && e.field === "exports" && e.justification?.target) (byTarget.get(e.justification.target) ?? byTarget.set(e.justification.target, new Set()).get(e.justification.target)!).add(e.package + e.key.slice(1));
const rows = readFileSync(tsv, "utf8").split("\n").filter(Boolean).map(l => l.split("\t")).filter(r => rulePat.test(r[0]!));
const groups = new Map<string, { classes: Set<string>; files: Set<string>; keys: Set<string> }>();
for (const [rule, from, to] of rows) {
  const keys = byTarget.get(to!) ?? new Set<string>();
  const inPkg = [...keys].some(k => k.startsWith(pkg));
  const cls = inPkg ? "batch-has-" + pkg : keys.size ? "batch-other-package" : "not-in-batch";
  const g = groups.get(to!) ?? groups.set(to!, { classes: new Set(), files: new Set(), keys: new Set() }).get(to!)!;
  g.classes.add(cls); g.files.add(from!); for (const k of keys) g.keys.add(k);
}
const tally: Record<string, number> = {};
for (const [t, g] of [...groups].sort()) { const c = [...g.classes].join("+"); tally[c] = (tally[c] ?? 0) + g.files.size; console.log(c.padEnd(28), String(g.files.size).padStart(3), t.replace("🧰️framework/", "FW/").slice(0, 150), "|", [...g.keys].slice(0, 2).join(",").slice(0, 100)); }
console.log(tally);
