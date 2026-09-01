# TypeScript Off-Facet Declaration Facts: Schema-First Boundary

## Reusable Current Footprint

The only current TypeScript source primitive consumed by root mutation inventory is mutationTaxonomyTsSpecs in root 📜️script.ts at 20850. It removes comments with regular expressions and extracts only static import/export-from specifiers. Its fact shape is { specifier, relation, modulePath: [] }; it does not retain declarations, aliases, source spans, nested namespaces, type structure, union members, or computed-form uncertainty.

Discovery 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts has no exported TypeScript declaration inspector or TypeScript compiler API use. Its exported inspection APIs are Rust-specific. This packet therefore proposes no tokenizer and changes no discovery/root source.

The existing development dependency is typescript@5.9.3 (package.json line 167). It is used only by the ticket oracle, not as a runtime dependency or exported API.

## Proposed Fact Contract

🧪️off-facet-typescript-declaration-census-57/🧬️schema/🔣️.json defines version-one neutral facts:

- declaration kind: type, interface, enum, class, or variable;
- raw compiler span { start, end }, exact module scope, and exported/local visibility;
- structural form object, union, reference, enum, class, or explicit unresolved;
- raw union/member spellings where the compiler exposes them;
- import/type-only-reexport aliases with source module and raw alias span.

The schema intentionally keeps conditional, mapped, and computed property forms unresolved. It never infers a mutation identity from a declaration or alias name.

## Golden Compiler Cases and Source RED

The ticket compiler oracle uses ts.createSourceFile as an independent test-only implementation. It validates strict Ajv closure and nine language-neutral cases:

1. comments, strings, and templates containing mutation-looking text;
2. local same-named Mutation interface;
3. imported type alias;
4. literal discriminated union;
5. exported object metadata;
6. nested namespace interface;
7. type-only reexport;
8. conditional/mapped/computed forms;
9. enum and class declarations.

The current source subject RED is explicit: discovery does not export inspectTypeScriptDeclarationFacts. The controller records sourceSubject.status = missing-export as the intended pre-implementation boundary, not as a passing production inspector.

Minimal future discovery footprint, pending source authorization: one new inspectTypeScriptDeclarationFacts(source) export and its owned fact types in discovery 🟦️component.ts, implemented behind a repository-owned interface. Root inventory integration should be a later separate change that consumes only captured MutationTaxonomySourceIndex.contents; this packet does not authorize it.

## Executed Oracle

~~~sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts'
~~~

Observed result: TypeScript compiler oracle 9/9 passed with source subject missing-export, retained at 🧪️off-facet-typescript-declaration-census-57/🧫️run-rYkCqb/. The controller recorded identical before/after input captures:

- discovery source: 807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423;
- schema: b94db30b047f925188d26c85a09f79c5cd314a1734beb8812b2d052a7848c98a;
- vectors: c8717936ca4500ecb75afc390efaa6b1a6e05812ffe5bc85795c3c820d4d9144;
- controller: 366f1c3ed7b34ada61256f8a0edd661f09f5e013d89911fb2c27b32f06a06850.

This proves the test oracle's golden facts and the absence of the proposed export at that captured source version. It does not prove mutation semantics, resolve aliases, establish module reachability, or census the workspace.

## Completeness and Actual-Subject Boundary

The contract now requires owned completeness plus source-coordinate diagnostics on every case. It does not expose TypeScript SyntaxKind names as a fact vocabulary. The owned diagnostics cover parse error, conditional/mapped/computed uncertainty, JSX and other unresolved expressions, function-local scope, default/namespace import, import-equals, export-star, destructuring, anonymous default class, and unsupported module statements.

The preserved original nine cases now retain diagnostics/completeness. Five additional cases cover default plus namespace import, import-equals, export-star, destructuring, function-local declaration, anonymous default class; regular expression, interpolated template, and ASI-separated declarations; escaped identifier; TSX JSX expression; and malformed TypeScript parse.

The reference command completed 14/14 with TypeScript 5.9.3 and strict Ajv:

~~~sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts' reference
~~~

Evidence is retained in 🧪️off-facet-typescript-declaration-census-57/🧫️run-sQzwPk/. That is reference-oracle evidence only.

The actual-subject command deliberately fails nonzero if the production export is absent or its output differs:

~~~sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-typescript-declaration-census-57/📜️script.ts' subject
~~~

Observed current-source RED: subject missing inspectTypeScriptDeclarationFacts export, retained with before/after captures in 🧫️run-yDXGYq/🔣️failure.json; the command exited nonzero. The new schema/vector/controller hashes are befc27415672f0ec50b38f24e8ab3d332e153aba25f359d6e36fb742a25716e4, a2812dffd8773156157a35ed446b5b29440863e74783633914b9944cab479d4d, and fc42ddb26cdbc244a8dd99a83d4c21376d74363f34252577eb6d597ab66e70d9 respectively. Discovery remains captured at 807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423.
