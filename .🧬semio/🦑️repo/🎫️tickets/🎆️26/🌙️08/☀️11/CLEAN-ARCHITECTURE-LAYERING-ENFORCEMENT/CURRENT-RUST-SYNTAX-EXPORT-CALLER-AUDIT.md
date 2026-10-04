# Current Rust Syntax Export and Caller Audit

Read-only source audit. No production edits, native runs, fixture updates, or Git mutations.

## Current Neutral Public Symbols

RustStructuralVisibility, RustToken, RustAttributes, RustVisibility, rustStringValue, rustTokens, rustTokenPairs, rustIdentifierSymbol, RustCompileScope, RustCompileExpansion, RustCompileReference, inspectRustCompileReferences, rustTokenText, rustTokenSegments, rustAttributes, rustVisibility, rustPathAttributes, rustFindTopLevel, RustMetadataAttributeFact, rustMetadataPath, rustMetadataAttributeHead, rustMetadataAttributes, rustMetadataDerives

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts

Named direct export/alias matches: []

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts

Named direct export/alias matches: [(7789, 'export interface RustMutationMetadataDeclarationFact { readonly name: string; readonly kind: "struct" | "enum" | "union"; readonly visibility: RustStructuralVisibility; readonly modulePath: readonly string[]; readonly derives: readonly string[]; readonly conditional?: true; readonly mutationLeaf: { readonly state: "absent" | "valid" | "malformed" | "ambiguous" | "conditional"; readonly contracts: readonly string[]; readonly detail: string | null }; }'), (7985, 'export type RustModuleMount = Readonly<{ kind: "root" } | { kind: "module"; from: string; modulePath: readonly string[]; sourceScope: readonly string[]; declarationOffset: number; inline: boolean; visibility: RustStructuralVisibility; macroUse?: true } | { kind: "include"; from: string; modulePath: readonly string[]; sourceScope: readonly string[]; line: number; path: string }>;'), (8017, 'export function inspectRustModuleGraph(files: readonly string[], readSource: (path: string) => string | undefined, options: Readonly<{ conventionalRoots?: boolean; strictManifests?: boolean; checkCancellation?: () => void; compileReferences?: ReadonlyMap<string, readonly RustCompileReference[]> }> = {}): RustModuleGraph {')]

## Captured Before Hashes: rust-syntax-extraction-before.json

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: before SHA-256 `e43ac92680773bec45f61d98e5fb91ef4aed87838624c83569d8254e25e0fdcb`; current `4aa09dbf067d976284df77f9e073f11eb4d405698132597823fdad1a73f60292`.
## Captured Before Hashes: rust-syntax-direct-callers-before.json

- `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/🧪️tests/🔮️ownership/🟦️.ts`: before SHA-256 `f334ab18d389b01ba4723c9b854ce24b815d150ff92a7efe0e63a53575b8310e`; current `e83633df6461021c3643c283e1189fab6b97798a8cd0371e79c98bb947eedb3f`.

```diff
--- before
+++ current
@@ -6,7 +6,7 @@
 import contract from "../../🧫️fixtures/🔮️ownership/🔣️.json";
 import schema from "../../🧬️schema/🔮️ownership/🔣️.json";
 import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
-import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../..");
 const read = (path: string) => readFileSync(resolve(root, path), "utf8");

```

- `🌎️hub/🧩️compositions/🪐️space/🧫️fixtures/🧪️tests/🔮️ownership/🟦️.ts`: before SHA-256 `f2170cdcd52b7aad06e7a6ff8c7842fba93014238e63e16a3c9cac6198801d9b`; current `f488ba1fd0ad7c9ad76d1ee34bb9c2cccef44e16131b44c29865d68d404c969d`.

```diff
--- before
+++ current
@@ -9,7 +9,7 @@
 import admissionSchema from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧬️schema/🔣️.json";
 import vectors from "../../../../../../✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧫️fixtures/🔣️.json";
 import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
-import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../..");
 const read = (path: string) => readFileSync(resolve(root, path), "utf8");

```

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts`: before SHA-256 `228055ddfd76db0c6390e4bbb5a26c92936f4e174257b84db2d2aababdad21b9`; current `a9eafd4355aa50bb092cce4a4824878dd60a9fac57ce3dee5757e1d16d863567`.

```diff
--- before
+++ current
@@ -5,7 +5,7 @@
 import Ajv from "ajv";
 import * as toml from "@iarna/toml";
 import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
-import { inspectRustCompileReferences, rustTokens, rustTokenPairs } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences, rustTokens, rustTokenPairs } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import contract from "../../🧫️fixtures/🖊️drawing-reader/🔣️.json";
 import { extractedDxfMutationSource } from "./🧩️preservation/🟦️.ts";
 import schema from "../../🧬️schema/🖊️drawing-reader/🔣️.json";

```

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🧩️preservation/🟦️.ts`: before SHA-256 `83f09ce1ebacd27a9bed9c6655086e9d9c05b5ce21497356541eb510b48d2ed1`; current `e72016f153f0c477813d718592149a5adee4d11f2e9320b60d56d0982b8a3a51`.

```diff
--- before
+++ current
@@ -1,7 +1,7 @@
 import { createHash } from "node:crypto";
 import { readFileSync } from "node:fs";
 import { resolve, relative } from "node:path";
-import { rustTokens, rustTokenPairs } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { rustTokens, rustTokenPairs } from "../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import contract from "../../../🧫️fixtures/🖊️drawing-reader/🔣️.json";
 
 const root = resolve(import.meta.dir, "../../../../../../..");

```

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️private-reader/🟦️.ts`: before SHA-256 `809d58c38315cbfeb227ee7a08947183957c650e93cccc321ac957feb3d754cb`; current `6e5461cfee4595a474e3cd2ce62b17c61909530f68e63fb77da2898d14042086`.

```diff
--- before
+++ current
@@ -8,7 +8,7 @@
 import schema from "../../🧬️schema/🧫️private-reader/🔣️.json";
 import composition from "../../🧫️fixtures/🧩️composition/🔣️.json";
 import { assertPrivateReaderInput, originalPrivateReaderSource } from "./🧩️preservation/🟦️.ts";
-import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../..");

```

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️private-reader/🧩️preservation/🟦️.ts`: before SHA-256 `067ebfcc3d8d60ddbdbc38f6646ee1753305099deadea913d4ee8cdffed834f9`; current `62efd30fc4e6aa20c66554e50c02bb48182145c4df67f329aabd035bc8c1215a`.

```diff
--- before
+++ current
@@ -2,7 +2,7 @@
 import { lstatSync, readFileSync } from "node:fs";
 import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
 import contract from "../../../🧫️fixtures/🧫️private-reader/🔣️.json";
-import { inspectRustCompileReferences } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences } from "../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../../..");
 const digest = (source: string) => createHash("sha256").update(source).digest("hex");

```

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts`: before SHA-256 `4ef31fff97f1129a4a4c6bc6d9f9bcbb3b8179036db6412e098a8ab77c17dffd`; current `cd2841772cfb81c677e6076866f4ef875e3827325cf48933fe4d255cb45f68ca`.

```diff
--- before
+++ current
@@ -5,7 +5,8 @@
 import Ajv from "ajv";
 import * as toml from "@iarna/toml";
 import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
