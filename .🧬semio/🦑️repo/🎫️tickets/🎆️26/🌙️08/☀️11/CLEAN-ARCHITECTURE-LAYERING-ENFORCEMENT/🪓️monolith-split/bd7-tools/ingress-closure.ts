import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { parse as parseToml } from "@iarna/toml";
const repository = "/Users/ueli/Documents/semio";
const fixturePath = join(repository, "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/🔣️.json");
const text = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(text);
const trailing = text.endsWith("\n") ? "\n" : "";
if (JSON.stringify(fixture, null, 2) + trailing !== text) throw new Error("fixture does not round-trip");
const read = (path: string) => readFileSync(join(repository, path), "utf8");
const workspace = parseToml(read("Cargo.toml")) as any;
const closure = new Map<string, any>();
const visit = (path: string) => {
  if (closure.has(path)) return;
  const data = parseToml(read(path)) as any, row: any = { package: data.package.name, manifest: path, edges: [] };
  closure.set(path, row);
  const tables: [string, Record<string, any>][] = [["normal", data.dependencies ?? {}], ...Object.entries(data.target ?? {}).map(([target, value]) => [target, (value as any).dependencies ?? {}] as [string, Record<string, any>])];
  for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
    const inherited = typeof declaration === "object" && declaration.workspace === true, edge = inherited ? workspace.workspace.dependencies[name] : declaration;
    if (typeof edge !== "object" || !edge.path) continue;
    const target = relative(repository, resolve(repository, inherited ? "" : dirname(path), edge.path, "Cargo.toml")).split("\\").join("/");
    row.edges.push({ scope, name, manifest: target }); visit(target);
  }
  row.edges.sort((a: any, b: any) => `${a.scope}:${a.name}`.localeCompare(`${b.scope}:${b.name}`));
};
visit(fixture.paths.manifest);
const next = [...closure.values()].sort((a, b) => a.manifest.localeCompare(b.manifest));
const before = JSON.stringify(fixture.closure), after = JSON.stringify(next);
console.log("closure rows", fixture.closure.length, "->", next.length, "changed", before !== after);
if (process.argv[2] === "write" && before !== after) {
  fixture.closure = next;
  writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + trailing);
  console.log("written");
}
