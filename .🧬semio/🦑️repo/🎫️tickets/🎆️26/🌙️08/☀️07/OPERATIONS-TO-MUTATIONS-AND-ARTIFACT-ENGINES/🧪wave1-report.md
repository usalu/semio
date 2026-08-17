# Wave 1 Report — Operations → Mutations (Kernel)

## Gate

`cargo check -p semio-framework-os-kernel` — **PASS** (see `🧪wave1-cargo-check.txt`).

Also verified `cargo check -p semio-framework-os-kernel --lib`.

## What changed

Core protocol rename (no aliases / no deprecations):

| Old | New |
|-----|-----|
| `Operation<P>` / `OperationDiff<P>` | `Mutation<P>` / `MutationDiff<P>` |
| `backwards` / `operation_id` | `inverse` / `mutation_id` |
| `OperationId` / `OperationMeta` / `Edit.operation_meta` | `MutationId` / `MutationMeta` / `Edit.mutation_meta` |
| Descriptor/Upcaster/Event/Envelope/Transform/Inverse* | `Mutation*` / `InverseMutation` |
| `OpDag` / `OpDagError` | `MutationDag` / `MutationDagError` |
| Collection helpers | `CollectionMutation`, `apply_collection_mutation`, `inverse_collection_mutation`, … |
| DocumentCommand fields / `replay_operations` | `mutations` / `replay_mutations` |
| DocumentApp / Emit / NoConfig* | `Mutation` / `document_mutations` / `Emit::mutations` / `No*Mutation` |
| Domain `*Operation` enums (playbook/flow/space/run/workflow/…) | `*Mutation` (+ `*MutationDsl` mirrors) |
| `neural_engine::Operation` trait | `Operator` (struct catalogue entry → `OperatorRecord` to avoid clash) |
| `ActionKind::Operation` (manifest root) | `ActionKind::Mutation` |

Added `ArtifactEngine` trait in `⚙️engine/🦀️component.rs` (byte-cache `Engine` kept).

Store still calls `Mutation::diff` / `Mutation::inverse` directly in `replay_mutations` — TODO left in place to route through `ArtifactEngine` in a later pass.

Bin entrypoints (`spr`/`pack`/`semio`) fixed to call `semio_framework_os_kernel::…` so the package gate typechecks (pre-existing broken `crate::os_*` / wrong crate name).

## Kept (op brand)

`OpText`, `OpBinary`, `print_op`/`parse_op`/`encode_op`/`decode_op`, `DslOps`, `LanguageRole::Ops`, `🔧️op` facet concept, `OpPayload`.

## Remaining known gaps

1. **Plugin crates under `✏️s/🔌️plugins/`** — not touched (Waves 3/4). Will not compile against renamed kernel types until migrated.
2. **TS renderer / framework-core TS** — not touched (Wave 5). Strings like `pendingOperations`, `KernelOperation`, live-mirror helpers still use Operation naming.
3. **DocumentStore ↔ ArtifactEngine wiring** — trait added; store apply/inverse path not yet driven through an owned `ArtifactEngine` (TODO in store).
4. **Taxonomy / policy / root `📜️script.ts`** — Wave 2.
5. **Grammar productions** (`start operation` → `start mutation`) — Wave 5 / grammar sweep; Rust `*MutationDsl` renamed but `.op.semio` grammars unchanged.
6. **UI copy / filter wire values** in plugin history (`withoutMutations` / German “Operationen”) may still say “Operations” in human labels — intentional until i18n pass.
7. **`content_addressed_entity_id("mutation", …)`** changes minted id namespace vs old `"operation"` prefix — acceptable (greenfield, no legacy).
8. **Neural `OperatorRecord`** rename may need call-site updates in any out-of-tree consumers of the old `Operator` struct name (kernel does not wire neural).
9. **Manifest `ActionKind`** variant + call sites renamed to `Mutation` (root enum lives under `framework/🔨️modules/🛂️manifest`, outside the OS path list but required for consistency). Other out-of-tree consumers may still need updates.
10. Occasional comment prose still says “operation” in the colloquial sense (op-log lines, OpText) — kept where it means the op grammar, not the Mutation trait.
11. **Repo MCP unavailable** in this session — ticket left open; close via `ticket_close` when MCP is available.

## Files touched (scope)

See `🧪wave1-rename-files.txt` for the bulk rename list (~45 files) plus follow-up edits to store/engine/vcs/bins/manifest and this report.

## Approach note

Renames applied ordered (compound identifiers first, then bare `Operation`, then `backwards`→`inverse`), protecting `OpText`/`OpBinary`/`print_op`/… Neural handled separately to free the `Operator` trait name.
