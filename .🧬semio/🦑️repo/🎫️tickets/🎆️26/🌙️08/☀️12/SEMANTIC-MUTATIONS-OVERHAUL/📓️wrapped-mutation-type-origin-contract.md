# Wrapped Mutation Type Origin Contract

## Purpose

The existing root reachability proof establishes public direct mounts and wrapped type correspondence, but discards the actual payload declaration origin after resolving public aliases. Metadata policy cannot inspect an arbitrary similarly named struct; it must inspect the declaration actually wrapped by the aggregate.

FND-WRAPPED-TYPE-ORIGIN-21 extends that one existing proof, not a competing source graph. Return the physical repository-relative Rust source locator, declaration name, and source-local module path for every successfully wrapped leaf. Preserve mount/wrapper validity, semantic folder/variant identity, and precise failure reasons. An ambiguous or unresolved declaration has no origin and cannot be approved.

Direct public struct/enum declarations and visible public import aliases resolve to their actual declaration. An allowed public child-facet reexport retains the child source/name rather than pretending the direct primary declared it. This does not widen metadata source authority: the current metadata derive still authorizes only direct canonical declarations, so later metadata policy must reject a child origin until a separate complete authority proof exists.

Reject private/restricted, duplicate, conditional, shadowed, escaped, and symlinked paths before using them as proof. Do not infer declaration origin from semantic names or a type-name suffix. Do not substitute a payload type alias for an actual declaration without proving its target. Preserve the existing exact public canonical mount requirement. Consume the already accepted Rust syntax/module/metadata facts where sufficient; keep all returned types repository-owned.

## Bounded Scope

The root `📜️script.ts` existing reachability function and its callsite, a dedicated neutral fixture/schema, and narrow repository-library tests are the write set. Expose a deliberately named repository-owned inspector only if necessary for reuse and testing; do not retain a duplicate private implementation. Discovery changes are permitted only for a specifically proven missing fact, with a separate test. No changes to production mutations, Cargo provider projections, mandatory traits/derive/registry, or broad structural policy activation are part of this packet.

The executor demonstrated that existing module/use graph facts omit general conditionality. The coordinator approved a minimal extension in the existing discovery parser/interfaces: preserve direct, ancestor, and inner `cfg`/`cfg_attr` conditionality for module/use facts, reusing accepted attribute parsing. Do not add a second local Rust parser. This extension needs exact regressions and must preserve existing module-graph consumers.

Further source review showed that the existing enum and variant structural facts also omit conditionality. The coordinator approved optional true-only conditional facts on those existing types, using the same accepted attribute parser. A disabled/conditional aggregate or variant, root inner conditionality, and conditional inline ancestry cannot establish an unconditional live wrapper. Retain competing declarations before rejecting ambiguity; filtering a conditional competitor away is not proof. Existing unconditional fact shapes must remain unchanged.

Raw root, leaf, filename and child-source locators must be rejected before filesystem access when non-NFC, empty, absolute, dot/parent-containing, backslash/colon/control-containing, or containing a case-folded excluded `compose` component. U+2028 and U+2029 are rejected as line separators. Do not normalize invalid input into validity. Keep per-component no-follow validation for otherwise valid locators. Exclusion tests use virtual filesystem/trapped access, never actual forbidden-path materialization or traversal.

Schema-first cases must assert exact origin values for direct, semantic alias, renamed public alias, and allowed child reexport routes. Negative cases must prove no origin for unbound or ambiguous wrappers and conditional/private/shadowed mounts. Compile accepted sources with rustc as an independent language oracle. Keep those materializations and logs inside the ticket and supply the ticket artifact directory to registered tests. Existing reachability fixtures must continue passing unchanged or receive an explicit reviewed contract correction, never a blanket expected-result rewrite.
