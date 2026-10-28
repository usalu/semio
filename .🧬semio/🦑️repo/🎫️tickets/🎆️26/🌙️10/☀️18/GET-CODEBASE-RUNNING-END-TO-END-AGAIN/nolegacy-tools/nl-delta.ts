/** 🔬️ Prose sites of the language-law findings of a root that HEAD does not have (multiset by literal), with every site listed: NL_ROOT=<root> bun nl-delta.ts <findings json> */
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.NL_ROOT ?? LIVE;
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
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
const findings = JSON.parse(readFileSync(process.argv[2]!, "utf8")).findings as { rule: string; path: string; detail: string }[];
for (const f of findings) {
  if (f.rule !== "prose-literal") continue;
  const now = sites(readFileSync(`${ROOT}/${f.path}`, "utf8"));
  const head = spawnSync("git", ["show", `HEAD:${f.path}`], { cwd: LIVE, encoding: "utf8", maxBuffer: 1 << 28 });
  const before = head.status === 0 ? sites(head.stdout) : [];
  const pool = new Map<string, number>();
  for (const s of before) pool.set(s.lit, (pool.get(s.lit) ?? 0) + 1);
  const fresh = now.filter((s) => { const n = pool.get(s.lit) ?? 0; if (n > 0) { pool.set(s.lit, n - 1); return false; } return true; });
  console.log(`## ${f.path} ${f.detail} head=${head.status === 0 ? before.length : "absent"} fresh=${fresh.length}`);
  for (const s of fresh) console.log(`  ${s.line}: ${s.lit.slice(0, 200)}`);
}
