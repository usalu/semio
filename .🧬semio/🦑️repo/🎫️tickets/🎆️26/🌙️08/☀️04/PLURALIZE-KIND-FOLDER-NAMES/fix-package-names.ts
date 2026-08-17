#!/usr/bin/env bun
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const SKIP = ["/node_modules/", "/.git/", "/target/", "/.🦑️repo/🎫️tickets/"];

const FIXES: [RegExp, string][] = [
  [/@semio-tech\/assetss+/g, "@semio-tech/assets"],
  [/@semio-tech\/iconss+/g, "@semio-tech/icons"],
  [/@semio-tech\/imagess+/g, "@semio-tech/images"],
  [/@semio-tech\/logoss+/g, "@semio-tech/logos"],
  [/@semio-tech\/puzzle-assetss+/g, "@semio-tech/puzzle-assets"],
  [/@semio-tech\/remodel-imagess+/g, "@semio-tech/remodel-images"],
];

function walk(dir: string, out: string[]) {
  if (SKIP.some((s) => dir.includes(s))) return;
  for (const e of readdirSync(dir)) {
    if (e === "node_modules" || e === "target") continue;
    const p = join(dir, e);
    if (SKIP.some((s) => p.includes(s))) continue;
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (/\.(ts|tsx|js|json|toml|md|mjs|cjs|rs|go)$/.test(e) || e === "launch.json") out.push(p);
  }
}

const files: string[] = [];
walk(root, files);
let n = 0;
for (const f of files) {
  const o = readFileSync(f, "utf8");
  let x = o;
  for (const [re, rep] of FIXES) x = x.replace(re, rep);
  if (x !== o) {
    writeFileSync(f, x);
    n++;
  }
}
console.log(`[DEBUG] fixed package names in ${n} files`);
