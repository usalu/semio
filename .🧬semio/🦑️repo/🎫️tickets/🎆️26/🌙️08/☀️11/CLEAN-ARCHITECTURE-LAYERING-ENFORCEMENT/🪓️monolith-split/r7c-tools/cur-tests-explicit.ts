// 🧪️ Makes the harness-driven leaf tests of the sequence editor self-contained: `use super::*;` and `super::x` paths become explicit absolute imports of the host module, so the carve can hoist the test above every layer. usage: bun seq-tests-explicit.ts [--write]
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, bindingsOf, usedNames, renderUses, type Binding } from "./lib-imports.ts";

const write = process.argv.includes("--write");
const REPO = "/Users/ueli/Documents/semio";
const E = `${REPO}/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`;
const ORIG = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-cur-tests`;
const W = "crate::editor::sourcing";
const rows: { dir: string; host: string }[] = [
  { dir: "🎭️modes/✏️edit/🪟️windows/🏊️pool", host: `${W}::modes::edit::windows::pool` },
  { dir: "🎭️modes/✏️edit/🪟️windows/👁️preview", host: `${W}::modes::edit::windows::preview` },
  { dir: "🎭️modes/✏️edit/🪟️windows/🔢️grid", host: `${W}::modes::edit::windows::grid` },
  { dir: "🎭️modes/✏️edit/🪟️windows/🧺️curated", host: `${W}::modes::edit::windows::curated` },
  { dir: "🎮️commands/➕️curation-add", host: `${W}::commands::curation_add` },
  { dir: "🎮️commands/📉️set-filter-min-availability", host: `${W}::commands::set_filter_min_availability` },
  { dir: "🎮️commands/📥️load-document-json", host: `${W}::commands::load_document_json` },
];
const parentOf = (path: string) => path.split("::").slice(0, -1).join("::");
for (const { dir, host } of rows) {
  const hostFile = `${E}/${dir}/🦀️.rs`;
  const testFile = `${E}/${dir}/🧪️tests/🔬️unit/🦀️.rs`;
  const hostText = readFileSync(hostFile, "utf8");
  let testText = readFileSync(testFile, "utf8");
  const tree = parseRustSourceTree(hostText);
  const provided = new Map<string, { kind: "item"; vis: string } | { kind: "use"; binding: Binding } | { kind: "mod" }>();
  for (const item of tree.items) {
    if (item.kind === "use") {
      if (item.visibility !== "private") continue;
      const cfgTest = item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a));
      for (const b of bindingsOf(hostText.slice(item.declStart, item.end), cfgTest)) {
        const head = b.path.split("::")[0]!;
        if (head === "super") b.path = b.path.replace(/^super/u, parentOf(host));
        else if (head === "self") b.path = b.path.replace(/^self/u, host);
        provided.set(b.name, { kind: "use", binding: b });
      }
    } else if (item.name && item.kind !== "impl") provided.set(item.name, { kind: "item", vis: item.visibility });
  }
  for (const m of tree.modules) provided.set(m.name, { kind: "mod" });
  const lines = testText.split("\n");
  const globIndex = lines.findIndex((l) => l.trim() === "use super::*;");
  const bodyForUse = lines.filter((l) => l.trim() !== "use super::*;").join("\n");
  const own = new Set<string>();
  for (const item of parseRustSourceTree(testText).items) { if (item.name && item.kind !== "impl") own.add(item.name); if (item.kind === "use") for (const b of bindingsOf(testText.slice(item.declStart, item.end), false)) own.add(b.name); }
  const used = usedNames(bodyForUse, [...provided.keys()]);
  const hostNames: string[] = [], roots: Binding[] = [], widen: string[] = [];
  for (const name of [...used].sort()) {
    if (own.has(name)) continue;
    const p = provided.get(name)!;
    if (p.kind === "use") roots.push(p.binding);
    else { hostNames.push(name); if (p.kind === "item" && p.vis !== "pub") widen.push(name); }
  }
  const imports = [...renderUses(roots), ...(hostNames.length ? [`use ${host}::${hostNames.length === 1 ? hostNames[0] : `{${hostNames.join(", ")}}`};`] : [])];
  if (globIndex >= 0) lines.splice(globIndex, 1, ...imports);
  let out = lines.join("\n");
  out = out.replace(/\bsuper::([A-Za-z_][A-Za-z0-9_]*)/gu, (_m, name: string) => `${host}::${name}`);
  console.log(`${dir}: ${imports.length} import line(s); widen ${widen.join(",") || "-"}; remaining super:: ${/\bsuper::/u.test(out)}`);
  if (write) {
    const bak = join(ORIG, testFile.slice(REPO.length));
    if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, readFileSync(testFile, "utf8")); }
    writeFileSync(testFile, out);
    if (widen.length) {
      const hbak = join(ORIG, hostFile.slice(REPO.length));
      if (!existsSync(hbak)) { mkdirSync(dirname(hbak), { recursive: true }); writeFileSync(hbak, hostText); }
      let h = hostText;
      for (const name of widen) {
        const re = new RegExp(`^(\\s*)(?:pub\\([a-z]+\\)\\s+)?(fn|const|static|struct|enum|type)\\s+${name}\\b`, "mu");
        if (!re.test(h)) { console.log(`cannot widen ${name} in ${dir}`); continue; }
        h = h.replace(re, `$1pub $2 ${name}`);
      }
      writeFileSync(hostFile, h);
    }
  }
}
