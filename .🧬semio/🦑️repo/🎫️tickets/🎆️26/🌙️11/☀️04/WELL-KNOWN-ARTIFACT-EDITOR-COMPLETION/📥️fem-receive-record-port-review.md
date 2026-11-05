# FEM Receive Record Port Review

Read-only concrete current mechanical issues reported to root:

- Both receive-record row/lists/variants/member/load/root receive_nested calls still explicitly supply two generic arguments; actual NativeSnapshotDecodeOwner now requires T,O,E. IdentityIndex nested Self,Self has same omission.
- Both support() FemDof vector allocation snippets refer to nongeneric T in size_of<Vec<T>>/size_of<T>; replace only those types with FemDof.
- DOF loop performs enum lookup/duplicate validation and push without recording each actual selected action; allocation alone is counted.
- declared() rejects wrong record field count before work/performed, producing work0 for an attempted semantic header validation. number finite-value validation similarly has no distinct validation work. Pin actual intended validation units rather than infer copied_items.

Vector allocation paths otherwise admit then checkpoint before performed+allocation; child outcomes are passed via meter.take. No source mutation/compiler job performed.
