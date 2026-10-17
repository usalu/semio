// 🔤️ Minimal Rust lexer and cfg evaluator for the cross-crate cfg(test) scanner.
export type Tok = {k: "i" | "p" | "l" | "s"; t: string; line: number};

const isIdStart = (c: number) => (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c === 95 || (c > 127 && /[\p{L}\p{Nl}]/u.test(String.fromCodePoint(c)));
const isIdCont = (c: number) => isIdStart(c) || (c >= 48 && c <= 57) || (c > 127 && /[\p{N}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(c)));

export const lex = (src: string): Tok[] => {
  const out: Tok[] = [];
  const n = src.length;
  let i = 0, line = 1;
  const push = (k: Tok["k"], t: string) => out.push({k, t, line});
  while (i < n) {
    const c = src.charCodeAt(i);
    if (c === 10) { line++; i++; continue; }
    if (c === 32 || c === 9 || c === 13) { i++; continue; }
    if (c === 47 && src.charCodeAt(i + 1) === 47) { while (i < n && src.charCodeAt(i) !== 10) i++; continue; }
    if (c === 47 && src.charCodeAt(i + 1) === 42) {
      let depth = 1; i += 2;
      while (i < n && depth > 0) {
        const d = src.charCodeAt(i);
        if (d === 10) line++;
        if (d === 47 && src.charCodeAt(i + 1) === 42) { depth++; i += 2; }
        else if (d === 42 && src.charCodeAt(i + 1) === 47) { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    if (isIdStart(c)) {
      let j = i;
      while (j < n) {
        const d = src.charCodeAt(j);
        if (isIdCont(d)) j += d > 0xffff ? 2 : 1; else break;
      }
      const word = src.slice(i, j);
      const q = src.charCodeAt(j);
      if ((word === "r" || word === "br" || word === "cr") && (q === 34 || q === 35)) {
        let h = j, hashes = 0;
        while (src.charCodeAt(h) === 35) { hashes++; h++; }
        if (src.charCodeAt(h) === 34) {
          const close = "\"" + "#".repeat(hashes);
          const end = src.indexOf(close, h + 1);
          const stop = end < 0 ? n : end + close.length;
          for (let k = i; k < stop; k++) if (src.charCodeAt(k) === 10) line++;
          push("s", src.slice(h + 1, end < 0 ? n : end));
          i = stop;
          continue;
        }
        if (word === "r" && q === 35 && isIdStart(src.charCodeAt(h))) {
          let e = h;
          while (e < n && isIdCont(src.charCodeAt(e))) e++;
          push("i", src.slice(h, e));
          i = e;
          continue;
        }
      }
      if ((word === "b" || word === "c") && q === 34) { i = j; continue; }
      if (word === "b" && q === 39) { i = j; continue; }
      push("i", word);
      i = j;
      continue;
    }
    if (c === 34) {
      let j = i + 1, text = "";
      while (j < n) {
        const d = src.charCodeAt(j);
        if (d === 92) { text += src[j + 1] ?? ""; if (src.charCodeAt(j + 1) === 10) line++; j += 2; continue; }
        if (d === 10) line++;
        if (d === 34) break;
        text += src[j]; j++;
      }
      push("s", text);
      i = j + 1;
      continue;
    }
    if (c === 39) {
      const next = src.charCodeAt(i + 1);
      if (next === 92) {
        let j = i + 2;
        while (j < n && src.charCodeAt(j) !== 39) j++;
        push("l", "'c'");
        i = j + 1;
        continue;
      }
      const cp = src.codePointAt(i + 1) ?? 0;
      const width = cp > 0xffff ? 2 : 1;
      if (src.charCodeAt(i + 1 + width) === 39) { push("l", "'c'"); i += 2 + width; continue; }
      let j = i + 1;
      while (j < n && isIdCont(src.charCodeAt(j))) j++;
      push("l", src.slice(i, j));
      i = Math.max(j, i + 1);
      continue;
    }
    if (c >= 48 && c <= 57) {
      let j = i + 1;
      while (j < n) {
        const d = src.charCodeAt(j);
        if (isIdCont(d)) j++;
        else if (d === 46 && src.charCodeAt(j + 1) >= 48 && src.charCodeAt(j + 1) <= 57) j++;
        else break;
      }
      push("l", src.slice(i, j));
      i = j;
      continue;
    }
    if (c === 58 && src.charCodeAt(i + 1) === 58) { push("p", "::"); i += 2; continue; }
    if (c === 45 && src.charCodeAt(i + 1) === 62) { push("p", "->"); i += 2; continue; }
    if (c === 61 && src.charCodeAt(i + 1) === 62) { push("p", "=>"); i += 2; continue; }
    push("p", src[i]!);
    i++;
  }
  return out;
};

export const isOpen = (t: Tok) => t.k === "p" && (t.t === "(" || t.t === "[" || t.t === "{");
export const isClose = (t: Tok) => t.k === "p" && (t.t === ")" || t.t === "]" || t.t === "}");

export const matchClose = (toks: Tok[], i: number): number => {
  let depth = 0;
  for (let j = i; j < toks.length; j++) {
    const t = toks[j]!;
    if (isOpen(t)) depth++;
    else if (isClose(t)) { depth--; if (depth === 0) return j; }
  }
  return toks.length - 1;
};

export type Cfg = {raw: string; names: string[]; features: string[]; gated: boolean; partial: boolean};

const FALSE = 1, TRUE = 2, BOTH = 3;
type Expr = {op: "pred"; name: string; value?: string} | {op: "all" | "any" | "not"; args: Expr[]};

const parseExpr = (toks: Tok[], pos: {i: number}): Expr => {
  const head = toks[pos.i++]!;
  const name = head.t;
  if ((name === "all" || name === "any" || name === "not") && toks[pos.i]?.t === "(") {
    pos.i++;
    const args: Expr[] = [];
    while (pos.i < toks.length && toks[pos.i]!.t !== ")") {
      args.push(parseExpr(toks, pos));
      if (toks[pos.i]?.t === ",") pos.i++;
    }
    pos.i++;
    return {op: name, args};
  }
  if (toks[pos.i]?.t === "=") { const value = toks[pos.i + 1]?.t; pos.i += 2; return {op: "pred", name, value}; }
  return {op: "pred", name};
};

const evalExpr = (e: Expr, testOn: boolean, featuresOff = false): number => {
  if (e.op === "pred") {
    if (e.name === "test" && e.value === undefined) return testOn ? TRUE : FALSE;
    if (e.name === "feature" && e.value !== undefined && featuresOff) return FALSE;
    return BOTH;
  }
  const vals = e.args.map(a => evalExpr(a, testOn, featuresOff));
  if (e.op === "not") { const v = vals[0] ?? BOTH; return ((v & FALSE) ? TRUE : 0) | ((v & TRUE) ? FALSE : 0); }
  if (e.op === "all") {
    const canTrue = vals.every(v => v & TRUE), canFalse = vals.some(v => v & FALSE);
    return (canTrue ? TRUE : 0) | (canFalse ? FALSE : 0);
  }
  const canTrue = vals.some(v => v & TRUE), canFalse = vals.every(v => v & FALSE);
  return (canTrue ? TRUE : 0) | (canFalse ? FALSE : 0);
};

const collect = (e: Expr, names: string[], features: string[]) => {
  if (e.op === "pred") { if (e.name === "feature" && e.value !== undefined) features.push(e.value); else if (e.value === undefined) names.push(e.name); return; }
  for (const a of e.args) collect(a, names, features);
};

export const parseCfg = (toks: Tok[]): Cfg => {
  const pos = {i: 0};
  const expr = parseExpr(toks, pos);
  const off = evalExpr(expr, false), on = evalExpr(expr, true);
  const names: string[] = [], features: string[] = [];
  collect(expr, names, features);
  const offNoFeatures = evalExpr(expr, false, true);
  return {raw: toks.map(t => t.t).join(" "), names, features, gated: off === FALSE && (on & TRUE) !== 0, partial: off === BOTH && offNoFeatures === FALSE && on === TRUE};
};
