import { existsSync } from "node:fs";
import { dirname } from "node:path";
import { read, stripComments, importsOf } from "./closure.ts";
import { ROOT } from "./targets.ts";

export type Fn = { name: string; file: string; start: number; end: number; text: string; kind: "function" | "arrow" | "class" | "method" };
export type Import = { file: string | null; imported: string };
export type Mod = { file: string; src: string; fns: Map<string, Fn[]>; named: Map<string, Import>; ns: Map<string, string>; reexports: { names: string[] | "*"; file: string; as?: Map<string, string> }[] };

const cache = new Map<string, Mod>();
const transpiler = new Bun.Transpiler({ loader: "tsx" });

export function block(s: string, open: number, o = "{", c = "}"): number {
  let d = 0, q = "";
  for (let i = open; i < s.length; i++) {
    const ch = s[i]!;
    if (q) { if (ch === "\\") i++; else if (ch === q) q = ""; continue; }
    if (ch === '"' || ch === "'" || ch === "`") { q = ch; continue; }
    if (ch === o) d++;
    else if (ch === c) { d--; if (d === 0) return i + 1; }
  }
  return s.length;
}
function skipTypeToBrace(s: string, from: number): number {
  let i = from, d = 0, q = "";
  for (; i < s.length; i++) {
    const ch = s[i]!;
    if (q) { if (ch === "\\") i++; else if (ch === q) q = ""; continue; }
    if (ch === '"' || ch === "'" || ch === "`") { q = ch; continue; }
    if (ch === "<" ) d++;
    else if (ch === ">" && s[i - 1] !== "=") d--;
    else if (ch === "{" && d <= 0) {
      const before = s.slice(Math.max(0, i - 3), i);
      if (/[:|&]\s*$/.test(before) && d <= 0 && /:\s*$/.test(s.slice(Math.max(0, from), i).trim().slice(-2))) { i = block(s, i) - 1; continue; }
      return i;
    }
  }
  return -1;
}

export function resolveFile(from: string, spec: string): string | null {
  if (spec.startsWith("node:") || spec.startsWith("bun:")) return null;
  try { const r = Bun.resolveSync(spec, dirname(from)); return r.startsWith(ROOT + "/") && !r.includes("/node_modules/") ? r : null; } catch { return null; }
}

