# Shared Value Clone Grants

## Outcome

The framework Value clone cursor now consumes an explicit item/byte grant on every advance and close call. Structural work consumes at most one item. UTF-8 copying consumes at most the supplied byte permit and the fixed 256-byte ceiling. Retained allocation capacity remains a whole-operation limit and is never conflated with copied-byte work.

The clone stores an incomplete UTF-8 scalar in a fixed four-byte cursor field. A one-byte grant therefore advances through a four-byte code point without creating an invalid `String` and without waiting for a later four-byte grant.

The immutable source abstraction is now the domain-neutral `protocol::value::DslValueSource` trait in the Value root. `Arc<DslValue>` implements it. The clone module no longer owns a clone-specific source trait, and the Store canonical admission can use the same indexed immutable source without depending on clone.

## API

- `DslValueSource::value(&self) -> &DslValue`
- `DslValueCloneGrant { maximum_items: usize, maximum_bytes: usize }`
- `DslValueCloneReceipt { structural_items: usize, copied_bytes: usize }`
- `DslValueCloneCursor::advance(grant) -> Result<DslValueCloneStep, &'static str>`
  - `Blocked(checkpoint)`
  - `Progress { receipt, checkpoint }`
  - `Complete { receipt, checkpoint }`
- `DslValueCloneCursor::take_value() -> Option<DslValue>` transfers a completed value and enters closing.
- `DslValueCloneCursor::cancel()` enters closing without releasing recursive ownership.
- `DslValueCloneCursor::close_step(grant) -> DslValueCloneCloseStep<R>`
  - `Blocked(checkpoint)`
  - `Progress { receipt, checkpoint }`
  - `Returned { source, receipt, checkpoint }`
  - `Complete(checkpoint)`
- `take_source` was removed. The exact source is transferred only by the terminal `Returned` close receipt.

The cursor and both step enums are `#[must_use]`. Cursor state is held in `ManuallyDrop`; normal debug Drop rejects live ownership, unwinding leaks it, and terminal Drop releases only an already-empty state. No Drop path recursively drains a partial value.

## Native laws

The language-neutral fixture declares grant sizes `0`, `1`, `7`, and `256`. The native serde_json-oracle tests cover:

1. all nine neutral values, exact numeric variants, ordered duplicate keys, and the large UTF-8 value;
2. zero-grant immobility;
3. independent item-only allocation and byte-only copy grants;
4. per-call receipt/checkpoint equality and the one-item/256-byte maxima;
5. forward progress through a four-byte code point under one-byte grants;
6. retained capacity remaining below the caller limit;
7. cancellation at every configured checkpoint for every grant size;
8. bounded close with a one-item/zero-byte grant and exact `Arc` identity return;
9. capacity/depth rejection with exact source return;
10. debug rejection of live recursive ownership in Drop.

## Validation

- `clone-grants-red-1.log`: expected RED against the old API; missing grant, receipt, and terminal close symbols.
- `clone-grants-green-1.log`: implementation compile exposed invalid `ManuallyDrop::as_mut`; corrected to its `DerefMut`.
- `clone-grants-green-2.log`: implementation compiled and ran three tests. Grant/cancellation and capacity/depth laws passed. The neutral-vector law failed only because its old assertion counted byte-copy calls as structural items; corrected to the exact three structural root-string steps.
- `clone-grants-green-3.log`: clone was not compiled because two unrelated mutation metadata fixture paths were stale after their authoritative fixtures moved.
- `clone-grants-green-4.log`: final four-law Nx run is active in tool session `81335` and is currently waiting for the ticket Cargo target lock. Root owns collecting this queued result.

All runs use `abstraction-ownership-validation:shared-value-clone` through Bun and Nx. The ticket validation wrapper fixes `CARGO_TARGET_DIR` to `🗑️generated/cargo-trinity` and `CARGO_INCREMENTAL=0`.

## Files

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧬️clone/🤝️contract.json`
- `🧰️framework/🔨️modules/🌱️value/🧬️clone/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧬️clone/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs`

The final file only updates two `include_str!` references to the authoritative `mutation/🧫️fixtures` locations. No old fixture directories were recreated.

## Root Handoff Checkpoint

The handed-off GREEN4 session is not accessible from the root tool session. Its log ends while waiting for Cargo, and process inspection found no remaining shared-value-clone/shared_value_clone command. It has no terminal success or failure receipt and is not validation evidence. Root added the independent zero-item-close regression, then started a fresh owned `clone-close-zero-red-1.log` run. No duplicate live clone run was created.

## Zero-Grant and Allocation Audit Follow-Up

`clone-close-zero-red-1.log` reached the expected missing `reserve_vec_with` compiler failure. Zero-item close now checks its grant before changing phase. Vector and string reservation share one checked boundary that rejects excessive requested capacity before allocation and excessive actual capacity before admitting the empty allocation. The regression tests exercise zero-item close before each active step and injected allocation capacities. GREEN validation is running in `clone-close-zero-green-1.log`; no final result is claimed yet.

## Final Audit Follow-Up Runtime Receipt

`clone-close-zero-green-2.log` is terminal GREEN: Nx exited 0 and all 5 focused native tests passed. Runtime debug receipts confirm exact 0/1/7/256 structural/byte grants, UTF-8 preservation, unchanged active phase after zero-item close, cancellation at every tested checkpoint, exact source return, injected allocator overcapacity rejection, and a caught expected live-owner Drop panic. The caught panic is the intended lifecycle rejection law and did not fail the suite.
