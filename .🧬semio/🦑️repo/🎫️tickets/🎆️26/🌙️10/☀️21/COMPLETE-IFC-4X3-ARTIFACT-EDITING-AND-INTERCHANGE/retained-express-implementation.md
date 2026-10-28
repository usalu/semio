# Retained EXPRESS Foundation Checkpoint

This is an ownership and source-receiving checkpoint, not completed native semantic admission. Native template publication still reaches `template.semantic-admission-unavailable`. The parent requested this checkpoint before independent ownership review and continuation execution.

## Implemented Source

`⚖️validation/metadata.rs` and `metadata-data.rs` provide immutable borrowed metadata for all 876 released entities and 436 types. Entity metadata includes direct and inherited positions, complete type-expression trees, derived declarations, inverse/WHERE/UNIQUE source and supertype expressions. Defined types include aliases, enums, selects and their rule sources. The retained input generator `retained-metadata.py` rejects unrepresented source fields; its generated asset is 3,074,096 UTF-8 bytes and records input SHA256 `5147f42334cbf6fc9727fbf357f1b7aae36ebacc59c6aca657d4c1ee84d638df`. Lookup uses a bounded 128-byte stack key and binary slice lookup. Actual released maximum inheritance depth is nine; the static traversal ceiling is 32.

Template carrier attributes, applicability, subtype and unit-domain resolution now use those static slices. The request no longer probes a cold lazy catalog and refuses zero-touch initialization. Test-only warmup calls were removed. Existing cold catalog APIs remain available to existing cold callers; retained template planning no longer invokes them.

`overlay.rs` retains only original/mutation coordinates. It allocates admitted row/scratch/patch buffers, receives one row per turn, merge-sorts one cell per turn, checks duplicate identities, and orders final argument patches one comparison/swap per turn. It supports the existing template planner's final distinct insertions and original argument replacements, explicitly rejecting other mutation shapes, shifted insertions, missing fields, repeated slots and insert-then-edit. This restriction describes the template batch contract, not a general mutation overlay. References can resolve forward insertions. Reads return the actual immutable source or mutation payload, without creating a second graph.

`receive.rs` traverses original values with a fixed depth frontier. A first controlled pass measures exact node/edge/field capacities; a second fills a nonrecursive arena one node or child edge per transition. Source text, binary text, enum names and typed names remain borrowed coordinates. Binary content scans at most 64 bytes per transition; nonfinite reals, dangling references, unknown typed names, malformed identifiers/header and invalid positional counts are refused. The arena retains scalar values and linked child handles, not cloned source strings/aggregates or a publication DTO. Complex constituent positions remain explicit. Full chain normalization, derived replacement and domain/EXPRESS validation belong to the unfinished execution stage; receiving readiness never means semantic validity.

`retained.rs` checks original cancellation, fuel/yield and all five grant axes before a transition, then debits the same original context. Exact contiguous arena/index births can legitimately require a larger physical capacity loan. These are ordinary allocator frontiers, not a claim that allocating a large contiguous buffer is a physical per-byte cursor. Source/mutation pointer-and-length identity is pinned; the caller must keep the original immutable owners unchanged. The real template work does so through `publication.view.source` and the final planner mutation vector.

`admission.rs` is wired into actual `IfcTemplateWork::Semantic`. It performs the real overlay and receiving stages against the captured `SnapshotSource` authority. It then refuses because retained expression/statement/population execution does not exist yet. Codec retirement still occurs first; semantic arena and overlay retire before planner mutations and source ownership. There is no cold evaluator call, synthetic context, alternate grant, unchecked Emit, candidate graph clone or new runtime dependency.

## Schema And Tests

The closed `retained.json`, neutral `retained-cases.json` and `retained.feature` were written before implementation. Seven native tests are declared across `retained-tests.rs`, `overlay-tests.rs`, `receive-tests.rs` and `receiving-goldens.rs`. They cover complete static positional/type metadata, zero-heap lookup, exact borrowed replacement/forward reference identity, source-owner mismatch, cancellation at every completed overlay/receiving prefix, receiving/close measured heap versus original receipts, denied item/copy/capacity/depth loans and release-underfunded retirement, controlled close, and all 15 existing independent authoring goldens through the real planner/overlay/receiver. Existing addressed native template work tests continue requiring refusal and no publication. These native tests have not executed.

Executed evidence:

- `retained-metadata.py --check`: full generated asset matches current production catalog input.
- `retained-oracle.py`: IfcOpenShell 0.8.3 independently matched every entity supertype/abstract flag and all 6,004 inherited attribute names/optionality/derived positions; enum/select members also match. JSON Schema Draft 2020-12 independently admits the neutral contract.
- Existing selected-template/capture/native-golden/shape suites: **66 pass, 5,702 assertions**. This executes the TS pipeline and shared authoring fixtures, not native Rust.
- Rustfmt parsed/formatted the nine new handwritten foundation/test modules; it also parsed the generated metadata and edited template integration. Native source parsing is not type checking or receipt execution.
- Authentic `cargo test --manifest-path <4x3 package>/Cargo.toml --lib retained` refuses during the compiler version probe, before rustc: **Original compiler ingress requires the repository launcher and authentic enforcing store**. No ingress guard or build infrastructure was changed or bypassed.

The independent oracle interface/dependencies and generated results reside under this ticket's shared `🗑️generated`; retained input scripts remain at ticket root. New source/input paths remain within 256 characters. Repo CLI limitations and direct-tool fallbacks are recorded in retained-express-plan.md.

## Required Continuation

The retained expression/statement machine and population execution are **not implemented** at this checkpoint. The static canonical program remains unchanged: 777 WHERE, 60 derived, 48 functions, two global rules, 165 inverse, six contextual bounds and four UNIQUE declarations. The future machine must execute all constructs and semantic lanes over the same owned arena; no selected-form subset may authorize publication. Contextual complex normalization, full domains, caches, exact semantics and diagnostic results still require that work.

`IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION`, `NATIVE_TEMPLATE_SEMANTIC_ADMISSION`, native authoring implementation and all native execution flags remain false. Only retained receiving implementation is marked present. The pre-existing cold evaluator's implementation flag does not describe original-owned retained execution.

Authentic native compilation, actual heap/receipt/cancellation execution, scheduler fairness, full registered Store/revision/history publication and complete rule differential execution remain required. This checkpoint requests independent Low ownership/overlay review before continuing the full evaluator. Foundation receiving alone does not satisfy the semantic-admission goal.
