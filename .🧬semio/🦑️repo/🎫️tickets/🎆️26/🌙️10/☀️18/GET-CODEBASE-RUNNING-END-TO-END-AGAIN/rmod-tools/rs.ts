import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { root, walkFiles } from "./lib.ts";

export type Tok = { k: "id" | "p" | "str" | "life" | "num"; s: string; a: number; b: number };

export const lex = (src: string): Tok[] => {
  const out: Tok[] = [];
  const n = src.length;
  let i = 0;
  const isIdStart = (c: string) => /[A-Za-z_\u0080-￿]/u.test(c);
  const isId = (c: string) => /[A-Za-z0-9_\u0080-￿]/u.test(c);
  while (i < n) {
    const c = src[i]!;
    if (c === " " || c === "\n" || c === "\t" || c === "\r") { i++; continue; }
    if (c === "/" && src[i + 1] === "/") { while (i < n && src[i] !== "\n") i++; continue; }
    if (c === "/" && src[i + 1] === "*") {
      let d = 1; i += 2;
      while (i < n && d > 0) {
        if (src[i] === "/" && src[i + 1] === "*") { d++; i += 2; } else if (src[i] === "*" && src[i + 1] === "/") { d--; i += 2; } else i++;
      }
      continue;
    }
    const a = i;
    if (c === '"' ) {
      i++;
      while (i < n && src[i] !== '"') { if (src[i] === "\\") i++; i++; }
      i++;
      out.push({ k: "str", s: src.slice(a, i), a, b: i });
      continue;
    }
    if (isIdStart(c)) {
      let j = i;
      while (j < n && isId(src[j]!)) j++;
      const w = src.slice(i, j);
      if ((w === "r" || w === "br" || w === "cr") && (src[j] === '"' || src[j] === "#")) {
        let h = 0; let k = j;
        while (src[k] === "#") { h++; k++; }
        if (src[k] === '"') {
          const end = '"' + "#".repeat(h);
          const e = src.indexOf(end, k + 1);
          i = e < 0 ? n : e + end.length;
          out.push({ k: "str", s: src.slice(a, i), a, b: i });
          continue;
        }
      }
      if ((w === "b" || w === "c") && src[j] === '"') {
        let k = j + 1;
        while (k < n && src[k] !== '"') { if (src[k] === "\\") k++; k++; }
        i = k + 1;
        out.push({ k: "str", s: src.slice(a, i), a, b: i });
        continue;
      }
      if (w === "b" && src[j] === "'") {
        let k = j + 1;
        while (k < n && src[k] !== "'") { if (src[k] === "\\") k++; k++; }
        i = k + 1;
        out.push({ k: "str", s: src.slice(a, i), a, b: i });
        continue;
      }
      if (w === "r" && src[j] === "#" && isIdStart(src[j + 1] ?? " ")) {
        let k = j + 1;
        while (k < n && isId(src[k]!)) k++;
        i = k;
        out.push({ k: "id", s: src.slice(a, i), a, b: i });
        continue;
      }
      i = j;
      out.push({ k: "id", s: w, a, b: i });
      continue;
    }
    if (c === "'") {
      if (src[i + 1] === "\\") {
        let k = i + 2;
        while (k < n && src[k] !== "'") k++;
        i = k + 1;
        out.push({ k: "str", s: src.slice(a, i), a, b: i });
        continue;
      }
      const cp = src.codePointAt(i + 1)!;
      const len = cp > 0xffff ? 2 : 1;
      if (src[i + 1 + len] === "'") {
        i = i + 2 + len;
        out.push({ k: "str", s: src.slice(a, i), a, b: i });
        continue;
      }
      let j = i + 1;
      while (j < n && isId(src[j]!)) j++;
      i = j;
      out.push({ k: "life", s: src.slice(a, i), a, b: i });
      continue;
    }
    if (c >= "0" && c <= "9") {
      let j = i;
      while (j < n) {
        const d = src[j]!;
        if (isId(d)) j++;
        else if (d === "." && src[j + 1] >= "0" && src[j + 1] <= "9") j++;
        else break;
      }
      i = j;
      out.push({ k: "num", s: src.slice(a, i), a, b: i });
      continue;
    }
    if (c === ":" && src[i + 1] === ":") { i += 2; out.push({ k: "p", s: "::", a, b: i }); continue; }
    i++;
    out.push({ k: "p", s: c, a, b: i });
  }
  return out;
};

export type UseEntry = { segs: string[]; toks: number[]; offs: [number, number][]; glob: boolean; alias: string | null; lead: boolean; bind: string | null; mod: ModDecl };
export type ModDecl = { name: string; nameTok: number; nameOff: [number, number]; file: string; inline: boolean; range: [number, number]; pathAttr: string | null; parent: ModDecl | null; kids: ModDecl[]; uses: UseEntry[]; externs: { crate: string; alias: string }[]; isRoot: boolean; includes: string[] };
export type Parsed = { file: string; size: number; root: ModDecl; decls: ModDecl[]; chains: Chain[] };
export type Chain = { segs: string[]; offs: [number, number][]; lead: boolean; mod: ModDecl };

