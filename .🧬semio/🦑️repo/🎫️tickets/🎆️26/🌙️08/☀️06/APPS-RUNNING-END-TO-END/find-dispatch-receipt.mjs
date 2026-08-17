import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";
const root = process.cwd();
const s = readdirSync(root).find((n) => n.includes("✏️") || (n.length <= 3 && n.includes("s")));
const hits = [];
function walk(d, depth = 0) {
  if (depth > 10) return;
  let ents;
  try { ents = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of ents) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = join(d, e.name);
    if (e.isFile() && e.name.includes("component.rs") && (p.includes("wasm") || p.includes("🌉️"))) {
      const src = readFileSync(p, "utf8");
      if (src.includes("dispatch_text")) {
        const snip = src.split("\n").filter((l) => /dispatch_text|dispatch_binary|CommandReceipt|map\(\|_|map_err|Ok\(\(\)\)/.test(l)).slice(0, 20);
        hits.push({ p, snip });
      }
    }
    if (e.isDirectory()) walk(p, depth + 1);
  }
}
walk(join(root, s));
console.log(JSON.stringify(hits, null, 2));
writeFileSync(join(process.argv[2], "dispatch-receipt-patterns.json"), JSON.stringify(hits, null, 2));
