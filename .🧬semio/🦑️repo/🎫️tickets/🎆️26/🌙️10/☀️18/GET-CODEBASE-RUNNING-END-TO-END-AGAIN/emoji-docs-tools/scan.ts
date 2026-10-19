import { blocksOf } from "./docs.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const files = (await Bun.file(`${out}/files.txt`).text()).split("\n").filter((p) => p && !p.endsWith(".go"));
const res: Record<string, { total: number; missing: number; skipped: number; mtime: number }> = {};
const gen: string[] = [];
let fi = 0;
for (const p of files) {
  fi++;
  let text: string;
  let st;
  try { const f = Bun.file(`${root}/${p}`); text = await f.text(); st = await f.stat(); } catch { continue; }
  if (text.length > 3_000_000) continue;
  let bl;
  try { bl = blocksOf(p, text); } catch (e) { console.error("ERR", p, String(e).slice(0, 80)); continue; }
  const missing = bl.filter((b) => !b.startEmoji);
  if (!bl.length) continue;
  res[p] = { total: bl.length, missing: missing.length, skipped: missing.filter((b) => b.skip).length, mtime: st.mtimeMs };
  if (fi % 5000 === 0) console.error("progress", fi, files.length);
}
await Bun.write(`${out}/scan.json`, JSON.stringify(res));
const all = Object.entries(res);
console.log("files with docs", all.length, "blocks", all.reduce((a, [, v]) => a + v.total, 0), "missing", all.reduce((a, [, v]) => a + v.missing, 0), "skipped", all.reduce((a, [, v]) => a + v.skipped, 0));
