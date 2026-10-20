import type { Item, Vis } from "./parse.ts";

const PUB: Vis = { k: "pub" };
const PRIV: Vis = { k: "priv" };

function pascal(s: string): string {
  return s.split("_").map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("");
}

function topNames(block: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let cur = "";
  for (const ch of block) {
    if (ch === "{" || ch === "(" || ch === "[") depth++;
    if (ch === "}" || ch === ")" || ch === "]") depth--;
    if (depth === 0 && ch === ",") {
      const m = /^\s*([A-Za-z_]\w*)/.exec(cur);
      if (m) out.push(m[1]);
      cur = "";
    } else cur += ch;
  }
  const m = /^\s*([A-Za-z_]\w*)/.exec(cur);
  if (m) out.push(m[1]);
  return out;
}

function braced(text: string, from: number): string {
  const s = text.indexOf("{", from);
  if (s < 0) return "";
  let d = 0;
  for (let i = s; i < text.length; i++) {
    if (text[i] === "{") d++;
    else if (text[i] === "}" && --d === 0) return text.slice(s + 1, i);
  }
  return text.slice(s + 1);
}

/** 🪄 Static models of item-defining macros that are not `macro_rules!` of the same crate: the names they bring into the enclosing module. */
export function macroModel(mn: string, text: string, cfg: string[], line: number, at: number, end: number): Item[] | null {
  if (mn !== "statechart") return null;
  const head = /\bmachine\s+([A-Za-z_]\w*)\s*\{/.exec(text);
  if (!head) return null;
  const name = head[1];
  const inner = text.slice(head.index + head[0].length);
  const ev = /\bevent\s+([A-Za-z_]\w*)\s*\{/.exec(inner);
  const eventName = ev ? ev[1] : "Event";
  const variants = ev ? topNames(braced(inner, ev.index)) : [];
  const states = [...inner.matchAll(/\b(?:state|final|parallel|history)\s+([A-Za-z_]\w*)/g)].map((m) => m[1].toUpperCase());
  const base = { cfg, line, at, end };
  const item = (kind: Item["kind"], n: string, extra: Partial<Item> = {}): Item => ({ kind, name: n, vis: PUB, ...base, fromMacro: true, ...extra });
  const glob = (segs: string[]): Item => ({ kind: "use", name: "", vis: PRIV, ...base, leaves: [{ segs, alias: null, glob: true, self: false, line, s: at, e: end, plen: segs.length, list: false }] });
  const empty = () => ({ refs: [], local: new Set<string>(), generics: new Set<string>(), opaque: false, opaqueLines: [] as number[] });
  const statesMod = item("mod", "states", { body: { items: [glob(["super", "super"]), ...states.map((s) => item("const", s))], ...empty() } });
  const body = { items: [glob(["super"]), item("enum", eventName, { variants }), item("struct", pascal(name), { ctor: true }), statesMod, item("type", "Snapshot"), item("const", "MANIFEST_JSON")], ...empty() };
  return [item("mod", name, { body })];
}
