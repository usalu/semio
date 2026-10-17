// 🧰 bd10: hoists test modules that reach upward crates into companion test crates. Mounts move, the test files stay at their taxonomy nodes.
// usage: bun hoist-tests.ts <spec.json> [--write]
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {walkCrate} from "./rust-mounts.ts";
import {parseRustSourceTree, useLeafNames, prunedUse, referencedItemNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {fixturesToRuntime, resolveFixturePlaceholders, absolutizeSuperPaths, hostPrivate, declaredNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
type Companion = {dir: string; doc: string; owners: string[]};
type Entry = {file: string; owner: string; companion: string; identOwner?: string; mountName?: string; rewriteOnly?: boolean};
type Spec = {widen?: boolean; crates: Record<string, {ident: string; root: string}>; companions: Record<string, Companion>; keepCrate?: string[]; files: Entry[]};
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as Spec;
const write = process.argv.includes("--write");
const rootSet = new Map<string, ReturnType<typeof walkCrate>>();
const walk = (owner: string) => { if (!rootSet.has(owner)) rootSet.set(owner, walkCrate(join(root, spec.crates[owner]!.root))); return rootSet.get(owner)!; };

const carryUses = (testText: string, parentText: string, parentModPath: readonly string[], ownerIdent: string): string[] => {
  const tree = parseRustSourceTree(parentText);
  const referenced = referencedItemNames(testText);
  const out: string[] = [];
  for (const item of tree.items) {
    if (item.kind !== "use" || item.visibility === "pub") continue;
    const statement = parentText.slice(item.declStart, item.end);
    const leaves = useLeafNames(statement).filter((name) => referenced.has(name) && !new RegExp(`\\buse\\b[^;]*\\b${name}\\b`, "u").test(testText));
    if (!leaves.length) continue;
    let text = prunedUse(statement, leaves);
    const abso = absolutizeSuperPaths(text, parentModPath);
    if (abso === null) continue;
    text = abso.replace(/(?<![\w:])crate::/gu, `${ownerIdent}::`).replace(/^\s*pub(\([^)]*\))?\s+/u, "");
    out.push(text.trim());
  }
  return out;
};
const insertUses = (text: string, uses: readonly string[]): string => {
  if (!uses.length) return text;
  const lines = text.split("\n");
  const at = lines.findIndex((line) => /^use /u.test(line));
  const block = uses.map((u) => (u.endsWith(";") ? u : u + ";"));
  if (at < 0) { const doc = lines.findLastIndex((line) => line.startsWith("//!")); lines.splice(doc + 1, 0, "", ...block); }
  else lines.splice(at, 0, ...block);
  return lines.join("\n");
};
const log: string[] = [];
const mounts: Record<string, string[]> = {};
const removals = new Map<string, {start: number; end: number}[]>();
const aliasNeeds: Record<string, Set<string>> = {};
const widenNames = new Map<string, Set<string>>();

for (const entry of spec.files) {
  const abs = join(root, entry.file);
  if (entry.rewriteOnly) {
    let plain = readFileSync(abs, "utf8");
    const keep = (spec.keepCrate ?? ["\u0000"]).join("|");
    const runtimeOnly = fixturesToRuntime(plain, abs, root);
    if (runtimeOnly !== null) plain = runtimeOnly;
    plain = plain.replace(new RegExp(`(?<![\\w:])crate::(?!(?:${keep})\\b)`, "gu"), `${spec.crates[entry.identOwner ?? entry.owner]!.ident}::`);
    plain = resolveFixturePlaceholders(plain, root, join(root, spec.companions[entry.companion]!.dir, "📦️packages/🦀️rust"));
    for (const alias of ["dsl", "protocol", "store", "schema"]) if (new RegExp(`(?<![\\w:])${alias}::`, "u").test(plain)) (aliasNeeds[entry.companion] ??= new Set()).add(alias);
    if (write) writeFileSync(abs, plain);
    log.push(`REWRITE ${entry.file.split("✳️any/")[1]}`);
    continue;
  }
  const walked = walk(entry.owner);
  const site = walked.sites.find((s) => s.target === abs);
  if (!site) { log.push(`NO MOUNT SITE ${entry.owner} ${entry.file}`); continue; }
  const companion = spec.companions[entry.companion]!;
  const ownerIdent = spec.crates[entry.identOwner ?? entry.owner]!.ident;
  let text = readFileSync(abs, "utf8");
  const runtime = fixturesToRuntime(text, abs, root);
  if (runtime !== null) text = runtime;
  const abso = absolutizeSuperPaths(text, site.modPath);
  if (abso === null) { log.push(`SUPER ESCAPES ${entry.file}`); continue; }
  text = abso.replace(new RegExp(`(?<![\\w:])crate::(?!(?:${(spec.keepCrate ?? ["\u0000"]).join("|")})\\b)`, "gu"), `${ownerIdent}::`);
  if (/(^|\n)\s*use\s+super::\*/u.test(readFileSync(abs, "utf8")) || new RegExp(`use\\s+${ownerIdent}${(walked.paths.get(site.file) ?? []).map((segment) => `::${segment}`).join("")}::\\*;`, "u").test(text)) text = insertUses(text, carryUses(text, readFileSync(site.file, "utf8"), walked.paths.get(site.file) ?? [], ownerIdent));
  text = resolveFixturePlaceholders(text, root, join(root, companion.dir, "📦️packages/🦀️rust"));
  for (const alias of ["dsl", "protocol", "store", "schema"]) if (new RegExp(`(?<![\\w:])${alias}::`, "u").test(text)) (aliasNeeds[entry.companion] ??= new Set()).add(alias);
  if (spec.widen !== false) {
    const original = readFileSync(abs, "utf8");
    const hp = hostPrivate(readFileSync(site.file, "utf8"), referencedItemNames(original), declaredNames(original, true), original, walked.paths.get(site.file) ?? []);
    if (hp.items.length) { const set = widenNames.get(site.file) ?? new Set<string>(); for (const name of hp.items) set.add(name); widenNames.set(site.file, set); }
  }
  const relPath = relative(join(root, companion.dir), abs);
  let name = site.node.name;
  const parent = site.modPath.length > 1 ? site.modPath.slice(-2, -1)[0]! : entry.owner;
  if (/^(tests|unit_tests|mutations_tests|pixels_tests|model_tests)$/u.test(name)) name = `${site.modPath.slice(0, -1).filter((segment) => segment !== "standards" && segment !== "v1" && segment !== "subsets" && segment !== "any").slice(-3).join("_") || parent}_tests`;
  else if (!name.startsWith(parent)) name = `${parent}_${name}`;
  if (entry.mountName) name = entry.mountName;
  const taken = new Set((mounts[entry.companion] ?? []).map((m) => /mod (\w+);/u.exec(m)![1]));
  if (taken.has(name)) name = `${entry.owner}_${name}`;
  (mounts[entry.companion] ??= []).push(`#[path = ${JSON.stringify(relPath)}]\nmod ${name};`);
  (removals.get(site.file) ?? removals.set(site.file, []).get(site.file)!).push({start: site.node.start, end: site.node.end});
  if (write) writeFileSync(abs, text);
  log.push(`OK ${entry.owner} -> ${entry.companion}: ${entry.file.split("✳️any/")[1]} as ${name} (mounted in ${relative(root, site.file).split("✳️any/")[1] ?? site.file})`);
}
if (write) {
  for (const [file, ranges] of removals) {
    let text = readFileSync(file, "utf8");
    for (const r of ranges.sort((a, b) => b.start - a.start)) {
      let start = r.start;
      while (start > 0 && (text[start - 1] === " " || text[start - 1] === "\t")) start -= 1;
      let end = r.end;
      if (text[end] === "\n") end += 1;
      text = text.slice(0, start) + text.slice(end);
    }
    writeFileSync(file, text);
  }
  for (const [file, names] of widenNames) {
    let text = readFileSync(file, "utf8");
    const tree = parseRustSourceTree(text);
    const edits: {start: number; end: number; text: string}[] = [];
    const structNames = new Set<string>();
    for (const name of names) {
      const target = tree.items.find((item) => item.name === name && !["use", "impl", "macro"].includes(item.kind)) ?? tree.modules.find((node) => node.name === name);
      if (!target) continue;
      const vis = /^pub(?:\([^)]*\))?\s*/u.exec(text.slice(target.declStart, target.declStart + 40));
      if (vis && vis[0].trim() === "pub") continue;
      edits.push({start: target.declStart, end: target.declStart + (vis ? vis[0].length : 0), text: "pub "});
      if ("kind" in target && (target as {kind: string}).kind === "struct") structNames.add(name);
    }
    for (const item of tree.items) {
      if (item.kind === "struct" && item.name && structNames.has(item.name)) {
        const vis = /^pub(?:\([^)]*\))?\s*/u.exec(text.slice(item.declStart, item.declStart + 40));
        const from = item.declStart + (vis ? vis[0].length : 0);
        const seg = text.slice(from, item.end);
        const widened = seg.replace(/(\{\s*)([\s\S]*)(\n\s*\})/u, (_all, open: string, body: string, close: string) => `${open}${body.replace(/^(\s*(?:#\[[^\n]*\]\s*)*)(?!pub)([a-z_][a-z0-9_]*\s*:)/gmu, "$1pub $2")}${close}`);
        if (widened !== seg) edits.push({start: from, end: item.end, text: widened});
      }
      if (item.kind === "impl") {
        const seg = text.slice(item.declStart, item.end);
        const head = /^\s*(?:#\[[^\n]*\]\s*)*impl(?:<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_]*)[^{]*\{/u.exec(seg);
        if (!head || !structNames.has(head[1]!) || /\bfor\b/u.test(head[0])) continue;
        const widened = seg.replace(/^(\s+)(?!pub)((?:async\s+|const\s+|unsafe\s+)*fn\s)/gmu, "$1pub $2");
        if (widened !== seg) edits.push({start: item.declStart, end: item.end, text: widened});
      }
    }
    for (const edit of edits.sort((a, b) => b.start - a.start || b.end - a.end)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
    writeFileSync(file, text);
    log.push(`WIDEN ${relative(root, file).split("✳️any/")[1] ?? file}: ${[...names].join(", ")}`);
  }
  for (const [key, list] of Object.entries(mounts)) {
    const companion = spec.companions[key]!;
    const existingFile = join(root, companion.dir, "🦀️.rs");
    const previous = existsSync(existingFile) ? readFileSync(existingFile, "utf8") : "";
    const previousMounts = [...previous.matchAll(/#\[path = "[^"]+"\]\nmod \w+;/gu)].map((m) => m[0]);
    for (const m of /extern crate \w+ as (\w+);/gu[Symbol.matchAll](previous)) (aliasNeeds[key] ??= new Set()).add(m[1]!);
    const body = [...previousMounts, ...list].join("\n");
    const target: Record<string, string> = {dsl: "semio_framework_os_kernel", protocol: "semio_framework_os_kernel", store: "semio_framework_os_kernel", schema: "semio_framework_schema"};
    const externs = [...(aliasNeeds[key] ?? [])].sort().map((a) => `extern crate ${target[a]} as ${a};`).join("\n");
    const dir = join(root, companion.dir);
    mkdirSync(dir, {recursive: true});
    writeFileSync(join(dir, "🦀️.rs"), `//! 🥣️ ${companion.doc}\n\n#![cfg(test)]\n\n${externs ? externs + "\n\n" : ""}${body}\n`);
  }
}
console.log(log.join("\n"));
console.log(JSON.stringify(Object.fromEntries(Object.entries(mounts).map(([k, v]) => [k, v.length]))));
