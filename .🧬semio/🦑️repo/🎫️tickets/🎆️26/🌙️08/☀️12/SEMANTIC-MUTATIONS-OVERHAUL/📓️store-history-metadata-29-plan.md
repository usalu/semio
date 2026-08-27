# Store History Metadata 29

## Ownership

The aggregate owner is `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🦀️.rs`.
Its only direct mutation children are `📌️commit-space-checkpoint`,
`🌿️create-space-alternative`, `🔀️switch-space-alternative`,
`🗑️remove-space-checkpoint`, `🧹️remove-space-alternative`, and
`🎯️restore-active-space-alternative`.

`RestoreActiveSpaceAlternative` replaces the inverse-only broad setter. Its payload always
carries `alternativeId`, including explicit `null` when restoring trunk state.

## Roster

> Blocked pending root vocabulary decision: `commit`, `switch`, and `restore` are not in
> `protocol::APPROVED_VERBS`, while `#[derive(Mutations)]` compile-time rejects an unapproved
> `MutationKind::SEMANTICS.verb`. The table records direct operation language, not yet an
> accepted compilable `SemanticDescriptor` roster.

| Variant | Semantic kind | Verb | Entity | Record |
| --- | --- | --- | --- | --- |
| CommitSpaceCheckpoint | commit-space-checkpoint | commit | space-checkpoint | CommittedSpaceCheckpoint |
| CreateSpaceAlternative | create-space-alternative | create | space-alternative | CreatedSpaceAlternative |
| SwitchSpaceAlternative | switch-space-alternative | switch | space-alternative | SwitchedSpaceAlternative |
| RemoveSpaceCheckpoint | remove-space-checkpoint | remove | space-checkpoint | RemovedSpaceCheckpoint |
| RemoveSpaceAlternative | remove-space-alternative | remove | space-alternative | RemovedSpaceAlternative |
| RestoreActiveSpaceAlternative | restore-active-space-alternative | restore | active-space-alternative | RestoredActiveSpaceAlternative |

The aggregate only derives mechanical delegation. Each leaf owns its payload, descriptor,
diff, inverse, label, and target. Existing store JSON text and pack codecs remain aggregate
mechanics.

## Resolved Algebraic Flaw

The former inverse of `RemoveSpaceAlternative` returned only `CreateSpaceAlternative`.
Creation activates the alternative, so removing an inactive alternative could not restore the
prior `activeAlternativeId`. With root authorization the leaf now returns restore-before-create;
the reversed inverse plan recreates the alternative first and then restores the prior active
identity. Leaf and Store regressions cover removing both an inactive and an active alternative.

## Open Semantic Inventory

This adoption intentionally does not resolve the pre-existing ordered-removal restoration,
dangling active-alternative identity, or dangling checkpoint-reference concerns. The repaired
inverse regression proves only that removal of an inactive or active alternative restores the
captured active identity after its own recreation. Codec ownership remains aggregate-generic:
the public text and binary surfaces have no leaf opcode or numeric tag yet, so descriptors keep
those fields `null` while declaring the actually supported aggregate surfaces.
