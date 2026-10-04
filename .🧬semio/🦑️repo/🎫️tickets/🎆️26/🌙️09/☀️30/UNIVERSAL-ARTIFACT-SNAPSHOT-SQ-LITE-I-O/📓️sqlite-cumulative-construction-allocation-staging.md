# SQLite Cumulative Construction Allocation Staging

## Contract

Root authorized a separate `max_allocation_bytes` / `maxAllocationBytes`, independently defaulting to 512 MiB. `max_value_bytes` remains literal semantic cell payload (NULL0, INTEGER/REAL8, UTF-8 text and intrinsic octets); rows, columns, schema and physical file limits remain distinct. Allocation counts cumulative admitted owned backing for one transfer. Successful stage completion, cancellation, refusal and retirement do not reset or refund it. Forecasts check forthcoming capacity; actual admission is counted once. Native controls receive only the remaining parent allowance and settle actual `owned_bytes`, including failed/canceled stages.

Schema-first neutral contract is in physical SQLite `🧬️schema/🧮️allocation/🔣️.json` and `🧫️fixtures/🧮️allocation/🔣️.json`. Strict guest JSON, fixture, WIT snapshot-limits, Rust SnapshotLimits and its full host/guest constructor paths carry the new literal resource field. Actual Rust full limits constructors are Default and guest native conversion; update-form consumers inherit the new default. No external dependency was added. Physical test-only Cargo now declares existing first-party Value to exercise real NativeDecodeControl.

## Public Producer Surfaces

Rust physical `SqliteSnapshotControl::allocation_remaining_bytes()` and `admit_allocation_bytes(count)` own the persistent ledger. `allocation_stage<T,E>(phase, operation)` gives the closure remaining bytes and phase callback; closure returns `(Result<T,E>, actualOwnedBytes)`. The inner typed result is preserved, while parent admission errors are a separate outer result. Stage ownership must settle even on inner refusal. Rust behavior was a strict refusal stub until Root's actual31-law baseline; authorized checked cumulative admission and settlement are now mounted, pending native green verification.

Source public `SqliteAllocationControl` provides `remainingBytes`, `admit`, and async `stage(create, operation)`, using a real child control's `ownedBytes` in a finally settlement. The class is explicitly exported by the framework TypeScript facade. Direct admission checks caller cancellation before construction; settling already admitted child backing bypasses new cancellation so cancellation cannot refund its ownership.

## Authored Runtime Laws and Receipts

Two ledger laws exercise exact32-byte cross-stage allocations, one-byte-under remaining refusal before output construction, lifetime persistence after failed owned construction and canceled admission without consumption. Independent BunSQLite computes the admitted total; strict Ajv validates the contract. A third law uses the actual first-party NativeDecodeControl copy producer:70,000 bytes succeed;100,000 bytes cancel at65,536; the next30,001-byte real copy refuses before charge/allocation because only30,000 remain. Semantic payload ceiling0 remains independently valid for these temporary producer buffers.

Existing registered Source route: `bun nx run @semio-tech/framework-rs:test-snapshot-sqlite-source --skip-nx-cache`, explicit `SEMIO_TEST_LEVEL=quick`. Initial two-law authentic RED executed39 laws,37passed/2failed,364 assertions,2.03s Bun/23.9s Nx. After adding the real bridge, authentic RED executed40 laws,37passed/3failed,364 assertions,9.6s Nx. Every new failure was the strict missing ledger/bridge producer; no fixture or import failure. Logs: `sqlite-allocation-shared-source-strict-red.log`, `sqlite-allocation-real-bridge-source-strict-red.log`. Source implementation was repaired after these actual failures; fresh registered Source verified40/40,379 assertions,1.471s Bun/9.8s Nx in sqlite-allocation-real-bridge-source-mounted-current.log.

Existing registered Native route: `bun nx run @semio-tech/framework-rs:test-snapshot-sqlite-native --skip-nx-cache`. Actual owner router is framework Rust `📜️script.ts test-snapshot-sqlite native`; it runs standalone physical package `semio-framework-io-sqlite-snapshot --lib --no-fail-fast` then the physical oracle build. Three new laws are `sqlite_snapshot_allocation_cross_stage_frontier_is_independent_of_semantic_payload`, `sqlite_snapshot_allocation_failed_stage_cannot_refund_owned_backing`, and `sqlite_snapshot_allocation_real_native_bridge_settles_interior_failure_before_next_copy`. Root owns the sole native lane; this agent performed no Cargo invocation. Rust stays stub until Root authentic assertions.

