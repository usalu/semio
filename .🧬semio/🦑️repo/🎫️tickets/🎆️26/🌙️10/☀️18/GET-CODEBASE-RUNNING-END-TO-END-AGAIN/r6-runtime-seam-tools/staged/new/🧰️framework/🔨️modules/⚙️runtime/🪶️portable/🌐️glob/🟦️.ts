import { readdirSync, statSync } from "node:fs";
import { readdir, stat } from "node:fs/promises";
import { join, resolve } from "node:path";
import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeGlobScanOptionsV1, RuntimeGlobV1 } from "../../📋️contract/🟦️.ts";

const MAXIMUM_ALTERNATIVES = 4096;
const NEVER = "\u0003";
const BOUND = "\u0004";
const SPECIALS = /[\\^$.*+?()[\]{}|/]/gu;

type SettledV1 = Readonly<{ cwd: string; dot: boolean; onlyFiles: boolean; followSymlinks: boolean; absolute: boolean }>;
type EntryV1 = Readonly<{ name: string; directory: boolean; file: boolean; link: boolean }>;

function escapeLiteral(character: string): string { return character.replace(SPECIALS, "\\$&"); }

function findClass(text: string, start: number): number {
  let index = start + 1;
  if (text[index] === "!" || text[index] === "^") index++;
  if (text[index] === "]") index++;
  for (; index < text.length; index++) {
    if (text[index] === "\\") { index++; continue; }
    if (text[index] === "]") return index;
  }
  return -1;
}

function braceGroup(text: string, open: number): { readonly end: number; readonly parts: string[] } | undefined {
  const parts: string[] = [];
  let depth = 0, start = open + 1;
  for (let index = open + 1; index < text.length; index++) {
    const character = text[index]!;
    if (character === "\\") { index++; continue; }
    if (character === "[") { const close = findClass(text, index); if (close >= 0) index = close; continue; }
    if (character === "{") { depth++; continue; }
    if (character === "}") { if (depth === 0) { parts.push(text.slice(start, index)); return { end: index, parts }; } depth--; continue; }
    if (character === "," && depth === 0) { parts.push(text.slice(start, index)); start = index + 1; }
  }
  return undefined;
}

function expandBraces(pattern: string): string[] {
  for (let index = 0; index < pattern.length; index++) {
    const character = pattern[index]!;
    if (character === "\\") { index++; continue; }
    if (character === "[") { const close = findClass(pattern, index); if (close >= 0) index = close; continue; }
    if (character !== "{") continue;
    const group = braceGroup(pattern, index);
    if (group === undefined) return [NEVER];
    const head = pattern.slice(0, index).replace(/\*+$/u, stars => BOUND.repeat(stars.length)), tail = pattern.slice(group.end + 1).replace(/^\*+(?!\/)/u, stars => BOUND.repeat(stars.length)), expanded = group.parts.flatMap(part => expandBraces(head + part + tail));
    if (expanded.length > MAXIMUM_ALTERNATIVES) throw Error(runtimeMessageV1("globTooMany", {}));
    return expanded;
  }
  return [pattern];
}

function classExpression(body: string): string {
  let negate = false, index = 0, output = "";
  if (body[0] === "!" || body[0] === "^") { negate = true; index = 1; }
  for (; index < body.length; index++) {
    const character = body[index]!;
    if (character === "\\") { index++; output += escapeLiteral(body[index] ?? "\\"); continue; }
    if (character === "-" && output.length > 0 && index < body.length - 1) { output += "-"; continue; }
    output += character === "]" || character === "^" || character === "[" ? `\\${character}` : escapeLiteral(character);
  }
  return `[${negate ? "^" : ""}${output}]`;
}

function segmentExpression(segment: string): string | undefined {
  let output = "";
  for (let index = 0; index < segment.length; index++) {
    const character = segment[index]!;
    if (character === "\\") { index++; output += escapeLiteral(segment[index] ?? "\\"); continue; }
    if (character === "[") {
      const close = findClass(segment, index);
      if (close < 0) return undefined;
      output += classExpression(segment.slice(index + 1, close)); index = close; continue;
    }
    if (character === "*") { while (segment[index + 1] === "*") index++; output += "[^/]*"; continue; }
    if (character === BOUND) { output += "[^/]*"; continue; }
    if (character === "?") { output += "[^/]"; continue; }
    output += escapeLiteral(character);
  }
  return output;
}

