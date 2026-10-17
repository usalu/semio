import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, existsSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/";
const paths = (await Bun.stdin.text()).split("\n").filter(Boolean);
let bad = 0;
for (const p of paths) {
  if (!existsSync(root + p)) { console.log("MISSING", p); bad++; continue; }
  const kind = p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const s = ts.createSourceFile(p, readFileSync(root + p, "utf8"), ts.ScriptTarget.Latest, false, kind);
  const d = (s as any).parseDiagnostics as ts.Diagnostic[];
  if (d.length) { bad++; console.log("FAIL", p, d.length, ts.flattenDiagnosticMessageText(d[0].messageText, " "), d[0].start); }
}
console.log("ts checked", paths.length, "bad", bad);
