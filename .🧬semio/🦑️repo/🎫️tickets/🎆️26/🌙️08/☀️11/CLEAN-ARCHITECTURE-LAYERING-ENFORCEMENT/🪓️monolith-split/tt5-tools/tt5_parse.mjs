// 🔎 Parse sweep: every tracked or untracked-not-ignored .ts/.tsx/.mts/.cts file, TypeScript createSourceFile parse diagnostics, `path:line:col TSxxxx message`.
import { readFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const require = createRequire(root + "/package.json");
const ts = require("typescript");
const git = spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*.ts", "*.tsx", "*.mts", "*.cts"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 29 });
const files = [...new Set(git.stdout.split("\0").filter(Boolean))].filter((f) => existsSync(root + "/" + f));
let count = 0, bad = 0;
for (const file of files) {
  let text;
  try { text = readFileSync(root + "/" + file, "utf8"); } catch { continue; }
  const kind = file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, false, kind);
  count++;
  if (source.parseDiagnostics.length > 0) {
    bad++;
    for (const d of source.parseDiagnostics) {
      const { line, character } = source.getLineAndCharacterOfPosition(d.start ?? 0);
      console.log(`${file}:${line + 1}:${character + 1} TS${d.code} ${ts.flattenDiagnosticMessageText(d.messageText, " ")}`);
    }
  }
}
console.error(`parsed=${count} files-with-errors=${bad}`);
