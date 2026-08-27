# Architect Child Owner Closure

## Scope

Removed the Architect program artifact's two process-global payload maps for benchmark and knowledge registers.

## Exact changes

- `ProgramBenchmarksWorkingTable` belongs to the exact `ProgramBenchmarksChild` minted for its records.
- `ProgramKnowledgeWorkingTable` belongs to the exact `ProgramKnowledgeChild` minted for its records.
- Both register accessors resolve only the addressed child and fail soft for a wire-only handle awaiting host materialization.
- Equal serialized identities cannot observe another document's rows, and the typed data retires with its child.
- A language-neutral fixture and serde_json oracle cover both owners, exact wire identities, and payload absence after reconstruction.

## Validation

- Bun fixture parse and exact booleans: green.
- Removed-symbol/source scan for both statics, `thread_local!`, `RefCell`, and `HashMap`: green.
- `git diff --check -- '✏️s/🔌️plugins/🏛️architect'`: green.
- Rust compiler/rustfmt validation remains queued behind Flow's exclusive compiler lease; no compiler-green claim is made here.
