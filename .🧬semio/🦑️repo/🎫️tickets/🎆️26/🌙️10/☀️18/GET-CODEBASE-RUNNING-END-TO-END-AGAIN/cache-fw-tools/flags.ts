import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
const pats: Record<string, RegExp> = {
  net: /\bfetch\(\s*["'`]https?:\/\/(?!127\.0\.0\.1|localhost|\[::1\])|https?:\/\/(?!127\.0\.0\.1|localhost|json-schema|www\.w3|semio|example|github\.com\/semio|tools\.ietf|spec\.|datatracker|opensource|unicode|developer\.mozilla|en\.wikipedia|schemas)[a-z0-9.-]+\.(com|org|io|dev|net)\/.{0,30}/i,
  serve: /Bun\.serve\(|createServer\(|\.listen\(/,
  browser: /playwright|chromium|puppeteer|launchBrowser/i,
  docker: /\bdocker\b/,
  git: /["'`]git["'`]\s*,|spawn\w*\(\s*["'`]git/,
  install: /["'`]install["'`]|bun add|npm install/,
};
const out: any[] = [];
for (const f of projectFiles) {
  const dir = dirname(f);
  const s = join(ROOT, dir, "📜️script.ts");
  if (!existsSync(s)) continue;
  const src = readFileSync(s, "utf8");
  const flags = Object.entries(pats).filter(([, r]) => r.test(src)).map(([k]) => k);
  if (flags.length) out.push([dir.slice(-70), readJson(f).name, flags.join(",")]);
}
for (const o of out) console.log(o.join(" | "));
console.log(out.length, "of", projectFiles.length);
