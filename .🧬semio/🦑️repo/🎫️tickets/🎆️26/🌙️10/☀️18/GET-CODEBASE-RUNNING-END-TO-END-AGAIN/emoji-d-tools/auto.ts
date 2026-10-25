import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { docstringOpenersOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const [candsPath, picksDir, fromS, toS, outPath] = process.argv.slice(2);
const from = Number(fromS), to = Number(toS);
const ROOT = "/Users/ueli/Documents/semio/";
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const norm = (s: string) => s.replaceAll("️", "");
const MARK = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator})/u;
const first = (s: string) => seg.segment(s)[Symbol.iterator]().next().value!.segment;
const cands: any[] = JSON.parse(readFileSync(candsPath!, "utf8"));
const fixed = new Map<number, string>();
for (const f of readdirSync(picksDir!).filter((f) => f.endsWith(".txt") && !f.startsWith("10-auto")).sort()) for (const l of readFileSync(`${picksDir}/${f}`, "utf8").split("\n")) {
  const m = /^(\d+)\s+(\S+)/.exec(l); if (m) fixed.set(Number(m[1]), norm(m[2]!));
}
const byPath = new Map<string, any[]>();
for (const c of cands) (byPath.get(c.path) ?? byPath.set(c.path, []).get(c.path)!).push(c);
const DENY = new Set([..."🥪🍱🧃🦄🏓🥢🔊⛏🧽🍴🗿🪆🎗🪧🥀🪤🎲♟🔴🍂🍁🍃🐙💯🎪🏢🏙🏬🔕🛍🔡🆙⤵🧶🔋🏝⛩"]);
const out: string[] = [], needs: string[] = [];
for (const [path, list] of byPath) {
  if (!list.some((c) => c.id >= from && c.id <= to)) continue;
  const text = readFileSync(ROOT + path, "utf8");
  const openers = docstringOpenersOfText(path, text);
  const lines = new Set(list.map((c) => c.line));
  const used = new Set<string>();
  for (const o of openers) if (MARK.test(o.content) && !lines.has(o.line)) used.add(norm(first(o.content)));
  for (const c of list) { const p = fixed.get(c.id); if (p) used.add(p); }
  for (const c of list) {
    if (fixed.has(c.id) || c.id < from || c.id > to) continue;
    const pick = (c.cands as string[]).map(norm).find((e) => !DENY.has(e) && !used.has(e) && MARK.test(e) && first(e) === e);
    if (!pick) { needs.push(`${c.id}|${c.decl}|used:${[...used].join("")}|${c.text.slice(0, 110)}`); continue; }
    used.add(pick); out.push(`${c.id} ${pick}`);
  }
}
writeFileSync(outPath!, out.sort((a, b) => Number(a.split(" ")[0]) - Number(b.split(" ")[0])).join("\n") + "\n");
console.log(out.length, "auto;", needs.length, "need manual");
writeFileSync(outPath! + ".needs", needs.join("\n") + "\n");
