# Compute Occurrence Scope Fixture Plan

Read-only current source, no execution. Ownership leaf still had source-wide canonicalImports during inspection; occurrence helper was not yet observed. Cases below are candidate Rust-native-valid shapes, not compiler receipts.

## Minimal scope rules

Use canonical module segments, exact enclosing lexical block chain and namespace. A module use is not automatically inherited by a nested module. Ordinary child blocks inherit lexical item visibility unless shadowed. A block use item is visible throughout the block, including earlier textual references; do not use declaration offset ordering as item visibility. Nested modules reset parent local lexical authority. Function type parameters and local types can shadow imported type names. Explicit canonical extern paths still require the relevant provider in every admitted manifest context. No sibling import can grant another sibling occurrence.

## Closed source rows

- Positive block item hoisting: `fn f() { let _: Option<EngineKey> = None; use semio_framework_2d::compute::EngineKey; }`.
- Positive inherited ordinary block: `fn f() { use semio_framework_2d::compute::EngineKey; { let _: Option<EngineKey> = None; } }`.
- Native-valid sibling refusal with independent other provider: `mod good { use semio_framework_2d::compute::EngineKey; } mod bad { use other::*; fn take(_: EngineKey) {} }`, other owns unrelated EngineKey. This directly distinguishes global import leakage.
- Native-valid local shadowing: `use semio_framework_2d::compute::EngineKey; fn f<EngineKey>(_: EngineKey) {}`. Decide contract classification: unrelated generic name versus reserved family spelling. It cannot be claimed a canonical EngineKey occurrence.
- Alias refusal: `use semio_framework_2d::compute::EngineKey as Key; fn f(_: Key) {}`; compiled successfully but authored direct-binding policy intentionally refuses.
- Reexport refusal: `pub use semio_framework_2d::compute::EngineKey;`; actual visibility fact should replace source-prefix regex, especially attributes and restricted pub forms.
- Unrelated Engine positive: `trait Engine { fn run(&self); } fn unrelated() { const ENGINE_ID: &str = "x"; }`. Coexistence of ENGINE_ID does not establish owned compute Engine trait identity. If forbidden, define reserved-name policy explicitly.
- Decoy positive: `const DOC: &str = r#"semio_framework_2d::compute::EngineKey"#;` plus doc comment mentioning family and function lifetime `'Engine`; literals/docs/lifetimes must not create family bindings.

## Mount/provider closure

Use same captured source bytes under two real manifests, one canonical direct normal2D provider and one other/no2D provider. Both contexts must be evaluated; missing second provider must produce explicit unproven-family-provider. Missing manifest bytes, source bytes, root target or denied/unresolved scope must produce explicit unproven outcome, never substitute empty source or nearest manifest. Optional/target-conditional provider needs explicit all-configuration authority; a provider row existing syntactically does not prove enabled dependency for all contexts.

Binding facts retain modulePath/blockScope/characterSpan and declarations. Build occurrence selector from those owned facts, not duplicate ad-hoc brace parsing. Refuse a competing visible unknown glob in that occurrence's scope; do not poison unrelated siblings. Preserve captured map and graph evidence and original budgets; no second tree traversal. High received these candidate rules directly.
