# Typed Inverse Contract Review

## Scope and conclusion

This is a read-only design review of the current mutation inverse seam. No production source, frozen Run review client, fixture, compiler artifact, Cargo target, or compose path was changed.

Attachment `pasted-text-1.txt` §5.4 requires every direct leaf to declare one of self inversion, an explicit inverse leaf, an inverse plan, or non-invertibility with a typed reason. The smallest sound cutover is a breaking replacement of both `Mutation::inverse` and `MutationKind::inverse`'s `Vec<Op>` result with one shared protocol-owned algebraic result:

```rust
pub struct MutationInverseReason {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

pub enum MutationInverse<Op> {
    Operations { first: Op, rest: Vec<Op> },
    NoChange,
    NonInvertible(MutationInverseReason),
}
```

`Operations` is intentionally non-empty. Static descriptor `invertibility` continues to declare the leaf's inversion strategy (`self`, `explicit-mutation`, `plan`, or `non-invertible`); it is not enough to carry a state-dependent reason. The runtime `NonInvertible` arm carries the stable code, human explanation, and target for the actual pre-state. No default, empty-vector compatibility result, synthetic removal leaf, or hidden replacement operation is sound.

## Required semantic distinctions

| Forward status | Typed inverse outcome | Store / undo behavior |
| --- | --- | --- |
| Applied and changed, exactly reversible | `Operations { first, rest }` | Persist the ordered plan; reverse operation-plan order at the existing replay boundary. |
| Applied and did not change the snapshot | `NoChange` | Persist the explicit no-change classification; undo consumes it without emitting an operation. |
| Applied and changed, but the domain has no valid restoration | `NonInvertible(reason)` | Persist the typed unavailable status; an undo request rejects before any compensating operation is applied. |
| Forward diff rejected by merge policy or fails `MutationDiff::apply` | no inverse result | Do not record the edit or publish an inverse. This is an apply rejection, not non-invertibility. |
| Target absent | `NoChange` only if the leaf's accepted forward semantics leave the snapshot unchanged; otherwise the forward path must reject | It must never be represented by `Operations { rest: [] }` or silently treated as undoable. |

The trait documentation must state that a caller invokes `inverse` only for a forward operation that has already passed its outcome-policy admission and whose diff applies to the supplied pre-state. This avoids a fourth `NotApplied` inverse variant: rejection belongs to `MutationOutcome` / `MutationApplyError`, not undo capability. Direct callers that need to inspect an unadmitted operation first use `diff` and policy admission.

`MutationInverseReason` should deliberately be a separate protocol type rather than `MutationApplyError`: the latter means “the supplied diff cannot be applied,” whereas this type explains a successfully applied, durable operation that cannot be compensated. The two wire shapes may be isomorphic (`code`, `message`, `target`) without collapsing their meanings.

