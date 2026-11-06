# In-place owner map

NativeOwnedPrefix stores MaybeUninit array and tracks initialized prefix, but push_from moves an entire T from Option. NativeOwnedStorage reserves the real typed backing and lends uninitialized_ptr; put_from and take_into still move whole T. Its take_box transfers initialized backing without moving T.

NativeRecordStorage constructs individual fields in the actual reserved backing and tracks initialized fields, but take_initialized currently reads the whole T by value; it does not commit initialized backing for take_box. A genuine generic in-place output seam must preserve partial-field ownership and close, validate initialized state and transfer actual backing. No implementation or test run.
Atomic handoff reread: zero-work and physical denial preserve custody, success transfers original backing, and the current finite-stock law prechecks raw fixed-work turns and quotes controlled retirement before executing. No concrete alignment or ZST defect found under its unsafe full-valid-T precondition. This is uncompiled source review only.
