# Off-Facet TypeScript Owned Parser Review

Status: bounded read-only preparation, 2026-08-28. Production, canonical tests, existing ticket57 inputs and launch files are unchanged. No global census, nested repository access, Git command, Cargo/rustc or native test was run.

## Decision

There is no existing owned TypeScript declaration parser in the inspected library that can satisfy the current fourteen-case contract unchanged. The nearest owned lexer is normalization's private `typescriptCollectionSyntax`; it supplies useful raw offsets and balanced-group machinery, but its deliberately restricted path-reference grammar is not the requested declaration grammar.

The smallest coherent next implementation is one new, pure TypeScript syntax/declaration domain in D, with an explicit supported grammar, owned scanner/parser/types and schema-first neutral tests. The insertion boundary is immediately after D's `//#endregion 🦀️RustStructure` and before `//#region 🧭️Discovery` ([D](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:6432)). Do not import N into D, repurpose Rust tokens, or widen N's path-admission lexer in place. N already imports D, and its reference-authority acceptance rules have independent tests. This must be a grammar-driven parser with fail-closed unsupported regions, not fourteen fixture recognizers or keyword scans.

This is a declaration-facts implementation, not proof of mutation identity. Keep the existing proposed name `inspectTypeScriptDeclarationFacts(source, language)`; do not manufacture a “mutation” result from a declaration name, `kind`/tag members, or a raw union string.

## Evidence Actually Read or Executed

- Read the complete [ticket controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts:1), [fourteen authored vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🔣️.json) and [closed JSON Schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧬️schema/🔣️.json).
- Read retained [run-sQzwPk result](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧫️run-sQzwPk/🔣️result.json): TypeScript 5.9.3 oracle 14/14, `sourceSubject.status = "missing-export"`, all four captured inputs stable. Receipt SHA-256: `12e8ee86a0127604c94694c06acd762bedb8c1aceba21fa0b85b8e99a796414a`. This is historical oracle evidence, not an owned-subject pass.
- The currently inspected D still has no `inspectTypeScriptDeclarationFacts` declaration. Its hash matches that retained receipt.
- Executed nine small source strings through the **actual captured ticket `compilerFacts` closure**, extracted by TypeScript AST and transpiled in memory. Input strings were parsed, never evaluated. No candidate filesystem paths were read. These were exploratory oracle observations, not new authored pass/fail goldens and not subject tests. Complete strings/results and closure identity are retained below.
- Read N's exact lexer, segment/template helpers and immutable path-collection consumer; D's Rust token/structure APIs, import-scanning and metadata source-proof joins; package facade import parsing and re-exports; the two exact neighboring TypeScript test owners. No dependency tree or entire-workspace scan was performed.

## Exact Reuse Map

