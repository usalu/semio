import { readFileSync } from "node:fs";
const needle = process.argv[2]!;
const files = readFileSync(process.argv[3]!, "utf8").split("\n").filter(l => l.startsWith("  ")).map(l => l.trim());
const forms = new Map<string, number>();
for (const f of new Set(files)) {
  let t = ""; try { t = readFileSync("/Users/ueli/Documents/semio/" + f, "utf8"); } catch { continue; }
  for (const line of t.split("\n")) if (line.includes(needle)) { const k = line.trim().replace(/[0-9]+/g, "#").slice(0, 230); forms.set(k, (forms.get(k) ?? 0) + 1); }
}
for (const [k, v] of [...forms].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(String(v).padStart(4), k);
