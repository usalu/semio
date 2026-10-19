import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { resolve, dirname, relative } from "node:path";
const root = resolve(process.argv[2]!);
const entry = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts");
const pattern = /((?:import|export)\b[^;'"`]*?from\s*|import\s*\(\s*|import\s+|require\(\s*)(["'])([^"'\n]+)\2/gm;
const seen = new Set<string>();
const bare: string[] = [];
const queue = [entry];
const cand = (p: string) => [p, p + ".ts", p + "/🟦️.ts", p.replace(/\.js$/, ".ts")];
while (queue.length) {
  const f = queue.pop()!;
  if (seen.has(f) || !existsSync(f)) continue;
  seen.add(f);
  if (!/\.(ts|tsx|mts)$/.test(f)) continue;
  const text = readFileSync(f, "utf8");
  for (const m of text.matchAll(pattern)) {
    const s = m[3]!;
    if (s.startsWith(".")) {
      const base = resolve(dirname(f), s.split("?")[0]!);
      const hit = cand(base).find((c) => existsSync(c) && !require("node:fs").statSync(c).isDirectory());
      if (hit) queue.push(hit);
    } else if (s.startsWith("@semio-tech/")) bare.push(relative(root, f) + " :: " + s);
  }
}
writeFileSync(process.argv[3]!, [...seen].map((x) => relative(root, x)).sort().join("\n") + "\n");
console.log("closure files", seen.size, "bare @semio-tech imports", bare.length);
for (const b of bare.slice(0, 30)) console.log(b);
