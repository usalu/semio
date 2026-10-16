/** 🧭 Mirrors nativeDependencyRoots over every owner and lists closure roots lacking a canonical owner. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = process.cwd();
const reg = JSON.parse(readFileSync(join(root, "📋️build-owners.json"), "utf8"));
const byRoot = new Map<string, string>();
for (const o of reg.owners) for (const r of [o.root, ...(o.nativeRoots ?? [])]) byRoot.set(r, o.name);
const toml = new Map<string, any>();
const read = (p: string) => { if (!toml.has(p)) toml.set(p, Bun.TOML.parse(readFileSync(p, "utf8"))); return toml.get(p); };
const workspaceOf = (dir: string) => {
  const m = read(join(dir, "Cargo.toml"));
  let w = m.package?.workspace ? resolve(dir, m.package.workspace) : dir;
  while (w !== root && !(existsSync(join(w, "Cargo.toml")) && read(join(w, "Cargo.toml")).workspace)) w = dirname(w);
  return w;
};
const children = (dir: string, tests: boolean): string[] => {
  const m = read(join(dir, "Cargo.toml")), ws = workspaceOf(dir);
  const auth = existsSync(join(ws, "Cargo.toml")) ? read(join(ws, "Cargo.toml")).workspace ?? {} : {};
  const out: string[] = [];
  for (const t of [m, ...Object.values<any>(m.target ?? {})])
    for (const kind of ["dependencies", "build-dependencies", "dev-dependencies"]) {
      if (kind === "dev-dependencies" && !tests) continue;
      for (const [alias, d] of Object.entries<any>(t[kind] ?? {})) {
        if (typeof d !== "object") continue;
        const inh = d.workspace === true, dep = inh ? auth.dependencies?.[alias] : d;
        if (dep?.path) out.push(resolve(inh ? ws : dir, dep.path));
      }
    }
  return out;
};
const missing = new Map<string, Set<string>>();
const closureCache = new Map<string, Set<string>>();
const closure = (dir: string, seen = new Set<string>()) => {
  if (seen.has(dir)) return seen; seen.add(dir);
  if (!existsSync(join(dir, "Cargo.toml"))) return seen;
  for (const c of children(dir, false)) closure(c, seen);
  return seen;
};
for (const o of reg.owners) {
  for (const r of [o.root, ...(o.nativeRoots ?? [])]) {
    const abs = join(root, r);
    if (!existsSync(join(abs, "Cargo.toml"))) continue;
    const seen = new Set<string>([abs]);
    for (const c of children(abs, true)) closure(c, seen);
    for (const d of seen) {
      const rel = relative(root, d);
      if (!byRoot.has(rel)) { if (!missing.has(rel)) missing.set(rel, new Set()); missing.get(rel)!.add(o.name); }
    }
  }
}
console.log(JSON.stringify([...missing].sort().map(([k, v]) => ({ root: k, users: [...v].length, sample: [...v].slice(0, 3) })), null, 1));
