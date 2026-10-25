import { readFileSync, writeFileSync } from "node:fs";
import { LEXICON } from "./lexicon.ts";
const [itemsPath, outPath, overridesPath] = process.argv.slice(2);
const names: Record<string, string> = JSON.parse(readFileSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/3d0a4ac3-5d70-4c6b-91df-262fe6307e4a/scratchpad/names.json", "utf8"));
const overrides: Record<string, string> = overridesPath ? JSON.parse(readFileSync(overridesPath, "utf8")) : {};
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const norm = (s: string) => s.replaceAll("️", "");
const STOP = new Set("the a an of to in for and or is are as with by on at from that this it its be one every each all any not into than then when which who whose their they them if so can may must will only also per out up over under via vs fn pub self let const use type struct enum impl export function async return true false null undefined string number boolean void readonly static mut dyn crate super trait mod class interface default new has have does was were been being how what where while within without about after before between both either other same such very more most less own like just using used uses use".split(" "));
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

const lex = new Map<string, Map<string, number>>();
const order: string[] = [];
for (const [rawKey, kw] of Object.entries(LEXICON)) {
  const key = norm(rawKey);
  const first = seg.segment(key)[Symbol.iterator]().next().value!.segment;
  if (first !== key || !/\p{Extended_Pictographic}|\p{So}/u.test(key)) continue;
  const m = lex.get(key) ?? new Map<string, number>();
  if (!lex.has(key)) order.push(key);
  kw.split(" ").forEach((w, i) => { const s = stem(w); if (!m.has(s)) m.set(s, i < 3 ? 1.5 : 1); });
  lex.set(key, m);
}
const BAD_NAME = /FACE|PERSON|MAN\b|WOMAN|HAND|FINGER|FLAG|HEAD|SMIL|BOY|GIRL|BABY|KISS|HEART WITH|SKIN|CAT FACE|MONKEY FACE|EYE\b|NOSE|MOUTH|TONGUE|FOOT\b|LEG\b|BICEP|SANTA|ANGEL|PRINCE|BRIDE|DANCER|RUNNER|SURFER|SWIMMER|CLAP|PRAY|WAVING|MUSCLE|SKULL|BOMB|PISTOL|KNIFE|DAGGER|SYRINGE|CIGARETTE|COFFIN|BEER|WINE|COCKTAIL|SAKE|TOILET|PILE OF|KEYCAP|REGIONAL|VARIATION|TAG |COMBINING|ZERO WIDTH|PLAYING CARD|MAHJONG TILE|DOMINO|CJK|SQUARED|CIRCLED|PARENTHESIZED|ENCLOSED|NEGATIVE/;
const extra = new Map<string, Set<string>>();
for (const [ch, name] of Object.entries(names)) {
  const cp = ch.codePointAt(0)!;
  if (!/\p{Emoji}/u.test(ch) || !/\p{Extended_Pictographic}/u.test(ch) || cp < 0x2190 || BAD_NAME.test(name)) continue;
  if (cp >= 0x1f600 && cp <= 0x1f64f) continue;
  const key = norm(ch);
  extra.set(key, new Set(name.split(" ").filter((w) => w.length > 2 && !STOP.has(w.toLowerCase())).map(stem)));
}
const poolOrder = [...order, ...[...extra.keys()].filter((k) => !lex.has(k))];
const kwOf = (e: string) => lex.get(e);

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
  const k = kwOf(e);
  const ex = extra.get(e);
  for (const [w, wt] of toks) {
    if (k?.has(w)) s += wt * k.get(w)!;
    else if (ex?.has(w)) s += wt * 0.6;
  }
  return s;
}
const KIND_POOL: Record<string, string[]> = {
  type: ["🏷", "🧱", "🧰", "🔖", "🧬"],
  fn: ["⚙", "🔧", "🛠", "🔩", "🧲"],
  const: ["📌", "🗿", "🧷", "📍", "⚓"],
  test: ["🧪", "🔬", "🩺", "✅", "🧫"],
};
const kindOf = (d: string) => /^(?:export\s+)?(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|type|interface|class)\b/.test(d) ? "type" : /\b(?:fn|function)\b/.test(d) ? "fn" : /\b(?:const|static|let)\b/.test(d) ? "const" : "fn";

const files: { path: string; all: Item[]; dup: string[] }[] = JSON.parse(readFileSync(itemsPath, "utf8"));
const out: any[] = [];
const stats = { total: 0, zero: 0, low: 0 };
for (const f of files) {
  const dupSet = new Set(f.dup);
  const used = new Set(f.all.map((o) => o.emoji));
  const toChange: { it: Item; toks: Map<string, number> }[] = [];
  for (const g of dupSet) {
    const members = f.all.filter((o) => o.emoji === g);
    let keep = members[0]!;
    let best = -1;
    for (const m of members) {
      const s = score(g, tokens(m)) + (overrides[`${f.path}:${m.line}`] ? -1e6 : 0);
      if (s > best) { best = s; keep = m; }
    }
    for (const m of members) if (m !== keep) toChange.push({ it: m, toks: tokens(m) });
  }
  toChange.sort((a, b) => a.it.line - b.it.line);
  for (const { it, toks } of toChange) {
    stats.total++;
    const ov = overrides[`${f.path}:${it.line}`];
    let pick: string | undefined;
    let sc = 0;
    if (ov) { pick = norm(ov); sc = 99; }
    else {
      for (const e of poolOrder) {
        if (used.has(e)) continue;
        const s = score(e, toks);
        if (s > sc) { sc = s; pick = e; }
      }
      if (!pick) {
        const pool = KIND_POOL[kindOf(it.decl)]!;
        pick = pool.find((e) => !used.has(e)) ?? poolOrder.find((e) => !used.has(e))!;
        stats.zero++;
      } else if (sc < 3) stats.low++;
    }
    used.add(pick!);
    out.push({ path: f.path, line: it.line, old: it.emoji, new: pick, score: sc, text: it.text.slice(0, 110), decl: it.decl.slice(0, 80) });
  }
}
writeFileSync(outPath, JSON.stringify(out));
console.log(stats);
