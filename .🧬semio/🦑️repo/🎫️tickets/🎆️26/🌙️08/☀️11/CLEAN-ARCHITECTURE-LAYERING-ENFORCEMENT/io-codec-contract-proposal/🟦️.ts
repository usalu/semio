import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import Parser from "web-tree-sitter";
import { parse as parseGraphql } from "graphql";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🔣️.json";
import { confidenceRank, parseConfidence } from "../../../🧬️schema/🟦️.ts";

const owner = resolve(import.meta.dir, "../.."), pkg = join(owner, "📦️packages/🦀️rust");
const workspace = resolve(pkg, "../../../../../.."), portablePath = (path: string) => relative(workspace, path).split("\\").join("/");

test("closed neutral codec vectors have independent schema admission and finite expected outputs", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const ajv = new Ajv({ strict: true, $data: true });
  const span = ajv.compile({ type: "object", additionalProperties: false, required: ["resource", "byte_start", "byte_end", "line", "column"], properties: { resource: { type: "string", pattern: "\\S" }, byte_start: { type: "integer", minimum: 0 }, byte_end: { type: "integer", minimum: { $data: "1/byte_start" } }, line: { type: ["integer", "null"], minimum: 1, maximum: 4294967295 }, column: { type: ["integer", "null"], minimum: 1, maximum: 4294967295 } }, oneOf: [{ properties: { line: { type: "null" }, column: { type: "null" } } }, { properties: { line: { type: "integer" }, column: { type: "integer" } } }] });
  for (const row of fixture.spans) expect(Boolean(span(row.value)), row.id).toBe(row.valid);
  const ceiling = ajv.compile({ type: "object", required: ["limit", "next"], properties: { limit: { type: "integer", minimum: 0 }, next: { type: "integer", minimum: 0, maximum: { $data: "1/limit" } } } });
  for (const row of fixture.budgets) {
    expect(Boolean(ceiling({ limit: row.limit, next: row.used + row.increment })), row.id).toBe(row.valid);
    expect(row.consumed, row.id).toBe(row.valid ? row.used + row.increment : row.used);
  }
  console.log("[DEBUG] independent Ajv: closed codec corpus, source ranges and four finite resource counters");
});

test("actual canonical confidence parser and ranks match the four-variant portable corpus", () => {
  const graphql = parseGraphql(readFileSync(resolve(owner, "../🧬️schema/🔗️.graphql"), "utf8")), declaration = graphql.definitions.find(node => node.kind === "EnumTypeDefinition" && node.name.value === "Confidence");
  expect(declaration?.kind).toBe("EnumTypeDefinition");
  if (declaration?.kind !== "EnumTypeDefinition") throw Error("canonical GraphQL Confidence declaration absent");
  expect(declaration.values!.map(row => row.name.value)).toEqual(fixture.confidence.map(row => row.value));
  const oracle = new Ajv({ strict: true }).compile({ oneOf: fixture.confidence.map(row => ({ type: "object", required: ["value", "rank", "identified"], properties: { value: { const: row.value }, rank: { const: row.rank }, identified: { const: row.identified } }, additionalProperties: false })) });
  for (const row of fixture.confidence) {
    const value = parseConfidence(row.value), rank = confidenceRank(value), identified = value !== "None";
    expect(oracle({ value, rank, identified }), row.value).toBe(true);
  }
  for (const value of ["", "none", "Unknown", 0, null, {}, []]) expect(() => parseConfidence(value)).toThrow();
  console.log("[DEBUG] canonical confidence runtime: None/Low/Medium/High exact ranks; explicit None exclusion; independent Ajv output equality");
});