-import { inspectRustCargoManifest, inspectRustCompileReferences, inspectRustModuleGraphFacts, inspectRustStructure, rustTokenPairs, rustTokens } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCargoManifest, inspectRustModuleGraphFacts, inspectRustStructure } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences, rustTokenPairs, rustTokens } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { packagesForOwner } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs";
 import contract from "../../🧫️fixtures/🧩️composition/🔣️.json";
 import selection from "../../🧫️fixtures/🌳️contribution-selection/🔣️.json";

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts`: before SHA-256 `7379f90447deb365d4e792e680dd48f647b261942cb1db40eff1fc065f9014af`; current `16894da08466d6185397cfcaeca4874aa4cf813e5dace4d42093b9eed843943f`.

```diff
--- before
+++ current
@@ -1,11 +1,12 @@
 import { posix } from "node:path";
-import { inspectRustCompileReferences, type RustCompileReference, type RustModuleContext } from "../../../🔍️discovery/🟦️.ts";
+import { type RustModuleContext } from "../../../🔍️discovery/🟦️.ts";
+import { type RustCompileReference } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import type { DependencyDirectionRule } from "../🟦️.ts";
 
-export type RustSourceReference = RustCompileReference;
-export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustSourceReference["kind"]; line: number; expansion?: RustSourceReference["expansion"] }>;
+
+export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustCompileReference["kind"]; line: number; expansion?: RustCompileReference["expansion"] }>;
 export type RustSourceOwnership = Readonly<{ contexts?: readonly RustModuleContext[]; manifestPaths?: readonly string[] }>;
-export type RustSourceTarget = Readonly<{ to: string; reference: RustSourceReference; directories: readonly string[] }>;
+export type RustSourceTarget = Readonly<{ to: string; reference: RustCompileReference; directories: readonly string[] }>;
 export type RustSourceInputNode = Readonly<{ path: string; kind: "file" | "directory" | "symlink" }>;
 export type RustSourceInputInventory = ReadonlyMap<string, RustSourceInputNode["kind"]>;
 export type RustSourceInputProblem = "linked-input" | "missing-input" | "non-directory-ancestor" | "unexpected-input-kind" | "uncensused-source";
@@ -23,13 +24,10 @@
   return !directory && target.endsWith(".rs") && !sources.has(target) ? "uncensused-source" : null;
 }
 
-/** 📎️ Uses the shared Rust source scanner as the compile-time dependency contract. */
-export function rustSourceReferences(source: string): readonly RustSourceReference[] {
-  return inspectRustCompileReferences(source);
-}
+
 
 /** 🧭️ Resolves each authored input using Rust module and Cargo manifest provenance. */
