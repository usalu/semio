// ✂️ Line-range splitter for one Rust module file. usage: bun split.ts <spec.json> [--apply]
// A spec names the file, the descendant test files that see its private items, and the chunks (child modules) as line ranges of the
// ORIGINAL file. A segment may wrap its lines in the header and closing line of an inherent impl, so one impl block can be spread over
// several children. Private items, methods and fields another file still names are widened to pub(super) by widenChunk.
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {identifiers, widenChunk} from "./widen.ts";

type Seg = {lines: [number, number]; wrap?: {header: number; close: number}; dedent?: number};
type Chunk = {name: string; folder: string; doc: string; segments: Seg[]; before?: string[]; after?: string[]; uses?: string[]; separator?: string; reexport?: boolean | "private"; glob?: boolean; drop?: [number, number][]; mountAttrs?: string[]; replace?: [string, string][]};
type Spec = {file: string; original: string; descendants: string[]; chunks: Chunk[]; log?: string; compareReplace?: [string, string][]};

const specPath = process.argv[2]!;
const apply = process.argv.includes("--apply");
const spec = JSON.parse(readFileSync(specPath, "utf8")) as Spec;
const original = readFileSync(spec.original, "utf8");
const lines = original.split("\n");
const dir = dirname(spec.file);

const owner = new Map<number, number>();
spec.chunks.forEach((chunk, index) => {
  const claim = (n: number) => { if (owner.has(n)) throw new Error(`line ${n} claimed twice (chunk ${chunk.name})`); owner.set(n, index); };
  for (const [a, b] of chunk.drop ?? []) for (let n = a; n <= b; n += 1) claim(n);
  for (const segment of chunk.segments) {
    for (let n = segment.lines[0]; n <= segment.lines[1]; n += 1) claim(n);
    if (segment.wrap) { if (!owner.has(segment.wrap.header)) owner.set(segment.wrap.header, index); if (!owner.has(segment.wrap.close)) owner.set(segment.wrap.close, index); }
  }
});

const mounted = new Set<number>();
const rootLines: string[] = [];
lines.forEach((text, i) => {
  const n = i + 1;
  const index = owner.get(n);
  if (index === undefined) { rootLines.push(text); return; }
  if (!mounted.has(index)) {
    mounted.add(index);
    const c = spec.chunks[index]!;
    rootLines.push(...(c.mountAttrs ?? []), `#[path = "${c.folder}/🦀️.rs"]`, `mod ${c.name};`);
    if (c.reexport !== false) rootLines.push(`${c.reexport === "private" ? "" : "pub "}use self::${c.name}::*;`);
  }
});
let root = rootLines.join("\n").replace(/\n{4,}/gu, "\n\n\n");

const bodies = spec.chunks.map(chunk => {
  const parts: string[] = [];
  for (const segment of chunk.segments) {
    const body = lines.slice(segment.lines[0] - 1, segment.lines[1]).map(l => segment.dedent && l.startsWith(" ".repeat(segment.dedent)) ? l.slice(segment.dedent) : l).join("\n");
    parts.push(segment.wrap ? `${lines[segment.wrap.header - 1]}\n${body}\n${lines[segment.wrap.close - 1]}` : body);
  }
  let joined = parts.join(chunk.separator ?? "\n\n");
  for (const [from, to] of chunk.replace ?? []) { if (!joined.includes(from)) throw new Error(`replace anchor missing in ${chunk.name}: ${from}`); joined = joined.replaceAll(from, to); }
  return joined;
});
const wrapped = spec.chunks.map((chunk, i) => [...(chunk.before ?? []), bodies[i]!, ...(chunk.after ?? [])].join("\n"));
const descendantIds = new Set<string>();
for (const file of spec.descendants) for (const id of identifiers(readFileSync(file, "utf8"))) descendantIds.add(id);
const report: string[] = [];
const children = spec.chunks.map((chunk, i) => {
  const outside = new Set<string>([...identifiers(root), ...descendantIds]);
  spec.chunks.forEach((other, j) => { if (j !== i) for (const id of identifiers(wrapped[j]!)) outside.add(id); });
  const widened = widenChunk(wrapped[i]!, outside, report, chunk.name);
  const imports = `${chunk.glob === false ? "" : "use super::*;\n"}${(chunk.uses ?? []).map(u => u + "\n").join("")}`;
  const header = `//! ${chunk.doc}\n\n${imports === "" ? "" : imports + "\n"}`;
  return {chunk, text: header + widened.trimEnd() + "\n"};
});
root = root.trimEnd() + "\n";
const count = (text: string) => text.split("\n").length;
const out: string[] = [];
out.push(`root ${count(root)} lines; children: ${children.map(c => `${c.chunk.name}=${count(c.text)}`).join(" ")}`);
out.push(`widened ${report.length}:`);
for (const r of report) out.push("  " + r);
const norm = (text: string) => text.split("\n").map(l => { let t = l.trim().replaceAll("pub(super) ", ""); for (const [from, to] of spec.compareReplace ?? []) t = t.replaceAll(from, to); return t; }).filter(Boolean);
const before = new Map<string, number>();
for (const l of norm(original)) before.set(l, (before.get(l) ?? 0) + 1);
const after = new Map<string, number>();
for (const l of [...norm(root), ...children.flatMap(c => norm(c.text))]) after.set(l, (after.get(l) ?? 0) + 1);
let lost = 0;
for (const [l, n] of before) if ((after.get(l) ?? 0) < n) { lost += 1; out.push(`  LOST ${n - (after.get(l) ?? 0)} ${l.slice(0, 120)}`); }
out.push(`lines lost relative to the original: ${lost}`);
let added = 0;
for (const [l, n] of after) if ((before.get(l) ?? 0) < n) added += n - (before.get(l) ?? 0);
out.push(`line occurrences added (mounts, headers, wrappers, widened lines): ${added}`);
console.log(out.join("\n"));
if (apply) {
  for (const c of children) {
    const target = resolve(dir, c.chunk.folder, "🦀️.rs");
    if (existsSync(target)) throw new Error("exists " + target);
    mkdirSync(dirname(target), {recursive: true});
    writeFileSync(target, c.text);
  }
  writeFileSync(spec.file, root);
  console.log("applied");
}
