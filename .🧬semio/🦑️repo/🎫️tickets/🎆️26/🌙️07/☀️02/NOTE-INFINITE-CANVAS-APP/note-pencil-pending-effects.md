# Note Pencil Pending Effects

The attempt 51 browser experiment mounted the real guest with the isolated startup fix. After Pencil selection and the first canvas input, the guest panicked at the ordered-map destructor. Every following interaction trapped and no stroke was published.

The first panic's Wasm stack includes function 30342, which reads the retained ordered-map destructor diagnostic from address 9,145,308. Its caller, function 86741, retires the local view fields and is called by function 86990 at `0x2215f54`. The function body and preserved location at address `0xe37e3c` identify `plugin_drive_pending_effects` in `typed-operations/🦀️.rs`, specifically the path that dropped a temporary `ViewModel` containing the live Pencil utility map. This is a second entry point to the same owned-view failure previously found in exchange finish.

Background turns and explicit surface refreshes now borrow the registered surface view and the mutable app as disjoint fields inside their existing instance guards. The refresh site had the same clone, synchronous poll and implicit drop. The exchange-finish path already borrows. The separate encoded pure-command path takes a view into owned command metadata and was outside the reproduced stack.

## Verification

- The focused Bun effect-view and effect-admission test files passed five tests and 36 assertions. The SQLite fixture independently checked that paused and published polls each leave one registered surface owner.
- The stand-alone Rust probe compiled the exact background and refresh poll calls from current source, under their instance guards. Both baseline clone paths reproduced the temporary-view drop failure (exit 101). Both guarded borrows completed the pending then published sequence (exit 0). Run with `repo tools exec --light python -- '<ticket>/verify-pending-effect-borrow.py'`.
- The production browser guest needs regeneration before Pencil rendering, Undo and erasers can be verified on the corrected guest.

## Changed Files

- [Background effect poll](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs)
- [Surface refresh poll](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs)
- [Focused effect-view test](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🏁️exchange-finish/effect-view.test.ts)
- [Stand-alone Rust probe](verify-pending-effect-borrow.py)
