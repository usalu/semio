// 🔬️ Standalone diagnostic harness. Extracts the real, unexported Rust manifest-reference-proof
// functions straight out of 🧹️normalization/🟦️.ts (same technique 🧪️rust-finite-target-consumption
// uses) and runs them directly against REAL repo files — no `inventoryTaxonomy`/`planTaxonomy`, no
// "clean taxonomy plan" CLI, no repo-wide generator-input enumeration. Read-only. Prints which guard
// (if any) makes `rustFiniteManifestTargets`/`rustManifestReferenceTokens` bail for a given file.
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, posix, relative, resolve, sep } from "node:path";
import ts from "typescript";
import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const sourcePath = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const functions = new Set(["sha256", "canonicalArrayKey", "canonicalValue", "canonicalJson", "generatorPathCompare", "sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "lstatOrNull", "checkCancellation", "ancestorReferenceCoordinateRoot", "lineLocation", "regexTokens", "rustTokens", "rustCodeOnlyTextForMacroTrust", "referenceTokens", "referenceAdapter", "unsupportedReferenceTokens", "addUniqueIndex", "referencePathIndex", "rustContextFiles", "unprovenRustReferenceTargets", "rustReferenceNeedsOwnership", "rustReferenceGraph", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers", "referenceTokensIncludingUnsupported", "splitTokenSuffix", "resolveReferencePath", "resolveReferenceTokenPath"]);
const constants = new Set(["LEXICAL_OPAQUE_ROOTS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS", "RUST_RESERVED_KEYWORDS", "indexedLineContent", "indexedLineStarts", "rustReferenceGraphs", "rustUnprovenReferenceTargets", "rustReferenceContextFiles"]);
let extracted = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isClassDeclaration(node) ? node.name?.text === "TaxonomyCancellationError" : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");

// [DEBUG] instrument every early-return in rustFiniteManifestTargets with a checkpoint tag.
const checkpoints: [string, string][] = [
  ["if (!index.repoRoot || !view || !contexts.length || contexts.some((context) => context.manifestPath === null)) return result;", 'if (!index.repoRoot || !view || !contexts.length || contexts.some((context) => context.manifestPath === null)) { console.error("[DEBUG]", "CP1-no-view-or-contexts", path); return result; }'],
  ["if (manifests.length !== 1 || view.hashes.get(path) !== sha256(content)) return result;", 'if (manifests.length !== 1 || view.hashes.get(path) !== sha256(content)) { console.error("[DEBUG]", "CP2-manifests-or-hash", path, manifests.length); return result; }'],
  ["if (!proofPaths.includes(path)) return result;", 'if (!proofPaths.includes(path)) { console.error("[DEBUG]", "CP3-not-in-proofPaths", path); return result; }'],
  ['if (!index.contextPathSet.has(source) || !sameRoot(source) || !view.hashes.has(source)) return result;', 'if (!index.contextPathSet.has(source) || !sameRoot(source) || !view.hashes.has(source)) { console.error("[DEBUG]", "CP4-source-not-readable", source, index.contextPathSet.has(source), sameRoot(source), view.hashes.has(source)); return result; }'],
  ['if (!before?.isFile()) return result;', 'if (!before?.isFile()) { console.error("[DEBUG]", "CP5-not-a-file", source); return result; }'],
  ['if (after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.byteLength !== before.size || sha256(bytes) !== view.hashes.get(source)) return result;', 'if (after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.byteLength !== before.size || sha256(bytes) !== view.hashes.get(source)) { console.error("[DEBUG]", "CP6-changed-during-snapshot", source); return result; }'],
  ['if (!manifest.valid || manifest.dependencies.includes("std")) return result;', 'if (!manifest.valid || manifest.dependencies.includes("std")) { console.error("[DEBUG]", "CP7-manifest-invalid-or-std", manifest.valid, manifest.dependencies); return result; }'],
  ['if (/[#!]/u.test(withoutPathAttributes) || /\\bmacro\\b/u.test(text) || parentImports && /\\b(?:std|env)\\b/u.test(text)) return result;', 'if (/[#!]/u.test(withoutPathAttributes) || /\\bmacro\\b/u.test(text) || parentImports && /\\b(?:std|env)\\b/u.test(text)) { const m = /[#!]/u.exec(withoutPathAttributes); console.error("[DEBUG]", "CP8-ancestor-untrusted", source, { hashBangIndex: m?.index, hashBangContext: m ? withoutPathAttributes.slice(Math.max(0, m.index - 80), m.index + 80) : null }); return result; }'],
  ['if (physicalPath(posix.dirname(manifests[0]!), [manifest.libPath ?? "src/lib.rs"]) !== context.crateRoot) return result;', 'if (physicalPath(posix.dirname(manifests[0]!), [manifest.libPath ?? "src/lib.rs"]) !== context.crateRoot) { console.error("[DEBUG]", "CP9-crateroot-mismatch", context.crateRoot, context.modulePath); return result; }'],
  ['if (physicalPath(base, [raw]) !== next) return result;', 'if (physicalPath(base, [raw]) !== next) { console.error("[DEBUG]", "CP10-owner-target-mismatch", context.modulePath, source, next, base, raw); return result; }'],
  ['if (proven !== 1) return result;', 'if (proven !== 1) { console.error("[DEBUG]", "CP11-proven-not-1", context.modulePath, source, next, proven); return result; }'],
  ['if (error instanceof TaxonomyCancellationError) throw error;\n    return result;', 'if (error instanceof TaxonomyCancellationError) throw error;\n    console.error("[DEBUG]", "CP12-caught-error", path, error); return result;'],
];
for (const [needle, replacement] of checkpoints) {
  if (!extracted.includes(needle)) throw new Error("checkpoint needle not found: " + needle.slice(0, 60));
  extracted = extracted.replace(needle, replacement);
}

