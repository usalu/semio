# Retained Borrowed Input Boundary

Root's actual Drawing native2 compilation reached two E0271 diagnostics at retained-command line586. The unconstrained input literal inferred domain associated types as Arc types because the same original Arc reference was supplied both as the domain snapshot and its optional owner capability.

The literal now explicitly names `ArtifactCommandInputs::<A>`, borrows the domain snapshot/config through the original Arc's `as_ref()`, and keeps the same original Arc reference in `snapshot_owner`. It introduces no clone, new header, replacement read or new ownership capability. The host's independent normal-work demand/grant checks remain unchanged.

The existing neutral immutable-reader fixture and native original-pointer assertion (`std::ptr::eq(owner.as_ref(), input.snapshot)`) describe the intended boundary. This report records the compiler red and narrow repair, not a fresh native pass. Root coordinates the canonical Drawing production replay after the Base SQL guard frontier settles.
