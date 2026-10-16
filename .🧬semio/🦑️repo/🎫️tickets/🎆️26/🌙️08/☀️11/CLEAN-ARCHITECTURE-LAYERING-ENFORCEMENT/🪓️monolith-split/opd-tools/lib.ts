// 🧰️ opd copy of the dm2 library on top of dd.ts: workspace membership of a lock, edge propagation across every lock, crate scaffolding for framework (project.json) and plugin (Cargo.toml only) crates.
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {dirname, join, relative} from "node:path";
import {execFileSync} from "node:child_process";
import {DRY, ROOT, abs, put, read, log, blocksOf, withDeps, nameOf, depsOf, cmp, newBlock, versionOf, packageNameOf} from "./dd.ts";
export * from "./dd.ts";

export const allLocks = (): string[] => execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", "*Cargo.lock"], {cwd: ROOT, maxBuffer: 1 << 28}).toString("utf8").split("\0").filter(p => p && !p.startsWith(".🧬semio/") && existsSync(abs(p)));

import {resolve} from "node:path";
const manifestPaths = (): string[] => execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", "*Cargo.toml"], {cwd: ROOT, maxBuffer: 1 << 28}).toString("utf8").split("\0").filter(p => p && !p.startsWith(".🧬semio/") && existsSync(abs(p)));
let nameIndex: Map<string, string[]> | undefined;
const manifestCache = new Map<string, any>();
const parsedManifest = (dir: string): any => {
  const key = dir;
  if (!manifestCache.has(key)) { try { manifestCache.set(key, Bun.TOML.parse(read(`${dir ? dir + "/" : ""}Cargo.toml`))); } catch { manifestCache.set(key, {}); } }
  return manifestCache.get(key);
};
const packageDirs = (name: string): string[] => {
  if (!nameIndex) {
    nameIndex = new Map();
    for (const m of manifestPaths()) {
      const dir = dirname(m) === "." ? "" : dirname(m);
      const parsed = parsedManifest(dir);
      if (parsed.package?.name) nameIndex.set(parsed.package.name, [...(nameIndex.get(parsed.package.name) ?? []), dir]);
    }
  }
  return nameIndex.get(name) ?? [];
};
const nearestWorkspaceRoot = (dir: string): string | undefined => {
  for (let cur = dir; ; cur = dirname(cur) === "." ? "" : dirname(cur)) {
    if (existsSync(abs(`${cur ? cur + "/" : ""}Cargo.toml`)) && parsedManifest(cur).workspace) return cur;
    if (cur === "") return undefined;
  }
};
const underRoot = (dir: string, root: string): boolean => root === "" || dir === root || dir.startsWith(root + "/");
export function isMember(lockPath: string, name: string): boolean {
  const root = dirname(lockPath) === "." ? "" : dirname(lockPath);
  for (const dir of packageDirs(name)) {
    if (!underRoot(dir, root)) continue;
    const parsed = parsedManifest(dir);
    const pointer = parsed.package?.workspace as string | undefined;
    const ownRoot = pointer ? relative(abs(""), resolve(abs(dir), pointer)).replaceAll("\\", "/") : nearestWorkspaceRoot(dir);
    if (ownRoot !== root) continue;
    const exclude = (parsedManifest(root).workspace?.exclude ?? []) as string[];
    const rel = root === "" ? dir : dir.slice(root.length + 1);
    if (exclude.some(e => new Bun.Glob(e.replace(/\/$/, "")).match(rel))) continue;
    return true;
  }
  return false;
}
export const pathBlock = (blocks: string[], name: string): number => blocks.findIndex(b => nameOf(b) === name && !/\nsource = /u.test(b));
const countNamed = (blocks: string[], name: string): number => blocks.filter(b => nameOf(b) === name).length;
export function depToken(blocks: string[], name: string): string {
  const same = blocks.filter(b => nameOf(b) === name);
  if (same.length === 1) return name;
  if (same.length === 0) throw new Error(`lock has no package ${name}`);
  throw new Error(`ambiguous dependency ${name}: ${same.map(versionOf).join(", ")}`);
}