| Existing source | Useful existing contract | Boundary preventing direct census reuse |
| --- | --- | --- |
| [N TypeScriptCollectionToken/Syntax](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:3658) | Raw source `text`, half-open `start/end`, token kind, enclosing group/scope and pair table. | Private to path-reference proof. No decoded identifier identity, line-break trivia, language mode or diagnostics. |
| [N typescriptCollectionSyntax](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:3673) | Discards comments; keeps quoted strings/flat templates/recognized regex literals atomic; rejects unclosed groups; preserves JS-string offsets. | Identifiers are ASCII `[A-Za-z_$][A-Za-z0-9_$]*`; escaped/Unicode identifiers are not recognized. Templates stop at the next unescaped backtick rather than recursively parsing interpolation. Regex classification uses previous-token heuristics. No JSX mode, TypeScript grammar, ASI, syntax-error span or declaration nodes. Returns only syntax or null. |
| [N typescriptCollectionSegments](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:3734) | Splits comma ranges while skipping balanced nested groups. | Comma-only helper; not a type/statement parser. Angle brackets, contextual `type`, expression precedence and semicolon insertion need TypeScript-specific handling. |
| [N typescriptCollectionEmbeddedExpressions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:3747) | Exposes simple interpolation expressions for a narrowly bounded binding proof. | Explicitly returns null for braces/backticks inside interpolation. It is not a recursive JavaScript template scanner. |
| [N typescriptPathCollectionReferenceAuthority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:3794) | Conservative immutable local/import binding and lexical-owner checks. | Starts with path-reader-specific `for/readFileSync/node:path/node:fs` admission. Uses globally unique binding counts, exact unescaped strings and restricted for-of readers. Its result is editable physical path spans, not declarations or provider identity. |
| [D rustTokens / delimiter helpers](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:4512) | Owned token/pair/segment design, no external AST in its public surface. | Rust comments/raw strings/chars/operators/identifier rules are not JavaScript. `rustTokenText` at 4626 normalizes whitespace; using it would destroy exact TS member spellings. Reuse the design, not Rust lexical output. |
| [D inspectRustStructure](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:5667) and [inspectRustMutationMetadataFacts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:5834) | Precedent: pure source-to-owned structural facts; provider resolution is separate. | Rust-specific declarations, imports, modules, macros and attributes. Neither is a TypeScript grammar or mutation-identity authority for TS. |
| [D scanRegistryCompilerImports](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:6889) | Public owned records `{path, kind}`; Bun capability hidden behind runtime validation. | Calls Bun.Transpiler.scanImports. It provides module paths, not declaration nodes, local/imported symbol pairs, type-only status, module scopes or spans. It is not an owned declaration parser. |
| [D registryStaticImports](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:6909) | Explicit leaf-extension language selection, no fallback. | Deduplicates/sorts paths and includes Unicode specifier repair. Alias identity and source coordinates are already discarded. Do not use it to fabricate the alias facts. |
| [D classifyPackageSourceRole](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:6987) / [N typescriptSyntax](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:2324) | Existing package-role/content hints. | Regular-expression classifications, not lexical declaration evidence; comment/string lookalikes and raw keyword matches cannot prove census identity. |
| [Package parseTsImportSpecs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:559) | Existing single-line import-path helper. | Regex paths only, no comment/string scope or aliases. Not a parser reuse candidate. |
| [Package export facade](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:6084) | Already `export *` from D. | A new D export automatically reaches the library facade; no additional package runtime export join is needed. |

The [path-collection test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-path-collection/🟦️.ts:1) independently rejects escaped identifiers, tests shadow bindings/template uses and extracts N's actual `typescriptCollection*` declarations under both Bun and TypeScript transpilation. Any shared-lexer extraction would therefore require that owner's review plus unchanged path-authority golden results. It is not needed for the first bounded D parser packet.

The [registry-language test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️registry-import-language/🟦️.test.ts:1) independently checks module-import paths and explicit language selection. That contract is useful as an architectural example, not declaration or alias-identity proof.

## Contract Clarifications Before Implementing the Parser

### Coordinates and lexical identity

The current oracle uses TypeScript `getStart(source, false)` and `node.end` ([oracle span rule](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts:65)). These are half-open **UTF-16 code-unit** offsets into exactly `sourceLines.join("\\n")`, not UTF-8 bytes or grapheme positions.

Declaration names and imported/local aliases use decoded identifier `.text`; member names and namespace/module path segments use raw `getText(source)`. A string-named ambient module therefore currently produces a modulePath segment including its quotes. Raw physical file spelling is a separate caller-owned identity. Do not normalize source, line endings or identifiers to NFC to obtain spans. The escaped-identifier vector must decode its semantic name while keeping its raw span.

Variable spans cover the individual VariableDeclaration, not `export const`; other declaration spans cover their statements. Alias spans cover individual import/export specifiers. Diagnostic spans intentionally differ by form. Preserve these distinctions and deterministic source ordering.

The current JSON Schema closes all objects, but only requires nonnegative span integers. Native/owned tests must additionally assert `0 <= start <= end <= source.length`, preserved source substrings and valid coordinate boundaries. No JSON Schema assertion presently connects those offsets to source length. Invalid language, malformed lexical input and empty input also need explicit contract cases.

### “Complete” is not full syntax, type, or mutation resolution

The current [oracle](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts:89) visits module and namespace statements. It does not resolve modules, type-check, inspect inherited members, evaluate initializers or generally enumerate declarations inside executable bodies. It sets `complete` solely when its selected diagnostic list is empty.

This already allows string/template/regex initializer summaries with `form: "unresolved"` to be complete; a blanket “any unresolved means incomplete” rewrite would contradict the reviewed fourteen-case contract. Conversely, the new observations below show that complete can coexist with omitted default aliases, object spreads, heritage or method-local declarations. Do not treat complete as a closed-world mutation census.

