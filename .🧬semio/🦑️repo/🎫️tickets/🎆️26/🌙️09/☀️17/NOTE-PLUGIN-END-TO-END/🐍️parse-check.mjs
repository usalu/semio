#!/usr/bin/env bun
/** 🔎️ TypeScript parse diagnostics plus the fused-statement signature (`if (…)` / `for (…)` / `else` head followed by 3+ spaces and a statement) per file. */
import ts from "typescript";
import { readFileSync } from "node:fs";
for (const path of process.argv.slice(2)) {
  const text = readFileSync(path, "utf8");
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const fused = text.split("\n").map((line, index) => [index + 1, line]).filter(([, line]) => /^\s*(?:\}\s*)?(?:if|for|while)\s*\(.*\)\s{3,}\S|\belse\s{3,}\S/.test(line));
  console.log(`[DEBUG] parse=${sf.parseDiagnostics.length} fused=${fused.length} ${path}`);
  for (const [line, content] of fused.slice(0, 5)) console.log(`[DEBUG]   ${line}: ${content.trim().slice(0, 140)}`);
}
