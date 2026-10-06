import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve, dirname, posix } from "node:path";
import { createHash } from "node:crypto";
import { isDeepStrictEqual } from "node:util";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
const ticket = resolve(import.meta.dir, "..");
let root = ticket;
while (!existsSync(resolve(root, "nx.json"))) root = dirname(root);
const command = process.argv[2], epoch = process.argv[3] ?? "1";
if (!/^[1-9]\d*$/.test(epoch)) throw Error("Positive specific conformance epoch required");
const generated = resolve(ticket, "🗑️generated/specific-conformance"), sha = (body: string) => createHash("sha256").update(body).digest("hex");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
function save(path: string, value: unknown) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, JSON.stringify(value, null, 2) + "\n", { flag: "wx" }); }
const fixturePath = resolve(import.meta.dir, "🧫️fixtures/🔣️.json"), schemaPath = resolve(import.meta.dir, "🧬️schema/🔣️.json"), fixtureSource = readFileSync(fixturePath, "utf8"), schemaSource = readFileSync(schemaPath, "utf8"), fixture = JSON.parse(fixtureSource);
const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(schemaSource));
if (!validate(fixture)) throw Error("Closed specific ownership fixture refused " + JSON.stringify(validate.errors));
const { validateJsonSchemaSubset } = await import(resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
if (validateJsonSchemaSubset(JSON.parse(schemaSource), fixture).length) throw Error("First-party closed Specific fixture refused");
const hostileFixtures = [{ ...fixture, alias: "forwarding" }, ...["moved", "helpers"].flatMap(key => fixture[key].map((row: any, index: number) => ({ ...fixture, [key]: fixture[key].map((value: any, at: number) => at === index ? { ...value, directory: value.previousDirectory, module: value.previousModule } : value) })))];
for (const candidate of hostileFixtures) if (validate(candidate) || !validateJsonSchemaSubset(JSON.parse(schemaSource), candidate).length) throw Error("Canonical mapping hostile accepted");
const os = "🧰️framework/🛍️products/💻️os", grammarPath = os + "/🔨️modules/🗣️dsl/📖️grammar/🧪️tests/🔬️unit/🦀️.rs", sweep = os + "/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests";
const composition = "✏️s/🧑‍💻dev/🧩️composition", targetRoot = composition + "/🧪️tests/🧹️fixture-sweep", manifestPath = composition + "/📦️packages/🦀️rust/Cargo.toml";
const { default: RustParser } = await import("web-tree-sitter"); await RustParser.init(); const namingParser = new RustParser();
namingParser.setLanguage(await RustParser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
const mappings = [...fixture.helpers, ...fixture.moved], moduleNames = new Map(mappings.map((row: any) => [row.previousModule, row.module]));
function qualify(source: string, documentation = false) {
  const tree = namingParser.parse(source), edits = tree.rootNode.descendantsOfType(["identifier", "type_identifier"]).filter(node => moduleNames.has(node.text)).map(node => ({ start: node.startIndex, end: node.endIndex, inserted: moduleNames.get(node.text)! }));
  if (documentation) for (const node of tree.rootNode.descendantsOfType(["line_comment", "block_comment"])) {
    let parent = node.parent; while (parent && !["function_item", "struct_item", "enum_item", "trait_item", "impl_item", "const_item", "type_item"].includes(parent.type)) parent = parent.parent; if (parent) continue;
    let inserted = node.text; for (const [previous, canonical] of moduleNames) inserted = inserted.replaceAll(previous, canonical);
    inserted = inserted.replaceAll("P2-PW: ", "").replace(/\bM5\b/gu, "Specific conformance");
    if (inserted !== node.text) edits.push({ start: node.startIndex, end: node.endIndex, inserted });
  }
  let after = source; for (const edit of edits.sort((a, b) => b.start - a.start)) after = after.slice(0, edit.start) + edit.inserted + after.slice(edit.end);
  tree.delete(); return after.replaceAll("crate::os_", "semio_framework_os_kernel::os_");
}
const testNames = (source: string) => [...source.matchAll(/#\[(?:semio_framework_async_macros::async_test|test)\]\s*(?:async\s+)?fn\s+([A-Za-z0-9_]+)\s*\(/gu)].map(row => row[1]);
if (command === "test-canonical") {
  const path = resolve(generated, "source-" + epoch + ".json"), source = readFileSync(path, "utf8"), staged = JSON.parse(source);
  const entry = staged.rows.find((row: any) => row.path === targetRoot + "/🦀️.rs");
  for (const row of [...fixture.helpers, ...fixture.moved]) {
    if (!entry?.after?.includes('mod ' + row.module + ';') || !staged.rows.some((pair: any) => pair.path === targetRoot + '/' + row.directory + '/🦀️.rs' && pair.after !== null)) throw Error("RED: Canonical Specific module is absent " + row.module);
  }
  console.log("[DEBUG] nine canonical Specific directory/module owners accepted");
  save(resolve(generated, "canonical-controls-" + epoch + ".json"), { source: { path, sha256: sha(source) }, mappings: [...fixture.helpers, ...fixture.moved], accepted: true, nativeExecuted: false, productionWrites: false });
} else if (command === "test-current") {
  const manifest = parseToml(read(manifestPath)) as any;
  if (!manifest.test.some((row: any) => row.name === fixture.target)) throw Error("RED: Specific conformance target is absent");
  if (testNames(read(grammarPath)).includes(fixture.moved[0].laws[0])) throw Error("RED: General grammar still owns Specific sweep");
  console.log("[DEBUG] current Specific law ownership accepted");
} else if (command === "stage" || command === "stage-canonical") {
  const rows: any[] = [], transfers: any[] = [];
  function pair(path: string, after: string | null) {
    const before = existsSync(resolve(root, path)) ? read(path) : null;
    rows.push({ path, before, after, inverseBody: before, beforeHash: before === null ? null : sha(before), afterHash: after === null ? null : sha(after), forward: { path, before, after }, inverse: { path, before: after, after: before } });
  }
  const grammar = read(grammarPath), first = grammar.indexOf("/// 🌾️"), next = grammar.indexOf("#[semio_framework_async_macros::async_test]\nasync fn recognizer_matches_plain_arrow_via_registered_edge_macro");
  if (first < 0 || next < first || !grammar.startsWith('#[path = "')) throw Error("Exact original grammar cohort required");
  const genericStart = grammar.indexOf("use semio_framework_dsl::");
  pair(grammarPath, grammar.slice(genericStart, first) + grammar.slice(next));
  const discoveryPath = "🧰️framework/🔨️modules/📁️filesystem/🔎️discovery/🦀️.rs";
  const facadePath = "🧰️framework/📦️packages/🦀️rust/🦀️.rs", facade = read(facadePath), anchor = '#[path = "../../🔨️modules/📁️filesystem/📷️snapshot/🦀️.rs"]';
  if (facade.split(anchor).length !== 2 || facade.includes("pub mod directory_discovery;")) throw Error("Defining discovery facade predecessor differs");
  pair(facadePath, facade.replace(anchor, '#[path = "../../🔨️modules/📁️filesystem/🔎️discovery/🦀️.rs"]\npub mod directory_discovery;\n\n' + anchor));
  const lawStart = grammar.indexOf("#[semio_framework_async_macros::async_test]\nasync fn every_shipped_grammar_semio_parses_and_compiles", first);
  if (lawStart < first || lawStart >= next) throw Error("Exact Specific grammar law attribute required");
  const originalGrammarLaw = grammar.slice(lawStart, next);
  const movedGrammar = 'use semio_framework::directory_discovery;\nuse semio_framework_dsl::{parse_grammar, Recognizer, SemioDialect};\n\n/// 🌾️ Parses and compiles every shipped Specific grammar with the selected OS family.\n' + qualify(originalGrammarLaw);
  pair(targetRoot + "/📖️grammar/🦀️.rs", movedGrammar);
  transfers.push({ module: fixture.moved[0].module, previousModule: fixture.moved[0].previousModule, previousDirectory: fixture.moved[0].previousDirectory, directory: fixture.moved[0].directory, from: grammarPath, to: targetRoot + "/📖️grammar/🦀️.rs", before: originalGrammarLaw, after: qualify(originalGrammarLaw), laws: fixture.moved[0].laws });
  for (const row of [...fixture.moved.filter((row: any) => row.source === "sweep"), ...fixture.helpers]) {
    const from = sweep + "/" + row.previousDirectory + "/🦀️.rs", to = targetRoot + "/" + row.directory + "/🦀️.rs", before = read(from), after = qualify(before, true);
    pair(to, after); pair(from, null); transfers.push({ module: row.module, previousModule: row.previousModule, previousDirectory: row.previousDirectory, directory: row.directory, from, to, before, after, laws: row.laws ?? [] });
  }
  const entry = "//! 🧭️ Specific grammar and protocol composition laws.\n\n" + [...fixture.helpers, ...fixture.moved].map((row: any) => '#[path = "' + row.directory + '/🦀️.rs"]\nmod ' + row.module + ";").join("\n\n") + "\n";
  pair(targetRoot + "/🦀️.rs", entry);
  const sweepRootPath = sweep + "/🧹️fixture-sweep/🦀️.rs";
  pair(sweepRootPath, "//! 🧬️ OS envelope protocol laws.\n\n#[cfg(test)]\n#[path = \"../🔬️m5-semio-envelope-protocol/🦀️.rs\"]\nmod m5_semio_envelope_protocol;\n");
  const manifestBefore = read(manifestPath), parsed = parseToml(manifestBefore) as any;
  if (parsed.test.some((row: any) => row.name === fixture.target)) throw Error("Specific target already exists");
  let manifestAfter = manifestBefore + '\n[[test]]\nname = "' + fixture.target + '"\npath = "../../🧪️tests/🧹️fixture-sweep/🦀️.rs"\n';
  if (!(parsed["dev-dependencies"] ?? {})["semio-framework-dsl"]) manifestAfter = manifestAfter.replace("\n[dev-dependencies]\n", "\n[dev-dependencies]\nsemio-framework-dsl = { workspace = true }\n");
  if (!isDeepStrictEqual(parseToml(manifestAfter), Bun.TOML.parse(manifestAfter))) throw Error("Independent manifest parser differs");
  pair(manifestPath, manifestAfter);
  const owningFixturePath = composition + "/🧫️fixtures/🔣️.json", owningFixture = JSON.parse(read(owningFixturePath));
  for (const row of transfers.filter(row => row.laws.length)) owningFixture.suites.push({ target: fixture.target, path: posix.relative(composition, row.to), previousPath: row.from, sha256: sha(rows.find(pair => pair.path === row.to).after), laws: row.laws.map((law: string) => row.module + "::" + law), targetPath: posix.relative(composition, targetRoot + "/🦀️.rs") });
  pair(owningFixturePath, JSON.stringify(owningFixture, null, 2) + "\n");
  const projectPath = composition + "/📦️packages/🦀️rust/📋️project.json", project = JSON.parse(read(projectPath));
  if (project.targets["test-specific-dsl-conformance"]) throw Error("Specific owning Nx route exists");
  project.targets["test-specific-dsl-conformance"] = { executor: "nx:run-commands", cache: false, options: { cwd: composition + "/📦️packages/🦀️rust", command: "bun 📜️script.ts test --test " + fixture.target } };
  pair(projectPath, JSON.stringify(project, null, 2) + "\n");
  const { default: Parser } = await import("web-tree-sitter"); await Parser.init(); const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
  function bodies(source: string) { const tree = parser.parse(source), result = tree.rootNode.descendantsOfType("function_item").map(node => ({ name: node.childForFieldName("name")!.text, body: node.childForFieldName("body")!.text })); if (tree.rootNode.hasError()) throw Error("Rust grammar refused"); tree.delete(); return result; }
  const observations = transfers.map(row => {
    const expected = bodies(row.before).map(fn => ({ ...fn, body: qualify(fn.body) })), own = bodies(row.after), oracle = bodies(rows.find(pair => pair.path === row.to).after).filter(fn => expected.some(row => row.name === fn.name));
    if (!isDeepStrictEqual(expected, own) || !isDeepStrictEqual(own, oracle) || !isDeepStrictEqual(testNames(row.before), row.laws) || row.after !== qualify(row.before, true)) throw Error("Original law/helper body preservation refused " + row.from);
    return { from: row.from, to: row.to, functions: own.length, laws: row.laws, own, oracle, exact: true };
  });
  const generic = rows.find(row => row.path === grammarPath).after, genericBodies = bodies(grammar).filter(fn => fixture.retainedGrammar.includes(fn.name)), retainedBodies = bodies(generic);
  if (!isDeepStrictEqual(testNames(generic), fixture.retainedGrammar) || !isDeepStrictEqual(genericBodies, retainedBodies)) throw Error("Original generic laws changed");
  const envelopePath = sweep + "/🔬️m5-semio-envelope-protocol/🦀️.rs", envelope = read(envelopePath);
  if (!isDeepStrictEqual(testNames(envelope), fixture.retainedEnvelope)) throw Error("Neutral envelope law roster changed");
  const grammarChecks = rows.filter(row => row.after !== null && row.path.endsWith(".rs")).map(row => ({ path: row.path, functions: bodies(row.after).length, clean: true })); parser.delete();
  const entryTree = namingParser.parse(entry), declaredModules = entryTree.rootNode.descendantsOfType("mod_item").map(node => node.childForFieldName("name")!.text); entryTree.delete();
  if (!isDeepStrictEqual(declaredModules, mappings.map((row: any) => row.module))) throw Error("Independent Rust canonical module roster differs");
  const canonicalModules = new Set(declaredModules), moduleTargets = mappings.map((mapping: any) => {
    const path = targetRoot + "/" + mapping.directory + "/🦀️.rs", row = rows.find(row => row.path === path); if (!row?.after) throw Error("Canonical authored module target absent " + path);
    const tree = namingParser.parse(row.after), privateReferences = [...new Set(tree.rootNode.descendantsOfType(["scoped_identifier", "scoped_type_identifier"]).map(node => /^super::([A-Za-z_][A-Za-z0-9_]*)/u.exec(node.text)?.[1]).filter(Boolean))];
    if (privateReferences.some(module => !canonicalModules.has(module)) || tree.rootNode.descendantsOfType(["identifier", "type_identifier"]).some(node => moduleNames.has(node.text))) throw Error("Old private module reference survived " + path); tree.delete();
    return { previousDirectory: mapping.previousDirectory, previousModule: mapping.previousModule, directory: mapping.directory, module: mapping.module, path, sha256: row.afterHash, privateReferences, physicalBefore: row.before, authoredTarget: true, alias: false };
  });
  const qualifiedRoster = transfers.flatMap(row => row.laws.map((law: string) => ({ originalPath: row.from, previous: row.previousModule + "::" + law, canonical: row.module + "::" + law, target: fixture.target, path: row.to })));
  if (qualifiedRoster.length !== 5 || new Set(qualifiedRoster.map(row => row.canonical)).size !== 5) throw Error("Complete Specific qualified law roster differs");
  const { SourceProjection } = await import(resolve(root, "🧰️framework/🔨️modules/📁️filesystem/📷️snapshot/🟦️.ts"));
  const projection = new SourceProjection(rows, path => existsSync(resolve(root, path)) ? read(path) : null);
  const deleted = rows.filter(row => row.after === null).map(row => row.path);
  if (deleted.some(path => projection.resolve(path, () => { throw Error("Deleted source physical fallback"); }) !== null)) throw Error("Deleted source reappeared");
  const discoveryInputs = [discoveryPath, posix.dirname(discoveryPath) + "/🧪️tests/🦀️.rs", posix.dirname(discoveryPath) + "/🧫️fixtures/🔣️.json", posix.dirname(discoveryPath) + "/🧬️schema/🔣️.json"].map(path => { const source = read(path); return { path, source, sha256: sha(source) }; });
  if (!isDeepStrictEqual(testNames(discoveryInputs[1]!.source), ["cycle_safe_discovery_matches_closed_graph_corpus"])) throw Error("Defining discovery law roster changed");
  const predecessorPath = resolve(generated, "source-2.json"), predecessorSource = readFileSync(predecessorPath, "utf8");
  for (const row of rows) if ((existsSync(resolve(root, row.path)) ? read(row.path) : null) !== row.before) throw Error("Actual source advanced during canonical stage " + row.path);
  for (const frame of discoveryInputs) if (read(frame.path) !== frame.source) throw Error("Defining discovery source advanced " + frame.path);
  save(resolve(generated, "source-" + epoch + ".json"), { at: new Date().toISOString(), predecessor: { path: predecessorPath, sha256: sha(predecessorSource), semanticEquivalence: false }, fixture: { path: fixturePath, source: fixtureSource, sha256: sha(fixtureSource) }, schema: { path: schemaPath, source: schemaSource, sha256: sha(schemaSource) }, schemaControls: { firstParty: true, thirdParty: "Ajv", mappings: 9, hostileRefusals: hostileFixtures.length }, moduleTargets, qualifiedRoster, rows, transfers, observations, grammarChecks, retained: [{ path: envelopePath, source: envelope, sha256: sha(envelope), laws: fixture.retainedEnvelope }, { path: grammarPath, source: generic, sha256: sha(generic), laws: fixture.retainedGrammar }], discoveryInputs, discoveryLaw: { from: "os_dsl::grammar::original_shipped_grammar_tests::directory_discovery::tests::cycle_safe_discovery_matches_closed_graph_corpus", to: "directory_discovery::tests::cycle_safe_discovery_matches_closed_graph_corpus", owningPackage: "semio-framework", nativeExecuted: false }, deleted, inverse: projection.inverse(), nativeExecuted: false, productionWrites: false });
  console.log(JSON.stringify({ rows: rows.length, movedLaws: fixture.moved.flatMap((row: any) => row.laws).length, retainedLaws: fixture.retainedGrammar.length + fixture.retainedEnvelope.length, helpers: fixture.helpers.length, retiredSources: deleted.length, functions: observations.reduce((total, row) => total + row.functions, 0), rustGrammarClean: true, nativeExecuted: false, productionWrites: false }));
} else if (command === "test-stage") {
  const authorityPath = resolve(generated, "source-" + epoch + ".json"), authoritySource = readFileSync(authorityPath, "utf8"), authority = JSON.parse(authoritySource);
  if (!authority.observations.every((row: any) => row.exact && isDeepStrictEqual(row.own, row.oracle)) || authority.deleted.length !== 8 || authority.rows.some((row: any) => row.inverseBody !== row.before || row.beforeHash !== (row.before === null ? null : sha(row.before)) || row.afterHash !== (row.after === null ? null : sha(row.after)))) throw Error("Specific projection control refused");
  const defining = authority.rows.find((row: any) => row.path === manifestPath), manifest = parseToml(defining.after) as any;
  const ownFixtureRow = authority.rows.find((row: any) => row.path === composition + "/🧫️fixtures/🔣️.json"), ownFixture = JSON.parse(ownFixtureRow.after), owningSchema = JSON.parse(read(composition + "/🧬️schema/🔣️.json"));
  const ownValidate = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").compile(owningSchema.$defs.SCompositionLawsV1);
  if (!ownValidate(ownFixture) || !manifest.test.some((row: any) => row.name === fixture.target) || ownFixture.suites.filter((row: any) => row.target === fixture.target).flatMap((row: any) => row.laws).length !== 5) throw Error("Specific owning target/fixture refused");
  console.log("[DEBUG] Specific conformance projection: five exact original laws, six retained neutral laws, eight tombstones, closed owning schema and independent manifest parser accepted");
  save(resolve(generated, "controls-" + epoch + ".json"), { at: new Date().toISOString(), authority: { path: authorityPath, sha256: sha(authoritySource) }, movedLaws: 5, retainedNeutralLaws: 6, retiredSources: 8, controlsPassed: true, nativeExecuted: false, productionWrites: false });
} else throw Error("Unknown specific conformance command");
