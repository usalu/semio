import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const SKIP = new Set(["node_modules", ".git", "dist", ".next", "target"]);

const REPLACERS: [RegExp, string][] = [
  [/repo\/lib\/js\/src\/bundle-script\.ts/g, "repo/lib/js/index.ts"],
  [/repo\/lib\/js\/src\/linter\.ts/g, "repo/lib/js/index.ts"],
  [/repo\/lib\/js\/src\/cli\.ts/g, "repo/lib/js/index.ts"],
  [/repo\/lib\/js\/src\/dependency-boundary\.ts/g, "repo/lib/js/index.ts"],
  [/repo\/lib\/js\/src\/script\.ts/g, "repo/lib/js/index.ts"],
  [/\.\/src\/bundle-script\.ts/g, "./index.ts"],
  [/\.\/src\/linter\.ts/g, "./index.ts"],
  [/\.\/src\/cli\.ts/g, "./index.ts"],
  [/\.\/src\/dependency-boundary\.ts/g, "./index.ts"],
  [/\.\/src\/script\.ts/g, "./index.ts"],
  [/lib\/js\/src\/bundle-script\.ts/g, "lib/js/index.ts"],
  [/lib\/js\/src\/linter\.ts/g, "lib/js/index.ts"],
  [/lib\/js\/src\/cli\.ts/g, "lib/js/index.ts"],
  [/lib\/js\/src\/dependency-boundary\.ts/g, "lib/js/index.ts"],
  [/lib\/js\/src\/script\.ts/g, "lib/js/index.ts"],
];

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    if (SKIP.has(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) out.push(...walk(p));
    else if (/\.tsx?$/.test(name)) out.push(p);
  }
  return out;
}

let n = 0;
for (const file of walk(root)) {
  if (file.includes(`${join(".repo", "🎫")}`) && file.includes("CONSOLIDATE")) continue;
  let text = readFileSync(file, "utf8");
  let next = text;
  for (const [re, rep] of REPLACERS) next = next.replace(re, rep);
  if (next !== text) {
    writeFileSync(file, next);
    n++;
  }
}
console.log(`updated ${n} files`);
