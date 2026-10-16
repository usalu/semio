// 🌳️ Read-only module-owner scan (pch copy of audit5-tools/modowners.ts; output file is argv[3]): which tracked Rust files are reached from more than one package (lib, bin and test roots, explicit #[path] and include! mounts).
import {readFileSync, existsSync, writeFileSync} from "node:fs";
import {dirname, join, normalize, relative, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const GEN = process.argv[2]!;
const list = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files -- '*Cargo.toml'`.cwd(ROOT).text()).split("\n").filter(p => p && !p.startsWith(".🧬semio/"));
const untracked = (await Bun.$`git --no-optional-locks -c core.quotepath=false ls-files --others --exclude-standard -- '*Cargo.toml'`.cwd(ROOT).text()).split("\n").filter(p => p && !p.startsWith(".🧬semio/"));
const manifests = [...new Set([...list, ...untracked])].filter(p => existsSync(join(ROOT, p)));
type Root = {file: string; kind: string; pkgDir: string; manifest: string};
const roots: Root[] = [];
for (const m of manifests) {
  const text = readFileSync(join(ROOT, m), "utf8");
  let parsed: any;
  try { parsed = Bun.TOML.parse(text); } catch { continue; }
  if (!parsed.package) continue;
  const dir = dirname(join(ROOT, m));
  const add = (rel: string | undefined, kind: string, fallback?: string) => {
    const r = rel ?? fallback;
    if (!r) return;
    const file = normalize(join(dir, r));
    if (existsSync(file)) roots.push({file, kind, pkgDir: dir, manifest: m});
  };
  add(parsed.lib?.path, "lib", parsed.lib ? undefined : (existsSync(join(dir, "src/lib.rs")) ? "src/lib.rs" : undefined));
  for (const b of parsed.bin ?? []) add(b.path ?? `src/bin/${b.name}.rs`, "bin");
  for (const t of parsed.test ?? []) add(t.path ?? `tests/${t.name}.rs`, "test");
  if (!parsed.bin && existsSync(join(dir, "src/main.rs"))) roots.push({file: join(dir, "src/main.rs"), kind: "bin", pkgDir: dir, manifest: m});
}
const CFG_TEST = /cfg\(\s*test\s*\)/;
const stripComments = (text: string) => text.replace(/\/\*[\s\S]*?\*\//g, m => m.replace(/[^\n]/g, " ")).replace(/(^|[^:"])\/\/[^\n]*/g, (m, p) => p + " ".repeat(m.length - p.length));
const MOUNT = /((?:#\[[^\]\n]*\]\s*)*)(?:pub(?:\([^)\n]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;/g;
const PATH_ATTR = /#\[\s*path\s*=\s*"([^"]+)"\s*\]/;
const INCLUDE = /\binclude!\(\s*"([^"]+)"\s*\)/g;
type Mount = {from: string; line: number; target: string; cfgTest: boolean; kind: string};
const mountsOf = (file: string): Mount[] => {
  const raw = readFileSync(file, "utf8");
  const clean = stripComments(raw);
  const here = dirname(file);
  const out: Mount[] = [];
  for (const m of clean.matchAll(MOUNT)) {
    const attrs = m[1] ?? "";
    const pm = PATH_ATTR.exec(attrs);
    const line = clean.slice(0, m.index! + (m[0].length - m[0].trimStart().length)).split("\n").length;
    let target: string | undefined;
    if (pm) target = normalize(join(here, pm[1]!));
    else {
      const stem = file.split("/").pop()!.replace(/\.rs$/, "");
      const base = stem === "mod" ? here : join(here, stem);
      for (const cand of [join(base, m[2] + ".rs"), join(base, m[2]!, "mod.rs")]) if (existsSync(cand)) { target = normalize(cand); break; }
      if (!target) for (const cand of [join(here, m[2] + ".rs"), join(here, m[2]!, "mod.rs")]) if (existsSync(cand)) { target = normalize(cand); break; }
    }
    if (target) out.push({from: file, line, target, cfgTest: CFG_TEST.test(attrs), kind: pm ? "path" : "implicit"});
  }
  for (const m of clean.matchAll(INCLUDE)) {
    out.push({from: file, line: clean.slice(0, m.index!).split("\n").length, target: normalize(join(here, m[1]!)), cfgTest: false, kind: "include"});
  }
  return out;
};
const sites = new Map<string, string[]>();
const owners = new Map<string, Map<string, {roots: Set<string>; mounts: Mount[]}>>();
const seenPerPkg = new Map<string, Set<string>>();
let rootCount = 0, fileCount = 0;
for (const root of roots) {
  rootCount++;
  const pkgSeen = seenPerPkg.get(root.pkgDir) ?? new Set<string>();
  seenPerPkg.set(root.pkgDir, pkgSeen);
  const queue = [root.file];
  while (queue.length) {
    const f = queue.shift()!;
    if (pkgSeen.has(f)) continue;
    pkgSeen.add(f);
    fileCount++;
    const entry = owners.get(f) ?? new Map();
    owners.set(f, entry);
    const cur = entry.get(root.pkgDir) ?? {roots: new Set<string>(), mounts: []};
    entry.set(root.pkgDir, cur);
    cur.roots.add(root.kind + ":" + relative(ROOT, root.file));
    for (const mt of mountsOf(f)) {
      if (existsSync(mt.target)) {
        queue.push(mt.target);
        const key = mt.target + "\u0000" + root.pkgDir;
        const list = sites.get(key) ?? [];
        if (list.length < 6) list.push(relative(ROOT, mt.from) + ":" + mt.line + (mt.cfgTest ? " [cfg(test)]" : "") + " " + mt.kind);
        sites.set(key, list);
      }
    }
  }
}
const rows: string[] = [];
let multi = 0, sameCrossTest = 0;
const multiByPkg = new Map<string, number>();
for (const [file, entry] of owners) {
  if (entry.size < 2) continue;
  multi++;
  const pkgs = [...entry.keys()].map(p => relative(ROOT, p) || ".");
  const key = pkgs.sort().join(" | ");
  multiByPkg.set(key, (multiByPkg.get(key) ?? 0) + 1);
  const siteText = [...entry.keys()].map(pk => relative(ROOT, pk) + "=" + (sites.get(file + "\u0000" + pk) ?? ["root"]).join(",")).join(" ; ");
  rows.push(relative(ROOT, file) + "\t" + pkgs.length + "\t" + pkgs.join(" ; ") + "\t" + siteText);
}
writeFileSync(join(GEN, process.argv[3]!), rows.sort().join("\n") + "\n");
console.log("manifests", manifests.length, "roots", rootCount, "reached file-package pairs", fileCount, "files owned by >1 package", multi);
const top = [...multiByPkg.entries()].sort((a, b) => b[1] - a[1]).slice(0, 25);
for (const [k, n] of top) console.log(n, k.slice(0, 400));
