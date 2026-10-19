import { readFileSync, readdirSync } from "node:fs";
const C = process.argv[2]!;
const changed = readFileSync(`${C}/lay13-tools/ncpr-changed.txt`, "utf8").split("\n").filter(Boolean).map((p) => p.normalize("NFC"));
const logs = readdirSync(`${C}/🗑️generated/tsc4-logs`).filter((f) => f.endsWith(".txt"));
for (const log of logs) {
  const lines = readFileSync(`${C}/🗑️generated/tsc4-logs/${log}`, "utf8").split("\n").filter((l) => /error TS\d+/.test(l));
  const hits = lines.filter((l) => { const p = l.slice(0, l.indexOf("(")).normalize("NFC"); return changed.some((c) => c.endsWith(p)); });
  console.log(`${log}: ${lines.length} errors, ${hits.length} in lay13-changed files`);
  for (const h of hits) console.log("   ", h.slice(0, 300));
}