## Remaining Concrete Integration

Adding a field and ledger does not establish complete automatic construction admission. TIFF SQL backing indexes/frontiers/final IFD/tag/value vectors need explicit guarded materialization, replacing opaque allocator-size assumptions with authored admitted storage. TIFF and HTML native producer hooks must consume the parent bridge; they currently construct native controls from their older semantic budget. Physical row/page/schema temporary storage and all other provider backing allocations also remain inventoried obligations. Do not claim universal transfer allocation coverage from ledger laws alone.

TIFF's neutral minimal construction census declares one IFD, one Long tag, four values, nine semantic entities and zero retained pixel/schema bytes. Registered Source thirteen-law corpus is GREEN13/13,1,683 assertions,30.3s Nx; log `tiff6-construction-census-source-current.log`. The independent SQLite census validates those entities and exact semantic byte admission separately from container storage. This is not provider backing-budget proof.

Changed shared owners: physical SQLite Rust/TS roots, physical allocation schema/fixture and Rust/TS unit facets; physical Cargo test manifest; framework TS facade; Plugin strict SQLite wire schema/fixture/Rust, WIT record, host constructor and guest glue constructor. No AGENTS, Git/worktree or new executable command was changed.


## Measured Shared Producers and Authorized Rust Repair

Current registered Source verification is **40/40 GREEN, zero failures,379 assertions**,1.471s Bun/9.8s Nx, log `sqlite-allocation-real-bridge-source-mounted-current.log`. All prior payload/wire/schema laws ran with the new literal allocation option.

Root then measured actual standalone physical Rust baseline: Nextest `4861d2d1-470d-4f0f-91fe-602d87702580`,31executed/28passed/3failed,0outside,168ms assertions/14.5s Nx. Each of the three new allocation laws failed the strict admission/bridge stub, while all twenty-eight original laws passed. Root repaired only fixture NativeDecodeProgress namespaces and an enclosing Result alias before this authentic receipt. Log `root-authentic-sqlite-shared-allocation3-current-typed-fixture-prerequisites.log`.

After authorization the Rust ledger now commits checked cumulative backing admission without reset/refund. The bridge checkpoints before the child, supplies only remaining allocation, settles actual child owned bytes on both success and typed refusal/cancellation, and returns the inner typed result without prose reclassification. No real TIFF/HTML producer is yet switched to it: owner repeated-control laws must first establish their own genuine reset RED. Root owns the shared31-law green verification; this lane performed no Cargo.

## Root Owning Native Allocation Baseline

Actual uncached @semio-tech/framework-rs:test-snapshot-sqlite-native reached all31 physical SQLite owner tests after correcting only test imports and the local Result alias. Nextest4861d2d1-470d-4f0f-91fe-602d87702580:31run,28passed,3failed,zero outside,168ms; Nx14.5s. New persistent-ledger success/failure and actual NativeDecodeControl bridge laws fail precisely unimplemented admission/bridge stubs. Original28 pass. No compiler refusal was counted as a feature failure. Root authorized Rust bridge behavior after this real receipt.

## Real Owner Integration Staging

Actual public TIFF and HTML producer ledger laws are now staged in their existing Native cohorts, with shared literal closed allocationBridge fixture contract. TIFF also has an exact semantic-byte reconstruction law requiring container backing admission. This distinguishes a functioning common ledger from real owner integration. Existing freshNative constructor occurrences still use max_value_bytes and reset their own ledgers; no universal construction-budget completion is claimed. Rust fleet owns Store snapshot-capability helper integration/tests separately. Source current shared control40/379 GREEN and Root original Native31 authentic stubRED remain distinct receipts.


Root has now actually verified the current shared physical Native32/32,0outside: Nextest06d51e39-5e6a-445f-87f8-387a478da196,173ms assertions/16.5s Nx. It includes all three allocation-ledger/actual-child settlement laws and the new typed-refusal law. This replaces the prior compiler-blocked shared reverify status only; it does not prove unmounted full-transfer dispatcher/physical consumers or every artifact owner.
