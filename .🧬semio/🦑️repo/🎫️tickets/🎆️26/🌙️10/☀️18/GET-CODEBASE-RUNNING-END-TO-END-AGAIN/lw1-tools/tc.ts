import ts from "typescript";
import { resolve } from "node:path";
import { readFileSync } from "node:fs";
const list = process.argv.slice(2).flatMap(f => readFileSync(f, "utf8").split("\n").filter(Boolean));
const files = list.map(f => resolve(f));
const program = ts.createProgram(files, { noEmit: true, strict: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, skipLibCheck: true, jsx: ts.JsxEmit.ReactJSX, types: ["bun", "node"], lib: ["lib.esnext.d.ts", "lib.dom.d.ts"] });
let bad = 0;
const byCode = new Map<number, number>();
for (const f of files) {
  const sf = program.getSourceFile(f);
  if (!sf) { console.log("MISSING", f); bad++; continue; }
  const diags = [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)];
  for (const d of diags) { bad++; byCode.set(d.code, (byCode.get(d.code) ?? 0) + 1); const { line } = ts.getLineAndCharacterOfPosition(sf, d.start ?? 0); console.log(f.replace(process.cwd() + "/", "") + ":" + (line + 1) + ": TS" + d.code + " " + ts.flattenDiagnosticMessageText(d.messageText, "\n").slice(0, 260)); }
}
console.log("checked", files.length, "diagnostics", bad, JSON.stringify([...byCode]));
