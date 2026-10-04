# Compiler Syntax Extraction Audit

Read-only bounded source inspection; no native/Cargo/tests run. Discovery snapshot was preserved before dependency analysis. Other exact inputs are captured below. This is a helper/caller census in the inspected owners, not whole-repository coverage.

## Smallest acyclic boundary

Current `inspectRustCompileReferences` is still authored in Repo `📚️library/🔍️discovery/🟦️.ts` at6509. Its body ends before `rustTokenText`6806; `rustTokenSegments`6815 is a helper, not part of that body. Move its pure syntax closure together into a neutral Compiler Rust syntax owner. Direct helper names are `rustTokens`6370, `rustTokenPairs`6480, `rustIdentifierSymbol`6500, `rustStringValue`6329, `rustTokenSegments`6815, `rustTokenText`6806, `rustAttributes`7848, `rustVisibility`7861, `rustPathAttributes`7870, `rustFindTopLevel`8057, `rustMetadataAttributes`8395. Metadata flattening additionally needs `rustMetadataAttributeHead`8376; move token/attribute/visibility/metadata fact types alongside them. `rustMetadataPath`8360 is needed for generic derive/attribute observations and adjacent metadata consumers; do not move the OS-specific `rustMutationLeafEvidence` with this pure boundary. Verify helper types/imports against the full snapshot rather than copying only exported functions.

The scanner depends on `posix.isAbsolute` for portable inline mount refusal; keep it behind an owned portable path predicate or use the existing system implementation internally. Its public result uses only owned primitive/readonly collection structures, and does not require Node path objects. Preserve all local macro matcher/expression logic and fail-closed conditions unchanged. The scanner is synchronous and accepts only source; adding expensive producer observations requires explicit owned cancellation/budgets, not an arbitrary resolver callback that can exempt unsupported input expressions.

Neutral dependency direction: lexical tokens/strings/pairs/segments → generic metadata/item observations and compile-reference syntax → Repo Cargo/module/physical inventory integration. Producer slot observation belongs above generic syntax but below Repo policy; schema closure remains an independent Schema service. Compiler syntax must not import Repo, OS, Schema registry or sourceTargets. Keep Cargo manifest projection, inventory, filesystem ownership, configuration participation and physical target joins in Repo until a separate domain-neutral capture contract is established. Rewire all current callsites directly to the canonical neutral owner; do not leave Repo forwarding aliases.

Repo source wrapper imports scanner/reference/context from discovery and delegates `rustSourceReferences` at28. Its `rustSourceTargets` and direction edges retain physical/context policy. Binding imports tokens/pairs/identifier plus Cargo/module projection from discovery: split syntax imports from remaining policy imports. Ownership likewise imports tokens/pairs/identifier plus graph/Cargo proof. Central execution obtains compile references before module graph construction (137–144) and targets afterward (186); deferred registered templates need sealed syntax observations carried through this two-phase flow, not an early catch-and-ignore exception.

## Existing owners and authority interfaces

Existing neutral Compiler `📖️syntax/🦀️.rs` is currently a math expression parser using neutral DSL lexing and Diagnostic spans/limits. It is not a Rust token parser or proc-macro producer/resource registry. Add a language-specific Rust subtree rather than importing its math semantics or replacing the math owner. In the inspected Compiler files there is no existing proc-macro/resource registration interface.

Current neutral Schema closure `📄️source/🔗️closure/🟦️.ts` already supplies finite source records, policy, controls, source positions, reference edges, resource identities and closure output. Reuse it for URI/resource resolution; do not invent a second schema index in Compiler. Its source paths are supplied strings and do not prove filesystem reads. Existing neutral Process `📥️capture/🟦️.ts` captures owned process output with cancellation/deadline/output limit; it does not capture/seal physical source inventories and must not be presented as that authority. Repo existing Cargo/module binding and captured source inventory remain the authoritative integration surface for source/digest/path/provider joins.

## Original laws and proof

Move the existing full syntax law corpus with its original inputs/expectations intact (or keep the original registered test suite importing the new owner). Keep complete Repo rust-source-direction, scopes/graph, attribute/participation/roots and native `.d` cohorts executable. New generic token/quote-template tests must use independent Rust compiler expansion/dep-info oracles where emission is claimed. Exact source quote spans identify a possible producer slot; only actual compiler expansion and escaped `.d` execution establish emitted dependencies/configuration participation. Pure lexical tests and registered source facts do not discharge native consumer coverage. No such execution was performed here.

## Full Captured Inputs

| Source | Bytes | SHA256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | 1019997 | `e43ac92680773bec45f61d98e5fb91ef4aed87838624c83569d8254e25e0fdcb` |
| `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️.rs` | 20430 | `0a351e727fc043a25efc11a98f350eb3731c56a2f5b16ae140b1e30d4fe68cd9` |
| `🧰️framework/🔨️modules/🏃️process/📥️capture/🟦️.ts` | 2967 | `45ed517acdca68e5ff7b98647b27af470337551e84b7ceca70dbaf6b4e11d3dd` |
| `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🟦️.ts` | 26256 | `b11edbd06f37eaf61a83450e6bdb306f61a7dbf79f96849b1311ff709d70f8d8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts` | 6825 | `7379f90447deb365d4e792e680dd48f647b261942cb1db40eff1fc065f9014af` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts` | 31349 | `0871a65f2c6dee4c630ec0574b17642348347305a25d6af5b35a6a679076fdb3` |