## Why the current API is unsound

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs` defines `Mutation<P>::inverse(&self, base) -> Vec<Self>`. Its public façade, `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`, repeats the same `MutationKind<P, Op>::inverse -> Vec<Op>` contract and explicitly documents an absent target as `Vec::new()`.

That loses the distinction above before any caller can act on it. `MutationInvertibility` is already a required one-of field in the complete fourteen-field `MutationLeafDescriptor`; current descriptor parsing/emission in both derive sources also recognizes `non-invertible`. It contains no reason and cannot describe a pre-state-dependent outcome. Keeping that static enum and adding the runtime result is smaller and truer than widening the exact descriptor schema with a fabricated default reason.

This also exposes a current descriptor truth gap: `FinishRunNode` is statically labelled `explicit-mutation`, yet the retained actual Run review proved first insertion has no valid inverse. A replacement may yield an explicit restoration, but a total `explicit-mutation` declaration is not true for every legal pre-state. Until the leaf's domain is changed, it must be conservatively declared `non-invertible` and return a typed reason for its insertion state, while retaining an `Operations` result for a legitimate replacement. No `RemoveRunRecord` or history-erasing leaf is authorized merely to satisfy an inverse test.

## Mandatory adoption set

1. **Protocol contract.** Change `Mutation<P>` in `📡️replication/🎮️mutation` and `MutationKind<P, Op>` in `📡️spr/🎮️command` together. Delete the empty-vector absence documentation. Add the typed reason/result and its equality/serde tests in the protocol-owned module; do not add a conversion returning `Vec`.
2. **Descriptor authority and derive JSON parser.** Retain the four static `MutationInvertibility` values and make validation test the contract relationship: a statically `non-invertible` leaf may produce `NonInvertible` or `NoChange`; a self/explicit/plan leaf may not silently return an untyped empty result. Both current derive implementations (`🗣️dsl/✨️derive/🦀️component.rs` and its `📦️packages/🦀️rust/📦️glue.rs`) parse and emit the fourteen-field descriptor, so they must compile against the new type but need no mutation-specific switch.
3. **Aggregate derive.** `#[derive(Mutations)]` currently emits direct variant delegation at lines 1703 and 1771–1774. Its generated aggregate result must be `MutationInverse<Self>` and mechanically map `Operations` payloads through the aggregate variant, preserve `NoChange`, and preserve `NonInvertible(reason)` exactly. It must not inspect Run, Workflow, or any other leaf.
4. **Composite derive and fold.** `fold_plan_inverse` currently returns `Vec<Op>`, turns plan failure into `Vec::new()`, and flattens each local result. It must instead return `MutationInverse<Op>`: plan failure is a typed non-invertible planning reason; `NoChange` local steps are skipped; the first non-invertible local result stops composition and is reported with its reason; only an all-exact/non-changing plan yields a non-empty `Operations` plan or `NoChange`. `#[derive(CompositeMutation)]` only delegates to this helper and must remain that way.
5. **Artifact store history model.** `ArtifactStore::replay_mutations`, `replay_suffix`, and `replay_suffix_partitioned` currently calculate and flatten inverse vectors before policy admission, then cache `HashMap<String, Vec<Mutation>>`. `Edit.inverse: Vec<Mutation>`, its text/Spr printing and parsing, reload recomputation, and `edit_mutations()` all therefore erase status. The persisted model must carry one `MutationInverse<Mutation>` per forward operation (aligned with `forwards` and `mutation_meta`), or an equivalent lossless edit-level sequence. An edit containing any `NonInvertible` operation is durable but unavailable to undo; `undo` must fail atomically with the retained reason. A no-change entry is distinct and needs no compensating op. The store must first evaluate/apply candidate diffs and policy admission, then derive/persist inverses only for admitted operations; rejected operations never become typed non-invertibility records.
6. **Remote replay / quarantine.** The two suffix replay functions recompute inverses while rebuilding state. They need the same per-operation typed result and must not put an unavailable edit in `rebased_inverse` as an empty sequence. Quarantine stays a forward-policy result, separate from inverse availability.
7. **Kernel and plugin publication.** `🎠️kernel::InverseMutation`, `KernelMutation`, and `UndoGroup` presently require an opaque `inverse_diff`; Plugin's `result_from_last_edit` serializes the whole flattened edit inverse for every forward. Its grouped-child fallback intentionally writes `encode_ops_vec(&[])`. All of those transport shapes must gain an explicit inverse-status union or stop publishing an inverse for unavailable operations. Empty bytes must not mean success. The public action/command tests at Plugin lines 35186 and 36598 are consumers of real inverse bytes and must become status-aware; child best-effort publication cannot claim an empty inverse as undoable.
8. **Causal / database envelope consumers.** `📡️replication/🔗️causal::InverseMutation` and `🎠️kernel::InverseMutation` are opaque binary inverse carriers. `🛢️db/📄️artifact::undo` unconditionally swaps the original inverse into a compensating envelope. It must reject unavailable status before emitting that envelope. `📝️wal`'s `WAL_INVERSE` remains opaque only after the new status is encoded losslessly.
9. **Direct implementations and test helpers.** Every handwritten `Mutation` / `MutationKind` implementation intentionally breaks at compile time. Test helpers such as `assert_operation_round_trip` may accept only `Operations`; they should report `NoChange` and `NonInvertible` as distinct failed preconditions rather than treating either as an empty successful plan.

No compatibility wrapper returning `Vec`, descriptor default, default reason, or fake inverse leaf belongs in this adoption set.

## Observed source evidence

- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs` contains both the `Mutation` trait's vector result and the existing typed `MutationApplyError` / `MutationOutcome` boundary.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` contains the facade trait, `fold_plan_inverse`, and generated-composite target. Its current plan-failure `Vec::new()` is neither a no-change proof nor a non-invertibility diagnostic.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` contains only mechanical aggregate delegation for normal leaves and helper delegation for composites, which is the right propagation point.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` computes inverses in local replay, remote suffix replay, and partitioned replay; it also exposes the flattened edit inverse to publication. Its `replay_mutations` evaluates `inverse` before collecting / admitting all outcome messages, so it currently cannot distinguish rejected forward attempts from a durable unavailable inverse.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` builds `KernelMutation.inverse` by encoding that flattened vector and uses an empty encoded vector for grouped children. This is a visible false-positive undo capability.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, `📡️replication/🔗️causal/🦀️component.rs`, and `🛢️db/📄️artifact/🦀️component.rs` are transport / inverse-undo consumers that require an availability representation before the generic cutover is complete.

The scoped inspection found static `"non-invertible"` descriptor values for the four Run leaves `StartRun`, `StartRunNode`, `AppendRunLog`, and `SealRun`, plus Workflow `UpdateNodePorts`. This is a scoped fact only; it is not a claim about the whole repository.

## UpdateNodePorts sentinel review

`UpdateNodePorts` is a zero-field leaf. Its diff is always `WorkflowDiff::SyncNodePorts`; application rebuilds parameter-derived input ports and node height through `sync_workflow_parameter_ports`. The helper is idempotent, but it can change an unsynchronised workflow. Its current `Vec::new()` therefore cannot establish that it is a harmless no-change.

Before accepting its static `non-invertible` label, add two source-owned sentinel tests:

1. A canonical already-synchronised snapshot must apply `SyncNodePorts` unchanged and return `NoChange`.
2. An intentionally stale parameter-port / height snapshot must change under the same operation and return `NonInvertible` with an exact code, message, and `nodes` target unless a genuine restoring leaf is introduced.

If the stale case cannot be constructed under snapshot invariants, remove the leaf rather than retaining a fake non-invertible operation. This review makes no claim which result will hold until those tests execute.

## Required verification after implementation

- Schema-first vectors for exact one-operation inversion, multi-operation plan order, accepted no-change, typed non-invertibility, rejected forward operation, and a composite containing each state.
- A third-party / language-neutral runtime should decode the public inverse-status union and reject omitted or mixed status fields.
- Rust compiler/runtime tests must prove aggregate delegation preserves all three variants, composite propagation stops before an unavailable inverse, store undo rejects atomically with the exact reason, remote replay retains the status, and plugin/kernel publication never encodes an unavailable inverse as an empty op vector.
- Existing Run first-insertion red must become a typed non-invertibility assertion, not a fabricated history-removal test. Replacement inversion remains a separate full-snapshot order law.

## Remaining uncertainty

- This review did not enumerate every handwritten inverse implementation; it identifies the mandatory generic boundaries and observed consumers, not a full-monorepo count.
- The persisted `Edit` record is re-exported through the VCS layer; its exact owning declaration and all serialisation schema migration sites need a dedicated implementation pass. The change is intentionally breaking: historical `inverse: Vec<Op>` records must not receive a guessed default status.
- The attachment does not specify whether static `non-invertible` means “never invertible” or “not total over legal pre-states.” The conservative interpretation above is the only one compatible with `FinishRunNode` without inventing a fifth descriptor enum value. Root should freeze that wording before changing descriptor expectations.

## Coordinator Review Boundary

The three-state runtime result is the selected direction for the forthcoming generic contract cutover, not an already implemented API. Static `non-invertible` must conservatively mean inversion is not guaranteed over every admitted pre-state; it may therefore produce an exact `Operations` result for a reversible instance, `NoChange` for an unchanged instance, or a typed `NonInvertible` reason. This resolves the inconsistent narrower wording in adoption item2 above without inventing a fifth static classification.

Aggregate delegation must preserve the leaf result directly: `MutationKind<P, Op>` already returns the owner's aggregate `Op`, and an inverse may name a different leaf. It must not wrap every inverse in the forward variant as suggested by adoption item3. That would both change semantics and reintroduce a per-forward inverse restriction.

Before production adoption, the exact direct implementations, persistence schema owners and affected peer runtime boundaries must be enumerated and assigned. Current peer lifecycle work is independently active in Kernel/Plugin; no source edit to that ownership is authorized by this design note. There will be no parallel old-vector compatibility method or guessed persisted status. Existing first-insertion evidence stays red until the typed result reaches the real undo consumers.
