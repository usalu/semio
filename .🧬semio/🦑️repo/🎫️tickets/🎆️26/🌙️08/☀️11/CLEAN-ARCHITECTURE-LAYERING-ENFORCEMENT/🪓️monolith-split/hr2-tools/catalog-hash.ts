/** 🔏️ hr2 schema-catalog hash keeper: snapshots which catalog hashes are fresh before a rename, then recomputes exactly those afterwards. Usage: bun catalog-hash.ts snapshot <out.json> | fix <snapshot.json> [--write] */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json";
const [mode, arg, flag] = process.argv.slice(2);
const text = readFileSync(join(ROOT, CATALOG), "utf8");
const doc = JSON.parse(text);
const sha = (path: string): string | null => {
  try {
    return createHash("sha256").update(readFileSync(join(ROOT, path))).digest("hex");
  } catch {
    return null;
  }
};
if (mode === "snapshot") {
  const fresh: Record<string, string[]> = {};
  let total = 0, ok = 0;
  for (const [scope, row] of Object.entries<any>(doc.scopes)) {
    for (const [file, hash] of Object.entries<string>(row.hashes ?? {})) {
      total++;
      if (sha(join(row.path, file)) === hash) { (fresh[scope] ??= []).push(file); ok++; }
    }
  }
  writeFileSync(arg!, JSON.stringify(fresh));
  console.log(`hashes=${total} fresh=${ok} roundtrip=${JSON.stringify(doc, null, 2) + "\n" === text}`);
} else if (mode === "fix") {
  const fresh: Record<string, string[]> = JSON.parse(readFileSync(arg!, "utf8"));
  let changed = 0, missing = 0, stillFresh = 0;
  for (const [scope, files] of Object.entries(fresh)) {
    const row = doc.scopes[scope];
    if (!row) { missing++; continue; }
    for (const file of files) {
      const now = sha(join(row.path, file));
      if (now === null) { missing++; continue; }
      if (row.hashes[file] === now) { stillFresh++; continue; }
      row.hashes[file] = now;
      changed++;
    }
  }
  console.log(`changed=${changed} unchanged=${stillFresh} missing=${missing}`);
  if (flag === "--write") {
    if (readFileSync(join(ROOT, CATALOG), "utf8") !== text) throw new Error("catalog changed under us");
    writeFileSync(join(ROOT, CATALOG), JSON.stringify(doc, null, 2) + "\n");
  }
}
