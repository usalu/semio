import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const rem = readFileSync("all-rem.jsonl", "utf8").split("\n").filter(Boolean).map((x) => JSON.parse(x));
const keys = new Map<string, { f: string; n: number }[]>();
for (const r of rem) {
  const body = r.t.replace(/^(\/\/+|\/\*+)\s*/, "").split("\n")[0];
  const k = body.slice(0, 36);
  if (k.length < 30) continue;
  const a = keys.get(k) ?? [];
  a.push({ f: r.f, n: 0 });
  keys.set(k, a);
}
const files = readFileSync("cand.txt", "utf8").split("\n").filter(Boolean);
const readers = files.filter((f) => !/📚️library/.test(f));
let scanned = 0;
const hits: string[] = [];
const keySet = new Set(keys.keys());
for (const f of readers) {
  let t: string;
  try { t = readFileSync(ROOT + f, "utf8"); } catch { continue; }
  if (!/readFileSync|read_to_string|include_str|Bun\.file|readText|readFile|fs::read/.test(t)) continue;
  scanned++;
  const seen = new Map<string, number>();
  for (const line of t.split("\n")) {
    const b = line.replace(/^[\s"'`\/*]+/, "").slice(0, 36);
    if (b.length >= 30 && keySet.has(b)) seen.set(b, (seen.get(b) ?? 0) + 1);
  }
  for (const [k, c] of seen) {
    const own = keys.get(k)!.some((o) => o.f === f);
    if (own && c < 2) continue;
    hits.push(`${f} <- ${k} x${c}`);
  }
}
console.log("scanned readers", scanned, "keys", keys.size, "hits", hits.length);
console.log(hits.slice(0, 40).join("\n"));
