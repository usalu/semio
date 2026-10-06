import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { closeSync, existsSync, fstatSync, fsyncSync, ftruncateSync, lstatSync, mkdirSync, openSync, readFileSync, readSync, unlinkSync, writeFileSync, writeSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), require = createRequire(join(root, "package.json"));
const directory = join(ticket, "🗑️generated/runtime-direction"), signal = new AbortController();
process.once("SIGINT", () => signal.abort()); process.once("SIGTERM", () => signal.abort());
const sha = (value: string) => createHash("sha256").update(value).digest("hex"), read = (path: string) => readFileSync(join(root, path), "utf8");
const frames: any[] = [], capture = (path: string) => { const source = read(path), frame = { path, source, sha256: sha(source), inverse: source }; frames.push(frame); return frame; };
const save = (name: string, value: unknown) => { mkdirSync(directory, { recursive: true }); const path = join(directory, name); assert.equal(existsSync(path), false); writeFileSync(path, JSON.stringify(value, null, 2)); return path; };

/** 📎️ Checks the finite current fixture-path corrections against exact physical owners. */
async function paths(command: string, epoch: string) {
  const schemaSource = readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"), fixtureSource = readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8");
  const schema = JSON.parse(schemaSource), fixture = JSON.parse(fixtureSource), { default: Ajv } = await import("ajv");
  const { validateJsonSchemaSubset } = await import(join(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
  const validate = new Ajv({ strict: true }).compile(schema); assert.equal(validate(fixture), true); assert.deepEqual(validateJsonSchemaSubset(schema, fixture), []);
  const hostile = { ...fixture, foreign: true }; assert.equal(validate(hostile), false); assert.ok(validateJsonSchemaSubset(schema, hostile).length);
  const apiPath = "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts", apiFrame = capture(apiPath), api = await import(join(root, apiPath));
  const pathe = require("pathe"), diff = require("diff"), controls: any[] = [], sources = new Map<string, string>(), firstFailures: any[] = [];
  for (const row of fixture.cases) {
    signal.signal.throwIfAborted();
    assert.equal(posix.relative(posix.dirname(row.sourcePath), row.targetPath), row.afterLiteral);
    assert.equal(pathe.relative(pathe.dirname(row.sourcePath), row.targetPath), row.afterLiteral);
    assert.equal(posix.normalize(posix.join(posix.dirname(row.sourcePath), row.afterLiteral)), row.targetPath);
    const source = sources.get(row.sourcePath) ?? capture(row.sourcePath).source; sources.set(row.sourcePath, source);
    const currentTarget = posix.normalize(posix.join(posix.dirname(row.sourcePath), row.beforeLiteral));
    assert.notEqual(currentTarget, row.targetPath);
    const target = capture(row.targetPath); assert.ok(target.source.length > 0);
    const matches = api.inspectRustCompileReferences(source).filter((ref: any) => ref.kind === "include_str" && ref.path === row.beforeLiteral);
    assert.equal(matches.length, row.occurrences);
    firstFailures.push({ id: row.id, actual: currentTarget, expected: row.targetPath, currentTargetExists: existsSync(join(root, currentTarget)) });
    controls.push({ id: row.id, actual: row.afterLiteral, nodePath: row.afterLiteral, pathe: row.afterLiteral, targetHash: target.sha256, occurrences: row.occurrences });
    console.error("[DEBUG] fixture path owner " + row.id + " current=" + currentTarget + " proposed=" + row.targetPath);
  }
  if (command === "red-paths") {
    save("paths-red-" + epoch + ".json", { at: new Date().toISOString(), schemaSource, fixtureSource, frames, firstFailures, controls, sourceWrites: 0, expectedTerminal: 1 });
    assert.deepEqual(firstFailures.map(row => row.actual), firstFailures.map(row => row.expected), "current includes must resolve their declared physical fixture owners");
    return;
  }
  const loaded = require("web-tree-sitter"), Parser = loaded.Parser ?? loaded.default ?? loaded; await Parser.init();
  const parser = new Parser(); parser.setLanguage(await (Parser.Language ?? loaded.Language).load(join(dirname(require.resolve("tree-sitter-wasms/package.json")), "out/tree-sitter-rust.wasm")));
  const rows: any[] = [];
  for (const [path, before] of sources) {
    let after = before;
    for (const row of fixture.cases.filter((row: any) => row.sourcePath === path)) {
      const literal = JSON.stringify(row.beforeLiteral), successor = JSON.stringify(row.afterLiteral);
      assert.equal(after.split(literal).length - 1, row.occurrences); after = after.replaceAll(literal, successor);
    }
    const oldTree = parser.parse(before), newTree = parser.parse(after);
    assert.equal(oldTree.rootNode.hasError(), false); assert.equal(newTree.rootNode.hasError(), false);
    const original = oldTree.rootNode.descendantsOfType("function_item"), proposed = newTree.rootNode.descendantsOfType("function_item");
    assert.equal(proposed.length, original.length);
    for (let index = 0; index < original.length; index++) {
      let expected = original[index].text;
      for (const row of fixture.cases.filter((row: any) => row.sourcePath === path)) expected = expected.replaceAll(JSON.stringify(row.beforeLiteral), JSON.stringify(row.afterLiteral));
      assert.equal(proposed[index].text, expected);
    }
    const physicalIncludes = newTree.rootNode.descendantsOfType("macro_invocation").filter((node: any) => node.childForFieldName("macro")?.text === "include_str").map((node: any) => {
      const argument = node.namedChildren.find((child: any) => child.type === "token_tree")?.namedChildren[0];
      return argument?.type === "string_literal" ? JSON.parse(argument.text) : null;
    });
    for (const row of fixture.cases.filter((row: any) => row.sourcePath === path)) {
      assert.equal(physicalIncludes.filter((value: string) => value === row.afterLiteral).length, row.occurrences);
      assert.equal(api.inspectRustCompileReferences(after).filter((ref: any) => ref.path === row.afterLiteral).length, row.occurrences);
    }
    const forward = diff.createPatch(path, before, after), inverseEdit = diff.createPatch(path, after, before);
    assert.equal(diff.applyPatch(before, forward, { fuzzFactor: 0 }), after); assert.equal(diff.applyPatch(after, inverseEdit, { fuzzFactor: 0 }), before);
    rows.push({ path, before, current: before, after, authored: after, inverse: before, beforeHash: sha(before), afterHash: sha(after), forward, inverseEdit, functions: original.length, qualifierOnly: true });
    oldTree.delete(); newTree.delete();
  }
  parser.delete();
  for (const path of ["🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml"]) capture(path);
  for (const frame of frames.filter(row => row.path.endsWith("Cargo.toml"))) assert.deepEqual(Bun.TOML.parse(frame.source), require("@iarna/toml").parse(frame.source));
  const flowManifest = frames.find(row => row.path.endsWith("🌊️flow/📦️packages/🦀️rust/Cargo.toml"));
  const declaredFlow = Bun.TOML.parse(flowManifest.source).test.find((row: any) => row.name === "flow_mesh_pack_wire");
  assert.equal(posix.normalize(posix.join(posix.dirname(flowManifest.path), declaredFlow.path)), rows.find(row => row.path.includes("mesh-pack-wire")).path);
  for (const row of rows) assert.equal(read(row.path), row.before); for (const frame of frames) assert.equal(read(frame.path), frame.source);
  const authority = { at: new Date().toISOString(), schemaSource, fixtureSource, rows, frames, controls, firstFailures, compiler: apiFrame, sourceReady: true, sourceWrites: 0, nativeExecuted: false, scope: "Two current full law bodies, four include literal corrections only. All existing predicates/expected fixture bytes remain exact. Flow integration target is physically declared; General DSL schema root is captured, with no broad missing-mount absence or native selection assertion." };
  console.log(JSON.stringify({ path: save("paths-source-" + epoch + ".json", authority), rows: rows.length, cases: controls.length, substitutions: fixture.cases.reduce((sum: number, row: any) => sum + row.occurrences, 0), sourceWrites: 0, nativeExecuted: false }));
}

/** 📤️ Stages observable quote-generated inputs without rewriting the owning derive. */
async function generation(command: string, epoch: string) {
  const domain = "📤️generation", owner = "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust", executionPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
  const schemaSource = readFileSync(join(import.meta.dir, domain, "🧬️schema/🔣️.json"), "utf8"), fixtureSource = readFileSync(join(import.meta.dir, domain, "🧫️fixtures/🔣️.json"), "utf8");
  const fixture = JSON.parse(fixtureSource), schema = JSON.parse(schemaSource), { default: Ajv } = await import("ajv"), validate = new Ajv({ strict: true }).compile(schema);
  const outputContractSource = readFileSync(join(import.meta.dir, domain, "🔣️.json"), "utf8"), outputContract = JSON.parse(outputContractSource), validateOutput = new Ajv({ strict: true }).compile(outputContract);
  const { validateJsonSchemaSubset } = await import(join(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
  assert.equal(validate(fixture), true); assert.deepEqual(validateJsonSchemaSubset(schema, fixture), []); assert.equal(validate({ ...fixture, foreign: true }), false);
  const apiFrame = capture(owner + "/🟦️.ts"), current = await import(join(root, apiFrame.path)), failures: any[] = [];
  if (command === "red-generation") {
    for (const row of fixture.cases) {
      try { const outputs: any[] = []; current.inspectRustCompileReferences(row.source, (output: any) => outputs.push(output)); assert.deepEqual(outputs.flatMap(output => output.inputs.map((input: any) => input.expression)), row.expressions); }
      catch (error) { failures.push({ id: row.id, error: (error as Error).message }); }
    }
    save("generation-red-" + epoch + ".json", { at: new Date().toISOString(), apiFrame, schemaSource, fixtureSource, outputContractSource, failures, sourceWrites: 0, expectedTerminal: 1 });
    assert.deepEqual(failures, [], "current compile inspector must explicitly retain quote-generated inputs"); return;
  }
  const definitions = readFileSync(join(import.meta.dir, domain, "🧫️fixtures/🟦️.ts"), "utf8");
  let after = apiFrame.source;
  const replace = (before: string, successor: string) => { assert.equal(after.split(before).length - 1, 1, before.slice(0, 120)); after = after.replace(before, successor); };
  replace('export function inspectRustCompileReferences(source: string): readonly RustCompileReference[] {', definitions + '\nexport function inspectRustCompileReferences(source: string, onGeneratedTokens?: (output: RustGeneratedTokenOutput) => void): readonly RustCompileReference[] {');
  replace('const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), references: RustCompileReference[] = [];', 'const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), references: RustCompileReference[] = [];\n  const generated = rustGeneratedTokenOutputs(source, tokens, pairs);\n  if (generated.length && !onGeneratedTokens) throw Error("Unsupported Rust compile token output requires an explicit generated-input observer");\n  for (const range of generated) onGeneratedTokens!(range.output);');
  replace('const attributes = rustAttributes(tokens, pairs, index, tokens.length), definition = attributes.next;', 'const emitted = generated.find(range => range.first <= index && index <= range.close);\n    if (emitted) { index = emitted.close; continue; }\n    const attributes = rustAttributes(tokens, pairs, index, tokens.length), definition = attributes.next;');
  replace('const inner = tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[", innerClose = inner ? pairs.get(index + 2) : undefined;', 'const emitted = generated.find(range => range.first === index);\n      if (emitted) { index = emitted.close; continue; }\n      const inner = tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[", innerClose = inner ? pairs.get(index + 2) : undefined;');
  const evaluated = { exports: {} as any }, transpiled = require("typescript").transpileModule(after, { compilerOptions: { module: require("typescript").ModuleKind.CommonJS, target: require("typescript").ScriptTarget.ES2022 } }).outputText;
  new Function("exports", "module", transpiled)(evaluated.exports, evaluated); const proposed = evaluated.exports;
  const loaded = require("web-tree-sitter"), Parser = loaded.Parser ?? loaded.default ?? loaded; await Parser.init(); const parser = new Parser(); parser.setLanguage(await (Parser.Language ?? loaded.Language).load(join(dirname(require.resolve("tree-sitter-wasms/package.json")), "out/tree-sitter-rust.wasm")));
  const controls: any[] = [];
  for (const row of fixture.cases) {
    signal.signal.throwIfAborted(); const outputs: any[] = [], references = proposed.inspectRustCompileReferences(row.source, (output: any) => outputs.push(output));
    assert.deepEqual(outputs.flatMap(output => output.inputs.map((input: any) => input.expression)), row.expressions, row.id);
    assert.equal(outputs.length, 1); assert.equal(outputs[0].macro, row.macro);
    assert.equal(validateOutput(outputs[0]), true); assert.deepEqual(validateJsonSchemaSubset(outputContract, outputs[0]), []); assert.equal(validateOutput({ ...outputs[0], foreign: true }), false);
    assert.deepEqual(references.map((ref: any) => ref.path), row.authored ? [row.authored] : []);
    assert.throws(() => proposed.inspectRustCompileReferences(row.source), /explicit generated-input observer/);
    const tree = parser.parse(row.source); assert.equal(tree.rootNode.hasError(), false); const generator = tree.rootNode.descendantsOfType("macro_invocation").find((node: any) => node.childForFieldName("macro")?.text === row.macro); assert.ok(generator);
    assert.equal(outputs[0].start, generator.startIndex); assert.equal(outputs[0].end, generator.endIndex);
    const independent: string[] = [], visit = (node: any) => {
      const children = node.children;
      for (let index = 0; index < children.length; index++) {
        if (["include", "include_str", "include_bytes"].includes(children[index].text) && children[index + 1]?.text === "!" && children[index + 2]?.type === "token_tree") { const input = children[index + 2]; independent.push(row.source.slice(input.startIndex + 1, input.endIndex - 1)); index += 2; }
        else if (children[index].type === "token_tree") visit(children[index]);
      }
    }; visit(generator.namedChildren.find((node: any) => node.type === "token_tree")); assert.deepEqual(independent, row.expressions, row.id); tree.delete();
    controls.push({ id: row.id, references, outputs, thirdPartyExpressions: independent }); console.error("[DEBUG] quote generator retained " + row.id);
  }
  for (const row of fixture.unsupported) { assert.throws(() => proposed.inspectRustCompileReferences(row.source, () => {}), /Unsupported Rust compile/); controls.push({ id: row.id, refused: true }); }
  for (const row of fixture.authoredMacros) { const outputs: any[] = []; assert.deepEqual(proposed.inspectRustCompileReferences(row.source, (output: any) => outputs.push(output)).map((ref: any) => ref.path), row.paths); assert.deepEqual(outputs, []); controls.push({ id: row.id, paths: row.paths, authored: true }); }
  parser.delete();
  const originalCorpus = capture(owner + "/🧫️fixtures/🔣️.json"); const corpus = JSON.parse(originalCorpus.source);
  for (const row of corpus.cases) assert.deepEqual(proposed.inspectRustCompileReferences(row.source), row.references);
  for (const row of corpus.unsupported) assert.throws(() => proposed.inspectRustCompileReferences(row.source), /Unsupported Rust compile/);
  const derive = capture("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs"), deriveOutputs: any[] = [], deriveReferences = proposed.inspectRustCompileReferences(derive.source, (output: any) => deriveOutputs.push(output));
  assert.ok(deriveOutputs.some(output => output.inputs.some((input: any) => input.expression === "#taxonomy_dependency")));
  const deriveManifest = capture("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml"); capture("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs");
  assert.deepEqual(Bun.TOML.parse(deriveManifest.source), require("@iarna/toml").parse(deriveManifest.source)); assert.equal(Bun.TOML.parse(deriveManifest.source).dependencies.quote, "1.0");
  const execution = capture(executionPath); let executionAfter = execution.source;
  const change = (before: string, successor: string) => { assert.equal(executionAfter.split(before).length - 1, 1); executionAfter = executionAfter.replace(before, successor); };
  change('import { inspectRustCompileReferences }', 'import { inspectRustCompileReferences, type RustGeneratedTokenOutput }');
  change('inspectRustModuleGraph, inspectRustModuleGraphFacts', 'inspectRustModuleGraph, inspectRustModuleGraphFacts, projectCargoProviderManifest');
  change('"unresolved-template-scope"; from:', '"unresolved-template-scope" | "unresolved-generator-origin"; from:');
  change('schemaVersion: 1; files: number;', 'schemaVersion: 1; generatedTokens: readonly Readonly<{ from: string; output: RustGeneratedTokenOutput; manifests: readonly string[] }>[]; files: number;');
  change('const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>();', 'const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>(), deferred = new Map<string, RustGeneratedTokenOutput[]>();\n    const generatedTokens: { from: string; output: RustGeneratedTokenOutput; manifests: string[] }[] = [];');
  change('try { compileReferences.set(path, inspectRustCompileReferences(source)); }', 'try { const outputs: RustGeneratedTokenOutput[] = []; compileReferences.set(path, inspectRustCompileReferences(source, output => outputs.push(output))); if (outputs.length) deferred.set(path, outputs); }');
  change('const contexts = graph.contexts.get(path), manifestPaths: string[] = [];', 'const contexts = graph.contexts.get(path), manifestPaths: string[] = [];\n      for (const output of deferred.get(path) ?? []) {\n        const owners = contexts?.filter(context => context.sourceScope.length === 0) ?? [], manifests = [...new Set(owners.flatMap(context => context.manifestPath ? [context.manifestPath] : []))];\n        const sealed = owners.length > 0 && owners.every(context => context.manifestPath && !graph.invalidManifests.has(context.manifestPath) && context.sourceChain.at(-1) === path && context.mount.kind !== "include") && manifests.every(locator => {\n          const source = sources.get(locator); if (source === undefined) return false;\n          const projection = projectCargoProviderManifest({ locator, source }), candidates = projection.dependencies.filter(binding => binding.key === "quote");\n          return candidates.length === 1 && candidates[0]!.source === "version" && typeof candidates[0]!.version === "string" && !candidates[0]!.targetCondition && !candidates[0]!.workspaceInherited && !candidates[0]!.localPath && !candidates[0]!.packageOverride && Object.keys(candidates[0]!.unsupported).length === 0;\n        });\n        generatedTokens.push({ from: path, output, manifests });\n        if (!sealed) problems.push({ code: "unresolved-generator-origin", from: path, line: output.line, detail: "Token generator requires a captured direct normal quote provider and exclusive lexical scope: " + output.macro });\n      }');
  change('return { schemaVersion: 1, files, references, violations: edges, problems, runtime };', 'return { schemaVersion: 1, files, references, violations: edges, problems, runtime, generatedTokens };');
  const law = readFileSync(join(import.meta.dir, domain, "🧫️fixtures/🧪️tests/🟦️.ts"), "utf8"), diff = require("diff");
  const pair = (path: string, after: string) => { const before = existsSync(join(root, path)) ? read(path) : null, forward = diff.createPatch(path, before ?? "", after), inverseEdit = diff.createPatch(path, after, before ?? ""); assert.equal(diff.applyPatch(before ?? "", forward, { fuzzFactor: 0 }), after); assert.equal(diff.applyPatch(after, inverseEdit, { fuzzFactor: 0 }), before ?? ""); return { path, before, current: before, after, authored: after, inverse: before, beforeHash: before === null ? null : sha(before), afterHash: sha(after), forward, inverseEdit }; };
  const rows = [pair(apiFrame.path, after), pair(owner + "/" + domain + "/🔣️.json", outputContractSource), pair(owner + "/" + domain + "/🧬️schema/🔣️.json", schemaSource), pair(owner + "/" + domain + "/🧫️fixtures/🔣️.json", fixtureSource), pair(owner + "/" + domain + "/🧪️tests/🟦️.ts", law), pair(executionPath, executionAfter)];
  const ts = require("typescript"); for (const row of rows.filter(row => row.path.endsWith(".ts"))) assert.equal(ts.createSourceFile(row.path, row.after, ts.ScriptTarget.Latest, true).parseDiagnostics.length, 0);
  const discovery = capture("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"), module = await import(join(root, discovery.path));
  const projection = module.projectCargoProviderManifest({ locator: deriveManifest.path, source: deriveManifest.source }); assert.equal(projection.dependencies.find((row: any) => row.key === "quote").source, "version");
  for (const row of rows) assert.equal(existsSync(join(root, row.path)) ? read(row.path) : null, row.before); for (const frame of frames) assert.equal(read(frame.path), frame.source);
  console.log(JSON.stringify({ path: save("generation-source-" + epoch + ".json", { at: new Date().toISOString(), rows, frames, controls, deriveOutputs, deriveReferences, schemaSource, fixtureSource, sourceReady: true, sourceWrites: 0, nativeExecuted: false, scope: "Observable generator inputs; exact direct/alias or qualified quote syntax with closed lexical origins, explicit observer, normal provider census qualification. Unresolved origins refuse. Original derive body byte-exact. No blanket name-based skip/native result." }), rows: rows.length, controls: controls.length, generatedDeriveInputs: deriveOutputs.reduce((sum, row) => sum + row.inputs.length, 0), sourceWrites: 0 }));
}

/** 🧪️ Reaches the generator laws through both original owning test commands. */
function stageGenerationRoutes(epoch: string) {
  const sourcePath = join(directory, "generation-source-2.json"), proofPath = join(directory, "generation-source-independent-2.json"), sourceBytes = readFileSync(sourcePath, "utf8"), proofBytes = readFileSync(proofPath, "utf8"), source = JSON.parse(sourceBytes), proof = JSON.parse(proofBytes);
  assert.equal(proof.admitted, true); assert.equal(proof.authorityHash, sha(sourceBytes)); assert.equal(source.rows.length, 6);
  const generalLaw = "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🟦️.ts", repoLaw = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts", generatedLaw = "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📤️generation/🧪️tests/🟦️.ts", diff = require("diff"), additions: any[] = [];
  for (const path of [generalLaw, repoLaw]) {
    const before = capture(path).source, relative = posix.relative(posix.dirname(path), generatedLaw), after = before + '\nimport ' + JSON.stringify(relative.startsWith(".") ? relative : "./" + relative) + ';\n';
    assert.ok(!before.includes(generatedLaw) && !before.includes(relative));
    const forward = diff.createPatch(path, before, after), inverseEdit = diff.createPatch(path, after, before); assert.equal(diff.applyPatch(before, forward, { fuzzFactor: 0 }), after); assert.equal(diff.applyPatch(after, inverseEdit, { fuzzFactor: 0 }), before);
    additions.push({ path, before, current: before, after, authored: after, inverse: before, beforeHash: sha(before), afterHash: sha(after), forward, inverseEdit, originalBodyExact: true, lawImport: relative });
  }
  const generalScript = capture("🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/📜️script.ts"), repoScript = capture("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts");
  assert.ok(generalScript.source.includes('resolve(this.root, "../../🧪️tests/🟦️.ts")')); assert.ok(repoScript.source.includes('if (segments[0] === "rust-source-direction")'));
  const ts = require("typescript"), api = source.rows[0], absolute = resolve(root, api.path), options = { noEmit: true, strict: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true }, host = ts.createCompilerHost(options), original = host.getSourceFile.bind(host);
  host.getSourceFile = (path: string, language: any, onError: any, newFile: any) => path === absolute ? ts.createSourceFile(absolute, api.after, language, true) : original(path, language, onError, newFile);
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([absolute], options, host)).map((row: any) => ts.flattenDiagnosticMessageText(row.messageText, "\n")); assert.deepEqual(diagnostics, []);
  const rows = [...source.rows, ...additions], contexts = [...source.frames, ...frames]; for (const row of rows) assert.equal(existsSync(join(root, row.path)) ? read(row.path) : null, row.before); for (const frame of contexts) assert.equal(read(frame.path), frame.source);
  console.log(JSON.stringify({ path: save("generation-source-" + epoch + ".json", { ...source, at: new Date().toISOString(), rows, frames: contexts, predecessor: { path: sourcePath, sha256: sha(sourceBytes), source: sourceBytes, proofPath, proofHash: sha(proofBytes) }, routeReachability: { general: { package: "@semio-tech/compiler-syntax-rust-ts", target: "test", originalScript: generalScript, originalLaw: generalLaw }, repo: { package: "@semio-tech/repo-lib", target: "test-rust-source-direction", originalScript: repoScript, originalLaw: repoLaw }, generatedLaw, directImports: additions.map(row => ({ path: row.path, lawImport: row.lawImport })), aliases: false }, strictDiagnostics: diagnostics, sourceWrites: 0, scope: "Exact independently admitted six generator rows plus only two direct imports into unchanged original General and Repo law rosters. Existing permanent commands and launch registrations reach the new laws, without aliases or filters. Strict owning API diagnostics zero; no whole test/native result." }), rows: rows.length, controls: source.controls.length, strictDiagnostics: diagnostics.length, sourceWrites: 0 }));
}

/** 🦀️ Qualifies the admitted native path bodies against current foreign TypeScript observers. */
async function stageRustPathPublication(epoch: string) {
  const path = join(ticket, "🗑️generated/rust-path-native/source-1.json"), proofPath = join(ticket, "🗑️generated/rust-path-native/source-independent-1.json"), runtimePath = join(ticket, "🗑️generated/rust-path-whole/epoch-2/independent-green-whole-source-checkpoint-1.json");
  const sourceBytes = readFileSync(path, "utf8"), proofBytes = readFileSync(proofPath, "utf8"), runtimeBytes = readFileSync(runtimePath, "utf8"), source = JSON.parse(sourceBytes), proof = JSON.parse(proofBytes), runtime = JSON.parse(runtimeBytes);
  assert.equal(proof.admitted, true); assert.equal(proof.authorityHash, sha(sourceBytes)); assert.equal(runtime.ready, true); assert.equal(runtime.completed, 56); assert.equal(runtime.passed, 56); assert.equal(runtime.failed, 0); assert.equal(runtime.sharedCasesObserved, 35);
  const { hashFileV1 } = await import(join(ticket, "native-json-inputs/📜️script.ts"));
  const bindings = runtime.bindings ?? runtime.boundFiles; assert.ok(Array.isArray(bindings)); assert.equal(bindings.length, 13);
  const options = { signal: signal.signal, onProgress: (row: any) => console.error("[DEBUG] native path publication binding " + row.phase + " " + row.bytes + "/" + row.total) };
  for (const binding of bindings) assert.equal(await hashFileV1(binding.path, options), binding.sha256);
  const advances: any[] = [], currentFrames = source.contexts.map((frame: any) => {
    const current = read(frame.path); if (current !== frame.source) advances.push({ path: frame.path, before: frame.source, current, inverse: frame.source, beforeHash: sha(frame.source), currentHash: sha(current), semanticEquivalence: false });
    return { ...frame, source: current, sha256: sha(current), inverse: current };
  });
  for (const row of source.rows) assert.equal(existsSync(join(root, row.path)) ? read(row.path) : null, row.before);
  for (const frame of currentFrames) assert.equal(read(frame.path), frame.source); assert.equal(readFileSync(runtimePath, "utf8"), runtimeBytes);
  console.log(JSON.stringify({ path: save("rustpaths-source-" + epoch + ".json", { ...source, at: new Date().toISOString(), frames: currentFrames, predecessor: { path, source: sourceBytes, sha256: sha(sourceBytes), proofPath, proofSource: proofBytes, proofHash: sha(proofBytes) }, runtime: { path: runtimePath, source: runtimeBytes, sha256: sha(runtimeBytes), bindings }, codec: { path: join(ticket, "native-json-inputs/📜️script.ts"), source: readFileSync(join(ticket, "native-json-inputs/📜️script.ts"), "utf8") }, advances, sourceReady: true, sourceWrites: 0, scope: "Exact five admitted native GREEN source bodies; complete current TypeScript observer advances are retained without historical semantic equivalence. Original whole56/56 and35 shared controls proof bound; no new native run or source writes." }), rows: source.rows.length, advances: advances.length, sourceWrites: 0 }));
}

/** 📝️ Publishes independently admitted pairs under durable immediate guards. */
async function publishCohort(kind: "paths" | "generation" | "ownership" | "rustpaths" | "hubnote" | "interactionstate", sourceEpoch: string, epoch: string) {
  const authorityPath = join(directory, kind + "-source-" + sourceEpoch + ".json"), proofPath = join(directory, kind + "-source-independent-" + sourceEpoch + ".json"), authorityBytes = readFileSync(authorityPath, "utf8"), proofBytes = readFileSync(proofPath, "utf8");
  const authority = JSON.parse(authorityBytes), proof = JSON.parse(proofBytes); assert.equal(proof.admitted, true); assert.equal(proof.authorityPath, authorityPath); assert.equal(proof.authorityHash, sha(authorityBytes)); assert.equal(proof.rows, authority.rows.length);
  const publisherSource = readFileSync(import.meta.path, "utf8"), publisherFrame = { path: import.meta.path, source: publisherSource, sha256: sha(publisherSource) };
  if (kind === "paths") { assert.equal(proof.literalSubstitutions, 4); assert.equal(proof.originalBodiesOtherwiseExact, true); assert.equal(authority.rows.length, 2); } else if (kind === "generation") { assert.equal(authority.rows.length, 8); assert.equal(proof.explicitObserver, true); } else if (kind === "ownership") { assert.equal(authority.rows.length, 10); assert.equal(proof.bindings, 8); assert.equal(authority.neutralFixtureByteExact, true); assert.equal(authority.neutralSchemaByteExact, true); } else if (kind === "hubnote") { assert.equal(authority.rows.length, 6); assert.equal(authority.publicRuntimeOnly, true); assert.equal(authority.controls.assertions.length, 8); assert.equal(authority.binaryFrames.length, 1); } else if (kind === "interactionstate") { assert.equal(authority.rows.length, 4); assert.equal(authority.conservation.changedFunctions, 2); assert.equal(authority.conservation.dynamicEncodersUnchanged, true); assert.equal(authority.conservation.storeLawAndCorpusUnchanged, true); } else { assert.equal(authority.rows.length, 5); assert.equal(authority.runtime.bindings.length, 13); }
  assert.equal(authority.sourceReady, true); const completed = new Map<string, string | null>(), physicalSource = (path: string) => existsSync(join(root, path)) ? read(path) : null;
  const dispatchRows = kind === "ownership" ? [...authority.rows.filter((row: any) => row.before === null), ...authority.rows.filter((row: any) => row.before !== null && row.after !== null), ...authority.rows.filter((row: any) => row.after === null)] : authority.rows;
  const { hashFileV1 } = await import(join(ticket, "native-json-inputs/📜️script.ts"));
  const guard = async (writing?: string) => {
    if (authority.runtime) { assert.equal(readFileSync(authority.runtime.path, "utf8"), authority.runtime.source); assert.equal(readFileSync(authority.codec.path, "utf8"), authority.codec.source); for (const binding of authority.runtime.bindings) assert.equal(await hashFileV1(binding.path, { signal: signal.signal }), binding.sha256); }
    signal.signal.throwIfAborted(); assert.equal(readFileSync(authorityPath, "utf8"), authorityBytes); assert.equal(readFileSync(proofPath, "utf8"), proofBytes);
    if (kind === "hubnote") assert.equal(readFileSync(authority.controls.red.path, "utf8"), authority.controls.red.source);
    if (kind === "interactionstate") assert.equal(readFileSync(authority.inspection.path, "utf8"), authority.inspection.source);
    assert.equal(readFileSync(import.meta.path, "utf8"), publisherSource);
    for (const frame of [...authority.frames, ...(authority.inventory ?? [])]) assert.equal(physicalSource(frame.path), completed.has(frame.path) ? completed.get(frame.path) : frame.source, "defining frame advanced: " + frame.path);
    for (const frame of authority.binaryFrames ?? []) assert.deepEqual(readFileSync(join(root, frame.path)), Buffer.from(frame.base64, "base64"), "defining binary advanced: " + frame.path);
    for (const row of authority.rows) if (row.path !== writing) assert.equal(physicalSource(row.path), completed.has(row.path) ? completed.get(row.path) : row.before, "predecessor advanced: " + row.path);
  };
  await guard(); mkdirSync(directory, { recursive: true }); const journalPath = join(directory, kind + "-publication-" + epoch + ".jsonl"), journal = openSync(journalPath, "wx"), applied: any[] = [];
  const record = (value: any) => { const buffer = Buffer.from(JSON.stringify(value) + "\n"); let offset = 0; while (offset < buffer.length) offset += writeSync(journal, buffer, offset, buffer.length - offset); fsyncSync(journal); };
  try {
    record({ kind: "begin", at: new Date().toISOString(), authorityPath, authorityHash: sha(authorityBytes), proofPath, proofHash: sha(proofBytes), publisherFrame, rows: authority.rows, dispatchRows, frames: authority.frames, binaryFrames: authority.binaryFrames ?? [], runtime: authority.runtime ?? null, codec: authority.codec ?? null, advances: authority.advances ?? [] });
    if (process.platform !== "win32") { const parent = openSync(directory, "r"); try { fsyncSync(parent); } finally { closeSync(parent); } }
    for (const row of dispatchRows) {
      await guard(); const path = join(root, row.path), physical = row.before === null ? null : lstatSync(path); if (physical) { assert.equal(physical.isSymbolicLink(), false); assert.equal(physical.isFile(), true); }
      record({ kind: "row-begin", path: row.path, before: row.before, after: row.after, inverse: row.inverse, beforeHash: row.beforeHash, afterHash: row.afterHash, inode: physical?.ino ?? null, device: physical?.dev ?? null });
      mkdirSync(dirname(path), { recursive: true }); for (let parent = dirname(path); parent !== dirname(root); parent = dirname(parent)) assert.equal(lstatSync(parent).isSymbolicLink(), false);
      const fd = openSync(path, row.before === null ? "wx+" : "r+");
      try {
        const original = fstatSync(fd); if (physical) { assert.equal(original.dev, physical.dev); assert.equal(original.ino, physical.ino); }
        const beforeBytes = Buffer.from(row.before ?? ""), observed = Buffer.alloc(original.size); let offset = 0; while (offset < observed.length) { const count = readSync(fd, observed, offset, observed.length - offset, offset); assert.ok(count > 0); offset += count; } assert.deepEqual(observed, beforeBytes);
        await guard(row.path); const immediate = lstatSync(path), descriptor = fstatSync(fd); assert.equal(immediate.isSymbolicLink(), false); assert.equal(immediate.dev, descriptor.dev); assert.equal(immediate.ino, descriptor.ino); assert.equal(descriptor.size, beforeBytes.length);
        const immediatelyObserved = Buffer.alloc(descriptor.size); offset = 0; while (offset < immediatelyObserved.length) { const count = readSync(fd, immediatelyObserved, offset, immediatelyObserved.length - offset, offset); assert.ok(count > 0); offset += count; } assert.deepEqual(immediatelyObserved, beforeBytes);
        if (row.after === null) { assert.ok(physical); unlinkSync(path); assert.equal(existsSync(path), false); }
        else { const afterBytes = Buffer.from(row.after); offset = 0; while (offset < afterBytes.length) { const count = writeSync(fd, afterBytes, offset, afterBytes.length - offset, offset); assert.ok(count > 0); offset += count; } ftruncateSync(fd, afterBytes.length); fsyncSync(fd); }
        assert.equal(physicalSource(row.path), row.after); completed.set(row.path, row.after); if ((row.before === null || row.after === null) && process.platform !== "win32") { const parent = openSync(dirname(path), "r"); try { fsyncSync(parent); } finally { closeSync(parent); } } record({ kind: "row-commit", path: row.path, afterHash: row.after === null ? null : sha(readFileSync(path, "utf8")), inverse: row.inverse }); applied.push({ path: row.path, beforeHash: row.beforeHash, afterHash: row.afterHash }); console.error("[DEBUG] exact owned source published " + row.path);
      } finally { closeSync(fd); }
    }
    await guard(); record({ kind: "complete", applied, sourceWrites: applied.length, nativeExecuted: false });
  } catch (error) { record({ kind: "refused", applied, error: (error as Error).message, restoration: "No automatic restoration of shared foreign bytes" }); throw error; }
  finally { closeSync(journal); }
  console.log(JSON.stringify({ path: save(kind + "-published-" + epoch + ".json", { at: new Date().toISOString(), authorityPath, authorityHash: sha(authorityBytes), proofPath, proofHash: sha(proofBytes), journalPath, journalHash: sha(readFileSync(journalPath, "utf8")), rows: authority.rows, applied, sourceWrites: applied.length, nativeExecuted: false, scope: "Actual guarded owned source publications; complete predecessor/inverse frames conserved. No native/whole census claim." }), sourceWrites: applied.length, nativeExecuted: false }));
}

/** 🧭️ Relocates the complete neutral surface contract and preserves every owning consumer. */
async function ownership(command: string, epoch: string) {
  const schemaSource = readFileSync(join(import.meta.dir, "🧭️ownership/🧬️schema/🔣️.json"), "utf8"), fixtureSource = readFileSync(join(import.meta.dir, "🧭️ownership/🧫️fixtures/🔣️.json"), "utf8"), fixture = JSON.parse(fixtureSource);
  const { default: Ajv } = await import("ajv"), validate = new Ajv({ strict: true }).compile(JSON.parse(schemaSource)), { validateJsonSchemaSubset } = await import(join(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
  assert.equal(validate(fixture), true); assert.deepEqual(validateJsonSchemaSubset(JSON.parse(schemaSource), fixture), []); assert.equal(validate({ ...fixture, foreign: true }), false);
  const { SourceProjection } = await import(join(root, "🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts")), pathe = require("pathe"), diff = require("diff"), ts = require("typescript"), api = await import(join(root, "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts"));
  const sources = new Map<string, string>(), controls: any[] = [], rows: any[] = [];
  const add = (path: string, before: string | null, after: string | null) => {
    const forward = diff.createPatch(path, before ?? "", after ?? ""), inverseEdit = diff.createPatch(path, after ?? "", before ?? "");
    assert.equal(diff.applyPatch(before ?? "", forward, { fuzzFactor: 0 }), after ?? ""); assert.equal(diff.applyPatch(after ?? "", inverseEdit, { fuzzFactor: 0 }), before ?? "");
    rows.push({ path, before, current: before, after, authored: after, inverse: before, beforeHash: before === null ? null : sha(before), afterHash: after === null ? null : sha(after), forward, inverseEdit });
  };
  for (const move of fixture.moves) {
    const source = capture(move.from).source; assert.equal(existsSync(join(root, move.to)), false); add(move.from, source, null); add(move.to, null, source);
  }
  for (const row of fixture.bindings) {
    signal.signal.throwIfAborted(); const source = sources.get(row.path) ?? capture(row.path).source; sources.set(row.path, source);
    const afterLiteral = row.mode === "relative" ? posix.relative(posix.dirname(row.path), row.target) : 'join(repoRoot, ' + JSON.stringify(row.target) + ')';
    if (row.mode === "relative") { assert.equal(afterLiteral, pathe.relative(pathe.dirname(row.path), row.target)); assert.equal(posix.normalize(posix.join(posix.dirname(row.path), afterLiteral)), row.target); }
    const needle = row.mode === "relative" ? JSON.stringify(row.oldLiteral) : row.oldLiteral, successor = row.mode === "relative" ? JSON.stringify(afterLiteral) : afterLiteral;
    assert.equal(source.split(needle).length - 1, row.occurrences); controls.push({ ...row, needle, successor, afterLiteral, independentPath: afterLiteral });
  }
  const general = fixture.bindings[0], refs = api.inspectRustCompileReferences(sources.get(general.path)), originalGeneralViolation = refs.filter((row: any) => row.path === general.oldLiteral);
  assert.equal(originalGeneralViolation.length, 1);
  if (command === "red-ownership") {
    save("ownership-red-" + epoch + ".json", { at: new Date().toISOString(), schemaSource, fixtureSource, frames, controls, originalGeneralViolation, sourceWrites: 0, expectedTerminal: 1 });
    assert.deepEqual(originalGeneralViolation, [], "General raster residency must consume its neutral defining fixture owner"); return;
  }
  const loaded = require("web-tree-sitter"), Parser = loaded.Parser ?? loaded.default ?? loaded; await Parser.init(); const parser = new Parser();
  parser.setLanguage(await (Parser.Language ?? loaded.Language).load(join(dirname(require.resolve("tree-sitter-wasms/package.json")), "out/tree-sitter-rust.wasm")));
  const functionConservation: any[] = [];
  for (const [path, before] of sources) {
    const substitutions = controls.filter(row => row.path === path); let after = before; for (const row of substitutions) after = after.replaceAll(row.needle, row.successor); add(path, before, after);
    if (path.endsWith(".rs")) {
      const first = parser.parse(before), next = parser.parse(after); assert.equal(first.rootNode.hasError(), false); assert.equal(next.rootNode.hasError(), false);
      const original = first.rootNode.descendantsOfType("function_item"), proposed = next.rootNode.descendantsOfType("function_item"); assert.equal(original.length, proposed.length);
      for (let index = 0; index < original.length; index++) { let expected = original[index].text; for (const row of substitutions) expected = expected.replaceAll(row.needle, row.successor); assert.equal(proposed[index].text, expected); }
      functionConservation.push({ path, functions: original.length, originalBodiesExceptDeclaredBindings: true }); first.delete(); next.delete();
    } else {
      const first = ts.createSourceFile(path, before, ts.ScriptTarget.Latest, true), next = ts.createSourceFile(path, after, ts.ScriptTarget.Latest, true); assert.deepEqual(first.parseDiagnostics, []); assert.deepEqual(next.parseDiagnostics, []);
      const collect = (tree: any) => { const values: string[] = []; const visit = (node: any) => { if (ts.isFunctionDeclaration(node) || ts.isMethodDeclaration(node) || ts.isArrowFunction(node) || ts.isFunctionExpression(node)) values.push(node.getText(tree)); ts.forEachChild(node, visit); }; visit(tree); return values; };
      const original = collect(first), proposed = collect(next); assert.equal(original.length, proposed.length); for (let index = 0; index < original.length; index++) { let expected = original[index]; for (const row of substitutions) expected = expected.replaceAll(row.needle, row.successor); assert.equal(proposed[index], expected); }
      functionConservation.push({ path, functions: original.length, originalBodiesExceptDeclaredBindings: true });
    }
  }
  parser.delete();
  const projection = new SourceProjection(rows, (path: string) => existsSync(join(root, path)) ? read(path) : null);
  for (const move of fixture.moves) { assert.equal(projection.resolve(move.from), null); assert.equal(projection.resolve(move.from), null); assert.equal(projection.require(move.to), read(move.from)); assert.throws(() => projection.require(move.from), (error: any) => error.code === "deleted-reference"); }
  assert.deepEqual(projection.inverse(), rows.map(row => ({ path: row.path, before: row.after, after: row.before })));
  const neutralSchema = JSON.parse(projection.require(fixture.moves[1].to)), neutralFixture = JSON.parse(projection.require(fixture.moves[0].to)), Ajv2020 = require("ajv/dist/2020").default;
  const neutralValidate = new Ajv2020({ strict: false }).compile(neutralSchema); assert.equal(neutralValidate(neutralFixture), true); assert.deepEqual(validateJsonSchemaSubset(neutralSchema, neutralFixture), []);
  assert.equal(neutralFixture.schema, "framework.world3d.inline-surface/v1"); assert.ok(neutralFixture.pixelCases.some((row: any) => row.id === "authoredMipLinear"));
  const inventory: any[] = [];
  for (const base of ["🧰️framework", "✏️s"]) {
    let visited = 0; for await (const path of new Bun.Glob("**/*.{rs,ts,json,toml}").scan({ cwd: join(root, base), onlyFiles: true, followSymlinks: false })) {
      signal.signal.throwIfAborted(); if (path.split("/").some(part => ["node_modules", "target", "dist", ".git", "🗑️generated"].includes(part))) continue;
      const locator = base + "/" + path, source = read(locator); if (source.includes("world3d-inline-surface")) {
        const relevant = rows.find(row => row.path === locator); inventory.push({ path: locator, source, sha256: sha(source), projected: relevant ? relevant.after : source, owned: Boolean(relevant) });
      }
      if (++visited % 500 === 0) { console.error("[DEBUG] neutral fixture reference inventory " + base + " " + visited); await new Promise(resolve => setTimeout(resolve, 0)); }
    }
  }
  const remainingLiteralProviderRefs = inventory.filter(frame => !rows.some(row => row.path === frame.path) && (frame.source.includes(fixture.moves[0].from) || frame.source.includes(fixture.moves[1].from)));
  assert.deepEqual(remainingLiteralProviderRefs, []);
  for (const path of ["🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts", "🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts", "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts"]) capture(path);
  const uiManifest = frames.find(row => row.path.endsWith("🖱️ui/📦️packages/🦀️rust/Cargo.toml")); assert.deepEqual(Bun.TOML.parse(uiManifest.source), require("@iarna/toml").parse(uiManifest.source));
  const draw = frames.find(row => row.path.endsWith("🖍️draw/🦀️.rs")); assert.ok(draw.source.includes('#[path = "../../../🧪️tests/🖼️raster-residency/🦀️.rs"]'));
  for (const row of rows) assert.equal(existsSync(join(root, row.path)) ? read(row.path) : null, row.before); for (const frame of frames) assert.equal(read(frame.path), frame.source); for (const frame of inventory) assert.equal(read(frame.path), frame.source);
  console.log(JSON.stringify({ path: save("ownership-source-" + epoch + ".json", { at: new Date().toISOString(), schemaSource, fixtureSource, rows, frames, controls, functionConservation, inventory, originalGeneralViolation, neutralFixtureByteExact: true, neutralSchemaByteExact: true, publicApiChanged: false, privateHarnessRetained: true, sourceReady: true, sourceWrites: 0, nativeExecuted: false, scope: "Complete neutral fixture/schema ownership move and direct existing consumer bindings only; original predicates and private GPU harness exact. Two explicit tombstones use defining SourceProjection, no physical fallback. Full textual matching inventory is bounded; computed reads/Rust expansion and genuine native/GPU execution remain unproved." }), rows: rows.length, controls: controls.length, functions: functionConservation.reduce((sum, row) => sum + row.functions, 0), inventory: inventory.length, sourceWrites: 0 }));
}

const [command, epoch = "1"] = process.argv.slice(2);
assert.ok(["red-paths", "stage-paths", "red-generation", "stage-generation", "stage-generation-routes", "publish-paths", "publish-generation", "red-ownership", "stage-ownership", "publish-ownership", "stage-rust-path-publication", "publish-rust-paths", "publish-hubnote", "publish-interaction-state"].includes(command!));
if (command === "publish-interaction-state") await publishCohort("interactionstate", "3", epoch); else if (command === "publish-hubnote") await publishCohort("hubnote", "1", epoch); else if (command === "stage-rust-path-publication") await stageRustPathPublication(epoch); else if (command === "publish-rust-paths") await publishCohort("rustpaths", "2", epoch); else if (command === "publish-ownership") await publishCohort("ownership", "1", epoch); else if (command?.endsWith("ownership")) await ownership(command, epoch); else if (command === "publish-paths") await publishCohort("paths", "1", epoch); else if (command === "publish-generation") await publishCohort("generation", "3", epoch); else if (command === "stage-generation-routes") stageGenerationRoutes(epoch); else if (command?.endsWith("generation")) await generation(command, epoch); else await paths(command!, epoch);
