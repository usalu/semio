/** 🧪 Profiles the files each analysed target reaches (handler functions and the test files they run) for host-resource markers; writes `🗑️generated/uncached-owners/profile.json`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio", G = join(import.meta.dir, "../🗑️generated/uncached-owners");
const recs = JSON.parse(readFileSync(join(G, "records.json"), "utf8"));
const P: Record<string, RegExp> = {
  spawn: /\b(spawn|spawnSync|execFile|execFileSync|execSync|Bun\.spawn)\(/g,
  server: /Bun\.serve|createServer|\.listen\(|WebSocketServer/g,
  browser: /playwright|chromium|puppeteer|launchBrowser|page\.goto/g,
  docker: /\bdocker\b|docker-compose|compose\.ya?ml/g,
  net: /\bfetch\(|net\.connect|https?:\/\/(?!localhost|127\.0\.0\.1|example\.|schemas?\.|json-schema|www\.w3|github\.com\/semio)/g,
  gitw: /git["',\s]+(commit|add|checkout|stash|reset|push|pull|clone|fetch|config|init)\b/g,
  signal: /process\.kill\(|\.kill\(|SIGTERM|SIGKILL/g,
  sleep: /Bun\.sleep|setTimeout\(|setInterval\(/g,
  home: /homedir\(\)|process\.env\.HOME|\$HOME|XDG_/g,
  store: /\.cache\/b|⚡️cache|repositoryCacheStore|withRepositoryCacheStore|toolingStore|productsStore|ToolingStore/g,
  disk: /hdiutil|diskutil|statfs|\.dmg\b|sparseimage/g,
  repoRun: /["']repo["']\s*,\s*["'](run|run-many|dashboard)|bun repo run/g,
  tmp: /mkdtemp|tmpdir\(|SEMIO_TEST_ARTIFACT_DIR/g,
};
const out: Record<string, any> = {};
for (const r of recs) {
  const files = new Set<string>(); for (const o of r.evidence?.owned ?? []) files.add(o.split("#")[0]); for (const f of r.evidence?.files ?? []) files.add(f);
  const total: Record<string, number> = {};
  for (const f of files) {
    let text = ""; try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
    text = text.replace(/\/\*[\s\S]*?\*\/|(^|[^:])\/\/.*$/gm, "$1");
    for (const [k, re] of Object.entries(P)) { const n = (text.match(re) ?? []).length; if (n) total[k] = (total[k] ?? 0) + n; }
  }
  const ev = r.evidence;
  out[r.project + ":" + r.target] = { files: files.size, profile: total, strong: ev?.writes.filter((w: any) => w.strong).length ?? 0, other: ev?.writes.filter((w: any) => !w.strong).length ?? 0, infra: (ev?.infra ?? []).map((x: string) => x.split("@")[0]), flags: r.flags.filter((f: string) => !f.startsWith("strong-write")) };
}
writeFileSync(join(G, "profile.json"), JSON.stringify(out));
console.log(Object.keys(out).length);
