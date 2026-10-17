// 🧲️ Merges the duplicate file-level imports of the shared fixture reader and drops `std::path` imports that the rewrite left unused.
// usage: bun normalize.ts [--apply]
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const F = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const G = join(F, "🗑️generated");
const apply = process.argv.includes("--apply");
const files = new Set<string>();
for (const name of readdirSync(G).filter(n => /^frd-(mig|pdf|sweep|fix).*\.json$/u.test(n))) {
  try { for (const o of JSON.parse(readFileSync(join(G, name), "utf8")) as { file?: string; status?: string }[]) if (o.status === "applied" && o.file) files.add(o.file); } catch { }
}
const IMPORT = /^use semio_framework_test_fixtures::(?:(\w+)|\{([^}]*)\});[ \t]*$/u;
let merged = 0, trimmed = 0, touched = 0;
for (const rel of [...files].sort()) {
  const abs = join(ROOT, rel);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  const lines = text.split("\n");
  let changed = false;
  const at = lines.map((l, i) => (IMPORT.test(l) ? i : -1)).filter(i => i >= 0);
  if (at.length > 1) {
    const names = new Set<string>();
    for (const i of at) { const m = IMPORT.exec(lines[i]!)!; for (const n of (m[1] ?? m[2] ?? "").split(",").map(s => s.trim()).filter(Boolean)) names.add(n); }
    const all = [...names].sort();
    lines[at[0]!] = all.length === 1 ? `use semio_framework_test_fixtures::${all[0]};` : `use semio_framework_test_fixtures::{${all.join(", ")}};`;
    for (const i of at.slice(1).reverse()) lines.splice(i, 1);
    changed = true; merged += 1;
  }
  let out = lines.join("\n");
  if (/std::path::(Path|PathBuf|\{)/u.test(out)) {
    const tokens = rustTokens(out);
    const useLine = /^use std::path::(?:(Path|PathBuf)|\{([^}]*)\});[ \t]*$/mu.exec(out);
    if (useLine) {
      const declared = (useLine[1] ?? useLine[2] ?? "").split(",").map(s => s.trim()).filter(Boolean);
      const useStart = useLine.index, useEnd = useLine.index + useLine[0].length;
      const used = (name: string) => tokens.some(t => t.kind === "identifier" && t.text === name && (t.start < useStart || t.start >= useEnd));
      const keep = declared.filter(name => !["Path", "PathBuf"].includes(name) || used(name));
      if (keep.length !== declared.length) {
        const replacement = keep.length === 0 ? "" : keep.length === 1 ? `use std::path::${keep[0]};` : `use std::path::{${keep.join(", ")}};`;
        out = out.slice(0, useStart) + replacement + out.slice(useEnd + (keep.length === 0 && out[useEnd] === "\n" ? 1 : 0));
        changed = true; trimmed += 1;
      }
    }
  }
  if (!changed) continue;
  touched += 1;
  if (apply) { if (readFileSync(abs, "utf8") !== text) { console.error("raced", rel); continue; } writeFileSync(abs, out); }
}
console.log(JSON.stringify({ files: files.size, touched, merged, trimmed, apply }));
