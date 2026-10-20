import { join, relative, resolve, dirname } from "node:path";
import { existsSync, statSync } from "node:fs";
import { ROOT, type T } from "./targets.ts";
import { parse, findFn, block, resolveFile, type Fn } from "./module.ts";
import { analyzeAnchors, type Finding } from "./flow2.ts";

export const rel = (a: string) => relative(ROOT, a);
export const domainOf = (root: string) => { const s = root.split("/"); if (s[0] === "🧰️framework") return s[1] === "📦️packages" ? s.slice(0, 4).join("/") : s.slice(0, 3).join("/"); return s[0]!; };

const KEYWORDS = new Set(["if", "for", "while", "switch", "catch", "function", "return", "await", "new", "typeof", "super", "import", "async", "assert", "describe", "test", "it", "expect", "Promise", "Array", "Object", "Number", "String", "Boolean", "Math", "JSON", "Set", "Map", "Error", "BigInt", "Symbol", "Date", "RegExp", "Uint8Array", "console", "require", "process", "resolve", "join", "relative", "dirname", "basename", "isAbsolute", "readFileSync", "existsSync", "readdirSync", "statSync", "lstatSync", "realpathSync"]);
export type Spawn = { name: string; args: string; fn: string };
export type Write = { fn: string; line: number; prim: string; arg: string; strong: boolean; anchored: boolean };
export type Evidence = { texts: { file: string; text: string }[]; files: string[]; entry: string; entryKind: string; owned: string[]; infra: string[]; spawns: Spawn[]; writes: Write[]; unresolved: string[] };

const SPAWN = /^(runCmd|runOwnedCommand|runRepositoryCommand|runBudgetedTestCommand|spawnSync|spawn|exec|execSync|execFile|execFileSync|runCommand|runBun|runProcess|runChecked|runStep|sh|shell|\w*Spawn\w*|run[A-Z]\w*Command)$/;
const anchorCache = new Map<string, Finding[]>();
const findings = (file: string) => anchorCache.get(file) ?? (anchorCache.set(file, analyzeAnchors(file)), anchorCache.get(file)!);

