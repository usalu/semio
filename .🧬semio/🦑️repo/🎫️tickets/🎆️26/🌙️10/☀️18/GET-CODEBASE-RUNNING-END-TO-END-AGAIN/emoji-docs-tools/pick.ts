import { CONCEPTS as BASE } from "./map.ts";
import { EXTRA } from "./map-extra.ts";
import { NOUNS } from "./map-nouns.ts";
const CONCEPTS = [...NOUNS, ...BASE, ...EXTRA];

const FE0F = "️";
export const norm = (e: string) => e.replaceAll(FE0F, "");
export const dress = (e: string) => norm(e) + FE0F;
const PICT = /^\p{Extended_Pictographic}$/u;

const STOP = new Set("the and for with that this from into onto over under are was were has have had its their they them can may must not nor but all any per via out off one two own new old get set let use used uses using when while where which what who whose how why than then also only each every both either neither some such same other another more most less least very just can will would should could shall might does did done doing being been".split(" "));

type Concept = { words: string[]; emojis: string[] };
const concepts: Concept[] = CONCEPTS.map(([w, e]) => ({
  words: w.split(" ").filter((x) => x && !x.endsWith("_")),
  emojis: [...new Set(e.split(" ").map(norm).filter((x) => x && !x.includes("‍") && !x.includes("⃣") && [...x].length === 1 && PICT.test(x)))],
}));
const wordMap = new Map<string, Concept[]>();
for (const c of concepts) for (const w of c.words) { const l = wordMap.get(w) ?? []; l.push(c); wordMap.set(w, l); }

export function stems(w: string): string[] {
  const out = [w];
  if (w.endsWith("ies")) out.push(w.slice(0, -3) + "y");
  if (w.endsWith("es")) out.push(w.slice(0, -2));
  if (w.endsWith("s") && !w.endsWith("ss")) out.push(w.slice(0, -1));
  if (w.endsWith("ing")) { out.push(w.slice(0, -3)); out.push(w.slice(0, -3) + "e"); }
  if (w.endsWith("ed")) { out.push(w.slice(0, -2)); out.push(w.slice(0, -1)); }
  if (w.endsWith("er")) out.push(w.slice(0, -2));
  if (w.endsWith("ly")) out.push(w.slice(0, -2));
  if (w.endsWith("ion")) out.push(w.slice(0, -3));
  return out;
}

