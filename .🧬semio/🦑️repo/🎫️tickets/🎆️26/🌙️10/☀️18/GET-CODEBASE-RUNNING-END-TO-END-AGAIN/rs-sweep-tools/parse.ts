import Parser from "/Users/ueli/Documents/semio/node_modules/web-tree-sitter/tree-sitter.js";
import { existsSync, readdirSync } from "node:fs";
import { createHash } from "node:crypto";
import { macroModel } from "./macros.ts";

export type Vis = { k: "priv" } | { k: "pub" } | { k: "crate" } | { k: "super" } | { k: "self" } | { k: "in"; path: string[] };
export type UseLeaf = { segs: string[]; alias: string | null; glob: boolean; self: boolean; line: number; s: number; e: number; plen: number; list: boolean };
export type Ref = { segs: string[]; line: number; kind: "path" | "macro" | "use" | "include"; bare?: boolean; cfg: string[]; leaves?: UseLeaf[]; file?: string; src?: string; at?: number };
export type Method = { name: string; cfg: string[]; line: number };
export type Item = {
  kind: "mod" | "use" | "struct" | "enum" | "union" | "trait" | "fn" | "const" | "static" | "type" | "macro" | "extern_crate" | "impl" | "macro_call";
  name: string;
  vis: Vis;
  cfg: string[];
  line: number;
  at: number;
  end: number;
  visEnd?: number;
  bodyStart?: number;
  path?: string;
  body?: Body;
  leaves?: UseLeaf[];
  variants?: string[];
  ctor?: boolean;
  macroExport?: boolean;
  proc?: { derive?: string[]; attr?: boolean; bang?: boolean };
  implKey?: string;
  implTrait?: boolean;
  methods?: Method[];
  alias?: string;
  fromMacro?: boolean;
};
export type Body = { items: Item[]; refs: Ref[]; local: Set<string>; generics: Set<string>; opaque: boolean; opaqueLines: number[] };
export type FileModel = { path: string; body: Body; cfg: string[]; errors: number[]; size: number; hash: string; useAt: number };

const PRIV: Vis = { k: "priv" };
const HARMLESS_MACROS = new Set(["compile_error", "const_assert", "const_assert_eq", "assert", "assert_eq", "debug_assert", "static_assertions", "include_str", "include_bytes", "env", "concat", "stringify", "cfg"]);
const ITEM_KINDS: Record<string, Item["kind"]> = {
  struct_item: "struct",
  enum_item: "enum",
  union_item: "union",
  trait_item: "trait",
  function_item: "fn",
  function_signature_item: "fn",
  const_item: "const",
  static_item: "static",
  type_item: "type",
};

export const BARE = process.env.RS_BARE === "1";
let parser: any = null;

/** 🌲 Loads the tree-sitter Rust grammar once from a wasm file. */
export async function initParser(wasm: string): Promise<void> {
  await Parser.init();
  const lang = await Parser.Language.load(wasm);
  parser = new Parser();
  parser.setLanguage(lang);
}

/** 🔍 Finds a tree-sitter Rust wasm in node_modules or in the tool caches without installing anything. */
export function findWasm(root: string): string | null {
  const direct = `${root}/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm`;
  if (existsSync(direct)) return direct;
  const stack = [`${root}/.🧬semio/🦑️repo/⚡️cache/tools`, `${root}/.🧬semio/🌐hub`];
  const seen: string[] = [];
  const walk = (dir: string, depth: number): string | null => {
    if (depth > 6) return null;
    let names: import("node:fs").Dirent[];
    try {
      names = readdirSync(dir, { withFileTypes: true });
    } catch {
      return null;
    }
    for (const n of names) {
      if (n.isDirectory() && n.name === "tree-sitter-wasms" && existsSync(`${dir}/${n.name}/out/tree-sitter-rust.wasm`)) return `${dir}/${n.name}/out/tree-sitter-rust.wasm`;
    }
    for (const n of names) if (n.isDirectory() && !n.name.startsWith(".") ) {
      const r = walk(`${dir}/${n.name}`, depth + 1);
      if (r) return r;
    }
    return null;
  };
  for (const s of stack) {
    seen.push(s);
    const r = walk(s, 0);
    if (r) return r;
  }
  return null;
}