Conservative completeness is a prerequisite to production, not an optional reporting improvement. Before code, add desired neutral cases and fix the oracle contract for four concrete gaps: mixed default-plus-named imports must retain the default alias or diagnose it; every returned `unsupported-type` structure must carry a diagnostic; computed type-literal members must diagnose just as the existing interface/value forms do; and union constituents must be visited for nested conditional/mapped/unsupported syntax instead of only copied as raw strings. The current union branch at controller line73 returns before any such walk; the declaration branch at lines110–111 diagnoses only an immediately conditional or mapped root. The supplementary two actual observations below confirm complete/empty diagnostics for both nested union forms. Nested property/generic type regions likewise need an explicit coverage rule rather than an implicit skip.

The root must review these schema/reference refinements before an owned implementation is judged. Also decide whether object spreads, heritage, executable-local declarations and bodyless ambient modules are retained or explicitly incomplete. Recommended first boundary: module/namespace declaration summaries and exact named import/re-export facts, with explicit incompleteness whenever unrepresented syntax could hide a required alias/declaration or invalidate statement boundaries. Value evaluation and type expansion stay unresolved. Preserve the existing fourteen authored vectors as historical evidence; any newly approved expectation changes require a separate recorded contract delta, not implementation-derived goldens.

Observed, not newly approved expectations:

| Exploratory input | Actual current oracle consequence |
| --- | --- |
| Default plus named import | Retains named alias only; drops default binding; complete. |
| Object spread | Lists named `kind` member; omits spread and reports no unresolved reason; complete. |
| Interface heritage | Lists own `local` member; no `extends DomainMutation` edge; complete. |
| Computed type-literal member | `unresolved: "computed-property"`, no diagnostic; complete. Interface/variable computed forms do diagnose. |
| Method-local interface | Lists containing class and method only; inner `Mutation` absent; complete. |
| Bodyless ambient module | Empty facts, no diagnostics; complete. |
| Primitive type alias | Retained declaration with `unsupported-type`, no diagnostics; complete. |
| Parenthesized/chained assertions | Only one wrapper is removed; expression remains unresolved/incomplete. |
| String-named ambient module | Raw quoted modulePath segment; complete. |

These are limits of the current reference projection, not evidence that an absent owned implementation passed or failed those cases. Keep the existing fourteen goldens intact when adding reviewed coverage cases; do not generate desired data from implementation output.

### Mutation identity remains a separate authority

The current facts contain no admitted source locator/hash, resolved provider, descriptor provenance, payload type graph, `extends/implements` edges or unique cross-file binding proof. A local `interface Mutation` is explicitly one ordinary declaration. `import { Mutation as M } from "./protocol"` records a relationship spelling, not proof of what that module exports.

D already keeps analogous Rust steps separate: [token-derived facts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:5833) deliberately do not resolve crates, while [inspectMutationMetadataSource](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:7777) requires an already-wrapped origin, a unique module context and canonical provider routes. That Rust/Cargo resolver is not a TypeScript resolver. A later TS identity packet must bind actual admitted source bytes and exact module/provider authority, handle shadowing, re-export cycles and ambiguity, and retain unresolved cases. No filename suffix, declaration suffix, shared member name or raw discriminator-shaped union is an adequate substitute.

## Smallest Next Source and Test Footprint

