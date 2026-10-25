import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { LEXICON } from "./lexicon.ts";
const [itemsPath, outDir] = process.argv.slice(2);
const names: Record<string, string> = JSON.parse(readFileSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/3d0a4ac3-5d70-4c6b-91df-262fe6307e4a/scratchpad/names.json", "utf8"));
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const norm = (s: string) => s.replaceAll("️", "");
const STOP = new Set("the a an of to in for and or is are as with by on at from that this it its be one every each all any not into than then when which who whose their they them if so can may must will only also per out up over under via vs fn pub self let const use type struct enum impl export function async return true false null undefined string number boolean void readonly static mut dyn crate super trait mod class interface default new has have does was were been being how what where while within without about after before between both either other same such very more most less own like just using used uses".split(" "));
function stem(w: string): string {
  w = w.toLowerCase();
  if (w.length > 4 && w.endsWith("ies")) return w.slice(0, -3) + "y";
  if (w.length > 5 && w.endsWith("ing")) return w.slice(0, -3);
  if (w.length > 4 && w.endsWith("ed")) return w.slice(0, -2);
  if (w.length > 4 && w.endsWith("es")) return w.slice(0, -2);
  if (w.length > 3 && w.endsWith("s") && !w.endsWith("ss")) return w.slice(0, -1);
  return w;
}
const words = (s: string) => s.replace(/`[^`]*`/g, (m) => " " + m.slice(1, -1) + " ").replace(/\{@link\s+([^}]*)\}/g, " $1 ").replace(/\[\[?([^\]]*)\]?\]/g, " $1 ").replace(/([a-z0-9])([A-Z])/g, "$1 $2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2").replace(/[^A-Za-z]+/g, " ").split(" ").filter((w) => w.length > 2 && !STOP.has(w.toLowerCase())).map(stem);
const DROP = new Set(["®", "©", "™", "#", "*", "🥈", "🥉", "🥇", "👪", "🫂", "🧘", "🏳", "🏴", "🎌", "🙈", "❤", "🧡", "💛", "💚", "💙", "💜", "🖤", "🤍", "💔", "🚬", "🍺", "🍷", "🥂", "🍾", "🔫", "🗡", "💀", "☠", "⚰", "🚽", "🛁", "🚿", "🪒", "🧴", "🧻", "🎅", "🧙", "🧚", "🧛", "🧜", "🧞", "🧟", "🏳️‍🌈", "♈", "⛎", "♀", "♂", "⚧", "🕉", "☸", "✝", "☪", "🕎", "🔯", "🎽", "🧸", "🪀", "🪁", "🎃", "🎄", "🎎", "🎏", "🎐", "🎑", "🎍", "🧧", "🪅", "🎠", "🎡", "🎢", "🎰", "🎱", "⚽", "⚾", "🏀", "🏈", "🎾", "🏐", "🏉", "🎳", "🏸", "🥊", "🥋", "⛸", "🛶", "🤿", "🛹", "🎿", "🛷", "🥌", "🔞", "💦", "♠", "♥", "♦", "♣", "🅰", "🆎", "🅱", "🅾", "Ⓜ", "🔟"]);
const lex = new Map<string, Map<string, number>>();
const order: string[] = [];
for (const [rawKey, kw] of Object.entries(LEXICON)) {
  const key = norm(rawKey);
  if (DROP.has(key)) continue;
  const first = seg.segment(key)[Symbol.iterator]().next().value!.segment;
  if (first !== key || !/\p{Extended_Pictographic}|\p{So}/u.test(key)) continue;
  const m = lex.get(key) ?? new Map<string, number>();
  if (!lex.has(key)) order.push(key);
  kw.split(" ").forEach((w, i) => { const s = stem(w); if (!m.has(s)) m.set(s, i < 3 ? 1.5 : 1); });
  lex.set(key, m);
}
type Item = { line: number; emoji: string; text: string; decl: string };
function tokens(it: Item): Map<string, number> {
  const t = new Map<string, number>();
  const add = (ws: string[], w: number) => ws.forEach((x) => t.set(x, Math.max(t.get(x) ?? 0, w)));
  const declPart = it.decl.replace(/^(?:export\s+|pub(?:\([^)]*\))?\s+|async\s+|default\s+|declare\s+|unsafe\s+|const\s+(?=fn))+/, "");
  const nameMatch = /^(?:function\*?|fn|struct|enum|trait|type|interface|class|const|let|var|mod|impl(?:<[^>]*>)?|static|abstract class|protected|private|public|readonly)\s+([A-Za-z0-9_$]+)/.exec(declPart) ?? /^([A-Za-z0-9_$]+)\s*[:(=]/.exec(declPart);
  if (nameMatch) add(words(nameMatch[1]!), 3);
  const quoted = /(?:describe|test|it)\w*\(\s*["'`]([^"'`]*)/.exec(declPart);
  if (quoted) add(words(quoted[1]!), 2.5);
  const first = it.text.split(/(?<=[.:;])\s|\s[—–-]\s|\n/)[0] ?? it.text;
  const fw = words(first);
  add(fw.slice(0, 6), 2);
  add(fw.slice(6), 1);
  return t;
}
function score(e: string, toks: Map<string, number>): number {
  let s = 0;
  const k = lex.get(e);
  for (const [w, wt] of toks) if (k?.has(w)) s += wt * k.get(w)!;
  return s;
}
const nameOf = (d: string) => {
  const m = /(?:function\*?|fn|struct|enum|trait|type|interface|class|const|let|var|mod|static)\s+([A-Za-z0-9_$]+)/.exec(d) ?? /^(?:export\s+|pub\s+)*([A-Za-z0-9_$]+)\s*[:(=]/.exec(d);
  return m ? m[1]! : d.slice(0, 30);
};
const files: { path: string; all: Item[]; dup: string[] }[] = JSON.parse(readFileSync(itemsPath, "utf8"));
const out: any[] = [];
for (const f of files) {
  const dupSet = new Set(f.dup);
  const fixed = new Set(f.all.map((o) => o.emoji));
  for (const g of dupSet) {
    const members = f.all.filter((o) => o.emoji === g);
    let keep = members[0]!, best = -1;
    for (const m of members) { const s = score(g, tokens(m)); if (s > best) { best = s; keep = m; } }
    for (const m of members) {
      if (m === keep) continue;
      const toks = tokens(m);
      const ranked = order.filter((e) => !fixed.has(e)).map((e) => [e, score(e, toks)] as const).filter(([, s]) => s > 0).sort((a, b) => b[1] - a[1]).slice(0, 4).map(([e]) => e);
      out.push({ path: f.path, line: m.line, old: m.emoji, text: m.text, decl: nameOf(m.decl), cands: ranked });
    }
  }
}
out.forEach((o, i) => (o.id = i));
mkdirSync(outDir!, { recursive: true });
writeFileSync(`${outDir}/cands.json`, JSON.stringify(out));
const per = 220;
const fixedOf = new Map(files.map((f) => [f.path, [...new Set(f.all.map((o) => o.emoji))].join("")]));
for (let c = 0; c * per < out.length; c++) {
  const rows: string[] = [];
  let last = "";
  for (const o of out.slice(c * per, (c + 1) * per)) {
    if (o.path !== last) { rows.push(`## ${o.path.split("/").slice(-3).join("/")} used:${fixedOf.get(o.path)}`); last = o.path; }
    rows.push(`${o.id}|${o.old}|${o.cands.join("")}|${o.decl.slice(0, 28)}|${o.text.replace(/^\S+\s*/, "").slice(0, 92)}`);
  }
  writeFileSync(`${outDir}/review-${String(c).padStart(2, "0")}.txt`, rows.join("\n") + "\n");
}
console.log(out.length, out.filter((o) => o.cands.length === 0).length, "no candidates");
