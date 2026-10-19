// 🧲 Rewrites artifact-package `commands`/`testCommands` records into explicitly wired ScriptRouter constants. usage: bun b1-routers.ts <list.txt> [--write]
import {readFileSync} from "node:fs";
import ts from "typescript";
import {accepts, guardedWrite, ledger, read, rel, ROUTING, syntaxOk} from "./lib.ts";
const write = process.argv.includes("--write");
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = read(p);
  if (!/runArtifact(Rust|TypeScript)PackageMain/.test(src) || !/\b(testCommands|commands)\s*:/.test(src)) continue;
  const f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
  let call: ts.CallExpression | null = null, stmt: ts.Statement | null = null;
  for (const st of f.statements) {
    const visit = (n: ts.Node): void => { if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && /^runArtifact(Rust|TypeScript)PackageMain$/.test(n.expression.text)) { call = n; stmt ??= st; } ts.forEachChild(n, visit); };
    visit(st);
  }
  if (!call || !stmt) { count("no-call"); ledger({tool: "b1", p, result: "no-call"}); continue; }
  const options = (call as ts.CallExpression).arguments[2];
  if (!options || !ts.isObjectLiteralExpression(options)) { count("options-not-literal"); ledger({tool: "b1", p, result: "options-not-literal"}); continue; }
  const edits: {start: number; end: number; text: string}[] = [], decls: string[] = [];
  let bad = false;
  for (const prop of options.properties) {
    if (!ts.isPropertyAssignment(prop) || !ts.isIdentifier(prop.name) || !["commands", "testCommands"].includes(prop.name.text)) continue;
    const name = prop.name.text, init = prop.initializer;
    if (!ts.isObjectLiteralExpression(init)) { bad = true; break; }
    const regs: string[] = [];
    for (const row of init.properties) {
      if (!ts.isPropertyAssignment(row) && !ts.isShorthandPropertyAssignment(row)) { bad = true; break; }
      const key = ts.isShorthandPropertyAssignment(row) ? row.name.text : ts.isIdentifier(row.name) ? row.name.text : ts.isStringLiteral(row.name) ? row.name.text : null;
      const value = ts.isShorthandPropertyAssignment(row) ? row.name.text : ts.isPropertyAssignment(row) && ts.isIdentifier(row.initializer) ? row.initializer.text : null;
      if (key === null || value === null) { bad = true; break; }
      regs.push(`.register(${JSON.stringify(key)}, ${value})`);
    }
    if (bad) break;
    const id = name === "commands" ? "router" : "testRouter";
    let used = false; const scan = (n: ts.Node): void => { if (ts.isIdentifier(n) && n.text === id) used = true; ts.forEachChild(n, scan); }; scan(f); if (used) { bad = true; break; }
    decls.push(`const ${id} = new ScriptRouter(import.meta.dir)${regs.join("")};\n`);
    edits.push({start: prop.getStart(), end: prop.end, text: `${name}: ${id}`});
  }
  if (bad || !edits.length) { if (process.env.DBG) console.log("UNSUPPORTED", p, bad); count("unsupported"); ledger({tool: "b1", p, result: "unsupported"}); continue; }
  edits.push({start: stmt.getStart(), end: stmt.getStart(), text: decls.join("")});
  let out = src;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  const routingImport = /import\s*\{([^}]*)\}\s*from\s*"([^"]*🧭️routing\/🟦️\.ts)";/u.exec(out) ;
  const hasRouter = /import\s*\{[^}]*\bScriptRouter\b[^}]*\}\s*from/u.test(out);
  if (!hasRouter) {
    if (routingImport) out = out.replace(routingImport[0], `import {${routingImport[1]!.trim()}${routingImport[1]!.trim() ? ", " : ""}ScriptRouter} from "${routingImport[2]}";`);
    else out = out.replace(/^(#!.*\n)?/, (m) => `${m}import { ScriptRouter } from "${rel(p, ROUTING)}";\n`);
  }
  const ok = syntaxOk(p, out);
  const acc = ok && accepts(out);
  if (!ok) { count("syntax-fail"); ledger({tool: "b1", p, result: "syntax-fail"}); continue; }
  if (!write) { count(acc ? "ok-accepted" : "ok-still-rejected"); continue; }
  const age = (await import("./lib.ts")).ageMinutes(p);
  if (age < 15) { count("skip-recent"); ledger({tool: "b1", p, result: "skip-recent"}); continue; }
  if (read(p) !== src) { count("skip-changed"); continue; }
  (await import("node:fs")).writeFileSync(`/Users/ueli/Documents/semio/${p}`, out);
  count(acc ? "written-accepted" : "written-still-rejected"); ledger({tool: "b1", p, result: acc ? "written-accepted" : "written-still-rejected"});
}
console.log(tally);
