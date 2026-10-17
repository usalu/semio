// 🧹️ Replaces the `use super::*;` glob of fragment modules by the explicit imports they need, and repoints outside references of glob-re-exported names at the module that defines them.
// usage: bun r8a-unglob.ts <hub-file> <hub-crate-path> [--write] [--repath=<dir>,...]
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { parseRustSourceTree, identifierSet, useLeafNames, prunedUse } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const { rustTokens } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
const args = process.argv.slice(2);
const flags = args.filter((a) => a.startsWith("--"));
const [hubFile, hubPath] = args.filter((a) => !a.startsWith("--")) as [string, string];
const write = flags.includes("--write");
const repath = (flags.find((f) => f.startsWith("--repath=")) ?? "").slice(9).split(",").filter(Boolean);
const quiet = flags.includes("--quiet");

type Use = { statement: string; leaves: string[]; test: boolean; pub: boolean; glob: string | null };
type Scope = { file: string; path: string; text: string; parent: Scope | null; items: Map<string, { vis: string; start: number }>; uses: Use[]; kids: Map<string, Scope>; superGlob: boolean; globs: string[] };

const traitNames = new Set<string>(
  execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "-h", "-o", "-E", "trait [A-Z][A-Za-z0-9_]*", "--", "🧰️framework/**/*.rs", "✏️s/🔌️plugins/🧩️puzzle/**/*.rs"], { cwd: "/Users/ueli/Documents/semio", maxBuffer: 1 << 28 }).toString("utf8").split("\n").map((l) => l.replace(/^trait /u, "").trim()).filter(Boolean),
);

