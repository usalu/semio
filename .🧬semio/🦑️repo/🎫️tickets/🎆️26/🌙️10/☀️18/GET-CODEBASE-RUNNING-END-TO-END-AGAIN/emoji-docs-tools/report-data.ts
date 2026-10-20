import { blocksOf } from "./docs.ts";
import { norm } from "./pick.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const names = ["applied-apply.json", "applied-apply-delta.json", "applied-apply-pass2.json", "applied-apply-pass3.json", "applied-apply-pass4.json"];
const perFile = new Map<string, any[]>();
for (const n of names) for (const x of JSON.parse(await Bun.file(`${out}/${n}`).text())) { const l = perFile.get(x.p) ?? []; l.push(...x.edits); perFile.set(x.p, l); }
const go = JSON.parse(await Bun.file(`${out}/go-applied-apply.json`).text());
const tree: Record<string, Record<string, number>> = {};
const how: Record<string, number> = {};
const bump = (p: string, lang: string, n: number) => { const t = p.split("/").slice(0, 2).join("/").replace(/^(\.storybook)\/.*/, "$1"); tree[t] ??= {}; tree[t][lang] = (tree[t][lang] ?? 0) + n; };
let total = 0;
const all: any[] = [];
for (const [p, es] of perFile) { const lang = p.endsWith(".rs") ? "rust" : "ts/js"; bump(p, lang, es.length); total += es.length; for (const e of es) { how[e.how] = (how[e.how] ?? 0) + 1; all.push({ p, ...e }); } }
for (const g of go) { bump(g.p, "go", g.n); total += g.n; how["go-" + g.edits[0].how] = (how["go-" + g.edits[0].how] ?? 0) + g.n; for (const e of g.edits) all.push({ p: g.p, ...e }); }
let dupFiles = 0, dupEmoji = 0, dupFromPicks = 0;
for (const [p, es] of perFile) {
  let text = ""; try { text = await Bun.file(`${root}/${p}`).text(); } catch { continue; }
  const counts = new Map<string, number>();
  for (const b of blocksOf(p, text)) if (b.startEmoji) counts.set(norm(b.startEmoji), (counts.get(norm(b.startEmoji)) ?? 0) + 1);
  let d = 0;
  for (const e of new Set(es.filter((e) => e.emoji).map((e) => norm(e.emoji)))) if ((counts.get(e) ?? 0) > 1) { d++; if (es.some((x) => norm(x.emoji) === e && (x.how === "dict" || x.how.endsWith("+dict") || x.how === "pool" || x.how === "collision" || x.how === "dict-exhausted"))) dupFromPicks++; }
  if (d) { dupFiles++; dupEmoji += d; }
}
console.log(JSON.stringify({ total, files: perFile.size + go.length, tree, how, dupFiles, dupEmoji, dupFromPicks }, null, 1));
const picks = all.filter((e) => e.how === "dict" || e.how.endsWith("+dict"));
const step = Math.floor(picks.length / 80);
const sample = [];
for (let i = 7; i < picks.length && sample.length < 80; i += step) { const r = picks[i]; sample.push(`| ${r.emoji} | \`${(r.name || "-").slice(0, 34)}\` | ${r.head.replace(/\|/g, "/").replace(/`/g, "'").slice(0, 70)} | ${r.p.split("/").slice(-2).join("/")} |`); }
await Bun.write(`${out}/sample80.md`, sample.join("\n"));