-export function rustSourceTargets(from: string, references: readonly RustSourceReference[], ownership: RustSourceOwnership = {}): readonly RustSourceTarget[] {
+export function rustSourceTargets(from: string, references: readonly RustCompileReference[], ownership: RustSourceOwnership = {}): readonly RustSourceTarget[] {
   const targets: RustSourceTarget[] = [];
   for (const reference of references) {
     if (reference.inlineBase !== undefined) {
@@ -69,7 +67,7 @@
 }
 
 /** 🏛️ Enforces the authored boundary policy on compile-time file inputs, including fixture and test edges. */
-export function rustSourceDirectionEdges(from: string, references: readonly RustSourceReference[], rules: readonly DependencyDirectionRule[], ownership: RustSourceOwnership = {}): readonly RustSourceDirectionEdge[] {
+export function rustSourceDirectionEdges(from: string, references: readonly RustCompileReference[], rules: readonly DependencyDirectionRule[], ownership: RustSourceOwnership = {}): readonly RustSourceDirectionEdge[] {
   const matches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(path));
   return distinctRustRows(rustSourceTargets(from, references, ownership).flatMap(({ to, reference }) => {
     const targetMatches = (patterns: readonly string[]): boolean => matches(patterns, to) || reference.directory === true && matches(patterns, `${to}/`);

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`: before SHA-256 `b095607c10470a3d42f33216c82143545d88c683311c99694bccdbb066fd3dab`; current `53634e53c354dbc8a1c8768e172521ede2dd791ff427f456ea716dd9551ed950`.

```diff
--- before
+++ current
@@ -1,9 +1,11 @@
+import { inspectRustCompileReferences } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { dirname, join, posix, resolve } from "node:path";
 import { lstatSync, readFileSync, readdirSync } from "node:fs";
 import { mkdir, stat as followedStat, writeFile } from "node:fs/promises";
-import { rustSourceDirectionEdges, rustSourceReferences, rustSourceTargets, rustSourceTargetProblem, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem, type RustSourceTarget } from "../🟦️.ts";
+import { rustSourceDirectionEdges, rustSourceTargets, rustSourceTargetProblem, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem, type RustSourceTarget } from "../🟦️.ts";
 import type { DependencyDirectionRule } from "../../🟦️.ts";
-import { inspectRustModuleGraph, inspectRustModuleGraphFacts, type RustCompileExpansion } from "../../../../🔍️discovery/🟦️.ts";
+import { inspectRustModuleGraph, inspectRustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
+import { type RustCompileExpansion } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { loadDependencyDirectionPolicy } from "../../🚀️bootstrap/🟦️.ts";
 import { rustCompilerAttributeOriginsClosed } from "../🔗️binding/🟦️.ts";
 import { COMPUTE_OWNERSHIP_CONTRACT_PATH, rustFamilyOwnershipActive, inspectRustFamilyOwnership, readRustFamilyOwnershipContract, type RustFamilyOwnershipProblem } from "../📍️ownership/🟦️.ts";
@@ -134,11 +136,11 @@
       pending = children.flat();
     }
     sources = new Map([...sources].sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))));
-    const compileReferences = new Map<string, ReturnType<typeof rustSourceReferences>>();
+    const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>();
     for (const [path, source] of sources) {
       await checkpoint();
       if (!path.endsWith(".rs")) continue;
-      try { compileReferences.set(path, rustSourceReferences(source)); } catch (error) { problems.push({ code: "unsupported-expression", from: path, detail: (error as Error).message }); }
+      try { compileReferences.set(path, inspectRustCompileReferences(source)); } catch (error) { problems.push({ code: "unsupported-expression", from: path, detail: (error as Error).message }); }
     }
     const graph = inspectRustModuleGraph([...sources.keys()], (path) => sources.get(path), { checkCancellation: check, compileReferences, strictManifests: true });
     if (rustFamilyOwnershipActive({ sources, graph, inventory, checkCancellation: check })) try {

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts`: before SHA-256 `85f351f174b77c6663bbab4078e8e569f6f574e7aae1c6d5f3cecd75bc1ea39d`; current `7d21470b166e41387c9b1480777e8f4b38559c20501411c450f4e86c6457f455`.

```diff
--- before
+++ current
@@ -1,6 +1,7 @@
 import {requireRecord,requireString,requireStringArray,requireExactKeys} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
 import {inspectRustBindingFacts,rustExternProviders,type RustBindingFacts} from "../🔗️binding/🟦️.ts";
-import {projectCargoProviderManifest,rustModuleScopeProof,rustTokens,rustTokenPairs,rustIdentifierSymbol,type RustModuleGraph,type RustToken} from "../../../../🔍️discovery/🟦️.ts";
+import { projectCargoProviderManifest, rustModuleScopeProof, type RustModuleGraph } from "../../../../🔍️discovery/🟦️.ts";
+import { rustTokens, rustTokenPairs, rustIdentifierSymbol, type RustToken } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import {rustSourceTargets,rustSourceTargetProblem,type RustSourceInputInventory} from "../🟦️.ts";
 
 export const COMPUTE_OWNERSHIP_CONTRACT_PATH="🧰️framework/🔨️modules/◻️2d/🧮️compute/🧫️fixtures/📍️binding-origin/🔣️.json";

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts`: before SHA-256 `0871a65f2c6dee4c630ec0574b17642348347305a25d6af5b35a6a679076fdb3`; current `27e0df5f8e532aabcaba584b3a4d4af49baad2fec3c52eaa7d3372dc21ae59fd`.

```diff
--- before
+++ current
@@ -1,6 +1,7 @@
 import { posix } from "node:path";
 import { rustSourceTargetProblem, rustSourceTargets, type RustSourceInputInventory, type RustSourceInputProblem } from "../🟦️.ts";
-import { projectCargoProviderManifest, cargoProviderTomlParser, inspectRustModuleGraphFacts, rustModuleScopeProof, rustTokens, rustTokenPairs, rustIdentifierSymbol, type CargoProviderManifestProjection, type CargoProviderTomlParser, type RustModuleContext, type RustModuleGraph, type RustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
+import { projectCargoProviderManifest, cargoProviderTomlParser, inspectRustModuleGraphFacts, rustModuleScopeProof, type CargoProviderManifestProjection, type CargoProviderTomlParser, type RustModuleContext, type RustModuleGraph, type RustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
+import { rustTokens, rustTokenPairs, rustIdentifierSymbol } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 export type RustBindingSpan = Readonly<{ start: number; end: number }>;
 export type RustBindingProblemKind = RustSourceInputProblem | "missing-manifest" | "invalid-manifest" | "missing-workspace-authority" | "unsupported-dependency-authority" | "external-provider-unproven" | "provider-package-mismatch" | "unproven-library-identity" | "provider-source-unproven" | "unknown-extern-root" | "ambiguous-extern-root" | "local-route-unproven" | "unsupported-use-tree" | "unproven-path-namespace" | "unproven-macro-output" | "orphan-context" | "unproven-module-mount" | "unproven-attribute-output" | "unsupported-edition-namespace" | "unproven-generic-scope" | "unsupported-extern-declaration" | "unproven-source-scope";

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts`: before SHA-256 `540e76c3ad99ca2d62b43270c3988f067e98972d3f183cc6b724a19e1da6a96e`; current `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909`.

```diff
--- before
+++ current
@@ -9,7 +9,8 @@
 import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
 import ts from "typescript";
 import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
-import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans, rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../🔍️discovery/🟦️.ts";
+import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans } from "../../🔍️discovery/🟦️.ts";
+import { rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 const root = resolve(import.meta.dir, "../../../../../../..");
 const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`: before SHA-256 `428f838a896bce5d1aa3dbb275bd7a2386f491cc0620e717c6956a63f001e87d`; current `363271c9dfd4db2338b0b6254b6590ace4ab74c21c17ed55dd24f91dbf6ec69f`.

```diff
--- before
+++ current
@@ -22,7 +22,8 @@
 import { parseCanonicalWgpuPackageCatalog, parseSemanticPackageBrowserProfile } from "../🔍️discovery/🟦️.ts";
 
 import { parseGeneratorInputProjection, parseSemanticOwnedCurrentSourceRevisions, semanticExactOwnedDocumentCorrectionAuthority, semanticOwnedInputFileSnapshot, type GeneratorInputProjection, type SemanticOwnedInputFileSnapshot } from "../🔍️discovery/🟦️.ts";
-import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans, rustTokens as rustSyntaxTokens, rustTokenPairs, validateFrozenCoordinateEvidenceContracts, type RustModuleGraph, type FrozenCoordinateEvidenceContract } from "../🔍️discovery/🟦️.ts";
+import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans, validateFrozenCoordinateEvidenceContracts, type RustModuleGraph, type FrozenCoordinateEvidenceContract } from "../🔍️discovery/🟦️.ts";
+import { rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { validateFrozenMarkdownCoordinateEvidenceContracts, type FrozenMarkdownCoordinateEvidenceContract } from "../🔍️discovery/🟦️.ts";
 import { cargoPackageRootBuildScriptPath, classifyPackageSource, classifyPackageSourceDisposition, fixedSourceDispositionDecision, implementationLeafBasenameFinding, jsonDocumentDuplicateKeys, mutationCatalogSourceOwner, mutationCatalogSourceOwnersProblems, mutationOwnerIdentity, mutationOwnerRelativePath, mutationPayloadSchemaProblems, subsetIdForDirectoryName, targetInsidePackageBoundaryFinding, taxonomyFileKindIsImplementation } from "../🔍️discovery/🟦️.ts";
 import { pathEmojiStatuteFindings, reservedDocumentationBasename } from "../../../../../🔨️modules/🪪️identity/🛣️path/🟦️.ts";

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts`: before SHA-256 `480579ac63569a9a4b5fc6ae8842ca6cd0eaff29a0b0281bd8f601fb92168f4e`; current `6a6bcc3b132fbda732426bd4e94d862ff0ab9bf0f55ad4e498a25cd5ef4ce16a`.

```diff
--- before
+++ current
@@ -1,7 +1,8 @@
 import { dirname, join, posix, relative, resolve } from "node:path";
 import type { BreachRecord } from "../../../🟦️.ts";
 import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../🔍️discovery/🟦️.ts";
-import { inspectRustCompileReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustStructure, type RustModuleGraph, type RustModuleContext, type RustModuleParticipation, type RustModuleParticipationReason } from "../../../🔍️discovery/🟦️.ts";
+import { inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustStructure, type RustModuleGraph, type RustModuleContext, type RustModuleParticipation, type RustModuleParticipationReason } from "../../../🔍️discovery/🟦️.ts";
+import { inspectRustCompileReferences } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { POLICY_RS_COMPONENT_LEAF_NAME, policyArtifactRootOfMutationsDir, policyStripEmoji } from "../🪪️identity/🟦️.ts";
 import { mutationTaxonomyCancelled, mutationTaxonomyCompare, mutationTaxonomyStructuralView, type MutationTaxonomyAssignmentRow, type MutationTaxonomyInventoryOptions, type MutationTaxonomySourceRecord } from "../📸️captured-source/🟦️.ts";
 import { mutationTaxonomySourceIndex, mutationTaxonomySourceSnapshot, type MutationTaxonomySourceIndex } from "../📇️index/🟦️.ts";

```

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🟦️.ts`: before SHA-256 `da863fbdc1f2b8d33e7eeebb79dc93eabd29c707d424c0bcfb69809dc0e1d682`; current `3bd6e2f2961e0e71ca1d5ba379d9c635e701e1b75e99c42a0be3ab761140f3f9`.

```diff
--- before
+++ current
@@ -4,7 +4,7 @@
 import { resolve } from "node:path";
 import Ajv from "ajv/dist/2020.js";
 import { inspectRustBindingFacts } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts";
-import { rustTokens, rustTokenPairs } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 type Channel = Readonly<{ snapshot: string; diff: string; mutation: string; leafOwner: string; command: string; editor: string; viewer: string; dialect: string; schema: string }>;
 type Fixture = Readonly<{ version: 1; source: string; originalSource: string; originalSha256: string; canonicalHandles: readonly string[]; channels: readonly Channel[] }>;

```

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧱️architecture/🟦️.ts`: before SHA-256 `be20256f64e3f04e21f91f54c01d011693c587965f0f3b5278d4b053f7fad0d9`; current `e11bbcadd40423836ff4dc82dcfd05a1c09bdfa9b475f2166871611faa609d1e`.

```diff
--- before
+++ current
@@ -8,7 +8,7 @@
 import TOML from "@iarna/toml";
 import ts from "typescript";
 import { createToken, Lexer } from "chevrotain";
-import { rustTokens, rustTokenPairs } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
+import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 
 type Specimen = Readonly<{ path: string; sha256: string; bytes: number; source: string }>;
 type Mapping = Readonly<{ role: string; source: string; destination: string; sha256: string }>;

```

## Captured Before Hashes: rust-syntax-remove-repo-aliases-before.json

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts`: before SHA-256 `310ced880dab2d2053140813a891c261bc700137e31c2ffadd5dbf78ab3e7cf7`; current `16894da08466d6185397cfcaeca4874aa4cf813e5dace4d42093b9eed843943f`.

```diff
--- before
+++ current
@@ -1,12 +1,12 @@
 import { posix } from "node:path";
 import { type RustModuleContext } from "../../../🔍️discovery/🟦️.ts";
-import { inspectRustCompileReferences, type RustCompileReference } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
+import { type RustCompileReference } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import type { DependencyDirectionRule } from "../🟦️.ts";
 
-export type RustSourceReference = RustCompileReference;
-export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustSourceReference["kind"]; line: number; expansion?: RustSourceReference["expansion"] }>;
+
+export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustCompileReference["kind"]; line: number; expansion?: RustCompileReference["expansion"] }>;
 export type RustSourceOwnership = Readonly<{ contexts?: readonly RustModuleContext[]; manifestPaths?: readonly string[] }>;
-export type RustSourceTarget = Readonly<{ to: string; reference: RustSourceReference; directories: readonly string[] }>;
+export type RustSourceTarget = Readonly<{ to: string; reference: RustCompileReference; directories: readonly string[] }>;
 export type RustSourceInputNode = Readonly<{ path: string; kind: "file" | "directory" | "symlink" }>;
 export type RustSourceInputInventory = ReadonlyMap<string, RustSourceInputNode["kind"]>;
 export type RustSourceInputProblem = "linked-input" | "missing-input" | "non-directory-ancestor" | "unexpected-input-kind" | "uncensused-source";
@@ -24,13 +24,10 @@
   return !directory && target.endsWith(".rs") && !sources.has(target) ? "uncensused-source" : null;
 }
 
-/** 📎️ Uses the shared Rust source scanner as the compile-time dependency contract. */
-export function rustSourceReferences(source: string): readonly RustSourceReference[] {
-  return inspectRustCompileReferences(source);
-}
+
 
 /** 🧭️ Resolves each authored input using Rust module and Cargo manifest provenance. */
-export function rustSourceTargets(from: string, references: readonly RustSourceReference[], ownership: RustSourceOwnership = {}): readonly RustSourceTarget[] {
+export function rustSourceTargets(from: string, references: readonly RustCompileReference[], ownership: RustSourceOwnership = {}): readonly RustSourceTarget[] {
   const targets: RustSourceTarget[] = [];
   for (const reference of references) {
     if (reference.inlineBase !== undefined) {
@@ -70,7 +67,7 @@
 }
 
 /** 🏛️ Enforces the authored boundary policy on compile-time file inputs, including fixture and test edges. */
-export function rustSourceDirectionEdges(from: string, references: readonly RustSourceReference[], rules: readonly DependencyDirectionRule[], ownership: RustSourceOwnership = {}): readonly RustSourceDirectionEdge[] {
+export function rustSourceDirectionEdges(from: string, references: readonly RustCompileReference[], rules: readonly DependencyDirectionRule[], ownership: RustSourceOwnership = {}): readonly RustSourceDirectionEdge[] {
   const matches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(path));
   return distinctRustRows(rustSourceTargets(from, references, ownership).flatMap(({ to, reference }) => {
     const targetMatches = (patterns: readonly string[]): boolean => matches(patterns, to) || reference.directory === true && matches(patterns, `${to}/`);

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`: before SHA-256 `e2ddba6536ad74f299270825c9dce64e72ab44e7256941f0923df1da610c82da`; current `53634e53c354dbc8a1c8768e172521ede2dd791ff427f456ea716dd9551ed950`.

```diff
--- before
+++ current
@@ -1,7 +1,8 @@
+import { inspectRustCompileReferences } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { dirname, join, posix, resolve } from "node:path";
 import { lstatSync, readFileSync, readdirSync } from "node:fs";
 import { mkdir, stat as followedStat, writeFile } from "node:fs/promises";
-import { rustSourceDirectionEdges, rustSourceReferences, rustSourceTargets, rustSourceTargetProblem, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem, type RustSourceTarget } from "../🟦️.ts";
+import { rustSourceDirectionEdges, rustSourceTargets, rustSourceTargetProblem, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem, type RustSourceTarget } from "../🟦️.ts";
 import type { DependencyDirectionRule } from "../../🟦️.ts";
 import { inspectRustModuleGraph, inspectRustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
 import { type RustCompileExpansion } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
@@ -135,11 +136,11 @@
       pending = children.flat();
     }
     sources = new Map([...sources].sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))));
-    const compileReferences = new Map<string, ReturnType<typeof rustSourceReferences>>();
+    const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>();
     for (const [path, source] of sources) {
       await checkpoint();
       if (!path.endsWith(".rs")) continue;
-      try { compileReferences.set(path, rustSourceReferences(source)); } catch (error) { problems.push({ code: "unsupported-expression", from: path, detail: (error as Error).message }); }
+      try { compileReferences.set(path, inspectRustCompileReferences(source)); } catch (error) { problems.push({ code: "unsupported-expression", from: path, detail: (error as Error).message }); }
     }
     const graph = inspectRustModuleGraph([...sources.keys()], (path) => sources.get(path), { checkCancellation: check, compileReferences, strictManifests: true });
     if (rustFamilyOwnershipActive({ sources, graph, inventory, checkCancellation: check })) try {

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts`: before SHA-256 `6dc95ba2d9bdd1b6e31becad5b8d1444e6225c8a79a6d19e555e9db950d8e1e0`; current `6e63b19217c96c9585d8442ef80f23d9d182286af486a31538f97c96583b1c3d`.

```diff
--- before
+++ current
@@ -1,3 +1,4 @@
+import { inspectRustCompileReferences, type RustCompileReference } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { expect, test } from "bun:test";
 import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
 import { dirname, isAbsolute, join, relative, resolve } from "node:path";
@@ -5,13 +6,13 @@
 import { parse as parseToml } from "@iarna/toml";
 import glob from "fast-glob";
 import { normalize as oracleNormalize, join as oracleJoin, dirname as oracleDirname, isAbsolute as oracleAbsolute } from "pathe";
-import { rustSourceDirectionEdges, rustSourceReferences, rustSourceTargets, rustSourceTargetProblem, type RustSourceReference, type RustSourceOwnership, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem } from "../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
+import { rustSourceDirectionEdges, rustSourceTargets, rustSourceTargetProblem, type RustSourceOwnership, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem } from "../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
 import type { DependencyDirectionRule } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
 import { inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof } from "../../🔍️discovery/🟦️.ts";
 import { inspectRustSourceDirection, inspectRustSourceInputs, type RustSourceInputFailure } from "../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
 
 const library = resolve(import.meta.dir, "../.."), read = (path: string): any => JSON.parse(readFileSync(join(library, path), "utf8"));
-const fixture = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json") as { cases: readonly { id: string; source: string; references: readonly RustSourceReference[] }[]; rules: readonly DependencyDirectionRule[]; directions: readonly { id: string; from: string; source: string; edges: readonly RustSourceDirectionEdge[] }[]; mounts: readonly { id: string; files: Readonly<Record<string, string>>; sourcePath: string; targets: readonly string[] }[]; unsupported: readonly { id: string; source: string }[] };
+const fixture = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json") as { cases: readonly { id: string; source: string; references: readonly RustCompileReference[] }[]; rules: readonly DependencyDirectionRule[]; directions: readonly { id: string; from: string; source: string; edges: readonly RustSourceDirectionEdge[] }[]; mounts: readonly { id: string; files: Readonly<Record<string, string>>; sourcePath: string; targets: readonly string[] }[]; unsupported: readonly { id: string; source: string }[] };
 const targets = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").targets as readonly { id: string; target: string; directory?: true; inventory: readonly RustSourceInputNode[]; sources: readonly string[]; problem: RustSourceInputProblem | null }[];
 function participationProjection(graph: ReturnType<typeof inspectRustModuleGraph>) {
   return graph.participations.map((row) => !("context" in row) ? { target: row.target, state: row.state, reason: row.reason } : { target: row.target, crateRoot: row.context.crateRoot, manifestPath: row.context.manifestPath, modulePath: row.context.modulePath, sourceScope: row.context.sourceScope, mount: row.context.mount.kind, state: row.state, reason: row.state === "denied" ? row.reason : null }).sort((a, b) => Buffer.from(JSON.stringify(a)).compare(Buffer.from(JSON.stringify(b))));
@@ -124,7 +125,7 @@
 
 test("portable native path forms cannot escape the authored workspace", () => {
   for (const row of read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").escapes as readonly { id: string; from: string; source: string }[]) {
-    const refs = rustSourceReferences(row.source), path = refs[0]!.path;
+    const refs = inspectRustCompileReferences(row.source), path = refs[0]!.path;
     expect(oracleAbsolute(path) || oracleNormalize(oracleJoin(oracleDirname(row.from), path)).startsWith("../"), row.id).toBe(true);
     expect(() => rustSourceDirectionEdges(row.from, refs, fixture.rules), row.id).toThrow("escapes");
   }
@@ -170,12 +171,12 @@
 test("portable compile-time paths preserve exact literal identity and ignore lexical decoys", () => {
   const validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
   expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
-  for (const row of fixture.cases) expect(rustSourceReferences(row.source), row.id).toEqual(row.references);
+  for (const row of fixture.cases) expect(inspectRustCompileReferences(row.source), row.id).toEqual(row.references);
 });
 
 test("inline reference metadata preserves its closed portable scope contract", () => {
   const corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json"), validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
-  const rows = corpus.referenceRejections as readonly { id: string; reference: RustSourceReference; invalidSchema: boolean; ownership?: RustSourceOwnership }[];
+  const rows = corpus.referenceRejections as readonly { id: string; reference: RustCompileReference; invalidSchema: boolean; ownership?: RustSourceOwnership }[];
   expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
   for (const row of rows) {
     expect(validate({ ...corpus, cases: [{ id: row.id, source: "source", references: [row.reference] }] }), row.id).toBe(!row.invalidSchema);
@@ -197,14 +198,14 @@
       for (const [path, source] of Object.entries(row.files)) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
       const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "inline_oracle", "--crate-type", "lib", "--emit=dep-info=dependencies.d", row.root], cwd);
       expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
-      const nativeInputs = dependencies(cwd).map(oracleNormalize), refs = rustSourceReferences(row.files[row.source]!);
+      const nativeInputs = dependencies(cwd).map(oracleNormalize), refs = inspectRustCompileReferences(row.files[row.source]!);
       const targets = rustSourceDirectionEdges(row.source, refs, rules).map((edge) => edge.to);
       expect([...new Set(targets)].sort(), row.id).toEqual([...row.targets].sort());
       expect(row.targets.filter((path) => path.endsWith(".rs") || path.endsWith(".txt")).every((path) => nativeInputs.includes(path)), row.id).toBe(true);
       expect(await inspectRustSourceInputs(cwd, rustSourceTargets(row.source, refs), new Set(Object.keys(row.files)))).toEqual([]);
-      expect(() => rustSourceTargets(row.source, rustSourceReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture.txt"));'))).toThrow("manifest provenance");
-    }
-    for (const row of corpus.inlineRejections as readonly { id: string; from: string; source: string; error: string }[]) expect(() => rustSourceTargets(row.from, rustSourceReferences(row.source)), row.id).toThrow(row.error);
+      expect(() => rustSourceTargets(row.source, inspectRustCompileReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture.txt"));'))).toThrow("manifest provenance");
+    }
+    for (const row of corpus.inlineRejections as readonly { id: string; from: string; source: string; error: string }[]) expect(() => rustSourceTargets(row.from, inspectRustCompileReferences(row.source)), row.id).toThrow(row.error);
   } finally { rmSync(root, { recursive: true, force: true }); }
 }, 45_000);
 
@@ -228,7 +229,7 @@
         expect(code, errors).toBe(0);
         expect(actual.replaceAll("\r\n", "\n"), row.id).toBe(row.stdout!);
       }
-      const targets = rustSourceTargets(row.root, rustSourceReferences(row.files[row.root]!));
+      const targets = rustSourceTargets(row.root, inspectRustCompileReferences(row.files[row.root]!));
       const failures = await inspectRustSourceInputs(cwd, targets, new Set(Object.keys(row.files)));
       expect([...new Map(failures.map((failure) => [JSON.stringify(failure), failure])).values()], row.id).toEqual([...row.failures]);
     }
@@ -264,9 +265,9 @@
       const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--test", "--emit=dep-info=dependencies.d", "source.rs"], cwd, { ...process.env, CARGO_MANIFEST_DIR: cwd });
       expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
       const actual = dependencies(cwd);
-      const compileReferences = new Map(Object.entries(row.files).filter(([path]) => path.endsWith(".rs")).map(([path, source]) => [path, rustSourceReferences(source)]));
+      const compileReferences = new Map(Object.entries(row.files).filter(([path]) => path.endsWith(".rs")).map(([path, source]) => [path, inspectRustCompileReferences(source)]));
       const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => row.files[path], { compileReferences });
-      const refs = rustSourceReferences(row.files[row.sourcePath]!);
+      const refs = inspectRustCompileReferences(row.files[row.sourcePath]!);
       const rules: DependencyDirectionRule[] = [{ name: "all-inputs", severity: "error", from: { path: [".*"] }, to: { path: [".*"] } }];
       const edges = rustSourceDirectionEdges(row.sourcePath, refs, rules, { contexts: graph.contexts.get(row.sourcePath) });
       const targets = edges.filter((edge) => row.targets.includes(edge.to)).map((edge) => edge.to);
@@ -282,7 +283,7 @@
   if (!output) throw new Error("Rust macro oracle requires caller-owned output");
   mkdirSync(output, { recursive: true });
   const root = realpathSync(mkdtempSync(join(output, "rust-macro-inputs-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
-  const rows = corpus.macroScopes as readonly { id: string; root: string; files: Readonly<Record<string, string>>; references: readonly RustSourceReference[]; stdout: string; nativeTests?: readonly string[] }[];
+  const rows = corpus.macroScopes as readonly { id: string; root: string; files: Readonly<Record<string, string>>; references: readonly RustCompileReference[]; stdout: string; nativeTests?: readonly string[] }[];
   const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json"), validate = new Ajv({ strict: true }).compile(schema);
   expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
   expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
@@ -310,7 +311,7 @@
     for (const name of nativeTests) expect(lawRun.stdout).toContain("test " + name + " ... ok");
     const nativeInputs = dependencies(root).map(oracleNormalize), expectedInputs: string[] = ["macro-host.rs"];
     for (const row of rows) {
-      const references = rustSourceReferences(row.files[row.root]!);
+      const references = inspectRustCompileReferences(row.files[row.root]!);
       expect(references, row.id).toEqual(row.references);
       const resolved = rustSourceTargets(row.root, references), expected = references.map((ref) => oracleNormalize(oracleJoin(oracleDirname(row.root), ref.path)));
       expect(resolved.map((target) => target.to), row.id).toEqual(expected);
@@ -331,7 +332,7 @@
         for (const [path, source] of Object.entries(row.files ?? {})) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
         const native = await nativeCommand(["rustc", "--crate-name", "macro_refusal", "--emit=dep-info=dependencies.d", "general/source.rs"], cwd);
         expect(native.status === 0, row.id + ": " + native.stdout + native.stderr).toBe(row.native);
-        expect(() => rustSourceReferences(row.source), row.id).toThrow("Unsupported Rust compile");
+        expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile");
       }
     }));
     for (const worker of workers) if (worker.status === "rejected") throw worker.reason;
@@ -347,9 +348,9 @@
   expect(validate(read("🧫️fixtures/🧱️rust-source-direction/🔣️.json")), JSON.stringify(validate.errors)).toBe(true);
   try {
     writeLayerOracle(root, row.files);
-    if (row.problem) expect(() => rustSourceReferences(row.files[row.source]!)).toThrow("Unsupported Rust compile");
+    if (row.problem) expect(() => inspectRustCompileReferences(row.files[row.source]!)).toThrow("Unsupported Rust compile");
     else {
-      const refs = rustSourceReferences(row.files[row.source]!);
+      const refs = inspectRustCompileReferences(row.files[row.source]!);
       expect(refs).toHaveLength(2);
       expect(refs.every((ref) => JSON.stringify(ref.expansion?.scope) === JSON.stringify(row.scope))).toBe(true);
     }
@@ -372,7 +373,7 @@
     for (const row of rows) {
       const cwd = join(root, row.id);
       writeLayerOracle(cwd, row.files);
-      const refs = rustSourceReferences(row.files[row.source]!);
+      const refs = inspectRustCompileReferences(row.files[row.source]!);
       expect(refs.filter(ref => ref.expansion)).toHaveLength(2);
       const report = await inspectRustSourceDirection(cwd);
       expect(report.violations, row.id).toEqual([]);
@@ -443,7 +444,7 @@
         if (row.id !== "physically-delete-parent-mount") expect(runtimeOut).toContain("sealed-input sealed-input");
         const live = Object.keys(row.files).filter((path) => !row.remove?.includes(path));
         const sources = new Map(live.map((path) => [path, readFileSync(join(cwd, path), "utf8")]));
-        const refs = new Map(live.filter((path) => path.endsWith(".rs")).map((path) => { try { return [path, rustSourceReferences(sources.get(path)!)] as const; } catch { return [path, []] as const; } }));
+        const refs = new Map(live.filter((path) => path.endsWith(".rs")).map((path) => { try { return [path, inspectRustCompileReferences(sources.get(path)!)] as const; } catch { return [path, []] as const; } }));
         const graph = inspectRustModuleGraph(live, (path) => sources.get(path), { strictManifests: true, compileReferences: refs });
         const facts = inspectRustModuleGraphFacts(sources.get(row.source)!);
         expect(facts.scopes.length, row.id).toBeGreaterThan(0);
@@ -520,18 +521,18 @@
 
 test("framework source and test paths cannot depend on implementation files", () => {
   for (const row of fixture.directions) {
-    const refs = rustSourceReferences(row.source);
+    const refs = inspectRustCompileReferences(row.source);
     expect(rustSourceDirectionEdges(row.from, refs, fixture.rules), row.id).toEqual(row.edges);
     const targets = refs.map((ref) => oracleNormalize(oracleJoin(oracleDirname(row.from), ref.path)));
     expect(row.edges.every((edge) => targets.includes(edge.to)), row.id).toBe(true);
   }
-  expect(() => rustSourceDirectionEdges("general/tests/source.rs", rustSourceReferences('include!("../../../outside.rs");'), fixture.rules)).toThrow("escapes");
+  expect(() => rustSourceDirectionEdges("general/tests/source.rs", inspectRustCompileReferences('include!("../../../outside.rs");'), fixture.rules)).toThrow("escapes");
 });
 
 test("unsupported compile expressions fail closed", () => {
-  for (const row of fixture.unsupported) expect(() => rustSourceReferences(row.source), row.id).toThrow("Unsupported Rust compile");
-  const refs = rustSourceReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../specific/fixture.txt"));');
+  for (const row of fixture.unsupported) expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile");
+  const refs = inspectRustCompileReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../specific/fixture.txt"));');
   expect(() => rustSourceDirectionEdges("general/source.rs", refs, fixture.rules)).toThrow("manifest provenance");
   expect(rustSourceDirectionEdges("general/source.rs", refs, fixture.rules, { manifestPaths: ["general/Cargo.toml"] })).toEqual([{ rule: "framework-no-implementation", from: "general/source.rs", to: "specific/fixture.txt", kind: "include_str", line: 1 }]);
-  expect(rustSourceDirectionEdges("general/source.rs", rustSourceReferences('include!(concat!(env!("OUT_DIR"), "/generated.rs"));'), fixture.rules)).toEqual([]);
+  expect(rustSourceDirectionEdges("general/source.rs", inspectRustCompileReferences('include!(concat!(env!("OUT_DIR"), "/generated.rs"));'), fixture.rules)).toEqual([]);
 });

```

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🧾️attributes/🟦️.ts`: before SHA-256 `fba3d3b4e60d83f5d84b365cdd2e6c44d89cfece2bc98bc0feb15ca9eb6f2e2d`; current `305fc7c1b5d3763ed676cf9bbfb6d18d0833a7f8b306a1aa8df073af4c23a36f`.

```diff
--- before
+++ current
@@ -1,13 +1,14 @@
+import { inspectRustCompileReferences, type RustCompileReference } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { expect, test } from "bun:test";
 import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
 import { join, resolve } from "node:path";
 import Ajv from "ajv/dist/2020.js";
 import { normalize } from "pathe";
 import {inspectRustModuleGraph,inspectRustModuleGraphFacts,rustModuleScopeProof} from "../../../🔍️discovery/🟦️.ts";
-import { rustSourceReferences, rustSourceTargets, type RustSourceReference } from "../../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
+import { rustSourceTargets } from "../../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
 import { inspectRustSourceInputs } from "../../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
 
-type Case = Readonly<{ id: string; source: string; files: Readonly<Record<string, string>>; nativeInputs: readonly string[]; result: Readonly<{ state: "resolved"; references: readonly RustSourceReference[] } | { state: "unsupported-expression" }>; removal: Readonly<{ paths: readonly string[]; native: boolean }> | null }>;
+type Case = Readonly<{ id: string; source: string; files: Readonly<Record<string, string>>; nativeInputs: readonly string[]; result: Readonly<{ state: "resolved"; references: readonly RustCompileReference[] } | { state: "unsupported-expression" }>; removal: Readonly<{ paths: readonly string[]; native: boolean }> | null }>;
 const library = resolve(import.meta.dir, "../../.."), read = (path: string): unknown => JSON.parse(readFileSync(join(library, path), "utf8"));
 const corpus = read("🧫️fixtures/🧱️rust-source-direction/🧾️attributes/🔣️.json") as { readonly schemaVersion: 1; readonly featureCases:readonly Readonly<{id:string;source:string;modulePath:readonly string[];resolved:boolean;files:Readonly<Record<string,string>>;mountPath:readonly string[];mountResolved:boolean;native:boolean;diagnostic:string}>[]; readonly cases: readonly Case[] };
 
@@ -55,9 +56,9 @@
     expect(runtimeStatus, runtimeErr).toBe(0);
     expect(runtimeOut.replaceAll("\r\n", "\n")).toBe("[DEBUG] attribute oracle\n");
     if (row.result.state === "unsupported-expression") {
-      expect(() => rustSourceReferences(row.source), row.id).toThrow("Unsupported Rust compile attribute expression");
+      expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile attribute expression");
     } else {
-      const references = rustSourceReferences(row.source), targets = rustSourceTargets("source.rs", references), sources = new Set(["source.rs"]);
+      const references = inspectRustCompileReferences(row.source), targets = rustSourceTargets("source.rs", references), sources = new Set(["source.rs"]);
       expect(references, row.id).toEqual(row.result.references);
       expect(await inspectRustSourceInputs(root, targets, sources), row.id).toEqual([]);
       if (row.removal) {
@@ -66,7 +67,7 @@
         expect(after.status === 0, row.id + ": " + after.stdout + after.stderr).toBe(row.removal.native);
         if (!row.removal.native) expect(after.stderr).toContain("couldn't read");
         expect(readFileSync(join(root, "source.rs"), "utf8"), row.id).toBe(row.source);
-        expect(rustSourceReferences(row.source), row.id).toEqual(row.result.references);
+        expect(inspectRustCompileReferences(row.source), row.id).toEqual(row.result.references);
         expect(await inspectRustSourceInputs(root, targets, sources), row.id).toEqual(row.result.references.filter((reference) => row.removal!.paths.includes(reference.path)).map((reference) => ({ code: "missing-input", to: reference.path, kind: reference.kind, line: reference.line })));
       }
     }

```

- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🧪️tests/🧱️definition-ownership/🟦️.ts`: before SHA-256 `0245e21d9147bbc76eeb6918d83593f314c3c5477fe57b798e8a736f5e44949a`; current `ee463eae4245438cf88eb73c1d63bf2a717a908fe9ebb918985a779079ecf95f`.

```diff
--- before
+++ current
@@ -1,9 +1,10 @@
+import { inspectRustCompileReferences } from "../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
 import { expect, test } from "bun:test";
 import { existsSync, readFileSync } from "node:fs";
 import { join, resolve } from "node:path";
 import Ajv from "ajv/dist/2020.js";
 import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
-import { rustSourceDirectionEdges, rustSourceReferences, rustSourceTargets } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
+import { rustSourceDirectionEdges, rustSourceTargets } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
 
 const contract = resolve(import.meta.dir, "../.."), plugin = resolve(contract, "../..");
 const read = (path: string): string => readFileSync(join(contract, path), "utf8");
@@ -17,7 +18,7 @@
 
 test("the real contract resolves its assembler outside removable artifact inputs", () => {
   const source = readFileSync(join(plugin, fixture.source), "utf8");
-  const references = rustSourceReferences(source).filter(row => row.kind === "path" && (row.path === fixture.mount || row.path.includes(fixture.forbiddenPrefix)));
+  const references = inspectRustCompileReferences(source).filter(row => row.kind === "path" && (row.path === fixture.mount || row.path.includes(fixture.forbiddenPrefix)));
   const targets = rustSourceTargets(fixture.source, references);
   expect(targets.map(row => row.to)).toEqual([fixture.definition]);
   expect(targets.map(row => row.to)).toEqual(references.map(row => oracleNormalize(oracleJoin("📇️registry/🧬️contract", row.path))));
@@ -28,7 +29,7 @@
 
 test("the assembler and declared owner inputs contain no artifact implementation", () => {
   const definition = readFileSync(join(plugin, fixture.definition), "utf8");
-  expect(rustSourceReferences(definition)).toEqual([]);
+  expect(inspectRustCompileReferences(definition)).toEqual([]);
   const project = JSON.parse(read("📦️packages/🦀️rust/📋️project.json"));
   expect(project.namedInputs.nativeSources.every((path: string) => !path.includes("/🗿️artifacts/"))).toBe(true);
   expect(definition).toContain("pub fn assemble_definition");

```

## Whole Tracked-Style TypeScript Text Scan

`rg --files -g *.ts -g *.tsx` scan: 0 stale named imports/exports of the 25 moved declarations from Repo discovery/library; 0 remaining old alias identifier files. This scan excludes ignored ticket snapshots and does not establish arbitrary computed dynamic-loader behavior.

Repo library still uses `export * from "./🔍️discovery/🟦️.ts"` at line 5604; discovery has no exports for these moved declarations. Its line-1 neutral import is consumed locally, not reexported.

The physical-reference-context test has a namespace discovery import, but its current accesses are `inspectRustManifestPathCandidates`; the Rust token declarations are now read from `rustSyntaxPath`. No moved namespace access was found in that inspected file.

## Hub Preservation Scope

All seven captured Hub direct callers have exactly their scanner import cut changed; their function bodies, assertions, fixture imports, and restoration operations remain byte-exact against the captured before source. The private reader preservation helper still verifies helper SHA-256, exact reference count and include bindings, every owned input SHA-256, and restored full caller equality. Drawing preservation still verifies every moved body SHA-256 and exact retained source equality. Composition still calls both preservation helpers and retains its original SHA-256 assertions.

## Independent Current Fixture Hash Checks

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs`: expected `67bc6e53228e4af5176f5e65ad9fd5c1d47f63e699b7a6c53c559467803d9950`, actual `67bc6e53228e4af5176f5e65ad9fd5c1d47f63e699b7a6c53c559467803d9950`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before/🔣️.json`: expected `4854f12d82cceeb19e8f69eca215d0426ad9e998b2645e64943394b48c44f2f8`, actual `4854f12d82cceeb19e8f69eca215d0426ad9e998b2645e64943394b48c44f2f8`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation/🔣️.json`: expected `c6693ad818998bde348894e25e7ab27d160bc5b417abc9e60587eec16170419c`, actual `c6693ad818998bde348894e25e7ab27d160bc5b417abc9e60587eec16170419c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome/🔣️.json`: expected `7a5a7d68df9ad1703f7e5415caa03230228f1a6a4edf85b60d86f8ee5391f8ab`, actual `7a5a7d68df9ad1703f7e5415caa03230228f1a6a4edf85b60d86f8ee5391f8ab`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation/🔣️.json`: expected `c6693ad818998bde348894e25e7ab27d160bc5b417abc9e60587eec16170419c`, actual `c6693ad818998bde348894e25e7ab27d160bc5b417abc9e60587eec16170419c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome/🔣️.json`: expected `d2a81b51136af5be8326cbd98957610a34e7017ac480f0a877898fbdc0b356a6`, actual `d2a81b51136af5be8326cbd98957610a34e7017ac480f0a877898fbdc0b356a6`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation/🔣️.json`: expected `278c0895d15af3dc45c947dd5c7e4faf92f814789bbabb2991712a0a15fe4935`, actual `278c0895d15af3dc45c947dd5c7e4faf92f814789bbabb2991712a0a15fe4935`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff/🔣️.json`: expected `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, actual `0438a5368b8508250f6caa0003673a98ed4a2c557bd93452a8acabfd03a0c87c`, match `True`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome/🔣️.json`: expected `23e65896ed2e3c1a84eac7c79d6648136d496c59bfb355612bc24cd73c77859a`, actual `23e65896ed2e3c1a84eac7c79d6648136d496c59bfb355612bc24cd73c77859a`, match `True`.

16 actual private-reader helper/input file hashes checked, 0 mismatches. Drawing hashes cover sliced bodies and require the original registered harness for runtime assurance; no native test was run.

## Actionable Findings and Proof Limits

No stale named export/import or old alias blocker found in the audited current TypeScript source. No Hub preservation body/assertion loss found in the seven captured direct callers. A successful full Repo typecheck, the registered private-reader/drawing/composition tests, and the original 32-law suite remain separate runtime proof obligations; this read-only audit does not claim those checks passed. No fixture hashes were reset and no assertions were skipped.
## Exact Captured Declaration Hashes

Hashes below are UTF-8 SHA-256 of each captured `original` declaration; current-body equivalence is outside this export/caller audit.

- `RustStructuralVisibility`: `e6f0c9aae0df743d0717e23cd7dcca41eee8c76f92383ff54c08b582e1c945db`.
- `RustTokenKind`: `a59c8236014c9e9fb7fbb2499d015f0e92ac5a0068f203864f92845b3aac575a`.
- `RustToken`: `e186cc688a3b2d7e662c11c844cf7dc277b34c08bd018c80841916c49953551a`.
- `RustAttributes`: `52b92e7291ff8ac8d8b902be6b192ea418ba540adc31c9c0ce5f7704d3732c55`.
- `RustVisibility`: `ef657de6f28fa89df98a0473c6213dd0141736404e7ad5ada9e2fabd777e0f7e`.
- `rustIdentifierPart`: `c1bfd775b52c141cf0c348c0f6cbdcf5aa002131fc7f599b49eb2aa2eb1f14a9`.
- `rustStringValue`: `b0cc2ee773294d665a9e906762c31c13f49729629dc619e73117d1819906a639`.
- `rustTokens`: `eee0d9be05f8ffd839ceba4a2551f87cf068e029f472155610f73023923101c4`.
- `rustTokenPairs`: `d24caeddf9e5d5225925ba588e3e836367716d3d0f4e0d6b0eacc491e9646833`.
- `rustIdentifierSymbol`: `6508480d0cf5c6fb1a1723e1d3aca0e78e1ea1922bd61698512e28cbd8bd2ab3`.
- `RustCompileScope`: `ee3d28b3bfc6b0039ac427bcc52060062424b86aa7ccb67b9f2aecc90a5ffd45`.
- `RustCompileExpansion`: `0e829a0a8b3eb4d9ac152dd5f59931768bdcbec1f7f328725024c6d654f16a4a`.
- `RustCompileReference`: `a0395496e848eaf37f42bd6aa6a1cb692f0ab92b272c436f07c17be115c8fa77`.
- `inspectRustCompileReferences`: `b710c3420979dbe289d456f6ee79520f411b9604d90c3cf6a611b1f1a60770ad`.
- `rustTokenText`: `72e8f141262bbbc4321292e9b2d9e8605e6650411b33172ff95fd0d8b0863b62`.
- `rustTokenSegments`: `fa0dd91fe04ac5ed1907e11d7b4207954349d1bfb30307491f4612f6a7aa7a85`.
- `rustAttributes`: `be0e47993940ff444c7866c1806f89c4845af3bb89c6abf19c7e7b26754616fc`.
- `rustVisibility`: `b86234b80a4d877c9eea5be3af54065978f126f316dbd904178924f062a6e88c`.
- `rustPathAttributes`: `ac731b45c5e3d9b6c5e38ad135043cecd488e7ba56a461868097ecc039a34b52`.
- `rustFindTopLevel`: `39f10715c5a3e7b91e1bae09183cd2e1910a2de2e6212f38c8a58e746d84c7b3`.
- `RustMetadataAttributeFact`: `932fcb78acbf2df4e0c238b67332e53d83c0d6414a1beb80b545c631490c26ab`.
- `rustMetadataPath`: `11189fd139c8754d080ee3e22a2429deb0e32d89fd278239a98b452741a69502`.
- `rustMetadataAttributeHead`: `90bfdc9819521111b6630b2e0c716bbb402eefec1d227359245ecc5605737978`.
- `rustMetadataAttributes`: `e17f719bdfcc72a220a2ac53e09cffe8728b9823f4ab84288232531b23782522`.
- `rustMetadataDerives`: `79ca46859c93085dd100a032f31521b7d642535682583c72f8db762b8bf67f10`.
