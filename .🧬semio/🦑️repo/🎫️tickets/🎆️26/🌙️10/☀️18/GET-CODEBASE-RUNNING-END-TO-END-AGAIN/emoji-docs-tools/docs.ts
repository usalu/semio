import ts from "typescript";

export const LEAD = /^(?:\p{Extended_Pictographic}|\p{Regional_Indicator}{2}|[0-9#*]️?⃣)(?:️|‍\p{Extended_Pictographic}|\p{Emoji_Modifier})*/u;

export type Block = {
  kind: "rs" | "ts";
  line: number;
  insertAt: number;
  pad: "before" | "after" | "both";
  first: string;
  head: string;
  skip?: string;
  decl: string;
  startEmoji?: string;
};

const isWs = (c: string) => c === " " || c === "\t";

function lineOf(starts: number[], pos: number) {
  let lo = 0, hi = starts.length - 1;
  while (lo < hi) { const mid = (lo + hi + 1) >> 1; if (starts[mid] <= pos) lo = mid; else hi = mid - 1; }
  return lo + 1;
}

function lineStarts(text: string) {
  const s = [0];
  for (let i = 0; i < text.length; i++) if (text.charCodeAt(i) === 10) s.push(i + 1);
  return s;
}

function nextDecl(text: string, from: number) {
  let i = from;
  let out = "";
  let guard = 0;
  while (i < text.length && guard++ < 12) {
    let e = text.indexOf("\n", i);
    if (e < 0) e = text.length;
    const l = text.slice(i, e).trim();
    i = e + 1;
    if (!l || l.startsWith("#[") && !l.includes("pub ") || l.startsWith("///") || l.startsWith("//") || l.startsWith("*") || l.startsWith("/*")) continue;
    out = l.slice(0, 200);
    break;
  }
  return out;
}

function structural(first: string) {
  if (first.startsWith("```") || first.startsWith("~~~")) return "fence";
  if (/^#{1,6}\s/.test(first)) return "heading";
  if (first.startsWith("|")) return "table";
  if (/^(?:[-*+]\s|\d+[.)]\s|>\s?)/.test(first)) return "list";
  if (/^\[[^\]]*\]:/.test(first)) return "linkdef";
  if (first.startsWith("<")) return "html";
  if (first.startsWith("@")) return "tag";
  return undefined;
}

export function rustBlocks(text: string): Block[] {
  const n = text.length;
  const toks: { start: number; end: number }[] = [];
  let i = 0;
  while (i < n) {
    const c = text[i];
    const d = text[i + 1];
    if (c === "/" && d === "/") {
      let e = text.indexOf("\n", i);
      if (e < 0) e = n;
      const third = text[i + 2];
      const fourth = text[i + 3];
      if ((third === "/" && fourth !== "/") || third === "!") toks.push({ start: i, end: e });
      i = e;
      continue;
    }
    if (c === "/" && d === "*") {
      let depth = 1;
      i += 2;
      while (i < n && depth > 0) {
        if (text[i] === "/" && text[i + 1] === "*") { depth++; i += 2; }
        else if (text[i] === "*" && text[i + 1] === "/") { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    if (c === "r" || c === "b" || c === "c") {
      let j = i;
      if ((c === "b" || c === "c") && text[j + 1] === "r") j++;
      if (text[j] === "r" || (c === "b" && text[i + 1] === '"') || (c === "c" && text[i + 1] === '"')) {
        const prev = i > 0 ? text[i - 1] : " ";
        if (!/[A-Za-z0-9_]/.test(prev)) {
          if (text[j] === "r") {
            let k = j + 1;
            let hashes = 0;
            while (text[k] === "#") { hashes++; k++; }
            if (text[k] === '"') {
              const close = '"' + "#".repeat(hashes);
              const e = text.indexOf(close, k + 1);
              i = e < 0 ? n : e + close.length;
              continue;
            }
          } else {
            i = j + 1;
            continue;
          }
        }
      }
    }
    if (c === '"') {
      i++;
      while (i < n && text[i] !== '"') i += text[i] === "\\" ? 2 : 1;
      i++;
      continue;
    }
    if (c === "'") {
      if (d === "\\") {
        let e = i + 2;
        while (e < n && text[e] !== "'") e++;
        i = e + 1;
        continue;
      }
      const cp = text.codePointAt(i + 1);
      const w = cp !== undefined && cp > 0xffff ? 2 : 1;
      if (text[i + 1 + w] === "'") { i += 2 + w; continue; }
      i++;
      continue;
    }
    i++;
  }
  const starts = lineStarts(text);
  const out: Block[] = [];
  let k = 0;
  while (k < toks.length) {
    const group = [toks[k]];
    let m = k + 1;
    while (m < toks.length) {
      const between = text.slice(group[group.length - 1].end, toks[m].start);
      if (/^\n[ \t]*$/.test(between) && text.slice(toks[m].start, toks[m].start + 3) === text.slice(group[0].start, group[0].start + 3)) { group.push(toks[m]); m++; } else break;
    }
    k = m;
    const lead = text.slice(starts[lineOf(starts, group[0].start) - 1], group[0].start);
    if (!/^[ \t]*$/.test(lead)) continue;
    let pick = group[0];
    let body = "";
    for (const t of group) {
      body = text.slice(t.start + 3, t.end);
      if (body.trim()) { pick = t; break; }
    }
    if (!body.trim()) continue;
    const trimmed = body.trim();
    const m2 = trimmed.match(LEAD);
    const base: Block = {
      kind: "rs", line: lineOf(starts, pick.start), insertAt: 0, pad: "after", first: trimmed, head: trimmed.slice(0, 120),
      decl: nextDecl(text, group[group.length - 1].end + 1),
    };
    if (m2) { base.startEmoji = m2[0]; out.push(base); continue; }
    const sk = structural(trimmed);
    if (sk) base.skip = sk;
    if (body.startsWith(" ")) { base.insertAt = pick.start + 4; base.pad = "after"; }
    else { base.insertAt = pick.start + 3; base.pad = "both"; }
    out.push(base);
  }
  return out;
}

export function tsBlocks(path: string, text: string): Block[] {
  const kind = path.endsWith("x") && !path.endsWith(".mjs") ? ts.ScriptKind.TSX : path.endsWith(".js") || path.endsWith(".mjs") ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, false, kind);
  const poss = new Set<number>();
  const visit = (node: ts.Node) => {
    poss.add(node.pos);
    poss.add(node.end);
    ts.forEachChild(node, visit);
  };
  visit(sf);
  const seen = new Set<number>();
  const ranges: ts.CommentRange[] = [];
  for (const p of poss) {
    for (const r of ts.getLeadingCommentRanges(text, p) ?? []) if (!seen.has(r.pos)) { seen.add(r.pos); ranges.push(r); }
    for (const r of ts.getTrailingCommentRanges(text, p) ?? []) if (!seen.has(r.pos)) { seen.add(r.pos); ranges.push(r); }
  }
  ranges.sort((a, b) => a.pos - b.pos);
  const starts = lineStarts(text);
  const out: Block[] = [];
  for (const r of ranges) {
    if (r.kind !== ts.SyntaxKind.MultiLineCommentTrivia) continue;
    if (!text.startsWith("/**", r.pos) || text.startsWith("/***", r.pos) || text.startsWith("/**/", r.pos)) continue;
    const endBody = r.end - 2;
    let j = r.pos + 3;
    let firstLineEnd = text.indexOf("\n", j);
    if (firstLineEnd < 0 || firstLineEnd > endBody) firstLineEnd = endBody;
    let insertAt = -1;
    let pad: "before" | "after" | "both" = "after";
    let first = text.slice(j, firstLineEnd);
    if (first.trim()) {
      let p = j;
      while (p < firstLineEnd && isWs(text[p])) p++;
      insertAt = p === j ? j : j + 1;
      pad = p === j ? "both" : "after";
      if (p > j) insertAt = j + 1;
      first = first.trim();
    } else {
      let ln = firstLineEnd + 1;
      let found = false;
      while (ln < endBody) {
        let le = text.indexOf("\n", ln);
        if (le < 0 || le > endBody) le = endBody;
        const raw = text.slice(ln, le);
        const mm = raw.match(/^([ \t]*\*+)([ \t]?)(.*)$/);
        const content = mm ? mm[3] : raw.trim();
        if (content.trim()) {
          if (mm) { insertAt = ln + mm[1].length + (mm[2] ? 1 : 0); pad = mm[2] ? "after" : "both"; }
          else { insertAt = ln + (raw.length - raw.trimStart().length); pad = "after"; }
          first = content.trim();
          found = true;
          break;
        }
        ln = le + 1;
      }
      if (!found) continue;
    }
    first = first.replace(/\s*\*\/$/, "");
    const m2 = first.match(LEAD);
    const b: Block = { kind: "ts", line: lineOf(starts, r.pos), insertAt, pad, first, head: first.slice(0, 120), decl: nextDecl(text, r.end) };
    if (m2) b.startEmoji = m2[0];
    else {
      const sk = structural(first);
      if (sk) b.skip = sk;
    }
    out.push(b);
  }
  return out;
}

export function blocksOf(path: string, text: string): Block[] {
  if (path.endsWith(".rs")) return rustBlocks(text);
  return tsBlocks(path, text);
}
