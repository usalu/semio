import { readFileSync } from "node:fs";
import { join } from "node:path";
import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const T = import.meta.dir;
function newText(backend: string): string {
  const [root, ...children] = listModuleTree(join(repoRoot, "🌎️hub/📇️directory", backend, "🦀️.rs")).map((file) => readFileSync(file, "utf8"));
  const adapter = root!.indexOf("\nimpl HubDirectory for ");
  const rows = backend === "🌐️neo4j" ? [readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs"), "utf8")] : [];
  return [root!.slice(0, adapter), ...children.map((text) => text.replaceAll("pub(super) ", "")), ...rows].join("\n");
}
const old: Record<string, string> = {
  "🪶️sqlite": readFileSync(join(T, "orig/sqlite.rs.txt"), "utf8"),
  "🐘️postgres": readFileSync(join(T, "orig/postgres.rs.txt"), "utf8"),
  "🌐️neo4j": readFileSync(join(T, "orig/neo4j.rs.txt"), "utf8") + "\n" + readFileSync(join(T, "orig/neo4j-rows.rs.txt"), "utf8"),
};
const script = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), "utf8");
const literals = new Set<string>();
for (const m of script.matchAll(/"((?:[^"\\\n]|\\.){6,300})"|`((?:[^`\\]|\\.){6,300})`|'((?:[^'\\\n]|\\.){6,300})'/g)) {
  const raw = m[1] ?? m[2] ?? m[3]!;
  if (raw.includes("${")) continue;
  let value = raw;
  try { value = JSON.parse('"' + raw.replace(/"/g, '\\"') + '"'); } catch { value = raw.replace(/\\n/g, "\n").replace(/\\"/g, '"'); }
  literals.add(value);
}
const count = (text: string, needle: string) => (needle.length ? text.split(needle).length - 1 : 0);
let diffs = 0, relevant = 0;
for (const [backend, oldText] of Object.entries(old)) {
  const text = newText(backend);
  for (const literal of literals) {
    const a = count(oldText, literal), b = count(text, literal);
    if (a === 0 && b === 0) continue;
    if (literal.length < 12 || !/[(){}_.:=\s$\[\]<>-]/.test(literal)) continue;
    relevant++;
    if (a !== b) { diffs++; console.log(`${backend}\told=${a}\tnew=${b}\t${JSON.stringify(literal).slice(0, 150)}`); }
  }
}
console.log(`literals=${literals.size} relevant-pairs=${relevant} differing=${diffs}`);