function callsOf(file: string, text: string): { name: string; ns?: string; args: string }[] {
  const m = parse(file);
  const out: { name: string; ns?: string; args: string }[] = [];
  for (const mm of text.matchAll(/(?<![.\w$])([A-Za-z_$][\w$]*)\s*(?:<[^()]*>)?\(/g)) {
    const name = mm[1]!; if (KEYWORDS.has(name) || name === "run" || name === "constructor") continue;
    if (/(?:\basync|\bfunction|\bget|\bset|\bstatic|\bpublic|\bprivate|\breadonly|\bnew)\s+$/.test(text.slice(Math.max(0, mm.index! - 12), mm.index!))) { if (!/\bnew\s+$/.test(text.slice(Math.max(0, mm.index! - 6), mm.index!))) continue; }
    const open = mm.index! + mm[0].length - 1;
    out.push({ name, args: text.slice(open + 1, Math.min(block(text, open, "(", ")") - 1, open + 400)) });
  }
  for (const mm of text.matchAll(/\b([A-Za-z_$][\w$]*)\.([A-Za-z_$][\w$]*)\s*\(/g)) if (m.ns.has(mm[1]!)) out.push({ name: mm[2]!, ns: mm[1]!, args: "" });
  for (const mm of text.matchAll(/\(await import\(\s*["']([^"']+)["']\s*\)\)\.([A-Za-z_$][\w$]*)\s*\(/g)) out.push({ name: mm[2]!, ns: "@" + mm[1]!, args: "" });
  for (const mm of text.matchAll(/new\s+([A-Za-z_$][\w$]*)\s*\(/g)) out.push({ name: mm[1]!, args: "" });
  return out;
}

export function stripForeign(text: string, words: string[]): string {
  const re = /if\s*\(\s*(?:[\w.]+\.length\s*[=!]==?\s*\d+\s*&&\s*)?(command|(?:segments|args|rest)\[\d\]|(?:process\.)?argv\[\d\])\s*===?\s*["']([^"']+)["']\s*\)\s*\{/g;
  let out = text, m: RegExpExecArray | null;
  const spans: [number, number][] = [];
  while ((m = re.exec(text))) {
    if (words.includes(m[2]!)) continue;
    const open = m.index + m[0].length - 1;
    const end = block(text, open);
    spans.push([m.index, end]);
    re.lastIndex = end;
  }
  for (const [a, b] of spans.reverse()) out = out.slice(0, a) + " ".repeat(b - a) + out.slice(b);
  return out;
}
const fn_ = (fn: Fn, words: string[]): Fn => {
  const base = fn.name.includes("[") || fn.kind === "class" || fn.name === "<file>" ? fn : narrow(fn, words);
  if (fn.name === "<file>") return fn;
  const text = stripForeign(base.text, words);
  return text === base.text ? base : { ...base, text };
};
export function narrow(fn: Fn, words: string[]): Fn {
  const w = words[1];
  if (!w) return fn;
  const re = new RegExp(`(?:segments|args|rest)\\[0\\]\\s*===\\s*[\"']${w.replace(/[-]/g, "\\-")}[\"']`);
  const m = re.exec(fn.text);
  if (!m) return fn;
  let i = fn.text.indexOf("{", m.index);
  const nl = fn.text.indexOf("\n", m.index);
  if (i < 0 || (nl > 0 && i > nl + 200)) return fn;
  const end = block(fn.text, i);
  const text = fn.text.slice(m.index, end);
  return { ...fn, name: fn.name + "[" + w + "]", text, start: fn.start + m.index, end: fn.start + end };
}

const fileLit = (file: string, text: string): string[] => {
  const out = new Set<string>();
  for (const m of text.matchAll(/[\"'`]([^\"'`\n${}]{3,300}\.ts)[\"'`]/g)) {
    for (const base of [dirname(file), ROOT]) {
      const p = resolve(base, m[1]!);
      if (p.startsWith(ROOT + "/") && !p.includes("/node_modules/") && !p.includes("package-commands") && p !== file && existsSync(p) && statSync(p).isFile()) { out.add(p); break; }
    }
  }
  return [...out];
};

export function walk(entries: { file: string; fn: Fn }[], domain: string, words: string[] = [], cap = 400): Evidence {
  const ev: Evidence = { texts: [], files: [], entry: "", entryKind: "", owned: [], infra: [], spawns: [], writes: [], unresolved: [] };
  const seen = new Set<string>();
  const infra = new Set<string>(), unresolved = new Set<string>();
  const stack = [...entries];
  while (stack.length && seen.size < cap) {
    const popped = stack.pop()!;
    const file = popped.file, fn = fn_(popped.fn, words);
    const id = `${rel(file)}#${fn.name}@${fn.start}`;
    if (seen.has(id)) continue;
    seen.add(id);
    ev.owned.push(`${rel(file)}#${fn.name}`);
    if (/runArtifact(Rust|TypeScript)PackageMain/.test(fn.text)) ev.texts.push({ file: rel(file), text: fn.text });
    for (const f of findings(file)) if (f.index >= fn.start && f.index < fn.end && !f.tmp) ev.writes.push({ fn: `${rel(file)}#${fn.name}`, line: f.line, prim: f.prim, arg: f.arg, strong: f.strong, anchored: f.anchored });
    const mod = parse(file);
    const calls = callsOf(file, fn.text);
    const runsTests = calls.some((c) => SPAWN.test(c.name) && /["']test["']/.test(c.args));
    for (const lit of runsTests ? fileLit(file, fn.text) : []) {
      const lm = parse(lit);
      stack.push({ file: lit, fn: { name: "<file>", file: lit, start: 0, end: lm.src.length, text: lm.src, kind: "function" } });
      ev.files.push(rel(lit));
    }
    for (const c of calls) {
      if (SPAWN.test(c.name)) ev.spawns.push({ name: c.name, args: c.args.replace(/\s+/g, " ").slice(0, 260), fn: `${rel(file)}#${fn.name}` });
      let targets: Fn[] = [];
      let where: string | null = null;
      if (c.ns) {
        const nsFile = c.ns.startsWith("@") ? mod.ns.get(c.ns) ?? null : mod.ns.get(c.ns) ?? null;
        if (nsFile) { targets = findFn(nsFile, c.name); where = nsFile; }
      } else {
        targets = findFn(file, c.name);
        where = mod.named.get(c.name)?.file ?? (mod.fns.has(c.name) ? file : null);
      }
      if (!targets.length) { if (!SPAWN.test(c.name)) unresolved.add(c.name); continue; }
      for (const t of targets) {
        if (rel(t.file).startsWith(domain + "/") || rel(t.file) === domain) stack.push({ file: t.file, fn: t });
        else infra.add(`${c.name}@${rel(t.file)}`);
      }
    }
  }
  ev.infra = [...infra].sort();
  ev.unresolved = [...unresolved].sort();
  return ev;
}

export function entryFor(t: T): { entries: { file: string; fn: Fn }[]; kind: string; label: string } {
  const script = join(ROOT, t.script!);
  const mod = parse(script);
  const word = t.words[0] ?? "";
  const esc = word.replace(/[-]/g, "\\-");
  const reg = new RegExp(`\\.register(Lazy)?\\(\\s*["']${esc}["']\\s*,\\s*`);
  const m = reg.exec(mod.src);
  if (m) {
    const rest = mod.src.slice(m.index + m[0].length);
    if (m[1]) {
      const li = /^async\s*\(\)\s*=>\s*\(\s*await\s+import\(\s*["']([^"']+)["']\s*\)\s*\)\.([A-Za-z_$][\w$]*)/.exec(rest);
      if (li) { const f = resolveFile(script, li[1]!); const fns = f ? findFn(f, li[2]!) : []; if (fns.length) return { entries: fns.map((fn) => ({ file: fn.file, fn })), kind: "registered", label: `${rel(script)}#lazy:${li[2]}` }; }
    } else if (/^class\b/.test(rest)) {
      const open = rest.indexOf("{"); const end = block(rest, open);
      const at = m.index + m[0].length;
      const fn: Fn = { name: `class@${word}`, file: script, start: at, end: at + end, text: rest.slice(0, end), kind: "class" };
      return { entries: [{ file: script, fn }], kind: "registered", label: `${rel(script)}#inline:${word}` };
    } else {
      const id = /^([A-Za-z_$][\w$]*)/.exec(rest);
      if (id) { const fns = findFn(script, id[1]!); if (fns.length) return { entries: fns.map((fn) => ({ file: fn.file, fn })), kind: "registered", label: `${rel(script)}#${id[1]}` }; }
    }
  }
  const classes = [...mod.fns.values()].flat().filter((f) => f.kind === "class");
  let text = mod.src;
  for (const c of [...classes].sort((a, b) => b.start - a.start)) text = text.slice(0, c.start) + " ".repeat(c.end - c.start) + text.slice(c.end);
  const fn: Fn = { name: "<script>", file: script, start: 0, end: text.length, text, kind: "function" };
  const hasStd = /runArtifact(Rust|TypeScript)PackageMain/.test(mod.src);
  return { entries: [{ file: script, fn }, ...(hasStd ? [] : classes.map((c) => ({ file: script, fn: c })))], kind: "program", label: rel(script) };
}
