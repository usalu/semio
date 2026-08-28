# Workflow Run Direct 31 Plan

## Canonical Owner Taxonomy

`WorkflowMutation` owns the 18-leaf collection at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs`.
`RunMutation` owns the five-leaf collection at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs`.
Both are mechanically derived aggregates. The existing workflow component mounts and re-exports
them while retaining snapshots and shared domain types only. No hand-filled descriptor array, old
inline enum, or `WorkflowMutationDsl`/`RunMutationDsl` conversion twin remains.

Every leaf owns `🦀️.rs`, `🔣️.json`, and `🧬️schema/🔣️.json`; each descriptor points to the
latter. The aggregate owns mechanical dispatch and codecs. Existing DSL text keywords and binary
variant ordinals are preserved as descriptor codec identities rather than silently reassigned.

All 23 descriptors have these fixed values: `schemaVersion: 1`, `payloadSchema:
"🧬️schema/🔣️.json"`, `diffParticipation: "apply-only"`, `composition: "atomic"`, and
`requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"]`. They have no
TypeScript, GraphQL, or protobuf surface today. `textOpcode` is a real DSL keyword and `binaryTag`
is its current zero-based `DslVariants` ordinal in the `format=1 | ordinal-varint | body` codec.

## Workflow Roster

| Owner | Variant / semantic kind | Verb / record | Text / binary | Invertibility |
| --- | --- | --- | --- | --- |
| `➕️add-node` | AddNode / add-node | add / AddedWorkflowNode | add-node / 0 | explicit-mutation |
| `🗑️remove-node` | RemoveNode / remove-node | remove / RemovedWorkflowNode | remove-node / 1 | plan |
| `🔗connect-ports` | ConnectPorts / connect-ports | connect / ConnectedWorkflowPorts | connect-ports / 2 | explicit-mutation |
| `✂️disconnect-edge` | DisconnectEdge / disconnect-edge | disconnect / DisconnectedWorkflowEdge | disconnect-edge / 3 | explicit-mutation |
| `↔️move-node` | MoveNode / move-node | move / MovedWorkflowNode | move-node / 4 | explicit-mutation |
| `✏️rename-node` | RenameNode / rename-node | rename / RenamedWorkflowNode | rename-node / 5 | explicit-mutation |
| `🧩add-parameter` | AddParameter / add-parameter | add / AddedWorkflowParameter | add-parameter / 6 | explicit-mutation |
| `🧹remove-parameter` | RemoveParameter / remove-parameter | remove / RemovedWorkflowParameter | remove-parameter / 7 | plan |
| `🩹change-parameter` | ChangeParameter / change-parameter | change / ChangedWorkflowParameter | change-parameter / 8 | explicit-mutation |
| `🔒bind-parameter-field` | BindParameterField / bind-parameter-field | bind / BoundWorkflowParameterField | bind-parameter-field / 9 | explicit-mutation |
| `🔓unbind-parameter-field` | UnbindParameterField / unbind-parameter-field | unbind / UnboundWorkflowParameterField | unbind-parameter-field / 10 | explicit-mutation |
| `🔄update-node-ports` | UpdateNodePorts / update-node-ports | update / UpdatedWorkflowNodePorts | update-node-ports / 11 | non-invertible |
| `📥add-input` | AddInput / add-input | add / AddedWorkflowInput | add-input / 12 | explicit-mutation |
| `🚮remove-input` | RemoveInput / remove-input | remove / RemovedWorkflowInput | remove-input / 13 | plan |
| `🔌bind-input` | BindInput / bind-input | bind / BoundWorkflowInput | bind-input / 14 | explicit-mutation |
| `🚪unbind-input` | UnbindInput / unbind-input | unbind / UnboundWorkflowInput | unbind-input / 15 | explicit-mutation |
| `📤bind-output` | BindOutput / bind-output | bind / BoundWorkflowOutput | bind-output / 16 | explicit-mutation |
| `⛔️unbind-output` | UnbindOutput / unbind-output | unbind / UnboundWorkflowOutput | unbind-output / 17 | explicit-mutation |

