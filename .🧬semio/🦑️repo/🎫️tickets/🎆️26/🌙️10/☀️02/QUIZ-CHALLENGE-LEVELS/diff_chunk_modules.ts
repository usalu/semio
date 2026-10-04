/** ⚖️ Compares the entry chunk's module groups of two `site_chunk_modules.vite.ts` reports (rendered bytes before
 * minification, and gzip of each group's rendered code alone).
 * Usage: `bun diff_chunk_modules.ts <a-label> <b-label>` */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const entryOf = (label: string): Map<string, { raw: number; gzip: number }> => {
  const lines = readFileSync(resolve(here, `🗑️generated/site-final/${label}-modules.txt`), "utf8").split("\n");
  const start = lines.findIndex((line) => line.includes("entry=true"));
  const end = lines.findIndex((line, index) => index > start && line.startsWith("=="));
  return new Map(lines.slice(start + 1, end < 0 ? undefined : end).filter(Boolean).map((line) => {
    const [, raw, gzip, key] = /^\s*(\d+)\s+(\d+)gz\s+(.*)$/u.exec(line)!;
    return [key!, { raw: Number(raw), gzip: Number(gzip) }];
  }));
};
const [a, b] = [entryOf(process.argv[2]!), entryOf(process.argv[3]!)];
const rows = [...new Set([...a.keys(), ...b.keys()])].map((key) => ({ key, raw: (b.get(key)?.raw ?? 0) - (a.get(key)?.raw ?? 0), gzip: (b.get(key)?.gzip ?? 0) - (a.get(key)?.gzip ?? 0) })).filter((row) => row.raw !== 0);
for (const row of rows.sort((x, y) => y.raw - x.raw)) console.log(`${String(row.raw).padStart(8)} raw ${String(row.gzip).padStart(7)} gz  ${row.key}`);
console.log(`sum ${rows.reduce((s, r) => s + r.raw, 0)} raw ${rows.reduce((s, r) => s + r.gzip, 0)} gz (groups gzipped alone)`);
