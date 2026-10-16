// 🧵️ Small Rust text helpers shared by the RENDERER-TEST-DEPS tools: comment/string masking, use-tree expansion, cfg(test) region finding.
// @see ../RESULT-RENDERER-TEST-DEPS.md

export const mask = (text: string): string => {
  const out = text.split("");
  const n = text.length;
  let i = 0;
  const blank = (a: number, b: number) => { for (let k = a; k < b && k < n; k++) if (out[k] !== "\n") out[k] = " "; };
  while (i < n) {
    const c = text[i]!;
    if (c === "/" && text[i + 1] === "/") { let j = text.indexOf("\n", i); if (j < 0) j = n; blank(i, j); i = j; continue; }
    if (c === "/" && text[i + 1] === "*") {
      let d = 1, j = i + 2;
      while (j < n && d) { if (text.startsWith("/*", j)) { d++; j += 2; } else if (text.startsWith("*/", j)) { d--; j += 2; } else j++; }
      blank(i, j); i = j; continue;
    }
    if ((c === "r" || c === "b") && (i === 0 || !/[A-Za-z0-9_]/.test(text[i - 1]!))) {
      const m = /^b?r(#*)"/.exec(text.slice(i, i + 40));
      if (m) {
        const close = '"' + m[1];
        let j = text.indexOf(close, i + m[0].length);
        j = j < 0 ? n : j + close.length;
        blank(i, j); i = j; continue;
      }
    }
    if (c === '"') {
      let j = i + 1;
      while (j < n && text[j] !== '"') j += text[j] === "\\" ? 2 : 1;
      j = Math.min(n, j + 1);
      blank(i, j); i = j; continue;
    }
    if (c === "'") {
      const m = /^'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'/.exec(text.slice(i, i + 16));
      if (m) { blank(i, i + m[0].length); i += m[0].length; continue; }
    }
    i++;
  }
  return out.join("");
};

export type UseLeaf = {segs: string[]; kind: "item" | "glob" | "self"; alias?: string};

export const expandUse = (tree: string, prefix: string[] = []): UseLeaf[] => {
  tree = tree.trim();
  const parts: string[] = [];
  let depth = 0, cur = "";
  for (const ch of tree) {
    if (ch === "{") depth++;
    if (ch === "}") depth--;
    if (ch === "," && depth === 0) { parts.push(cur); cur = ""; } else cur += ch;
  }
  parts.push(cur);
  if (parts.length > 1) return parts.filter(p => p.trim()).flatMap(p => expandUse(p, prefix));
  const t = parts[0]!.trim();
  if (!t) return [];
  const brace = /^(.*?)::\s*\{([\s\S]*)\}$/.exec(t);
  if (brace) return expandUse(brace[2]!, [...prefix, ...brace[1]!.split("::").map(s => s.trim()).filter(Boolean)]);
  if (t.startsWith("{") && t.endsWith("}")) return expandUse(t.slice(1, -1), prefix);
  let alias: string | undefined;
  let body = t;
  const am = /^([\s\S]*?)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)$/.exec(t);
  if (am) { body = am[1]!; alias = am[2]; }
  const segs = [...prefix, ...body.split("::").map(s => s.trim()).filter(Boolean)];
  if (segs[segs.length - 1] === "*") return [{segs: segs.slice(0, -1), kind: "glob"}];
  if (segs[segs.length - 1] === "self") return [{segs: segs.slice(0, -1), kind: "self", alias}];
  return [{segs, kind: "item", alias}];
};

export const lineOf = (text: string, pos: number): number => {
  let n = 1;
  for (let i = 0; i < pos; i++) if (text.charCodeAt(i) === 10) n++;
  return n;
};

export type Region = {start: number; end: number; attr: string};

