# Retained Total Binding

The archived browser sample measured 489,649,268 copy bytes in the bridge's operation remainder. The subsequent guest close fault showed 225,712,402 copy bytes in its spendable view and approximately half the bridge remainder in items, capacity and release. The recorded poll and input pairs accounted for about 4.9 KB of copy each on average, including larger poll outliers. These samples compare different views from different points in the operation.

The guest decoder in `🧵️job/⏱️context/🛬️owned/🦀️.rs` quotes half the current quantum for an unquoted native receiving scope. The scope issues a persistent escrow claim covering items, copy, capacity, release and work. `OriginalReturnBudgetSnapshot::raw_remaining` subtracts physical settlements. Its `remaining` additionally subtracts amounts held for live claims. Settlement charges the redeemed amount and returns the unused hold. The staged trace is consistent with a live scope. A same-epoch comparison of physical and held counters would be needed to attribute the exact gap or certify that the earlier 11,740-byte difference was fully accounted for.

`OriginalActorContext::validate_currency` already authenticates the epoch grant as a subset of its initial physical grant minus fresh spend. `borrow_context` also subtracts the epoch's fresh effects before passing the total to the Job context. A new Job context copies the independent return book's spendable stock into `original_total_retained` on attach. The current `bind_original_retained_total` takes the per-axis minimum with that attached snapshot, so a legitimate hold can reduce the bound total for the entire borrowed context. The snapshot may also reflect settlements in an older escrow origin. It cannot authenticate a new physical total.

The narrow correction would reject a proposed total when any axis exceeds the fresh physical remainder, an earlier explicit binding, or the current policy. Existing escrow availability would still constrain business spending, and an explicit policy would remain narrow even if unspent escrow returned mid-turn. Binding an actor total would preserve the existing escrow origin and would leave fuel and retirement policy untouched.

The focused Job contract test reserves a real sized return claim, binds the physical remainder, and checks that the spendable view still excludes held credit. It redeems part of the claim and verifies that the physical debit is recorded once and unused credit returns. It also checks each axis for overissue and exercises rebinding under both a prior cap and an active policy.

The pinned Rust compiler parsed the new contract test. A standalone Rust harness built from the current and proposed binding method bodies rejected the original at its overissue assertion and passed with the proposed check. This harness substitutes a finite budget to exercise the binder independently. Ajv validates the portable Job fixture against its schema, and SQLite produced the expected bind or reject result and spendability for five independent cases.

The strict binding is implemented in `🧵️job/⏱️context/🦀️.rs`. It validates against the physical remainder, the current turn policy and any previous explicit binding. The current escrow book still determines how much of the bound total can be spent. The native WGPU caller now binds inside its existing business loan after admission has accounted for current holds and same-call setup charges.

The pinned Rust compiler parsed the production changes and the real-claim contract test. Running the full Cargo contract was deferred to leave the constrained compiler storage available to the component builds. The archive build had already compiled its Job context when the strict binder landed, so its archive probe alone cannot verify the new binder.

## Files

- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs`
