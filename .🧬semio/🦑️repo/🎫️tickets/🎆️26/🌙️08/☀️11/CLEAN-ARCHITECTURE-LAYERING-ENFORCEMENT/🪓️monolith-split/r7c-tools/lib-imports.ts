// 🧰️ Shared helpers of the r7c import rewriters: use-tree expansion into bindings, usage detection by tokens and grouped use rendering.
import { readFileSync } from "node:fs";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
export const { parseRustSourceTree, useLeafNames } = await import(`${L}/🌳️module-tree/🟦️.ts`);
export const { rustTokens } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");

export type Binding = { name: string; path: string; alias: boolean; cfgTest: boolean };

export const stripComments = (text: string): string => text.replace(/\/\*[\s\S]*?\*\//gu, " ").replace(/\/\/[^\n]*/gu, " ");

export function expandUse(statement: string): { path: string; alias: string | null }[] {
  let body = stripComments(statement).trim().replace(/^pub(?:\([^)]*\))?\s+/u, "").replace(/^use\s+/u, "").replace(/;\s*$/u, "").trim();
  const out: { path: string; alias: string | null }[] = [];
  let i = 0;
  const ws = () => { while (i < body.length && /\s/u.test(body[i]!)) i += 1; };
  const tree = (prefix: string[]): void => {
    ws();
    const segs = [...prefix];
    for (;;) {
      ws();
      if (body[i] === "{") {
        i += 1;
        for (;;) {
          ws();
          if (body[i] === "}") { i += 1; return; }
          if (i >= body.length) return;
          tree(segs);
          ws();
          if (body[i] === ",") i += 1;
        }
      }
      const m = /^(?:::)?([A-Za-z_][A-Za-z0-9_]*|\*|self)/u.exec(body.slice(i));
      if (!m) { i += 1; return; }
      i += m[0].length;
      segs.push(m[1]!);
      ws();
      if (body.slice(i, i + 2) === "::") { i += 2; continue; }
      let alias: string | null = null;
      const a = /^as\s+([A-Za-z_][A-Za-z0-9_]*|_)/u.exec(body.slice(i));
      if (a) { alias = a[1]!; i += a[0].length; }
      const path = segs[segs.length - 1] === "self" ? segs.slice(0, -1) : segs;
      out.push({ path: path.join("::"), alias });
      return;
    }
  };
  tree([]);
  return out;
}

export function bindingsOf(statement: string, cfgTest: boolean): Binding[] {
  const res: Binding[] = [];
  for (const { path, alias } of expandUse(statement)) {
    if (path.endsWith("::*") || path === "*") continue;
    const last = path.split("::").pop()!;
    res.push({ name: alias ?? last, path, alias: alias !== null && alias !== last, cfgTest });
  }
  return res;
}

export function usedNames(text: string, names: Iterable<string>): Set<string> {
  const wanted = new Set(names);
  const tokens = rustTokens(text);
  const used = new Set<string>();
  const local = new Set<string>();
  const decl = new Set([".", "fn", "let", "mut", "struct", "enum", "const", "static", "type", "mod", "trait", "use", "ref", "for"]);
  tokens.forEach((t: any, k: number) => {
    if (t.kind !== "identifier" || !wanted.has(t.text) || !/^[a-z_]/u.test(t.text)) return;
    const prev = tokens[k - 1]?.text, next = tokens[k + 1]?.text;
    if ((prev === "let" || prev === "mut" || prev === "ref" || prev === "for") || (prev === "|" && (next === "|" || next === ",")) || next === ":") local.add(t.text);
  });
  for (let k = 0; k < tokens.length; k += 1) {
    const t = tokens[k]!;
    if (t.kind !== "identifier" || !wanted.has(t.text) || used.has(t.text)) continue;
    const prev = tokens[k - 1]?.text, next = tokens[k + 1]?.text;
    const upper = /^[A-Z]/u.test(t.text);
    if (prev !== undefined && (upper ? ["fn", "struct", "enum", "trait", "type", "mod", "const", "static", "use", "."] : [...decl]).includes(prev)) continue;
    if (prev === "::" && !["crate", "self", "super"].includes(tokens[k - 2]?.text ?? "")) continue;
    if (next === ":" && !upper) continue;
    if (upper) { used.add(t.text); continue; }
    if (next === "::" || next === "!") { used.add(t.text); continue; }
    if (next === "(") { used.add(t.text); continue; }
    if (local.has(t.text)) continue;
    if ((prev === "(" || prev === "," || prev === "=" || prev === "&" || prev === "[" || prev === "|") && (next === ")" || next === "," || next === ";" || next === "]" || next === "}")) used.add(t.text);
  }
  return used;
}

export function renderUses(bindings: Binding[], indent = ""): string[] {
  const lines: string[] = [];
  const grouped = new Map<string, string[]>();
  const test = new Map<string, boolean>();
  for (const b of bindings) {
    const idx = b.path.lastIndexOf("::");
    if (b.alias || idx < 0) {
      lines.push(`${b.cfgTest ? `${indent}#[cfg(test)]\n` : ""}${indent}use ${b.path}${b.alias ? ` as ${b.name}` : ""};`);
      continue;
    }
    const key = `${b.cfgTest ? "T:" : "N:"}${b.path.slice(0, idx)}`;
    grouped.set(key, [...(grouped.get(key) ?? []), b.path.slice(idx + 2)]);
    test.set(key, b.cfgTest);
  }
  for (const [key, leaves] of grouped) {
    const parent = key.slice(2);
    const uniq = [...new Set(leaves)].sort();
    lines.push(`${test.get(key) ? `${indent}#[cfg(test)]\n` : ""}${indent}use ${parent}::${uniq.length === 1 ? uniq[0] : `{${uniq.join(", ")}}`};`);
  }
  return lines;
}

export const readText = (p: string) => readFileSync(p, "utf8");