Every workflow descriptor has `outcomeClasses: ["applied"]`. The existing apply boundary may
return `MutationApplyError`, but no leaf currently adds outcome diagnostics. The three `plan`
operations preserve cascade restoration order: removed node restores dependent edges/bindings;
removed parameter restores parameter bindings; removed input restores input bindings.

## Run Roster

| Owner | Variant / semantic kind | Verb / record | Text / binary | Invertibility |
| --- | --- | --- | --- | --- |
| `🚀start-run` | StartRun / start-run | start / StartedRun | start-run / 0 | non-invertible |
| `▶️start-run-node` | StartRunNode / start-run-node | start / StartedRunNode | start-run-node / 1 | non-invertible |
| `✅finish-run-node` | FinishRunNode / finish-run-node | finish / FinishedRunNode | finish-run-node / 2 | explicit-mutation |
| `🪵append-run-log` | AppendRunLog / append-run-log | append / AppendedRunLog | append-run-log / 3 | non-invertible |
| `🔏seal-run` | SealRun / seal-run | seal / SealedRun | seal-run / 4 | non-invertible |

The shared approved vocabulary must add exactly `start/Started`, `finish/Finished`,
`append/Appended`, and `seal/Sealed`; this is root-owned. Every run descriptor has
`outcomeClasses: ["applied"]`; sealed-run and already-started rejection stays at the existing
checked application boundary until a separately authorized outcome migration.

## Schema-First Matrix

Each leaf schema is a strict payload object (`additionalProperties: false`) and the aggregate
schema is a strict `operation` envelope with one branch per leaf. Shared `$defs` in the aggregate
schema carry only current persisted domain shapes (`WorkflowNode`, `WorkflowEdge`,
`WorkflowParameter`, `WorkflowInput`, `WorkflowParameterBinding`, `WorkflowInputBinding`,
`WorkflowOutputBinding`, `RunParameterValue`, `RunNodeRecord`, `RunTrigger`, and `RunStatus`);
leaf schemas reference those definitions through stable aggregate-local `$ref`s.

The neutral fixture matrix must contain one representative valid payload for all 23 leaves, one
invalid envelope tag, one unknown-field case for every payload category, and codec assertions for
each fixed text opcode and binary tag. It separately verifies JSON-schema acceptance from Rust
semantic apply/inverse behavior. The existing 18 + 5 OpText/OpBinary round-trip vectors become
leaf-owned codec vectors; aggregate tests only assert structural roster/order correspondence.

## Leaf Roles And Write Boundaries

Each leaf is the only owner of its payload struct, `MutationKind` semantics, forward diff, inverse,
DSL field attributes, descriptor, and payload schema. `RemoveNode`, `RemoveParameter`, and
`RemoveInput` additionally own their ordered restoring plans. `FinishRunNode` owns replacement
inverse behavior; the other four Run leaves explicitly own their non-invertibility. The two
aggregate roots may only wrap leaves, delegate the derive, and perform codec dispatch from the
leaf-declared keyword/tag. Shared snapshot types, `WorkflowDiff`/`RunDiff` application mechanics,
and checked sealed-run admission remain in their existing source owners; no leaf duplicates them.

## Write Set And Callers

- `🔁️workflow/🦀️component.rs`: replace the two inline enums/manual `Mutation` impls and the two
  private DSL twins with canonical aggregate mounts/reexports; retain common snapshot/diff domain
  types only.
- New `🔁️workflow/🧬️schema/🧬️mutations/🦀️.rs` plus its 18 direct leaf owners, and new
  `🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs` plus its five direct leaf owners.
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` and
  `💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs`: keep their public workflow mount, but update any
  canonical source-path assertion/mount that names the former aggregate location.
- Constructors and pattern matches migrate from inline named variants to wrapped payload structs in
  `🔁️workflow/🦀️component.rs`, `🏃️run/🦀️component.rs`, `🏃️run/📦️bin.rs`, and
  `💻️os/🖥️host/🦀️component.rs`. `🔌️plugin/🏗️builder/🦀️component.rs` is documentation-only unless
  its compile-time imports prove otherwise.
- Existing workflow/run and host OpText/OpBinary tests migrate to leaf fixtures; no generated target
  and no FlowConfigMutation arm is edited.

No production source has been edited by this proposal, and no Cargo command has run.
