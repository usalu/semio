import { dirname, resolve } from "node:path";
import { readFileSync } from "node:fs";

type Dependency = string | { workspace?: boolean; path?: string; package?: string };
type DependencyTables = {
  dependencies?: Record<string, Dependency>;
  "build-dependencies"?: Record<string, Dependency>;
  "dev-dependencies"?: Record<string, Dependency>;
};
type Manifest = DependencyTables & {
  package?: { name: string; workspace?: string };
  workspace?: { members: string[]; dependencies?: Record<string, Dependency> };
  target?: Record<string, DependencyTables>;
};

const manifests = new Map<string, Manifest>();
function read(path: string): Manifest {
  let manifest = manifests.get(path);
  if (!manifest) {
    manifest = Bun.TOML.parse(readFileSync(path, "utf8")) as Manifest;
    manifests.set(path, manifest);
  }
  return manifest;
}

function workspaceFor(path: string, manifest: Manifest): string {
  if (manifest.workspace) return path;
  if (manifest.package?.workspace) return resolve(dirname(path), manifest.package.workspace, "Cargo.toml");
  let folder = dirname(dirname(path));
  while (folder.startsWith(process.cwd())) {
    const candidate = resolve(folder, "Cargo.toml");
    try {
      if (read(candidate).workspace) return candidate;
    } catch (error) {
      if (!(error instanceof Error && "code" in error && error.code === "ENOENT")) throw error;
    }
    const parent = dirname(folder);
    if (parent === folder) break;
    folder = parent;
  }
  throw Error("Workspace manifest missing for " + path);
}

const members = new Set(process.argv.slice(2).map(file => resolve(file)).flatMap(path =>
  read(path).workspace?.members.map(member => resolve(dirname(path), member, "Cargo.toml")) ?? [path],
));
const pending = [...members], visited = new Set<string>();
const failures: string[] = [];

for (const path of pending) {
  if (visited.has(path)) continue;
  visited.add(path);
  const manifest = read(path);
  const kinds = members.has(path) ? ["dependencies", "build-dependencies", "dev-dependencies"] as const : ["dependencies", "build-dependencies"] as const;
  for (const table of [manifest, ...Object.values(manifest.target ?? {})]) {
    for (const kind of kinds) {
      for (const [name, declaration] of Object.entries(table[kind] ?? {})) {
        if (typeof declaration === "string") continue;
        try {
          const owner = declaration.workspace ? workspaceFor(path, manifest) : path;
          const dependency = declaration.workspace ? read(owner).workspace?.dependencies?.[name] : declaration;
          if (dependency === undefined) throw Error("missing workspace dependency in " + owner);
          if (typeof dependency === "string" || !dependency.path) continue;
          const file = resolve(dirname(owner), dependency.path, "Cargo.toml");
          const expected = dependency.package ?? name;
          if (read(file).package?.name !== expected) throw Error("expected package " + expected + " at " + file);
          pending.push(file);
        } catch (error) {
          failures.push(`${path}: ${kind}.${name}: ${(error as Error).message}`);
        }
      }
    }
  }
}

for (const failure of failures) console.error("[DEBUG] " + failure);
console.log(`[DEBUG] Inspected ${visited.size} package manifests with ${failures.length} unresolved path or workspace dependencies.`);
if (failures.length) process.exitCode = 1;
