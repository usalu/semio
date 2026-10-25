import { readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { execSync } from "node:child_process";
const files = execSync("git ls-files -co --exclude-standard -- '*Cargo.toml'", { encoding: "utf8", maxBuffer: 1 << 28 }).split("\n").filter(Boolean);
const pkgs = new Map<string, { dir: string; text: string }>();
const byDir = new Map<string, string>();
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  const m = /\[package\][^\[]*?\bname\s*=\s*"([^"]+)"/s.exec(t);
  if (m) { pkgs.set(m[1]!, { dir: dirname(f), text: t }); byDir.set(dirname(f), m[1]!); }
}
// workspace.dependencies path map
const wsPaths = new Map<string, string>();
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  const sec = /\[workspace\.dependencies\]([\s\S]*?)(?:\n\[|$)/.exec(t);
  if (!sec) continue;
  for (const line of sec[1]!.split("\n")) {
    const m = /^([A-Za-z0-9_\-]+)\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"/.exec(line.trim());
    if (m) wsPaths.set(m[1]!, normalize(join(dirname(f), m[2]!)));
  }
}
const prefixes = process.argv.slice(2);
const roots = [...pkgs].filter(([, v]) => prefixes.some((x) => v.dir.startsWith(x))).map(([k]) => k);
const seen = new Set<string>();
const stack = roots.filter((r) => pkgs.has(r));

while (stack.length) {
  const n = stack.pop()!;
  if (seen.has(n)) continue;
  seen.add(n);
  const { dir, text } = pkgs.get(n)!;
  const deps = text.split(/\n\[/).filter((s) => /^(?:dependencies|dev-dependencies|build-dependencies|target\.[^\]]+\.dependencies)\]/.test(s));
  for (const sec of deps) for (const line of sec.split("\n").slice(1)) {
    const m = /^([A-Za-z0-9_\-]+)\s*=\s*(.*)$/.exec(line.trim());
    if (!m) continue;
    const name = m[1]!;
    const pm = /package\s*=\s*"([^"]+)"/.exec(m[2]!);
    const real = pm ? pm[1]! : name;
    const p = /path\s*=\s*"([^"]+)"/.exec(m[2]!);
    if (p) { const d = normalize(join(dir, p[1]!)); const pn = byDir.get(d); if (pn) stack.push(pn); }
    else if (/workspace\s*=\s*true/.test(m[2]!) && wsPaths.has(name)) { const pn = byDir.get(wsPaths.get(name)!); if (pn) stack.push(pn); }
    else if (pkgs.has(real) && /^semio/.test(real)) stack.push(real);
  }
}
const dirs = [...seen].map((n) => pkgs.get(n)!.dir);
console.log(JSON.stringify(dirs));
