/**
 * 📚️ Docstring repair: rewrites literal `\u{…}` / `\uXXXX` escapes and `¶` pilcrows inside Rust doc comment lines (`//!` and `///`)
 * under a directory into the real emoji and backticks they stand for, then lists any that remain.
 * Usage: `bun r6-z-mutations-docstrings.ts <dir>`; `--check` only reports.
 */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const check = process.argv.includes("--check");

export const decode = (line: string) =>
  line
    .replace(/\\u\{([0-9a-fA-F]+)\}/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/\\u([0-9a-fA-F]{4})/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replaceAll("¶", "`");

function files(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return files(path);
    return name.endsWith(".rs") ? [path] : [];
  });
}

if (import.meta.main) {
  let fixed = 0;
  for (const path of files(root)) {
    const text = readFileSync(path, "utf8");
    const eol = text.includes("\r\n") ? "\r\n" : "\n";
    const lines = text.split(eol);
    const next = lines.map((line) => (/^\s*\/\/[\/!]/.test(line) ? decode(line) : line));
    if (next.some((line, index) => line !== lines[index])) {
      fixed++;
      console.log(`${check ? "needs repair" : "repaired"}: ${path}`);
      if (!check) writeFileSync(path, next.join(eol));
    }
  }
  console.log(`${fixed} files`);
}
