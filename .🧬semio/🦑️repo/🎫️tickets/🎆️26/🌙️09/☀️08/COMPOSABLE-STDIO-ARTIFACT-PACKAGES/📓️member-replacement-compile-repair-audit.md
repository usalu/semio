# Member-Replacement Compile-Repair Audit

Captured: 2026-09-09T16:02:23+02:00. Read-only source review; no build or test was run.


## Scope and retained diagnostic

`🗑️generated/semio-full-recovery-3-compiler-diagnostics.json` records the prior `semio-framework-plugin` compiler result (`361950c94d7e664b`). Its relevant rows were the missing `Send` bound on `MemberFactory::Open`, the one-parameter `ArtifactStoreReplacementAdmissionTarget`, and the non-exhaustive replacement poll over newly introduced lifecycle states. This review inspected their current source settlement only.

## Result

No source-level defect was found in the three repairs.

- `🏪️store/🦀️.rs` now constrains `MemberFactory::Open` as `MemberOpenOperation<Member = Self> + Send`. The retained `M::Open` owner is stored in `ActiveArtifactStoreReplacement` and reaches the `PluginApp` worker boundary; the constraint belongs on that deferred open operation, without unnecessarily requiring every `SpaceMember` itself to be `Send`.
- `🔌️plugin/🦀️.rs` parameterizes `ArtifactStoreReplacementAdmissionTarget<'_, A, M>` with the same `M: SpaceMember + MemberFactory` that types its `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., M>>`. Its target rejects schema, dialect, capacity, and initialization-job refusal by returning the exact input envelope. `ArtifactEnvelopeCompletedRecordOwner::try_publish_to` restores that rejected envelope to its app-owned slot, so this generic repair does not create a fallible stack-local ownership escape.
- The eleven current `ActiveArtifactStoreReplacementState` variants have an exhaustive public poll mapping: `Initializing`, `AwaitingMembers`, `OpeningMembers`, `ValidatingClosure`, `PreparingCandidateViews`, and `CandidateReady` return `Pending`; `ClosingRejectedMember` and all three retirement states return `Progress`; a terminal committed non-faulted job is `Ready`, a terminal cancelled job is `Cancelled`, and the remaining terminal case is `Fault`.

The maintenance dispatcher matches that meaning. Cancellation while awaiting, opening, closing a rejected member, validating, or preparing sends the job to `RetiringRejectedMembers`. A member-open rejection first enters `ClosingRejectedMember`, then its retained `M::Open` is closed by `drive_rejected_members` under the supplied item/byte grant. The same bounded retirement drains the active ingress, registry, child owners, and candidate before terminal completion. `Drop` still asserts `Complete` and emptiness of each retained session, store, ingress, open-operation, candidate registry, and child-retirement owner. The repair preserves strict terminal ownership and does not downgrade rejection, cancellation, or denial into success.

## PDF default-feature visibility

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/Cargo.toml` has `default = []` and non-optional dependencies on both `semio-framework-os-kernel` and `semio-framework-plugin`. The plugin package also has `default = []` and a non-optional kernel dependency. The canonical definition is `🏪️store/🧩️composition/🚪️open/🦀️.rs:15`; it is re-exported at `🏪️store/🦀️.rs:40-43` as `MEMBER_OPEN_IDENTITY_BYTES`, with no feature guard. The plugin's closure driver therefore sees the current public `store::MEMBER_OPEN_IDENTITY_BYTES` path under PDF minimal features.

This establishes source and manifest visibility only. A fresh Semio/PDF compile acceptance remains pending; this audit makes no runtime-passing claim.
