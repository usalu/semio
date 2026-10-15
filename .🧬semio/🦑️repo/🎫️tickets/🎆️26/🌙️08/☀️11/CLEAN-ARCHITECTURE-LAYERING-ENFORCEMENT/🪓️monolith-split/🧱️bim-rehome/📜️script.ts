#!/usr/bin/env bun
/**
 * 🧱️ Re-homes the inline mount manifest of the bim model artifact lib root into the files that own each mounted subtree.
 *
 * Commands:
 *   extract <entry.rs> <relativeTo>               print the resolved module tree of a crate as JSON
 *   plan <modelRoot> <planOut.json>               compute every edit in memory and write the plan (no source file is touched)
 *   apply <planOut.json>                          write the plan; refuses when any source changed since the plan was computed
 *   compare <orig.json> <new.json> <plan.json>    prove that the module trees are equal modulo the declared re-homing
 *
 * See `RESULT-BIM-MODEL-REHOME.md` in the parent folder for the design.
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, normalize, relative, resolve, sep } from "node:path";

type Token = { readonly kind: "id" | "punct" | "str" | "other"; readonly text: string; readonly start: number; readonly end: number };

const isIdStart = (c: string): boolean => /[\p{L}_]/u.test(c);
const isIdPart = (c: string): boolean => /[\p{L}\p{N}_]/u.test(c);

/** 🔤️ Rust lexer that keeps only what the module-declaration parser needs: identifiers, punctuation and opaque literals. */
function tokenize(src: string): Token[] {
  const out: Token[] = [];
  const n = src.length;
  let i = 0;
  while (i < n) {
    const c = src[i]!;
    if (/\s/u.test(c)) { i++; continue; }
    if (c === "/" && src[i + 1] === "/") { while (i < n && src[i] !== "\n") i++; continue; }
    if (c === "/" && src[i + 1] === "*") {
      let depth = 1;
      i += 2;
      while (i < n && depth > 0) {
        if (src[i] === "/" && src[i + 1] === "*") { depth++; i += 2; }
        else if (src[i] === "*" && src[i + 1] === "/") { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    const start = i;
    const raw = /^(?:br|rb|r)(#*)"/u.exec(src.slice(i, i + 12));
    if (raw && (c === "r" || c === "b")) {
      const hashes = raw[1]!;
      const closing = `"${hashes}`;
      const bodyStart = i + raw[0].length;
      const close = src.indexOf(closing, bodyStart);
      i = close < 0 ? n : close + closing.length;
      out.push({ kind: "str", text: src.slice(start, i), start, end: i });
      continue;
    }
    if (c === '"' || (c === "b" && src[i + 1] === '"')) {
      i += c === "b" ? 2 : 1;
      while (i < n && src[i] !== '"') i += src[i] === "\\" ? 2 : 1;
      i++;
      out.push({ kind: "str", text: src.slice(start, i), start, end: i });
      continue;
    }
    if (c === "'" || (c === "b" && src[i + 1] === "'")) {
      const q = c === "b" ? i + 1 : i;
      const after = src[q + 1];
      const isChar = after === "\\" || src[q + 2] === "'" || (after !== undefined && [...after].length === 1 && src[q + 1 + after.length] === "'");
      if (isChar) {
        i = q + 1;
        while (i < n && src[i] !== "'") i += src[i] === "\\" ? 2 : 1;
        i++;
        out.push({ kind: "str", text: src.slice(start, i), start, end: i });
        continue;
      }
      i = q + 1;
      while (i < n && isIdPart(src[i]!)) i++;
      out.push({ kind: "other", text: src.slice(start, i), start, end: i });
      continue;
    }
    if (isIdStart(c)) {
      while (i < n && isIdPart(src[i]!)) i++;
      out.push({ kind: "id", text: src.slice(start, i), start, end: i });
      continue;
    }
    if (/\p{N}/u.test(c)) {
      while (i < n && /[\p{L}\p{N}_.]/u.test(src[i]!) && !(src[i] === "." && !/\p{N}/u.test(src[i + 1] ?? ""))) i++;
      out.push({ kind: "other", text: src.slice(start, i), start, end: i });
      continue;
    }
    i++;
    out.push({ kind: "punct", text: c, start, end: i });
  }
  return out;
}

export type Attr = { readonly text: string; readonly path?: string; readonly cfg: boolean };
export type ModItem = { readonly kind: "mod"; readonly name: string; readonly vis: string; readonly attrs: Attr[]; readonly inline: boolean; readonly items: Item[]; readonly start: number; readonly end: number };
export type UseItem = { readonly kind: "use"; readonly text: string; readonly start: number; readonly end: number };
export type Item = ModItem | UseItem;

const OPEN = new Set(["(", "[", "{"]);
const CLOSE = new Set([")", "]", "}"]);

function matching(tokens: Token[], open: number): number {
  let depth = 0;
  for (let i = open; i < tokens.length; i++) {
    const t = tokens[i]!;
    if (t.kind !== "punct") continue;
    if (OPEN.has(t.text)) depth++;
    else if (CLOSE.has(t.text) && --depth === 0) return i;
  }
  throw new Error("unbalanced delimiter");
}

function parseAttr(src: string, tokens: Token[], i: number): { readonly attr: Attr | null; readonly next: number } {
  let j = i + 1;
  const inner = tokens[j]?.text === "!";
  if (inner) j++;
  if (tokens[j]?.text !== "[") throw new Error(`malformed attribute at ${tokens[i]!.start}`);
  const close = matching(tokens, j);
  const text = src.slice(tokens[i]!.start, tokens[close]!.end);
  if (inner) return { attr: null, next: close + 1 };
  const path = /^#\[\s*path\s*=\s*"((?:[^"\\]|\\.)*)"\s*\]$/u.exec(text)?.[1];
  return { attr: { text, path, cfg: /^#\[\s*cfg\s*\(/u.test(text) }, next: close + 1 };
}

/** 🧭️ Module-level item parser: reports `mod` declarations (inline or file) and `use` items, and skips every other item opaquely. */
export function parseItems(src: string, tokens: Token[] = tokenize(src), from = 0, to = tokens.length): Item[] {
  const items: Item[] = [];
  let i = from;
  while (i < to) {
    const first = tokens[i]!;
    if (first.kind === "punct" && first.text === ";") { i++; continue; }
    const attrs: Attr[] = [];
    let start = first.start;
    let sawAttr = false;
    while (i < to && tokens[i]!.text === "#" && tokens[i]!.kind === "punct") {
      const { attr, next } = parseAttr(src, tokens, i);
      if (attr) { attrs.push(attr); if (!sawAttr) start = tokens[i]!.start; sawAttr = true; }
      i = next;
    }
    if (i >= to) break;
    const itemTokenStart = i;
    let vis = "";
    if (tokens[i]!.kind === "id" && tokens[i]!.text === "pub") {
      vis = "pub";
      i++;
      if (tokens[i]?.text === "(") { const close = matching(tokens, i); vis = src.slice(tokens[i - 1]!.start, tokens[close]!.end).replace(/\s+/gu, ""); i = close + 1; }
    }
    const head = tokens[i]!;
    if (head.kind === "id" && head.text === "mod" && tokens[i + 1]?.kind === "id" && (tokens[i + 2]?.text === ";" || tokens[i + 2]?.text === "{")) {
      const name = tokens[i + 1]!.text;
      if (tokens[i + 2]!.text === ";") {
        items.push({ kind: "mod", name, vis, attrs, inline: false, items: [], start, end: tokens[i + 2]!.end });
        i += 3;
        continue;
      }
      const close = matching(tokens, i + 2);
      items.push({ kind: "mod", name, vis, attrs, inline: true, items: parseItems(src, tokens, i + 3, close), start, end: tokens[close]!.end });
      i = close + 1;
      continue;
    }
    if (head.kind === "id" && head.text === "use") {
      let j = i;
      let depth = 0;
      for (; j < to; j++) {
        const t = tokens[j]!;
        if (t.kind === "punct" && OPEN.has(t.text)) depth++;
        else if (t.kind === "punct" && CLOSE.has(t.text)) depth--;
        else if (t.kind === "punct" && t.text === ";" && depth === 0) break;
      }
      const end = tokens[Math.min(j, to - 1)]!.end;
      items.push({ kind: "use", text: src.slice(tokens[itemTokenStart]!.start, end).replace(/\s+/gu, " "), start, end });
      i = j + 1;
      continue;
    }
    let depth = 0;
    let j = i;
    for (; j < to; j++) {
      const t = tokens[j]!;
      if (t.kind !== "punct") continue;
      if (OPEN.has(t.text)) { if (t.text === "{" && depth === 0) { j = matching(tokens, j); break; } depth++; }
      else if (CLOSE.has(t.text)) depth--;
      else if (t.text === ";" && depth === 0) break;
    }
    i = j + 1;
  }
  return items;
}

export type Row = { readonly path: string[]; readonly kind: "file" | "inline"; readonly file: string; readonly missing: boolean; readonly vis: string; readonly cfg: string[]; readonly globChild: boolean };

/** 🌳️ Resolved module tree of a crate: every module path with its file (or declaring file when inline), visibility and accumulated `cfg`. Mirrors rustc's module-file resolution; every `#[path]`-loaded file owns its directory. */
export function extractTree(entry: string, relativeTo: string): Row[] {
  const rows: Row[] = [];
  const rel = (abs: string): string => relative(relativeTo, abs).split(sep).join("/");
  const seen = new Set<string>();
  const walkFile = (file: string, dirPath: string, modPath: string[], cfgs: string[]): void => {
    const key = `${file}\0${modPath.join("::")}`;
    if (seen.has(key)) return;
    seen.add(key);
    const src = readFileSync(file, "utf8");
    walkItems(parseItems(src), file, dirPath, modPath, cfgs);
  };
  const walkItems = (items: Item[], declaringFile: string, dirPath: string, modPath: string[], cfgs: string[]): void => {
    const globs = new Set(items.filter((x): x is UseItem => x.kind === "use").map((x) => /^pub use ([A-Za-z0-9_]+)::\*;$/u.exec(x.text)?.[1]).filter((x): x is string => x !== undefined));
    for (const item of items) {
      if (item.kind !== "mod") continue;
      const own = item.attrs.filter((a) => a.cfg).map((a) => a.text.replace(/\s+/gu, ""));
      const cfgsN = [...cfgs, ...own];
      const pathAttr = item.attrs.find((a) => a.path !== undefined)?.path;
      const path = [...modPath, item.name];
      if (item.inline) {
        rows.push({ path, kind: "inline", file: rel(declaringFile), missing: false, vis: item.vis, cfg: cfgsN, globChild: globs.has(item.name) });
        walkItems(item.items, declaringFile, pathAttr !== undefined ? normalize(join(dirPath, pathAttr)) : join(dirPath, item.name), path, cfgsN);
        continue;
      }
      let file: string;
      let childDir: string;
      if (pathAttr !== undefined) {
        file = normalize(join(dirPath, pathAttr));
        childDir = dirname(file);
      } else {
        const flat = join(dirPath, `${item.name}.rs`);
        const nested = join(dirPath, item.name, "mod.rs");
        file = existsSync(flat) ? flat : nested;
        childDir = existsSync(flat) ? join(dirPath, item.name) : dirname(nested);
      }
      const missing = !existsSync(file);
      rows.push({ path, kind: "file", file: rel(file), missing, vis: item.vis, cfg: cfgsN, globChild: globs.has(item.name) });
      if (!missing) walkFile(file, childDir, path, cfgsN);
    }
  };
  walkFile(resolve(entry), dirname(resolve(entry)), [], []);
  return rows;
}

/** 🪞️ Effective path: a module declared inside a glob-re-exported child (`mod component; pub use component::*;`) is addressed through its parent, so the hop segment is dropped. */
export function effectivePaths(rows: Row[]): Map<string, Row> {
  const byPath = new Map(rows.map((r) => [r.path.join("::"), r]));
  const out = new Map<string, Row>();
  for (const row of rows) {
    const eff: string[] = [];
    for (let i = 0; i < row.path.length; i++) {
      const prefix = row.path.slice(0, i + 1).join("::");
      const isLast = i === row.path.length - 1;
      if (!isLast && byPath.get(prefix)?.globChild) continue;
      eff.push(row.path[i]!);
    }
    let key = eff.join("::");
    for (let n = 2; out.has(key); n++) key = `${eff.join("::")}#${n}`;
    out.set(key, row);
  }
  return out;
}

type FsPlan = { readonly file: string; readonly mode: "create" | "append" | "replace"; readonly baseSha?: string; readonly content: string };
type Plan = { readonly modelRoot: string; readonly files: FsPlan[]; readonly created: string[]; readonly summary: Record<string, number> };

const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const posix = (p: string): string => p.split(sep).join("/");

const COLLECTION_DOC: Record<string, string> = {
  "🎮️commands": "BIM {surface} commands: every command module of the surface, each mounted from its own folder.",
  "🎭️modes": "BIM {surface} modes: every mode of the surface, each mounted from its own folder.",
  "📌️panels": "BIM {surface} panels: every panel of the editor, each mounted from its own folder.",
  "📚️examples": "BIM examples: the demo, the house and the office documents with the unit-test checks.",
};

const COLLECTION_DIRS: Record<string, string> = { commands: "🎮️commands", modes: "🎭️modes", panels: "📌️panels", examples: "📚️examples" };

type Ctx = { readonly file: string; readonly dir: string; readonly lines: string[]; readonly indent: number };

/** ⬆️ Longest `super::super::…` chain (also `pub(super)`) anywhere in a source text. */
export function maxSuperClimb(src: string): number {
  const t = tokenize(src);
  let max = 0;
  for (let i = 0; i < t.length; i++) {
    if (t[i]!.kind !== "id" || t[i]!.text !== "super") continue;
    if (t[i - 1]?.text === ":" && t[i - 2]?.text === ":" ) continue;
    let k = 1;
    let j = i;
    while (t[j + 1]?.text === ":" && t[j + 2]?.text === ":" && t[j + 3]?.text === "super") { k++; j += 3; }
    if (k > max) max = k;
  }
  return max;
}

const ITEM_KEYWORDS = new Set(["fn", "struct", "enum", "union", "trait", "type", "const", "static", "mod"]);

/** 🏷️ Names bound at the top level of a source text: items and `use` leaves/aliases (approximation, used only to refuse a move that would collide). */
export function topLevelNames(src: string): Set<string> {
  const t = tokenize(src);
  const names = new Set<string>();
  let depth = 0;
  for (let i = 0; i < t.length; i++) {
    const x = t[i]!;
    if (x.kind === "punct" && OPEN.has(x.text)) { depth++; continue; }
    if (x.kind === "punct" && CLOSE.has(x.text)) { depth--; continue; }
    if (depth !== 0 || x.kind !== "id") continue;
    if (ITEM_KEYWORDS.has(x.text) && t[i + 1]?.kind === "id") { names.add(t[i + 1]!.text); continue; }
    if (x.text === "use") {
      let d = 0;
      let last: string | undefined;
      for (i = i + 1; i < t.length; i++) {
        const y = t[i]!;
        if (y.kind === "punct" && OPEN.has(y.text)) d++;
        else if (y.kind === "punct" && CLOSE.has(y.text)) d--;
        else if (y.kind === "punct" && y.text === ";" && d === 0) break;
        else if (y.kind === "id" && !["use", "pub", "crate", "self", "super"].includes(y.text)) {
          if (y.text === "as") { const alias = t[i + 1]; if (alias?.kind === "id") { if (last) names.delete(last); names.add(alias.text); last = alias.text; i++; } continue; }
          if (t[i + 1]?.text === ":" && t[i + 2]?.text === ":") continue;
          names.add(y.text);
          last = y.text;
        }
      }
    }
  }
  return names;
}

class Rehomer {
  readonly outputs = new Map<string, { mode: "create" | "append" | "replace"; lines: string[]; baseSha?: string }>();
  readonly created: string[] = [];
  readonly summary: Record<string, number> = { hop: 0, newLeafFile: 0, newCollectionFile: 0, inlineKept: 0, fileMount: 0 };
  readonly pinned: string[] = [];
  private readonly depthOfFile = new Map<string, string[]>();
  private readonly climbCache = new Map<string, number>();
  private readonly nameCache = new Map<string, Set<string>>();

  constructor(private readonly src: string, private readonly rootFile: string, rows: Row[]) {
    const rootDir = dirname(rootFile);
    for (const row of rows) if (row.kind === "file" && !row.missing) this.depthOfFile.set(normalize(join(rootDir, row.file)), row.path);
  }

  private climb(file: string): number {
    let k = this.climbCache.get(file);
    if (k === undefined) { k = maxSuperClimb(readFileSync(file, "utf8")); this.climbCache.set(file, k); }
    return k;
  }

  private namesOf(file: string): Set<string> {
    let names = this.nameCache.get(file);
    if (!names) { names = topLevelNames(readFileSync(file, "utf8")); this.nameCache.set(file, names); }
    return names;
  }

  /** 🛡️ A child may move into its parent's component only when no file below it climbs more `super`s than it is nested below the parent (the hop adds one level) and the component does not already bind the child's name. */
  private hoppable(child: ModItem, parentRaw: string[], compFile: string, why: string[]): boolean {
    if (this.namesOf(compFile).has(child.name)) { why.push(`${[...parentRaw, child.name].join("::")}: name bound by ${basename(compFile)}`); return false; }
    const childRaw = [...parentRaw, child.name];
    for (const [file, raw] of this.depthOfFile) {
      if (raw.length < childRaw.length || childRaw.some((seg, i) => raw[i] !== seg)) continue;
      const m = raw.length - parentRaw.length;
      const k = this.climb(file);
      if (k > m) { why.push(`${childRaw.join("::")}: ${relative(dirname(this.rootFile), file)} climbs ${k} > depth ${m}`); return false; }
    }
    return true;
  }

  private target(file: string): Ctx {
    let entry = this.outputs.get(file);
    if (!entry) {
      entry = { mode: "append", lines: [], baseSha: sha(readFileSync(file, "utf8")) };
      this.outputs.set(file, entry);
    }
    return { file, dir: dirname(file), lines: entry.lines, indent: 0 };
  }

  private fresh(file: string, doc: string): Ctx {
    if (existsSync(file) || this.outputs.has(file)) throw new Error(`refusing to create an existing file: ${file}`);
    const lines = [doc, ""];
    this.outputs.set(file, { mode: "create", lines });
    this.created.push(file);
    return { file, dir: dirname(file), lines, indent: 0 };
  }

  private push(ctx: Ctx, text: string): void {
    ctx.lines.push(`${"    ".repeat(ctx.indent)}${text}`);
  }

  private relPath(ctx: Ctx, abs: string): string {
    const r = posix(relative(ctx.dir, abs));
    return r === "" ? "." : r;
  }

  private attrLines(ctx: Ctx, mod: ModItem, dirPath: string, rewritePathTo?: string): void {
    for (const a of mod.attrs) {
      if (a.path === undefined) { this.push(ctx, a.text); continue; }
      if (mod.inline) { this.push(ctx, a.text); continue; }
      const abs = normalize(join(dirPath, a.path));
      this.push(ctx, `#[path = ${JSON.stringify(rewritePathTo ?? this.relPath(ctx, abs))}]`);
    }
  }

  private declLine(mod: ModItem): string {
    return `${mod.vis ? `${mod.vis} ` : ""}mod ${mod.name}`;
  }

  private innerDirOf(mod: ModItem, dirPath: string): string {
    const pathAttr = mod.attrs.find((a) => a.path !== undefined)?.path;
    return pathAttr !== undefined ? normalize(join(dirPath, pathAttr)) : join(dirPath, mod.name);
  }

  private descendantFiles(mod: ModItem, dirPath: string, out: string[] = []): string[] {
    const inner = this.innerDirOf(mod, dirPath);
    for (const item of mod.items) {
      if (item.kind !== "mod") continue;
      if (item.inline) { this.descendantFiles(item, inner, out); continue; }
      const p = item.attrs.find((a) => a.path !== undefined)?.path;
      if (p !== undefined) out.push(normalize(join(inner, p)));
    }
    return out;
  }

  private commonDir(files: string[]): string {
    const parts = files.map((f) => dirname(f).split(sep));
    let common = parts[0]!;
    for (const p of parts) { let k = 0; while (k < common.length && k < p.length && common[k] === p[k]) k++; common = common.slice(0, k); }
    return common.join(sep);
  }

  /** 🧩️ Emits one module declaration into `ctx`, re-homing any children that the shape rules move to an owning file. `dirPath` is the directory its `#[path]` attributes resolved against in the original manifest. */
  emit(mod: ModItem, dirPath: string, ctx: Ctx, parentRaw: string[]): void {
    const raw = [...parentRaw, mod.name];
    if (!mod.inline) {
      this.attrLines(ctx, mod, dirPath);
      this.push(ctx, `${this.declLine(mod)};`);
      this.summary.fileMount!++;
      return;
    }
    const pathAttr = mod.attrs.find((a) => a.path !== undefined)?.path;
    const innerDir = pathAttr !== undefined ? normalize(join(dirPath, pathAttr)) : join(dirPath, mod.name);
    const children = mod.items.filter((x): x is ModItem => x.kind === "mod");
    const globs = new Set(mod.items.filter((x): x is UseItem => x.kind === "use").map((x) => /^pub use ([A-Za-z0-9_]+)::\*;$/u.exec(x.text)?.[1]).filter((x): x is string => x !== undefined));
    const comp = children.find((c) => !c.inline && globs.has(c.name) && c.attrs.some((a) => a.path !== undefined));
    const compFile = comp ? normalize(join(innerDir, comp.attrs.find((a) => a.path !== undefined)!.path!)) : undefined;
    const others = children.filter((c) => c !== comp);
    const allFiles = this.descendantFiles(mod, dirPath);

    const emitInlineBlock = (inner: (inner: Ctx) => void): void => {
      this.attrLines(ctx, mod, dirPath);
      this.push(ctx, `${this.declLine(mod)} {`);
      inner({ ...ctx, indent: ctx.indent + 1 });
      this.push(ctx, "}");
    };

    if (comp && compFile) {
      if (others.length === 0) {
        emitInlineBlock((inner) => { for (const item of mod.items) this.emitItem(item, innerDir, inner, raw); });
        this.summary.inlineKept!++;
        return;
      }
      const common = this.commonDir(allFiles);
      if (common === dirname(compFile) && compFile.endsWith(`${sep}🦀️.rs`)) {
        const why: string[] = [];
        const movable = others.filter((child) => this.hoppable(child, raw, compFile, why));
        this.pinned.push(...why);
        emitInlineBlock((inner) => { for (const item of mod.items) if (item.kind === "use" || item === comp || (item.kind === "mod" && !movable.includes(item))) this.emitItem(item, innerDir, inner, raw); });
        if (movable.length) {
          const owner = this.target(compFile);
          for (const child of movable) this.emit(child, innerDir, owner, raw);
          this.summary.hop!++;
        }
        return;
      }
      const leaf = join(common, "🦀️.rs");
      const emoji = /^[^a-z0-9]+/u.exec(common.split(sep).at(-1)!)?.[0] ?? "🧩️";
      const slug = common.split(sep).at(-1)!.slice(emoji.length);
      const abs = leaf;
      const doc = `//! ${emoji} \`${slug}\` mutation leaf: mounts the payload, its behavior facets and its tests under one module.`;
      this.attrLines(ctx, { ...mod, attrs: mod.attrs.filter((a) => a.path === undefined) }, dirPath);
      this.push(ctx, `#[path = ${JSON.stringify(this.relPath(ctx, abs))}]`);
      this.push(ctx, `${this.declLine(mod)};`);
      const owner = this.fresh(abs, doc);
      for (const item of mod.items) this.emitItem(item, innerDir, owner, raw);
      this.summary.newLeafFile!++;
      return;
    }

    const common = allFiles.length ? this.commonDir(allFiles) : "";
    const collectionDir = COLLECTION_DIRS[mod.name];
    const segments = common.split(sep);
    const at = collectionDir === undefined ? -1 : segments.lastIndexOf(collectionDir);
    if (collectionDir !== undefined && at >= 0 && !existsSync(join(segments.slice(0, at + 1).join(sep), "🦀️.rs"))) {
      const base = collectionDir;
      const common = segments.slice(0, at + 1).join(sep);
      const surface = common.includes(`${sep}👁️viewer${sep}`) ? "viewer" : "editor";
      const emoji = /^[^a-z0-9]+/u.exec(base)?.[0] ?? "🧩️";
      const doc = `//! ${emoji} ${COLLECTION_DOC[base]!.replace("{surface}", surface)}`;
      const abs = join(common, "🦀️.rs");
      this.attrLines(ctx, { ...mod, attrs: mod.attrs.filter((a) => a.path === undefined) }, dirPath);
      this.push(ctx, `#[path = ${JSON.stringify(this.relPath(ctx, abs))}]`);
      this.push(ctx, `${this.declLine(mod)};`);
      const owner = this.fresh(abs, doc);
      for (const item of mod.items) this.emitItem(item, innerDir, owner, raw);
      this.summary.newCollectionFile!++;
      return;
    }
    emitInlineBlock((inner) => { for (const item of mod.items) this.emitItem(item, innerDir, inner, raw); });
    this.summary.inlineKept!++;
  }

  private emitItem(item: Item, dirPath: string, ctx: Ctx, parentRaw: string[]): void {
    if (item.kind === "use") { this.push(ctx, item.text); return; }
    this.emit(item, dirPath, ctx, parentRaw);
  }

  run(): { readonly head: string; readonly rootLines: string[] } {
    const text = readFileSync(this.rootFile, "utf8");
    const items = parseItems(text);
    const mods = items.filter((x): x is ModItem => x.kind === "mod");
    const firstManifest = mods.find((m) => m.inline);
    if (!firstManifest) throw new Error("no inline manifest module found");
    const head = text.slice(0, firstManifest.start);
    const tail = text.slice(Math.max(...mods.map((m) => m.end)));
    if (tail.trim() !== "") throw new Error(`unexpected tail after manifest: ${JSON.stringify(tail.slice(0, 80))}`);
    const rootDir = dirname(this.rootFile);
    const ctx: Ctx = { file: this.rootFile, dir: rootDir, lines: [], indent: 0 };
    for (const m of mods) if (m.inline) { this.emit(m, rootDir, ctx, []); ctx.lines.push(""); }
    return { head, rootLines: ctx.lines };
  }
}

function cmdPlan(modelRoot: string, planOut: string): void {
  const root = resolve(modelRoot);
  const rootFile = join(root, "🦀️.rs");
  const original = readFileSync(rootFile, "utf8");
  const rehomer = new Rehomer(original, rootFile, extractTree(rootFile, root));
  const { head, rootLines } = rehomer.run();
  const files: FsPlan[] = [{ file: rootFile, mode: "replace", baseSha: sha(original), content: `${head}${rootLines.join("\n").replace(/\n+$/u, "")}\n` }];
  for (const [file, entry] of rehomer.outputs) {
    if (entry.mode === "create") { files.push({ file, mode: "create", content: `${entry.lines.join("\n").replace(/\n+$/u, "")}\n` }); continue; }
    const base = readFileSync(file, "utf8");
    if (base.includes("//#region 🧩️Modules")) throw new Error(`${file} already has a 🧩️Modules region`);
    const block = ["//#region 🧩️Modules", ...entry.lines, "//#endregion 🧩️Modules"].join("\n");
    files.push({ file, mode: "append", baseSha: entry.baseSha, content: `${base.replace(/\n*$/u, "\n")}\n${block}\n` });
  }
  const plan: Plan = { modelRoot: root, files, created: rehomer.created, summary: rehomer.summary };
  writeFileSync(planOut, JSON.stringify(plan, null, 1));
  console.log(JSON.stringify({ files: files.length, created: rehomer.created.length, summary: rehomer.summary, rootLines: files[0]!.content.split("\n").length, pinned: rehomer.pinned }, null, 1));
}

function cmdApply(planFile: string, remapTo?: string): void {
  const loaded = JSON.parse(readFileSync(planFile, "utf8")) as Plan;
  const plan: Plan = remapTo === undefined ? loaded : { ...loaded, modelRoot: resolve(remapTo), files: loaded.files.map((f) => ({ ...f, file: join(resolve(remapTo), relative(loaded.modelRoot, f.file)) })), created: loaded.created.map((f) => join(resolve(remapTo), relative(loaded.modelRoot, f))) };
  for (const f of plan.files) {
    if (f.mode === "create") { if (existsSync(f.file)) throw new Error(`exists: ${f.file}`); continue; }
    if (sha(readFileSync(f.file, "utf8")) !== f.baseSha) throw new Error(`changed since plan: ${f.file}`);
  }
  const order = [...plan.files.filter((f) => f.mode === "create"), ...plan.files.filter((f) => f.mode === "append"), ...plan.files.filter((f) => f.mode === "replace")];
  for (const f of order) {
    if (f.mode === "create") mkdirSync(dirname(f.file), { recursive: true });
    writeFileSync(f.file, f.content);
  }
  console.log(`applied ${order.length} files`);
}

function cmdCompare(origFile: string, newFile: string, planFile: string): void {
  const orig = effectivePaths(JSON.parse(readFileSync(origFile, "utf8")) as Row[]);
  const next = effectivePaths(JSON.parse(readFileSync(newFile, "utf8")) as Row[]);
  const plan = JSON.parse(readFileSync(planFile, "utf8")) as Plan;
  const created = new Set(plan.created.map((f) => posix(relative(plan.modelRoot, f))));
  const problems: string[] = [];
  let converted = 0;
  for (const [key, a] of orig) {
    const b = next.get(key);
    if (!b) { problems.push(`missing in new: ${key}`); continue; }
    if (a.vis !== b.vis) problems.push(`visibility ${key}: ${a.vis} != ${b.vis}`);
    if (a.cfg.join("|") !== b.cfg.join("|")) problems.push(`cfg ${key}: [${a.cfg.join(",")}] != [${b.cfg.join(",")}]`);
    if (a.missing !== b.missing) problems.push(`missing flag ${key}`);
    if (a.kind === b.kind && (a.kind === "inline" || a.file === b.file)) continue;
    if (a.kind === "inline" && b.kind === "file" && created.has(b.file)) { converted++; continue; }
    problems.push(`target ${key}: ${a.kind}:${a.file} != ${b.kind}:${b.file}`);
  }
  for (const key of next.keys()) if (!orig.has(key)) problems.push(`extra in new: ${key}`);
  const result = { origModules: orig.size, newModules: next.size, inlineToFileConversions: converted, problems: problems.length };
  console.log(JSON.stringify(result));
  if (problems.length) { console.log(problems.slice(0, 50).join("\n")); process.exit(1); }
}

type Diagnostic = { readonly level: string; readonly code: string | null; readonly message: string; readonly file: string; readonly label: string };

const CRATE_NAME = "semio_s_artifact_bim_model";
const TOOLCHAIN = "+nightly-2026-07-20";

/** 🔬️ Full-resolution oracle run: `rustc --emit=metadata` from the lib root; every diagnostic is kept as `{level, code, message, file relative to the model root, primary span label}` and line numbers are dropped. */
function cmdOracle(modelRoot: string, outFile: string, flags: string[]): void {
  const root = resolve(modelRoot);
  const scratch = `${outFile}.rmeta`;
  const run = Bun.spawnSync(["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", CRATE_NAME, "--emit=metadata", "-o", scratch, "--error-format=json", "--json=diagnostic-short", ...flags, join(root, "🦀️.rs")], { cwd: root, stderr: "pipe", stdout: "pipe" });
  const rows: Diagnostic[] = [];
  for (const line of new TextDecoder().decode(run.stderr).split("\n")) {
    if (!line.startsWith("{")) continue;
    const j = JSON.parse(line) as { level: string; code?: { code: string } | null; message: string; spans?: { is_primary: boolean; file_name: string; label?: string | null }[] };
    if (j.message.startsWith("aborting due to")) continue;
    const primary = j.spans?.find((x) => x.is_primary);
    const file = primary ? posix(relative(root, resolve(root, primary.file_name))) : "";
    const norm = (text: string): string => text.split(root).join("<root>").replace(/(?:\/\.)+(?=\/)/gu, "").replace(/\s+/gu, " ");
    rows.push({ level: j.level, code: j.code?.code ?? null, message: norm(j.message), file, label: norm(primary?.label ?? "") });
  }
  writeFileSync(outFile, JSON.stringify(rows));
  const byCode: Record<string, number> = {};
  for (const r of rows) byCode[`${r.level}:${r.code ?? "-"}`] = (byCode[`${r.level}:${r.code ?? "-"}`] ?? 0) + 1;
  console.log(JSON.stringify({ flags, exit: run.exitCode, diagnostics: rows.length, byCode }));
}

function cmdOracleDiff(a: string, b: string): void {
  const hop = (text: string): string => text.replace(/\b(?:[a-z0-9]+_)?component::/gu, "");
  const key = (d: Diagnostic): string => `${d.level}|${d.code}|${hop(d.message)}|${d.file}|${hop(d.label)}`;
  const count = (rows: Diagnostic[]): Map<string, number> => { const m = new Map<string, number>(); for (const r of rows) m.set(key(r), (m.get(key(r)) ?? 0) + 1); return m; };
  const ma = count(JSON.parse(readFileSync(a, "utf8")) as Diagnostic[]);
  const mb = count(JSON.parse(readFileSync(b, "utf8")) as Diagnostic[]);
  const lines: string[] = [];
  for (const k of new Set([...ma.keys(), ...mb.keys()])) if ((ma.get(k) ?? 0) !== (mb.get(k) ?? 0)) lines.push(`${(mb.get(k) ?? 0) - (ma.get(k) ?? 0) > 0 ? "+" : "-"}${Math.abs((mb.get(k) ?? 0) - (ma.get(k) ?? 0))} ${k}`);
  console.log(JSON.stringify({ origDistinct: ma.size, newDistinct: mb.size, differing: lines.length }));
  if (lines.length) { console.log(lines.slice(0, 60).join("\n")); process.exit(1); }
}

const [command, ...args] = process.argv.slice(2);
if (command === "extract") console.log(JSON.stringify(extractTree(args[0]!, resolve(args[1]!)), null, 0));
else if (command === "plan") cmdPlan(args[0]!, args[1]!);
else if (command === "apply") cmdApply(args[0]!, args[1]);
else if (command === "compare") cmdCompare(args[0]!, args[1]!, args[2]!);
else if (command === "oracle") cmdOracle(args[0]!, args[1]!, args.slice(2));
else if (command === "oracle-diff") cmdOracleDiff(args[0]!, args[1]!);
else if (command !== undefined) { console.error(`unknown command ${command}`); process.exit(2); }