1. **Schema/reference first.** Keep the existing fourteen cases. Clarify coverage and coordinate semantics, then add reviewed cases for the omissions above plus nested templates, regex/division, ASI across comments, Unicode/escaped names, dotted/merged namespaces, malformed syntax recovery and TS-vs-TSX disambiguation. Do not silently broaden the oracle.
2. **One D region only.** Define repository-owned result types matching the closed fact schema, a private TS scanner, a bounded statement/member parser and `inspectTypeScriptDeclarationFacts(source: string, language: "ts" | "tsx")`. No `typescript`/AST types, compiler fallback, source IO, taxonomy lookups or mutation-provider decision in this runtime API.
3. **Owned scanner requirements.** Keep raw UTF-16 positions and semantic identifier values separate; retain line-break boundaries for ASI; scan comment/string/template/regex/JSX states without treating literal contents as declarations. Pair ordinary delimiters and parse type-argument/expression contexts deliberately. Unsupported or malformed regions must produce explicit incomplete evidence; do not convert a null lexer result into empty complete facts. Use raw source slices for union/reference/member spellings, never pretty-print them.
4. **Owned parser scope.** Parse actual top-level/module statements for the five declaration forms, exact named import/re-export specifiers and namespace nesting. Preserve all declaration/alias occurrences by span; do not deduplicate same-spelling identities. Unsupported syntax must not let the parser jump into a comment/string/body and invent a declaration. Continue only from a proven boundary; otherwise retain an incomplete suffix diagnostic. Match the authored diagnostics for supported malformed forms without claiming general TypeScript error-recovery parity.
5. **Canonical neutral owner when root approves mounting.** Use the existing library test pattern at `🧪️tests/🧪️typescript-declaration-facts/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}`. Validate actual returned facts with the actual closed schema, run independent TypeScript compiler projection and compare authored values. Add one package-test import and exact positive collected-test count. Reuse the schema definitions rather than maintaining a descriptor/result replica.
6. **Caller integration later.** The census may consume these facts for already-admitted source strings and report per-source incompleteness. Joining mutation/provider identities, repository admission, progress/cancellation and source endpoint stability stays caller-owned. A source-size/token budget or mid-file cancellation must never return a complete empty record; if required for the next bounded scanner, first add its explicit result/exception contract. No new roots list or full-workspace scan is part of this packet.
7. **Preserve peer boundaries.** No changes to N's path/reference parser, D's Rust/metadata/registry regions, S scanner, taxonomy or admission. A future common lexer extraction can move genuinely shared machinery once both independently authored consumers prove their own acceptance boundaries. Do not duplicate N wholesale and rename it, and do not broaden its current physical-read authority to get census cases green.

## Proposed Conservative Extension Boundary (Awaiting Root Review)

The original fourteen compiler goldens remain historical/current inputs. The following are **desired** source-owned parser facts; they are deliberately not generated from the future D implementation. Each coordinate is a half-open UTF-16 range in the exact `sourceLines.join("\n")` source. Declaration spans remain the current statement/individual-variable rule. Diagnostic spans are the smallest AST/scanner region that caused the omission: an import clause part, a spread/computed member, a heritage clause, a method body, a bodyless module name/body boundary, an unsupported type node, or the parser's reported error range. Empty source uses `{start:0,end:0}`.

| Desired condition | Conservative retained facts | Proposed diagnostic code and coordinate |
| --- | --- | --- |
| Mixed default + named import | Retain every named alias with its original specifier span; do not invent a default alias record. | `unsupported-default-import` over the default binding only. |
| Object spread | Retain explicit property names only. | `unresolved-object-spread` over the `...expr` member. |
| Interface/class heritage | Retain own members and the declaration. Do not manufacture inherited members or a resolved provider edge. | `unresolved-heritage` over each `extends`/`implements` clause. |
| Computed type-literal member | Retain only noncomputed members. | `unresolved-computed-type-member` over the computed member; do not reuse a broad value-property code. |
| Conditional/mapped members nested in a union | Retain the outer union spelling in source order, never expand it. | `unresolved-conditional-union-member` or `unresolved-mapped-union-member` over the corresponding union member. |
| Primitive/unsupported type | Retain its type declaration with `form:"unresolved"` and raw empty members. | `unsupported-type-node` over the exact type node. |
| Class method-local declaration | Retain the containing class and direct member name only. Never descend into executable bodies. | `unsupported-class-member-body` over the method body, not a synthetic nested declaration. |
| Bodyless ambient module | Retain no phantom child/module declaration. | `unsupported-ambient-module-body` over the ambient module name/body boundary. |
| Empty input | Do not return a complete empty census. | `empty-source` at `{start:0,end:0}`. |
| Malformed syntax | Retain only declarations bounded before recovery uncertainty; parse diagnostics are owned and sorted by start/end/code. | `parse-error` at the TypeScript parser error span, plus `unsupported-recovery-suffix` if an owned scanner cannot prove a later statement boundary. |

Lexical-only cases—nested templates, regex versus division, ASI separated by comments, Unicode/escaped identifiers, dotted and merged namespaces, and TSX—must prove that literal/comment/body text cannot establish declarations. They are complete only when all statements end at a proven lexical boundary; otherwise the parser emits `unsupported-recovery-suffix`. Unicode semantic names may be decoded, but raw source and all ranges stay unnormalized. Dotted namespace paths use one raw/decoded segment per syntactic component, in source order; merged namespace blocks are distinct declaration occurrences, never deduplicated.

