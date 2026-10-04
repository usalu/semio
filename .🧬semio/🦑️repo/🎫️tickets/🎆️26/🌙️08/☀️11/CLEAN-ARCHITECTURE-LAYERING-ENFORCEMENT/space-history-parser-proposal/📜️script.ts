import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { readFileSync, writeFileSync } from "node:fs";
import ts from "typescript";
import Parser from "web-tree-sitter";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), output = join(ticket, "🗑️generated/space-history-parser"), proposalPath = join(output, "guarded-one-row-proposal-1.json"), proposal = JSON.parse(readFileSync(proposalPath, "utf8")), row = proposal.rows[0], originalPath = resolve(root, row.path), require = createRequire(join(root, "package.json")), hash = (source: string) => createHash("sha256").update(source).digest("hex"), snapshot = resolve(dirname(originalPath), "../.."), store = resolve(snapshot, "../../.."), nativePath = join(snapshot, "🧪️tests/🪶️sqlite/🦀️.rs"), bindingPath = join(snapshot, "🪶️sqlite/🦀️.rs"), storePath = join(store, "🦀️.rs");
if (process.argv[2] !== "validate") throw Error("Expected validate");
if (readFileSync(originalPath, "utf8") !== row.before || row.inverse !== row.before || hash(row.authored) !== row.authoredHash) throw Error("One-row current source guard changed");
const host = ts.createCompilerHost({}), oldRead = host.readFile.bind(host);
host.readFile = path => resolve(path) === originalPath ? row.authored : oldRead(path);
const options = { strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, esModuleInterop: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: ["bun"] }, program = ts.createProgram([originalPath], options, host), diagnostics = ts.getPreEmitDiagnostics(program).map(item => ({ code: item.code, path: item.file?.fileName, message: ts.flattenDiagnosticMessageText(item.messageText, "\n") }));
writeFileSync(join(output, "strict-typescript-1.json"), JSON.stringify({ diagnostics, version: ts.version, imported: program.getSourceFiles().filter(file => !file.isDeclarationFile).map(file => ({ path: file.fileName, hash: hash(file.text) })) }, null, 2));
if (diagnostics.length) throw Error(JSON.stringify(diagnostics));
const dependencies = new Map<string, { path: string; source: string; hash: string; inverse: string }>(), expectation = require("bun:test").expect;
for (const file of program.getSourceFiles().filter(file => !file.isDeclarationFile && resolve(file.fileName) !== originalPath)) dependencies.set(resolve(file.fileName), { path: resolve(file.fileName), source: file.text, hash: hash(file.text), inverse: file.text });
async function run(source: string, overrides = new Map<string, string>()): Promise<{ name: string; passed: boolean; error: string | null }[]> {
 const callbacks: { name: string; operation: () => unknown }[] = [], modules = new Map<string, any>();
 const read = (path: string | URL, encoding?: any) => { const absolute = resolve(path instanceof URL ? fileURLToPath(path) : String(path)), contents = overrides.get(absolute); if (contents !== undefined) return encoding ? contents : Buffer.from(contents); const actual = readFileSync(path, encoding); if (typeof actual === "string") dependencies.set(absolute, { path: absolute, source: actual, hash: hash(actual), inverse: actual }); return actual; };
 const load = (path: string, authored?: string): any => {
  if (modules.has(path)) return modules.get(path);
  const input = authored ?? readFileSync(path, "utf8"), exports: any = {}; modules.set(path, exports); if (authored === undefined) dependencies.set(path, { path, source: input, hash: hash(input), inverse: input });
  const transformed = ts.transpileModule(input.replaceAll("import.meta.url", JSON.stringify(pathToFileURL(path).href)).replaceAll("import.meta.dir", JSON.stringify(dirname(path))), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS, esModuleInterop: true } }).outputText;
  new Function("require", "exports", transformed)(name => {
   if (name === "bun:test") return { expect: expectation, test: (name: string, operation: () => unknown) => callbacks.push({ name, operation }) };
   if (name === "web-tree-sitter") return { __esModule: true, default: Parser };
   if (name === "node:fs") return { ...require(name), readFileSync: read };
   if (name.startsWith(".")) { const absolute = Bun.resolveSync(name, dirname(path)); return absolute.includes("/🔬️oracle/") ? load(absolute) : require(absolute); }
   return require(name);
  }, exports); return exports;
 };
 load(originalPath, source);
 const results = [];
 for (const callback of callbacks) { try { await callback.operation(); results.push({ name: callback.name, passed: true, error: null }); } catch (error) { results.push({ name: callback.name, passed: false, error: String(error) }); } }
 if (results.length !== 10) throw Error("Complete original ten-law source cohort changed");
 return results;
}
const before = await run(row.before), green = await run(row.authored);
writeFileSync(join(output, "hosted-ten-law-before-after-1.json"), JSON.stringify({ before, green, scope: "complete original ten callbacks hosted with exact authored TS and actual independent Bun SQLite oracle; no mounted registered/native run" }, null, 2));
if (before.filter(item => !item.passed).length !== 3 || green.some(item => !item.passed)) throw Error("Original formatting RED or complete parser successor GREEN not established");
const native = readFileSync(nativePath, "utf8"), binding = readFileSync(bindingPath, "utf8"), storeSource = readFileSync(storePath, "utf8"), contract = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"), "utf8"));
await Parser.init(); const parser = new Parser(); parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
const tree = parser.parse(native), law = tree.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === contract.nativeLaw); if (!law) throw Error("Exact current native ownership law missing");
const replaceLaw = (changed: string) => native.slice(0, law.startIndex) + changed + native.slice(law.endIndex), loop = "for physical in [false, true]", loopsChanged = law.text.replace(loop, "for physical in [false]"); if (loopsChanged === law.text) throw Error("Current parsed physical-loop witness changed");
const appended = replaceLaw(loopsChanged) + "\nfn unrelated_physical_loop() { for physical in [false, true] { let _ = physical; } }\n";
const variants = [
 { id: "wrong-roundtrip-source", path: nativePath, source: native.replace("assert_eq!(imported.value, source)", "assert_eq!(imported.value, independent)") },
 { id: "wrong-native-path", path: storePath, source: storeSource.replace("📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs", "📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/wrong.rs") },
 { id: "wrong-dialect-literal", path: bindingPath, source: binding.replace('StandardId("1")', 'StandardId("2")') },
 { id: "omitted-physical-case-despite-later-lookalike", path: nativePath, source: appended },
 { id: "parser-error-refused", path: bindingPath, source: binding + "\nfn unterminated(" },
 { id: "string-lookalike-is-not-path-attribute", path: storePath, source: storeSource.replace('path = "📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"', 'path = "wrong.rs"') + '\nconst LOOKALIKE: &str = r###"#[path = "📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"]"###;\n' }
];
tree.delete(); parser.delete();
const hostile = [];
for (const variant of variants) { const original = readFileSync(variant.path, "utf8"); if (original === variant.source) throw Error("Hostile source mutation did not alter witness"); const results = await run(row.authored, new Map([[variant.path, variant.source]])), failures = results.filter(result => !result.passed); if (!failures.length) throw Error("Semantic hostile source accepted " + variant.id); hostile.push({ ...variant, failures, refused: true }); }
const formatting = new Map([[nativePath, native.replace("assert_eq!(imported.value, source)", "assert_eq! ( imported . value , source )").replace("for physical in [false, true]", "for physical\n in [ false , true ]")], [bindingPath, binding.replace('standard: crate::os_io::StandardId("1")', 'standard : crate :: os_io :: StandardId ( "1" )')], [storePath, storeSource.replace('path = "📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"', 'path\n = "📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"')]]), formatted = await run(row.authored, formatting);
if (formatted.some(item => !item.passed)) throw Error("Equivalent parsed formatting changed source authority");
const sourceGuards = [...dependencies.values()].map(item => ({ ...item, currentHash: hash(readFileSync(item.path, "utf8")), changed: readFileSync(item.path, "utf8") !== item.source })); if (sourceGuards.some(item => item.changed) || readFileSync(originalPath, "utf8") !== row.before) throw Error("Actual source changed during hosted proof");
writeFileSync(join(output, "guarded-one-row-proof-1.json"), JSON.stringify({ proposal, before, green, hostile, formatted, sourceGuards, strictDiagnostics: diagnostics, sourceWrites: 0, mounted: false, registered: "Root owns actual registered rerun", native: "not invoked" }, null, 2));
writeFileSync(join(ticket, "CURRENT-SPACE-HISTORY-PARSER-AWARE-TEST-REPAIR.md"), "# SpaceHistory Parser-Aware Test Repair\n\nOne guarded TS source row is proposed at generated/space-history-parser/guarded-one-row-proposal-1.json. Complete current before/authored/inverse and all hashes are retained. Only formatting-sensitive assertions are changed: exact parsed Rust token sequences retain literal contents and punctuation; comments cannot imitate code, parser errors refuse, and native ownership requirements are scoped to the exact parsed named function. All original semantic assertions and schemas remain. Formatted production Rust is unchanged.\n\nThe normal Nx ticket validate command runs the complete original ten source callbacks with the actual independent Bun SQLite relational oracle. Before: seven pass/three formatting failures. Authored successor: ten pass. Six semantic hostile source cases refuse (wrong roundtrip expression, wrong path, wrong dialect, omitted physical case with a later lookalike function, parser error, and string lookalike path); formatting-equivalent Rust variants retain all ten passes. Strict TypeScript reports zero diagnostics. Full actual source/schema/fixture guards and inverses remain exact.\n\nguarded-one-row-proof-1.json and hosted-ten-law-before-after-1.json retain runtime evidence. This is the complete exact-authored hosted callback proof, not a mounted registered or native execution. Root owns one-row publication and the same actual registered gate rerun. No Cargo/native/generation or workspace source write occurred here.\n");
console.log("[DEBUG] complete original source laws before7/3 RED after10 GREEN, hostile6 refused, equivalent formatting10 GREEN, strictTS0, current source guards exact; one TS row unmounted");