const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(extracted);

const dependencies = {
  createHash, posix, basename, dirname, join, resolve, relative, isAbsolute, sep,
  lstatSync: (path: string) => lstatSync(path),
  readFileSync: (...args: Parameters<typeof readFileSync>) => (readFileSync as any)(...args),
  existsSync: (path: string) => existsSync(path),
  inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustSyntaxTokens, rustTokenPairs,
};
const impl = new Function(...Object.keys(dependencies), compiled + "\nreturn { index: referencePathIndex, graph: rustReferenceGraph, tokens: rustManifestReferenceTokens, finite: rustFiniteManifestTargets, needsOwnership: rustReferenceNeedsOwnership, contextFiles: rustContextFiles, macroTrustText: rustCodeOnlyTextForMacroTrust, MACROS: RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS, DEFS: RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS, STDEXPR: RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS };")(...Object.values(dependencies));

function relFromRoot(p: string): string { return relative(root, resolve(root, p)).split(sep).join("/"); }

const targets = [
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations/🦀️.rs",
];
import { readdirSync } from "node:fs";
function walk(dir: string, out: string[]): void {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) { out.push(relative(root, full).split(sep).join("/")); walk(full, out); }
    else if (entry.isFile()) out.push(relative(root, full).split(sep).join("/"));
  }
}
const scopeFiles: string[] = [];
walk(resolve(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf"), scopeFiles);
walk(resolve(root, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust"), scopeFiles);

const index = impl.index(scopeFiles, root, [], scopeFiles, undefined, new Set(targets));

for (const target of targets) {
  console.log("\n=== " + target + " ===");
  const content = readFileSync(resolve(root, target), "utf8");
  console.log("needsOwnership:", impl.needsOwnership(target, [], index, []));
  const view = impl.graph(target, index);
  const contexts = view?.graph.contexts.get(target) ?? [];
  console.log("contexts.length:", contexts.length);
  for (const c of contexts) console.log("  manifestPath:", c.manifestPath, "sourceChain:", c.sourceChain, "modulePath:", c.modulePath);
  console.log("unreadableInputs:", [...(view?.unreadableInputs?.keys() ?? [])]);
  const tokens = impl.tokens(target, content, index);
  for (const t of tokens) {
    console.log(JSON.stringify({ start: t.start, end: t.end, value: t.value, structuredLocation: t.structuredLocation, unsupportedReason: t.unsupportedReason, rewriteKind: t.rewriteKind, physicalTargets: t.physicalTargets }));
  }
}
