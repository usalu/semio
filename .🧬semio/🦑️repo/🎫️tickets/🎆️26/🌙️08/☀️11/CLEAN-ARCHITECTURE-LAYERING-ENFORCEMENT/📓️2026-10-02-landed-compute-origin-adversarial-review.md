# Compute Origin Adversarial Review

Read-only current landed ownership leaf `library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts`; no jobs. Contract source exists with six symbols plus Engine trait. Targeted executor search did not yet show inspectRustFamilyOwnership integration at inspection; do not infer whole route coverage.

Actual strengths: inputs are captured sources+admitted graph+inventory, no second tree enumeration. Canonical declaration requires owner manifest/root/module target and exactly one top-level type definition. Each retained context is checked for scope proof/strict manifest, and direct normal Cargo provider must match exact extern/package/manifest/root. Missing input becomes unproven declaration/provider rather than a successful physical reread. Foreign mounts of canonical bytes reject. Aliases/reexports/opaque family macro syntax have explicit refusal outcomes.

## Actual soundness gap

canonicalImports is a source-wide Set. It discards import modulePath/blockScope and is reused for every bare token and qualified namespace. Native-valid hostile shape with two real providers:

```rust
mod good { use semio_framework_2d::compute::EngineKey; }
mod bad { use other::*; fn take(_: EngineKey) {} }
```

`other` defines its own EngineKey. The other glob is not selected by family imports filter. The bad bare type borrows good's source-wide canonicalImports; current consumer has a legitimate direct2D provider and resolved scope, so can be admitted even though its actual type origin is other. Check each occurrence against its exact enclosing module and lexical block import alternatives; glob/unknown competing names must refuse. Do not convert a source-level set into semantic authority. This counterexample was sent High directly.

Names-only declaration detection also rejects unrelated local EngineKey/EngineRep definitions regardless of canonical origin. Decide explicitly whether contract reserves these identifiers repository-wide; if not, use actual provider/import origin to distinguish unrelated names. Engine trait heuristic requiring ENGINE_ID anywhere in source still admits unrelated local Engine trait when another source item contains ENGINE_ID. Close unrelated-name rows rather than claiming semantic identity from token coexistence.

Providers cache currently keyed only manifest and always compileKind=test. Review normal library/build contexts and target-specific dependencies: strict normal provider selection is useful, but actual compile target/config authority must not be invented from uniform test mode. Multiple mounts require each context proof, with provider cache key including every relevant authority input if resolution differs.

## Required closed rows

Sibling canonical import versus unrelated external glob; nested block canonical import versus outer bare family; unrelated local EngineKey and Engine trait; explicit alias/reexport refusal; raw canonical symbols; same source two manifest mounts one missing provider; orphan/missing source; invalid/unresolved scope; foreign mount of canonical declaration. Native success and policy refusal are distinct expected outputs. Contract schema alone is not behavioral corpus or independent oracle; retain full source fixtures and actual Cargo/rustc provider proof before complete claim. No native outcome has been run here.
