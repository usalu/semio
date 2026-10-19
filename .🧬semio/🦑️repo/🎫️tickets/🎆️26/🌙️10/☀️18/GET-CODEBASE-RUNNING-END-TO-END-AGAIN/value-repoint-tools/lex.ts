export type Kind = "id" | "p" | "s" | "c" | "w" | "n" | "l";
export type Tok = { k: Kind; t: string; s: number; e: number };

const IDSTART = /[\p{XID_Start}_]/u;
const IDCONT = /[\p{XID_Continue}]/u;

/** 🔤 Lexes Rust source into identifiers, punctuation, literals, comments and whitespace without losing a byte. */
export function lex(src: string): Tok[] {
  const out: Tok[] = [];
  const n = src.length;
  let i = 0;
  const push = (k: Kind, s: number, e: number) => out.push({ k, t: src.slice(s, e), s, e });
  while (i < n) {
    const c = src[i];
    const s = i;
    if (c === " " || c === "\t" || c === "\n" || c === "\r") {
      while (i < n && (src[i] === " " || src[i] === "\t" || src[i] === "\n" || src[i] === "\r")) i++;
      push("w", s, i);
      continue;
    }
    if (c === "/" && src[i + 1] === "/") {
      while (i < n && src[i] !== "\n") i++;
      push("c", s, i);
      continue;
    }
    if (c === "/" && src[i + 1] === "*") {
      let d = 1;
      i += 2;
      while (i < n && d > 0) {
        if (src[i] === "/" && src[i + 1] === "*") {
          d++;
          i += 2;
        } else if (src[i] === "*" && src[i + 1] === "/") {
          d--;
          i += 2;
        } else i++;
      }
      push("c", s, i);
      continue;
    }
    const raw = /^(?:br|cr|r)(#*)"/.exec(src.slice(i, i + 40));
    if (raw) {
      const close = '"' + raw[1];
      const from = i + raw[0].length;
      const at = src.indexOf(close, from);
      i = at < 0 ? n : at + close.length;
      push("s", s, i);
      continue;
    }
    if (c === '"' || ((c === "b" || c === "c") && src[i + 1] === '"')) {
      i += c === '"' ? 1 : 2;
      while (i < n && src[i] !== '"') i += src[i] === "\\" ? 2 : 1;
      i++;
      push("s", s, Math.min(i, n));
      i = Math.min(i, n);
      continue;
    }
    if (c === "b" && src[i + 1] === "'") {
      i += src[i + 2] === "\\" ? 4 : 2;
      while (i < n && src[i] !== "'") i += src[i] === "\\" ? 2 : 1;
      i++;
      push("s", s, Math.min(i, n));
      i = Math.min(i, n);
      continue;
    }
    if (c === "'") {
      if (src[i + 1] === "\\") {
        i += 3;
        while (i < n && src[i] !== "'") i += src[i] === "\\" ? 2 : 1;
        i++;
        push("s", s, Math.min(i, n));
        i = Math.min(i, n);
        continue;
      }
      const cp = src.codePointAt(i + 1);
      const w = cp !== undefined && cp > 0xffff ? 2 : 1;
      if (src[i + 1 + w] === "'") {
        i += 2 + w;
        push("s", s, i);
        continue;
      }
      i++;
      while (i < n && IDCONT.test(src[i])) i++;
      push("l", s, i);
      continue;
    }
    if (c === "r" && src[i + 1] === "#" && IDSTART.test(src[i + 2] ?? "")) {
      i += 2;
      while (i < n && IDCONT.test(src[i])) i++;
      push("id", s, i);
      continue;
    }
    if (IDSTART.test(c) || (c >= "\ud800" && c <= "\udbff" && IDSTART.test(src.slice(i, i + 2)))) {
      i++;
      while (i < n && (IDCONT.test(src[i]) || (src[i] >= "\ud800" && src[i] <= "\udbff" && IDCONT.test(src.slice(i, i + 2))))) i += src[i] >= "\ud800" && src[i] <= "\udbff" ? 2 : 1;
      push("id", s, i);
      continue;
    }
    if (c >= "0" && c <= "9") {
      while (i < n && /[0-9a-zA-Z_]/.test(src[i])) i++;
      push("n", s, i);
      continue;
    }
    if (c === ":" && src[i + 1] === ":") {
      i += 2;
      push("p", s, i);
      continue;
    }
    i++;
    push("p", s, i);
  }
  return out;
}

export function lineOf(src: string, off: number): number {
  let l = 1;
  for (let i = src.indexOf("\n"); i >= 0 && i < off; i = src.indexOf("\n", i + 1)) l++;
  return l;
}
