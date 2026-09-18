/** 📊️ Classifies `plugin-registry check` taxonomy-tree violations by rule class and by plugin. */

import { readFileSync } from "node:fs";

type Row = Readonly<{ plugin: string; rule: string; detail: string }>;

const RULES: readonly Readonly<{ id: string; test: (detail: string) => boolean }>[] = [
  { id: "unreachable-from-cargo-manifest", test: (detail) => / is not reachable from Cargo manifest /u.test(detail) },
  { id: "window-unexpected-child", test: (detail) => /^window .* has unexpected child /u.test(detail) },
  { id: "surface-missing-presence-schema", test: (detail) => /^surface .* is missing 👥️presence\/🧬️schema\//u.test(detail) },
  { id: "surface-missing-config-schema", test: (detail) => /^surface .* is missing 🎚️config\/🧬️schema\//u.test(detail) },
  { id: "artifact-subset-missing-io", test: (detail) => /^artifact .* is missing 🚪️io\/$/u.test(detail) },
  { id: "artifact-subset-missing-examples", test: (detail) => /^artifact .* is missing 📚️examples\/$/u.test(detail) },
  { id: "plugin-root-missing-commands", test: (detail) => /^plugin root is missing /u.test(detail) },
  { id: "example-slug-invalid", test: (detail) => /is not a valid emoji\+VS16\+kebab slug/u.test(detail) },
];

function classify(detail: string): string {
  return RULES.find((rule) => rule.test(detail))?.id ?? "other";
}

function parse(path: string): readonly Row[] {
  const rows: Row[] = [];
  for (const line of readFileSync(path, "utf8").split("\n")) {
    const match = /^ {2}- ([^:]+): (.*)$/u.exec(line);
    if (!match) continue;
    rows.push({ plugin: match[1]!, rule: classify(match[2]!), detail: match[2]! });
  }
  return rows;
}

function tally(rows: readonly Row[], key: (row: Row) => string): ReadonlyMap<string, number> {
  const counts = new Map<string, number>();
  for (const row of rows) counts.set(key(row), (counts.get(key(row)) ?? 0) + 1);
  return new Map([...counts].sort((left, right) => right[1] - left[1] || left[0].localeCompare(right[0])));
}

const rows = parse(process.argv[2] ?? "🗑️generated/v1-plugin-registry-check.txt");
console.log(`total violations: ${rows.length}`);
console.log(`plugins cited: ${new Set(rows.map((row) => row.plugin)).size}`);
console.log("\n## by rule class\n");
for (const [rule, count] of tally(rows, (row) => row.rule)) console.log(`${String(count).padStart(5)}  ${rule}`);
console.log("\n## by plugin\n");
for (const [plugin, count] of tally(rows, (row) => row.plugin)) console.log(`${String(count).padStart(5)}  ${plugin}`);
console.log("\n## rule × plugin\n");
for (const [rule] of tally(rows, (row) => row.rule)) {
  const scoped = rows.filter((row) => row.rule === rule);
  console.log(`${rule} (${scoped.length}): ${[...tally(scoped, (row) => row.plugin)].map(([plugin, count]) => `${plugin}=${count}`).join(" ")}`);
}
console.log("\n## sample per rule\n");
for (const [rule] of tally(rows, (row) => row.rule)) {
  for (const row of rows.filter((candidate) => candidate.rule === rule).slice(0, 3)) console.log(`${rule} | ${row.plugin} | ${row.detail}`);
}
console.log("\n## distinct shapes for unreachable-from-cargo-manifest\n");
const shapes = tally(rows.filter((row) => row.rule === "unreachable-from-cargo-manifest"), (row) => row.detail.split(" is not reachable")[0]!.split("/").map((segment, index) => index === 1 || /^🔖️|^🪆️|^🧩️|^📃️/u.test(segment) ? "<n>" : segment).join("/"));
for (const [shape, count] of shapes) console.log(`${String(count).padStart(5)}  ${shape}`);
