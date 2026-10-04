# Ingress Digest Retirement Static Candidate Review

Materialized source-ready receipt is absent at this read. No final source or runtime admission. Candidate script source only reviewed; no source writes, staging run, native tool or generator execution by this lane. Nx staging attempts reportedly rejected on graph cycles before execution; this is not feature RED.

The candidate stages exactly schema, fixture and ownership gate. It deletes regionDigest/lawDigests API coherently and introduces closed constant bounds and seven source witnesses: private extension/formatting accepted; wrong public owner/type/constant, missing law and hostile normal dependency refused. Full original fourteen public types and five constants remain closed fixture/schema declarations. AST exact pub visibility and module path/cfg/test attributes are checked; canonical entry facade must expose exact fourteen+five names and one declared command_ingress module. Actual five bound expressions are token-checked. Owner/entry/native-law AST errors explicitly reject. The original four law names must be #[test] functions; each has assertion macros and three corpus laws call the existing fixture helper. This is public contract/law presence proof, not replacement runtime algorithm equivalence. Original full transferred law bodies remain ticket receipts and actual native laws must rerun.

No mandatory Specific source reads added. Current normal manifest closure still reads only the declared neutral manifest graph, root Cargo and retained general Actor corpus. The facade assertion is canonical Replication package API, not an OS compatibility export. Token serialization preserves punctuation, and the bounds expressions contain only identifiers and numeric literals; it is not a general literal-whitespace normalizer. Hosted source fixture mutations target actual required regions and retain closed case ids. Final hosted callback/strict diagnostics/current guards are not yet independently checked because receipt is absent.

Potential proof scope limit: seven witnesses do not themselves exercise malformed AST or wrong facade/attribute cfg branches; source checks cover these but hostile runtime attribution must come from materialized proof. Do not claim all source changes semantically admitted merely from successful staging or law-presence assertions.

Candidate SHA256: cda261f73a99623963ae68dd5c8a08437825837d93d89c8adcf1258f2f60b69d

## Full Static Candidate

