import { readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const jsDir = join(root, "repo/lib/js");
const srcDir = join(jsDir, "src");
const SKIP = new Set(["node_modules", ".git", "dist", ".next", "target", "pkg", "out"]);

const REPLACERS: [RegExp, string][] = [
  [/repo\/lib\/js\/src\/index\.ts/g, "repo/lib/js/index.ts"],
  [/lib\/js\/src\/index\.ts/g, "lib/js/index.ts"],
  [/\.\/src\/index\.ts/g, "./index.ts"],
  [/\.\/src\/index\.test\.ts/g, "./index.test.ts"],
];

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    if (SKIP.has(name)) continue;
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) out.push(...walk(p));
    else if (/\.(tsx?|mjs|json|md)$/.test(name)) out.push(p);
  }
  return out;
}

renameSync(join(srcDir, "index.ts"), join(jsDir, "index.ts"));
renameSync(join(srcDir, "index.test.ts"), join(jsDir, "index.test.ts"));
try {
  rmSync(srcDir, { recursive: true });
} catch {
  /* empty */
}

const pkg = JSON.parse(readFileSync(join(jsDir, "package.json"), "utf8"));
pkg.exports["."] = "./index.ts";
pkg.scripts.test = "bun test ./index.test.ts";
writeFileSync(join(jsDir, "package.json"), `${JSON.stringify(pkg, null, 2)}\n`);

const tsconfig = JSON.parse(readFileSync(join(jsDir, "tsconfig.json"), "utf8"));
tsconfig.include = ["*.ts", "bin/**/*.ts"];
tsconfig.exclude = ["*.test.ts"];
writeFileSync(join(jsDir, "tsconfig.json"), `${JSON.stringify(tsconfig, null, 2)}\n`);

let n = 0;
for (const file of walk(root)) {
  if (file.includes(`${join(".repo", "🎫️")}`) && file.includes("flatten-repo-lib-js-src")) continue;
  let text = readFileSync(file, "utf8");
  let next = text;
  for (const [re, rep] of REPLACERS) next = next.replace(re, rep);
  if (next !== text) {
    writeFileSync(file, next);
    n++;
  }
}
console.log(`flattened repo/lib/js and updated ${n} files`);
