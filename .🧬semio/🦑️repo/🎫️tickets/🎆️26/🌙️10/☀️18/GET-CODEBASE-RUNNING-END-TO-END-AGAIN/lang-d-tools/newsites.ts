import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const law = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts");
const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [
  new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu"),
];
const sites = (text: string): [number, string][] => {
  const out: [number, string][] = [];
  text.split("\n").forEach((line, i) => {
    if (/^\s*(?:\/\/|\/\*|\*)/.test(line)) return;
    for (const site of SITES) for (const m of line.matchAll(site)) {
      const lit = m[1]!;
      if (lit.startsWith("`") && lit.includes("\n")) continue;
      if (law.isProseV1(lit.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) out.push([i + 1, m[0]]);
    }
  });
  return out;
};
for (const f of JSON.parse(readFileSync(process.argv[2]!, "utf8"))) {
  const r = spawnSync("git", ["show", `HEAD:${f.path}`], { cwd: "/Users/ueli/Documents/semio", encoding: "utf8", maxBuffer: 1 << 28 });
  const old = new Map<string, number>();
  if (r.status === 0) for (const [, s] of sites(r.stdout)) old.set(s, (old.get(s) ?? 0) + 1);
  console.log(`### ${f.path} ${f.detail}`);
  for (const [line, s] of sites(readFileSync(f.path, "utf8"))) {
    const n = old.get(s) ?? 0;
    if (n > 0) old.set(s, n - 1);
    else console.log(`  ${line}: ${s.slice(0, 220)}`);
  }
}
