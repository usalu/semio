# Erased factory child original control

FactoryOwnedRetirement stores an erased unique child returned by its installed value factory, but its ErasedSnapshotRetirement implementation does not override requires_original_control or controlled close and selects ordinary close_factory_ticket for that child. FactorySharedRetirement has the same shape after its alias and pending-value stages. As their factory return type is Box<dyn ErasedSnapshotRetirement>, the actual selected child can require original control. The default false hides this requirement and routes the child through ordinary closure.

Select and forward the actual unique child controlled path only when it is the next action and requires it. Original or pending payload admission, alias transfer, factory handoff and terminal outer shell are distinct local actions, with their own selected work; do not double-reserve a child that already owns original work admission.

The native decoder retirement recipient explicitly requires original control and delegates selected child through controlled/original ticket paths; its reserved-slot shell admits and commits locally. This is a focused source review, not runtime verification.
