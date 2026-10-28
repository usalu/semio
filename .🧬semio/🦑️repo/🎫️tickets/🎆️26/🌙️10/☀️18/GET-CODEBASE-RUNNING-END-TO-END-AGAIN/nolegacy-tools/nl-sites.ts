/** 🔭️ Every prose site of the prose-literal findings of a root with 140 characters of context, fresh sites (not in HEAD) marked `+`: NL_ROOT=<root> bun nl-sites.ts <findings json> [all] */
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.NL_ROOT ?? LIVE;
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu")];
const sites = (text: string) => {
  const out: { line: number; lit: string; context: string }[] = [];
  text.split("\n").forEach((line, i) => {
    if (/^\s*(\/\/|\/\*|\*)/u.test(line)) return;
    for (const site of SITES) for (const m of line.matchAll(site)) {
      const lit = m[1]!;
      if (lit.startsWith("`") && lit.includes("\n")) continue;
      if (language.isProseV1(lit.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) out.push({ line: i + 1, lit, context: line.slice(Math.max(0, m.index! - 140), m.index! + m[0].length + 60) });
    }
  });
  return out;
};
const all = process.argv[3] === "all";
for (const f of JSON.parse(readFileSync(process.argv[2]!, "utf8")).findings as { rule: string; path: string; detail: string }[]) {
  if (f.rule !== "prose-literal") continue;
  const now = sites(readFileSync(`${ROOT}/${f.path}`, "utf8"));
  const head = spawnSync("git", ["show", `HEAD:${f.path}`], { cwd: LIVE, encoding: "utf8", maxBuffer: 1 << 28 });
  const pool = new Map<string, number>();
  for (const s of head.status === 0 ? sites(head.stdout) : []) pool.set(s.lit, (pool.get(s.lit) ?? 0) + 1);
  console.log(`## ${f.path} ${f.detail}`);
  for (const s of now) { const n = pool.get(s.lit) ?? 0; const fresh = n === 0; if (!fresh) pool.set(s.lit, n - 1); if (fresh || all) console.log(`${fresh ? "+" : " "} ${s.line}: ${s.context}`); }
}
