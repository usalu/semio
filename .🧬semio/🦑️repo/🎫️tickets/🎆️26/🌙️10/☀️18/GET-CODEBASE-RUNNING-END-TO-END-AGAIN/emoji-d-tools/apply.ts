import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { docstringOpenersOfText, docstringEmojiReuseOfText, docstringHitsOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const [candsPath, picksDir, dryFlag, stateArg] = process.argv.slice(2);
const dry = dryFlag === "--dry";
const ROOT = "/Users/ueli/Documents/semio/";
const statePath = stateArg ?? `${picksDir}/../applied.json`;
const state: { applied: string[] } = existsSync(statePath) ? JSON.parse(readFileSync(statePath, "utf8")) : { applied: [] };
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const norm = (s: string) => s.replaceAll("️", "");
const MARK = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)/u;
const firstGrapheme = (s: string) => seg.segment(s)[Symbol.iterator]().next().value!.segment;
const cands: any[] = JSON.parse(readFileSync(candsPath!, "utf8"));
const byId = new Map(cands.map((c) => [c.id, c]));
const picks = new Map<number, string>();
for (const f of readdirSync(picksDir!).filter((f) => f.endsWith(".txt")).sort()) for (const l of readFileSync(`${picksDir}/${f}`, "utf8").split("\n")) {
  const m = /^(\d+)\s+(\S+)/.exec(l);
  if (m) picks.set(Number(m[1]), norm(m[2]!));
}
const perFile = new Map<string, { c: any; pick: string }[]>();
for (const [id, pick] of picks) {
  const c = byId.get(id);
  if (!c) { console.log("unknown id", id); continue; }
  if (firstGrapheme(pick) !== pick || !MARK.test(pick)) { console.log("BAD PICK", id, pick); continue; }
  (perFile.get(c.path) ?? perFile.set(c.path, []).get(c.path)!).push({ c, pick });
}
const report = { applied: 0, files: 0, hot: [] as string[], drift: [] as string[], conflict: [] as string[], skippedDone: 0 };
const now = Date.now();
for (const [path, list] of perFile) {
  if (state.applied.includes(path)) { report.skippedDone++; continue; }
  const abs = ROOT + path;
  let text: string;
  try { text = readFileSync(abs, "utf8"); } catch { report.drift.push(path + " (unreadable)"); continue; }
  if (!dry && now - statSync(abs).mtimeMs < 30 * 60 * 1000) { report.hot.push(path); continue; }
  const lines = text.split("\n");
  const openers = docstringOpenersOfText(path, text);
  const byLine = new Map(openers.map((o) => [o.line, o]));
  const edits: { idx: number; at: number; len: number; pick: string }[] = [];
  let bad = false;
  for (const { c, pick } of list) {
    let o = byLine.get(c.line);
    const key = c.text.replace(/^\S+\s*/, "").slice(0, 40);
    const ok = (x: typeof o) => x && norm(firstGrapheme(x.content)) === c.old && x.content.replace(/^\S+\s*/, "").startsWith(key);
    if (!ok(o)) o = openers.find((x) => ok(x));
    if (!o) { report.drift.push(`${path}:${c.line}`); bad = true; continue; }
    const raw = lines[o.line - 1]!;
    const g = firstGrapheme(o.content);
    const at = raw.indexOf(o.content);
    if (at < 0 || !raw.startsWith(g, at)) { report.drift.push(`${path}:${c.line} (column)`); bad = true; continue; }
    edits.push({ idx: o.line - 1, at, len: g.length, pick });
  }
  if (bad) continue;
  const after = new Map<number, string>();
  for (const o of openers) if (MARK.test(o.content)) after.set(o.line, norm(firstGrapheme(o.content)));
  for (const e of edits) after.set(e.idx + 1, e.pick);
  const counts = new Map<string, number>();
  for (const v of after.values()) counts.set(v, (counts.get(v) ?? 0) + 1);
  const dupLeft = [...counts].filter(([, n]) => n > 1).map(([e]) => e);
  if (dupLeft.length) { report.conflict.push(`${path}: ${dupLeft.join("")}`); continue; }
  for (const e of edits.sort((a, b) => b.idx - a.idx || b.at - a.at)) {
    const raw = lines[e.idx]!;
    lines[e.idx] = raw.slice(0, e.at) + e.pick + "️" + raw.slice(e.at + e.len);
  }
  const next = lines.join("\n");
  const reuseBefore = docstringEmojiReuseOfText(path, text).length;
  const reuseAfter = docstringEmojiReuseOfText(path, next);
  const hitsBefore = docstringHitsOfText(path, text).length, hitsAfter = docstringHitsOfText(path, next).length;
  if (hitsAfter > hitsBefore) { report.conflict.push(`${path}: new first-law hits`); continue; }
  if (!dry) {
    if (readFileSync(abs, "utf8") !== text) { report.drift.push(path + " (changed while editing)"); continue; }
    writeFileSync(abs, next);
    state.applied.push(path);
  }
  void reuseBefore; void reuseAfter;
  report.applied += edits.length;
  report.files++;
}
if (!dry) writeFileSync(statePath, JSON.stringify(state));
console.log(JSON.stringify({ ...report, hot: report.hot.length, drift: report.drift.length, conflict: report.conflict.length }));
if (report.hot.length) console.log("HOT", report.hot.join("\n"));
if (report.drift.length) console.log("DRIFT", report.drift.join("\n"));
if (report.conflict.length) console.log("CONFLICT", report.conflict.join("\n"));
