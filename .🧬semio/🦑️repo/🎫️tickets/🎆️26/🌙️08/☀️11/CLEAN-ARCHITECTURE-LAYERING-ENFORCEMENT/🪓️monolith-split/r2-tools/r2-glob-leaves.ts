// 🧹️ Replaces the host glob imports (`use super::*;`, `use super::super::*;`) of the mutation leaves of one aggregate by explicit imports, so a leaf no longer depends on the private imports of its aggregate file.
// usage: bun r2-glob-leaves.ts <aggregate-dir> <crate-path-of-aggregate e.g. crate::schema::mutations> [--write]
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { parseRustSourceTree, identifierSet, useLeafNames, prunedUse } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const { rustTokens } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
const [dir, hostPath, flag] = process.argv.slice(2) as [string, string, string?];
const write = flag === "--write";
const aggFile = join(dir, "🦀️.rs");
let aggText = readFileSync(aggFile, "utf8");
const agg = parseRustSourceTree(aggText);
type Provided = { kind: "use"; statement: string; test: boolean } | { kind: "item"; name: string; vis: string; start: number; declStart: number } | { kind: "mod"; name: string };
const provided = new Map<string, Provided>();
for (const item of agg.items) {
  if (item.kind === "use") {
    if (item.visibility !== "private") continue;
    const statement = aggText.slice(item.declStart, item.end);
    const test = item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a));
    for (const leaf of useLeafNames(statement)) provided.set(leaf, { kind: "use", statement, test });
  } else if (item.name && item.kind !== "impl" && item.kind !== "macro" && item.kind !== "extern-crate") provided.set(item.name, { kind: "item", name: item.name, vis: item.visibility, start: item.start, declStart: item.declStart });
}
for (const node of agg.modules) if (!node.conditionalTest) provided.set(node.name, { kind: "mod", name: node.name });
const usedAsItem = (text: string, name: string): boolean => {
  const tokens = rustTokens(text);
  const upper = /^[A-Z]/u.test(name);
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i].text !== name) continue;
    const prev = tokens[i - 1]?.text, next = tokens[i + 1]?.text;
    if (prev === "." || prev === "let" || prev === "mut" || prev === "fn" || prev === "struct" || prev === "enum" || prev === "const" || prev === "static" || prev === "type" || prev === "trait" || prev === "mod") continue;
    if (upper || next === "(" || next === "::" || next === "!" || prev === "::" || prev === "&" || prev === "(" || prev === "," || prev === "=" || prev === "->" || prev === ":" || prev === "<") return true;
  }
  return false;
};
const widen = new Set<string>();
const files: { file: string; glob: string; keepGlob: boolean }[] = [];
for (const name of readdirSync(dir)) {
  const leaf = join(dir, name);
  if (!statSync(leaf).isDirectory() || !existsSync(join(leaf, "🦀️.rs"))) continue;
  files.push({ file: join(leaf, "🦀️.rs"), glob: "use super::*;", keepGlob: false });
  for (const child of ["🔺️diff", "↩️inverse"]) if (existsSync(join(leaf, child, "🦀️.rs"))) files.push({ file: join(leaf, child, "🦀️.rs"), glob: "use super::super::*;", keepGlob: false });
  const tests = join(leaf, "🧪️tests");
  if (existsSync(tests)) for (const t of readdirSync(tests)) if (existsSync(join(tests, t, "🦀️.rs"))) files.push({ file: join(tests, t, "🦀️.rs"), glob: "use super::*;", keepGlob: true });
}
let changed = 0;
for (const { file, glob, keepGlob } of files) {
  const text = readFileSync(file, "utf8");
  const count = text.split("\n").filter((l) => l.trim() === glob).length;
  if (count !== 1) { console.log(`skip ${file.slice(-60)}: glob line count ${count}`); continue; }
  const own = new Set<string>();
  for (const item of parseRustSourceTree(text).items) { if (item.name) own.add(item.name); if (item.kind === "use") for (const l of useLeafNames(text.slice(item.declStart, item.end))) own.add(l); }
  const ids = identifierSet(text);
  const uses = new Map<string, string[]>();
  const items: string[] = [], mods: string[] = [];
  for (const [name, p] of provided) {
    if (!ids.has(name) || own.has(name) || !usedAsItem(text, name)) continue;
    if (p.kind === "use") { if (p.test && !keepGlob && file.includes("tests")) { } uses.set(p.statement, [...(uses.get(p.statement) ?? []), name]); if (p.test) uses.set("@test:" + p.statement, []); }
    else if (p.kind === "item") { items.push(name); if (p.vis === "private" || (p.vis !== "pub" && !file.includes("tests"))) widen.add(name); }
    else mods.push(name);
  }
  const lines: string[] = [];
  for (const [statement, leaves] of uses) { if (statement.startsWith("@test:") ) continue; const test = uses.has("@test:" + statement); lines.push(`${test ? "#[cfg(test)]\n" : ""}${prunedUse(statement, leaves)}`); }
  const paths = [...items, ...mods].sort();
  if (paths.length) lines.push(`use ${hostPath}::{${paths.join(", ")}};`);
  const replacement = (keepGlob ? [glob, ...lines] : lines).join("\n");
  const out = text.split("\n").map((l) => l.trim() === glob ? replacement : l).join("\n");
  console.log(`${write ? "write" : "plan "} ${file.slice(dir.length)} -> ${lines.length} import line(s): ${[...uses.values()].flat().concat(paths).join(", ")}`);
  if (write) writeFileSync(file, out);
  changed += 1;
}
if (widen.size) {
  const names = [...widen];
  console.log("widen to pub in aggregate:", names.join(", "));
  if (write) {
    let t = aggText;
    for (const name of names) {
      const p = provided.get(name) as Extract<Provided, { kind: "item" }>;
      const head = aggText.slice(p.declStart, p.declStart + 12);
      const re = new RegExp(`^(\\s*)(?:pub\\([a-z]+\\)\\s+)?(fn|const|static|struct|enum|type)\\s+${name}\\b`, "mu");
      if (!re.test(t)) { console.log("cannot widen", name, head); continue; }
      t = t.replace(re, `$1pub $2 ${name}`);
    }
    writeFileSync(aggFile, t);
  }
}
console.log("files", changed);
