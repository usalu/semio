// inserts the new audit modules into the two explicit module lists (config-graph deny, readme-reviewed-fixture-inputs copies) keeping their ordering and layout. usage: bun lists.ts [APPLY]
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/";
const discovery = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/";
const added = readdirSync(root + discovery).filter((n) => /audit/.test(n)).map((n) => `${discovery}${n}/🟦️.ts`);
const targets: [string, string][] = [
  [root + "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json", "deny"],
  [root + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json", "copies"],
];
for (const [file, key] of targets) {
  const text = readFileSync(file, "utf8"), doc = JSON.parse(text) as Record<string, string[]>;
  const list = doc[key]!;
  const jsSorted = [...list].sort(), utf16Order = list.every((v, i) => v === jsSorted[i]);
  const segmentSorted = list.filter((p) => p.startsWith(discovery));
  console.log(key, "entries", list.length, "utf16-sorted", utf16Order, "discovery entries", segmentSorted.length, "already has audit", list.filter((p) => /audit/.test(p)).length);
  const missing = added.filter((p) => !list.includes(p));
  const merged = [...list];
  for (const path of missing) {
    let at = merged.findIndex((existing) => existing.startsWith(discovery) && existing > path);
    if (at < 0) { const last = merged.map((p, i) => (p.startsWith(discovery) ? i : -1)).filter((i) => i >= 0).pop()!; at = last + 1; }
    merged.splice(at, 0, path);
  }
  const lineFor = (path: string): string => text.split("\n").find((l) => l.includes(JSON.stringify(path)))!;
  const sample = lineFor(list[0]!);
  const indent = sample.match(/^\s*/)![0];
  const startLine = text.indexOf(`"${key}": [`);
  const open = text.indexOf("[", startLine), close = text.indexOf("]", open);
  const body = merged.map((p) => `${indent}${JSON.stringify(p)}`).join(",\n");
  const next = `${text.slice(0, open + 1)}\n${body}\n${text.slice(close).replace(/^\]/, text.slice(text.lastIndexOf("\n", close) + 1, close) + "]")}`;
  const check = JSON.parse(next) as Record<string, string[]>;
  if (JSON.stringify(check[key]) !== JSON.stringify(merged)) throw new Error("rewrite mismatch for " + key);
  const otherKeysSame = JSON.stringify({ ...check, [key]: 0 }) === JSON.stringify({ ...doc, [key]: 0 });
  console.log(key, "insert", missing.length, "other keys unchanged", otherKeysSame, "diff lines", next.split("\n").length - text.split("\n").length);
  if (process.argv[2] === "APPLY") writeFileSync(file, next);
}