function splitSegments(pattern: string): string[] {
  const segments: string[] = [];
  let current = "";
  for (let index = 0; index < pattern.length; index++) {
    const character = pattern[index]!;
    if (character === "\\") { current += pattern.slice(index, index + 2); index++; continue; }
    if (character === "[") { const close = findClass(pattern, index); if (close >= 0) { current += pattern.slice(index, close + 1); index = close; continue; } }
    if (character === "/") { segments.push(current); current = ""; continue; }
    current += character;
  }
  segments.push(current);
  return segments;
}

/** 🧵️ One brace-free alternative: its segments for scanning and its whole-path expression for matching. */
interface AlternativeV1 { readonly segments: readonly string[]; readonly expression: RegExp | undefined }

function compileAlternative(pattern: string): AlternativeV1 {
  if (pattern === NEVER) return { segments: [], expression: undefined };
  const segments = splitSegments(pattern);
  let output = "";
  for (let index = 0; index < segments.length; index++) {
    const segment = segments[index]!, last = index === segments.length - 1;
    if (isGlobstar(segment)) { output += last ? ".*" : "(?:[^/]*/)*"; continue; }
    const text = segmentExpression(segment);
    if (text === undefined) return { segments, expression: undefined };
    output += text + (last ? "" : "/");
  }
  return { segments, expression: new RegExp(`^${output}$`, "su") };
}