export function words(text: string): string[] {
  return text
    .replace(/[`'"]/g, " ")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2")
    .toLowerCase()
    .split(/[^a-z]+/)
    .filter((w) => w.length >= 2 && !STOP.has(w));
}

export function lookup(word: string): Concept[] | undefined {
  for (const s of stems(word)) { const c = wordMap.get(s); if (c) return c; }
  return undefined;
}

const NEUTRAL = "🔹 🔸 🔷 🔶 🔵 🟠 🟡 🟢 🟣 🟤 💠 🔘 🔳 🔲 🟥 🟧 🟨 🟩 🟦 🟪 🟫 ⬛ ⬜ ⚪ ⚫ 🔴 📍 📌 📎 🔖 🧷 🪧 🧩 🎲 🎯 🧿 🔮 🧭 🗺 🪙 💎 🧱 🪨 🧲 🧰 🔧 🔩 ⚙ 🪛 🔨 ⚒ 🛠 🗜 ⚖ 🔗 ⛓ 🧵 🪢 🧶 🪡 📐 📏 📒 📓 📔 📕 📗 📘 📙 📚 📖 🔭 🔬 💡 🕯 🪔 🔦 🏮 🎈 🎏 🎐 🪁 🪀 🪅 🧸 🎁 🏺 🪞 🛡 ⚓ ⛵ 🚩 🏁 🎌 🏴 🏳 ⭐ 🌟 ✨ ⚡ 🔥 💧 🌊 🌈 ☀ 🌙 ☁ ❄ 🍀 🌱 🌿 🍃 🍂 🌳 🌲 🌵 🪵 🍄 🐚 🪸 🦋 🐝 🐞 🐜 🕷 🦉 🦊 🐙 🦑 🐋 🐬 🐢 🦎 🐍 🦀 🐠 🪼".split(" ").map(norm).filter((x) => x && PICT.test(x) && [...x].length === 1);

export function hash(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619) >>> 0; }
  return h >>> 0;
}

export type Ctx = { name: string; head: string; moduleDir: string; moduleLevel: boolean; tag: string };

const GENERIC = new Set("type types kind kinds value values field fields model models schema state data info item items entry entries list set map record records object objects id ids name names key keys count text string number input output result results config option options default new add make get use used register change changes mode modes status source target targets artifact artifacts snapshot snapshots derived state states current impl function fn struct pub const crate semio standards subsets any api base common core one per via rep".split(" "));

export function candidates(ctx: Ctx): { list: string[]; hit: boolean } {
  const seen = new Set<string>();
  const list: string[] = [];
  const add = (cs: Concept[] | undefined) => { if (!cs) return; for (const c of cs) for (const e of c.emojis) if (!seen.has(e)) { seen.add(e); list.push(e); } };
  const nameW = words(ctx.name);
  const dirW = words(ctx.moduleDir);
  const prose = ctx.head.replace(/`[^`]*`/g, " ");
  const headW = ctx.tag ? [] : words(prose.slice(0, 160)).slice(0, 16);
  const codeW = ctx.tag ? [] : words((ctx.head.match(/`[^`]*`/g) ?? []).join(" ")).slice(0, 6);
  const groups = ctx.moduleLevel ? [headW, dirW, nameW, codeW] : [nameW, headW, codeW, dirW.slice(-2)];
  const done = new Set<string>();
  for (const generic of [false, true]) for (const ws of groups) for (const w of ws) {
    if (GENERIC.has(w) !== generic) continue;
    if (done.has(w)) continue;
    const cs = lookup(w);
    if (!cs) continue;
    done.add(w);
    add(cs);
  }
  return { list, hit: list.length > 0 };
}

export function choose(ctx: Ctx, used: Set<string>, moduleUsed: Set<string>): { emoji: string; how: string } {
  const { list, hit } = candidates(ctx);
  for (let i = 0; i < Math.min(3, list.length); i++) if (!used.has(list[i]) && !moduleUsed.has(list[i])) return { emoji: list[i], how: "dict" };
  for (const e of list) if (!used.has(e)) return { emoji: e, how: "dict" };
  const seed = hash(ctx.name + "|" + ctx.head);
  for (let i = 0; i < NEUTRAL.length; i++) { const e = NEUTRAL[(seed + i) % NEUTRAL.length]; if (!used.has(e) && !moduleUsed.has(e)) return { emoji: e, how: hit ? "dict-exhausted" : "pool" }; }
  for (let i = 0; i < NEUTRAL.length; i++) { const e = NEUTRAL[(seed + i) % NEUTRAL.length]; if (!used.has(e)) return { emoji: e, how: hit ? "dict-exhausted" : "pool" }; }
  return { emoji: NEUTRAL[seed % NEUTRAL.length], how: "collision" };
}

export function declName(decl: string): string {
  let d = decl.replace(/#\[[^\]]*\]\s*/g, "").trim();
  d = d.replace(/^(?:pub(?:\([^)]*\))?\s+)/, "").replace(/^export\s+(?:default\s+)?/, "").replace(/^(?:declare|abstract|async|unsafe|extern\s+"[^"]*"|const|static|readonly|public|private|protected|override)\s+/g, "");
  d = d.replace(/^(?:async|unsafe|const|readonly|static)\s+/g, "");
  let m = d.match(/^(?:fn|struct|enum|trait|type|mod|union|macro_rules!|function\*?|class|interface|namespace|let|var|const|static)\s+([A-Za-z_$][\w$]*)/);
  if (m) return m[1];
  m = d.match(/^impl(?:<[^>]*>)?\s+(?:[\w:<>,\s&']+?\s+for\s+)?([A-Za-z_][\w:]*)/);
  if (m) return m[1];
  if (/^(?:use|import|extern|from|export\s*\{|export\s*\*|\}|\)|\])/.test(d)) return "";
  m = d.match(/^["']?([A-Za-z_$][\w$]*)["']?\s*[?!]?\s*[:(<,{=;]/);
  if (m) return m[1];
  m = d.match(/^([A-Za-z_$][\w$]*)\s*$/);
  if (m) return m[1];
  m = d.match(/^["']?([A-Za-z_$][\w$]*)/);
  return m ? m[1] : "";
}
