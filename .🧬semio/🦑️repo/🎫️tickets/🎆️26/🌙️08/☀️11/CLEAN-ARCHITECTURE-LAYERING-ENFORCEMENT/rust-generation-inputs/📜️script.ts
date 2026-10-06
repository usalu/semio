import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), require = createRequire(join(root, "package.json")), owner = "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust", directory = join(ticket, "🗑️generated/rust-generation"), signal = new AbortController();
process.once("SIGINT", () => signal.abort()); process.once("SIGTERM", () => signal.abort());
const sha = (source: string) => createHash("sha256").update(source).digest("hex"), physical = (path: string) => existsSync(join(root, path)) ? readFileSync(join(root, path), "utf8") : null;
const contexts: any[] = [], capture = (path: string) => { const source = physical(path); assert.notEqual(source, null); const frame = { path, source, sha256: sha(source!), inverse: source }; contexts.push(frame); return frame; };
const save = (name: string, value: unknown) => { mkdirSync(directory, { recursive: true }); const path = join(directory, name); assert.equal(existsSync(path), false); writeFileSync(path, JSON.stringify(value, null, 2)); return path; };

/** 🦀️ Stages the shared std-owned lexical and generated-input counterpart without executing Cargo. */
async function stage(epoch: string) {
  const pathSourcePath = join(ticket, "🗑️generated/rust-path-native/source-1.json"), proofPath = join(ticket, "🗑️generated/rust-path-native/source-independent-1.json"), sourceBytes = readFileSync(pathSourcePath, "utf8"), proofBytes = readFileSync(proofPath, "utf8"), source = JSON.parse(sourceBytes), proof = JSON.parse(proofBytes);
  assert.equal(proof.admitted, true); assert.equal(proof.authorityHash, sha(sourceBytes)); assert.equal(proof.contractCases, 35);
  const corpus = capture(owner + "/📤️generation/🧫️fixtures/🔣️.json"), contract = capture(owner + "/📤️generation/🔣️.json"), corpusSchema = capture(owner + "/📤️generation/🧬️schema/🔣️.json"), compiler = capture(owner + "/🟦️.ts"), { inspectRustCompileReferences } = await import(join(root, compiler.path));
  const schemaSource = readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"), schema = JSON.parse(schemaSource), { default: Ajv } = await import("ajv"), ajv = new Ajv({ strict: true }); ajv.addSchema(JSON.parse(contract.source!));
  const fixture = JSON.parse(corpus.source!), cases: any[] = [];
  assert.equal(ajv.compile(JSON.parse(corpusSchema.source!))(fixture), true);
  for (const row of fixture.cases) { signal.signal.throwIfAborted(); const outputs: any[] = []; inspectRustCompileReferences(row.source, (output: any) => outputs.push(output)); assert.deepEqual(outputs.flatMap(output => output.inputs.map((input: any) => input.expression)), row.expressions); cases.push({ id: row.id, outputs, refusal: null }); }
  for (const row of fixture.unsupported) { assert.throws(() => inspectRustCompileReferences(row.source, () => {}), /Unsupported Rust compile/); cases.push({ id: row.id, outputs: [], refusal: "unresolved-generator-origin" }); }
  for (const row of fixture.authoredMacros) { const outputs: any[] = [], refs = inspectRustCompileReferences(row.source, (output: any) => outputs.push(output)); assert.deepEqual(outputs, []); assert.deepEqual(refs.filter((row: any) => row.kind === "include_str").map((row: any) => row.path), row.paths); cases.push({ id: row.id, outputs, refusal: null }); }
  const outputs = { schemaVersion: 1, cases }; assert.equal(ajv.compile(schema)(outputs), true); assert.equal(ajv.compile(schema)({ ...outputs, foreign: true }), false);
  const diff = require("diff"), rows: any[] = [], redRows: any[] = [], advances: any[] = [];
  const add = (path: string, after: string, prior?: any) => {
    const before = physical(path); if (prior && before !== prior.before) advances.push({ path, before: prior.before, current: before, inverse: prior.before, priorAfter: prior.after, semanticEquivalence: false });
    const forward = diff.createPatch(path, before ?? "", after), inverseEdit = diff.createPatch(path, after, before ?? ""); assert.equal(diff.applyPatch(before ?? "", forward, { fuzzFactor: 0 }), after); assert.equal(diff.applyPatch(after, inverseEdit, { fuzzFactor: 0 }), before ?? "");
    const row = { path, before, current: before, after, authored: after, inverse: before, beforeHash: before === null ? null : sha(before), afterHash: sha(after), forward, inverseEdit }; rows.push(row); return row;
  };
  const pathRow = source.rows.find((row: any) => row.path === owner + "/📁️paths/🦀️.rs"), lexicalStart = pathRow.after.indexOf("#[derive(Clone, Copy, PartialEq, Eq)]"), lexicalEnd = pathRow.after.indexOf("fn qualified_call(");
  assert.ok(lexicalStart > 0 && lexicalEnd > lexicalStart);
  let lexical = pathRow.after.slice(lexicalStart, lexicalEnd);
  for (const [before, after] of [["enum Kind {", "pub(super) enum Kind {"], ["struct Token<'a> {", "pub(super) struct Token<'a> {"], ["fn tokens(", "pub(super) fn tokens("], ["fn string_value(", "pub(super) fn string_value("]]) { assert.equal(lexical.split(before).length - 1, 1); lexical = lexical.replace(before, after); }
  for (const name of ["kind", "text", "line", "start", "end"]) lexical = lexical.replace("    " + name + ":", "    pub(super) " + name + ":");
  lexical = lexical.replace("    pub(super) end: usize,\n", "    pub(super) end: usize,\n    pub(super) byte_start: usize,\n    pub(super) byte_end: usize,\n").replace("line, start: utf16, end });", "line, start: utf16, end, byte_start: start, byte_end: byte });");
  const pairStart = pathRow.after.indexOf("    let mut pairs = vec![None; tokens.len()];"), pairEnd = pathRow.after.indexOf("    let mut output = Vec::new();", pairStart); assert.ok(pairStart > lexicalEnd && pairEnd > pairStart);
  const pairing = pathRow.after.slice(pairStart, pairEnd);
  lexical = "//! 🧬️ Shared private Rust tokens preserve UTF-16 and original byte positions.\n" + lexical + "\npub(super) fn token_pairs(tokens: &[Token<'_>]) -> Vec<Option<usize>> {\n" + pairing + "    pairs\n}\n";
  let pathAfter = pathRow.after.slice(0, lexicalStart) + "use super::tokens::{Kind, tokens, string_value, token_pairs};\n\n" + pathRow.after.slice(lexicalEnd); assert.ok(pathAfter.includes(pairing)); pathAfter = pathAfter.replace(pairing, "    let pairs = token_pairs(&tokens);\n");
  const generation = readFileSync(join(ticket, "runtime-direction-inputs/📤️generation/🧫️fixtures/🦀️.rs"), "utf8"), law = readFileSync(join(import.meta.dir, "🧫️fixtures/🧪️tests/🦀️.rs"), "utf8");
  for (const prior of source.rows) {
    let after = prior.after;
    if (prior.path === pathRow.path) after = pathAfter;
    if (prior.path === owner + "/🦀️.rs") after += '\n#[path = "🧬️tokens/🦀️.rs"]\nmod tokens;\n\n#[path = "📤️generation/🦀️.rs"]\npub mod generation;\n';
    if (physical(prior.path) !== prior.before && physical(prior.path) !== prior.after) throw Error("Foreign Rust path predecessor requires fresh exact continuation: " + prior.path);
    add(prior.path, after, prior);
  }
  add(owner + "/🧬️tokens/🦀️.rs", lexical); add(owner + "/📤️generation/🦀️.rs", generation); add(owner + "/📤️generation/🧪️tests/🦀️.rs", law);
  add(owner + "/📤️generation/🧫️fixtures/🎯️outputs/🔣️.json", JSON.stringify(outputs, null, 2) + "\n"); add(owner + "/📤️generation/🧬️schema/🎯️outputs/🔣️.json", schemaSource);
  const loaded = require("web-tree-sitter"), Parser = loaded.Parser ?? loaded.default ?? loaded; await Parser.init(); const parser = new Parser(); parser.setLanguage(await (Parser.Language ?? loaded.Language).load(join(dirname(require.resolve("tree-sitter-wasms/package.json")), "out/tree-sitter-rust.wasm")));
  const grammars: any[] = [];
  for (const row of rows.filter(row => row.path.endsWith(".rs"))) { const tree = parser.parse(row.after); assert.equal(tree.rootNode.hasError(), false, row.path); grammars.push({ path: row.path, functions: tree.rootNode.descendantsOfType("function_item").length, grammarErrors: false }); tree.delete(); }
  const generationTree = parser.parse(generation), inspector = generationTree.rootNode.descendantsOfType("function_item").filter((node: any) => node.childForFieldName("name")?.text === "inspect_rust_generated_token_outputs"); assert.equal(inspector.length, 1);
  const redGeneration = generation.slice(0, inspector[0].startIndex) + generation.slice(inspector[0].endIndex); generationTree.delete(); parser.delete();
  for (const row of rows) redRows.push(row.path === owner + "/📤️generation/🦀️.rs" ? { ...row, after: redGeneration, authored: redGeneration, afterHash: sha(redGeneration), forward: diff.createPatch(row.path, row.before ?? "", redGeneration), inverseEdit: diff.createPatch(row.path, redGeneration, row.before ?? "") } : row);
  for (const frame of source.contexts) { const current = capture(frame.path); if (current.source !== frame.source) advances.push({ path: frame.path, before: frame.source, current: current.source, inverse: frame.source, semanticEquivalence: false }); }
  const nativeScript = capture("🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/📜️script.ts"); assert.ok(nativeScript.source!.includes("test"));
  for (const row of rows) assert.equal(physical(row.path), row.before); for (const frame of contexts) assert.equal(physical(frame.path), frame.source);
  const common = { at: new Date().toISOString(), predecessor: { path: pathSourcePath, source: sourceBytes, sha256: sha(sourceBytes), proofPath, proofSource: proofBytes, proofHash: sha(proofBytes) }, contexts, advances, controls: cases, schemaSource, grammars, originalPathControls: 35, originalGeneratorControls: 17, sharedLexer: { original: pathRow.after.slice(lexicalStart, lexicalEnd), proposed: lexical, originalPairing: pairing, proposedPath: pathAfter }, route: { script: nativeScript, cargoArgs: ["--lib"], filters: [], whole: true }, sourceReady: true, sourceWrites: 0, nativeExecuted: false, scope: "Separate std-owned generated-output inspector and shared private lexer; exact original17 generator+35path controls retained. Same closed corpus, full TS outputs, installed schema/Rust grammar controls; native compiler/RED/GREEN pending. Local authored macro is not classified as generated; full Rust compile-reference expansion is outside this standalone API." };
  const green = save("source-" + epoch + ".json", { ...common, rows }), red = save("red-source-" + epoch + ".json", { ...common, rows: redRows, absentInspectorOnly: true });
  console.log(JSON.stringify({ green, red, rows: rows.length, generatorControls: cases.length, pathControls: 35, grammars: grammars.length, sourceWrites: 0, nativeExecuted: false }));
}
const [command, epoch = "1"] = process.argv.slice(2);
assert.equal(command, "stage");
await stage(epoch);
