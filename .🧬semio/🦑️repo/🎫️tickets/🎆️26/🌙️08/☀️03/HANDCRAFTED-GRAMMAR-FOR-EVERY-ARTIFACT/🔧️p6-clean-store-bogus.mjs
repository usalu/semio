#!/usr/bin/env bun
/** [DEBUG] P6: remove bogus DocumentCodec/OpCodec regions for types lacking DslDocument/DslOps. */
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
function findStore() {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", "🏪️store", "🦀️component.rs");
    try { if (statSync(p).isFile()) return p; } catch {}
  }
  throw new Error("store missing");
}
const file = findStore();
let t = readFileSync(file, "utf8");

function typeHasDerive(content, typeName, macro) {
  // Find derive block immediately before struct/enum TypeName
  const re = new RegExp(`#\\[derive\\s*\\(([^)]*)\\)\\][\\s\\S]{0,400}?(?:pub(?:\\([^)]*\\))?\\s+)?(?:struct|enum)\\s+${typeName}\\b`);
  const m = content.match(re);
  if (!m) return false;
  return new RegExp(`\\b(?:crate::os_dsl::)?${macro}\\b`).test(m[1]);
}

const removed = [];
t = t.replace(/\n\/\/#region 🔖️(DocumentCodec|OpCodec)\n([\s\S]*?)\/\/#endregion 🔖️\1\n/g, (full, kind, body) => {
  const impls = [...body.matchAll(/impl (?:[\w:]+::)*(DocumentDsl|DocumentPack|OpText|OpBinary) for ([A-Za-z0-9_]+)/g)];
  if (impls.length === 0) return full;
  const typeName = impls[0][2];
  const needsDoc = kind === "DocumentCodec";
  const needsOps = kind === "OpCodec";
  const ok = needsDoc ? typeHasDerive(t, typeName, "DslDocument") : typeHasDerive(t, typeName, "DslOps");
  // Special-case: DocumentPack-only region without DocumentDsl is invalid if type lacks DslDocument
  if (!ok) {
    removed.push({ kind, typeName });
    return "\n";
  }
  // Also remove DocumentCodec that only got attached to wrong types even if somehow matched
  if (typeName === "DocumentPackFiles" || typeName === "DemoDiff" || typeName === "does") {
    removed.push({ kind, typeName, forced: true });
    return "\n";
  }
  return full;
});

// TimestampedOperation may have both DslOps and wrongly DocumentDsl — check DocumentCodec for it
// If TimestampedOperation has only DslOps, DocumentCodec should already be removed by ok check.

writeFileSync(file, t);
writeFileSync(join(ticket, "🧪p6-store-removed.json"), JSON.stringify(removed, null, 2));
console.log(JSON.stringify({ removed: removed.length, removed }, null, 2));

// Re-list remaining
const regions = [...t.matchAll(/\/\/#region 🔖️(DocumentCodec|OpCodec)\n([\s\S]*?)\/\/#endregion 🔖️\1\n/g)];
for (const m of regions) {
  const impls = [...m[2].matchAll(/impl (?:[\w:]+::)*(DocumentDsl|DocumentPack|OpText|OpBinary) for ([A-Za-z0-9_]+)/g)];
  for (const im of impls) console.log("KEEP", m[1], im[1], "for", im[2]);
}
