import { existsSync, readFileSync } from "node:fs";
import { resolve, join, relative, dirname } from "node:path";
import * as toml from "@iarna/toml";
const root = process.cwd();
const packagePath = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust";
const members = new Set<string>();
const visit = (path: string): void => {
  const member = relative(root, path).replaceAll("\\", "/");
  if (members.has(member)) return;
  members.add(member);
  const manifest = toml.parse(readFileSync(join(path, "Cargo.toml"), "utf8")) as any;
  const source = toml.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as any;
  const sections = [manifest, ...Object.values(manifest.target ?? {})] as any[];
  for (const section of sections) for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) for (const [name, declaration] of Object.entries(section[kind] ?? {}) as [string, any][]) {
    const inherited = declaration.workspace ? source.workspace.dependencies[name] : undefined;
    const dependency = inherited ?? declaration;
    if (dependency.path) visit(resolve(inherited ? root : path, dependency.path));
  }
};
visit(join(root, packagePath));
const owners = [...members].map(m => m.slice(0, m.indexOf("/📦️packages/")));
console.log([...members].join("\n"));
const seen = new Set<string>();
const visitSource = (path: string, from: string): void => {
  if (seen.has(path) || !existsSync(path)) return;
  seen.add(path);
  const owned = relative(root, path).replaceAll("\\", "/");
  if (owned.includes("/🧪️tests/") && !owned.startsWith("🧰️framework/🔨️modules/🧬️schema/✅️validator/")) return;
  if (!owners.some(o => owned.startsWith(o + "/"))) { console.log("PUSH", owned, "from", from); owners.push(owned.split("/").slice(0, 3).join("/")); }
  if (!path.endsWith(".rs")) return;
  const body = readFileSync(path, "utf8");
  for (const match of body.matchAll(/(?:#\[\s*path\s*=\s*|include(?:_str|_bytes)?!\s*\(\s*)"([^"\n]+)"/gu)) visitSource(resolve(dirname(path), match[1]!), owned);
};
for (const member of members) { const path = join(root, member); const manifest = toml.parse(readFileSync(join(path, "Cargo.toml"), "utf8")) as any; visitSource(resolve(path, manifest.lib?.path ?? "src/lib.rs"), member); }
