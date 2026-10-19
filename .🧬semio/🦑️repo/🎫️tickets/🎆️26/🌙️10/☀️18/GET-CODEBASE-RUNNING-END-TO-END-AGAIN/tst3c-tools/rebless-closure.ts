import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio", frameworkWorkspace = "🧰️framework";
const owner = "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress";
const file = join(root, owner, "🧪️tests/🧫️fixtures/🔣️.json");
const raw = readFileSync(file, "utf8"), fixture = JSON.parse(raw);
const read = (path: string) => readFileSync(join(root, path), "utf8");
const workspace = Bun.TOML.parse(read(`${frameworkWorkspace}/Cargo.toml`)) as any, closure = new Map<string, any>();
const visit = (path: string) => {
  if (closure.has(path)) return;
  const data = Bun.TOML.parse(read(path)) as any, row: any = { package: data.package.name, manifest: path, edges: [] }; closure.set(path, row);
  const tables: [string, Record<string, any>][] = [["normal", data.dependencies ?? {}], ...Object.entries(data.target ?? {}).map(([target, value]) => [target, (value as any).dependencies ?? {}] as [string, Record<string, any>])];
  for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
    const inherited = typeof declaration === "object" && declaration.workspace === true, edge = inherited ? workspace.workspace.dependencies[name] : declaration;
    if (typeof edge !== "object" || !edge.path) continue;
    const target = relative(root, resolve(root, inherited ? frameworkWorkspace : dirname(path), edge.path, "Cargo.toml")).split("\\").join("/");
    row.edges.push({ scope, name, manifest: target }); visit(target);
  }
  row.edges.sort((a: any, b: any) => `${a.scope}:${a.name}`.localeCompare(`${b.scope}:${b.name}`));
};
visit(fixture.paths.manifest);
const next = [...closure.values()].sort((a, b) => a.manifest.localeCompare(b.manifest));
const same = JSON.stringify(next) === JSON.stringify(fixture.closure);
console.log("closure rows", next.length, "same", same);
const probe = JSON.stringify(fixture, null, 2) + "\n";
console.log("formatting preserved", probe === raw);
if (process.argv[2] === "apply" && !same && probe === raw) { fixture.closure = next; writeFileSync(file, JSON.stringify(fixture, null, 2) + "\n"); console.log("written"); }
const oldBy = new Map(fixture.closure.map((r: any) => [r.manifest, JSON.stringify(r)]));
for (const r of next) if (oldBy.get(r.manifest) !== JSON.stringify(r)) console.log("CHANGED/NEW", r.package, oldBy.has(r.manifest) ? "changed" : "new");
for (const m of oldBy.keys()) if (!closure.has(m as string)) console.log("REMOVED", m);
