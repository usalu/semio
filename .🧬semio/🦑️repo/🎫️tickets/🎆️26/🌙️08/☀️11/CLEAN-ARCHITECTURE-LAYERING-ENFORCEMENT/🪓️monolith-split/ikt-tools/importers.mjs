import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
const R = "/Users/ueli/Documents/semio";
const targets = process.argv.slice(2).map((p) => path.resolve(p));
const git = (args) => execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"] }).split("\n").filter(Boolean);
const files = new Set([...git(["ls-files", "--", "*.ts", "*.tsx", "*.mts", "*.mjs", "*.js", "*.jsx", "*.cjs"]), ...git(["ls-files", "--others", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts", "*.mjs", "*.js", "*.jsx", "*.cjs"])]);
const SPEC = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\s*\(\s*)(["'])(\.{1,2}\/[^"'\n]*?)\1/g;
const rows = [];
for (const rel of files) {
  if (rel.includes("node_modules/") || rel.startsWith(".🧬semio/")) continue;
  const abs = path.join(R, rel);
  let text;
  try { text = fs.readFileSync(abs, "utf8"); } catch { continue; }
  if (!text.includes("/")) continue;
  let m;
  SPEC.lastIndex = 0;
  while ((m = SPEC.exec(text))) {
    const spec = m[2].replace(/\?.*$/, "");
    const resolved = path.resolve(path.dirname(abs), spec);
    for (const t of targets) {
      const stem = t.replace(/\.(tsx?|mts)$/, "");
      if (resolved === t || resolved === stem) {
        const line = text.slice(0, m.index).split("\n").length;
        rows.push(`${rel}:${line}\t${m[2]}\t${path.basename(path.dirname(t))}`);
      }
    }
  }
}
console.log(rows.sort().join("\n"));
console.error("importers:", new Set(rows.map((r) => r.split(":")[0])).size, "specifiers:", rows.length);
