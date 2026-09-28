# Retained Clone and Paged List Current Audit

## Scope and provenance

Read-only audit of the shared Store retained-snapshot source, cancellation/retirement flow, generic preparation seam, and newly authored unmounted `PagedList` retained-clone and `RetireOwned` primitives.

The reviewed source matched the frozen Native 8 core cut:

- `retained-clone/🦀️.rs`: `8eb94040b63d533e2e967676818990636a4cd21bcc428e1d2a2ea397182ff5e9`
- `retained-clone/🧩preparation/🦀️.rs`: `a051fc8805821b527c7cdead0b44d40d2e975a6bdc46331dac7b9cbab25744f1`
- derive expansion: `c2f2d7ff935b68b6e9db42134abfd1ef21974643d071d3793021a5d90d98b500`
- mounted native unit laws: `6683d961117cc38746a829513f0cb8873a8961e46623ed6eb0743d1d428e8a69`

The new unmounted `PagedList` files hash to `02c501cbb576960f65dc4966b49d569bd38ad4952f7b0c3e203474fad2ffe53f` and `f87ef42e1eb388385d6b7115025f24e34e2699431bfa64b6ad6b728d16fd7468`. They are a future mount candidate, not shipped Stdio runtime code.

`retained-clone-source-check-11.log` completed successfully and reported `payload=2097152`, `orderedMap=71`, and `paged=513`. This is a source/oracle result only. At audit time PID 33698 remained in the native Cargo artifact-directory lock; `retained-clone-kernel-native-8.log` records `nx run @semio-tech/framework-os-kernel:test retained_clone -- --nocapture` and has no Rust compiler or test result. No native pass is claimed.

## Confirmed lifecycle properties

