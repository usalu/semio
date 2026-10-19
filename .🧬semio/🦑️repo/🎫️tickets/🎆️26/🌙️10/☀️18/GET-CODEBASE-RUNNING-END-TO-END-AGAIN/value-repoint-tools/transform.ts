import { lex, lineOf, Tok } from "./lex.ts";

export type Moved = { modules: Record<string, string>; functions: Record<string, string>; items: Record<string, string>; deriveMacroNames: string[] };
export type Verdict = "ok" | "missing" | "unknown";
export type Ctx = { moved: Moved; facade: string; verify: (lib: string, segs: string[]) => Verdict; aliasHeads?: string[]; globHeads?: string[] };
export type Note = { kind: string; line: number; text: string };
export type Result = { out: string; libs: Set<string>; notes: Note[]; rewrites: number };

type UT = { lead: boolean; prefix: string[]; glob: boolean; group: UT[] | null; alias: string | null };
type Leaf = { segs: string[]; glob: boolean; alias: string | null };
type Stmt = { attrStart: number; useTok: number; semi: number; start: number; end: number; headerEnd: number; tree: UT };
type Edit = { s: number; e: number; text: string };

const ITEM_PREV = new Set([";", "}", "{", "]", ")", ","]);

/** 🧭 Rewrites facade paths of moved modules to their owning crates in one Rust source; unresolved paths stay and are noted. */
export function transform(src: string, ctx: Ctx): Result {
  const res: Result = { out: src, libs: new Set(), notes: [], rewrites: 0 };
  if (!src.includes(ctx.facade) && !(ctx.aliasHeads ?? []).some((h) => src.includes(`${h}::value::`) || src.includes(`${h}::__value::`)) && !(ctx.globHeads ?? []).some((h) => src.includes(`${h}::`))) return res;
  const F = ctx.facade;
  const { modules, items, functions } = ctx.moved;
  const derive = new Set(ctx.moved.deriveMacroNames);
  const heads = ctx.aliasHeads ?? [];
  const gheads = ctx.globHeads ?? [];
  const isMovedName = (n: string | undefined) => n !== undefined && (modules[n] !== undefined || items[n] !== undefined || functions[n] !== undefined);
  const toks = lex(src);
  const sig: number[] = [];
  for (let i = 0; i < toks.length; i++) if (toks[i].k !== "w" && toks[i].k !== "c") sig.push(i);
  const T = (j: number): Tok | undefined => (j >= 0 && j < sig.length ? toks[sig[j]] : undefined);
  const isP = (j: number, t: string) => T(j)?.k === "p" && T(j)!.t === t;
  const isId = (j: number, t?: string) => T(j)?.k === "id" && (t === undefined || T(j)!.t === t);
  const note = (kind: string, off: number, text: string) => res.notes.push({ kind, line: lineOf(src, off), text });
  const edits: Edit[] = [];

  const deriveRanges: [number, number][] = [];
  const deriveNames = new Set<string>();
  for (let j = 0; j < sig.length; j++) {
    if (!isId(j, "derive") || !isP(j + 1, "(")) continue;
    let d = 0;
    let k = j + 1;
    for (; k < sig.length; k++) {
      if (isP(k, "(")) d++;
      else if (isP(k, ")") && --d === 0) break;
      else if (isId(k)) deriveNames.add(T(k)!.t);
    }
    deriveRanges.push([T(j + 1)!.s, T(Math.min(k, sig.length - 1))!.e]);
  }
  const inDerive = (off: number) => deriveRanges.some(([a, b]) => off >= a && off < b);

  function parseTree(j: number): { t: UT; j: number } | null {
    const t: UT = { lead: false, prefix: [], glob: false, group: null, alias: null };
    if (isP(j, "::")) {
      t.lead = true;
      j++;
    }
    for (;;) {
      if (isP(j, "{")) {
        j++;
        t.group = [];
        while (!isP(j, "}")) {
          const c = parseTree(j);
          if (!c) return null;
          t.group.push(c.t);
          j = c.j;
          if (isP(j, ",")) j++;
          else if (!isP(j, "}")) return null;
        }
        return { t, j: j + 1 };
      }
      if (isP(j, "*")) {
        t.glob = true;
        return { t, j: j + 1 };
      }
      if (!isId(j)) return null;
      t.prefix.push(T(j)!.t);
      j++;
      if (isP(j, "::")) {
        j++;
        continue;
      }
      if (isId(j, "as") && (isId(j + 1) || T(j + 1)?.t === "_")) {
        t.alias = T(j + 1)!.t;
        j += 2;
      }
      return { t, j };
    }
  }

  const stmts: Stmt[] = [];
  const inStmt = (off: number) => stmts.some((s) => off >= s.start && off < s.end);
  for (let j = 0; j < sig.length; j++) {
    if (!isId(j, "use") || isP(j + 1, "<")) continue;
    const prev = T(j - 1);
    const okPrev = !prev || (prev.k === "p" && ITEM_PREV.has(prev.t)) || (prev.k === "id" && prev.t === "pub");
    if (!okPrev) continue;
    const pt = parseTree(j + 1);
    if (!pt || !isP(pt.j, ";")) {
      let k = j;
      while (k < sig.length && !isP(k, ";")) k++;
      const body = src.slice(T(j)!.s, T(Math.min(k, sig.length - 1))!.e);
      if (body.includes(F)) note("use-parse-failure", T(j)!.s, body.slice(0, 120));
      continue;
    }
    let a = j - 1;
    if (isId(a, "pub")) a--;
    else if (isP(a, ")")) {
      let d = 0;
      let b = a;
      for (; b >= 0; b--) {
        if (isP(b, ")")) d++;
        else if (isP(b, "(") && --d === 0) break;
      }
      if (isId(b - 1, "pub")) a = b - 2;
    }
    for (;;) {
      if (!isP(a, "]")) break;
      let d = 0;
      let b = a;
      for (; b >= 0; b--) {
        if (isP(b, "]")) d++;
        else if (isP(b, "[") && --d === 0) break;
      }
      if (b >= 1 && isP(b - 1, "#")) a = b - 2;
      else break;
    }
    const first = a + 1;
    stmts.push({ attrStart: T(first)!.s, useTok: T(j)!.s, semi: T(pt.j)!.e, start: T(first)!.s, end: T(pt.j)!.e, headerEnd: T(j)!.s, tree: pt.t });
    j = pt.j;
  }

  function leaves(t: UT, acc: string[], out: Leaf[]): void {
    const p = [...acc, ...t.prefix];
    if (t.group) for (const c of t.group) leaves(c, p, out);
    else out.push({ segs: p, glob: t.glob, alias: t.alias });
  }

  const outsideUse = (tj: number) => !inStmt(T(tj)!.s);
  const macroUsed = new Set<string>();
  const traitUsed = new Set<string>();
  const bound = new Set<string>();
  const localDef = new Set<string>();
  const bareUse = new Map<string, number>();
  for (let j = 0; j < sig.length; j++) {
    const t = T(j)!;
    if (t.k !== "id") continue;
    if ((t.t === "struct" || t.t === "enum" || t.t === "trait" || t.t === "fn" || t.t === "type" || t.t === "const" || t.t === "static" || t.t === "mod" || t.t === "union") && isId(j + 1)) localDef.add(T(j + 1)!.t);
    if (!outsideUse(j) || t.t === F) continue;
    const prevColons = isP(j - 1, "::") || isP(j - 1, ".");
    if (derive.has(t.t) || modules[t.t] !== undefined || items[t.t] !== undefined || functions[t.t] !== undefined) {
      if (inDerive(t.s)) macroUsed.add(t.t);
      else if (!prevColons) {
        traitUsed.add(t.t);
        if (!isP(j + 1, "::")) bareUse.set(t.t, (bareUse.get(t.t) ?? 0) + 1);
      }
    }
  }
  for (const s of stmts) {
    const ls: Leaf[] = [];
    leaves(s.tree, [], ls);
    for (const l of ls) if (!l.glob) bound.add(l.alias ?? l.segs[l.segs.length - 1]);
  }

  function classify(leaf: Leaf): { kind: "keep" } | { kind: "move"; segs: string[]; checkSegs: string[] } | { kind: "unresolved"; why: string } | { kind: "rootglob" } | { kind: "alias" } {
    const l: Leaf = heads.includes(leaf.segs[0]) && (leaf.segs[1] === "value" || leaf.segs[1] === "__value") && leaf.segs.length > 2 ? { ...leaf, segs: [F, ...leaf.segs.slice(2)] } : gheads.includes(leaf.segs[0]) && isMovedName(leaf.segs[1]) ? { ...leaf, segs: [F, ...leaf.segs.slice(1)] } : leaf;
    if (l.segs[0] !== F) return { kind: "keep" };
    let r = l.segs.slice(1);
    if (r.length === 0) return l.glob ? { kind: "rootglob" } : l.alias ? { kind: "alias" } : { kind: "keep" };
    if (r[0] === "value" && r.length > 1 && (modules[r[1]] || items[r[1]] || functions[r[1]])) r = r.slice(1);
    let lib: string;
    let segs: string[];
    if (modules[r[0]]) {
      lib = modules[r[0]];
      segs = [lib, ...r];
    } else if (items[r[0]]) {
      lib = modules[items[r[0]]];
      segs = [lib, items[r[0]], ...r];
    } else if (functions[r[0]]) {
      lib = functions[r[0]];
      segs = [lib, ...r];
    } else return { kind: "keep" };
    const checkSegs = segs.slice(1).filter((x, i, a) => !(x === "self" && i === a.length - 1));
    const v = ctx.verify(lib, checkSegs);
    if (v !== "ok") return { kind: "unresolved", why: `${v}: ${segs.join("::")}${l.glob ? "::*" : ""}` };
    return { kind: "move", segs, checkSegs };
  }

  function emit(t: UT, keep: (l: Leaf) => boolean, acc: string[]): string | null {
    const p = [...acc, ...t.prefix];
    const lead = t.lead ? "::" : "";
    const pre = lead + t.prefix.join("::");
    if (t.group) {
      const parts = t.group.map((c) => emit(c, keep, p)).filter((x): x is string => x !== null);
      if (!parts.length) return null;
      if (parts.length === 1 && parts[0] !== "self" && !parts[0].startsWith("self ")) return pre ? `${pre}::${parts[0]}` : parts[0];
      return pre ? `${pre}::{${parts.join(", ")}}` : `{${parts.join(", ")}}`;
    }
    const l: Leaf = { segs: p, glob: t.glob, alias: t.alias };
    if (!keep(l)) return null;
    return pre + (t.glob ? (pre ? "::*" : "*") : "") + (t.alias ? ` as ${t.alias}` : "");
  }

  const lineStartIndent = (off: number): string | null => {
    const ls = src.lastIndexOf("\n", off - 1) + 1;
    const seg = src.slice(ls, off);
    return /^[ \t]*$/.test(seg) ? seg : null;
  };

  type TN = { kids: Map<string, TN>; ends: string[] };
  const node = (): TN => ({ kids: new Map(), ends: [] });
  function emitNode(name: string, n: TN): string[] {
    if (!n.kids.size) return n.ends.map((e) => (e === "::*" ? `${name}::*` : `${name}${e}`));
    const inner = [...n.ends.map((e) => (e === "" ? "self" : e === "::*" ? "*" : `self${e}`)), ...[...n.kids].flatMap(([k, v]) => emitNode(k, v))];
    if (inner.length === 1 && inner[0] !== "self" && !inner[0].startsWith("self ")) return [`${name}::${inner[0]}`];
    return [`${name}::{${inner.join(", ")}}`];
  }
  function newStatements(header: string, moves: { segs: string[]; glob: boolean; alias: string | null }[], sep: string): string {
    const libs = new Map<string, TN>();
    for (const m of moves) {
      let segs = m.segs;
      if (segs[segs.length - 1] === "self") segs = segs.slice(0, -1);
      let cur = libs.get(segs[0]) ?? libs.set(segs[0], node()).get(segs[0])!;
      for (const seg of segs.slice(1)) cur = cur.kids.get(seg) ?? cur.kids.set(seg, node()).get(seg)!;
      const end = m.glob ? "::*" : m.alias ? ` as ${m.alias}` : "";
      if (!cur.ends.includes(end)) cur.ends.push(end);
    }
    return [...libs].map(([lib, tn]) => {
      const items = [...tn.kids].flatMap(([k, v]) => emitNode(k, v));
      return `${header}use ${lib}::${items.length === 1 ? items[0] : `{${items.join(", ")}}`};`;
    }).join(sep);
  }

  for (const s of stmts) {
    const ls: Leaf[] = [];
    leaves(s.tree, [], ls);
    if (!ls.some((l) => l.segs[0] === F || (heads.includes(l.segs[0]) && (l.segs[1] === "value" || l.segs[1] === "__value")) || (gheads.includes(l.segs[0]) && isMovedName(l.segs[1])))) continue;
    const lk = (l: Leaf) => `${l.segs.join("::")}|${l.glob}|${l.alias ?? ""}`;
    const verdict = new Map<string, ReturnType<typeof classify>>();
    const moves: { segs: string[]; glob: boolean; alias: string | null }[] = [];
    const keepOld = new Set<string>();
    const isPub = /\bpub\b/.test(src.slice(s.attrStart, s.useTok).replace(/#\[[^\]]*\]/g, ""));
    for (const l of ls) {
      const c = classify(l);
      verdict.set(lk(l), c);
      if (c.kind === "alias") note("alias", s.start, src.slice(s.start, s.end).slice(0, 120));
      else if (c.kind === "rootglob") {
        if (isPub) note("reexport-glob", s.start, "pub use facade::* re-exports moved items implicitly");
      } else if (c.kind === "unresolved") note("unresolved-target", s.start, c.why);
      else if (c.kind === "move") {
        const name = l.segs[l.segs.length - 1];
        const coll = !l.glob && derive.has(name) && (l.segs.length === 2 || (l.segs.length === 3 && l.segs[1] === "value"));
        if (coll && !l.alias) {
          const m = macroUsed.has(name);
          const tr = traitUsed.has(name);
          if (m) keepOld.add(lk(l));
          if (m && !tr) continue;
        }
        moves.push({ segs: c.segs, glob: l.glob, alias: l.alias });
      }
    }
    if (!moves.length) continue;
    const kept = emit(s.tree, (l) => verdict.get(lk(l))?.kind !== "move" || keepOld.has(lk(l)), []);
    const header = src.slice(s.attrStart, s.useTok);
    const indent = lineStartIndent(s.attrStart);
    const sep = indent === null ? " " : "\n" + indent;
    const fresh = newStatements(header, moves, sep);
    const root = s.tree.lead ? "::" : "";
    const text = kept === null ? fresh : `${header}use ${kept.startsWith("::") ? "" : root}${kept};${sep}${fresh}`;
    edits.push({ s: s.start, e: s.end, text });
    for (const m of moves) res.libs.add(m.segs[0]);
    res.rewrites += moves.length;
  }

  for (const s of stmts) {
    const ls: Leaf[] = [];
    leaves(s.tree, [], ls);
    const hasRoot = ls.some((l) => l.segs.length === 1 && l.segs[0] === F && l.glob);
    if (!hasRoot) continue;
    const isPub = /\bpub\b/.test(src.slice(s.start, s.useTok).replace(/#\[[^\]]*\]/g, ""));
    if (isPub) continue;
    const need: { segs: string[]; glob: boolean; alias: string | null }[] = [];
    const seen = new Set<string>();
    for (let j = 0; j < sig.length; j++) {
      const t = T(j)!;
      if (t.k !== "id" || !outsideUse(j) || isP(j - 1, "::") || isP(j - 1, ".") || seen.has(t.t)) continue;
      const mod = modules[t.t] !== undefined;
      const item = items[t.t] !== undefined;
      const fn = functions[t.t] !== undefined;
      if (!mod && !item && !fn) continue;
      if (mod && !isP(j + 1, "::")) continue;
      if (inDerive(t.s) && derive.has(t.t)) continue;
      seen.add(t.t);
      if (bound.has(t.t) || localDef.has(t.t)) {
        note("rootglob-name-clash", t.s, t.t);
        continue;
      }
      const segs = mod ? [modules[t.t], t.t] : item ? [modules[items[t.t]], items[t.t], t.t] : [functions[t.t], t.t];
      const v = ctx.verify(segs[0], segs.slice(1));
      if (v !== "ok") {
        note("unresolved-target", t.s, `${v}: ${segs.join("::")}`);
        continue;
      }
      need.push({ segs, glob: false, alias: null });
    }
    if (!need.length) continue;
    const header = src.slice(s.attrStart, s.useTok);
    const indent = lineStartIndent(s.attrStart);
    const sep = indent === null ? " " : "\n" + indent;
    edits.push({ s: s.end, e: s.end, text: sep + newStatements(header, need, sep) });
    for (const m of need) res.libs.add(m.segs[0]);
    res.rewrites += need.length;
  }

  for (let j = 0; j < sig.length; j++) {
    const t = T(j)!;
    if (t.k !== "id" || (t.t !== F && !heads.includes(t.t) && !gheads.includes(t.t)) || !outsideUse(j) || isP(j - 1, ".") || isP(j + 1, "!") || !isP(j + 1, "::")) continue;
    if (isP(j - 1, "::") && (isId(j - 2) || isP(j - 2, ">"))) continue;
    if (isP(j - 1, "$")) continue;
    let base = j;
    if (t.t !== F) {
      if (heads.includes(t.t) && (isId(j + 2, "value") || isId(j + 2, "__value")) && isP(j + 3, "::")) base = j + 2;
      else if (!(gheads.includes(t.t) && isMovedName(T(j + 2)?.t))) continue;
    }
    const ids: number[] = [];
    let k = base;
    while (isP(k + 1, "::") && isId(k + 2)) {
      ids.push(k + 2);
      k += 2;
    }
    if (!ids.length) {
      if (isP(j + 1, "::") && isP(j + 2, "{")) note("nonuse-brace-group", t.s, "facade brace group outside a use statement");
      continue;
    }
    let first = 0;
    if (T(ids[0])!.t === "value" && ids.length > 1 && (modules[T(ids[1])!.t] || items[T(ids[1])!.t] || functions[T(ids[1])!.t])) first = 1;
    const name = T(ids[first])!.t;
    if (derive.has(name) && inDerive(t.s)) continue;
    const rest = ids.slice(first).map((x) => T(x)!.t);
    let lib: string;
    let verifyPath: string[];
    let text: string;
    let endTok: number;
    if (modules[name]) {
      lib = modules[name];
      verifyPath = rest;
      text = lib;
      endTok = first === 1 ? ids[0] : base !== j ? base : -1;
    } else if (items[name]) {
      lib = modules[items[name]];
      verifyPath = [items[name], name];
      text = `${lib}::${items[name]}::${name}`;
      endTok = ids[first];
    } else if (functions[name]) {
      lib = functions[name];
      verifyPath = [name];
      text = `${lib}::${name}`;
      endTok = ids[first];
    } else continue;
    const v = ctx.verify(lib, verifyPath);
    if (v !== "ok") {
      note("unresolved-target", t.s, `${v}: ${lib}::${verifyPath.join("::")}`);
      continue;
    }
    edits.push({ s: t.s, e: endTok === -1 ? t.e : T(endTok)!.e, text });
    res.libs.add(lib);
    res.rewrites++;
  }

  for (const t of toks) {
    if (t.k === "c" && t.t.includes(F)) {
      const re = new RegExp(`${F}::(?:value::)?([A-Za-z_]+)`, "g");
      const nt = t.t.replace(re, (m, n: string) => {
        if (modules[n]) return `${modules[n]}::${n}`;
        if (items[n]) return `${modules[items[n]]}::${items[n]}::${n}`;
        if (functions[n]) return `${functions[n]}::${n}`;
        return m;
      });
      if (nt !== t.t) edits.push({ s: t.s, e: t.e, text: nt });
    } else if (t.k === "s" && t.t.includes(F + "::")) {
      const re = new RegExp(`${F}::(?:value::)?([A-Za-z_]+)`, "g");
      for (const m of t.t.matchAll(re)) if (modules[m[1]] || items[m[1]] || functions[m[1]]) note("string-mention", t.s, t.t.slice(0, 140));
    }
  }
  if (/\bextern\s+crate\s+semio_framework_value\s+as\b/.test(src)) note("alias", 0, "extern crate alias");

  edits.sort((a, b) => b.s - a.s || b.e - a.e);
  let out = src;
  let last = Infinity;
  for (const e of edits) {
    if (e.e > last) {
      note("edit-overlap", e.s, "skipped overlapping edit");
      continue;
    }
    out = out.slice(0, e.s) + e.text + out.slice(e.e);
    last = e.s;
  }
  res.out = out;
  return res;
}
