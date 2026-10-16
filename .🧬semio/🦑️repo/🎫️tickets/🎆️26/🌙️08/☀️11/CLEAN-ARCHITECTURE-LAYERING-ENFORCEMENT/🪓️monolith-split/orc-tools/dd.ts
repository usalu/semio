// 🧰️ (orc copy of the opd/dm2 helpers; backups in the session scratchpad orcpre, registries in orc-tools) Helpers for the shared-mount dedupe edits: exact text replacement, manifest sections, lock entries, workspace dependency tables, owner rows. Every write re-reads the file first.
import {copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, relative} from "node:path";
export const ROOT = "/Users/ueli/Documents/semio";
export const DRY = !process.argv.includes("--write");
export const abs = (p: string) => join(ROOT, p);
export const read = (p: string) => readFileSync(abs(p), "utf8");
export const out: string[] = [];
export const log = (line: string) => { out.push(line); console.log(line); };
const PRE = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/orcpre/";
const TOOLS = dirname(require("node:url").fileURLToPath(import.meta.url)) + "/";
const NEW_REGISTRY = join(TOOLS, "new-files.json");
const TOUCHED = join(TOOLS, "touched.txt");
export function backup(path: string): void {
  const dest = PRE + path;
  if (existsSync(dest) || !existsSync(abs(path))) return;
  mkdirSync(dirname(dest), {recursive: true});
  copyFileSync(abs(path), dest);
}
const noteTouched = (path: string): void => {
  const known = existsSync(TOUCHED) ? readFileSync(TOUCHED, "utf8").split("\n") : [];
  if (!known.includes(path)) writeFileSync(TOUCHED, [...known.filter(Boolean), path].join("\n") + "\n");
};
export function put(path: string, text: string, create = false): void {
  if (!create && !existsSync(abs(path))) throw new Error(`missing file ${path}`);
  if (create && existsSync(abs(path)) && read(path) !== text) throw new Error(`refusing to overwrite ${path}`);
  log(`${DRY ? "(dry) " : ""}${create ? "create" : "edit  "} ${path}`);
  if (DRY) return;
  if (create) {
    const registry: string[] = existsSync(NEW_REGISTRY) ? JSON.parse(readFileSync(NEW_REGISTRY, "utf8")) : [];
    if (!registry.includes(path)) writeFileSync(NEW_REGISTRY, JSON.stringify([...registry, path]));
  } else backup(path);
  noteTouched(path);
  mkdirSync(dirname(abs(path)), {recursive: true});
  writeFileSync(abs(path), text);
}
export function replaceOnce(path: string, find: string, replacement: string): void {
  const text = read(path);
  const count = text.split(find).length - 1;
  if (count !== 1) throw new Error(`anchor count ${count} in ${path}: ${find.slice(0, 80)}`);
  put(path, text.replace(find, () => replacement));
}
export function relPosix(fromDir: string, toDir: string): string {
  return relative(abs(fromDir), abs(toDir)).replaceAll("\\", "/");
}
const sectionBounds = (text: string, header: string): {start: number; end: number} | undefined => {
  const match = new RegExp(`^${header.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}[ \\t]*\\r?$`, "m").exec(text);
  if (!match) return undefined;
  const start = match.index + match[0].length;
  const next = /^\[/m.exec(text.slice(start));
  return {start, end: next ? start + next.index : text.length};
};
export function addToSection(manifestPath: string, header: string, line: string): void {
  const text = read(manifestPath);
  const key = line.split("=")[0]!.trim();
  const bounds = sectionBounds(text, header);
  if (!bounds) {
    put(manifestPath, `${text.trimEnd()}\n\n${header}\n${line}\n`);
    return;
  }
  const body = text.slice(bounds.start, bounds.end);
  if (new RegExp(`^${key.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*=`, "m").test(body)) { log(`  already present in ${manifestPath}: ${key}`); return; }
  const trimmed = body.replace(/\s+$/u, "");
  const tail = body.slice(trimmed.length);
  put(manifestPath, text.slice(0, bounds.start) + trimmed + "\n" + line + (tail.includes("\n\n") ? "\n\n" : "\n") + text.slice(bounds.end));
}
export function workspaceDependencyTable(workspaceManifest: string, name: string, path: string): void {
  const text = read(workspaceManifest);
  if (text.includes(`[workspace.dependencies.${name}]`)) { log(`  workspace dependency already present: ${name}`); return; }
  const indented = /^ +\[workspace\.dependencies\./m.test(text);
  const first = indented ? "    " : "";
  const block = `${first}[workspace.dependencies.${name}]\n${first}path = "${path}"\n`;
  const anchor = /^\[workspace\.lints/m.exec(text);
  if (anchor) put(workspaceManifest, text.slice(0, anchor.index).trimEnd() + "\n\n" + block + "\n" + text.slice(anchor.index));
  else put(workspaceManifest, text.trimEnd() + "\n\n" + block);
}
export const blocksOf = (lock: string): {header: string; blocks: string[]} => {
  const parts = lock.split(/\n(?=\[\[package\]\]\n)/u);
  const header = parts[0]!.startsWith("[[package]]") ? "" : parts.shift()!;
  return {header, blocks: parts.map(b => b.trimEnd() + "\n")};
};
export const cmp = (a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b));
export const nameOf = (block: string) => /name = "([^"]+)"/u.exec(block)![1]!;
export const versionOf = (block: string) => /version = "([^"]+)"/u.exec(block)![1]!;
export const depsOf = (block: string): string[] => { const m = /\ndependencies = \[([\s\S]*?)\n\]/u.exec(block); return m ? [...m[1]!.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map(e => e[1]!) : []; };
export const withDeps = (block: string, deps: string[]): string => {
  const sorted = [...new Set(deps)].sort(cmp);
  const rendered = sorted.length ? `dependencies = [\n${sorted.map(d => ` "${d}",`).join("\n")}\n]\n` : "";
  if (/\ndependencies = \[[\s\S]*?\n\]\n/u.test(block)) return block.replace(/dependencies = \[[\s\S]*?\n\]\n/u, rendered);
  const head = /^\[\[package\]\]\nname = "[^"]+"\nversion = "[^"]+"\n(?:source = "[^"]*"\n)?(?:checksum = "[^"]*"\n)?/u.exec(block)![0];
  return head + rendered + block.slice(head.length);
};
export const newBlock = (name: string, version: string, deps: string[]) => withDeps(`[[package]]\nname = "${name}"\nversion = "${version}"\n`, deps);
export function lockEnsure(lockPath: string, name: string, version: string, deps: string[], addTo: string[]): void {
  const {header, blocks} = blocksOf(read(lockPath));
  const at = blocks.findIndex(b => nameOf(b) === name && !/\nsource = /u.test(b));
  if (at >= 0) blocks[at] = withDeps(blocks[at]!, [...depsOf(blocks[at]!), ...deps]);
  else {
    const next = blocks.findIndex(b => cmp(nameOf(b), name) > 0);
    blocks.splice(next < 0 ? blocks.length : next, 0, newBlock(name, version, deps));
  }
  for (const consumer of addTo) {
    const index = blocks.findIndex(b => nameOf(b) === consumer && !/\nsource = /u.test(b));
    if (index < 0) throw new Error(`lock ${lockPath} has no path package ${consumer}`);
    blocks[index] = withDeps(blocks[index]!, [...depsOf(blocks[index]!), name]);
  }
  put(lockPath, `${header ? header + "\n" : ""}${blocks.map(b => b.trimEnd()).join("\n\n")}\n`);
}
export function lockRemoveDependency(lockPath: string, consumer: string, dep: string): void {
  const {header, blocks} = blocksOf(read(lockPath));
  const index = blocks.findIndex(b => nameOf(b) === consumer && !/\nsource = /u.test(b));
  if (index < 0) throw new Error(`lock ${lockPath} has no path package ${consumer}`);
  blocks[index] = withDeps(blocks[index]!, depsOf(blocks[index]!).filter(d => d !== dep));
  put(lockPath, `${header ? header + "\n" : ""}${blocks.map(b => b.trimEnd()).join("\n\n")}\n`);
}
export function lockDrop(lockPath: string, name: string): void {
  const {header, blocks} = blocksOf(read(lockPath));
  const kept = blocks.filter(b => !(nameOf(b) === name && !/\nsource = /u.test(b)));
  if (kept.length === blocks.length) { log(`  lock has no path package ${name}`); return; }
  put(lockPath, `${header ? header + "\n" : ""}${kept.map(b => b.trimEnd()).join("\n\n")}\n`);
}
export async function ownerRowsAddViaGenerator(rows: {name: string; root: string}[]): Promise<void> {
  const {insertBuildOwnerRows} = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts"));
  const file = "📋️build-owners.json";
  const {text, added} = insertBuildOwnerRows(read(file), rows);
  if (!added.length) { log("  owner rows already present"); return; }
  log(`  owner rows: ${added.join(", ")}`);
  put(file, text);
}
export function depPackages(manifestPath: string, workspaceManifestPath?: string, sections: string[] = ["dependencies"]): string[] {
  const manifest = Bun.TOML.parse(read(manifestPath)) as any;
  const wsDeps = workspaceManifestPath ? ((Bun.TOML.parse(read(workspaceManifestPath)) as any).workspace?.dependencies ?? {}) : {};
  const names = new Set<string>();
  const take = (table: Record<string, any> | undefined) => {
    for (const [key, spec] of Object.entries(table ?? {})) {
      const inherited = typeof spec === "object" && spec.workspace === true ? wsDeps[key] : undefined;
      const own = typeof spec === "object" ? spec.package : undefined;
      const inheritedPackage = typeof inherited === "object" ? inherited.package : undefined;
      names.add(own ?? inheritedPackage ?? key);
    }
  };
  for (const section of sections) take(manifest[section]);
  for (const target of Object.values(manifest.target ?? {}) as any[]) for (const section of sections) take(target[section]);
  return [...names].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
}
export function packageNameOf(manifestPath: string): string { return (Bun.TOML.parse(read(manifestPath)) as any).package.name; }
export function ownerRowRemove(name: string): void {
  const file = "📋️build-owners.json";
  const text = read(file);
  const at = text.indexOf(`"name": "${name}"`);
  if (at < 0) { log(`  owner row absent: ${name}`); return; }
  const start = text.lastIndexOf("\n    {\n", at) + 1;
  const endMarker = text.indexOf("\n    }", at);
  const end = text.indexOf("\n", endMarker + 1) + 1;
  if (start <= 0 || endMarker < 0) throw new Error(`cannot delimit owner row ${name}`);
  let next = text.slice(0, start) + text.slice(end);
  if (/,\n\s*\]/u.test(next.slice(start - 3, start + 12))) next = next.replace(/,(\n\s*\])/u, "$1");
  JSON.parse(next);
  put(file, next);
}
export function deleteFile(path: string): void {
  log(`${DRY ? "(dry) " : ""}delete ${path}`);
  if (!DRY) { backup(path); noteTouched(path); const { rmSync } = require("node:fs"); rmSync(abs(path)); }
}
export function replaceRegexOnce(path: string, pattern: RegExp, replacement: string): void {
  const text = read(path);
  const matches = [...text.matchAll(new RegExp(pattern.source, pattern.flags.includes("g") ? pattern.flags : pattern.flags + "g"))];
  if (matches.length !== 1) throw new Error(`pattern count ${matches.length} in ${path}: ${pattern}`);
  put(path, text.replace(pattern, () => replacement));
}
export function lockClosureFile(sourceLockPath: string, rootName: string, rootVersion: string, rootDeps: string[]): string {
  const {header, blocks} = blocksOf(read(sourceLockPath));
  const byName = new Map<string, string[]>();
  for (const b of blocks) byName.set(nameOf(b), [...(byName.get(nameOf(b)) ?? []), b]);
  const keep = new Set<string>();
  const stack = [...rootDeps];
  while (stack.length) {
    const n = stack.pop()!;
    if (keep.has(n)) continue;
    const found = byName.get(n);
    if (!found) throw new Error(`source lock lacks ${n}`);
    keep.add(n);
    for (const b of found) for (const d of depsOf(b)) stack.push(d.split(" ")[0]!);
  }
  const chosen = [newBlock(rootName, rootVersion, rootDeps), ...blocks.filter(b => keep.has(nameOf(b)))].sort((a, b) => cmp(nameOf(a), nameOf(b)));
  return `${header ? header + "\n" : ""}${chosen.map(b => b.trimEnd()).join("\n\n")}\n`;
}
