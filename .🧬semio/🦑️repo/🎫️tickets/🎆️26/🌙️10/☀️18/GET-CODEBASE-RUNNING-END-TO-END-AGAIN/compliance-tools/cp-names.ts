/** 🏷️ Taxonomy path-emoji statute on the paths of a list and their siblings (the repo law walks 465k paths): bun cp-names.ts <list> <out json>. */
import { readFileSync, readdirSync, writeFileSync, lstatSync } from "node:fs";
import { posix } from "node:path";
const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.CP_ROOT ?? LIVE;
const L = `${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const taxonomy = (await import(`${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`)).loadTaxonomy();
const { pathEmojiStatuteFindings, reservedDocumentationBasename } = await import(`${LIVE}/🧰️framework/📦️packages/🟦️typescript/🪪️identity/🛣️path/🟦️.ts`);
const mine = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((p) => p.normalize("NFC"));
const dirs = new Set<string>();
for (const p of mine) for (let d = posix.dirname(p); d !== "." && d !== ""; d = posix.dirname(d)) dirs.add(d);
const RESERVED = /^(README\.md|package\.json|Cargo\.toml|Cargo\.lock|tsconfig[\w.-]*\.json|BUILD\.bazel|\.gitignore|\.gitattributes|LICENSE|AGENTS\.md|CLAUDE\.md|bun\.lock|go\.mod|go\.sum)$/u;
const entries = new Map<string, { path: string; nodeKind: "file" | "directory"; reserved: boolean }>();
const addEntry = (path: string, nodeKind: "file" | "directory"): void => {
  const name = posix.basename(path);
  entries.set(path, { path, nodeKind, reserved: RESERVED.test(name) || reservedDocumentationBasename(name) === true });
};
for (const d of dirs) {
  addEntry(d, "directory");
  let names: string[] = [];
  try { names = readdirSync(`${ROOT}/${d}`); } catch { continue; }
  for (const n of names) {
    const rel = `${d}/${n}`.normalize("NFC");
    if (n === "node_modules" || n === "target" || n === "dist" || n.startsWith(".") && n !== ".🧾️store") continue;
    try { addEntry(rel, lstatSync(`${ROOT}/${rel}`).isDirectory() ? "directory" : "file"); } catch { /* vanished */ }
  }
}
for (const p of mine) addEntry(p, "file");
const findings = pathEmojiStatuteFindings([...entries.values()], taxonomy.pathEmojiPolicy.genericEmojiIdentities) as { kind: string; path: string; sibling?: string; emoji?: string }[];
const mineSet = new Set([...mine, ...dirs]);
const rel = findings.filter((f) => mineSet.has(f.path));
writeFileSync(process.argv[3]!, JSON.stringify({ entries: entries.size, findings: findings.length, mine: rel }, null, 1));
console.error("[DEBUG] names", entries.size, "entries", findings.length, "findings", rel.length, "touching the list, rss MB", Math.round(process.memoryUsage().rss / 1048576));
