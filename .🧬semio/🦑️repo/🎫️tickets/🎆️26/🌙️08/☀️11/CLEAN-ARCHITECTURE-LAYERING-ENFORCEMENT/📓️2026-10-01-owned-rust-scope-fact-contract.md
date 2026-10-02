# Owned Rust Scope Fact Contract

Read-only consumer inspection, no tests or source edits. Proposed schema-first contract belongs beside existing Rust module graph facts in Repo discovery; use one tokenized metadata parser and one recursive lexical scope traversal.

## Canonical Fact Shape

Extend inspectRustModuleGraphFacts return with required `scopes: readonly RustModuleScopeFact[]`. Each row represents an actual root or inline module body, with canonical modulePath, kind root/inline, half-open UTF16 body range, and retained metadata proof. Root always has a row, including files without modules. Inline rows retain declarationOffset and their inner attribute ranges; outer declaration metadata stays in existing module fact. Use a closed unresolved union shared with module facts, including unsupported-attribute and ambiguous-path; malformed metadata must have explicit unresolved proof. Scope rewriting proof should retain exact source offsets/attribute names, not just an inherited boolean conditional flag.

Canonical module name/modulePath use rustIdentifierSymbol, while authored keyword decisions continue using token.text. DeclarationOffset and body ranges preserve raw source provenance; optional authored spelling is evidence, never identity. Conventional filename resolution for `mod r#sealed;` must use canonical sealed.rs/sealed/mod.rs, not r#sealed.rs. All graph key consumers must use this same identity.

Expose one pure owned selector such as `rustModuleScopeProof(facts, sourceScope)` that combines applicable root and inline ancestor scope facts. Direct consumers must call this selector rather than duplicate metadata scanning or ignore root metadata. It returns retained proof or typed unresolved evidence. Avoid building a second regex/token parser in execution or normalization.

## Graph Admission

Before creating a root context, inspect the physical source root scope proof. Refuse unresolved root scope authority, while still inventorying authored literal/attribute inputs. Before creating inline child context, require resolved outer module fact plus resolved body scope and every applicable ancestor scope. Before an out-of-line child context, require resolved parent scope/mount, then resolve child physical root scope before admission. Included physical files likewise have their own root facts; their include origin stays explicit and cannot become root/module authority.

Contexts should carry an owned proof reference, such as sourcePath+scope body offset, or a closed retained scope proof chain where external consumers need direct provenance. Do not add a second boolean that silently disagrees with scopes. sourceChain remains physical file provenance; inline ancestry needs scope facts because an inline module does not append a file to that chain. Graph targets must never publish an unresolved scope; contexts should not remain eligible for executable template seals if root/inline authority is unresolved.

## Exact Consumer Changes

- discovery/🟦️.ts:8597–8647: factsBySource cache and root/module/include admission consume the new scopes selector. Existing graph JSON context dedup must preserve origin/proof alternatives.
- source/🏃️execution/🟦️.ts:115–126: scope seal consumes admitted contexts/proof; include every occurrence's canonical reference.modulePath in coherence, not only expansion objects.
- normalization/mutation/evidence/🟦️.ts:183–187: existing !module.unresolved is insufficient for newly retained root/inline unresolved scopes. Require scope proof before physical path fallback.
- normalization/mutation/structural-reachability/🟦️.ts:327–335 and342–350: direct mounts and child type-origin traversal need enclosing scope proof as well as !entry.unresolved.
- authoring/mutation-tree/🟦️.ts:194–200: existing mount reuse ignores unresolved metadata entirely. Reject an unresolved existing mount or enclosing scope rather than treating its path/visibility match as an authoritative reusable declaration. Existing mount census should retain unresolved duplicates so authoring cannot add a parallel mount.
- discovery mutationMetadataRoute walk:12490–12517 uses graph.targets/contexts and needs no independent scanner once graph admission is authoritative; canonicalize incoming Rust path components consistently before graph lookup.
- evidence types:122–124 currently project graph contexts to a narrow structural subset. Either consume graph's canonical admitted proof guarantee or retain explicit proof in this projection; do not permit arbitrary fake graph contexts to gain authority by type narrowing.
- Workspace-contract, native-source-ownership, writable-path-authority and finite-target-consumption API declarations need the new required scopes return contract and hostile facts. Their mocks must not synthesize resolved empty scopes for unknown input.

## Small Closed Cases

Root unknown inner cfg_attr rewrite; inline unknown inner rewrite; private out-of-line leaf unknown inner rewrite; accepted inert root+inline doc/cfg metadata sibling; canonical sealed/r#sealed same-key dual-target ambiguity; canonical raw default filename lookup; physical child deletion; actual fallback normalization rejection despite existing target file. Native default and enabled configuration proofs are distinct from the all-authored graph expected refusal. Root scopes with no modules must still return explicit proof, avoiding empty-facts authority.

The current graph-facts parseScope inner loop at8675–8681 preserves only conditional evidence. Replacing that loss with these shared domain facts closes the reported gap without introducing separate scanners. No passing claim is made.
