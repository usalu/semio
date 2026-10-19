export type Removal = { pos: number; end: number; exact?: boolean };

const DIRECTIVE = /^(eslint[- :]|eslint$|biome-|prettier-|istanbul |c8 |v8 |tslint:|oxlint|deno-lint|cspell:|rustfmt::|clippy::|jshint|jscs)/;

/** 🔖 Keep-rules shared by every language engine: docstrings, region/marker comments, tool directives, SAFETY notes. */
export function keepText(raw: string): boolean {
  if (raw.startsWith("///") && !raw.startsWith("////")) return true;
  if (raw.startsWith("//!")) return true;
  if (raw.startsWith("/**") && !raw.startsWith("/**/") && !raw.startsWith("/***")) return true;
  if (raw.startsWith("/*!")) return true;
  if (raw.includes("🔖")) return true;
  const body = (raw.startsWith("//") ? raw.slice(2) : raw.slice(2, raw.endsWith("*/") ? -2 : undefined)).trim();
  if (body.startsWith("@") || body.startsWith("#") || body.startsWith("!")) return true;
  if (DIRECTIVE.test(body)) return true;
  if (body.startsWith("SAFETY:") || body.startsWith("SAFETY ")) return true;
  return false;
}

/** 🧹 Deletes the given comment ranges from text; whole lines when a comment stands alone, otherwise only the comment and its leading blank. */
export function applyRemovals(text: string, ranges: Removal[]): string {
  const rs = [...ranges].sort((a, b) => b.pos - a.pos);
  let t = text;
  for (const r of rs) {
    let { pos, end } = r;
    const ls = t.lastIndexOf("\n", pos - 1) + 1;
    let le = t.indexOf("\n", end);
    if (le < 0) le = t.length;
    const before = t.slice(ls, pos);
    const after = t.slice(end, le);
    if (before.trim() === "" && after.trim() === "") {
      const cut = le < t.length ? le + 1 : le;
      t = t.slice(0, ls) + t.slice(cut);
      t = tidy(t, ls);
      continue;
    }
    if (r.exact) {
      t = t.slice(0, pos) + t.slice(end);
      continue;
    }
    if (after.trim() === "") {
      let s = pos;
      while (s > ls && (t[s - 1] === " " || t[s - 1] === "\t")) s--;
      t = t.slice(0, s) + t.slice(end);
      continue;
    }
    let s = pos;
    let e = end;
    if ((s === ls || " \t({[".includes(t[s - 1])) && (t[e] === " " || t[e] === "\t")) e++;
    t = t.slice(0, s) + t.slice(e);
  }
  return t;
}

function lineAt(t: string, i: number): [number, number] {
  const s = t.lastIndexOf("\n", i - 1) + 1;
  let e = t.indexOf("\n", i);
  if (e < 0) e = t.length;
  return [s, e];
}

function tidy(t: string, at: number): string {
  if (at >= t.length && at === 0) return t;
  const prevEnd = at - 1;
  if (prevEnd < 0) return t;
  const [ps, pe] = lineAt(t, prevEnd);
  const prev = t.slice(ps, pe).trim();
  if (at >= t.length) return t;
  const [ns, ne] = lineAt(t, at);
  const next = t.slice(ns, ne).trim();
  if (prev === "" && next === "") return t.slice(0, ns) + t.slice(ne + 1);
  if (prev === "" && /^[})\]]/.test(next)) return t.slice(0, ps) + t.slice(pe + 1);
  if (next === "" && /[{(\[]$/.test(prev)) return t.slice(0, ns) + t.slice(ne + 1);
  return t;
}
