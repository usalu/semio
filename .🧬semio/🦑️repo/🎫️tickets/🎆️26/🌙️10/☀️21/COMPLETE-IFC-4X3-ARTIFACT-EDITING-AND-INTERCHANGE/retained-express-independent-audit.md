# Independent Retained EXPRESS Foundation Audit

Read-only source checkpoint on 2026-10-21. No production edits, compiler execution, native test execution or compiler-ingress bypass. This audit inherits the parent ticket's documented absence of a usable general repo command route and uses direct read/search/write fallback only. Existing ticket lifecycle and shared outputs are untouched.

## Findings

No additional confirmed P1/P2 correctness or Rust type/borrow defect was established in the inspected foundation. This is a source review of the receiving/overlay checkpoint, not acceptance of complete retained semantic admission. The explicit refusal after receiving is documented unfinished execution, not a newly discovered hidden defect.

## Source Evidence

Paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any`.

- `🧬️schema/⚖️validation/metadata.rs:57-93` normalizes into a fixed stack key, binary-searches immutable declaration slices, and traverses bounded released inheritance without catalog construction. The existing generator/full projection and third-party declaration results are recorded in retained-express-implementation.md; this audit did not independently rerun them or inspect every generated declaration.
- `🧬️schema/⚖️validation/overlay.rs:70-92,104-250` pins source object/entities pointer and length plus immutable mutation pointer and length. The template-specific insertion and replacement shapes are checked before readiness. Source and insertion coordinates survive the persistent merge sort; duplicate identities and duplicate argument slots are refused. Sorted patch coordinates point into the original mutation payload, rather than an intermediate copied graph.
- `🧬️schema/⚖️validation/overlay.rs:261-283` resolves inserted rows and patched original arguments by their actual original owners. This supports references to final forward insertions. Complex constituent roots bypass only the top-level argument patch lookup, consistently with the template mutation contract's top-level SetEntityArg shape.
- `🧬️schema/⚖️validation/receive.rs:124-150,169-291` preserves explicit component/root coordinates, measures nodes/edges/fields first, then receives into separately retained nonrecursive buffers. Text, binary, enum and typed names remain borrowed source coordinates. Traversal uses a fixed 256-frame frontier; binary scans advance at most 64 bytes. Source lookup during traversal is bounded by that frontier and sorted overlay lookup. It does not recursively clone a population.
- `🧬️schema/⚖️validation/receive.rs:133-144,293-324` quotes checked arena byte capacities before the contiguous allocations; normal transitions check original authority, original cancellation/fuel and the five grant axes through retained.rs before advancing and debit the same context afterwards. No code in the reviewed route constructs another context or invokes the cold evaluator. Exact contiguous allocation remains an allocator frontier requiring a sufficiently large capacity loan.
- `🧬️schema/⚖️validation/receive.rs:330-348` reconstructs a borrowed source value using retained parent/child coordinates and a bounded local path. It does not itself quote an original turn; future execution callers must include this bounded work and stack usage in their own transitions. No current production evaluator caller exists at this checkpoint.
- `🧬️schema/⚖️validation/admission.rs:18-46` advances real overlay and receiving stages, then explicitly refuses publication. `admission.rs:48-75` retires receiver storage before overlay storage. `🧬️schema/🏷️templates/✍️apply/work.rs:64-155,204-206,287-289,294-314` attaches this object to the captured SnapshotSource and final planner mutation owner, closes codecs first and semantic scratch before planner mutation/source retirement.

## Scope And Remaining Evidence

The source identity checks intentionally do not hash mutable contents. Their caller must keep the captured source and final mutation slice immutable across all turns. The inspected real template owner does so; these helpers should not be interpreted as general progressive mutation validation.

Complex chain normalization, derived replacement, domains, expression and statement continuations, contextual bounds, inverse/WHERE/UNIQUE/global execution remain the separately documented continuation work. No receiving-ready result authorizes Emit. Native compilation remains blocked before rustc according to the recorded authentic ingress attempt. Source review therefore cannot establish Rust type checking, real heap-versus-receipt measurements, cancellation execution, scheduler fairness, registered Store publication, base revision enforcement or full differential semantic outcomes. The written native tests are evidence of intended coverage only until that authentic route executes.
