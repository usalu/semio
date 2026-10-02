# Minimal Consumer Scope Refusal Rows

Read-only current source lookup; no edits/tests/compilers. All paths below are relative to Repo library. The proposed scope API remains absent from current discovery symbol search, so harness declarations are prospective integration requirements.

| Actual consumer | Existing canonical owner | Minimal native-valid refusal |
| --- | --- | --- |
| authoring mutation-tree:194–200 existing mount reuse | fixture 🧫️fixtures/🏗️mutation-scaffolding/🔣️.json; schema 🧬️schema/🏗️mutation-scaffolding/🔣️.json; workspace-contract law7411 | canonical existing public insert_page mount plus root `#![cfg_attr(any(), rewrite)]`; scaffold refuses and exact filesystem snapshot remains unchanged |
| structural-reachability:327–335 root mount | fixture 🧫️fixtures/📡️mutation-reachability/🔣️.json; schema same owner 🛂️schema/🔣️.json; law7590 | public-canonical source with dormant root inner rewriting; expected accepted=false, independent native success |
| structural-reachability:342–350 child type origin | fixture 🧫️fixtures/🧬️mutation-type-origin/🔣️.json; schema same owner 🛂️schema/🔣️.json; law7625 | public-child-reexport source with `#![cfg_attr(any(), rewrite)]` in child physical file; compileAccepted=true, expected=null |
| evidence:183–187 physical fallback | fixture 🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json; output schema 🧬️schema/📋️mutation-inventory/🔣️.json; law7530 | bare module-qualified use, canonical path mount with dormant rewriting metadata, target exists; canonical graph refuses and actual inventory fallback must not assign that target origin |

The inventory consumer fixture input currently uses an inline closed fixtureSchema in workspace-contract:7535, distinct from the output inventory schema. Put new input case structure under real schema ownership, or extend the existing input fixture contract explicitly; do not mistake validating output JSON for validating authored refusal rows. Existing basic inventory law7500 contains inline synthetic sources and no native matrix, so the consumer fixture is the more precise extension point.

Scaffold fixture sources use semantic aggregate metadata and may depend on provider crates; native proof should use the corresponding exact mount/root scope Rust source in the source-direction oracle, or an actual consumer package, not assume a bare standalone rustc can compile the full scaffold aggregate. Existing reachability harness only compiles accepted rows: add independent nativeAccepted metadata to prove native-success/refusal. Type-origin already separates compileAccepted and expected, requiring no ad hoc exception.

## Harness Injection And Type Drift

`🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts:130` injects actual graph functions into extracted normalization execution. Contracts at161–173 declare RustModuleMount/context/graph and ModuleFact without unresolved metadata; graph-facts function declares only modules+uses. Add the real required scopes array, shared unresolved union, and exact selector signature; inject rustModuleScopeProof alongside inspectRustModuleGraphFacts. Do not manufacture an empty resolved scope array in mocks.

`🧪️tests/✍️rust-writable-path-authority/🟦️.ts:38` compiles helper functions extracted from finite-target-consumption and supplies graph dependencies. It also needs the selector import/injection because extraction includes the first harness's implementation. Keep both compiler implementations and no-follow read witnesses.

Current source-direction goldens now correctly contain inner-root local bounds[42,193), inline positive[26,177), inline rewrite[55,206), plain local[13,164), code-emission[13,189), attributed[51,202). Preserve half-open UTF16 semantics and canonical modulePath['inside'] on inline occurrences. Root/module/include mount union remains required. Unknown scope/body proof should be tested independently from target ambiguity; a known+unresolved same-key row needs local-template and child descendant variants so retained contexts cannot grant fallback authority.

All mutations to fixture/schema/provider are Root-owned. No passing/native-success claim follows this source audit.
