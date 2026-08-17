# Wave 5 Report — Operations → Mutations (TS/TSX/WIT)

## Scope

Framework OS + core TypeScript/TSX/WIT only. No plugin Rust, no taxonomy.json, no root `📜️script.ts`.

## Status

**Complete** (identifier rename). Vitest: **297 passed / 5 failed** on `@semio-tech/framework-renderer-react` when run directly via vitest (nx budget wrapper kills the suite at 15s/300s before results).

The 5 failures are **unrelated** to this rename:
1. `declarative forms parity > renders selectable builder cards with selection ring` — missing `ring-primary`
2. `framework renderer hosts > interprets virtual file system component scenes` — invalid element type object
3. `s workflow flow routing > isolates render faults in ShellFaultBoundary` — missing `toHaveTextContent` matcher
4–5. `shell option locks` mit-bestand funding logo path expects `️logo/` but got `️logos/` (separate demonstrator funding logos ticket)

Mutation-specific focused slice (live mirror / parseInvocation / action kind) **passed**.

## Repo MCP

Unavailable in this session (`Server "repo" not found`). Worked in existing open ticket `26/08/07/OPERATIONS-TO-MUTATIONS-AND-ARTIFACT-ENGINES`. Could not call `ticket_reopen` / `ticket_close` / read `repo://goals`.

## Renames applied (no aliases)

| Old | New |
|-----|-----|
| `KernelOperation` | `KernelMutation` |
| `InverseOperation` | `InverseMutation` |
| `OperationEnvelope` / `WireOperationEnvelope` | `MutationEnvelope` / `WireMutationEnvelope` |
| `operation_id` | `mutation_id` |
| `applyOperations` | `applyMutations` |
| `encode/decodeOperationEnvelopesPack` | `encode/decodeMutationEnvelopesPack` |
| `operationEnvelopeToWire` / `FromWire` | `mutationEnvelopeToWire` / `FromWire` |
| `remoteOperations` / `localOperations` / `pendingOperations` | `*Mutations` |
| `inverseOperations` | `inverseMutations` |
| `relayOperationsToHub` | `relayMutationsToHub` |
| `Puzzle2dLiveMirrorOperations` (+ collect/push) | `Puzzle2dLiveMirrorMutations` |
| `ActionDefinition.kind` `"operation"` | `"mutation"` |
| backbone message kind `"operations"` | `"mutations"` |
| WIT comment `apply-operations` | `apply-mutations` |
| `InvocationResponse.operations` / `UndoGroup.operations` | `mutations` (aligned with Rust `InvocationResult`) |

## Kept

- OpText / `️op` facet brand, `draw.operation` schema ids
- GraphQL `OperationDefinition`
- NodeGraph fixture `operations` / `operation: "setFixture"| "move"`
- Ink/event unrelated `operation` concepts
- `targetOperation` field on inverse (still matches Rust `target_operation` serde)

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Board2dHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellSync/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts`

## Logs

- `🧪wave5-vitest.txt` — full direct vitest run (297 pass / 5 unrelated fail)
- `🧪wave5-vitest-focused.txt` — focused rename slice (3 pass)
- `🧪wave5-vitest-mutation-slice.txt` — mutation-scoped tests (6 pass)
- `mcp-unavailable-wave5.txt` — MCP note

## Notes

- Rust DocumentEvent/DocumentSyncStatus already use `RemoteMutations` / `pendingMutations` / `mutation_id`; TS now matches.
- Generated `ActionKind` in `️manifest/generated` updated to `"mutation"`.
- `bun nx run …:test` without `long` dies at 15s budget with no results; with `long` still hung 300s under the budget wrapper (no progress). Direct `bunx vitest run` completes in ~6s.