function isLiteral(segment: string): boolean { return !/[*?[\\\u0004]/u.test(segment); }
function isGlobstar(segment: string): boolean { return segment === "**"; }
function child(relative: string, name: string): string { return relative === "" ? name : relative.endsWith("/") ? relative + name : `${relative}/${name}`; }
function byName(left: { name: string }, right: { name: string }): number { return left.name < right.name ? -1 : left.name > right.name ? 1 : 0; }

function listSync(directory: string): EntryV1[] {
  try {
    return readdirSync(directory, { withFileTypes: true }).sort(byName).map(entry => {
      const link = entry.isSymbolicLink();
      if (!link) return { name: entry.name, link, directory: entry.isDirectory(), file: entry.isFile() };
      try { const info = statSync(join(directory, entry.name)); return { name: entry.name, link, directory: info.isDirectory(), file: info.isFile() }; } catch { return { name: entry.name, link, directory: false, file: false }; }
    });
  } catch { return []; }
}

async function listAsync(directory: string): Promise<EntryV1[]> {
  try {
    const rows = (await readdir(directory, { withFileTypes: true })).sort(byName), entries: EntryV1[] = [];
    for (const entry of rows) {
      const link = entry.isSymbolicLink();
      if (!link) { entries.push({ name: entry.name, link, directory: entry.isDirectory(), file: entry.isFile() }); continue; }
      try { const info = await stat(join(directory, entry.name)); entries.push({ name: entry.name, link, directory: info.isDirectory(), file: info.isFile() }); } catch { entries.push({ name: entry.name, link, directory: false, file: false }); }
    }
    return entries;
  } catch { return []; }
}

function deeper(entry: EntryV1, settled: SettledV1): boolean { return entry.directory && (!entry.link || settled.followSymlinks); }
function hidden(entry: EntryV1, settled: SettledV1): boolean { return !settled.dot && entry.name.startsWith("."); }
function wanted(entry: EntryV1, settled: SettledV1): boolean { return !settled.onlyFiles || entry.file; }

function* walkSync(segments: readonly string[], settled: SettledV1, relative: string, depth: number): Generator<string> {
  const segment = segments[depth];
  if (segment === undefined) return;
  const last = depth === segments.length - 1, base = join(settled.cwd, relative || ".");
  if (isGlobstar(segment)) {
    if (!last) yield* walkSync(segments, settled, relative, depth + 1);
    for (const entry of listSync(base)) {
      if (hidden(entry, settled)) continue;
      const path = child(relative, entry.name);
      if (last && wanted(entry, settled)) yield path;
      if (deeper(entry, settled)) yield* last ? everything(path, settled) : walkSync(segments, settled, path, depth);
    }
    return;
  }
  if (isLiteral(segment)) {
    const name = segment;
    if (name.startsWith(".") && name !== "." && name !== ".." && !settled.dot) return;
    let info: ReturnType<typeof statSync>;
    try { info = statSync(join(base, name)); } catch { return; }
    const path = child(relative, name);
    if (last) { if (!settled.onlyFiles || info.isFile()) yield path; return; }
    if (info.isDirectory()) yield* walkSync(segments, settled, path, depth + 1);
    return;
  }
  const text = segmentExpression(segment);
  if (text === undefined) return;
  const matcher = new RegExp(`^${text}$`, "su");
  for (const entry of listSync(base)) {
    if (hidden(entry, settled) || !matcher.test(entry.name)) continue;
    const path = child(relative, entry.name);
    if (last) { if (wanted(entry, settled)) yield path; continue; }
    if (deeper(entry, settled)) yield* walkSync(segments, settled, path, depth + 1);
  }
}

function* everything(relative: string, settled: SettledV1): Generator<string> {
  for (const entry of listSync(join(settled.cwd, relative))) {
    if (hidden(entry, settled)) continue;
    const path = child(relative, entry.name);
    if (wanted(entry, settled)) yield path;
    if (deeper(entry, settled)) yield* everything(path, settled);
  }
}

async function* walkAsync(segments: readonly string[], settled: SettledV1, relative: string, depth: number): AsyncGenerator<string> {
  const segment = segments[depth];
  if (segment === undefined) return;
  const last = depth === segments.length - 1, base = join(settled.cwd, relative || ".");
  if (isGlobstar(segment)) {
    if (!last) yield* walkAsync(segments, settled, relative, depth + 1);
    for (const entry of await listAsync(base)) {
      if (hidden(entry, settled)) continue;
      const path = child(relative, entry.name);
      if (last && wanted(entry, settled)) yield path;
      if (deeper(entry, settled)) yield* last ? everythingAsync(path, settled) : walkAsync(segments, settled, path, depth);
    }
    return;
  }
  if (isLiteral(segment)) {
    const name = segment;
    if (name.startsWith(".") && name !== "." && name !== ".." && !settled.dot) return;
    let info: Awaited<ReturnType<typeof stat>>;
    try { info = await stat(join(base, name)); } catch { return; }
    const path = child(relative, name);
    if (last) { if (!settled.onlyFiles || info.isFile()) yield path; return; }
    if (info.isDirectory()) yield* walkAsync(segments, settled, path, depth + 1);
    return;
  }
  const text = segmentExpression(segment);
  if (text === undefined) return;
  const matcher = new RegExp(`^${text}$`, "su");
  for (const entry of await listAsync(base)) {
    if (hidden(entry, settled) || !matcher.test(entry.name)) continue;
    const path = child(relative, entry.name);
    if (last) { if (wanted(entry, settled)) yield path; continue; }
    if (deeper(entry, settled)) yield* walkAsync(segments, settled, path, depth + 1);
  }
}

async function* everythingAsync(relative: string, settled: SettledV1): AsyncGenerator<string> {
  for (const entry of await listAsync(join(settled.cwd, relative))) {
    if (hidden(entry, settled)) continue;
    const path = child(relative, entry.name);
    if (wanted(entry, settled)) yield path;
    if (deeper(entry, settled)) yield* everythingAsync(path, settled);
  }
}

function settle(options: RuntimeGlobScanOptionsV1 | undefined): SettledV1 {
  return { cwd: resolve(options?.cwd ?? process.cwd()), dot: options?.dot ?? false, onlyFiles: options?.onlyFiles ?? true, followSymlinks: options?.followSymlinks ?? false, absolute: options?.absolute ?? false };
}

/** 🌐️ Compiles one glob pattern (`*`, `**`, `?`, classes, braces, `!` negation) into matching and scanning that are our own code over `node:fs`, with the Bun glob as test oracle. */
export function compileGlobV1(pattern: string): RuntimeGlobV1 {
  const negated = pattern.startsWith("!");
  const alternatives = expandBraces(negated ? pattern.slice(1) : pattern).map(compileAlternative);
  const match = (path: string): boolean => alternatives.some(alternative => alternative.expression?.test(path) === true) !== negated;
  const reject = (): never => { throw Error(runtimeMessageV1("globNegated", {})); };
  if (negated) return { match, scan: reject, scanSync: reject };
  const publish = (settled: SettledV1, path: string): string => (settled.absolute ? join(settled.cwd, path) : path);
  return {
    match,
    *scanSync(options) {
      const settled = settle(options), seen = new Set<string>();
      for (const alternative of alternatives) {
        if (alternative.expression === undefined) continue;
        if (alternative.segments[0] === "") throw Error(runtimeMessageV1("globAbsolute", {}));
        for (const path of walkSync(alternative.segments, settled, "", 0)) if (!seen.has(path)) { seen.add(path); yield publish(settled, path); }
      }
    },
    async *scan(options) {
      const settled = settle(options), seen = new Set<string>();
      for (const alternative of alternatives) {
        if (alternative.expression === undefined) continue;
        if (alternative.segments[0] === "") throw Error(runtimeMessageV1("globAbsolute", {}));
        for await (const path of walkAsync(alternative.segments, settled, "", 0)) if (!seen.has(path)) { seen.add(path); yield publish(settled, path); }
      }
    },
  };
}
