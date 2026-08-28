# Flow Mutation Retirement 44

## Existing Handoff

The current `FlowMutationRetirement` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` owns `Option<FlowMutation>` but, after checking only `maximumItems`, clears it and reports `Complete`. It neither observes a zero-byte grant nor decomposes the direct payload.

A suitable owned handoff already exists: `FlowRetirement` and `FlowOwner` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧵️retained/🦀️component.rs`. `FlowRetirement::close_step` is bounded, refuses zero item or byte grants with `Blocked`, preserves its frontier on every nonterminal step, and reports `Complete` only when `is_empty()`.

## Released-Block Design

The replacement keeps the existing factory and `FlowMutationRetirement` outer owner, but gives it two phases:

1. `mutation: Option<FlowMutation>` transfers a concrete variant into a private `FlowRetirement` frontier only after both grants are positive; it reports a bounded transfer `Pending { releasedItems: 1, releasedBytes: 0 }`.
2. The retained frontier receives subsequent grants unchanged. Its `Complete` result is accepted only when the outer mutation is absent and the frontier is terminal-empty. All zero grant/refusal paths leave both owners intact; errors from the nested owner are propagated with the frontier still owned.

No `Option::take` may be used as a release certificate: every take must immediately move its fields into `FlowOwner` values. No cold/synchronous retirement loop is permitted.

The ten concrete mappings are:

| Direct leaf | Payload ownership moved to `FlowOwner` |
| --- | --- |
| `AddWidget` | `Widget` |
| `RemoveWidget` | `Bytes(id.into_bytes())` |
| `MoveWidget` | `Bytes(id.into_bytes())` |
| `ChangeWidget` | `Bytes(id.into_bytes())`, then `Widget` |
| `AddSynapse` | `Specs(vec![synapse])` |
| `RemoveSynapse` | `Bytes(id.into_bytes())` |
| `MoveSynapse` | `Bytes(id.into_bytes())` |
| `ChangeSynapse` | `Bytes(id.into_bytes())`, then `Specs(vec![synapse])` |
| `ChangeLayout` | `Layout(entries)` |
| `ReplaceFlowFixture` | `Fixture(fixture)` |

Indices are scalar coordinates and require no owned frontier entry.

## Bounded Write Set

- Existing authorized block only: `🌿️vcs/🦀️component.rs` (`FlowMutationRetirement` and its factory).
- New owner-local helper: `🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs`.
- New owner-local native tests and neutral schema/vectors beneath `🌿️vcs/🧬️schema/🧹️retirement/`.
- Ticket-only controller `🧪️flow-mutation-retirement-44/📜️script.ts` and retained report.

The helper sits beside the schema root, not as a direct child of `🧬️mutations`; every child of that collection remains a concrete operation. It imports the transparent direct aggregate and retained `FlowOwner`/`FlowRetirement` seam but has no FlowDiff, registry, plugin, or config dependency.

## Test Matrix

Neutral vectors will cover all ten variants, zero item grant, zero byte grant, cancellation/refusal before transfer, nested fault propagation with ownership retained, bounded multi-step completion, and the invariant that `Complete` is emitted only after the owner is empty. The controller will compile the vector schema through Ajv and compare its state machine to a third-party JSON reference. Native tests will exercise the real helper and factory but will not be executed until the root-controlled Flow build slot opens.

The adjacent `FlowFixtureRetirement` and `FlowSnapshotRetirement` bytes are excluded. FlowDiff/schema/controller work, SharedRegistry, Plugin Flow `SetContributions`, collection replacement/removal, and partial decode cleanup are excluded.
