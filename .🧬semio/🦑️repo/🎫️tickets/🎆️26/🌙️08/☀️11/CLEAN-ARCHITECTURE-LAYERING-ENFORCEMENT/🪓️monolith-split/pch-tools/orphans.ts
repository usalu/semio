// 🕸️ Read-only: every [package] manifest under a path prefix must be a workspace member (cargo semantics), a workspace root itself, and covered by the lock of its workspace.
import {readFileSync, existsSync, readdirSync, statSync} from "node:fs";
import {dirname, join, relative} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const prefix = process.argv[2]!;
const outFile = process.argv[3]!;
const tracked = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files -- ${prefix}`.cwd(ROOT).text()).split("\n").filter(Boolean);
const untracked = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files --others --exclude-standard -- ${prefix}`.cwd(ROOT).text()).split("\n").filter(Boolean);
const files = [...new Set([...tracked, ...untracked])].filter(p => p.endsWith("/Cargo.toml") && existsSync(join(ROOT, p)));
const toml = (p: string) => Bun.TOML.parse(readFileSync(join(ROOT, p), "utf8")) as any;
const expand = (rootDir: string, pat: string): string[] => {
  if (!/[*?\[]/.test(pat)) return [pat.replace(/\/$/, "")];
  const out: string[] = [];
  const glob = new Bun.Glob(pat);
  const walk = (dir: string, depth: number) => {
    if (depth > 14) return;
    let ents: string[] = [];
    try { ents = readdirSync(join(ROOT, rootDir, dir)); } catch { return; }
    for (const e of ents) {
      const rel = dir ? `${dir}/${e}` : e;
      let st; try { st = statSync(join(ROOT, rootDir, rel)); } catch { continue; }
      if (!st.isDirectory()) continue;
      if (glob.match(rel)) out.push(rel);
      walk(rel, depth + 1);
    }
  };
  walk("", 0);
  return out;
};
const wsCache = new Map<string, any>();
const wsOf = (dir: string) => {
  if (wsCache.has(dir)) return wsCache.get(dir);
  const m = join(dir, "Cargo.toml");
  let v: any = null;
  if (existsSync(join(ROOT, m))) {
    const t = toml(m);
    if (t.workspace) {
      const members = new Set<string>();
      for (const p of t.workspace.members ?? []) for (const e of expand(dir, p)) members.add(e);
      v = {dir, manifest: m, members, exclude: t.workspace.exclude ?? [], hasPackage: !!t.package};
    }
  }
  wsCache.set(dir, v);
  return v;
};
const lockCache = new Map<string, Set<string> | null>();
const lockNames = (dir: string) => {
  if (lockCache.has(dir)) return lockCache.get(dir)!;
  const p = join(dir, "Cargo.lock");
  let s: Set<string> | null = null;
  if (existsSync(join(ROOT, p))) {
    s = new Set();
    for (const m of readFileSync(join(ROOT, p), "utf8").matchAll(/^name = "([^"]+)"/gm)) s.add(m[1]!);
  }
  lockCache.set(dir, s);
  return s;
};
const rows: string[] = [];
const count: Record<string, number> = {};
const bump = (k: string) => { count[k] = (count[k] ?? 0) + 1; };
for (const m of files) {
  const t = toml(m);
  if (!t.package) continue;
  const pdir = dirname(m);
  const name = t.package.name as string;
  let verdict = "";
  let wsManifest = "";
  if (t.workspace) { verdict = "own-workspace-root"; wsManifest = m; }
  else {
    let cur = dirname(pdir);
    while (true) {
      const ws = wsOf(cur);
      if (ws) {
        wsManifest = ws.manifest;
        const rel = relative(cur, pdir);
        if (ws.members.has(rel)) verdict = "member";
        else {
          const excluded = ws.exclude.some((ex: string) => rel === ex || rel.startsWith(ex.replace(/\/$/, "") + "/"));
          verdict = excluded ? "excluded-not-member" : "not-listed";
        }
        break;
      }
      const up = dirname(cur);
      if (up === cur || up === ".") { verdict = "no-workspace"; break; }
      cur = up;
    }
  }
  let lock = "";
  if (verdict === "member") {
    const names = lockNames(dirname(wsManifest));
    lock = names === null ? "no-lock" : names.has(name) ? "in-lock" : "NOT-IN-LOCK";
  }
  bump(verdict + (lock ? "/" + lock : ""));
  rows.push([m, name, verdict, lock, wsManifest].join("\t"));
}
await Bun.write(outFile, rows.join("\n") + "\n");
console.log(JSON.stringify(count, null, 1));
