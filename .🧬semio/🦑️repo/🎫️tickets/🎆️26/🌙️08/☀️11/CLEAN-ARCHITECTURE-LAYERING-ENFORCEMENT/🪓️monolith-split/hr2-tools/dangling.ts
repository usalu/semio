/** 🔗️ hr2 dangling-reference scanner: resolves Cargo path deps, Rust path/include literals, TS relative specifiers and repo-rooted path strings in a file list and reports the ones that do not exist. Usage: bun dangling.ts <filelist.txt> <out.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const [listPath, outPath] = process.argv.slice(2);
const files = readFileSync(listPath!, "utf8").split("\n").filter((x) => x.length > 0);
const dangling: { file: string; kind: string; ref: string }[] = [];
let checked = 0;
const ROOT_PREFIXES = ["✏️s/", "🧰️framework/", "🌎️hub/", "♻️mit-bestand/"];
function check(file: string, kind: string, ref: string, base: string) {
  checked++;
  const target = ref.startsWith("/") ? ref : normalize(join(ROOT, base, ref));
  if (!existsSync(target)) dangling.push({ file, kind, ref });
}
for (const file of files) {
  let text: string;
  try {
    text = readFileSync(join(ROOT, file), "utf8");
  } catch {
    continue;
  }
  const dir = dirname(file);
  if (file.endsWith("Cargo.toml")) {
    for (const m of text.matchAll(/\bpath\s*=\s*"([^"]+)"/gu)) check(file, "cargo-path", m[1]!, dir);
  } else if (file.endsWith(".rs")) {
    for (const m of text.matchAll(/#\[path\s*=\s*"([^"]+)"\]/gu)) check(file, "rust-path", m[1]!, dir);
    for (const m of text.matchAll(/include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)/gu)) check(file, "rust-include", m[1]!, dir);
  } else if (/\.(ts|tsx|mts|cts|js|mjs|cjs)$/u.test(file)) {
    for (const m of text.matchAll(/(?:from|import\()\s*["'](\.{1,2}\/[^"']+)["']/gu)) check(file, "ts-import", m[1]!, dir);
  }
  for (const m of text.matchAll(/["'`]((?:✏️s|🧰️framework|🌎️hub|♻️mit-bestand)\/[^"'`\s${}]+)["'`]/gu)) {
    const ref = m[1]!;
    if (ROOT_PREFIXES.some((p) => ref.startsWith(p)) && !/[*{}$]/u.test(ref)) check(file, "root-string", ref, "");
  }
}
console.log(`files=${files.length} checked=${checked} dangling=${dangling.length}`);
writeFileSync(outPath!, JSON.stringify(dangling, null, 1));
