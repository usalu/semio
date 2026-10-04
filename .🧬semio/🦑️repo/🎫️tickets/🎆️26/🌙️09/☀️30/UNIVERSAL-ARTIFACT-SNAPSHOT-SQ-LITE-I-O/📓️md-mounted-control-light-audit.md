# CommonMark Mounted Control Light Audit

Current physical production readback following Root's authentic MD22 refusal. No Cargo/build/runtime execution by this auditor. This report supersedes the prior unmounted-draft status for the inspected modules; Root owns the upcoming Native receipt.

## Actionable Findings

1. Snapshot SQLite owner `🪶️sqlite/🦀️.rs:165` reconstructs a raw `MdSnapshot`, then performs a fallible final `(ReconstructSnapshot,1,1)` checkpoint. Cancellation there bypasses `OwnedSnapshot` and ordinarily drops potentially deep blocks/inlines. Wrap the returned candidate in the existing `OwnedSnapshot` before that checkpoint, and take its value only on success. Add an exact final-checkpoint cancellation law with a deep specimen and bounded stack. Earlier reconstruction guards do not cover this terminal window.
2. `📦️pack/🦀️.rs` ordinary `ArtifactDsl::parse_dsl` calls `Snapshot::try_into`, which uses ordinary block/inline reconstruction. The ordinary block leaf currently lacks the unrelated-variant-field validation implemented in `reconstruct_controlled`. The mounted handwritten invalid text law (`sqlite` tests line 108) expects `paragraph raw="unowned text"` to refuse, but ordinary reconstruction currently discards that raw field. Share checked conversion semantics or explicitly add the same closed variant checks to ordinary reconstruction. Native controlled refusal does not establish this ordinary terminal's behavior.

## Mounted Paths And Types

Snapshot owner mounts `#[path="📦️pack/🦀️.rs"] mod owned_pack`; that module mounts `🧱️block/🦀️.rs` and `🧩️inline/🦀️.rs`. SQLite owner mounts actual decode/encode/retire hooks to this module. The authored grammar is `📸️snapshot/📝️text/📖️.grammar.semio`; the test's `../../📝️text/📖️.grammar.semio` resolves correctly from `snapshot/🧪️tests/🪶️sqlite/`.

Logical controlled routines return `ValueError`. Their constructor closures convert through `TextError::from_value_error`, as required by the actual shared native physical helpers' `Result<T,TextError>` and `Result<RecordValue,TextError>` callback signatures. The physical public terminals return String. No inspected logical constructor returned String directly or substituted uncontrolled projection/metadata for controlled native work.

## Empty Item Stage And Guards

Mounted block reconstruction now scopes the exact `items.len()` workload and steps after every item insertion. `paid_children` scopes child work and restores the parent, including zero-child stages. An empty item therefore advances the enclosing known item frontier. The mounted decode law uses `MD_RECONSTRUCT_ACTIVE` to distinguish actual reconstruction from parser/binder callbacks; this directly addresses the earlier audit gap.

`OwnedSlots`, `OwnedNodes` and `OwnedItems` retain partially consumed descendants. Transferred child nodes are stored before a fallible step; scalar prerequisites precede descendant transfer. SQLite Reader reconstruction now wraps node/item accumulators and `take_tail` results, including Link children while URL/title copies can refuse. No leaked typed partial ownership was found within those guarded stages; the final raw candidate checkpoint above remains the exception.

## Allocation-Free Linear Retirement

The mounted retirement is a new destructive work-vector algorithm, not the earlier ancestor-revisit fallback. It pops a child, reuses the freed vector slot for a synthetic continuation node, places that continuation at the bottom using swap, and moves the old frontier into it. List retirement similarly reuses a popped item slot and a popped child slot. Each push is backed by a just-freed slot in that same vector, so no reserve/allocation is needed. Fields discarded by variant matching are ordinary String/scalar payloads; recursive descendants are moved into the iterative frontier. No tree-recursive call chain, forgotten field, or ancestor scan appears.

Each original node/item is consumed; each descent creates at most a bounded number of continuation nodes, processed after its preceding children. Continuations consume existing frontier work rather than rebuilding ancestors. This supports linear node/item work and bounded stack by inspection, with no runtime claim.

The new mounted allocator law refuses all allocation APIs during trait retirement, asserts zero allocation attempts, zero tracked live bytes, and a linear visit ceiling on a 64 KiB thread stack. Cases include 8,192 inline Link levels with URL/title strings, 8,192 mixed quote/list levels with literal siblings, and 4,096 mixed empty/nonempty list items owning Image/Code/HtmlInline payloads. Fixture construction precedes refusal. This is materially stronger than the former direct-fallback tests, and checks actual `retire_sqlite_snapshot` dispatch. Execution remains Root-owned; this audit has not run it.

The allocator's tracking is thread-local and enabled only inside the fresh worker during fixture construction/retirement. It observes the whole worker's allocations while tracking, rather than pointer-tagging fixture allocations. No inspected fixture transfers pre-tracked pointers from another thread. General use beyond this bounded test would require more precise pointer ownership accounting.

## Corrected Ordinary Mixed-Field Readback

Root independently read the actual ordinary block reconstructor in snapshot/📦️pack/🧱️block/🦀️.rs:23. Its final leftover-field predicate includes raw.is_some(), so paragraph raw is rejected rather than silently discarded. The earlier acceptance finding is withdrawn. Controlled reconstruction checks fields before construction; the ordinary path checks leftovers after constructing a candidate, which may warrant a separate lifecycle witness. No failure of that ordinary predicate has been executed. Final SQL raw-candidate cancellation remains a concrete independent finding and now has Native24 staging.
