# Note Archive Record Allocation

Importing the 805-byte Note archive reached the persisted history `DecodeForward` phase and failed with `retained record-body physical credits`. That phase admits the next mutation through the OS DSL reader, whose record-body phase uses the default Pack limits.

The default total allocation policy is 4 GiB. The record-body receiver previously converted the policy to `usize` and fell back to `usize::MAX` on wasm32. The body cursor requires an allocation ceiling no higher than `isize::MAX`, so it refused the receiver even for a short mutation. Record-body admission now caps the policy at `isize::MAX` before converting it, as full-document admission already does. Per-turn grants and actual allocation accounting use their existing limits.

The retained Pack test uses neutral allocation policy rows for 32-bit and 64-bit targets. It decodes the same record under each policy, checks the decoded value and per-turn grant, and retires both the decoder and the decoded record. The scoped whitespace check passed. Browser import and close verification require a guest built from the updated receiver.
The independent SQLite numeric oracle agreed with all six fixture outcomes. Rustc dep-info for staging attempt 41 identifies the prior Pack decoder body, so the archive correction needs the next guest publication. The current attempt includes the utility-switch and paid-retirement changes.

## Native worker close after replay

The fresh archive no longer failed at the Pack limit. A focused HMR-off browser run retained the guest trace before the actor transport reduced its error to a semantic tag. The live cleanup continuation for imported instance 2 reported `original worker close reported fewer effects than its original recipient`. The cleanup ladder then reported `prior-outcome` in epoch 28883. The browser still showed the original empty Demo.

The worker close wrappers previously compared reported current-turn progress against the entire physical delta in the original recipient. The physical recipient also includes previously reserved returns when a nested close redeems them. The same worker authority already removes those redemptions when settling its ordinary operation receipt. The close wrappers now do the same reconciliation before charging only the remaining current effects. The invariant still refuses an underreported current effect.

The neutral four-case fixture covers a redemption-only turn, mixed progress, ordinary progress and a genuine current-turn underreport. Ajv validates the fixture and an independent SQLite oracle checks all four outcomes. The receipt source assertion failed before the change. The Rust law exercises the actual recipient method. Native and browser verification of this correction are pending. Staging attempt 43 had already compiled the previous worker authority before this edit.
