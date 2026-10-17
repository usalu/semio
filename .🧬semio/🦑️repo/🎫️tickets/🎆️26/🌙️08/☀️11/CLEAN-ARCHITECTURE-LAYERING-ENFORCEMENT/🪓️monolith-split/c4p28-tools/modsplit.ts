// ✂️ c4p28 module splitter: cuts verbatim line ranges of the ORIGINAL snapshot of one Rust lib root into child modules (`#[path] mod` plus a glob re-export) and stages the result.
// usage: bun modsplit.ts <spec.json> [--stage]
// Spec: {file (repo-relative), original (snapshot path), stage (stage dir), descendants[] (repo-relative files that see private items), chunks[]}.
// Chunk: {name, folder, doc, segments[{lines:[a,b], wrap?:{header,close}}], before?, after?, drop?, reexport?: "auto"|"pub"|"private"|"none", mountAttrs?, extraUses?, replace?: [from, to][] (exact once, applied after widening)}.
// An item another file of the tree names is widened to pub(super) by widenChunk. The re-export of a child is `pub use` when it holds a pub item, `pub(crate) use` when it holds only pub(crate), else a private `use`.
import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {identifiers, widenChunk} from "./widen.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";

const REPO = "/Users/ueli/Documents/semio";
type Seg = {lines: [number, number]; wrap?: {header: number; close: number}; before?: string[]; after?: string[]};
type Chunk = {name: string; folder: string; doc: string; segments: Seg[]; before?: string[]; after?: string[]; drop?: [number, number][]; reexport?: "auto" | "pub" | "private" | "none"; mountAttrs?: string[]; extraUses?: string[]; mountAt?: number; replace?: [string, string][]};
type Spec = {file: string; original: string; stage: string; descendants: string[]; chunks: Chunk[]};

const specPath = process.argv[2]!;
const stageIt = process.argv.includes("--stage");
const spec = JSON.parse(readFileSync(specPath, "utf8")) as Spec;
const original = readFileSync(spec.original, "utf8");
const lines = original.split("\n");
const fileAbs = resolve(REPO, spec.file);
const dir = dirname(fileAbs);

const owner = new Map<number, number>();
spec.chunks.forEach((chunk, index) => {
  const claim = (n: number) => { if (owner.has(n)) throw new Error(`line ${n} claimed twice (chunk ${chunk.name})`); owner.set(n, index); };
  for (const [a, b] of chunk.drop ?? []) for (let n = a; n <= b; n += 1) claim(n);
  for (const segment of chunk.segments) {
    for (let n = segment.lines[0]; n <= segment.lines[1]; n += 1) claim(n);
    if (segment.wrap) { if (!owner.has(segment.wrap.header)) owner.set(segment.wrap.header, index); if (!owner.has(segment.wrap.close)) owner.set(segment.wrap.close, index); }
  }
});

const mountedAt = new Set<number>();
const rootLines: string[] = [];
const explicit = new Map<number, number[]>();
spec.chunks.forEach((chunk, index) => { if (chunk.mountAt !== undefined) explicit.set(chunk.mountAt, [...(explicit.get(chunk.mountAt) ?? []), index]); });
lines.forEach((text, i) => {
  const n = i + 1;
  for (const index of explicit.get(n) ?? []) { mountedAt.add(index); rootLines.push(`@@MOUNT:${index}@@`); }
  const index = owner.get(n);
  if (index === undefined) { rootLines.push(text); return; }
  if (!mountedAt.has(index) && spec.chunks[index]!.mountAt === undefined) { mountedAt.add(index); rootLines.push(`@@MOUNT:${index}@@`); }
});

