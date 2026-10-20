// 👻 Lists references to a missing 📜️script.ts whose directory owns a 🟦️.ts: leftovers of a script-to-module rename. usage: bun ghosts.ts
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {C} from "./vfs.ts";
const REPO = "/Users/ueli/Documents/semio";
const files = readFileSync(`${C}/tax-stage-tools/index/grep-files.txt`, "utf8").split("\n").filter(Boolean);
const found = new Map<string, {file: string; line: number; spec: string}[]>();
for (const rel of files) {
  if (!/\.(ts|tsx|mts|mjs|js|json)$/u.test(rel)) continue;
  let text: string; try { text = readFileSync(join(REPO, rel), "utf8"); } catch { continue; }
  if (!text.includes("📜️script.ts")) continue;
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) for (const m of lines[i]!.matchAll(/(["'`])((?:\\.|(?!\1)[^\n\\])*?📜️script\.ts)\1/gu)) {
    const spec = m[2]!;
    let target: string | null = null;
    if (spec.startsWith("./") || spec.startsWith("../")) target = resolve(dirname(join(REPO, rel)), spec).slice(REPO.length + 1);
    else if (/^(?:🧰️|✏️|🌎️|♻️|🏢️|🎓️)/u.test(spec)) target = spec;
    if (!target || target.includes("${") || existsSync(join(REPO, target))) continue;
    if (existsSync(join(REPO, dirname(target), "🟦️.ts"))) (found.get(target) ?? found.set(target, []).get(target)!).push({file: rel, line: i + 1, spec});
  }
}
for (const [t, rows] of found) { console.log(`GHOST ${t.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "L")} (${rows.length})`); for (const r of rows.slice(0, 4)) console.log(`    ${r.file.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "L").slice(-100)}:${r.line}`); }
writeFileSync(`${C}/🗑️generated/tax-stage/ghosts.json`, JSON.stringify([...found].map(([target, rows]) => ({target, rows})), null, 1));