export function parse(file: string): Mod {
  const hit = cache.get(file);
  if (hit) return hit;
  const src = stripComments(read(file));
  const m: Mod = { file, src, fns: new Map(), named: new Map(), ns: new Map(), reexports: [] };
  cache.set(file, m);
  const addFn = (f: Fn) => (m.fns.get(f.name) ?? m.fns.set(f.name, []).get(f.name)!).push(f);
  for (const mm of src.matchAll(/(?:^|[\n;}])\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s*\*?\s*([A-Za-z_$][\w$]*)\s*(?:<[^(]*>)?\s*\(/g)) {
    const open = mm.index! + mm[0].length - 1;
    const close = block(src, open, "(", ")");
    const b = skipTypeToBrace(src, close);
    if (b < 0) continue;
    const end = block(src, b);
    addFn({ name: mm[1]!, file, start: mm.index!, end, text: src.slice(mm.index!, end), kind: "function" });
  }
  for (const mm of src.matchAll(/(?:^|[\n;}])\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=\n]+)?=\s*(?:async\s*)?(?:<[^>(]*>\s*)?(\(|[A-Za-z_$][\w$]*\s*=>)/g)) {
    let i = mm.index! + mm[0].length - 1;
    let arrow: number;
    if (mm[2] === "(") { const close = block(src, i, "(", ")"); arrow = src.indexOf("=>", close); if (arrow < 0 || arrow - close > 200) continue; arrow += 2; }
    else arrow = mm.index! + mm[0].length;
    let j = arrow; while (/\s/.test(src[j] ?? "")) j++;
    let end: number;
    if (src[j] === "{") end = block(src, j);
    else { let d = 0, q = ""; end = j; for (; end < src.length; end++) { const ch = src[end]!; if (q) { if (ch === "\\") end++; else if (ch === q) q = ""; continue; } if (ch === '"' || ch === "'" || ch === "`") { q = ch; continue; } if ("([{".includes(ch)) d++; else if (")]}".includes(ch)) { if (d === 0) break; d--; } else if ((ch === ";" ) && d === 0) break; } }
    addFn({ name: mm[1]!, file, start: mm.index!, end, text: src.slice(mm.index!, end), kind: "arrow" });
  }
  for (const mm of src.matchAll(/(?:^|[\n;}])\s*(?:export\s+)?(?:default\s+)?(?:abstract\s+)?class\s+([A-Za-z_$][\w$]*)[^{]*\{/g)) {
    const open = mm.index! + mm[0].length - 1;
    const end = block(src, open);
    addFn({ name: mm[1]!, file, start: mm.index!, end, text: src.slice(mm.index!, end), kind: "class" });
  }
  for (const mm of src.matchAll(/import\s+(type\s+)?([\s\S]*?)\s+from\s*["']([^"']+)["']/g)) {
    if (mm[1]) continue;
    const target = resolveFile(file, mm[3]!);
    const clause = mm[2]!;
    const nm = /\{([\s\S]*)\}/.exec(clause);
    if (nm) for (const part of nm[1]!.split(",")) { const p = part.trim().replace(/^type\s+/, ""); if (!p) continue; const [a, b] = p.split(/\s+as\s+/); m.named.set((b ?? a)!.trim(), { file: target, imported: a!.trim() }); }
    const star = /\*\s+as\s+([\w$]+)/.exec(clause); if (star && target) m.ns.set(star[1]!, target);
    const def = /^([A-Za-z_$][\w$]*)\s*(?:,|$)/.exec(clause.trim()); if (def && !clause.trim().startsWith("{") && !clause.trim().startsWith("*")) m.named.set(def[1]!, { file: target, imported: "default" });
  }
  for (const mm of src.matchAll(/(?:const|let|var)\s+\{([^}]*)\}\s*=\s*await\s+import\(\s*["']([^"']+)["']\s*\)/g)) {
    const target = resolveFile(file, mm[2]!);
    for (const part of mm[1]!.split(",")) { const p = part.trim(); if (!p) continue; const [a, b] = p.split(":"); m.named.set((b ?? a)!.trim(), { file: target, imported: a!.trim() }); }
  }
  for (const mm of src.matchAll(/(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*await\s+import\(\s*["']([^"']+)["']\s*\)/g)) { const target = resolveFile(file, mm[2]!); if (target) m.ns.set(mm[1]!, target); }
  for (const mm of src.matchAll(/export\s+(type\s+)?\{([^}]*)\}\s*from\s*["']([^"']+)["']/g)) {
    if (mm[1]) continue; const target = resolveFile(file, mm[3]!); if (!target) continue;
    const as = new Map<string, string>(); const names: string[] = [];
    for (const part of mm[2]!.split(",")) { const p = part.trim(); if (!p) continue; const [a, b] = p.split(/\s+as\s+/); names.push((b ?? a)!.trim()); if (b) as.set(b.trim(), a!.trim()); }
    m.reexports.push({ names, file: target, as });
  }
  for (const mm of src.matchAll(/export\s+\*\s+(?:as\s+[\w$]+\s+)?from\s*["']([^"']+)["']/g)) { const target = resolveFile(file, mm[1]!); if (target) m.reexports.push({ names: "*", file: target }); }
  return m;
}

export function findFn(file: string, name: string, seen = new Set<string>()): Fn[] {
  const key = file + "\0" + name;
  if (seen.has(key)) return [];
  seen.add(key);
  const m = parse(file);
  const own = m.fns.get(name);
  if (own) return own;
  const imp = m.named.get(name);
  if (imp?.file) { const r = findFn(imp.file, imp.imported === "default" ? name : imp.imported, seen); if (r.length) return r; }
  for (const r of m.reexports) {
    if (r.names === "*") { const x = findFn(r.file, name, seen); if (x.length) return x; }
    else if (r.names.includes(name)) { const x = findFn(r.file, r.as?.get(name) ?? name, seen); if (x.length) return x; }
  }
  return [];
}