test("actual neutral entry and manifest expose exactly the owned contracts without an OS normal edge", async () => {
  const source = readFileSync(join(pkg, "Cargo.toml"), "utf8"), manifest = TOML.parse(source) as { dependencies: Record<string, { path: string }>; package: { name: string } };
  expect(Bun.TOML.parse(source)).toEqual(manifest);
  expect(manifest.package.name).toBe(fixture.package);
  expect(Object.keys(manifest.dependencies)).toEqual(fixture.normalDependencies);
  expect(resolve(pkg, manifest.dependencies["semio-framework-io-schema"].path)).toBe(resolve(owner, "../🧬️schema/📦️packages/🦀️rust"));
  const rootManifest = TOML.parse(readFileSync(join(workspace, "Cargo.toml"), "utf8")) as any, normal = new Map<string, any>();
  const visit = (path: string) => {
    const key = portablePath(path);
    if (normal.has(key)) return;
    expect(key.startsWith("../") || key.startsWith("/"), key).toBe(false);
    const bytes = readFileSync(path, "utf8"), parsed = TOML.parse(bytes) as any;
    expect(Bun.TOML.parse(bytes), key).toEqual(parsed);
    const edges: { scope: string; name: string; package: string; manifest: string | null }[] = [];
    normal.set(key, { package: parsed.package.name, manifest: key, edges });
    const tables = [["normal", parsed.dependencies ?? {}], ...Object.entries(parsed.target ?? {}).map(([target, table]: [string, any]) => [target, table.dependencies ?? {}])];
    for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
      const inherited = typeof declaration === "object" && (declaration as any).workspace === true;
      const data: any = inherited ? rootManifest.workspace.dependencies[name] : declaration;
      expect(data, name).toBeDefined();
      const target = typeof data === "object" && data.path ? resolve(inherited ? workspace : dirname(path), data.path, "Cargo.toml") : null;
      expect(target !== null || !name.startsWith("semio-"), name).toBe(true);
      const packageName = target ? (TOML.parse(readFileSync(target, "utf8")) as any).package.name : data.package ?? name;
      if (typeof data === "object" && data.package) expect(packageName, name).toBe(data.package);
      edges.push({ scope: scope as string, name, package: packageName, manifest: target ? portablePath(target) : null });
      if (target) visit(target);
    }
    edges.sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)));
  };
  visit(join(pkg, "Cargo.toml"));
  expect([...normal.values()].sort((a, b) => a.manifest.localeCompare(b.manifest))).toEqual(fixture.authority.normalClosure);
  expect(rootManifest.workspace.members.filter((path: string) => path === fixture.authority.cargoMember)).toEqual([fixture.authority.cargoMember]);
  const bunWorkspace = JSON.parse(readFileSync(join(workspace, "package.json"), "utf8"));
  const match = (pattern: string) => new Bun.Glob(pattern.replace(/^!/, "")).match(fixture.authority.cargoMember);
  expect(bunWorkspace.workspaces.some((pattern: string) => !pattern.startsWith("!") && match(pattern))).toBe(true);
  expect(bunWorkspace.workspaces.some((pattern: string) => pattern.startsWith("!") && match(pattern))).toBe(false);
  const project = JSON.parse(readFileSync(join(pkg, "📋️project.json"), "utf8")), packageJson = JSON.parse(readFileSync(join(pkg, "package.json"), "utf8"));
  expect(project.name).toBe(fixture.authority.project); expect(packageJson.name).toBe(project.name);
  expect(project.sourceRoot).toBe(portablePath(owner));
  for (const command of ["test-ownership", "test-native"]) {
    expect(project.targets[command].options.cwd).toBe(fixture.authority.cargoMember);
    expect(project.targets[command].options.command).toBe(`bun ./📜️script.ts ${command}`);
    expect(packageJson.scripts[command]).toBe(`bun nx run ${project.name}:${command}`);
  }
  await Parser.init(); const parser = new Parser(); parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", pkg)), "out/tree-sitter-rust.wasm")));
  for (const mount of fixture.authority.mounts) {
    const path = join(workspace, mount.source), mounted = parser.parse(readFileSync(path, "utf8")), items = mounted.rootNode.namedChildren;
    const index = items.findIndex(node => node.type === "mod_item" && node.childForFieldName("name")?.text === mount.module);
    expect(index, mount.module).toBeGreaterThan(-1);
    const attributes: Parser.SyntaxNode[] = [];
    for (let at = index - 1; at >= 0 && items[at].type === "attribute_item"; at--) attributes.unshift(items[at]);
    const attribute = attributes.find(node => node.text.startsWith("#[path"));
    expect(attribute, mount.module).toBeDefined();
    const literals: string[] = [];
    const strings = (node: Parser.SyntaxNode) => { if (node.type === "string_literal") literals.push(JSON.parse(node.text)); for (const child of node.namedChildren) strings(child); };
    strings(attribute!); expect(literals.length).toBe(1);
    expect(portablePath(resolve(dirname(path), literals[0]))).toBe(mount.target);
    expect(readFileSync(join(workspace, mount.target), "utf8").length).toBeGreaterThan(0);
  }
  const owned = parser.parse(readFileSync(join(owner, "🦀️.rs"), "utf8"));
  expect(owned.rootNode.hasError()).toBe(false);
  const names = owned.rootNode.namedChildren.filter(node => ["struct_item", "enum_item", "trait_item", "type_item"].includes(node.type) && node.children.some(child => child.type === "visibility_modifier" && child.text === "pub")).map(node => node.childForFieldName("name")!.text).sort();
  expect(names).toEqual([...fixture.exports].sort());
  const retired = parser.parse(readFileSync(resolve(owner, "../🦀️.rs"), "utf8"));
  expect(retired.rootNode.hasError()).toBe(false);
  expect(retired.rootNode.namedChildren.filter(node => ["struct_item", "enum_item", "trait_item", "type_item"].includes(node.type)).map(node => node.childForFieldName("name")?.text).filter(name => name === "Confidence" || fixture.exports.includes(name!))).toEqual([]);
  const entry = readFileSync(join(pkg, "🦀️.rs"), "utf8"), tree = parser.parse(entry);
  expect(tree.rootNode.hasError()).toBe(false);
  const exports = tree.rootNode.namedChildren.filter(node => node.type === "use_declaration" && node.text.startsWith("pub use contracts::"));
  expect(exports.length).toBe(1); expect(exports[0].text.includes("*")).toBe(false);
  for (const name of fixture.exports) expect(new RegExp(`\\b${name}\\b`).test(exports[0].text), name).toBe(true);
  const schemaPath = resolve(owner, "../🧬️schema/🦀️.rs"), schemaTree = parser.parse(readFileSync(schemaPath, "utf8"));
  const declarations = schemaTree.rootNode.namedChildren;
  const registry = declarations.find(node => node.type === "const_item" && node.childForFieldName("name")?.text === fixture.authority.registry.constant);
  expect(registry).toBeDefined();
  const leaves: string[] = [];
  const collect = (node: Parser.SyntaxNode) => {
    if (node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "include_str") {
      const tokens = node.namedChildren.find(child => child.type === "token_tree")?.namedChildren;
      expect(tokens?.length).toBe(1); expect(tokens?.[0].type).toBe("string_literal");
      leaves.push(portablePath(resolve(dirname(schemaPath), JSON.parse(tokens![0].text))));
    }
    for (const child of node.namedChildren) collect(child);
  };
  collect(registry!);
  expect(leaves).toEqual(fixture.authority.registry.leaves);
  for (const leaf of leaves) expect(readFileSync(join(workspace, leaf), "utf8").length, leaf).toBeGreaterThan(0);
  const registration = declarations.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === fixture.authority.registry.function);
  expect(registration?.text.includes(fixture.authority.registry.constant)).toBe(true);
  const schemaEntry = parser.parse(readFileSync(resolve(owner, "../🧬️schema/📦️packages/🦀️rust/🦀️.rs"), "utf8"));
  expect(schemaEntry.rootNode.namedChildren.some(node => node.type === "use_declaration" && node.text.startsWith("pub use vocabulary::") && node.text.includes(fixture.authority.registry.function))).toBe(true);
  console.log("[DEBUG] neutral exports32; full normal closure7 and registry include_str5; independent tree-sitter/@iarna-toml; explicit Cargo/Bun/owned route membership");
});
