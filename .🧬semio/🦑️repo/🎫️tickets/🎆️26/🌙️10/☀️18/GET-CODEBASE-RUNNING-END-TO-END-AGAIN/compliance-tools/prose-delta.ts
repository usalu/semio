/** 🔬️ Prose sites of the listed files that HEAD does not have (multiset difference by literal): bun prose-delta.ts <list> */
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio";
const language = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu")];
const sites = (text: string): { line: number; lit: string }[] => {
  const out: { line: number; lit: string }[] = [];
  text.split("\n").forEach((line, i) => {
    if (/^\s*(\/\/|\/\*|\*)/u.test(line)) return;
    for (const site of SITES) for (const m of line.matchAll(site)) {
      const lit = m[1]!;
      if (lit.startsWith("`") && lit.includes("\n")) continue;
      if (language.isProseV1(lit.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) out.push({ line: i + 1, lit });
    }
  });
  return out;
};
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const now = sites(readFileSync(`${ROOT}/${p}`, "utf8"));
  const head = spawnSync("git", ["show", `HEAD:${p}`], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 });
  const before = head.status === 0 ? sites(head.stdout) : [];
  const pool = new Map<string, number>();
  for (const s of before) pool.set(s.lit, (pool.get(s.lit) ?? 0) + 1);
  const fresh = now.filter((s) => { const n = pool.get(s.lit) ?? 0; if (n > 0) { pool.set(s.lit, n - 1); return false; } return true; });
  console.log(`## ${p} now=${now.length} head=${before.length} fresh=${fresh.length}`);
  for (const f of fresh) console.log(`${f.line}: ${f.lit.slice(0, 150)}`);
}
