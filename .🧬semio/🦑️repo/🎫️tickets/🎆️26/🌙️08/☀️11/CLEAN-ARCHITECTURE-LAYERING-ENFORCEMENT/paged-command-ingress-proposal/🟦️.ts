import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import Parser from "web-tree-sitter";

type Reader = (path: string) => string;
type ClosureRow = { package: string; manifest: string; edges: { scope: string; name: string; manifest: string }[] };
type Fixture = {
  version: number;
  owner: string;
  exports: { types: string[]; constants: string[] };
  regionDigest: string;
  laws: string[];
  lawDigests: Record<string, string>;
  fixture: { path: string; digest: string };
  closure: ClosureRow[];
  paths: { entry: string; manifest: string; neutralTests: string };
};

const root = resolve(import.meta.dir, "../../../../../../../../../.."), owner = "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress", digest = (value: string) => createHash("sha256").update(value).digest("hex");

/** 🏛️ Admits canonical owner bytes, dependency authority and exact moved native laws. */
export async function inspectPagedCommandIngressOwnership(repository: string, read: Reader = path => readFileSync(join(repository, path), "utf8")): Promise<{ types: number; constants: number; laws: number; closure: number }> {
  const fixture = JSON.parse(read(`${owner}/🧫️fixtures/🔣️.json`)) as Fixture, schema = JSON.parse(read(`${owner}/🧬️schema/🔣️.json`)), validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repository)), "out/tree-sitter-rust.wasm")));
  const trees: Parser.Tree[] = [];
  const ast = (path: string) => { const tree = parser.parse(read(path)); trees.push(tree); return tree.rootNode; };
  const nodes = (node: Parser.SyntaxNode): Parser.SyntaxNode[] => [node, ...node.namedChildren.flatMap(nodes)];
  const publicNames = (node: Parser.SyntaxNode, kinds: string[]) => node.namedChildren.filter(child => kinds.includes(child.type) && child.namedChildren.some(part => part.type === "visibility_modifier" && part.text === "pub")).map(child => child.childForFieldName("name")!.text);
  try {
    const component = read(`${owner}/🦀️.rs`), start = component.indexOf("//#region 🔖️PagedCommandIngress\n"), end = component.indexOf("//#endregion 🔖️PagedCommandIngress", start) + "//#endregion 🔖️PagedCommandIngress".length;
    expect(start).toBeGreaterThanOrEqual(0);
    expect(digest(component.slice(start, end).replaceAll("semio_framework_value::", "protocol::value::"))).toBe(fixture.regionDigest);
    const ownerAst = ast(`${owner}/🦀️.rs`);
    expect(ownerAst.hasError()).toBe(false);
    expect(publicNames(ownerAst, ["struct_item", "enum_item", "type_item"])).toEqual(fixture.exports.types);
    expect(publicNames(ownerAst, ["const_item"])).toEqual(fixture.exports.constants);
    const entry = read(fixture.paths.entry);
    expect(entry).toContain(`#[path = "../../📡️wire/🎮️command/📥️ingress/🦀️.rs"]\n    pub mod command_ingress;`);
    expect(component).toContain('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod unit_tests;');
    const workspace = parseToml(read("Cargo.toml")) as any, closure = new Map<string, ClosureRow>();
    const manifest = (path: string): any => { const source = read(path), first = parseToml(source), second = Bun.TOML.parse(source); expect<object>(first).toEqual(second); return first; };
    const visit = (path: string) => {
      if (closure.has(path)) return;
      const data = manifest(path), row: ClosureRow = { package: data.package.name, manifest: path, edges: [] }; closure.set(path, row);
      const tables: [string, Record<string, any>][] = [["normal", data.dependencies ?? {}], ...Object.entries(data.target ?? {}).map(([target, value]) => [target, (value as any).dependencies ?? {}] as [string, Record<string, any>])];
      for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
        const inherited = typeof declaration === "object" && declaration.workspace === true, edge = inherited ? workspace.workspace.dependencies[name] : declaration;
        expect(edge, `unresolved dependency ${path}:${name}`).toBeDefined();
        if (typeof edge !== "object" || !edge.path) { expect((edge.package ?? name).startsWith("semio-")).toBe(false); continue; }
        const target = relative(repository, resolve(repository, inherited ? "" : dirname(path), edge.path, "Cargo.toml")).split("\\").join("/");
        row.edges.push({ scope, name, manifest: target }); visit(target);
      }
      row.edges.sort((a, b) => `${a.scope}:${a.name}`.localeCompare(`${b.scope}:${b.name}`));
    };
    visit(fixture.paths.manifest);
    expect([...closure.values()].sort((a, b) => a.manifest.localeCompare(b.manifest))).toEqual(fixture.closure);
    expect([...closure.values()].filter(row => !row.manifest.startsWith("🧰️framework/🔨️modules/"))).toEqual([]);
    expect(workspace.workspace.members).toContain(dirname(fixture.paths.manifest));
    expect(manifest(fixture.paths.manifest).lib.name).toBe("protocol");
    const lawAst = ast(fixture.paths.neutralTests), functions = nodes(lawAst).filter(node => node.type === "function_item"), laws = functions.filter(node => fixture.laws.includes(node.childForFieldName("name")?.text ?? ""));
    expect(laws.map(node => node.childForFieldName("name")!.text)).toEqual(fixture.laws);
    for (const law of laws) expect(digest(law.text)).toBe(fixture.lawDigests[law.childForFieldName("name")!.text]);
    const includes = nodes(lawAst).filter(node => node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "include_str");
    expect(includes).toHaveLength(1);
    const literal = includes[0]!.namedChildren.find(node => node.type === "token_tree")!.namedChildren[0]!.text;
    expect(relative(repository, resolve(repository, dirname(fixture.paths.neutralTests), JSON.parse(literal))).split("\\").join("/")).toBe(fixture.fixture.path);
    expect(digest(read(fixture.fixture.path))).toBe(fixture.fixture.digest);
    return { types: fixture.exports.types.length, constants: fixture.exports.constants.length, laws: fixture.laws.length, closure: closure.size };
  } finally { for (const tree of trees) tree.delete(); parser.delete(); }
}

