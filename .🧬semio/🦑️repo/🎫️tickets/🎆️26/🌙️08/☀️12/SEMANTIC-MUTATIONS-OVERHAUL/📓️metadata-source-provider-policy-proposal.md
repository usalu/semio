# Metadata Source-Provider Policy Proposal

## Purpose

This is a bounded design proposal only. It connects accepted FND Metadata Facts 12, FND Wrapped Type Origin 21, and FND Cargo Provider Binding 25 into one future proof for mandatory metadata source authority. It neither activates policy nor introduces a provider allowlist, approval API, or production mutation change.

## Required Inputs

For each candidate wrapped payload, the future policy receives only existing proof outputs:

1. FND21's successful `sourcePath`, `declarationName`, and source-local `modulePath` from the existing reachability/module-context proof.
2. FND12 declaration, attribute, public alias/re-export, conditionality, and manual-`MutationLeaf` implementation facts for that exact physical source and declaration.
3. FND25's selected local normal dependency binding: canonical consumer/workspace/provider manifest locators, package name, explicit library name and source locator, proc-macro flag, dependency key, path authority, and extern name.

No name suffix, semantic folder spelling, conventional Rust source path, Cargo member scan, dependency-graph traversal, or second Rust graph may fill a missing input.

## Proposed Decision Order

1. Require FND21 to return one unconditional, unambiguous wrapped declaration origin. A missing origin, a child origin without separately released source authority, or a conditional/shadowed/private route rejects.
2. Resolve that consumer declaration's manifest/library context through the existing module-context graph. It is never equated with a provider library root. Resolve the derive provider and lower-contract provider independently through FND25; each comparison is identity-based: canonical provider manifest locator, explicit library source locator, package name, library name, and proc-macro state must all agree.
3. Resolve derive/contract syntax only through FND12's exact declaration and alias/re-export facts in that already-proved consumer module context. Accept no spelling-only or decoy attribute.
4. Require the public Rust alias/re-export scope used by the wrapper to be the same unique, unconditional route established by FND21. Grouped, renamed, `self`, absolute, scoped, and child re-exports remain valid only when their existing facts prove the route; unresolved or competing aliases reject.
5. Reject any FND12 manual `MutationLeaf` implementation for the actual declaration or a competing declaration route. A manual implementation is not substitute evidence for the derive/contract proof.
6. Preserve FND25's selected-edge limitations: only the selected normal local dependency may provide identity; optional, target-conditional, build/dev, external, ambiguous, unsafe, or no-follow-invalid bindings reject before metadata inspection.

## Fail-Closed Boundaries

- Conditional `cfg`/`cfg_attr` evidence, including inherited module conditionality and conditional enum variants, is never promoted to active authority.
- A direct canonical declaration is not interchangeable with a child-facet re-export. FND21 deliberately retains the child source, so the future policy rejects it unless a dedicated child-source authority packet adds a complete identity proof.
- The compiler extern name is useful cross-check evidence, but never the canonical provider identity by itself. Package names, dependency keys, library names, and extern names remain distinct.
- A provider may be structurally bindable under FND25 yet not approved for metadata; FND25 proves identity and boundary safety only.

## Proposed Test Packet Before Activation

Schema-first vectors should combine exact FND21 origin triples with FND12 syntax facts and FND25 bindings. They need direct canonical acceptance; renamed/grouped public alias acceptance; same-name package override versus library/extern divergence; workspace-inherited binding; child-reexport rejection; manual-implementation rejection; conditional declaration/alias/module/variant rejection; source/manifest/library mismatch rejection; and unresolved/ambiguous/external binding rejection. Existing Rust compiler and independent TOML oracles remain the language parsers; a policy-specific result must not claim to replace either.

## Explicit Non-Goals

This proposal does not alter reachability, provider binding, Rust parsing, metadata facts, derives, registries, package manifests, Cargo commands, or global policy behavior. Root must freeze a separate packet before any implementation begins.
