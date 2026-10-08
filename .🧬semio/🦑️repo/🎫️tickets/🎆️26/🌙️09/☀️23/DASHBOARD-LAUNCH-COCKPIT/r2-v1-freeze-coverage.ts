import { readFileSync, writeFileSync } from "node:fs";
const [source, target] = process.argv.slice(2);
const coverage = JSON.parse(readFileSync(source!, "utf8"));
const seen = new Set<string>();
const selections: unknown[] = [];
const add = (origin: string, entry: any) => {
  const key = JSON.stringify([entry.id, entry.parameters ?? {}, entry.extraArgs ?? []]);
  if (seen.has(key)) return;
  seen.add(key);
  selections.push({ origin, id: entry.id, parameters: entry.parameters ?? {}, extraArgs: entry.extraArgs ?? [], expected: entry.expected, match: entry.match ?? entry.verdict ?? null, differences: (entry.differences ?? []).map((difference: any) => difference.kind) });
};
for (const row of coverage.rows) add("vscode", row);
for (const row of coverage.compounds) add("compound", row);
for (const row of coverage.claude) if (row.id) add("claude", row);
const ids = [...new Set(selections.map((selection: any) => selection.id))].sort();
const text = JSON.stringify({ _comment: "Frozen 2026-10-08 from the M-1a independent resolver (ticket 2026/09/23/DASHBOARD-LAUNCH-COCKPIT, m1a-coverage.ts) over the retained rows of the removed .vscode/launch.json, its compounds and .claude/launch.json: every selection the migration must keep resolvable, with the cmd/args/env/ready the launch row stood for.", ids: ids.length, selections }, null, 1);
writeFileSync(target!, text);
console.log(`${selections.length} selections, ${ids.length} ids, ${text.length} bytes`);
