import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve, basename } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const parse = (p: string) => Bun.TOML.parse(readFileSync(p, "utf8")) as any;
const wsPath = join(repo, "🎓️teaching/Cargo.toml");
const ws = parse(wsPath);
const wsDeps = ws.workspace.dependencies ?? {};
function findWorkspaceRoot(manifest: string, pkg: any): string | null {
  if (pkg.package?.workspace) return resolve(dirname(manifest), pkg.package.workspace, "Cargo.toml");
  let d = dirname(dirname(manifest));
  for (;;) { const c = join(d, "Cargo.toml"); if (existsSync(c) && parse(c).workspace) return c; const n = dirname(d); if (n === d) return null; d = n; }
}
const seen = new Map<string, any>();
const optionalHits: string[] = [];
const kinds = new Map<string,string[]>();
function visit(manifest: string, via: string) {
  if (seen.has(manifest)) return;
  const pkg = parse(manifest); seen.set(manifest, pkg);
  const root = pkg.workspace && !pkg.package ? manifest : findWorkspaceRoot(manifest, pkg);
  const rootDeps = root ? (parse(root).workspace?.dependencies ?? {}) : {};
  const tables: [string, any][] = [["dependencies", pkg.dependencies], ["build-dependencies", pkg["build-dependencies"]]];
  for (const [t, cfg] of Object.entries<any>(pkg.target ?? {})) { tables.push([`target.${t}.dependencies`, cfg.dependencies]); tables.push([`target.${t}.build-dependencies`, cfg["build-dependencies"]]); }
  for (const [kind, table] of tables) for (const [name, spec0] of Object.entries<any>(table ?? {})) {
    let spec = spec0; let base = dirname(manifest);
    if (typeof spec === "object" && spec.workspace) { const inh = rootDeps[name]; spec = { ...(typeof inh === "object" ? inh : { version: inh }), ...spec }; base = dirname(root!); if (spec.path) spec.path = spec.path; }
    if (typeof spec !== "object" || !spec.path) continue;
    if (spec.optional) optionalHits.push(`${basename(dirname(manifest))}:${pkg.package?.name} -> ${name} (${kind})`);
    const target = join(spec0.workspace ? dirname(root!) : dirname(manifest), spec.path, "Cargo.toml");
    visit(resolve(target), `${pkg.package?.name}`);
  }
}
const members: string[] = ws.workspace.members;
for (const m of members) { const mf = join(dirname(wsPath), m, "Cargo.toml"); if (parse(mf).package.name === "teaching-proctor") visit(mf, "root"); }
console.log(seen.size, "packages");
console.log("optional path deps:", optionalHits.length); for (const o of optionalHits.slice(0,30)) console.log(" ", o);
const dirs = [...seen.keys()].map(m => dirname(m).replace(repo + "/", "")).sort();
console.log(dirs.slice(0,15).join("\n"));