const interned = new Map<string, string>();
export const intern = (s: string): string => { let r = interned.get(s); if (r === undefined) { r = Buffer.from(s, 'utf8').toString('utf8'); interned.set(r, r); } return r; };
const kw = new Set(["crate", "self", "super", "Self"]);

export const parse = (file: string, old: Set<string>): Parsed => {
  const src = readFileSync(file, "utf8");
  const toks = lex(src);
  const match = new Int32Array(toks.length).fill(-1);
  const st: number[] = [];
  for (let i = 0; i < toks.length; i++) {
    const t = toks[i]!;
    if (t.k !== "p") continue;
    if (t.s === "{" || t.s === "(" || t.s === "[") st.push(i);
    else if (t.s === "}" || t.s === ")" || t.s === "]") { const o = st.pop(); if (o !== undefined) { match[o] = i; match[i] = o; } }
  }
  const rootDecl: ModDecl = { name: "", nameTok: -1, nameOff: [0, 0], file, inline: true, range: [0, toks.length], pathAttr: null, parent: null, kids: [], uses: [], externs: [], isRoot: true, includes: [] };
  const decls: ModDecl[] = [rootDecl];
  const chains: Chain[] = [];
  const P = (i: number, s: string) => toks[i]?.k === "p" && toks[i]!.s === s;
  const stack: ModDecl[] = [rootDecl];
  let attrs: { path: string | null }[] = [];
  const strVal = (t: Tok) => t.s.slice(1, -1);
  let i = 0;
  const parseUseTree = (j0: number, prefix: { segs: string[]; toks: number[] }, lead0: boolean, decl: ModDecl): number => {
    let j = j0;
    let lead = lead0;
    if (P(j, "::") && prefix.segs.length === 0) { lead = true; j++; }
    const segs = prefix.segs.slice();
    const tks = prefix.toks.slice();
    for (;;) {
      if (P(j, "{")) {
        const end = match[j]!;
        let k = j + 1;
        while (k < end) {
          k = parseUseTree(k, { segs, toks: tks }, lead, decl);
          if (P(k, ",")) k++;
        }
        return end + 1;
      }
      if (P(j, "*")) { decl.uses.push({ segs: segs.slice(), toks: tks.slice(), offs: [], glob: true, alias: null, lead, bind: null, mod: decl }); return j + 1; }
      const t = toks[j];
      if (!t || t.k !== "id") return j + 1;
      if (t.s === "self" && prefix.segs.length > 0 && segs.length === prefix.segs.length) {
        let alias: string | null = null;
        let m = j + 1;
        if (toks[m]?.s === "as") { alias = intern(toks[m + 1]!.s); m += 2; }
        decl.uses.push({ segs: segs.slice(), toks: tks.slice(), offs: [], glob: false, alias, lead, bind: alias ?? segs[segs.length - 1]!, mod: decl });
        return m;
      }
      segs.push(intern(t.s)); tks.push(j);
      j++;
      if (P(j, "::")) { j++; continue; }
      let alias: string | null = null;
      if (toks[j]?.k === "id" && toks[j]!.s === "as") { alias = intern(toks[j + 1]!.s); j += 2; }
      decl.uses.push({ segs: segs.slice(), toks: tks.slice(), offs: [], glob: false, alias, lead, bind: alias ?? segs[segs.length - 1]!, mod: decl });
      return j;
    }
  };
  const inUseRanges: [number, number][] = [];
  while (i < toks.length) {
    while (stack.length > 1 && i >= stack[stack.length - 1]!.range[1]) stack.pop();
    const cur = stack[stack.length - 1]!;
    const t = toks[i]!;
    if (t.k === "p" && t.s === "#" && (P(i + 1, "[") || (P(i + 1, "!") && P(i + 2, "[")))) {
      const o = P(i + 1, "[") ? i + 1 : i + 2;
      const e = match[o]!;
      let pa: string | null = null;
      if (toks[o + 1]?.s === "path" && P(o + 2, "=") && toks[o + 3]?.k === "str") pa = strVal(toks[o + 3]!);
      attrs.push({ path: pa });
      i = e + 1;
      continue;
    }
    if (t.k === "id" && t.s === "pub") {
      let j = i + 1;
      if (P(j, "(")) {
        if (toks[j + 1]?.s === "in") {
          const segs: string[] = []; const tk: number[] = [];
          let q = j + 2;
          if (P(q, "::")) q++;
          while (toks[q]?.k === "id") { segs.push(intern(toks[q]!.s)); tk.push(q); q++; if (P(q, "::")) q++; else break; }
          if (segs.length > 0 && segs.some((x) => old.has(x))) chains.push({ segs, offs: tk.map((z) => [toks[z]!.a, toks[z]!.b] as [number, number]), lead: false, mod: cur });
        }
        j = match[j]! + 1;
      }
      i = j;
      continue;
    }
    if (t.k === "id" && t.s === "mod" && toks[i + 1]?.k === "id" && (P(i + 2, ";") || P(i + 2, "{")) && !P(i - 1, "$") && !P(i - 1, ":")) {
      const pa = attrs.map((a) => a.path).find((x) => x !== null) ?? null;
      attrs = [];
      const name = intern(toks[i + 1]!.s);
      if (P(i + 2, ";")) {
        const d: ModDecl = { name, nameTok: i + 1, nameOff: [toks[i + 1]!.a, toks[i + 1]!.b], file, inline: false, range: [i, i + 3], pathAttr: pa, parent: cur, kids: [], uses: [], externs: [], isRoot: false, includes: [] };
        cur.kids.push(d); decls.push(d);
        i += 3;
      } else {
        const e = match[i + 2]!;
        const d: ModDecl = { name, nameTok: i + 1, nameOff: [toks[i + 1]!.a, toks[i + 1]!.b], file, inline: true, range: [i + 3, e], pathAttr: pa, parent: cur, kids: [], uses: [], externs: [], isRoot: false, includes: [] };
        cur.kids.push(d); decls.push(d);
        stack.push(d);
        i += 3;
      }
      continue;
    }
    if (t.k === "id" && t.s === "use" && !P(i + 1, "$") && !P(i + 1, "<") && !P(i - 1, ".")) {
      attrs = [];
      let j = i + 1;
      const end = (() => { let k = j; let dep = 0; while (k < toks.length) { const tk = toks[k]!; if (tk.k === "p") { if (tk.s === "{") dep++; else if (tk.s === "}") dep--; else if (tk.s === ";" && dep === 0) return k; } k++; } return toks.length; })();
      parseUseTree(j, { segs: [], toks: [] }, false, cur);
      inUseRanges.push([i, end]);
      i = end + 1;
      continue;
    }
    if (t.k === "id" && t.s === "extern" && toks[i + 1]?.s === "crate" && toks[i + 2]?.k === "id") {
      attrs = [];
      const c = toks[i + 2]!.s;
      let alias = c;
      let j = i + 3;
      if (toks[j]?.s === "as") { alias = toks[j + 1]!.s; j += 2; }
      cur.externs.push({ crate: intern(c), alias: intern(alias) });
      i = j + 1;
      continue;
    }
    if (t.k === "id" && t.s === "include" && P(i + 1, "!") && P(i + 2, "(") && toks[i + 3]?.k === "str" && toks[i + 3]!.s.startsWith('"') && P(i + 4, ")")) {
      cur.includes.push(toks[i + 3]!.s.slice(1, -1));
      attrs = [];
      i += 5;
      continue;
    }
    if (t.k === "id") {
      attrs = [];
      const prev = toks[i - 1];
      const dotPrev = !!prev && prev.k === "p" && prev.s === ".";
      const colPrev = !!prev && prev.k === "p" && prev.s === "::";
      const prev2 = toks[i - 2];
      const leadStart = colPrev && !(prev2 && (prev2.k === "id" || (prev2.k === "p" && prev2.s === ">")));
      if (!dotPrev && (!colPrev || leadStart)) {
        const segs = [intern(t.s)];
        const tk = [i];
        let j = i + 1;
        while (P(j, "::")) {
          if (P(j + 1, "<")) break;
          if (toks[j + 1]?.k !== "id") break;
          segs.push(intern(toks[j + 1]!.s)); tk.push(j + 1); j += 2;
        }
        if (segs.length > 1 && segs.some((x) => old.has(x))) chains.push({ segs, offs: tk.map((q) => [toks[q]!.a, toks[q]!.b] as [number, number]), lead: leadStart, mod: cur });
        i = j;
        continue;
      }
      i++;
      continue;
    }
    if (t.k === "p" && (t.s === ";" || t.s === "{" || t.s === "}")) attrs = [];
    i++;
  }
  for (const d of decls) for (const u of d.uses) { u.offs = u.toks.map((q) => [toks[q]!.a, toks[q]!.b] as [number, number]); u.toks = []; }
  return { file, size: src.length, root: rootDecl, decls, chains };
};

