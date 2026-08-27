# Shared Flow Selected Copy Checkpoint

## Verified Gates

- Coordinator canonical native r6: 22 passed, zero failed, 847 filtered; all four reader laws passed, including unchanged cancellation/root-binding regression and guarded failure destruction. Reader adoption is approved. This supersedes the historical failed three-reader run, not the negative fixture.
- Coordinator expanded OrderedMap native: 16 passed, zero failed, 192 filtered; includes the two OrderedSet laws, three new shared-entry laws and original eleven map laws. Map/Set adoption is approved.
- Canonical workspace Nx interactivity self-test: 737 passed; 33 exact factory owners, 254 custom rows, 25 generic rows. The newly integrated selected-copy source/strict-Ajv/Node/fast-json-stable-stringify packet contributes 13 checks: five selections, three schema hostiles and five source hostiles.
- Targeted `git diff --check` passed for this packet. No native compiler was run by this executor.

## Selected Typed API

### Allocation And Retirement Review Addendum

The constructor now also requires `FlowCopyAllocationBudget::new(maximum_single_bytes, maximum_total_bytes)`. Text/vector task construction performs no source-sized reservation. A dedicated phase checks both memory admissions and multiplication/addition overflow, then calls `try_reserve_exact` on an empty target. Later string pages and vector pushes use that reserved capacity without whole-buffer reallocation or concatenation. Admission is a local owner budget, not a claim of global memory reservation. Concrete factories still must derive it from their actual owning-domain admission; the mounted 262,144-byte decode limit is not assumed universal.

Five native selected-copy tests are now authored: the original three plus root-retirement overgrant/factory-close and allocation-admission/no-reallocation laws. The test records reservation timing for its actual fixture only. Its 16 MiB single/32 MiB total allowance does not establish maximum-envelope timing, an eight-millisecond bound, or global memory admission. Root owns the combined eleven-test Flow short gate. The original three-test count below is historical.

Root-retirement `Pending` values exceeding one item or the supplied byte grant are rejected without losing the retained owner. The factory handle is now an explicit Option included in terminal-empty and closed in its own final phase. Schema/source tests now include the two new rejection guards. An initial verifier rerun found the hostile failed-latch mutation only changed one of two assignments; the hostile mutation was corrected to replace both assignments, preserving the rejection law.

Framework Flow exports `retained::{FlowWidgetCopy, FlowSynapseCopy, FlowFixtureCopy}`. Each constructor receives an exact immutable `Arc<R>`, index, lifetime-preserving typed projection function and authoritative root-retirement factory. Projection starts only in the first positively granted advance; an absent index latches failure without dropping the root.

`advance(items, bytes)` returns actual copied string bytes and performs one task/frame phase. Strings use disjoint byte slices; vectors append one selected child; ordered maps, ordered sets and neural dictionaries share immutable roots without key comparisons or payload cloning. Domain records preserve scalar fields, exact labels and native field order. No JSON serialization, graph diff, map reinsertion or whole fixture clone is used.

Private rooted projections retain an immutable `Any + Send + Sync` allocation guard. The only private unsafe Send witness requires `T: Sync`; no public pointer or unsafe contract is exposed. Explicit close retires partial copied output and borrowed task frames before invoking the exact source retirement factory. Entire nonterminal state is behind `ManuallyDrop`; invalid drop reports failure without recursive automatic destruction or a second panic during unwinding.

Three native laws are authored, not yet executed: selected serde parity and unchanged map pointer identity across a worker; cancellation/zero grants/invalid projection with one final root destruction; guarded nonterminal drop. Cancellation cases include before first poll and middle of a long Unicode string. The schema/fixture is shared and strict. Node and fast-json-stable-stringify are independent fixture oracles, not execution of the Rust copier; native serde parity remains a separate gate.

## Field Adoption

Framework `FlowUi.nodes`, `FlowFixture.layout` and preview expanded membership now use codebase-owned ordered roots. Shared Flow retirement forwards neural `ValueRetirement` and ordered cursor actual-byte accounting. Cold DSL DTOs remain explicit standard-map/array representations. Changed ordinary fixture swaps explicitly retire detached fixtures; no automatic cold destructor was added. Procedural2d/3d typed codec fields and layout diff signatures have been adapted.

The Flow and Procedural native test routers now await the budgeted runner, propagate exit status, and forward filters. Suggested coordinator gates: `semio-framework-os-flow-core:test` with `flow_selected_copy_`, `flow_retirement_`, and `graph_parameter_`; `@semio-tech/procedural-plugin:test` with `generation_root_`.

## Remaining Work And Boundaries

This is a shared substrate checkpoint, not completion of slider commands. Flow peer owns its command and retained Store recipe; Procedural3d and Procedural2d exact new command registration, candidate/inverse/sealer and final runtime latest-wins integration remain unimplemented in this packet. Shared renderer already dispatches the exact small parameter event for all three consumers, with seven actual DOM tests passing previously; those tests do not prove backend registration.

Peer owns neural/Flow host retirement conversion. Cold whole-fixture import, generation edits, generic collection mutation paths and partial mounted decoder cleanup remain distinct, uncredited work; strict ordered ownership may expose their previously implicit destruction. The selected copier does not certify those paths. Parent owns subsequent compiler and live WASM checks.
