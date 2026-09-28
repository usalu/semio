#!/usr/bin/env bun
/** 🧾️ R10 probe: syntactic diagnostics of TS/TSX/JS files (TypeScript's own parser), JSON parse, gofmt -e for Go, rustfmt (stable, parse errors only) for Rust. Usage: bun syntax-check.ts <list-file> */
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9bb2d328-ac37-4388-a429-b069582cbcf9/scratchpad";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const path of files) {
  const text = readFileSync(`${root}/${path}`, "utf8");
  let problems: string[] = [];
  if (/\.(tsx?|jsx?|mjs)$/u.test(path)) {
    const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : path.endsWith(".js") ? ts.ScriptKind.JS : ts.ScriptKind.TS;
    const source = ts.createSourceFile(path, text, ts.ScriptTarget.ESNext, true, kind);
    problems = ((source as unknown as { parseDiagnostics: ts.Diagnostic[] }).parseDiagnostics ?? []).map((d) => `${source.getLineAndCharacterOfPosition(d.start ?? 0).line + 1}: ${ts.flattenDiagnosticMessageText(d.messageText, " ")}`);
  } else if (path.endsWith(".json")) {
    try { JSON.parse(text); } catch (error) { problems = [String(error)]; }
  } else if (path.endsWith(".rs")) {
    const result = spawnSync("rustfmt", ["--edition", "2024", "--emit", "stdout", `${root}/${path}`], { encoding: "utf8", cwd: SCRATCH, maxBuffer: 1 << 30 });
    const errors = result.stderr.split("\n").filter((line) => line.startsWith("error"));
    if (result.status !== 0 && errors.length) problems = errors.slice(0, 3);
  } else if (path.endsWith(".go")) {
    const result = spawnSync("gofmt", ["-e", "-l", `${root}/${path}`], { encoding: "utf8" });
    if (result.status !== 0) problems = [result.stderr.trim().slice(0, 300)];
  }
  if (problems.length) { bad += 1; console.log(`BAD ${path}\n  ${problems.slice(0, 3).join("\n  ")}`); }
}
console.log(`files=${files.length} bad=${bad}`);