The schema needs these codes added to its closed diagnostic vocabulary before their desired vectors are admitted. The controller must report a real subject failure when the export is missing or differs; compiler-reference pass counts remain separate and never alter the expected subject status. This proposal does not choose runtime TypeScript types or a compiler dependency for D.

## Ticket Controller Repair Required Before Any Subject GREEN Claim

The current subject branch calls `pathToFileURL` at line157 without importing it. The missing-export RED masks this concrete later failure. Fix that test-harness import before executing a mounted subject.

The controller also uses a fixed eight-level workspace ascent; has no fstat/open/endpoint identity comparison in `guardedRead`; checks ancestry only from the inferred workspace down; compares results through order-sensitive JSON.stringify; and writes “Discovery lacks the proposed export” plus `expected: "missing-export"` even after a possible future subject success. Those are harness changes, not parser behavior. Move to ancestor root discovery, full-ancestry nofollow/fstat input capture, explicit subject case results/counts and truthful reference/subject receipts. Preserve earlier runs. Capture parser/helper slices as well as whole D so unrelated peer drift is distinguishable. Do not call reference14 green a subject pass.

## Captured Inputs and Declaration Anchors

The following captures were obtained with lexical any-case Compose exclusion, full-ancestry lstat, O_NOFOLLOW, fstat/endpoint metadata checks and exact byte hashes. Initial inspected whole-file hashes matched the subsequent read shown here for D, N, the facade, current packet and the two selected test owners. No source was repinned or restored.