const sourceBlockCache = new Map<string, string | undefined>();
const findSourceBlock = (name: string, version?: string, registry = false): string | undefined => {
  const key = `${name}@${version ?? ""}@${registry}`;
  if (sourceBlockCache.has(key)) return sourceBlockCache.get(key);
  let found: string | undefined;
  for (const lock of allLocks()) {
    const text = read(lock);
    if (!text.includes(`name = "${name}"\n`)) continue;
    for (const b of blocksOf(text).blocks) {
      if (nameOf(b) !== name) continue;
      const hasSource = /\nsource = /u.test(b);
      if (registry !== hasSource) continue;
      if (version && versionOf(b) !== version) continue;
      found = b;
      break;
    }
    if (found) break;
  }
  sourceBlockCache.set(key, found);
  return found;
};
export function ensurePackage(blocks: string[], token: string, trail: string[] = []): string {
  const [name, version] = token.split(" ") as [string, string | undefined];
  const same = blocks.filter(b => nameOf(b) === name);
  if (same.length === 1) return name;
  if (same.length > 1) {
    const exact = version && same.some(b => versionOf(b) === version);
    if (exact) return token;
    throw new Error(`ambiguous ${name} in target lock without ${version}`);
  }
  if (trail.includes(name)) return name;
  const source = findSourceBlock(name, version, false) ?? findSourceBlock(name, version, true);
  if (!source) throw new Error(`no source block for ${token}`);
  const copy = [...depsOf(source)].map(dep => ensurePackage(blocks, dep, [...trail, name]));
  const normalized = withDeps(source, copy);
  const next = blocks.findIndex(b => cmp(nameOf(b), name) > 0);
  blocks.splice(next < 0 ? blocks.length : next, 0, normalized.trimEnd() + "\n");
  return name;
}

export type Edge = {consumer: string; dep: string; dev: boolean};
export type NewPackage = {name: string; version: string; deps: string[]};
export function propagateEdges(edges: Edge[], created: NewPackage[]): {changed: string[]; skipped: string[]} {
  const changed: string[] = [], skipped: string[] = [];
  for (const lock of allLocks()) {
    let text = read(lock);
    if (!edges.some(e => text.includes(`name = "${e.consumer}"\n`))) continue;
    const {header, blocks} = blocksOf(text);
        let touched = false;
    for (const edge of edges) {
      const at = pathBlock(blocks, edge.consumer);
      if (at < 0) continue;
      if (edge.dev && !isMember(lock, edge.consumer)) { skipped.push(`${lock} (${edge.consumer} is not a member)`); continue; }
      const made = created.find(c => c.name === edge.dep);
      if (made && pathBlock(blocks, made.name) < 0) {
        const deps = made.deps.map(d => depToken(blocks, d));
        const next = blocks.findIndex(b => cmp(nameOf(b), made.name) > 0);
        blocks.splice(next < 0 ? blocks.length : next, 0, newBlock(made.name, made.version, deps));
      } else if (!made && countNamed(blocks, edge.dep) === 0) {
        try { ensurePackage(blocks, edge.dep); log(`  + ${edge.dep} (with closure) added to ${lock}`); } catch (error) { skipped.push(`${lock} (${edge.consumer} -> ${edge.dep}: ${String(error).slice(0, 80)})`); continue; }
      }
      const consumerAt = pathBlock(blocks, edge.consumer);
      const token = depToken(blocks, edge.dep);
      const current = depsOf(blocks[consumerAt]!);
      if (current.some(d => d === token || d.split(" ")[0] === edge.dep)) continue;
      blocks[consumerAt] = withDeps(blocks[consumerAt]!, [...current, token]);
      touched = true;
    }
    if (touched) {
      changed.push(lock);
      put(lock, `${header ? header + "\n" : ""}${blocks.map(b => b.trimEnd()).join("\n\n")}\n`);
    }
  }
  return {changed, skipped};
}
export function propagateRemovals(edges: Edge[]): string[] {
  const changed: string[] = [];
  for (const lock of allLocks()) {
    const text = read(lock);
    if (!edges.some(e => text.includes(`name = "${e.consumer}"\n`))) continue;
    const {header, blocks} = blocksOf(text);
        let touched = false;
    for (const edge of edges) {
      const at = pathBlock(blocks, edge.consumer);
      if (at < 0) continue;
      if (edge.dev && !isMember(lock, edge.consumer)) continue;
      const current = depsOf(blocks[at]!);
      const kept = current.filter(d => d.split(" ")[0] !== edge.dep);
      if (kept.length === current.length) continue;
      blocks[at] = withDeps(blocks[at]!, kept);
      touched = true;
    }
    if (touched) { changed.push(lock); put(lock, `${header ? header + "\n" : ""}${blocks.map(b => b.trimEnd()).join("\n\n")}\n`); }
  }
  return changed;
}

