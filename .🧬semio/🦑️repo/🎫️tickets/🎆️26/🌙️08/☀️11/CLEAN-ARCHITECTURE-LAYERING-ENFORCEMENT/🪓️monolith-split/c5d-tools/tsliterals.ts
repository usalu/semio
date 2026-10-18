// 🔎 c5d: every string literal in the repo's TS files that is a relative path ending in `.ts`/`.json`/`.rs` and resolves (against the file's own directory) to one of the given files. Catches `new URL("../../🟦️.ts", import.meta.url)` style readers that carry no folder name.
import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const REPO = "/Users/ueli/Documents/semio";
const targets = new Set(process.argv.slice(2).map(p => resolve(p)));
const list = (args: string[]) => execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", ...args], {cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28}).split("\n").filter(Boolean);
const files = [...new Set([...list(["ls-files", "--", "*.ts", "*.tsx", "*.mts", "*.mjs", "*.js"]), ...list(["ls-files", "-o", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts", "*.mjs", "*.js"])])].filter(f => !f.startsWith(".🧬semio/") && !f.includes("node_modules/"));
const needles = [...targets].map(t => t.split("/").pop()!);
let hits = 0;
for (const rel of files) {
  const abs = resolve(REPO, rel);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  if (!needles.some(n => text.includes(n))) continue;
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true);
  const visit = (n: ts.Node): void => {
    if ((ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n)) && n.text.startsWith(".") && targets.has(resolve(dirname(abs), n.text))) {
      const p = n.parent;
      const isImport = ts.isImportDeclaration(p) || ts.isExportDeclaration(p) || (ts.isCallExpression(p) && p.expression.kind === ts.SyntaxKind.ImportKeyword);
      if (!isImport) { hits++; console.log(`${rel}:${sf.getLineAndCharacterOfPosition(n.getStart()).line + 1} ${JSON.stringify(n.text)} :: ${p.getText().slice(0, 140).replace(/\s+/g, " ")}`); }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
console.log("non-import literals resolving to a target:", hits);