const TEST_ATTR = /#\[\s*cfg\s*\(([^\]]*?)\)\s*\]/g;
const isTestCfg = (inner: string): boolean => /\btest\b/.test(inner) && !/\bnot\s*\(\s*(?:any\s*\(\s*)?test\b/.test(inner);

export const testRegions = (masked: string): Region[] => {
  const out: Region[] = [];
  let m: RegExpExecArray | null;
  TEST_ATTR.lastIndex = 0;
  while ((m = TEST_ATTR.exec(masked))) {
    if (!isTestCfg(m[1]!)) continue;
    let i = m.index + m[0].length;
    for (;;) {
      const rest = masked.slice(i);
      const a = /^\s*#\[[^\]]*\]/.exec(rest);
      if (!a) break;
      i += a[0].length;
    }
    let depth = 0, j = i, sawBrace = false;
    for (; j < masked.length; j++) {
      const ch = masked[j]!;
      if (ch === "{") { depth++; sawBrace = true; }
      else if (ch === "}") { depth--; if (depth <= 0 && sawBrace) { j++; break; } if (depth < 0) break; }
      else if (ch === ";" && depth === 0) { j++; break; }
    }
    out.push({start: m.index, end: j, attr: m[1]!.trim()});
    TEST_ATTR.lastIndex = Math.max(TEST_ATTR.lastIndex, m.index + m[0].length);
  }
  return out;
};

export const inRegions = (regions: Region[], pos: number): Region | undefined => regions.find(r => pos >= r.start && pos < r.end);

export type AliasRef = {pos: number; line: number; form: "use" | "path" | "alias" | "other"; segs: string[]; tree?: UseLeaf[]; stmtStart?: number; stmtEnd?: number; vis?: string; text: string};

export const findAliasRefs = (masked: string, alias: string): AliasRef[] => {
  const out: AliasRef[] = [];
  const rx = new RegExp(`(?<![A-Za-z0-9_:.$])${alias}\\b`, "g");
  let m: RegExpExecArray | null;
  const useStmt = new RegExp(`((?:pub(?:\\([^)]*\\))?\\s+)?)use\\s+(?:::)?${alias}\\b`, "g");
  const useStarts = new Map<number, RegExpExecArray>();
  while ((m = useStmt.exec(masked))) useStarts.set(m.index + m[0].length - alias.length, m);
  while ((m = rx.exec(masked))) {
    const pos = m.index;
    const line = lineOf(masked, pos);
    const us = useStarts.get(pos);
    if (us) {
      let depth = 0, j = pos;
      for (; j < masked.length; j++) { const ch = masked[j]!; if (ch === "{") depth++; else if (ch === "}") depth--; else if (ch === ";" && depth === 0) break; }
      const body = masked.slice(us.index + us[0].length - alias.length, j);
      const tree = expandUse(body);
      out.push({pos, line, form: "use", segs: [], tree, stmtStart: us.index, stmtEnd: j + 1, vis: us[1]!.trim(), text: body.replace(/\s+/g, " ").slice(0, 200)});
      continue;
    }
    const tail = masked.slice(pos + alias.length);
    const pm = /^((?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)+)/.exec(tail);
    const segs = pm ? pm[1]!.split("::").map(s => s.trim()).filter(Boolean) : [];
    const am = /^\s+as\s+([A-Za-z_]\w*)/.exec(tail);
    const form: AliasRef["form"] = segs.length ? "path" : am ? "alias" : "other";
    out.push({pos, line, form, segs, text: masked.slice(Math.max(0, pos - 20), pos + 90).replace(/\s+/g, " ")});
  }
  return out;
};

export const cfgCompiledWithoutTest = (attr: string): boolean => {
  const s = attr.replace(/\s+/g, "");
  let i = 0;
  type Tri = "T" | "F" | "U";
  const parse = (): Tri => {
    const m = /^([A-Za-z_][A-Za-z0-9_]*)/.exec(s.slice(i));
    if (!m) { i++; return "U"; }
    const name = m[1]!;
    i += name.length;
    if (s[i] === "(") {
      i++;
      const args: Tri[] = [];
      while (i < s.length && s[i] !== ")") { args.push(parse()); if (s[i] === ",") i++; }
      i++;
      if (name === "all") return args.includes("F") ? "F" : args.every(a => a === "T") ? "T" : "U";
      if (name === "any") return args.includes("T") ? "T" : args.every(a => a === "F") ? "F" : "U";
      if (name === "not") return args[0] === "T" ? "F" : args[0] === "F" ? "T" : "U";
      return "U";
    }
    if (s[i] === "=") { i++; while (i < s.length && s[i] !== "," && s[i] !== ")") i++; return "U"; }
    return name === "test" ? "F" : "U";
  };
  return parse() !== "F";
};
