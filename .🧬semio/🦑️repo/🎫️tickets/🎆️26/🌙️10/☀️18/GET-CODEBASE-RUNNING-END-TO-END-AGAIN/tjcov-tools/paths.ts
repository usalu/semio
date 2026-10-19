import { readdirSync, readFileSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const dir = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage";
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name.endsWith(".ts")) files.push(p); } };
walk(dir);
const prefixes = ["🧰️framework", "✏️s/", "🌎️hub", "🧬"];
const seen = new Set<string>();
for (const f of files) {
  const text = readFileSync(f, "utf8");
  const lines = text.split("\n");
  lines.forEach((line, i) => {
    for (const m of line.matchAll(/"((?:🧰️framework|✏️s|🌎️hub)[^"\n]*)"/g)) {
      const p = m[1]!;
      if (/\s/.test(p) || p.includes("${")) continue;
      const abs = join(root, p);
      if (!existsSync(abs)) { const k = f + ":" + (i + 1) + ":" + p; if (!seen.has(k)) { seen.add(k); console.log("MISSING", f.slice(dir.length + 1) + ":" + (i + 1), p); } }
    }
  });
}
