import { authoredSnapshotSqliteContract } from "../../../../../../🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts";
import { fileURLToPath } from "node:url";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import { parse as parseJsonc } from "jsonc-parser";

const snapshot = fileURLToPath(new URL("../../", import.meta.url)), store = resolve(snapshot, "../../.."), root = resolve(store, "../../../../..");
authoredSnapshotSqliteContract(snapshot);

test("closed owner-registration corpus covers every opening and native encoding without changing envelope dialect", () => {
  const schema = JSON.parse(readFileSync(join(snapshot, "🪶️sqlite/📣️registration/🧬️schema/🔣️.json"), "utf8")), corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus)).toBe(true);
  expect(corpus.cases.map((sample: any) => sample.id).sort()).toEqual(["create-binary", "create-text", "reload-binary", "reload-text", "retained-binary", "retained-text"]);
  for (const sample of corpus.cases) {
    expect(sample.envelopeDialect).toBe(sample.opening === "retained" ? corpus.coordinate : null);
    expect(sample.before).toEqual({ export: "UnsupportedOwner", import: "UnsupportedOwner" });
    expect(sample.after).toEqual({ export: "completeSnapshot", import: "completeSnapshot", metadata: "sameOwnerAndEncoding" });
  }
  const wrong = structuredClone(corpus); wrong.cases[0].before.export = "InvalidValue"; expect(validate(wrong)).toBe(false);
  console.log("[DEBUG] independent Ajv owner-registration admission: create/reload/retained × binary/text; typed pre-refusal and retained envelope metadata");
});

test("actual native isolated law admits explicit owner registration and generic constructors remain owner-neutral", async () => {
  await Parser.init(); const parser = new Parser(), trees: Parser.Tree[] = [];
  try {
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
  const parse = (path: string) => { const tree = parser.parse(readFileSync(path, "utf8")); trees.push(tree); return tree; };
  const collect = (node: Parser.SyntaxNode, predicate: (node: Parser.SyntaxNode) => boolean): Parser.SyntaxNode[] => [ ...(predicate(node) ? [node] : []), ...node.namedChildren.flatMap(child => collect(child, predicate)) ];
  const calls = (node: Parser.SyntaxNode) => collect(node, node => node.type === "call_expression").map(node => ({ name: node.childForFieldName("function")?.text ?? "", start: node.startIndex }));
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8")), lawTree = parse(join(snapshot, "🧪️tests/🪶️sqlite/🦀️.rs"));
  expect(lawTree.rootNode.hasError()).toBe(false);
  const law = lawTree.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === corpus.nativeLaw);
  expect(law).toBeDefined();
  const names = calls(law!), registration = names.filter(node => node.name === "store::space_history_sqlite::register_sqlite_snapshot");
  expect(registration.length).toBe(1);
  const before = names.filter(node => node.start < registration[0].start), after = names.filter(node => node.start > registration[0].start);
  for (const operation of ["io_export_sqlite_snapshot", "io_import_sqlite_snapshot"]) {
    expect(before.some(node => node.name.includes(operation)), operation).toBe(true);
    expect(after.some(node => node.name.includes(operation)), operation).toBe(true);
  }
  const assertions = collect(law!, node => node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "assert_eq").map(node => node.text.replaceAll(/\s/g, ""));
  for (const side of ["export_refusal", "import_refusal"]) expect(assertions).toContain(`assert_eq!(${side}.cause.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner)`);
  const strings = collect(law!, node => node.type === "string_literal").map(node => JSON.parse(node.text));
  expect(strings).toContain("{}::" + corpus.nativeLaw); expect(strings).toContain("--exact"); expect(strings).toContain(corpus.isolationVariable);
  expect(names.some(node => node.name.includes("current_exe"))).toBe(true);
  expect(law!.text.includes("for sample in corpus[\"cases\"]")).toBe(true);
  expect(law!.text.includes('sample["encoding"]')).toBe(true);
  expect(law!.text.includes("assert_eq!(imported.value,source)")).toBe(true);
  expect(law!.text.includes("assert_eq!(relational,independent)")).toBe(true);
  const storeTree = parse(join(store, "🦀️.rs"));
  const constructors = collect(storeTree.rootNode, node => node.type === "function_item" && ["new", "construct", "from_initialized_runtime_with_owners"].includes(node.childForFieldName("name")?.text ?? "") && node.parent?.parent?.type === "impl_item" && node.parent?.parent?.childForFieldName("type")?.text.startsWith("ArtifactStore<") === true);
  expect(constructors.map(node => node.childForFieldName("name")?.text).sort()).toEqual(["construct", "from_initialized_runtime_with_owners", "new"]);
  const hydration = parse(join(store, "🧾️document/📜️history/💧️hydration/🦀️.rs"));
  for (const node of [...constructors, hydration.rootNode]) expect(calls(node).filter(node => /(?:register_sqlite_snapshot|register_native_snapshot_codec|register_native_document_codec)(?:<|$)/.test(node.name))).toEqual([]);
  const binding = parse(join(snapshot, "🪶️sqlite/🦀️.rs")), ownerRegistration = binding.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "register_sqlite_snapshot");
  expect(calls(ownerRegistration!).some(node => node.name === "crate::os_io::register_native_snapshot_codec")).toBe(true);
  expect(ownerRegistration!.text.includes("SQLITE_SNAPSHOT_DIALECT")).toBe(true); expect(ownerRegistration!.text.includes("SpaceHistorySnapshot,SpaceHistoryMutation")).toBe(true);
  console.log("[DEBUG] actual tree-sitter owner admission: isolated exact selector; both typed refusals before owner publication; complete same-owner roundtrips after; generic Store/hydration unchanged");
  } finally { for (const tree of trees) tree.delete(); parser.delete(); }
});

test("actual owner literals and existing GUI source/native routes identify the same SpaceHistory boundary", () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8")), source = readFileSync(join(store, "🦀️.rs"), "utf8"), owner = readFileSync(join(snapshot, "🪶️sqlite/🦀️.rs"), "utf8");
  expect(source).toContain('#[path="📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"]');
  expect(source).toContain('#[path = "📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs"]');
  expect(owner).toContain('standard:crate::os_io::StandardId("1")'); expect(owner).toContain('subset:crate::os_io::SubsetId("*")');
  expect(source).toContain(`pub const S_SPACE_HISTORY_SCHEMA: &str = "${corpus.owner}";`);
  const seed = parseJsonc(readFileSync(join(root, ".vscode/🧩️launch.seed.jsonc"), "utf8")), project = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json"), "utf8"));
  for (const command of ["test-space-history-sqlite-source", "test-space-history-sqlite-native"]) {
    expect(seed.configurations.filter((row: any) => row.command === `bun nx run @semio-tech/framework-os-kernel:${command} --skip-nx-cache`).length).toBe(1);
    expect(project.targets[command].options.command).toBe(`bun ./📜️script.ts ${command}`);
  }
  console.log("[DEBUG] existing actual GUI/source/native gate routes and literal SpaceHistory mounts agree");
});