const bodies = spec.chunks.map(chunk => {
  const parts: string[] = [];
  for (const segment of chunk.segments) {
    const body = lines.slice(segment.lines[0] - 1, segment.lines[1]).join("\n");
    const inner = segment.wrap ? `${lines[segment.wrap.header - 1]}\n${body}\n${lines[segment.wrap.close - 1]}` : body;
    parts.push([...(segment.before ?? []), inner, ...(segment.after ?? [])].join("\n"));
  }
  return parts.join("\n\n");
});
const wrapped = spec.chunks.map((chunk, i) => [...(chunk.before ?? []), bodies[i]!, ...(chunk.after ?? [])].join("\n"));
const rootText0 = rootLines.filter(l => !l.startsWith("@@MOUNT:")).join("\n");
const descendantIds = new Set<string>();
for (const file of spec.descendants) for (const id of identifiers(readFileSync(resolve(REPO, file), "utf8"))) descendantIds.add(id);
const report: string[] = [];
const classify = (text: string): "pub" | "pub(crate)" | "private" => {
  const tree = parseRustSourceTree(text);
  let best: "pub" | "pub(crate)" | "private" = "private";
  for (const item of tree.items) {
    if (item.kind === "impl" || item.kind === "use" || item.kind === "extern-crate") continue;
    if (item.visibility === "pub") return "pub";
    if (item.visibility === "pub(crate)") best = "pub(crate)";
  }
  for (const mod of tree.modules) if (mod.visibility === "pub") return "pub";
  return best;
};
const children = spec.chunks.map((chunk, i) => {
  const outside = new Set<string>([...identifiers(rootText0), ...descendantIds]);
  spec.chunks.forEach((other, j) => { if (j !== i) for (const id of identifiers(wrapped[j]!)) outside.add(id); });
  let widened = widenChunk(wrapped[i]!, outside, report, chunk.name);
  for (const [from, to] of chunk.replace ?? []) {
    const hits = widened.split(from).length - 1;
    if (hits !== 1) throw new Error(`${chunk.name}: replace anchor occurs ${hits} times: ${from.slice(0, 80)}`);
    widened = widened.replace(from, to);
    report.push(`${chunk.name}: replaced ${from.trim().slice(0, 70)} -> ${to.trim().slice(0, 70)}`);
  }
  const imports = `use super::*;\n${(chunk.extraUses ?? []).map(u => u + "\n").join("")}`;
  const header = `//! ${chunk.doc}\n\n${imports}\n`;
  return {chunk, text: header + widened.trimEnd() + "\n", visibility: classify(widened)};
});
const mountText = (i: number): string => {
  const c = spec.chunks[i]!;
  const child = children[i]!;
  const mode = c.reexport ?? "auto";
  const uses = mode === "none" ? "" : mode === "pub" || (mode === "auto" && child.visibility === "pub") ? `pub use self::${c.name}::*;` : mode === "auto" && child.visibility === "pub(crate)" ? `pub(crate) use self::${c.name}::*;` : `use self::${c.name}::*;`;
  return [...(c.mountAttrs ?? []), `#[path = "${c.folder}/🦀️.rs"]`, `mod ${c.name};`, ...(uses === "" ? [] : [uses])].join("\n");
};
const root = rootLines.map(l => { const m = l.match(/^@@MOUNT:(\d+)@@$/); return m ? mountText(Number(m[1])) : l; }).join("\n").replace(/\n{3,}/gu, "\n\n").trimEnd() + "\n";
const count = (text: string) => text.split("\n").length;
const out: string[] = [];
out.push(`root ${count(root)} lines; children: ${children.map(c => `${c.chunk.name}=${count(c.text)}(${c.visibility})`).join(" ")}`);
out.push(`widened ${report.length}:`);
for (const r of report) out.push("  " + r);
const norm = (text: string) => text.split("\n").map(l => l.trim().replaceAll("pub(super) ", "")).filter(Boolean);
const before = new Map<string, number>();
for (const l of norm(original)) before.set(l, (before.get(l) ?? 0) + 1);
const after = new Map<string, number>();
for (const l of [...norm(root), ...children.flatMap(c => norm(c.text))]) after.set(l, (after.get(l) ?? 0) + 1);
let lost = 0;
for (const [l, n] of before) if ((after.get(l) ?? 0) < n) { lost += 1; out.push(`  LOST ${n - (after.get(l) ?? 0)} ${l.slice(0, 120)}`); }
out.push(`lines lost relative to the original: ${lost}`);
console.log(out.join("\n"));
if (stageIt) {
  const rel = (abs: string) => abs.slice(REPO.length + 1);
  const put = (abs: string, text: string) => { const target = resolve(spec.stage, rel(abs)); mkdirSync(dirname(target), {recursive: true}); writeFileSync(target, text); };
  put(fileAbs, root);
  for (const c of children) put(resolve(dir, c.chunk.folder, "🦀️.rs"), c.text);
  console.log("staged to " + spec.stage);
}
