// 🧮️ Prints the dependency closure rows of the command-ingress crate in the shape of its ownership fixture.
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { parse as parseToml } from "@iarna/toml";
const repository = "/Users/ueli/Documents/semio";
const read = (path: string) => readFileSync(join(repository, path), "utf8");
const workspace = parseToml(read("Cargo.toml")) as any, closure = new Map<string, any>();
const manifest = (path: string): any => parseToml(read(path));
const visit = (path: string) => {
  if (closure.has(path)) return;
  const data = manifest(path), row: any = { package: data.package.name, manifest: path, edges: [] }; closure.set(path, row);
  const tables: [string, Record<string, any>][] = [["normal", data.dependencies ?? {}], ...Object.entries(data.target ?? {}).map(([target, value]) => [target, (value as any).dependencies ?? {}] as [string, Record<string, any>])];
  for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
    const inherited = typeof declaration === "object" && declaration.workspace === true, edge = inherited ? workspace.workspace.dependencies[name] : declaration;
    if (typeof edge !== "object" || !edge.path) continue;
    const target = relative(repository, resolve(repository, inherited ? "" : dirname(path), edge.path, "Cargo.toml")).split("\\").join("/");
    row.edges.push({ scope, name, manifest: target }); visit(target);
  }
  row.edges.sort((a: any, b: any) => `${a.scope}:${a.name}`.localeCompare(`${b.scope}:${b.name}`));
};
visit(process.argv[2]!);
console.log(JSON.stringify([...closure.values()].sort((a, b) => a.manifest.localeCompare(b.manifest))));
