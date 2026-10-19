// 🚚 Renames implementation leaves to their canonical names and re-points every path reference to them.
// usage: bun rename-leaf.ts <pairs.tsv> [--write]   (rows: from<TAB>to, repo-relative paths)
import {existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, basename, join, relative, resolve} from "node:path";
import {GEN, ledger, REPO, ageMinutes, recordOwn} from "./lib.ts";

const write = process.argv.includes("--write");
const pairs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t") as [string, string]);
const abs = (p: string) => join(REPO, p);
const GENERIC = /^🟦️\.|^🟨️\.|^🦀️\.|^📜️script\.ts$/u;

const hitIndex = new Map<string, Set<string>>();
function indexTerms(terms: string[]): void {
  const cache = `${GEN}/grep-index-${process.argv[2]!.split("/").pop()}.json`;
  if (process.argv.includes("--reuse") && existsSync(cache)) { for (const [t, files] of Object.entries(JSON.parse(readFileSync(cache, "utf8")) as Record<string, string[]>)) hitIndex.set(t, new Set(files)); return; }
  indexTermsFresh(terms);
  writeFileSync(cache, JSON.stringify(Object.fromEntries([...hitIndex].map(([t, f]) => [t, [...f]]))));
}
function indexTermsFresh(terms: string[]): void {
  const args = ["grep", "--untracked", "-I", "-F", "-n"];
  for (const t of terms) args.push("-e", t);
  args.push("--", ".", ":!.🧬semio", ":!node_modules", ":!*/node_modules/*");
  let out = "";
  try { out = execFileSync("git", args, {cwd: REPO, maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"]}).toString(); } catch (e: any) { out = e.stdout?.toString() ?? ""; }
  for (const line of out.split("\n")) {
    const m = /^([^:]+):\d+:(.*)$/u.exec(line);
    if (!m) continue;
    for (const t of terms) if (m[2]!.includes(t)) (hitIndex.get(t) ?? hitIndex.set(t, new Set()).get(t)!).add(m[1]!);
  }
}
const grepFiles = (terms: string[]): string[] => [...new Set(terms.flatMap((t) => [...(hitIndex.get(t) ?? [])]))];

function rewriteStrings(text: string, referencerDir: string, from: string, to: string, newReferencerDir = referencerDir): {text: string; count: number} {
  let count = 0;
  const fromAbs = abs(from), toAbs = abs(to), fromRel = from;
  const out = text.replace(/(["'`])((?:(?!\1)[^\n\\])*)\1/gu, (m, q: string, v: string) => {
    if (v.includes("${")) return m;
    if (v.startsWith(".")) {
      if (resolve(referencerDir, v) === fromAbs) { count++; const r = relative(newReferencerDir, toAbs); return q + (r.startsWith(".") ? r : "./" + r) + q; }
      return m;
    }
    if (v === fromRel || v.endsWith("/" + fromRel)) { count++; return q + v.slice(0, v.length - fromRel.length) + to + q; }
    return m;
  });
  return {text: out, count};
}

function rebaseOwn(text: string, oldDir: string, newDir: string): string {
  return text.replace(/(["'`])(\.{1,2}\/(?:(?!\1)[^\n\\$])*)\1/gu, (m, q: string, v: string) => {
    const target = resolve(oldDir, v);
    if (!existsSync(target) && !existsSync(target.replace(/\.[^./]+$/u, "")) ) return m;
    const r = relative(newDir, target);
    return q + (r.startsWith(".") ? r : "./" + r) + q;
  });
}

indexTerms([...new Set(pairs.flatMap(([from]) => { const b = basename(from), parent = basename(dirname(from)); return [from, `${parent}/${b}`, ...(GENERIC.test(b) ? [] : [b])]; }))]);
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
for (const [from, to] of pairs) {
  if (!existsSync(abs(from))) { count("missing"); continue; }
  if (existsSync(abs(to))) { count("occupied"); ledger({tool: "rename-leaf", from, to, result: "occupied"}); continue; }
  if (ageMinutes(from) < 15) { count("skip-recent"); ledger({tool: "rename-leaf", from, to, result: "skip-recent"}); continue; }
  if (to.length >= 256) { count("too-long"); continue; }
  const base = basename(from), parent = basename(dirname(from));
  const terms = [from, `${parent}/${base}`, ...(GENERIC.test(base) ? [] : [base])];
  const referencers = grepFiles(terms).filter((f) => f !== from);
  const edits: {file: string; text: string; n: number}[] = [];
  const recent: string[] = [];
  for (const f of referencers) {
    if (!existsSync(abs(f))) continue;
    if (ageMinutes(f) < 15) { recent.push(f); continue; }
    const original = readFileSync(abs(f), "utf8");
    const r = rewriteStrings(original, dirname(abs(f)), from, to);
    if (r.count) edits.push({file: f, text: r.text, n: r.count});
  }
  if (recent.length) { count("skip-referencer-recent"); ledger({tool: "rename-leaf", from, to, result: "skip-referencer-recent", recent}); continue; }
  if (!write) { count("ok"); console.log(`ok ${from} -> ${to} (${edits.length} referencers)`); continue; }
  const oldDir = dirname(abs(from)), newDir = dirname(abs(to));
  let moved = readFileSync(abs(from), "utf8");
  if (oldDir !== newDir) moved = rebaseOwn(moved, oldDir, newDir);
  const self = rewriteStrings(moved, newDir, from, to);
  mkdirSync(newDir, {recursive: true});
  writeFileSync(abs(to), self.text);
  for (const e of edits) { writeFileSync(abs(e.file), e.text); recordOwn(e.file); }
  writeFileSync(abs(from), "");
  const fs = require("node:fs") as typeof import("node:fs");
  fs.unlinkSync(abs(from));
  recordOwn(to);
  count("renamed");
  ledger({tool: "rename-leaf", from, to, result: "renamed", referencers: edits.map((e) => e.file)});
  console.log(`renamed ${from} -> ${to} (${edits.length} referencers)`);
}
console.log(tally);
