// 🔎 Parse-only scan: prints every root of a generated tsconfig whose source has a syntax error (TypeScript's parseDiagnostics), one path per line.
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const project = process.argv[2];
const document = JSON.parse(readFileSync(project, "utf8"));
const base = dirname(resolve(project));
for (const file of document.files) {
  const path = resolve(base, file);
  let text;
  try { text = readFileSync(path, "utf8"); } catch { continue; }
  const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.ESNext, false, kind);
  if (source.parseDiagnostics.length > 0) console.log(file);
}
