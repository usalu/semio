#!/usr/bin/env bun
/** Line-oriented strip for ShellHost: remove `[DEBUG]` console lines and debug-only runtimeDiagnostics guards. */
import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2];
if (!path) process.exit(1);
let lines = readFileSync(path, "utf8").split("\n");

function isDebugConsoleLine(line) {
  return /console\.(log|debug|info|warn|error)\(/.test(line) && line.includes("[DEBUG]");
}

function parenBalance(line) {
  let n = 0;
  for (const ch of line) {
    if (ch === "(") n++;
    else if (ch === ")") n--;
  }
  return n;
}

const out = [];
let i = 0;
while (i < lines.length) {
  const line = lines[i];
  const trimmed = line.trimStart();

  if (trimmed.startsWith("if (runtimeDiagnosticsEnabled())")) {
    const rest = trimmed.slice("if (runtimeDiagnosticsEnabled())".length).trimStart();
    if (rest.startsWith("{")) {
      let j = i + 1;
      let depth = 1;
      const block = [];
      while (j < lines.length && depth > 0) {
        const l = lines[j];
        for (const ch of l) {
          if (ch === "{") depth++;
          else if (ch === "}") depth--;
        }
        block.push(l);
        j++;
      }
      const inner = block.slice(0, -1);
      if (inner.every((l) => l.trim() === "" || isDebugConsoleLine(l) || /^\s*\/\//.test(l))) {
        i = j;
        continue;
      }
    } else if (isDebugConsoleLine(rest) || (rest.length === 0 && isDebugConsoleLine(lines[i + 1] ?? ""))) {
      i += 1;
      continue;
    }
  }

  if (isDebugConsoleLine(line)) {
    let bal = parenBalance(line);
    i++;
    while (bal > 0 && i < lines.length) {
      bal += parenBalance(lines[i]);
      i++;
    }
    continue;
  }

  if (/if \([^)]+\) console\.(log|debug|info|warn|error)\(/.test(line) && line.includes("[DEBUG]")) {
    i++;
    continue;
  }

  if (/^\s*if \([^)]+\)\s*$/.test(line) && isDebugConsoleLine(lines[i + 1] ?? "")) {
    let j = i + 1;
    let bal = parenBalance(lines[j]);
    j++;
    while (bal > 0 && j < lines.length) {
      bal += parenBalance(lines[j]);
      j++;
    }
    if ((lines[j] ?? "").trim().startsWith("else")) {
      const elseLine = lines[j].trim();
      if (elseLine.startsWith("else if")) {
        i = j;
        continue;
      }
      if (isDebugConsoleLine(lines[j + 1] ?? "")) {
        let k = j + 1;
        let bal2 = parenBalance(lines[k]);
        k++;
        while (bal2 > 0 && k < lines.length) {
          bal2 += parenBalance(lines[k]);
          k++;
        }
        i = k;
        continue;
      }
    }
    i = j;
    continue;
  }

  out.push(line);
  i++;
}

writeFileSync(path, out.join("\n"));
console.log(`[strip-shellhost] ${lines.length} -> ${out.length} lines`);
