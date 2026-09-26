/**
 * 🪟️ Z3 one-off audit probe: the TypeScript import closure of the `dev s`, local hub, hub, semio MCP and repo-cache command
 * entrypoints, scanned for POSIX-only assumptions. Prints `pattern<TAB>file:line<TAB>text` rows.
 * Usage: bun windows-closure-scan.ts <repo-root> <entry…>
 */
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import ts from "typescript";

const [root, ...entries] = process.argv.slice(2);
const seen = new Set<string>();
const queue = entries.map((entry) => resolve(root!, entry));
const candidates = (base: string): string[] => [base, `${base}.ts`, `${base}.tsx`, `${base}.mjs`, `${base}.js`, resolve(base, "🟦️.ts"), resolve(base, "🟦️.tsx"), resolve(base, "📜️script.ts")];
while (queue.length) {
  const file = queue.pop()!;
  if (seen.has(file) || !existsSync(file) || statSync(file).isDirectory()) continue;
  seen.add(file);
  const info = ts.preProcessFile(readFileSync(file, "utf8"), true, true);
  for (const reference of info.importedFiles) {
    if (!reference.fileName.startsWith(".")) continue;
    const target = candidates(resolve(dirname(file), reference.fileName)).find((path) => existsSync(path) && !statSync(path).isDirectory());
    if (target) queue.push(target);
  }
}
const PATTERNS: [string, RegExp][] = [
  ["shell", /["'`](?:sh|bash|zsh|\/bin\/sh|\/bin\/bash)["'`]|shell:\s*true|execSync\(|exec\(`/],
  ["tmp", /["'`]\/tmp\b|\/dev\/null|\/proc\//],
  ["signal-group", /process\.kill\(\s*-|kill\(-/],
  ["posix-tool", /["'`](?:lsof|ps|pgrep|pkill|kill|which|uname|tar|rsync|curl|open|xdg-open|nohup|setsid|sw_vers|sysctl|vm_stat|df|du|find|grep|sed|chmod|chown|ln|cp|mv|rm|mkdir|touch|cat|head|tail|env|id|whoami|launchctl|ditto|codesign|xattr|caffeinate)["'`]\s*[,)\]]/],
  ["chmod", /chmodSync|chmod\(|mode:\s*0o?[0-7]{3}|0o600|0o700|0o755/],
  ["symlink", /symlinkSync|symlink\(/],
  ["home", /process\.env\.HOME\b|process\.env\[["']HOME["']\]/],
  ["path-delimiter", /\.split\(["']:["']\)|\.join\(["']:["']\)/],
  ["url-pathname", /new URL\([^)]*import\.meta\.url[^)]*\)\.pathname|\.pathname\b.*import\.meta/],
  ["uid", /process\.getuid|process\.geteuid|process\.getgid/],
  ["unix-socket", /\.sock["'`]|socketPath|unix:/],
  ["exe", /["'`](?:os-hub|semio-os-mcp|os-mcp|semio-wgpu-native|wasm-opt|trunk|wasm-bindgen|jco)["'`]\s*\)/],
  ["signal-on", /process\.on\(["']SIG(?:HUP|USR1|USR2|QUIT|PIPE|CHLD|WINCH)["']/],
  ["crlf", /readFileSync\([^)]*\)\.split\(["']\\n["']\)|\.split\(["']\\n["']\)/],
  ["slash-split", /\.split\(["']\/["']\)|\.replace\(\/\\\/\/g|\.lastIndexOf\(["']\/["']\)/],
];
const rows: string[] = [];
for (const file of [...seen].sort()) {
  const lines = readFileSync(file, "utf8").split(/\r?\n/);
  lines.forEach((line, index) => {
    if (/^\s*(\/\/|\*|\/\*\*)/.test(line)) return;
    for (const [name, pattern] of PATTERNS) if (pattern.test(line)) rows.push(`${name}\t${relative(root!, file)}:${index + 1}\t${line.trim().slice(0, 220)}`);
  });
}
console.log(`# closure ${seen.size} files`);
console.log(rows.join("\n"));