function visOf(n: any): Vis {
  const t = n?.text as string | undefined;
  if (!t) return PRIV;
  const m = /^pub\s*\(\s*(crate|super|self|in\s+([\s\S]+?))\s*\)$/.exec(t);
  if (!m) return { k: "pub" };
  if (m[1] === "crate") return { k: "crate" };
  if (m[1] === "super") return { k: "super" };
  if (m[1] === "self") return { k: "self" };
  return { k: "in", path: m[2].split("::").map((s) => s.trim()) };
}

function visChild(n: any): Vis {
  for (const c of n.children) if (c.type === "visibility_modifier") return visOf(c);
  return PRIV;
}

function str(n: any): string {
  const raw = /^r#*"/.test(n.text);
  const t = n.text.replace(/^r#*"|"#*$|^"|"$/g, "");
  return raw ? t : unescapeRust(t);
}

function segsOf(n: any): string[] | null {
  if (!n) return null;
  switch (n.type) {
    case "identifier":
    case "type_identifier":
    case "self":
    case "super":
    case "crate":
    case "metavariable":
      return [n.text];
    case "scoped_identifier":
    case "scoped_type_identifier": {
      const name = n.childForFieldName("name");
      const p = n.childForFieldName("path");
      if (!name) return null;
      if (!p) return n.text.startsWith("::") ? ["::", name.text] : [name.text];
      const head = segsOf(p);
      return head ? [...head, name.text] : null;
    }
    case "generic_type":
    case "generic_type_with_turbofish":
      return segsOf(n.childForFieldName("type"));
    default:
      return null;
  }
}

function leavesOf(n: any, prefix: string[], out: UseLeaf[], inList = false): void {
  const line = n.startPosition.row + 1;
  const pos = { s: n.startIndex, e: n.endIndex, plen: prefix.length, list: inList };
  switch (n.type) {
    case "identifier":
    case "self":
    case "super":
    case "crate":
    case "scoped_identifier": {
      const s = segsOf(n) ?? [];
      const all = [...prefix, ...s];
      const isSelf = n.type === "self" && prefix.length > 0;
      out.push({ segs: isSelf ? prefix : all, alias: null, glob: false, self: isSelf, line, ...pos });
      return;
    }
    case "use_as_clause": {
      const p = n.childForFieldName("path");
      const a = n.childForFieldName("alias");
      const s = p ? segsOf(p) ?? [] : [];
      const isSelf = p?.type === "self" && prefix.length > 0;
      out.push({ segs: isSelf ? prefix : [...prefix, ...s], alias: a ? a.text : null, glob: false, self: isSelf, line, ...pos });
      return;
    }
    case "use_wildcard": {
      const p = n.namedChildren[0];
      const s = p ? segsOf(p) ?? [] : [];
      out.push({ segs: [...prefix, ...s], alias: null, glob: true, self: false, line, ...pos });
      return;
    }
    case "scoped_use_list": {
      const p = n.childForFieldName("path");
      const l = n.childForFieldName("list");
      const s = p ? segsOf(p) ?? [] : [];
      if (l) leavesOf(l, [...prefix, ...s], out, true);
      return;
    }
    case "use_list":
      for (const c of n.namedChildren) leavesOf(c, prefix, out, true);
      return;
    default:
      return;
  }
}

type Attrs = { cfg: string[]; path?: string; dynEnum?: boolean; macroExport: boolean; proc: Item["proc"] | undefined };

function attrsOf(pending: any[]): Attrs {
  const a: Attrs = { cfg: [], macroExport: false, proc: undefined };
  for (const p of pending) {
    const t: string = p.text;
    const m = /^#!?\[\s*cfg\s*\(([\s\S]*)\)\s*\]$/.exec(t);
    if (m) {
      a.cfg.push(m[1].trim());
      continue;
    }
    const pm = /^#\[\s*path\s*=\s*("(?:[^"\\]|\\.)*")\s*\]$/.exec(t);
    if (pm) {
      a.path = unescapeRust(pm[1].slice(1, -1));
      continue;
    }
    if (/^#\[\s*macro_export\b/.test(t)) a.macroExport = true;
    if (/^#\[\s*(?:[A-Za-z_][\w]*::)*dyn_enum\b/.test(t)) a.dynEnum = true;
    const d = /^#\[\s*proc_macro_derive\s*\(\s*([A-Za-z0-9_]+)/.exec(t);
    if (d) a.proc = { ...(a.proc ?? {}), derive: [d[1]] };
    if (/^#\[\s*proc_macro_attribute\s*\]/.test(t)) a.proc = { ...(a.proc ?? {}), attr: true };
    if (/^#\[\s*proc_macro\s*\]/.test(t)) a.proc = { ...(a.proc ?? {}), bang: true };
  }
  return a;
}

function unescapeRust(s: string): string {
  return s.replace(/\\u\{([0-9a-fA-F_]+)\}|\\(.)/g, (_m, hex, ch) => (hex ? String.fromCodePoint(parseInt(hex.replace(/_/g, ""), 16)) : ch === "n" ? "\n" : ch === "t" ? "\t" : ch === "r" ? "\r" : ch === "0" ? "\0" : ch));
}

function newBody(): Body {
  return { items: [], refs: [], local: new Set(), generics: new Set(), opaque: false, opaqueLines: [] };
}

function collectGenerics(n: any, body: Body): void {
  for (const c of n.namedChildren) {
    if (c.type === "type_identifier") body.generics.add(c.text);
    else if (c.type === "constrained_type_parameter" || c.type === "optional_type_parameter") {
      const l = c.childForFieldName("left") ?? c.childForFieldName("name") ?? c.namedChildren[0];
      if (l) body.generics.add(l.text);
    } else if (c.type === "const_parameter") {
      const l = c.childForFieldName("name");
      if (l) body.generics.add(l.text);
    }
  }
}

const LOCAL_ITEMS = new Set(["struct_item", "enum_item", "union_item", "trait_item", "function_item", "const_item", "static_item", "type_item", "mod_item", "macro_definition"]);

function visitRefs(n: any, body: Body, cfg: string[], prefixOf: any | null): void {
  const t = n.type;
  if (t === "scoped_identifier" || t === "scoped_type_identifier") {
    if (prefixOf === null) {
      const s = segsOf(n);
      if (s && s.length >= 2) body.refs.push({ segs: s, line: n.startPosition.row + 1, kind: "path", cfg, at: n.startIndex });
    }
    const p = n.childForFieldName("path");
    for (const c of n.children) visitRefs(c, body, cfg, c === p ? n : null);
    return;
  }
  if (t === "type_identifier" && prefixOf === null && BARE) {
    const par = n.parent;
    const decl = par && par.childForFieldName?.("name")?.id === n.id;
    const inScoped = par && (par.type === "scoped_type_identifier" || par.type === "scoped_identifier");
    if (!decl && !inScoped && n.text !== "Self" && n.text !== "_" && par?.type !== "type_parameters" && par?.type !== "constrained_type_parameter" && par?.type !== "optional_type_parameter") body.refs.push({ segs: [n.text], line: n.startPosition.row + 1, kind: "path", bare: true, cfg, at: n.startIndex });
    return;
  }
  if (t === "generic_type" && prefixOf) {
    for (const c of n.children) visitRefs(c, body, cfg, null);
    return;
  }
  if (t === "type_parameters") collectGenerics(n, body);
  if (t === "use_declaration") {
    const leaves: UseLeaf[] = [];
    const arg = n.childForFieldName("argument");
    if (arg) leavesOf(arg, [], leaves);
    body.refs.push({ segs: [], line: n.startPosition.row + 1, kind: "use", cfg, leaves });
    for (const l of leaves) {
      if (l.glob) continue;
      const last = l.alias ?? l.segs[l.segs.length - 1];
      if (last) body.local.add(last);
    }
    return;
  }
  if (LOCAL_ITEMS.has(t) && prefixOf === null) {
    const nm = n.childForFieldName("name");
    if (nm) body.local.add(nm.text);
  }
  if (t === "macro_invocation") {
    const m = n.childForFieldName("macro");
    const s = m ? segsOf(m) : null;
    if (s && s.length >= 2) body.refs.push({ segs: s, line: n.startPosition.row + 1, kind: "macro", cfg, at: m.startIndex });
    const nm = s ? s[s.length - 1] : "";
    if (nm === "include_str" || nm === "include_bytes" || nm === "include" || nm.startsWith("fixture_")) {
      const tt = n.children.find((c: any) => c.type === "token_tree");
      const lit = tt?.namedChildren.find((c: any) => c.type === "string_literal");
      if (lit) body.refs.push({ segs: [nm], line: n.startPosition.row + 1, kind: "include", cfg, file: str(lit) });
    }
    return;
  }
  if (t === "token_tree") return;
  let pending: any[] = [];
  for (const c of n.children) {
    if (c.type === "attribute_item") {
      pending.push(c);
      continue;
    }
    if (c.type === "line_comment" || c.type === "block_comment") continue;
    const a = pending.length ? attrsOf(pending).cfg : [];
    pending = [];
    visitRefs(c, body, a.length ? [...cfg, ...a] : cfg, null);
  }
}

function pushOpaque(body: Body, name: string, line: number): void {
  if (HARMLESS_MACROS.has(name)) return;
  body.opaque = true;
  body.opaqueLines.push(line);
}

function parseItems(node: any, body: Body, fileCfg: string[], onError: (line: number) => void): void {
  let pending: any[] = [];
  for (const c of node.children) {
    const t = c.type;
    if (t === "attribute_item") {
      pending.push(c);
      continue;
    }
    if (t === "line_comment" || t === "block_comment" || t === "{" || t === "}" || t === "empty_statement") continue;
    if (t === "ERROR") {
      onError(c.startPosition.row + 1);
      pending = [];
      continue;
    }
    const at = attrsOf(pending);
    pending = [];
    const line = c.startPosition.row + 1;
    const vm = c.children?.find((x: any) => x.type === "visibility_modifier");
    const base = { cfg: at.cfg, line, at: c.startIndex, end: c.endIndex, visEnd: vm ? vm.endIndex : undefined };
    const name = c.childForFieldName?.("name")?.text ?? "";
    if (t === "mod_item") {
      const bodyNode = c.childForFieldName("body");
      const it: Item = { kind: "mod", name, vis: visChild(c), ...base, path: at.path };
      if (bodyNode) {
        it.bodyStart = bodyNode.startIndex + 1;
        it.body = newBody();
        parseItems(bodyNode, it.body, [], onError);
      }
      body.items.push(it);
    } else if (t === "use_declaration") {
      const leaves: UseLeaf[] = [];
      const arg = c.childForFieldName("argument");
      if (arg) leavesOf(arg, [], leaves);
      body.items.push({ kind: "use", name: "", vis: visChild(c), ...base, leaves });
    } else if (t === "extern_crate_declaration") {
      const ids = c.children.filter((x: any) => x.type === "identifier");
      body.items.push({ kind: "extern_crate", name: ids[0]?.text ?? "", alias: ids[1]?.text, vis: visChild(c), ...base });
      if (at.macroExport) body.items[body.items.length - 1].macroExport = true;
    } else if (t in ITEM_KINDS) {
      const kind = ITEM_KINDS[t];
      const it: Item = { kind, name, vis: visChild(c), ...base };
      if (kind === "struct") {
        it.ctor = c.children.some((x: any) => x.type === "ordered_field_declaration_list") || !c.children.some((x: any) => x.type === "field_declaration_list");
      }
      if (kind === "enum") {
        const vl = c.childForFieldName("body");
        it.variants = vl ? vl.namedChildren.filter((x: any) => x.type === "enum_variant").map((x: any) => x.childForFieldName("name")?.text ?? "") : [];
      }
      if (at.proc) it.proc = at.proc;
      if (kind === "trait" && at.dynEnum) body.items.push({ kind: "macro", name: `__semio_dispatch_${name}`, vis: PRIV, ...base, macroExport: true, fromMacro: true });
      const sub = body;
      collectGenerics(c.childForFieldName("type_parameters") ?? { namedChildren: [] }, sub);
      for (const x of c.children) {
        if (x.type === "visibility_modifier" || x.type === "identifier" || x.type === "type_identifier") continue;
        visitRefs(x, body, at.cfg, null);
      }
      body.items.push(it);
    } else if (t === "macro_definition") {
      const it: Item = { kind: "macro", name, vis: PRIV, ...base, macroExport: at.macroExport };
      body.items.push(it);
    } else if (t === "impl_item") {
      const ty = c.childForFieldName("type");
      const tr = c.childForFieldName("trait");
      const key = (ty?.text ?? "").replace(/\s+/g, "");
      const trKey = tr ? tr.text.replace(/\s+/g, "") : "";
      const it: Item = { kind: "impl", name: "", vis: PRIV, ...base, implKey: key, implTrait: !!tr, methods: [] };
      if (tr) it.name = trKey;
      const dl = c.childForFieldName("body");
      if (dl) {
        let mp: any[] = [];
        for (const m of dl.children) {
          if (m.type === "attribute_item") {
            mp.push(m);
            continue;
          }
          const ma = attrsOf(mp);
          mp = [];
          if (m.type === "function_item" || m.type === "const_item" || m.type === "type_item") {
            const mn = m.childForFieldName("name");
            if (mn) it.methods!.push({ name: mn.text, cfg: [...at.cfg, ...ma.cfg], line: m.startPosition.row + 1 });
          }
        }
      }
      collectGenerics(c.childForFieldName("type_parameters") ?? { namedChildren: [] }, body);
      for (const x of c.children) visitRefs(x, body, at.cfg, null);
      body.items.push(it);
    } else if (t === "foreign_mod_item") {
      const dl = c.children.find((x: any) => x.type === "declaration_list");
      if (dl) {
        let fp: any[] = [];
        for (const f of dl.children) {
          if (f.type === "attribute_item") {
            fp.push(f);
            continue;
          }
          const fa = attrsOf(fp);
          fp = [];
          const k = f.type === "function_signature_item" ? "fn" : f.type === "static_item" ? "static" : null;
          const fnm = f.childForFieldName?.("name");
          if (k && fnm) body.items.push({ kind: k as Item["kind"], name: fnm.text, vis: visChild(f), cfg: [...at.cfg, ...fa.cfg], line: f.startPosition.row + 1, at: f.startIndex, end: f.endIndex });
        }
      }
    } else if (t === "macro_invocation" || t === "expression_statement") {
      const mi = t === "macro_invocation" ? c : c.namedChildren.find((x: any) => x.type === "macro_invocation");
      if (!mi) {
        visitRefs(c, body, at.cfg, null);
        continue;
      }
      const ms = segsOf(mi.childForFieldName("macro")) ?? [];
      const mn = ms[ms.length - 1] ?? "";
      if (mn === "include") {
        const tt = mi.children.find((x: any) => x.type === "token_tree");
        const lit = tt?.namedChildren.find((x: any) => x.type === "string_literal");
        if (lit) body.items.push({ kind: "macro_call", name: "include", vis: PRIV, ...base, path: str(lit) });
        else pushOpaque(body, mn, line);
      } else {
        const ttm = mi.children.find((x: any) => x.type === "token_tree");
        const model = ttm ? macroModel(mn, ttm.text, at.cfg, line, c.startIndex, c.endIndex) : null;
        if (model) body.items.push(...model);
        else pushOpaque(body, mn, line);
        if (ms.length >= 2) body.refs.push({ segs: ms, line, kind: "macro", cfg: at.cfg });
        const tt = mi.children.find((x: any) => x.type === "token_tree");
        if (!model && tt && !HARMLESS_MACROS.has(mn)) {
          const re = /(?:(pub(?:\([^)]*\))?)\s+)?\b(enum|struct|union|type|fn|const|static|trait|mod)\s+([A-Za-z_][A-Za-z0-9_]*)/g;
          let mm: RegExpExecArray | null;
          const text: string = tt.text;
          while ((mm = re.exec(text))) {
            const kind = mm[2] === "enum" ? "enum" : (mm[2] as Item["kind"]);
            const it: Item = { kind, name: mm[3], vis: mm[1] ? visOf({ text: mm[1] }) : PRIV, cfg: at.cfg, line, at: c.startIndex, end: c.endIndex, variants: kind === "enum" ? [] : undefined, ctor: kind === "struct" ? true : undefined };
            it.fromMacro = true;
            body.items.push(it);
          }
        }
      }
    } else if (t === "inner_attribute_item" || t === "shebang") {
      const m = /^#!\[\s*cfg\s*\(([\s\S]*)\)\s*\]$/.exec(c.text);
      if (m) fileCfg.push(m[1].trim());
    } else if (t === "ERROR") {
      onError(line);
    } else {
      visitRefs(c, body, at.cfg, null);
    }
  }
}

/** 📄 Parses one Rust file into its item skeleton, path references and macro opacity. */
export function parseFile(path: string, src: string): FileModel {
  const tree = parser.parse(src);
  const body = newBody();
  const cfg: string[] = [];
  const errors: number[] = [];
  parseItems(tree.rootNode, body, cfg, (l) => errors.push(l));
  if (tree.rootNode.hasError() && errors.length === 0) errors.push(0);
  let useAt = -1;
  let first = -1;
  for (const c of tree.rootNode.children) {
    if (c.type === "use_declaration") useAt = c.endIndex;
    if (first < 0 && !["line_comment", "block_comment", "shebang", "inner_attribute_item"].includes(c.type)) first = c.startIndex;
  }
  if (useAt < 0) useAt = first < 0 ? src.length : first;
  else {
    const nl = src.indexOf("\n", useAt);
    useAt = nl < 0 ? src.length : nl + 1;
  }
  tree.delete();
  return { path, body, cfg, errors, size: src.length, hash: createHash("sha1").update(src).digest("hex"), useAt };
}