```typescript
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../.."), ticket = resolve(import.meta.dir, ".."), output = resolve(ticket, "🗑️generated/ingress-ownership-contract"), owner = "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress", hash = (source: string) => createHash("sha256").update(source).digest("hex");
const paths = [`${owner}/🧬️schema/🔣️.json`, `${owner}/🧫️fixtures/🔣️.json`, `${owner}/🏛️ownership/🧪️tests/🟦️.ts`];
const cases = [
  { id: "private-extension", accepted: true },
  { id: "formatting", accepted: true },
  { id: "public-owner", accepted: false },
  { id: "public-type", accepted: false },
  { id: "constant", accepted: false },
  { id: "missing-law", accepted: false },
  { id: "hostile-dependency", accepted: false },
];
const bounds = {
  COMMAND_PAGE_MAXIMUM_BYTES: "4_096",
  COMMAND_MAXIMUM_BYTES: "semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES",
  COMMAND_MAXIMUM_PAGES: "COMMAND_MAXIMUM_BYTES/COMMAND_PAGE_MAXIMUM_BYTES",
  COMMAND_BATCH_MAXIMUM_ITEMS: "64",
  INVOCATION_RESULT_PACK_MAXIMUM_BYTES: "COMMAND_MAXIMUM_BYTES",
};

function replace(source: string, before: string, after: string): string {
  if (!source.includes(before)) throw Error("Missing exact staged edit: " + before.slice(0, 100));
  return source.replace(before, after);
}

function pair(path: string, before: string, authored: string) {
  let start = 0, end = 0;
  while (start < before.length && start < authored.length && before[start] === authored[start]) start++;
  while (end < before.length - start && end < authored.length - start && before.at(-end - 1) === authored.at(-end - 1)) end++;
  const removed = before.slice(start, before.length - end), inserted = authored.slice(start, authored.length - end);
  return { path, before, beforeHash: hash(before), authored, authoredHash: hash(authored), edit: { start, removed, inserted }, inverse: { start, removed: inserted, inserted: removed }, forwardExact: before.slice(0, start) + inserted + before.slice(start + removed.length) === authored, inverseExact: authored.slice(0, start) + removed + authored.slice(start + inserted.length) === before };
}

function stage() {
  mkdirSync(output, { recursive: true });
  const before = paths.map(path => readFileSync(resolve(root, path), "utf8")), schema = JSON.parse(before[0]!), fixture = JSON.parse(before[1]!);
  for (const key of ["regionDigest", "lawDigests"]) { delete schema.properties[key]; schema.required = schema.required.filter((name: string) => name !== key); delete fixture[key]; }
  schema.required.push("bounds", "sourceCases");
  schema.properties.bounds = { const: bounds };
  schema.properties.sourceCases = { const: cases };
  fixture.bounds = bounds;
  fixture.sourceCases = cases;
  let source = before[2]!;
  source = replace(source, "  regionDigest: string;\n", "  bounds: Record<string, string>;\n  sourceCases: { id: string; accepted: boolean }[];\n");
  source = replace(source, "  lawDigests: Record<string, string>;\n", "");
  source = replace(source, "/** 🏛️ Admits canonical owner bytes, dependency authority and exact moved native laws. */", "/** 🏛️ Admits public neutral ownership, declared bounds, dependency authority and native law mounts. */");
  const start = source.indexOf("    const component = read("), end = source.indexOf("    const workspace = parseToml", start);
  if (start < 0 || end < 0) throw Error("Missing owner admission region");
  source = source.slice(0, start) + ownerAdmission + source.slice(end);
  source = replace(source, "    for (const law of laws) expect(digest(law.text)).toBe(fixture.lawDigests[law.childForFieldName(\"name\")!.text]);", lawAdmission);
  source = replace(source, "    const lawAst = ast(fixture.paths.neutralTests),", "    const lawAst = ast(fixture.paths.neutralTests);\n    expect(lawAst.hasError()).toBe(false);\n    const");
  source += syntheticTests;
  if (source.includes("regionDigest") || source.includes("lawDigests")) throw Error("Retired digest API remains");
  const rows = paths.map((path, index) => pair(path, before[index]!, index === 0 ? JSON.stringify(schema, null, 2) + "\n" : index === 1 ? JSON.stringify(fixture, null, 2) + "\n" : source));
  const parse = ts.createSourceFile(paths[2]!, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const parseDiagnostics = (parse as any).parseDiagnostics;
  if (parseDiagnostics.length) throw Error(ts.formatDiagnosticsWithColorAndContext(parseDiagnostics, { getCurrentDirectory: () => root, getCanonicalFileName: path => path, getNewLine: () => "\n" }));
  const contextPaths = [`${owner}/🦀️.rs`, fixture.paths.entry, fixture.paths.manifest, fixture.paths.neutralTests, fixture.fixture.path, "Cargo.toml", ...fixture.closure.map((row: any) => row.manifest)];
  const context = [...new Set<string>(contextPaths)].map(path => { const fullSource = readFileSync(resolve(root, path), "utf8"); return { path, fullSource, sha256: hash(fullSource) }; });
  const result = { at: new Date().toISOString(), mountReady: true, releaseToNative: false, sourceWrites: false, rows, context, schemaFirstCases: cases, nativeRuntimeExecuted: false, originalExtractionEvidence: "Original transfer hashes remain in ticket historical receipts; not an ongoing implementation immutability contract." };
  writeFileSync(resolve(output, "source-ready-1.json"), JSON.stringify(result, null, 2));
  const overlay = Object.fromEntries(rows.map(row => [resolve(root, row.path), row.authored]));
  let hosted = source.replace('import { readFileSync } from "node:fs";', 'import { readFileSync as physicalReadFileSync } from "node:fs";');
  hosted = hosted.replace('const root = resolve(import.meta.dir, "../../../../../../../..")', 'const root = ' + JSON.stringify(root));
  hosted = `const stagedSources: Record<string,string> = ${JSON.stringify(overlay)};\nconst readFileSync = (path: string, encoding: "utf8"): string => stagedSources[resolve(path)] ?? physicalReadFileSync(path, encoding);\n` + hosted;
  writeFileSync(resolve(output, "📜️script.ts"), hosted);
  const options: ts.CompilerOptions = { noEmit: true, strict: true, skipLibCheck: true, allowImportingTsExtensions: true, esModuleInterop: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: ["bun"] }, host = ts.createCompilerHost(options), originalRead = host.readFile;
  host.readFile = path => overlay[resolve(path)] ?? originalRead(path);
  const program = ts.createProgram([resolve(root, paths[2]!)], options, host), diagnostics = ts.getPreEmitDiagnostics(program);
  writeFileSync(resolve(output, "strict-proof-1.json"), JSON.stringify({ at: new Date().toISOString(), source: paths[2], options, diagnostics: diagnostics.map(d => ts.flattenDiagnosticMessageText(d.messageText, "\n")), sourceWrites: false, stagedVirtualHost: true }, null, 2));
  console.log(JSON.stringify({ rows: rows.length, context: context.length, inversesExact: rows.every(row => row.forwardExact && row.inverseExact), parseDiagnostics: parseDiagnostics.length, strictDiagnostics: diagnostics.length, output }));
  if (diagnostics.length) throw Error(ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCurrentDirectory: () => root, getCanonicalFileName: path => path, getNewLine: () => "\n" }));
}

const ownerAdmission = `    const component = read(\`\${owner}/🦀️.rs\`), ownerAst = ast(\`\${owner}/🦀️.rs\`);
    expect(ownerAst.hasError()).toBe(false);
    expect(publicNames(ownerAst, ["struct_item", "enum_item", "type_item"])).toEqual(fixture.exports.types);
    expect(publicNames(ownerAst, ["const_item"])).toEqual(fixture.exports.constants);
    const tokens = (node: Parser.SyntaxNode): string => node.type.endsWith("comment") ? "" : node.children.length ? node.children.map(tokens).join("") : node.text;
    const attributes = (node: Parser.SyntaxNode): Parser.SyntaxNode[] => {
      const result: Parser.SyntaxNode[] = [];
      for (let previous = node.previousNamedSibling; previous; previous = previous.previousNamedSibling) {
        if (previous.type.endsWith("comment")) continue;
        if (previous.type !== "attribute_item") break;
        result.unshift(previous.namedChildren[0]!);
      }
      return result;
    };
    const modulePath = (node: Parser.SyntaxNode) => {
      const values = attributes(node).filter(attribute => attribute.namedChildren[0]?.text === "path").map(attribute => JSON.parse(attribute.childForFieldName("value")!.text));
      expect(values).toHaveLength(1);
      return values[0] as string;
    };
    for (const constant of ownerAst.namedChildren.filter(node => node.type === "const_item" && fixture.exports.constants.includes(node.childForFieldName("name")!.text))) expect(tokens(constant.childForFieldName("value")!)).toBe(fixture.bounds[constant.childForFieldName("name")!.text]);
    const entryAst = ast(fixture.paths.entry);
    expect(entryAst.hasError()).toBe(false);
    const mounts = nodes(entryAst).filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "command_ingress");
    expect(mounts).toHaveLength(1);
    expect(mounts[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    expect(mounts[0]!.parent?.parent?.childForFieldName("name")?.text).toBe("wire");
    expect(modulePath(mounts[0]!)).toBe("../../📡️wire/🎮️command/📥️ingress/🦀️.rs");
    const facades = nodes(entryAst).filter(node => node.type === "use_declaration" && tokens(node.childForFieldName("argument")!.childForFieldName("path") ?? node) === "wire::command_ingress");
    expect(facades).toHaveLength(1);
    expect(facades[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    expect(facades[0]!.childForFieldName("argument")!.childForFieldName("list")!.namedChildren.map(node => node.text)).toEqual([...fixture.exports.types, ...fixture.exports.constants]);
    const testMounts = ownerAst.namedChildren.filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "unit_tests");
    expect(testMounts).toHaveLength(1);
    expect(modulePath(testMounts[0]!)).toBe("🧪️tests/🔬️unit/🦀️.rs");
    expect(attributes(testMounts[0]!).map(tokens)).toContain("cfg(test)");
`;

const lawAdmission = `    expect(functions.filter(node => attributes(node).some(attribute => tokens(attribute) === "test")).map(node => node.childForFieldName("name")!.text)).toEqual(fixture.laws);
    for (const law of laws) expect(nodes(law).some(node => node.type === "macro_invocation" && ["assert", "assert_eq"].includes(node.childForFieldName("macro")?.text ?? ""))).toBe(true);
    const corpusLaws = laws.slice(1);
    for (const law of corpusLaws) expect(nodes(law).some(node => node.type === "call_expression" && node.childForFieldName("function")?.text === "command_ingress_pages_fixture")).toBe(true);`;

const syntheticTests = `
test("schema-authored source witnesses admit private evolution and reject public ownership failures", async () => {
  const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")) as Fixture;
  for (const row of fixture.sourceCases) {
    const changes = new Map<string, string>(), componentPath = \`\${owner}/🦀️.rs\`, component = readFileSync(join(root, componentPath), "utf8");
    if (row.id === "private-extension") changes.set(componentPath, component + "\\nfn ownership_private_extension(value: usize) -> usize { value }\\n");
    else if (row.id === "formatting") {
      for (const path of [componentPath, fixture.paths.entry, fixture.paths.neutralTests]) changes.set(path, readFileSync(join(root, path), "utf8").replaceAll("#[path =", "#[ path  =").replaceAll("#[test]", "#[ test ]").replaceAll("pub mod command_ingress;", "pub  mod\\n command_ingress ;").replaceAll("mod unit_tests;", "mod\\n unit_tests ;"));
    } else if (row.id === "public-owner") changes.set(fixture.paths.entry, readFileSync(join(root, fixture.paths.entry), "utf8").replace("../../📡️wire/🎮️command/📥️ingress/🦀️.rs", "../../../../🛍️products/💻️os/🦀️.rs"));
    else if (row.id === "public-type") changes.set(componentPath, component.replace("pub struct FixedCommandPage", "pub struct UnownedCommandPage"));
    else if (row.id === "constant") changes.set(componentPath, component.replace("usize = 4_096", "usize = 8_192"));
    else if (row.id === "missing-law") changes.set(fixture.paths.neutralTests, readFileSync(join(root, fixture.paths.neutralTests), "utf8").replace("fn " + fixture.laws[0] + "(", "fn absent_native_ownership_law("));
    else if (row.id === "hostile-dependency") changes.set(fixture.paths.manifest, readFileSync(join(root, fixture.paths.manifest), "utf8").replace("[dependencies]", '[dependencies]\\nsemio-foreign-owner = "1"'));
    else throw Error("Unknown closed ownership source case: " + row.id);
    const read: Reader = path => changes.get(path) ?? readFileSync(join(root, path), "utf8");
    if (row.accepted) expect(await inspectPagedCommandIngressOwnership(root, read)).toEqual({ types: 14, constants: 5, laws: 4, closure: fixture.closure.length });
    else await expect(inspectPagedCommandIngressOwnership(root, read)).rejects.toThrow();
    console.log(\`[DEBUG] command ingress source witness \${row.id}: \${row.accepted ? "admitted" : "refused"}\`);
  }
});
`;

if (import.meta.main) {
  if (process.argv[2] !== "stage") throw Error("Expected stage");
  stage();
}

```
