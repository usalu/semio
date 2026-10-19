// 🧭 Finds and rewrites every quoted path reference to a moved file. Used by the rename and restructure tools.
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {basename, dirname, join, relative, resolve} from "node:path";
import {GEN, ageMinutes, recordOwn, REPO} from "./lib.ts";

const abs = (p: string) => join(REPO, p);
export const GENERIC = /^🟦️\.|^🟨️\.|^🦀️\.|^📜️script\.ts$/u;
const hitIndex = new Map<string, Set<string>>();

export const termsOf = (from: string): string[] => { const b = basename(from), parent = basename(dirname(from)); return [from, `${parent}/${b}`, ...(GENERIC.test(b) ? [] : [b])]; };

function indexFresh(terms: string[]): void {
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

export function buildIndex(terms: string[], cacheName: string, reuse: boolean): void {
  const cache = `${GEN}/grep-index-${cacheName}.json`;
  if (reuse && existsSync(cache)) { for (const [t, files] of Object.entries(JSON.parse(readFileSync(cache, "utf8")) as Record<string, string[]>)) hitIndex.set(t, new Set(files)); return; }
  indexFresh([...new Set(terms)]);
  writeFileSync(cache, JSON.stringify(Object.fromEntries([...hitIndex].map(([t, f]) => [t, [...f]]))));
}

export const referencersOf = (from: string): string[] => [...new Set(termsOf(from).flatMap((t) => [...(hitIndex.get(t) ?? [])]))].filter((f) => f !== from && existsSync(abs(f)));

export function rewriteStrings(text: string, referencerDir: string, from: string, to: string, newReferencerDir = referencerDir, importOnly = false): {text: string; count: number} {
  let count = 0;
  const fromAbs = abs(from), toAbs = abs(to);
  const out = text.replace(/(["'`])((?:(?!\1)[^\n\\])*)\1/gu, (m, q: string, v: string, offset: number) => {
    if (v.includes("${")) return m;
    if (importOnly && !/(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*|\brequire\s*\(\s*)$/u.test(text.slice(Math.max(0, offset - 24), offset))) return m;
    if (v.startsWith(".")) {
      if (resolve(referencerDir, v) === fromAbs) { count++; const r = relative(newReferencerDir, toAbs); return q + (r.startsWith(".") ? r : "./" + r) + q; }
      return m;
    }
    if (v === from || v.endsWith("/" + from)) { count++; return q + v.slice(0, v.length - from.length) + to + q; }
    return m;
  });
  return {text: out, count};
}

export type RepointPlan = {edits: {file: string; text: string; n: number}[]; recent: string[]};
export function planRepoint(from: string, to: string, skip: Set<string> = new Set(), importOnly = false): RepointPlan {
  const edits: RepointPlan["edits"] = [], recent: string[] = [];
  for (const f of referencersOf(from)) {
    if (skip.has(f)) continue;
    if (ageMinutes(f) < 15) { recent.push(f); continue; }
    const original = readFileSync(abs(f), "utf8");
    const r = rewriteStrings(original, dirname(abs(f)), from, to, dirname(abs(f)), importOnly);
    if (r.count) edits.push({file: f, text: r.text, n: r.count});
  }
  return {edits, recent};
}

export function applyRepoint(plan: RepointPlan): void {
  for (const e of plan.edits) { writeFileSync(abs(e.file), e.text); recordOwn(e.file); }
}

export function rebaseOwn(text: string, oldDir: string, newDir: string): string {
  return text.replace(/(["'`])(\.{1,2}\/(?:(?!\1)[^\n\\$])*)\1/gu, (m, q: string, v: string) => {
    const target = resolve(oldDir, v);
    if (!existsSync(target) && !existsSync(target.replace(/\.[^./]+$/u, ""))) return m;
    const r = relative(newDir, target);
    return q + (r.startsWith(".") ? r : "./" + r) + q;
  });
}
