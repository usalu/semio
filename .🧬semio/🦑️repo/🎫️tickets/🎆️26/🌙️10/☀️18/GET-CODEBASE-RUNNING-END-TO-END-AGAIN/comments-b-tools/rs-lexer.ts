import { keepText, type Removal } from "./keep.ts";

export type Tok = { t: "c" | "x"; s: string; pos: number; end: number };

const isIdStart = (ch: string) => /[\p{XID_Start}_]/u.test(ch);
const isIdCont = (ch: string) => /[\p{XID_Continue}]/u.test(ch);

/** 🔤 Minimal Rust lexer: comments (nested blocks), strings, raw strings, chars vs lifetimes, idents, numbers, punctuation. */
export function lexRs(text: string): Tok[] | undefined {
  const out: Tok[] = [];
  const n = text.length;
  let i = 0;
  const push = (t: "c" | "x", pos: number, end: number) => out.push({ t, s: text.slice(pos, end), pos, end });
  while (i < n) {
    const ch = text[i];
    if (ch === " " || ch === "\t" || ch === "\n" || ch === "\r") {
      i++;
      continue;
    }
    if (ch === "/" && text[i + 1] === "/") {
      let e = text.indexOf("\n", i);
      if (e < 0) e = n;
      let end = e;
      if (text[end - 1] === "\r") end--;
      push("c", i, end);
      i = e;
      continue;
    }
    if (ch === "/" && text[i + 1] === "*") {
      let depth = 1;
      let j = i + 2;
      while (j < n && depth > 0) {
        if (text[j] === "/" && text[j + 1] === "*") {
          depth++;
          j += 2;
        } else if (text[j] === "*" && text[j + 1] === "/") {
          depth--;
          j += 2;
        } else j++;
      }
      if (depth > 0) return undefined;
      push("c", i, j);
      i = j;
      continue;
    }
    const rawM = /^(?:b|c)?r(#*)"/.exec(text.slice(i, i + 12));
    if (rawM && (i === 0 || !isIdCont(text[i - 1]))) {
      const hashes = rawM[1];
      const close = `"${hashes}`;
      const start = i + rawM[0].length;
      const e = text.indexOf(close, start);
      if (e < 0) return undefined;
      push("x", i, e + close.length);
      i = e + close.length;
      continue;
    }
    if (ch === '"' || ((ch === "b" || ch === "c") && text[i + 1] === '"' && (i === 0 || !isIdCont(text[i - 1])))) {
      let j = i + (ch === '"' ? 1 : 2);
      while (j < n && text[j] !== '"') j += text[j] === "\\" ? 2 : 1;
      if (j >= n) return undefined;
      push("x", i, j + 1);
      i = j + 1;
      continue;
    }
    if (ch === "'" || (ch === "b" && text[i + 1] === "'" && (i === 0 || !isIdCont(text[i - 1])))) {
      const q = ch === "'" ? i : i + 1;
      const nx = text[q + 1];
      if (nx === "\\") {
        let j = q + 2;
        while (j < n && text[j] !== "'") j++;
        push("x", i, j + 1);
        i = j + 1;
        continue;
      }
      const cp = text.codePointAt(q + 1);
      const len = cp !== undefined && cp > 0xffff ? 2 : 1;
      if (text[q + 1 + len] === "'") {
        push("x", i, q + 2 + len);
        i = q + 2 + len;
        continue;
      }
      let j = q + 1;
      while (j < n && isIdCont(text[j])) j++;
      push("x", i, Math.max(j, q + 1));
      i = Math.max(j, q + 1);
      continue;
    }
    if (isIdStart(ch)) {
      let j = i + 1;
      while (j < n && isIdCont(text[j])) j++;
      push("x", i, j);
      i = j;
      continue;
    }
    if (/[0-9]/.test(ch)) {
      let j = i + 1;
      while (j < n && /[0-9A-Za-z_]/.test(text[j])) j++;
      push("x", i, j);
      i = j;
      continue;
    }
    push("x", i, i + 1);
    i++;
  }
  return out;
}

function isModOpen(toks: Tok[], idx: number): boolean {
  let k = idx - 1;
  let depth = 0;
  const head: string[] = [];
  for (; k >= 0; k--) {
    const s = toks[k];
    if (s.t === "c") continue;
    if (s.s === "]") depth++;
    else if (s.s === "[") depth--;
    if (depth === 0 && (s.s === ";" || s.s === "{" || s.s === "}")) break;
    head.push(s.s);
  }
  head.reverse();
  let p = 0;
  while (p < head.length) {
    if (head[p] === "#") {
      let d = 0;
      p++;
      if (head[p] === "!") p++;
      do {
        if (head[p] === "[") d++;
        if (head[p] === "]") d--;
        p++;
      } while (d > 0 && p < head.length);
    } else if (head[p] === "pub") {
      p++;
      if (head[p] === "(") while (p < head.length && head[p++] !== ")");
    } else break;
  }
  return head[p] === "mod";
}

/** 🔎 Lexer-based twin of the tree-sitter planner; used for files the grammar rejects and to cross-check the other planner. */
export function planRsLex(text: string): { removals: Removal[]; kept: number; error?: string } {
  const toks = lexRs(text);
  if (!toks) return { removals: [], kept: 0, error: "lex" };
  const stack: ("mod" | "other")[] = [];
  const comments: { pos: number; end: number; def: boolean; idx: number }[] = [];
  const closers: Record<string, string> = { "}": "{", ")": "(", "]": "[" };
  toks.forEach((tk, idx) => {
    if (tk.t === "c") {
      comments.push({ pos: tk.pos, end: tk.end, def: stack.includes("other"), idx });
      return;
    }
    if (tk.s === "{") stack.push(isModOpen(toks, idx) ? "mod" : "other");
    else if (tk.s === "(" || tk.s === "[") stack.push("other");
    else if (closers[tk.s]) stack.pop();
  });
  const safety = new Set<number>();
  comments.forEach((c, i) => {
    if (!/^\/\/\s*SAFETY\b/.test(text.slice(c.pos, c.end))) return;
    safety.add(i);
    for (let j = i + 1; j < comments.length; j++) {
      if (comments[j].idx !== comments[j - 1].idx + 1) break;
      const gap = text.slice(comments[j - 1].end, comments[j].pos);
      if (gap.split("\n").length > 2 || !text.slice(comments[j].pos, comments[j].end).startsWith("//")) break;
      safety.add(j);
    }
  });
  let kept = 0;
  const removals: Removal[] = [];
  comments.forEach((c, i) => {
    if (!c.def) return;
    if (safety.has(i) || keepText(text.slice(c.pos, c.end))) kept++;
    else removals.push({ pos: c.pos, end: c.end });
  });
  return { removals, kept };
}

/** 🧾 Comment-free token signature from the lexer (doc comments included). */
export function rsLexSignature(text: string): string[] | undefined {
  const toks = lexRs(text);
  if (!toks) return undefined;
  const out: string[] = [];
  for (const t of toks) {
    if (t.t === "x") out.push(t.s);
    else if (keepText(t.s) || /^\/\/\s*SAFETY\b/.test(t.s)) out.push(`D:${t.s.trimEnd()}`);
  }
  return out;
}
