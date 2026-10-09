# Guest Scoped Loan Current

Read-only 2026-10-09. Source review only, no execution. Proposed bridge is sound as a directional loan if the following receipt conditions are enforced.

- Original selected decode/encode remains borrowed for the whole invocation. `remaining = maximum_bytes.checked_sub(owned_bytes)` is the child ceiling; keep the prior original owned offset and verify every child allocation `(owned,next,maximum)` before charging the actual original control. Update child ledger only after original charge succeeds. Preserve the first original refusal instead of replacing allocation/work refusal with generic cancellation.
- A `RefCell<&mut OriginalControl>` lets observer and allocation closures share one original control synchronously. Use `try_borrow_mut` and return an invariant refusal on reentry; avoid panicking `borrow_mut`. Release the borrow before calling runtime continuation functions. Actual original `charge` already forwards its installed allocation port and its cancellation checkpoint.
- `scope_maximum` is already represented in current `maximum_bytes`. Do not call `admit_turn_capacity` or restore a larger ceiling inside the bridge. Scoped maximum guards must remain outside the child loan; prefunded maximum minus owned prevents allowance reset.
- Map child aggregate progress to the same original selected direction. `begin_stage(total); advance(completed)` publishes the actual cumulative owned bytes, but repeated reports reset stage/work and issue duplicate zero-progress observations. Track last `(completed,total)` if the guest reports one monotonic stage, and advance the delta. If reports represent independently restarted stages, identify actual phase transitions rather than guessing from `total` alone. Reject completed > total before original mutation where total is finite. Do not use guest allocation-owned bytes as original owned bytes; original includes prefunding.
- Both original `scoped_stage` implementations restore parent workload after normal return/refusal, retain allocation charges, and do **not** restore on unwind: decode `🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs:83–87`, encode sibling 93. Avoid claiming unwind restoration for scoped_stage. Existing `scoped_observer` uses an actual Drop guard and preserves ports/counters on unwind; an analogous stage guard is needed if unwind is in the contract.
- The same retirement recipient remains installed when the original control is borrowed. A forwarded identity receipt alone does not transfer retained VM failure custody into that recipient: explicitly distinguish host VM-owned cleanup from provider native intermediate/output cleanup. Snapshot import's `NativeSnapshotEncodeOwner::receive` remains the authentic native failure custody boundary.

## Explicit callback census

Current repository Rust declarations observed:

1. Store generic `export_snapshot_impl` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12304`: real typed export callback.
2. MCP `guest_sqlite_export` at `…/🌉️mcp/🏠️workspace/🦀️.rs:3185`: real Guest callback, still lacks new native decoder argument at this observation.
3. Test `sqlite_guest_refusal_export` at `…/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs:22`: explicit owner rejection callback, also old arity. This test callback must retain its authored rejection, not gain a fabricated provider.

`ArtifactSqliteSnapshotCodec.export` field currently takes original decoder at Store 12361; import takes original encoder owner at 12362–12364. If export now also needs independent full grant, change the actual declaration and these callback functions together; SQL limit bytes cannot manufacture the five-axis grant.

Whole repository `rg IoRunControl::decoding` found only `🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs:8`, no production call. Patch this actual unit fixture if constructor gains grant. `IoRunControl::new` and `encoding` already receive independent RetainedCloneGrant. This absence is a source census, not installed-runtime evidence.
