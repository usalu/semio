import { readFileSync, writeFileSync } from "node:fs";
export const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
export const fileOf = (cat: string, slug: string): string => `${ROOT}/${cat}/${slug}/🔣️.json`;
const matchEnd = (s: string, open: number): number => {
  const o = s[open]!, c = o === "[" ? "]" : "}";
  let depth = 0, str = false;
  for (let i = open; i < s.length; i++) {
    const ch = s[i]!;
    if (str) { if (ch === "\\") i++; else if (ch === '"') str = false; continue; }
    if (ch === '"') str = true;
    else if (ch === o) depth++;
    else if (ch === c && --depth === 0) return i;
  }
  throw new Error("unbalanced");
};
const rewrite = (text: string, from: number, keyRe: RegExp, edit: (value: any) => any, unit: number): string => {
  const m = keyRe.exec(text.slice(from));
  if (!m) throw new Error(`key ${keyRe} not found`);
  const open = from + m.index + m[0].length - 1;
  const end = matchEnd(text, open);
  const value = edit(JSON.parse(text.slice(open, end + 1)));
  const body = JSON.stringify(value, null, unit).split("\n").map((l, n) => (n === 0 ? l : m[1] + l)).join("\n");
  return text.slice(0, open) + body + text.slice(end + 1);
};
export function patchView(file: string, band: number | string, view: string, edit: (shapes: any[]) => any[]): void {
  let text = readFileSync(file, "utf8");
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const reps = text.indexOf('"representations"');
  const bandRe = new RegExp(`\\n( *)"${band}": \\{`);
  const bm = bandRe.exec(text.slice(reps))!;
  const bandOpen = reps + bm.index + bm[0].length - 1;
  const bandEnd = matchEnd(text, bandOpen);
  const geo = text.indexOf('"geometry"', bandOpen);
  if (geo < 0 || geo > bandEnd) throw new Error("no geometry");
  const viewRe = new RegExp(`\\n( *)"${view}": \\[`);
  const next = rewrite(text, geo, viewRe, edit, unit);
  writeFileSync(file, next);
}
export function patchSymbols(file: string, edit: (list: any[]) => any[]): void {
  const text = readFileSync(file, "utf8");
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  writeFileSync(file, rewrite(text, 0, /\n( *)"symbols": \[/, edit, unit));
}
export function patchBones(file: string, edit: (bones: any[]) => any[]): void {
  const text = readFileSync(file, "utf8");
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const rig = text.indexOf('"rig"');
  writeFileSync(file, rewrite(text, rig, /\n( *)"bones": \[/, edit, unit));
}
export function patchAnchors(file: string, edit: (anchors: any[]) => any[]): void {
  const text = readFileSync(file, "utf8");
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const rig = text.indexOf('"rig"');
  writeFileSync(file, rewrite(text, rig, /\n( *)"anchors": \[/, edit, unit));
}
export function patchArray(file: string, from: string, keyRe: RegExp, edit: (value: any) => any): void {
  const text = readFileSync(file, "utf8");
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const found = from === "" ? 0 : text.indexOf(from);
  if (found < 0) throw new Error(`marker ${from} not found`);
  const start = Math.max(0, text.lastIndexOf("\n", found));
  writeFileSync(file, rewrite(text, start, keyRe, edit, unit));
}
