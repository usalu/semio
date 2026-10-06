import { readFileSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
const root = "/Users/ueli/Documents/semio", ticket = dirname(import.meta.dir);
const paths = Bun.spawnSync(["rg", "--files", "🧰️framework", "✏️s", "🌎️hub", "-g", "*.json"], { cwd: root }).stdout.toString().trim().split("\n");
const rows: { path: string; name: string; keys: string[]; constants: string[]; signals: string[] }[] = [];
for (const path of paths) {
  let document: any;
  try { document = JSON.parse(readFileSync(join(root, path), "utf8")); } catch { continue; }
  if (!document?.$schema || !/json-schema\.org|json-schema\.org\/draft/u.test(document.$schema)) continue;
  for (const [name, schema] of [["<root>", document], ...Object.entries(document.$defs ?? document.definitions ?? {})] as [string, any][]) {
    const visited = new Set<unknown>(), keys = new Set<string>(), constants = new Set<string>();
    const visit = (node: any): void => {
      if (!node || typeof node !== "object" || visited.has(node)) return;
      visited.add(node);
      if (node.$ref?.startsWith("#/$defs/")) visit(document.$defs?.[node.$ref.slice(8)]);
      if (node.$ref?.startsWith("#/definitions/")) visit(document.definitions?.[node.$ref.slice(14)]);
      for (const [key, value] of Object.entries(node.properties ?? {}) as [string, any][]) {
        keys.add(key);
        if (value && Object.hasOwn(value, "const")) constants.add(key + "=" + JSON.stringify(value.const).slice(0, 150));
        visit(value);
      }
      for (const [key, value] of Object.entries(node)) if (!["properties", "$defs", "definitions"].includes(key)) {
        if (Array.isArray(value)) value.forEach(visit); else visit(value);
      }
    };
    visit(schema);
    const signals = [...keys].filter((key) => /^(?:expected|sourceFixture|hostiles?$|negativeCases$|comparisonHeapAllocations$)|(?:Survives|Agreement|Oracle|RejectedCases|ResumeCuts)$/u.test(key));
    if (signals.length || constants.size >= 4 && [...keys].some((key) => /cases|vectors|scenarios|grants|checks|observations|census|boundaries/iu.test(key))) rows.push({ path, name, keys: [...keys], constants: [...constants], signals });
  }
}
writeFileSync(join(ticket, "🗑️generated", "schema-structural-review.json"), JSON.stringify(rows, null, 2));
writeFileSync(join(ticket, "📓️remaining-structural-schema-review-2026-10-06.md"), "# Remaining Structural Schema Review\n\nRead-only candidate scan; candidates require semantic ownership review and are not automatic violations. Genuine decoder/report contracts remain authoritative.\n\n" + rows.map((row) => `- \`${row.path}\` — \`${row.name}\`: ${row.signals.join(", ") || "fixed example constants"}; constants ${row.constants.join("; ")}`).join("\n") + "\n");
console.log(`[DEBUG] structural schema candidate scan paths=${paths.length} candidates=${rows.length}`);
