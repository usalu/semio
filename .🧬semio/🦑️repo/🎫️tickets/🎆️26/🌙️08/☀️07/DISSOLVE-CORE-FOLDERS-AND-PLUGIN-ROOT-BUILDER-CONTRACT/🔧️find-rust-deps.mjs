import { readdirSync, readFileSync, statSync } from "fs";
import { join, relative } from "path";
const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", "target", ".git", ".nx", "dist", "build", ".repo-cache", ".venv", "🎫️tickets"]);
function walk(dir, acc=[]) {
  let entries; try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (SKIP.has(name) || name.startsWith(".")) continue;
    const p = join(dir, name);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, acc); else acc.push(p);
  }
  return acc;
}
const files = walk(ROOT).filter(f => f.endsWith("Cargo.toml"));
const hits = [];
for (const f of files) {
  const text = readFileSync(f, "utf8");
  if (/semio-framework-core/.test(text)) hits.push({f: relative(ROOT,f), text});
}
console.log("Cargo.toml with semio-framework-core:", hits.length);
for (const h of hits) {
  console.log("\n---", h.f, "---");
  for (const line of h.text.split("\n")) {
    if (/semio-framework-core|framework-core/.test(line) || /\[dependencies\]|\[dev-dependencies\]|package\s*=|name\s*=/.test(line) && /semio|package|name/.test(line)) {
      if (/semio-framework-core|framework.core|name =|package =/.test(line)) console.log(line);
    }
  }
  // print matching lines with context
  const lines = h.text.split("\n");
  for (let i=0;i<lines.length;i++) {
    if (/semio-framework-core/.test(lines[i])) {
      for (let j=Math.max(0,i-2); j<=Math.min(lines.length-1,i+2); j++) console.log(`  ${j+1}: ${lines[j]}`);
      console.log("  ---");
    }
  }
}

// also rust uses of crate::ui or ::ui::
console.log("\n=== rust refs to ::ui:: or crate::ui or mod ui ===");
const rsFiles = walk(join(ROOT,"🧰️framework")).filter(f => f.endsWith(".rs"));
for (const f of rsFiles) {
  if (f.includes("🧩core")) continue;
  const text = readFileSync(f, "utf8");
  if (/semio_framework_core::ui|use\s+.*\bui::|crate::ui::/.test(text) || /extern crate semio_framework_core/.test(text)) {
    console.log(relative(ROOT,f));
    for (const line of text.split("\n")) {
      if (/semio_framework_core|crate::ui|\bui::/.test(line) && /framework_core|crate::ui|semio_framework_core::ui/.test(line)) console.log(" ", line.trim().slice(0,140));
    }
  }
}
