/**
 * 🩹 Restores the intended specifier per edit when several edits of one file shared the same replacement string.
 *
 *   bun repair-reverts.ts <plan.json> <revert.json> <revert-closure.json> [apply]
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const [planPath, r1, r2, mode] = process.argv.slice(2) as string[];
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(planPath!, "utf8"));
const rev = new Set<string>();
for (const e of JSON.parse(readFileSync(r1!, "utf8")).reverted) rev.add(e.file + "\0" + e.before + "\0" + e.after);
for (const e of JSON.parse(readFileSync(r2!, "utf8"))) rev.add(e.file + "\0" + e.before + "\0" + e.after);
const groups = new Map<string, any[]>();
for (const e of plan.edits as any[]) { const k = e.file + "\0" + e.after; (groups.get(k) ?? groups.set(k, []).get(k)!).push(e); }
let repaired = 0, problems = 0, files = new Set<string>();
const quoted = (s: string): string[] => [`"${s}"`, `'${s}'`];
for (const list of groups.values()) {
  if (list.length < 2) continue;
  const file = list[0].file, abs = resolve(ROOT, file);
  list.sort((a, b) => a.start - b.start);
  const intended = list.map(e => (rev.has(e.file + "\0" + e.before + "\0" + e.after) ? e.before : e.after));
  if (new Set(intended).size === 1) continue;
  const tokens = new Set<string>([list[0].after, ...list.map(e => e.before)]);
  let text = readFileSync(abs, "utf8");
  const positions: { start: number; len: number }[] = [];
  const re = /(["'])((?:(?!\1)[^\n])*)\1/gu;
  for (const m of text.matchAll(re)) if (tokens.has(m[2]!) && !(positions.length >= list.length)) positions.push({ start: m.index!, len: m[0].length });
  if (positions.length !== list.length) { problems++; console.log("PROBLEM", file.slice(-70), positions.length, list.length); continue; }
  const cur = positions.map(p => text.slice(p.start + 1, p.start + p.len - 1));
  if (cur.every((c, i) => c === intended[i])) continue;
  repaired++; files.add(file);
  if (mode === "apply") {
    for (let i = positions.length - 1; i >= 0; i--) { const p = positions[i]!; text = text.slice(0, p.start + 1) + intended[i] + text.slice(p.start + p.len - 1); }
    writeFileSync(abs, text);
  }
}
console.log(JSON.stringify({ repaired, problems, files: files.size }));
