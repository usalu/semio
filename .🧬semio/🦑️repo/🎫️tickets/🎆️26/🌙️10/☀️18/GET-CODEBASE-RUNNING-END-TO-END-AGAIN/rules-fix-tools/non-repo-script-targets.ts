import { readFileSync, existsSync, writeFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const [list, out] = process.argv.slice(2);
const files = readFileSync(list, "utf8").split("\n").filter((f) => f && !f.startsWith(".🧬semio/"));
const owner = (p: string) => (p.startsWith("🧰️framework/") ? "🧰️framework" : p.startsWith("✏️s/") ? "✏️s" : "other");
const groups: Record<string, unknown[]> = { "🧰️framework": [], "✏️s": [], other: [] };
let total = 0, scanned = 0, bad = 0;
for (const f of files) {
  const full = `${root}/${f}`;
  if (!existsSync(full)) continue;
  let json: any;
  try { json = JSON.parse(readFileSync(full, "utf8")); } catch { bad++; continue; }
  scanned++;
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    total++;
    const executor = t.executor ?? null;
    if (executor === "repo:script") continue;
    const o = t.options ?? {};
    const sub = executor === null ? (o.command !== undefined || o.commands !== undefined ? "implicit-run-commands" : "implicit-no-command") : executor;
    groups[owner(f)].push({ project: json.name ?? null, file: f, target: name, executor, kind: sub, command: o.command ?? o.commands ?? null, cwd: o.cwd ?? null, dependsOn: t.dependsOn ?? null, cache: t.cache ?? null, usesBun: typeof o.command === "string" && o.command.startsWith("bun ") });
  }
}
const counts = Object.fromEntries(Object.entries(groups).map(([k, v]) => [k, v.length]));
const doc = { schemaVersion: 1, generatedAt: new Date().toISOString(), source: "working tree project.json files (tickets excluded)", filesScanned: scanned, unparsable: bad, targetsTotal: total, nonRepoScriptTotal: Object.values(counts).reduce((a, b) => a + b, 0), counts, groups };
writeFileSync(out, JSON.stringify(doc, null, 2) + "\n");
console.log(JSON.stringify({ scanned, bad, total, counts }));
