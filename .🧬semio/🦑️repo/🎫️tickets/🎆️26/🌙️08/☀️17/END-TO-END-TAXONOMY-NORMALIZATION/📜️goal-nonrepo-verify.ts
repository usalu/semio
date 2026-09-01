// Fail-before / pass-after verification for the rust-path-join-unproven proven-non-repo fix.
import { execSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, posix, relative, resolve, sep } from "node:path";
import * as ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import * as discoveryNew from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

const root = "/Users/ueli/Documents/semio";
const normPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const normRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const functions = new Set(["sha256", "canonicalArrayKey", "canonicalValue", "canonicalJson", "generatorPathCompare", "sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "lstatOrNull", "checkCancellation", "ancestorReferenceCoordinateRoot", "lineLocation", "regexTokens", "rustTokens", "rustCodeOnlyTextForMacroTrust", "referenceTokens", "referenceAdapter", "unsupportedReferenceTokens", "addUniqueIndex", "referencePathIndex", "rustContextFiles", "unprovenRustReferenceTargets", "rustReferenceNeedsOwnership", "rustReferenceGraph", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers", "referenceTokensIncludingUnsupported", "splitTokenSuffix", "resolveReferencePath", "resolveReferenceTokenPath"]);
const constants = new Set(["LEXICAL_OPAQUE_ROOTS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS", "RUST_RESERVED_KEYWORDS", "indexedLineContent", "indexedLineStarts", "rustReferenceGraphs", "rustUnprovenReferenceTargets", "rustReferenceContextFiles"]);

function extract(source: string): string {
  const syntax = ts.createSourceFile(normPath, source, ts.ScriptTarget.Latest, true);
  return syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isClassDeclaration(node) ? node.name?.text === "TaxonomyCancellationError" : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
}

function build(source: string) {
  const extracted = extract(source);
  const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(extracted);
  const dependencies = {
    createHash, posix, basename, dirname, join, resolve, relative, isAbsolute, sep,
    lstatSync: (p: string) => lstatSync(p),
    readFileSync: (...args: Parameters<typeof readFileSync>) => (readFileSync as any)(...args),
    existsSync: (p: string) => existsSync(p),
    ...discoveryNew,
  };
  return new Function(...Object.keys(dependencies), compiled + "\nreturn { index: referencePathIndex, tokens: rustManifestReferenceTokens };")(...Object.values(dependencies));
}

const currentSource = readFileSync(normPath, "utf8");
const oldSource = execSync(`git show HEAD:"${normRel}"`, { cwd: root, maxBuffer: 1024 * 1024 * 64 }).toString();

const oldImpl = build(oldSource);
const newImpl = build(currentSource);

function walk(dir: string, out: string[]): void {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) { out.push(relative(root, full).split(sep).join("/")); walk(full, out); }
    else if (entry.isFile()) out.push(relative(root, full).split(sep).join("/"));
  }
}

const targets = [
  { label: "vector-converter (CLI-arg + fn-parameter, ticket main.rs)", path: ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w18-mutation-fixture-completeness/🏗️vector-converter/src/main.rs", expectCleared: ["📸️snapshot", "⬅️before", "🦠️mutation", "🔺️diff", "🎯️outcome"], expectStillBlocked: ["🧪️tests"] },
  { label: "os dsl/derive materialize() (env::var_os/temp_dir helper hop)", path: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs", expectCleared: ["domain/🧬️mutations", "🆕️insert-page"], expectStillBlocked: ["nx.json"] },
  { label: "svg generator (fn-parameter out_dir)", path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🏭️generator/🦀️quick-xml-svg-codec/src/main.rs", expectCleared: ["before.svg", "after.svg"], expectStillBlocked: [] },
  { label: "dxf generator (fn-parameter out_dir)", path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🏭️generator/🦀️engine/src/main.rs", expectCleared: ["before.dxf", "after.dxf"], expectStillBlocked: [] },
];

let failures = 0;
for (const target of targets) {
  const abs = resolve(root, target.path);
  const content = readFileSync(abs, "utf8");
  const dir = dirname(abs);
  const files: string[] = [];
  walk(dir, files);
  const relPath = target.path;
  const before = oldImpl.index(files, root, [], files, undefined, new Set());
  const after = newImpl.index(files, root, [], files, undefined, new Set());
  const oldTokens = oldImpl.tokens(relPath, content, before);
  const newTokens = newImpl.tokens(relPath, content, after);
  console.log("\n===", target.label);
  console.log("OLD token count:", oldTokens.length, "NEW token count:", newTokens.length);
  for (const value of target.expectCleared) {
    const oldRow = oldTokens.find((t: any) => t.value === value);
    const newRow = newTokens.find((t: any) => t.value === value);
    const oldBlocked = oldRow && oldRow.unsupportedReason && !oldRow.rewriteKind;
    const newAbsent = !newRow;
    const ok = oldBlocked && newAbsent;
    console.log(`  [${ok ? "PASS" : "FAIL"}] "${value}": fail-before(blocked)=${Boolean(oldBlocked)} pass-after(absent)=${newAbsent}`);
    if (!ok) failures++;
  }
  for (const value of target.expectStillBlocked) {
    const oldRow = oldTokens.find((t: any) => t.value === value);
    const newRow = newTokens.find((t: any) => t.value === value);
    const oldBlocked = oldRow && oldRow.unsupportedReason && !oldRow.rewriteKind;
    const newBlocked = newRow && newRow.unsupportedReason && !newRow.rewriteKind;
    const ok = oldBlocked && newBlocked;
    console.log(`  [${ok ? "PASS" : "FAIL"}] "${value}" still blocked in both: old=${Boolean(oldBlocked)} new=${Boolean(newBlocked)}`);
    if (!ok) failures++;
  }
}
console.log("\n\nTOTAL FAILURES:", failures);
process.exit(failures > 0 ? 1 : 0);
