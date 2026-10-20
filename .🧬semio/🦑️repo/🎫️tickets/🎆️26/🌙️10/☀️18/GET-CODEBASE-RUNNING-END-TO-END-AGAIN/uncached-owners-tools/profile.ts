/** 🧪 Lexical host-resource profile of the files a test suite imports (transitively through relative static imports inside its own module directory): spawn, signals, timers, real storage roots, environment, network. `bun profile.ts <file>...` (workspace-relative). */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const PATTERNS: Record<string, RegExp> = {
  spawn: /\b(spawn|spawnSync|execFile|execFileSync|execSync|Bun\.spawn|Bun\.\$|runOwnedCommand|runRepositoryCommand|runCmd)\b/g,
  signal: /process\.kill|\.kill\(|SIGTERM|SIGKILL|SIGINT/g,
  timing: /performance\.now|Date\.now|setTimeout|Bun\.sleep|\bsleep\(/g,
  realStore: /\.cache\/b|⚡️cache|cacheRoot|toolingStore|repositoryCacheStore|withRepositoryCacheStore|productsStore|homedir\(\)|process\.env\.HOME|xdg|XDG_/g,
  disk: /hdiutil|diskutil|fixed-image|mount\b|statfs|df -|APFS|\.dmg|\.sparseimage/g,
  network: /fetch\(|https?:\/\/(?!localhost|127\.|example\.)|net\.connect|createServer|Bun\.serve|listen\(|WebSocket/g,
  platform: /process\.platform|os\.platform|process\.arch|os\.cpus|os\.totalmem|freemem/g,
  git: /\bgit\b/g,
  env: /process\.env\.\w+/g,
  tmp: /mkdtemp|tmpdir|SEMIO_TEST_ARTIFACT_DIR/g,
};
const files = new Set<string>(); const stack = process.argv.slice(2).map((f) => resolve(ROOT, f));
const importRe = /(?:from\s+|import\s*\(\s*|import\s+)["'](\.[^"']+)["']/g;
while (stack.length) {
  const f = stack.pop()!; if (files.has(f) || !existsSync(f) || !statSync(f).isFile() || files.size > 80) continue;
  files.add(f);
  const text = readFileSync(f, "utf8");
  if (process.env.PROFILE_DEPTH === "0") continue;
  for (const m of text.matchAll(importRe)) { const p = resolve(dirname(f), m[1]!); for (const c of [p, p + ".ts", join(p, "🟦️.ts")]) if (existsSync(c) && statSync(c).isFile()) { stack.push(c); break; } }
}
const total: Record<string, number> = {}; const per: string[] = [];
for (const f of files) {
  const text = readFileSync(f, "utf8").replace(/\/\*[\s\S]*?\*\/|(^|[^:])\/\/.*$/gm, "$1");
  const row: Record<string, number> = {};
  for (const [k, re] of Object.entries(PATTERNS)) { const n = (text.match(re) ?? []).length; if (n) { row[k] = n; total[k] = (total[k] ?? 0) + n; } }
  per.push(`${f.slice(ROOT.length + 1).split("/").slice(-3).join("/")} ${JSON.stringify(row)}`);
}
console.log(files.size, "files", JSON.stringify(total));
if (process.env.PROFILE_FILES) console.log(per.join("\n"));