```json
[
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "109708023",
      "mode": "33188",
      "size": "655775",
      "modifiedNs": "1787858918843947200",
      "changedNs": "1787858918843947200",
      "sha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
      "bytes": 655775
    },
    "declarations": [
      {
        "name": "RustToken",
        "startLine": 4471,
        "endLine": 4476,
        "sha256": "9bc494ade2142260796b7f5964ba4245837117b357d22fbe050438a7a722cc5f",
        "bytes": 130
      },
      {
        "name": "rustTokens",
        "startLine": 4512,
        "endLine": 4603,
        "sha256": "97e3357050e10498fe2da3e8bd17aad458bc1f937caa7dd66341d2fd37adf075",
        "bytes": 3521
      },
      {
        "name": "rustTokenPairs",
        "startLine": 4606,
        "endLine": 4623,
        "sha256": "3b104af66510903dd27261025846b262b165e66e138a2095ce57fdb9b43891cb",
        "bytes": 723
      },
      {
        "name": "rustTokenText",
        "startLine": 4626,
        "endLine": 4632,
        "sha256": "e5bb711175d33096a4a620a182f520f45cfac2dbdf1c3e2d7f2af438f3477f8e",
        "bytes": 286
      },
      {
        "name": "rustTokenSegments",
        "startLine": 4635,
        "endLine": 4650,
        "sha256": "b1b231a89d944525f89cdc4f5aa99aede614045e947633810f7e12aa0a09b4c5",
        "bytes": 668
      },
      {
        "name": "RustStructureParser",
        "startLine": 5457,
        "endLine": 5628,
        "sha256": "8e6ee63fd382eac77bf90b8db8b604aa235c369ed7ff77aa81027dad6e3e7368",
        "bytes": 10326
      },
      {
        "name": "inspectRustStructure",
        "startLine": 5667,
        "endLine": 5684,
        "sha256": "4f42e200ab96d4383b18157ffbbfbbd76d9ef01eca7fcf21b2076fdf0df46c08",
        "bytes": 589
      },
      {
        "name": "inspectRustMutationMetadataFacts",
        "startLine": 5834,
        "endLine": 5880,
        "sha256": "69f41b751f7f3fa108f788ecb808729833ca547941d7eb681bc50a368b2668f2",
        "bytes": 4123
      },
      {
        "name": "inspectMutationMetadataSource",
        "startLine": 7777,
        "endLine": 7798,
        "sha256": "970729d89675f6e36329e4d15fc248b7e3f77dc63a6f0195534f1f22c86f29fb",
        "bytes": 4266
      },
      {
        "name": "scanRegistryCompilerImports",
        "startLine": 6889,
        "endLine": 6906,
        "sha256": "ac31fd6b879fe2e2a77648b8d725a79b4e2a2b84be436b85d11e4d8e3d1ce772",
        "bytes": 1694
      },
      {
        "name": "registryStaticImports",
        "startLine": 6909,
        "endLine": 6920,
        "sha256": "cc96ad5500aeba5e0a61e61a057d4fd9304c5474efe201c71503940771f7e87d",
        "bytes": 1133
      },
      {
        "name": "classifyPackageSourceRole",
        "startLine": 6987,
        "endLine": 7035,
        "sha256": "50ac1b1de4202241d4cdb30c5fdbbe49b2b42b5e79ca207418720cf393f36d3c",
        "bytes": 4020
      }
    ]
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "129693956",
      "mode": "33188",
      "size": "901994",
      "modifiedNs": "1787875910101685600",
      "changedNs": "1787875910101685600",
      "sha256": "970b240e43810044e1d497c9319abe5561a8ae02c8db0fa2efac57fb2b4767cb",
      "bytes": 901994
    },
    "declarations": [
      {
        "name": "TypeScriptCollectionToken",
        "startLine": 3658,
        "endLine": 3665,
        "sha256": "a3ec7a50932c9f023c5605486c2dd094c30d8b695eaa5207073ed8fae168aa97",
        "bytes": 258
      },
      {
        "name": "TypeScriptCollectionSyntax",
        "startLine": 3667,
        "endLine": 3670,
        "sha256": "cff0c5269451e2f419311e0ded852e12466093f26117ba4579ecd7575fff327d",
        "bytes": 144
      },
      {
        "name": "typescriptCollectionSyntax",
        "startLine": 3673,
        "endLine": 3731,
        "sha256": "cc870c767089773959dd368ec2ec73c50ed399b70d1e8f188171dece6ca4de12",
        "bytes": 3181
      },
      {
        "name": "typescriptCollectionSegments",
        "startLine": 3734,
        "endLine": 3744,
        "sha256": "f920d151c1c31defce06235e579b98306b6d31174ad6e50fd011688d187f5611",
        "bytes": 533
      },
      {
        "name": "typescriptCollectionEmbeddedExpressions",
        "startLine": 3747,
        "endLine": 3762,
        "sha256": "5fa0c3f89b7fa1bdf9ceda9a75e4d79d25930685f8b5162a66e0b7aff43e8af7",
        "bytes": 801
      },
      {
        "name": "typescriptPathCollectionReferenceAuthority",
        "startLine": 3794,
        "endLine": 3925,
        "sha256": "f4c70db1e8943eadfc6d0ba950387551c6f6b8b7c19a5ebe31d8e1270bd8592e",
        "bytes": 10865
      },
      {
        "name": "typescriptTokens",
        "startLine": 3927,
        "endLine": 3968,
        "sha256": "b5998221c093a3a124f69f26282e6907ab21622637de09b2d3b1e7738fe83e41",
        "bytes": 3844
      },
      {
        "name": "typescriptSyntax",
        "startLine": 2324,
        "endLine": 2326,
        "sha256": "8d5aa134c2c598b97af1bcb584f1e109b00d650a9982c5a2e71b4dd1b0c56679",
        "bytes": 229
      }
    ]
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "107841437",
      "mode": "33188",
      "size": "259410",
      "modifiedNs": "1787862851349758600",
      "changedNs": "1787862851349758600",
      "sha256": "efdcd82a1a8bb2dedeb420c6d274174a11954fa12660e1081a2f401d9e9e3e49",
      "bytes": 259410
    },
    "declarations": [
      {
        "name": "parseTsImportSpecs",
        "startLine": 559,
        "endLine": 567,
        "sha256": "73a2a2f38becccfff7115b2af60afa7462a35dfd19aa97d2eda0c09de0e7cc08",
        "bytes": 260
      }
    ]
  },
  {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "134874988",
      "mode": "33188",
      "size": "16779",
      "modifiedNs": "1787874345040025940",
      "changedNs": "1787874345040025940",
      "sha256": "fc42ddb26cdbc244a8dd99a83d4c21376d74363f34252577eb6d597ab66e70d9",
      "bytes": 16779
    }
  },
  {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🔣️.json",
    "fingerprint": {
      "device": "16777230",
      "inode": "134874175",
      "mode": "33188",
      "size": "8952",
      "modifiedNs": "1787874329017939514",
      "changedNs": "1787874329017939514",
      "sha256": "a2812dffd8773156157a35ed446b5b29440863e74783633914b9944cab479d4d",
      "bytes": 8952
    }
  },
  {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/🧬️schema/🔣️.json",
    "fingerprint": {
      "device": "16777230",
      "inode": "134876195",
      "mode": "33188",
      "size": "3582",
      "modifiedNs": "1787874033298986398",
      "changedNs": "1787874033298986398",
      "sha256": "befc27415672f0ec50b38f24e8ab3d332e153aba25f359d6e36fb742a25716e4",
      "bytes": 3582
    }
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️typescript-path-collection/🟦️.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "133900570",
      "mode": "33188",
      "size": "19295",
      "modifiedNs": "1787851940686564202",
      "changedNs": "1787851940686564202",
      "sha256": "d2b9c39f1597d51207131efb9f7b2a51b0695db1b598aaff47b19d3a7c0205ed",
      "bytes": 19295
    }
  },
  {
    "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️registry-import-language/🟦️.test.ts",
    "fingerprint": {
      "device": "16777230",
      "inode": "133497634",
      "mode": "33188",
      "size": "12598",
      "modifiedNs": "1787849228471548875",
      "changedNs": "1787849228471548875",
      "sha256": "5489c96e23129f033a5ef8b4cbef4b61e29397bed7a4ba880f62d0fae8f1db6d",
      "bytes": 12598
    }
  }
]
```