export type Crate = { name: string; ident: string; manifest: string; roots: { file: string; kind: string }[]; dir: string };

export const loadCrates = (): Crate[] => {
  const files = walkFiles((n) => n === "Cargo.toml");
  const crates: Crate[] = [];
  for (const f of files) {
    let t: any;
    try { t = Bun.TOML.parse(readFileSync(f, "utf8")); } catch { continue; }
    const pkg = t.package;
    if (!pkg || typeof pkg.name !== "string") continue;
    const dir = dirname(f);
    const roots: { file: string; kind: string }[] = [];
    const lib = t.lib ?? {};
    const libPath = lib.path ?? "src/lib.rs";
    if (lib.path || existsSync(join(dir, "src/lib.rs"))) roots.push({ file: resolve(dir, libPath), kind: "lib" });
    for (const k of ["bin", "test", "bench", "example"]) for (const b of t[k] ?? []) if (b.path) roots.push({ file: resolve(dir, b.path), kind: k });
    if (existsSync(join(dir, "build.rs"))) roots.push({ file: join(dir, "build.rs"), kind: "build" });
    const ident = (lib.name ?? pkg.name).replace(/-/g, "_");
    crates.push({ name: pkg.name, ident, manifest: f, roots, dir });
  }
  return crates;
};

export { basename, dirname, join, resolve, root };
