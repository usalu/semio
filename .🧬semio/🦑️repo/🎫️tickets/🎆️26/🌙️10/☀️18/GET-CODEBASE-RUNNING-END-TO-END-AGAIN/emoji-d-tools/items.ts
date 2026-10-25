import { readFileSync, writeFileSync } from "node:fs";
import { docstringOpenersOfText, docstringEmojiReuseOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const [listPath, outPath] = process.argv.slice(2);
const paths = readFileSync(listPath, "utf8").split("\n").filter(Boolean);
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const MARK = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)/u;
export function declAfter(lines: string[], idx: number, isRs: boolean): string {
  let i = idx;
  const isDoc = (l: string) => /^\s*(?:\/\/[\/!]|\/\*\*|\*)/.test(l) || l.trim() === "" ;
  let inBlock = /\/\*\*/.test(lines[i]) && !/\*\//.test(lines[i]);
  for (let j = i; j < lines.length; j++) {
    const l = lines[j];
    if (inBlock) { if (/\*\//.test(l)) inBlock = false; continue; }
    if (j === i && /\*\//.test(l)) continue;
    if (/^\s*\/\/[\/!]/.test(l) || /^\s*#\[/.test(l) || /^\s*@/.test(l) || l.trim() === "" || /^\s*\/\*\*/.test(l) || /^\s*\/\//.test(l)) continue;
    if (/^\s*\*/.test(l)) continue;
    return l.trim().slice(0, 160);
  }
  return "";
}
const items: any[] = [];
for (const p of paths) {
  let t: string;
  try { t = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8"); } catch { continue; }
  const groups = docstringEmojiReuseOfText(p, t);
  if (!groups.length) continue;
  const lines = t.split("\n");
  const openers = docstringOpenersOfText(p, t);
  const info = new Map<number, string>();
  for (const o of openers) info.set(o.line, o.content);
  const all = openers.filter((o) => MARK.test(o.content)).map((o) => ({ line: o.line, emoji: seg.segment(o.content)[Symbol.iterator]().next().value!.segment.replaceAll("️", ""), text: o.content }));
  items.push({ path: p, all: all.map((o) => ({ ...o, decl: declAfter(lines, o.line - 1, p.endsWith(".rs")) })), dup: groups.map((g) => g.emoji) });
}
writeFileSync(outPath, JSON.stringify(items));
console.log(items.length);