export function ownerRowInsert(name: string, root: string, manifest: string): void {
  const file = "📋️build-owners.json";
  const document = JSON.parse(read(file)) as {owners: {name: string; root: string; manifest: string; packages: string[]}[]};
  if (document.owners.some(o => o.name === name || o.root === root)) { log(`  owner row already present: ${name}`); return; }
  const at = document.owners.findIndex(o => Buffer.compare(Buffer.from(o.name), Buffer.from(name)) > 0);
  document.owners.splice(at < 0 ? document.owners.length : at, 0, {name, root, manifest, packages: []});
  put(file, JSON.stringify(document, null, 2) + "\n");
}

export function membersInsert(workspaceManifest: string, relMember: string): void {
  const text = read(workspaceManifest);
  const match = /members\s*=\s*\[([\s\S]*?)\]/u.exec(text);
  if (!match) throw new Error(`no members list in ${workspaceManifest}`);
  if (match[1]!.includes(`"${relMember}"`)) { log(`  member already present: ${relMember}`); return; }
  const lines = match[1]!.split("\n");
  const entries = lines.map((line, i) => ({line, i, m: /^\s*"([^"]+)",?\s*$/u.exec(line)})).filter(e => e.m);
  const indent = /^(\s*)"/u.exec(entries[0]!.line)![1]!;
  const after = entries.find(e => Buffer.compare(Buffer.from(e.m![1]!), Buffer.from(relMember)) > 0);
  const newLine = `${indent}"${relMember}",`;
  let nextLines: string[];
  if (after) { nextLines = [...lines.slice(0, after.i), newLine, ...lines.slice(after.i)]; }
  else {
    const last = entries[entries.length - 1]!;
    const fixed = last.line.replace(/,?\s*$/u, ",");
    nextLines = [...lines.slice(0, last.i), fixed, newLine.replace(/,$/u, last.line.trimEnd().endsWith(",") ? "," : ""), ...lines.slice(last.i + 1)];
  }
  const replaced = text.slice(0, match.index) + text.slice(match.index).replace(match[1]!, nextLines.join("\n"));
  put(workspaceManifest, replaced);
}

export type CrateSpec = {dir: string; name: string; description: string; role: string; libPath: string; workspaceRel: string; id?: string; docLib?: boolean; dependencies: string[]; features?: string[]; framework: boolean; ownerName?: string};
export function crateManifest(spec: CrateSpec): string {
  const lines = [
    "[package]",
    `workspace = "${spec.workspaceRel}"`,
    `name = "${spec.name}"`,
    ...(spec.framework ? [`version = "0.1.0"`, `edition = "2021"`] : [`version.workspace = true`, `edition.workspace = true`]),
    `rust-version.workspace = true`,
    `description = "${spec.description}"`,
    "",
    "[package.metadata.semio]",
    `role = "${spec.role}"`,
    ...(spec.id ? [`id = "${spec.id}"`] : []),
    "",
    ...(spec.framework ? ["[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", ""] : []),
    "[lints]",
    "workspace = true",
    "",
    "[lib]",
    `path = "${spec.libPath}"`,
    "doctest = false",
    ...(spec.features?.length ? ["", "[features]", ...spec.features] : []),
    "",
    "[dependencies]",
    ...spec.dependencies,
    "",
  ];
  return lines.join("\n");
}
export function createCrate(spec: CrateSpec): void {
  put(`${spec.dir}/Cargo.toml`, crateManifest(spec), true);
  if (spec.framework) {
    const slug = spec.ownerName!;
    put(`${spec.dir}/📋️project.json`, JSON.stringify({name: slug, tags: ["scope:framework", "language:rust"]}, null, 2) + "\n", true);
    ownerRowInsert(slug, spec.dir, `${spec.dir}/📋️project.json`);
  } else ownerRowInsert(spec.name, spec.dir, `${spec.dir}/Cargo.toml`);
}
export const depLine = (name: string, fromDir: string, toDir: string, extra = ""): string => `${name} = { path = "${relative(abs(fromDir), abs(toDir)).replaceAll("\\", "/")}"${extra} }`;
