# Plugin R6 No-State Independent Review

## Scope and Status

Read-only review of the keyed no-state fixture disposers in [Plugin component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:33722), their native fixture test, the existing presence retirement, transient-root API, and bounded retirement factory. No source was edited and no Cargo, rustc, or native test was run.

## Presence Disposer

`KeyedNoPresenceStoreDisposer` has the correct ownership shape. Its first positive-item step calls `PresenceStore::begin_retirement`, which replaces both the live local root and peer root with the explicit empty `NoPresence`/empty-peer roots. The retained `PresenceStoreRetirement` then owns the displaced roots, lease registry, and factories. Its terminal witness requires all of those to be absent; the disposer additionally requires `owner.retirement_started()` and the owner's empty peer root.

This matches [PresenceStoreRetirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:52): a returned/live reader keeps the retirement pending or blocked until the lease registry becomes terminal-empty. The current fixture test has no held-reader case, so that behavior is structurally present but not executed proof.

The zero-item path is sound: it returns zero progress before beginning retirement. The short-byte path remains a test gap. The first positive-item call can detach the roots and report `Pending { released_items: 1, released_bytes: 0 }` even if `maximum_bytes` is below the bounded factory page. That is not an over-byte release, but the next factory step correctly returns zero progress until it receives `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`. Add an explicit short-byte fixture assertion before treating the byte-grant law as covered.

## Transient Disposer — Actionable Defects

`KeyedNoTransientStoreDisposer` is not an exact owner retirement under the actual `TransientStore` contract.

`TransientStore::current_root()` returns `self.current.clone()` ([Store component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:4281)). The disposer retires that clone, but it never replaces or detaches `TransientStore.current`. Therefore the original `Arc<NoTransient>` remains owned by the live `TransientStore` after the boxed retirement completes. Its current `terminal_is_empty` ignores `owner` entirely and returns `self.0.is_none()`.

`NoTransient` being an empty unit value does not repair that ownership mismatch: the container still has a live `current: Arc<P>` and the Plugin framework's `drive_artifact_owned_disposer` treats `terminal_is_empty(owner)` as the proof required before discarding the app allocation. A canonical empty transient root may be an acceptable terminal *state*, but it must be installed as such by a container-owned detach/replace operation and included in the terminal witness; retiring a clone is only a temporary reference-count change.

There are two additional direct-contract defects:

- Before any close call, `self.0.is_none()` makes `terminal_is_empty` true.
- After `Complete`, the disposer takes its retirement and returns to `None`; a repeated direct `close_step` recreates a fresh retirement from `owner.current_root()` and reports new work. Runtime sequencing currently avoids that because `drive_artifact_owned_disposer` drops the disposer after its first `Complete`, but the disposer itself is not terminal-idempotent.

The fixture test at [Plugin component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:33820) asserts only that the disposer reaches `self.0.is_none()`. It does not retain a weak reference to the original `current_root`, assert a terminal owner condition, exercise a repeated close, or test a short-byte grant. It consequently cannot detect the clone-retirement false completion.

## Required Repair Shape

Do not treat this review as a request for a generic shared change. The concrete fixture needs a truthful transient owner handoff: either an existing `TransientStore` operation that replaces/detaches the current root into a known canonical terminal root, or a narrowly approved store-owned primitive that does so. The returned displaced `Arc` must be held by the retirement; completion must require that retirement's terminal witness and the terminal-state witness on the actual owner. The disposer also needs explicit unstarted/complete state so its terminal predicate is false before close and remains true after completion without recreating work.

The repaired native fixture should cover zero item, sub-page bytes, a held original-root weak witness, complete-only-after-retirement, and a repeated post-complete call. Presence needs a separate held-reader and short-byte test to turn its existing structural behavior into native evidence.

## Bounded Factory Evidence

`BoundedConfigRetirementFactory` itself is consistent: it owns the passed `Arc`, returns zero progress for zero items or a sub-page byte grant, drops exactly one value only with a full page, and its `Drop` asserts `terminal_is_empty`. It cannot fix the transient disposer because the factory is only handed a clone from `current_root`, not the store's owned current root.

No compile or runtime conclusion is implied by this review.
