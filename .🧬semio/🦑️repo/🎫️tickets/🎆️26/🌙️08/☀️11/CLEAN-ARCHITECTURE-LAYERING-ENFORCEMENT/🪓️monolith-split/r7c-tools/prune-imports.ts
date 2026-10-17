// ✂️ Removes the private `use` leaves that nothing in the file, its inline modules or its descendant file modules names (token level; glob imports and `pub use` stay). usage: bun prune-imports.ts [--write] <file>...
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parseRustSourceTree, bindingsOf, rustTokens, type Binding } from "./lib-imports.ts";
import { createRequire } from "node:module";

const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { prunedUse } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const args = process.argv.slice(2);
const write = args.includes("--write");
const files = args.filter((a) => !a.startsWith("--"));
const REPO = "/Users/ueli/Documents/semio";
const logs = (process.env.PRUNE_WARN ?? "").split(",").filter(Boolean);
const localTraits = new Set<string>();
for (const dir of (process.env.PRUNE_TRAITS_DIR ?? "").split(",").filter(Boolean)) {
  const walkRs = (d: string): void => { for (const e of readdirSync(d, { withFileTypes: true })) { const p = join(d, e.name); if (e.isDirectory()) { if (e.name !== "node_modules" && e.name !== "target") walkRs(p); } else if (e.name.endsWith(".rs")) for (const m of readFileSync(p, "utf8").matchAll(/\btrait\s+([A-Za-z_][A-Za-z0-9_]*)/gu)) localTraits.add(m[1]!); } };
  walkRs(dir);
}
const warned = (path: string): Map<string, { line: number; names: Set<string> }[]> => {
  const out = new Map<string, { line: number; names: Set<string> }[]>();
  for (const l of readFileSync(path, "utf8").split("\n")) {
    const m = /^(.*?):(\d+):\d+: warning: unused imports?: (.*)$/u.exec(l);
    if (!m) continue;
    const at = m[1]!.indexOf("✏️s/");
    const file = at < 0 ? m[1]! : posixNormalize(m[1]!.slice(at));
    const names = new Set([...m[3]!.matchAll(/`([^`]+)`/gu)].map((x) => x[1]!.split("::").pop()!).filter((n) => n !== "*"));
    out.set(file, [...(out.get(file) ?? []), { line: Number(m[2]), names }]);
  }
  return out;
};
import { posix } from "node:path";
function posixNormalize(p: string): string { return posix.normalize(p); }
const warnedNative = logs.length === 2 ? warned(logs[0]!) : new Map(), warnedTest = logs.length === 2 ? warned(logs[1]!) : new Map();
const ORIG = process.env.PRUNE_ORIG ?? `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-prune`;

const descendants = (file: string, seen = new Set<string>()): string[] => {
  if (seen.has(file)) return [];
  seen.add(file);
  let text: string;
  try { text = readFileSync(file, "utf8"); } catch { return []; }
  const out: string[] = [];
  const visit = (nodes: readonly any[], base: string): void => {
    for (const n of nodes) {
      if (!n.inline) {
        if (n.pathAttribute) { const child = resolve(base, n.pathAttribute); out.push(child, ...descendants(child, seen)); }
      } else visit(n.children, n.pathAttribute ? resolve(base, n.pathAttribute) : base);
    }
  };
  visit(parseRustSourceTree(text).modules, dirname(file));
  return out;
};
const identsOutsideUses = (text: string): Set<string> => {
  const tree = parseRustSourceTree(text);
  const spans: [number, number][] = [];
  const collect = (items: readonly any[], nodes: readonly any[]): void => {
    for (const i of items) if (i.kind === "use") spans.push([i.declStart, i.end]);
    for (const n of nodes) collect(n.items, n.children);
  };
  collect(tree.items, tree.modules);
  const set = new Set<string>();
  for (const t of rustTokens(text)) {
    if (t.kind !== "identifier") continue;
    if (spans.some(([a, b]) => t.start >= a && t.start < b)) continue;
    set.add(t.text);
  }
  return set;
};

for (const file of files) {
  const text = readFileSync(file, "utf8");
  const used = identsOutsideUses(text);
  for (const d of descendants(file)) { try { for (const x of identsOutsideUses(readFileSync(d, "utf8"))) used.add(x); for (const t of rustTokens(readFileSync(d, "utf8"))) if (t.kind === "identifier") used.add(t.text); } catch { } }
  const tree = parseRustSourceTree(text);
  const edits: { start: number; end: number; text: string }[] = [];
  const log: string[] = [];
  const handle = (items: readonly any[], nodes: readonly any[]): void => {
    for (const item of items) {
      if (item.kind !== "use" || item.visibility !== "private") continue;
      const stmt = text.slice(item.declStart, item.end);
      if (/\*\s*;?\s*$/u.test(stmt.replace(/;\s*$/u, "")) && !/\{/u.test(stmt)) continue;
      const bs: Binding[] = bindingsOf(stmt, false);
      if (!bs.length) continue;
      const rel = file.slice(REPO.length + 1);
      const lineOf = (offset: number) => text.slice(0, offset).split("\n").length;
      const [l0, l1] = [lineOf(item.declStart), lineOf(item.end)];
      const hit = (m: Map<string, { line: number; names: Set<string> }[]>, name: string) => (m.get(rel) ?? []).some((w) => w.line >= l0 && w.line <= l1 && w.names.has(name));
      const testOnly = item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a));
      const dead = bs.filter((b) => !used.has(b.name) && b.name !== "_" && ((hit(warnedTest, b.name) && (testOnly || hit(warnedNative, b.name))) || (/^(crate|super|self)::/u.test(b.path) && !localTraits.has(b.name))));
      if (!dead.length) continue;
      const keep = bs.filter((b) => !dead.includes(b)).map((b) => b.name);
      log.push(`${dead.map((b) => b.name).join(",")}`);
      if (!keep.length) edits.push({ start: item.start, end: item.end, text: "" });
      else edits.push({ start: item.declStart, end: item.end, text: prunedUse(stmt, keep) });
    }
    for (const n of nodes) handle(n.items, n.children);
  };
  handle(tree.items, tree.modules);
  if (!edits.length) continue;
  let out = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  console.log(`${file.split("/").slice(-5).join("/")}: removed ${log.join(" | ")}`);
  if (write) {
    const bak = join(ORIG, file.slice(REPO.length));
    if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, text); }
    writeFileSync(file, out);
  }
}