`RetainedCloneSource::from_snapshot_read` first clones the read owner, then retains the `SnapshotRead` as its authority and a second Arc alias in the lease ([core:39-44](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs#L39-L44)). `RetainedCloneSource` declares `owner` before `lease` ([core:26-29](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs#L26-L29)), so normal field destruction releases the captured source alias before its lease. `RetainedCloneLeaseOwner` declares `_source_alias` before `_authority` ([core:20-24](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs#L20-L24)), so the auxiliary alias disappears before the retained `SnapshotRead` returns its lease.

The final authority drop reaches `SnapshotRead::drop`, which drops that read's Arc before calling `lease.return_now` ([Store:399-402](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs#L399-L402), [337-345](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs#L337-L345)). `try_release_aliased` releases a slot only when another Arc remains; with no external root it instead marks the exact slot returned ([Store:242-259](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs#L242-L259)). The maintenance pump then takes that exact typed Arc and transfers it into bounded owned retirement ([Store:262-298](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs#L262-L298)).

The live-source cancellation law constructs a 2 MiB `NeutralSnapshot` with no external root, drops the retained source before cursor close, requires the registry to contain the returned owner, and drains it over more than one bounded maintenance turn ([laws:307-335](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs#L307-L335)). The source ordering is correct by inspection; Native 8 must execute this strengthened law before it is credited.

The preparation close ladder first drains clone and edit cursors, then the sealer and every owned publication input, and releases its retained source after the publication authority ([preparation:371-439](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🦀️.rs#L371-L439)). The Store lifecycle fixture includes successful, cancelled, stale, and injected-fault paths ([Store laws:3389-3498](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs#L3389-L3498)).

## Findings

### P0 mount blocker — Abandoning a `PagedListCursor` synchronously drops its partial list

`PagedListRetirement` deliberately stores the list in `ManuallyDrop` and asserts terminal emptiness before it can be discarded ([paged-list:11-55](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L11-L55)). `PagedListCursor` does the opposite: its partial `values`, completed `output`, and `child_value` are ordinary fields ([paged-list:58-68](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L58-L68)) and it has no `Drop` implementation ([paged-list:76-253](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L76-L253)). A direct `drop(cursor)` therefore runs ordinary `PagedList` destruction over all populated pages and child values instead of the one-item/byte-bounded retirement ladder.

The current cancellation law always calls `begin_close` and drives `close_step` to terminal emptiness ([paged-list laws:97-124](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs#L97-L124)); it does not cover abandonment. This conflicts with the required guarantee that abandoning a partial cursor cannot free a whole `PagedList` synchronously. The code is unmounted, so this is not a shipped Stdio runtime defect; it is a P0 condition for any future mount.

Minimal repair: put every owner-bearing cursor state field behind one `ManuallyDrop` state container and add `Drop` that destroys it only after `terminal_is_empty()`. A nonterminal `Drop` cannot transfer ownership to maintenance by itself; it must fail fast while retaining that `ManuallyDrop` state, and the Store/driver must perform the ownership handoff before it permits a cursor to be lost. Add a panic-caught destructor-probe law for a multi-page partial list that proves no page or payload destructor runs before bounded close takes ownership.

### P1 shared-core defect — Parent cursors trust child-reported work and can return an over-budget step

`RetainedClone` and `RetainedCloneCursor` are public, unsealed extension contracts ([core:154-168](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs#L154-L168)). The `PagedList` parent forwards a residual grant to its child and then only `checked_add`s the returned progress ([paged-list:93-99](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L93-L99), [164-179](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L164-L179)). It similarly accepts any child close `released_items` and `released_bytes` ([123-125](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L123-L125)). `checked_add` catches integer overflow only; it does not enforce `progress.fits(remaining)`.

The generic preparation checks only the sum of copy and capacity bytes, not item work ([preparation:255-260](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🦀️.rs#L255-L260), [283-293](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🦀️.rs#L283-L293)). A defective external child or edit implementation can therefore report more item, copy, or capacity work than granted and have that value published in the Store checkpoint.

Minimal repair: centralize an `admit_progress(remaining, progress)` helper that rejects all three excess dimensions before adding them, and validate every `SnapshotRetirementStep::Pending` against its item/byte residual. Apply it at every parent-child accumulation point, including `PagedList`, vectors, boxes, tuples, generated records, and the preparation edit/clone paths. Add an intentionally nonconforming test cursor and test edit cursor; each must fail before parent state, progress, or publication advances.

### Deferred acceptance blocker — `PagedList` has no Rust compilation or runtime proof yet

The primitive is intentionally unmounted: core mounts only `ordered_map` and `preparation` ([core:13-16](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🦀️.rs#L13-L16)); no module path references `retained-clone/📋️paged-list/🦀️.rs`. This is not a shipped-runtime defect or a reason to change the deferred mounting decision. It is an acceptance blocker for the future mount. The 513-Unicode fixture and its Rust tests are presently unreachable from the crate, so neither the successful source oracle nor Native 8's `cargo test --lib retained_clone` invocation compiles or executes this code.

Before mounting any artifact root, add the module intentionally, expose its API only through the Store retained-clone surface, and execute its 513-entry copy, partial cancellation, page-by-page `RetireOwned`, exact capacity, and abandonment laws in a focused native target. Confirm that the mounted source set is newer than the Native 8 compilation start.

The raw `PagedListRetirement` cursor is nonrecursive: one `close_step` either removes one value and returns that value's child cursor, releases one empty-page allocation, or completes ([paged-list:17-38](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🦀️.rs#L17-L38)). The current 513-entry test does not exercise the production bounded scheduler, however: after `RetirementStep::Child`, it loops until that child is terminal in the same outer turn ([paged-list laws:77-85](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs#L77-L85)). `owned_retirement` instead stacks the child and charges each cursor action against `maximum_items`, while also rejecting returned bytes beyond the residual grant ([retirement:261-318](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs#L261-L318)). Replace the nested fixture loop with that actual wrapper and assert every emitted pending step. This is a future-mount proof gap, not a shipped Stdio runtime defect.

### P2 pre-mount design boundary — The generic edit contract cannot itself enforce bounded work

`RetainedCloneEditCursor::advance` receives an unrestricted `&mut P` ([preparation:26-33](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️retained-clone/🧩preparation/🦀️.rs#L26-L33)). The Store can reject a report that exceeds byte credit, but it cannot prevent an implementation from performing a whole-root scan, allocation, inverse construction, or mutation before reporting progress. The mounted test edit demonstrates that the seam permits whole `diff`/`apply` calls ([Store laws:3136-3149](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs#L3136-L3149)), albeit on a small fixture.

This does not affect a mounted Stdio artifact today because no Stdio root selects `RetainedClonePreparationFactory`; it must not be treated as proof that future Stdio edit implementations meet the interaction budget. Before the first mount, narrow the editor contract to bounded primitive operations or make the implementation's declared work envelope independently testable against a large sibling fixture, including cancel, stale, and error exits.

## Mount and validation conclusion

The absent `PagedList` mount is deliberate and should remain so until native proof is available. The source owner/lease repair and no-external-root registry law are coherent by inspection, and the existing Store close order preserves cancellation ownership. Native 8 has not validated success, cancellation, stale, fault, panic/unwind, or abort behavior; the locked invocation has produced no Rust result. The P0 candidate abandonment path, P1 parent-credit boundary, and retirement-law scheduling gap need repair before this primitive can be mounted or used as the retained clone path for a Stdio artifact.