## Complete Exploratory Oracle Evidence

The closure consists only of the captured `exported`, `span`, `namedMembers`, `typeStructure`, `expressionStructure` and `compilerFacts` declarations. A TypeScript compiler transpiled that closure for the read-only diagnostic; no alternate oracle logic or source parser was authored. These nine observations do not increase the existing fourteen-case gate count.

```json
{
  "kind": "read-only in-memory execution of captured compilerFacts, no subject and no candidate files",
  "controller": {
    "device": "16777230",
    "inode": "134874988",
    "mode": "33188",
    "size": "16779",
    "modifiedNs": "1787874345040025940",
    "changedNs": "1787874345040025940",
    "sha256": "fc42ddb26cdbc244a8dd99a83d4c21376d74363f34252577eb6d597ab66e70d9",
    "bytes": 16779
  },
  "closureSha256": "caf831798f3a0b7f5ee50cedeb7dc80c20c7a6358d9287f8751493e462e25780",
  "typescript": "5.9.3",
  "cases": [
    {
      "id": "mixed-default-named-import",
      "source": "import Default, { Mutation as M } from \"./protocol\"; export type Alias = M;",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "type",
            "name": "Alias",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 53,
              "end": 75
            },
            "structure": {
              "form": "reference",
              "members": [
                "M"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [
          {
            "relation": "import",
            "typeOnly": false,
            "imported": "Mutation",
            "local": "M",
            "moduleSpecifier": "./protocol",
            "modulePath": [],
            "span": {
              "start": 18,
              "end": 31
            }
          }
        ],
        "diagnostics": []
      }
    },
    {
      "id": "object-spread",
      "source": "export const metadata = { ...base, kind: \"add\" };",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "variable",
            "name": "metadata",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 13,
              "end": 48
            },
            "structure": {
              "form": "object",
              "members": [
                "kind"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "interface-heritage",
      "source": "export interface MyMutation extends DomainMutation { local: number }",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "interface",
            "name": "MyMutation",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 68
            },
            "structure": {
              "form": "object",
              "members": [
                "local"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "computed-type-literal",
      "source": "export type Computed = { [key]: number };",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "type",
            "name": "Computed",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 41
            },
            "structure": {
              "form": "object",
              "members": [],
              "unresolved": "computed-property"
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "method-local-interface",
      "source": "export class Container { run() { interface Mutation { value: number } } }",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "class",
            "name": "Container",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 73
            },
            "structure": {
              "form": "class",
              "members": [
                "run"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "bodyless-ambient-module",
      "source": "declare module \"./protocol\";",
      "result": {
        "completeness": "complete",
        "declarations": [],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "nonprimitive-type-alias",
      "source": "export type Scalar = number;",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "type",
            "name": "Scalar",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 28
            },
            "structure": {
              "form": "unresolved",
              "members": [],
              "unresolved": "unsupported-type"
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "chained-object-assertion",
      "source": "export const metadata = ({ kind: \"add\" } as const) satisfies object;",
      "result": {
        "completeness": "incomplete",
        "declarations": [
          {
            "kind": "variable",
            "name": "metadata",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 13,
              "end": 67
            },
            "structure": {
              "form": "unresolved",
              "members": [],
              "unresolved": "initializer:expression"
            }
          }
        ],
        "aliases": [],
        "diagnostics": [
          {
            "code": "unresolved-expression",
            "span": {
              "start": 13,
              "end": 67
            }
          }
        ]
      }
    },
    {
      "id": "ambient-raw-module-name",
      "source": "declare module \"plugin\" { export interface Mutation { value: number } }",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "interface",
            "name": "Mutation",
            "exported": true,
            "modulePath": [
              "\"plugin\""
            ],
            "span": {
              "start": 26,
              "end": 69
            },
            "structure": {
              "form": "object",
              "members": [
                "value"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    }
  ]
}
```