const build = (file: string, path: string, parent: Scope | null): Scope => {
  const text = readFileSync(file, "utf8");
  const tree = parseRustSourceTree(text);
  const scope: Scope = { file, path, text, parent, items: new Map(), uses: [], kids: new Map(), superGlob: false, globs: [] };
  for (const item of tree.items) {
    if (item.kind === "use") {
      const statement = text.slice(item.declStart, item.end);
      const test = item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a));
      const glob = /^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)::\*\s*;\s*$/u.exec(statement.replace(/^(?:\s*#\[[^\]]*\]\s*)*/u, ""))?.[1] ?? null;
      const isSuper = /^\s*use\s+super::\*\s*;\s*$/u.test(statement.replace(/^(?:\s*#\[[^\]]*\]\s*)*/u, ""));
      if (isSuper) { scope.superGlob = true; continue; }
      scope.uses.push({ statement: statement.replace(/^(?:\s*#\[[^\]]*\]\s*)*/u, "").trim(), leaves: useLeafNames(statement), test, pub: item.visibility !== "private", glob });
      if (glob) scope.globs.push(glob);
    } else if (item.name && item.kind !== "impl" && item.kind !== "extern-crate" && item.kind !== "macro") scope.items.set(item.name, { vis: item.visibility, start: item.start });
  }
  for (const m of text.matchAll(/\b(?:pub(?:\([^)]*\))?\s+)?(?:enum|struct|trait)\s+([A-Z][A-Za-z0-9_]*)/gu)) if (!scope.items.has(m[1]!)) scope.items.set(m[1]!, { vis: m[0]!.startsWith("pub") ? "pub" : "private", start: m.index ?? 0 });
  for (const node of tree.modules) {
    if (node.conditionalTest || node.pathAttribute === null || node.inline) continue;
    const child = resolve(dirname(file), node.pathAttribute);
    if (existsSync(child)) scope.kids.set(node.name, build(child, `${path}::${node.name}`, scope));
  }
  return scope;
};

type Found = { kind: "item"; scope: Scope; vis: string } | { kind: "use"; use: Use; scope: Scope } | { kind: "mod"; scope: Scope };
const resolveName = (scope: Scope, name: string, depth = 0): Found | null => {
  if (depth > 8) return null;
  if (scope.items.has(name)) return { kind: "item", scope, vis: scope.items.get(name)!.vis };
  for (const use of scope.uses) if (use.leaves.includes(name) && !use.glob) return { kind: "use", use, scope };
  for (const glob of scope.globs) {
    const kid = scope.kids.get(glob);
    if (kid) { const found = resolveName(kid, name, depth + 1); if (found && found.kind === "item") return found; }
  }
  if (scope.kids.has(name)) return { kind: "mod", scope: scope.kids.get(name)! };
  if (scope.superGlob && scope.parent) return resolveName(scope.parent, name, depth + 1);
  return null;
};

const usedAsItem = (text: string, name: string): boolean => {
  const tokens = rustTokens(text);
  const upper = /^[A-Z]/u.test(name);
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i].text !== name) continue;
    const prev = tokens[i - 1]?.text, next = tokens[i + 1]?.text;
    if (prev === ".") continue;
    if (!upper && (prev === "let" || prev === "mut" || prev === "fn" || prev === "const" || prev === "static" || prev === "type" || prev === "mod")) continue;
    if (prev === "struct" || prev === "enum" || prev === "trait") continue;
    if (next === ":" && tokens[i + 2]?.text !== ":" && !upper) continue;
    if (upper || next === "(" || next === "::" || next === "!" || prev === "::" || prev === "&" || prev === "(" || prev === "," || prev === "=" || prev === "->" || prev === ":" || prev === "<" || prev === "{" || prev === "[" || prev === "|" || prev === "*" || prev === "!") return true;
  }
  return false;
};

const root = build(hubFile, hubPath, null);
const all: Scope[] = [];
const collect = (scope: Scope): void => { all.push(scope); for (const kid of scope.kids.values()) collect(kid); };
collect(root);

const widenTargets = new Map<string, Set<string>>();
const edits = new Map<string, string>();
const log: string[] = [];

const complete = flags.includes("--complete");
if (process.env.SHOWFILE) console.log("SCOPES", all.map((s) => s.path.replace("crate::editor::puzzle5d", "")).join(" "));
if (complete) for (const s of all) if (s.parent && !s.superGlob) s.superGlob = true;
for (const scope of all) {
  if (!scope.superGlob || !scope.parent) continue;
  const text = scope.text;
  const own = new Set<string>([...scope.items.keys(), ...scope.uses.flatMap((u) => u.leaves), ...scope.kids.keys()]);
  for (const node of parseRustSourceTree(text).items) if (node.name) own.add(node.name);
  const ids = identifierSet(text);
  const useBlocks = new Map<string, { test: boolean; leaves: Set<string>; statement: string }>();
  const itemsBy = new Map<string, Set<string>>();
  const seen = new Set<string>();
  const visibleNames = new Set<string>();
  const gather = (s: Scope, depth = 0): void => {
    if (depth > 8) return;
    for (const n of s.items.keys()) visibleNames.add(n);
    for (const u of s.uses) for (const l of u.leaves) visibleNames.add(l);
    for (const g of s.globs) { const k = s.kids.get(g); if (k) gather(k, depth + 1); }
    for (const k of s.kids.keys()) visibleNames.add(k);
    if (s.superGlob && s.parent) gather(s.parent, depth + 1);
  };
  gather(scope.parent);
  for (const name of visibleNames) {
    if (own.has(name) || !ids.has(name)) continue;
    const found = resolveName(scope.parent, name);
    if (!found) continue;
    const used = usedAsItem(text, name);
    if (found.kind === "use") {
      const isTrait = traitNames.has(name) && /^[A-Z]/u.test(name);
      if (!used && !isTrait) continue;
      const key = found.use.statement;
      const entry = useBlocks.get(key) ?? { test: found.use.test, leaves: new Set<string>(), statement: found.use.statement };
      entry.leaves.add(name);
      useBlocks.set(key, entry);
      seen.add(name);
    } else if (found.kind === "item") {
      if (!used) continue;
      const set = itemsBy.get(found.scope.path) ?? new Set<string>();
      set.add(name);
      itemsBy.set(found.scope.path, set);
      if (found.vis !== "pub") { const w = widenTargets.get(found.scope.file) ?? new Set<string>(); w.add(name); widenTargets.set(found.scope.file, w); }
      seen.add(name);
    } else if (found.kind === "mod") {
      if (!new RegExp(`(?<![\\w:])${name}::`, "u").test(text)) continue;
      const set = itemsBy.get(found.scope.parent!.path) ?? new Set<string>();
      set.add(name);
      itemsBy.set(found.scope.parent!.path, set);
    }
  }
  const lines: string[] = [];
  for (const entry of useBlocks.values()) {
    const pruned = prunedUse(entry.statement, [...entry.leaves]);
    if (!pruned) continue;
    const unusedTraitOnly = [...entry.leaves].every((l) => !usedAsItem(text, l));
    const attrs = [entry.test ? "#[cfg(test)]" : "", unusedTraitOnly ? "#[allow(unused_imports)]" : ""].filter(Boolean);
    lines.push(`${attrs.length ? attrs.join(" ") + " " : ""}${pruned}`);
  }
  for (const [path, names] of itemsBy) lines.push(`use ${path}::{${[...names].sort().join(", ")}};`);
  const globLine = /^use super::\*;[ \t]*$/mu;
  if (process.env.SHOWFILE && scope.file.includes(process.env.SHOWFILE)) console.log("DEBUG-LINES", lines.length, [...visibleNames].length, scope.parent.file.slice(-40));
  if (complete) {
    if (!lines.length) continue;
    const uses = parseRustSourceTree(text).items.filter((i: any) => i.kind === "use");
    const at = uses.length ? Math.max(...uses.map((i: any) => i.end)) : (text.match(/^(?:\/\/![^\n]*\n)+/u)?.[0].length ?? 0);
    edits.set(scope.file, text.slice(0, at) + "\n" + lines.join("\n") + text.slice(at));
    log.push(`complete ${scope.file.slice(dirname(hubFile).length)} -> ${lines.length}`);
    continue;
  }
  if (!globLine.test(text)) { log.push(`skip ${scope.file}: no plain glob line`); continue; }
  edits.set(scope.file, text.replace(globLine, lines.join("\n")));
  if (flags.includes("--show") && (scope.file.includes("ctx-tmp") || (process.env.SHOWFILE && scope.file.includes(process.env.SHOWFILE)))) console.log(lines.join("\n"));
  log.push(`${write ? "write" : "plan "} ${scope.file.slice(dirname(hubFile).length)} -> ${lines.length} import line(s)`);
}

const widenIn = (file: string, names: Set<string>): void => {
  let text = edits.get(file) ?? readFileSync(file, "utf8");
  for (const name of names) {
    const re = new RegExp(`^(\\s*)(?:pub\\((?:crate|super|self|in [a-zA-Z_:]+)\\)\\s+)?((?:const |async |unsafe )*(?:fn|const|static|struct|enum|type|trait))\\s+${name}\\b`, "mu");
    if (!re.test(text)) { log.push(`cannot widen ${name} in ${file.slice(-50)}`); continue; }
    text = text.replace(re, (_m, indent: string, kw: string) => `${indent}pub ${kw} ${name}`);
  }
  edits.set(file, text);
};
for (const [file, names] of widenTargets) widenIn(file, names);

for (const scope of all) {
  let text = edits.get(scope.file) ?? scope.text;
  let changed = false;
  for (const kid of scope.kids.keys()) {
    const re = new RegExp(`^(\\s*)mod ${kid};`, "mu");
    if (re.test(text)) { text = text.replace(re, `$1pub mod ${kid};`); changed = true; }
  }
  if (changed) edits.set(scope.file, text);
}

if (repath.length) {
  const exported = new Map<string, string>();
  for (const glob of root.globs) {
    const kid = root.kids.get(glob);
    const decl = root.uses.find((u) => u.glob === glob);
    if (!kid || !decl?.pub) continue;
    for (const name of kid.items.keys()) if (!root.items.has(name)) exported.set(name, kid.path);
  }
  const files: string[] = [];
  const walk = (dir: string): void => { for (const entry of readdirSync(dir)) { const p = join(dir, entry); if (statSync(p).isDirectory()) walk(p); else if (p.endsWith("🦀️.rs")) files.push(p); } };
  for (const dir of repath) walk(dir);
  const prefix = hubPath;
  for (const file of files) {
    let text = edits.get(file) ?? readFileSync(file, "utf8");
    const before = text;
    text = text.replace(new RegExp(`use\\s+${prefix.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}::\\{([^;]*?)\\};`, "gu"), (whole: string, body: string) => {
      const entries: string[] = [];
      let depth = 0, current = "";
      for (const ch of body) {
        if (ch === "{") depth += 1;
        if (ch === "}") depth -= 1;
        if (ch === "," && depth === 0) { entries.push(current.trim()); current = ""; } else current += ch;
      }
      if (current.trim()) entries.push(current.trim());
      const stay: string[] = [];
      const moved = new Map<string, string[]>();
      for (const entry of entries) {
        const m = /^([A-Za-z_][A-Za-z0-9_]*)(\s+as\s+\w+)?$/u.exec(entry);
        const target = m ? exported.get(m[1]!) : undefined;
        if (m && target) moved.set(target, [...(moved.get(target) ?? []), entry]);
        else stay.push(entry);
      }
      if (!moved.size) return whole;
      const out: string[] = [];
      if (stay.length) out.push(`use ${prefix}::{${stay.join(", ")}};`);
      for (const [target, list] of moved) out.push(`use ${target}::{${list.join(", ")}};`);
      return out.join("\n");
    });
    text = text.replace(new RegExp(`use\\s+${prefix.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}::([A-Za-z_][A-Za-z0-9_]*)((?:\\s+as\\s+\\w+)?);`, "gu"), (whole: string, name: string, alias: string) => {
      const target = exported.get(name);
      return target ? `use ${target}::${name}${alias};` : whole;
    });
    text = text.replace(new RegExp(`(?<![A-Za-z0-9_:])${prefix.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}::([A-Za-z_][A-Za-z0-9_]*)\\b`, "gu"), (whole: string, name: string) => {
      const target = exported.get(name);
      return target ? `${target}::${name}` : whole;
    });
    if (text !== before) { edits.set(file, text); log.push(`repath ${file.slice(-70)}`); }
  }
}

if (!quiet) for (const line of log) console.log(line);
console.log(`files ${edits.size}${write ? " written" : " planned"}`);
if (write) {
  const undo: Record<string, string> = {};
  for (const [file] of edits) undo[file] = readFileSync(file, "utf8");
  const tag = (flags.find((f) => f.startsWith("--tag=")) ?? "--tag=unglob").slice(6);
  writeFileSync(`/Users/ueli/.cache/r8a/undo-${tag}-${Date.now()}.json`, JSON.stringify(undo));
  for (const [file, text] of edits) writeFileSync(file, text);
}
