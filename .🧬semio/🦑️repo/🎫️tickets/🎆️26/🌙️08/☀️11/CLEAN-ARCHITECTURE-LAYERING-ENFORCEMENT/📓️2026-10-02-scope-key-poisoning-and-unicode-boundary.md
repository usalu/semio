# Scope Key Poisoning And Unicode Boundary

Read-only current checkout inspection; no tests/compiler/Nx run. RustModuleScopeFact/rustModuleScopeProof are not present in the inspected current discovery source or library TS symbol search. Thus the requested shared scope surface remains proposed, not a landed API review. Existing return still exposes modules/uses/includes and current executor consumes contexts plus template scopes.

## Concrete Current Defect

Discovery/🟦️.ts:8630 skips unresolved module declarations before computing key or poisoning authority. Known target publication at8640–8641 compares only known differing targets. The physical executor local-block branch (source/🏃️execution/🟦️.ts:124) validates offsets/context but does not test graph.targets/ambiguousTargets. A prior known mount context may therefore survive even if a later conflict removes target. Descendants may already have been traversed and granted targets/contexts.

Use two-phase per-crate graph admission: collect canonical key obligations and all mount alternatives before authoritative traversal, or retain poisoned keys plus a complete final context/descendant invalidation pass. Unknown declaration must poison the canonical key even if target missing or undecodable. Provenance facts stay observable; successful membership must remain distinguishable from refused facts. Graph context lookup consumers must not regain authority from retained observational contexts. Canonical segment prefix matching, rather than string startsWith, determines descendant invalidation.

## Smallest Closed Matrix

| Case | Native default | Target authority | Finite seal |
| --- | --- | --- | --- |
| one private known file mount | success | known | module and local positive |
| known+unresolved same canonical key, known first | success | poisoned | module and local refusal |
| unresolved+known same key, reversed order | success | poisoned | identical refusal |
| unknown alternative same physical path | success | poisoned | refusal despite leaf equality |
| known+unresolved key with child file/module local template | success | parent+descendant poisoned | child refusal |
| known different targets same key under cfg alternatives | success | ambiguous | refusal |
| ordinary/raw same symbol alternative | success | same poisoned/ambiguous key | refusal |
| resolved inline body with ordinary local function | success | known inline context | local positive; module-level refusal |
| inline body unknown inner rewriting | success | scope unresolved | local refusal |

The unresolved branch uses cfg(feature="rewrite") plus cfg_attr(feature="rewrite",unknown_rewrite), while known branch uses not(feature="rewrite"); native default needs no unknown provider. The enabled branch requires an actual tiny procedural provider oracle if asserting expansion behavior. Each row retains actual manifest, physical files, constant+dynamic two sites, exact source/body offsets and complete typed problem arrays. Distinguish unproven target alternatives from inline body authority: the latter must not be represented merely as a conflicting physical filename.

The shared selector should require one exact root→inline enclosing scope chain for the queried canonical sourceScope, rejecting absent/duplicate/overlapping scope proofs. Inner root, inline and child physical root metadata must propagate to descendants. Unknown/malformed outer mount metadata is a mount obligation, while unknown inner metadata is body/scope authority; both are required and neither replaces the other.

## Consumer Surface

Graph admission at discovery:8610–8647; executor source:115–130; direct normalization evidence:183–187 and structural-reachability:327–350; authoring mutation-tree:194–200; normalization source-chain recheck:4554–4575; binding source facts:180. These require the same domain selector or authoritative admitted graph contract. Extracted test harnesses finite-target-consumption130/173 and writable-path-authority38 require selector injection/type declarations once introduced.

## Unicode Evidence And Uncertainty

Current rustIdentifierSymbol strips only r# (discovery:6467–6469). rustIdentifierPart accepts underscore or Unicode L/N categories at6291–6293 and excludes combining-mark category M. Repository path authority extensively enforces NFC, but path normalization is not evidence of Rust identifier equivalence. No current owned Rust identifier normalization law/provider contract was found in this bounded inspection. Do not silently infer an NFC symbol policy from filesystem rules.

A future closed native oracle should compare precomposed and decomposed identifiers in macro definitions/invocations/module declarations and prove actual rustc identity. Tokenization must first represent the whole valid identifier; normalizing a prematurely split token is insufficient. Whether the repository intends to admit all Rust Unicode identifiers or conservatively refuse unowned forms is unresolved. Record this explicitly and avoid extending scope based only on remembered Rust language semantics. Raw ASCII canonical identity already has genuine native fixture evidence reported by Root.