## Nested Union Supplement and Conservative Contract Requirement

Added after root identified the union-child gap. The initial report fingerprint was `369fe547ffd39cd4f219bcea61e09898e9fa06992e55e1c999a427f7d606cab1` (37157 bytes); this supplement retains that identity and the earlier nine observations. The two added inputs used the same unchanged actual oracle closure and TypeScript 5.9.3. Both returned complete with no diagnostics, despite containing nested type forms which are diagnosed at a top-level type alias.

The review now requires recursive unsupported-syntax accounting before production. This is not an instruction to duplicate the compiler or to flatten these expressions into guessed mutation payloads. The smallest honest source domain owns lexical state, statement/type grammar, exact spans and completeness accounting; protocol identity remains outside it. Existing D/N parser regions remain untouched until root coordinates an implementation packet.

```json
{
  "kind": "read-only actual compilerFacts nested union supplement, no subject",
  "priorReport": {
    "device": "16777230",
    "inode": "134918712",
    "mode": "33188",
    "size": "37157",
    "modifiedNs": "1787876873097805024",
    "changedNs": "1787876873097805024",
    "sha256": "369fe547ffd39cd4f219bcea61e09898e9fa06992e55e1c999a427f7d606cab1",
    "bytes": 37157
  },
  "controller": {
    "device": "16777230",
    "inode": "134874988",
    "mode": "33188",
    "size": "16779",
    "modifiedNs": "1787874345040025940",
    "changedNs": "1787874345040025940",
    "sha256": "fc42ddb26cdbc244a8dd99a83d4c21376d74363f34252577eb6d597ab66e70d9",
    "bytes": 16779
  },
  "closureSha256": "caf831798f3a0b7f5ee50cedeb7dc80c20c7a6358d9287f8751493e462e25780",
  "typescript": "5.9.3",
  "cases": [
    {
      "id": "union-nested-conditional",
      "source": "export type Combined<T> = { kind: \"plain\" } | (T extends string ? { kind: \"yes\" } : { kind: \"no\" });",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "type",
            "name": "Combined",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 100
            },
            "structure": {
              "form": "union",
              "members": [
                "{ kind: \"plain\" }",
                "(T extends string ? { kind: \"yes\" } : { kind: \"no\" })"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    },
    {
      "id": "union-nested-mapped",
      "source": "export type Combined<T> = { kind: \"plain\" } | { [K in keyof T]: T[K] };",
      "result": {
        "completeness": "complete",
        "declarations": [
          {
            "kind": "type",
            "name": "Combined",
            "exported": true,
            "modulePath": [],
            "span": {
              "start": 0,
              "end": 71
            },
            "structure": {
              "form": "union",
              "members": [
                "{ kind: \"plain\" }",
                "{ [K in keyof T]: T[K] }"
              ],
              "unresolved": null
            }
          }
        ],
        "aliases": [],
        "diagnostics": []
      }
    }
  ]
}
```

## Limitations

No new owned parser implementation, canonical test execution, complete declaration census or mutation-identity proof is claimed. The scope reviewed is the exact library source/API chain above, not every parser in the monorepo. One report-path preflight was initially called with the file-kind default for its directory and correctly rejected before any write; it was repeated with the explicit directory kind. All prior ticket inputs and evidence remain untouched.