test("the complete paged command owner and original native laws admit only their canonical neutral authority", async () => {
  const result = await inspectPagedCommandIngressOwnership(root);
  expect(result).toEqual({ types: 14, constants: 5, laws: 4, closure: JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")).closure.length });
});

test("closed ownership corpus rejects duplicate, omitted and hostile source authority", () => {
  const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")), validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(root, owner, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(fixture)).toBe(true);
  const changes = [(value: any) => value.exports.types.push(value.exports.types[0]), (value: any) => value.exports.types.pop(), (value: any) => value.exports.constants.reverse(), (value: any) => value.laws.pop(), (value: any) => value.owner = "os/channel", (value: any) => value.paths.neutralTests = value.owner, (value: any) => value.extra = true];
  for (const change of changes) { const hostile = structuredClone(fixture); change(hostile); expect(validate(hostile)).toBe(false); }
});

test("canonical package membership and executable routes retain the complete native cohort", () => {
  const pkg = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust", workspace = JSON.parse(readFileSync(join(root, "package.json"), "utf8")), project = JSON.parse(readFileSync(join(root, pkg, "📋️project.json"), "utf8")), registration = JSON.parse(readFileSync(join(root, pkg, "package.json"), "utf8")), script = readFileSync(join(root, pkg, "📜️script.ts"), "utf8");
  expect(workspace.workspaces.some((pattern: string) => !pattern.startsWith("!") && new Bun.Glob(pattern).match(pkg))).toBe(true);
  expect(workspace.workspaces.some((pattern: string) => pattern.startsWith("!") && new Bun.Glob(pattern.slice(1)).match(pkg))).toBe(false);
  expect(registration.name).toBe(project.name);
  expect(registration.nx.includedScripts).toEqual([]);
  for (const gate of ["test-command-ingress-ownership", "test-command-ingress-native"]) {
    expect(project.targets[gate].options.command).toBe(`bun ./📜️script.ts ${gate}`);
    expect(registration.scripts[gate]).toBe(`nx run ${project.name}:${gate}`);
    expect(project.targets[gate].cache).toBe(false);
    const inputs = project.targets[gate].inputs as string[];
    expect(inputs).toContain(`{workspaceRoot}/${owner}/**/*`);
    expect(inputs).toContain("{workspaceRoot}/🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json");
    expect(inputs).toContain("{workspaceRoot}/Cargo.toml");
    expect(inputs).toContain("{workspaceRoot}/package.json");
    expect(inputs.filter(value => value.includes("🛍️products/") || value.includes("🔨️modules/🎠️kernel/") || value.includes("🧰️framework/📦️packages/"))).toEqual([]);
  }
  expect(script).toContain('extraArgs:["--lib","--no-fail-fast"]');
  expect(script).toContain('packages:["semio-framework-replication"]');
  expect(script).toContain('if(rest.length)throw Error("test-command-ingress-native accepts only an execution level")');
});
