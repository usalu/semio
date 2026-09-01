# TypeScript Declaration Facts Contract 61

This ticket-only contract refines the off-facet TypeScript declaration facts packet. It does not change D, N, S, package facades, or a global census.

## Approved Semantics

- Valid empty and comment-only TypeScript produce complete empty facts. Missing-file authority belongs to admission, not this pure source parser.
- A future public subject rejects an invalid language with an owned `TypeError`; it must not select a fallback parser mode.
- Mixed default-plus-named imports retain named aliases and diagnose only the default binding with `unsupported-default-or-namespace-import` at that binding span.
- Object spreads retain explicit object members and diagnose `unresolved-object-spread` at the spread member.
- Interface/class heritage retains direct members only and diagnoses `unresolved-heritage` at each heritage clause.
- Conditional and mapped nodes are diagnosed recursively with the existing `unresolved-conditional-type` and `unresolved-mapped-type` at their exact node spans, including union members. The union remains raw source-order spelling; it is not expanded.
- A computed type-literal member reuses `unresolved-computed-property`, spanning the whole type literal. Primitive property annotations are not separately unsupported merely because member names are summarized.
- Direct unsupported summarized top-level or union type forms retain an unresolved declaration/union member and diagnose `unsupported-type-node` at the type node. A zero-width malformed type keeps only its existing `parse-error`.
- A nonempty class executable body is not traversed and gets `unsupported-class-member-body` over its body. Existing empty class methods remain unchanged.
- A bodyless ambient module gets `unsupported-ambient-module-body` over the whole statement. A scanner recovery suffix uses `unsupported-recovery-suffix` only when a later statement boundary cannot be proven.
- Diagnostics retain source order; compiler parse errors remain appended last, matching the original fourteen cases.

All positions are half-open UTF-16 offsets into the exact newline-joined vector input. Decoded Unicode identifier names and raw physical source spelling remain separate. This fact contract does not resolve TypeScript symbols, providers, mutation identity, inheritance, or executable-local declarations.

## Ticket-Only Reference Boundary

The closed ticket schema now admits the six approved distinct diagnostic codes and 11 desired cases while preserving the original fourteen case objects as historical/current cases. The desired cases cover valid empty/comment-only source; mixed default-plus-named imports; object spread; interface/class heritage and nonempty class body; computed type literal; union conditional/mapped members; primitive type; bodyless ambient module; nested template/regex/division/ASI/comment lexical isolation; and Unicode dotted namespace identity. Existing `malformed-parse` remains the malformed-source case and is unchanged; no recovery-suffix golden is fabricated before an owned scanner demonstrates an uncertain suffix boundary.

The first expanded reference execution intentionally failed at `mixed-default-named-import`: the original compiler oracle retained the named alias but silently accepted the omitted default binding. That RED is retained at 🧪️off-facet-typescript-declaration-census-57/🧫️run-DORvrc/🔣️failure.json. After the approved ticket-only oracle repair, TypeScript 5.9.3 accepted all 25/25 authored compiler facts in reference mode at 🧪️off-facet-typescript-declaration-census-57/🧫️run-3cNCte/🔣️result.json. Subject mode then exited nonzero with the genuine missing-export error at 🧪️off-facet-typescript-declaration-census-57/🧫️run-kAm4OH/🔣️failure.json; no ReferenceError or synthetic expected-success branch was accepted.

Current source-ready hashes: controller `10f245c7b668b9647030fc12e70ab100ec12fa48014dd6a99b72aa8f1c6aa9fb`; vectors `5ed6ebf3c6a14933bf33eba055d9428f04773b33708e528c92fad241d0deb389`; schema `22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774`; current D `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`. The controller now imports `pathToFileURL`, captures source inputs with full lstat ancestry plus `O_NOFOLLOW`/fstat endpoint comparison, and reports reference and subject result counts/status separately. No D/N/S production source changed.

The final ticket-only reference replay additionally asserts a fixed sequence of fourteen historical case SHA-256 values before invoking the compiler oracle; this checks original-vector structural equality independently of returned compiler facts. It passed 25/25 at 🧪️off-facet-typescript-declaration-census-57/🧫️run-2kpwvF/🔣️result.json. The corresponding subject RED is retained at 🧪️off-facet-typescript-declaration-census-57/🧫️run-6uNrfx/🔣️failure.json. Final release hashes supersede the preliminary values above: controller `7330c572fedda066f40bf15e1f3a1b873f7646e326dd716076a7016c03edefcf`; vectors `aaae3431b644204bf1f766044cd3159c30dfd4fd16720a6bbc89571aa00ad322`; schema unchanged `22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774`.

## Additive Nested-Type and Class-Body Boundary

Three further desired cases cover a conditional type nested in generic arguments, a mapped type nested in an interface property annotation, and nonempty constructor/getter/setter/static-block bodies. They preserve outer declaration form and raw union/reference members; nested uncertainty appears only as existing conditional/mapped diagnostics. Constructor and static-block names are excluded from class members, while accessors retain their `value` names. No new diagnostic code or source declaration category was added.

The desired-reference RED is retained at 🧪️off-facet-typescript-declaration-census-57/🧫️run-UiXHLu/🔣️failure.json: the then-current oracle incorrectly completed the generic conditional without a diagnostic. After recursive ticket-only reference repair, TypeScript 5.9.3 accepted 28/28 facts at 🧪️off-facet-typescript-declaration-census-57/🧫️run-qVJPyo/🔣️result.json. Subject mode remains a real missing-export nonzero RED at 🧪️off-facet-typescript-declaration-census-57/🧫️run-wW51Zq/🔣️failure.json. Final hashes: controller `06f7c6ad496081380dd6c743a20eafebdcd6c4f283b733e588452c6b09ea6aca`; vectors `d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48`; schema unchanged `22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774`.
