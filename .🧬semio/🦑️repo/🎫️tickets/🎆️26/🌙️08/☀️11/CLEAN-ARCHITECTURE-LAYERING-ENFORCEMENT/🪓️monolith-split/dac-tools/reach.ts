// 🧭️ Read-only reach graph: which package (lib name) compiles which Rust file, following #[path], implicit mod and include! mounts. Output: JSON file -> [{manifest, lib, kind, root}].
import {readFileSync, existsSync, writeFileSync} from "node:fs";
import {dirname, join, normalize, relative} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const OUT = process.argv[2]!;
const git = async (args: string[]) => (await Bun.$`git -c core.quotepath=false -c core.fsmonitor=false ${args}`.cwd(ROOT).text()).split("\n").filter(p => p && !p.startsWith(".🧬semio/"));
const tracked = await git(["ls-files", "--", "*Cargo.toml"]);
const untracked = await git(["ls-files", "--others", "--exclude-standard", "--", "*Cargo.toml"]);
const manifests = [...new Set([...tracked, ...untracked])].filter(p => existsSync(join(ROOT, p)));
type Root = {file: string; kind: string; pkgDir: string; manifest: string; lib: string};
const roots: Root[] = [];
for (const m of manifests) {
  let parsed: any;
  try { parsed = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")); } catch { continue; }
  if (!parsed.package) continue;
  const dir = dirname(join(ROOT, m));
  const lib = String(parsed.lib?.name ?? parsed.package.name).replace(/-/g, "_");
  const add = (rel: string | undefined, kind: string, fallback?: string) => {
    const r = rel ?? fallback;
    if (!r) return;
    const file = normalize(join(dir, r));
    if (existsSync(file)) roots.push({file, kind, pkgDir: dir, manifest: m, lib});
  };
  add(parsed.lib?.path, "lib", parsed.lib ? undefined : (existsSync(join(dir, "src/lib.rs")) ? "src/lib.rs" : undefined));
  for (const b of parsed.bin ?? []) add(b.path ?? `src/bin/${b.name}.rs`, "bin");
  for (const t of parsed.test ?? []) add(t.path ?? `tests/${t.name}.rs`, "test");
  if (!parsed.bin && existsSync(join(dir, "src/main.rs"))) roots.push({file: join(dir, "src/main.rs"), kind: "bin", pkgDir: dir, manifest: m, lib});
}
const stripComments = (text: string) => text.replace(/\/\*[\s\S]*?\*\//g, m => m.replace(/[^\n]/g, " ")).replace(/(^|[^:"])\/\/[^\n]*/g, (m, p) => p + " ".repeat(m.length - p.length));
const MOUNT = /((?:#\[[^\]\n]*\]\s*)*)(?:pub(?:\([^)\n]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;/g;
const PATH_ATTR = /#\[\s*path\s*=\s*"([^"]+)"\s*\]/;
const INCLUDE = /\binclude!\(\s*"([^"]+)"\s*\)/g;
const mountsOf = (file: string): string[] => {
  const raw = readFileSync(file, "utf8");
  const clean = stripComments(raw);
  const here = dirname(file);
  const out: string[] = [];
  for (const m of clean.matchAll(MOUNT)) {
    const pm = PATH_ATTR.exec(m[1] ?? "");
    let target: string | undefined;
    if (pm) target = normalize(join(here, pm[1]!));
    else {
      const stem = file.split("/").pop()!.replace(/\.rs$/, "");
      const base = stem === "mod" ? here : join(here, stem);
      for (const cand of [join(base, m[2] + ".rs"), join(base, m[2]!, "mod.rs")]) if (existsSync(cand)) { target = normalize(cand); break; }
      if (!target) for (const cand of [join(here, m[2] + ".rs"), join(here, m[2]!, "mod.rs")]) if (existsSync(cand)) { target = normalize(cand); break; }
    }
    if (target) out.push(target);
  }
  for (const m of clean.matchAll(INCLUDE)) out.push(normalize(join(here, m[1]!)));
  return out;
};
const owners = new Map<string, Map<string, {manifest: string; lib: string; kinds: Set<string>; root: string}>>();
const mountCache = new Map<string, string[]>();
for (const root of roots) {
  const seen = new Set<string>();
  const queue = [root.file];
  while (queue.length) {
    const f = queue.shift()!;
    if (seen.has(f)) continue;
    seen.add(f);
    const entry = owners.get(f) ?? new Map();
    owners.set(f, entry);
    const cur = entry.get(root.pkgDir + "|" + root.lib) ?? {manifest: root.manifest, lib: root.lib, kinds: new Set<string>(), root: relative(ROOT, root.file)};
    cur.kinds.add(root.kind);
    entry.set(root.pkgDir + "|" + root.lib, cur);
    let ms = mountCache.get(f);
    if (!ms) { ms = mountsOf(f); mountCache.set(f, ms); }
    for (const t of ms) if (existsSync(t)) queue.push(t);
  }
}
const result: Record<string, {manifest: string; lib: string; kinds: string[]; root: string}[]> = {};
for (const [f, entry] of owners) result[relative(ROOT, f)] = [...entry.values()].map(v => ({manifest: v.manifest, lib: v.lib, kinds: [...v.kinds], root: v.root}));
writeFileSync(OUT, JSON.stringify(result));
console.log("manifests", manifests.length, "roots", roots.length, "files", owners.size);
