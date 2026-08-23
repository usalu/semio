# P8 Terra Child Snapshot Ownership Audit

Date: 2026-08-22
Scope: read-only architecture audit of the store/plugin child-snapshot path. No source, manifest, lockfile, Cargo, Nx, or Git mutation was performed.

## Verdict

**RED — the bounded disposer cursor is real, but its ownership contract cannot guarantee bounded last-owner release.** Three public `Clone` capability types can create unregistered `Arc` aliases; production child construction has two direct generated-factory bypasses; and the present terminal witness proves only disposer-local emptiness, not the absence of live read/operation owners. Do not activate this path for an interactive operation until the replacement below lands.

The smallest sound replacement is one coherent packet, not a macro hook alone:

1. make snapshots **store-root-owned** rather than view-owned;
2. replace public owning reads with borrow-tied `SnapshotRead<'lease, T>` / `ErasedSnapshotRead<'lease>`;
3. give a worker one non-`Clone` `ChildContentOperationLease` per admitted immutable child root; and
4. make `space_members!` construct only a schema-bound child-store wrapper which already owns its root-retirement binding.

That leaves the existing fixed, budgeted registries as the implementation substrate, but removes their impossible task of retiring a snapshot that its live child store still owns.

## Reproduced ownership graph

```text
ArtifactStore.current Arc<P> ──┐
tail_undo_cache Arc<P> ────────┼── raw snapshot ownership aliases
resolution_candidate Arc<P> ──┤
SnapshotRead / ErasedSnapshot ┼── public Clone aliases
ChildContentEntry ────────────┤
ChildContentView Clone ───────┤
BoundedFirstStepCommandJob ───┘── retained across worker scheduling

old ChildContentView -> ChildContentRetirement -> member.retire_snapshot_read_erased()
                                                -> SemioSnapshotRetirement
                                                -> Blocked unless Arc is unique
```

The current cursor does the right thing once it owns the **unique** final `Arc`: it returns `Blocked` rather than dropping a shared root, observes exact item/byte grants, and accepts `Complete` only with `terminal_is_empty()`. The defect is earlier: neither a cloned read nor the live `ArtifactStore.current` participates in the child-root retirement registry. Consequently, that registry cannot either reclaim the root or establish a complete witness.

## Source evidence and exact call sites

### 1. Public read APIs create untracked owners

- [`store component.rs:41`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:41) derives `Clone` for `SnapshotRead<T>` and stores `Arc<T>`.
- [`store component.rs:67`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:67) does the same for `ErasedSnapshotRead`; [`:78`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:78) clones the erased `Arc` again to type it.
- [`store component.rs:5153`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:5153) exposes an owning `SnapshotRead`; [`:5162`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:5162) publicly exposes the raw `Arc` via `snapshot_root`.
- [`store component.rs:4790`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:4790) and [`:4797`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:4797) are long-lived store aliases. [`:6209`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6209) additionally copies both into a resolution candidate.
- The only non-test repository consumer of `ChildContentView::typed_read` is Flow’s duplicate-widget reducer: [line 225](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs:225) and [line 240](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs:240). The actual captured scene is synchronously consumed before its following `await`; this migration is mechanically local.

### 2. Child-content lifetime crosses async and worker boundaries

- `ArtifactView.children` is a public by-value `ChildContentView` at [`plugin component.rs:8459`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8459). The view and its constructors allow arbitrary retained values across an app async call.
- `ChildContentView` derives `Clone` at [`:8582`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8582). Each entry holds an erased owning read at [`:8551`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8551). `typed_read` clones it before typing at [`:8687`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8687).
- `ChildContentView::new` and `with_member` acquire a fresh erased `Arc` per child at [`:8667`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8667) and [`:8677`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8677). `take_one` creates one further clone during retirement at [`:8645`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8645).
- The interactive route clones the current child root into `BoundedFirstStepCommandJob` at [`:16493`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:16493), retains it as a job field at [`:13795`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:13795), then clones it again when constructing the handler view at [`:13825`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:13825). The job can sit in `AwaitWorker` before that step; this is a real cross-await/cancellation lifetime, not merely a render-local read.
- Non-job routes recreate whole child views and keep them throughout async `interaction_topology`, `render`, engagement, measures, effects, context-menu, and media calls. Representative anchors: [`:15614`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:15614) and [`:17763`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:17763).

### 3. The current registry is budgeted but witnesses the wrong boundary

- Publication pre-admits one of 64 fixed retirement slots at [`plugin component.rs:14383`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14383) and transfers the previous view at [`:14404`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14404).
- `ChildContentRetirement` preserves a rejected erased owner and checks nested `terminal_is_empty` at [`:8715`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8715) through [`:8779`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8779). Its close and maintenance paths retain the entry unless the terminal witness is true: [`:16838`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:16838) and [`:17141`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:17141).
- `SpaceMember::retire_snapshot_read_erased` selects an optional factory at [`store component.rs:7514`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7514). The matching `SemioSnapshotRetirement` correctly returns `Blocked` while `Arc::strong_count > 1` at [`stdio component.rs:548`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:548). A just-replaced view of an unchanged child necessarily shares its root with `ArtifactStore.current`, so that condition can persist without an operation leak.

Therefore the existing terminal witness means only “this one disposer has no `snapshot` or cursor,” not “the child root has no live store, view, or operation borrower.” Treating the two as equivalent would be false.

### 4. Generated construction bypass is production-reachable

- `create_member_store` and `open_member_store` return raw `ArtifactStore` with no retirement binding at [`store component.rs:647`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:647) and [`:668`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:668).
- The public `MemberFactory` contract is raw `create/open` at [`:7711`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7711); `space_members!` emits arms that call those helpers directly at [`:7951`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:7951).
- The composition coordinator’s real `ChildGenesis` path calls `Mc::create` at [`store component.rs:8985`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8985). `VcsArtifactApp::open_child` directly calls `M::open` at [`plugin component.rs:14479`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14479). Both bypass the stdio wrapper.
- stdio presently installs its 18 factories only *after* its own wrapper’s generated UFCS call at [`stdio component.rs:603`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:603), [`:666`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:666), and [`:676`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:676). Those wrappers are therefore not a universal construction boundary.

`register_child(member: M)` is a third bypass class: it accepts an already-created member at [`plugin component.rs:14517`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14517), and a blanket `SpaceMember for ArtifactStore` lets a raw store satisfy that generic contract. A macro-only hook would close the first two paths while leaving this one open.

## Smallest clean schema-first replacement

### A. One root lifecycle schema, owned by `store`

Introduce a store-local, ephemeral-only root schema; it is not serialized into a document envelope:

```rust
pub struct SnapshotRootKey {
    pub document_id: ArtifactId,
    pub generation: u64,
    pub content_revision: [u8; 32],
}

pub struct ChildSnapshotRootKey {
    pub parent: ArtifactId,
    pub slot: ChildSlotId,
    pub child: ArtifactId,
    pub root: SnapshotRootKey,
}

enum RootLeaseState { Live, CancelRequested, Returned }
```

`ArtifactStore` owns one `SnapshotRootCell<P>` for `current`, tail-undo, resolution candidate, and every published child entry. The cell, not every reader, owns the payload `Arc<P>`. Every store replacement transfers the old **store root** to its fixed retirement registry; tail-undo and resolution candidates hold root-cell references, never raw `Arc<P>`. The registry starts the typed `RetireOwned` cursor only after its exact root-cell alias count is zero. This is the only place a payload `Arc` can become final owner.

The child-content registry changes role: it cursor-releases immutable index pages and their `ChildSnapshotRootKey` references; it never calls `retire_snapshot_read_erased` for a view capture. Releasing an old parent root merely decrements each referenced child root’s tracked alias count. A child store’s own root registry later owns the payload terminal transition. This fixes the live-`current` deadlock structurally instead of treating it as external blocking.

Required terminal witness for a root generation:

```text
terminal = detached_from_current
        && parent_page_cursor_empty
        && all_child_root_refs_released
        && live_operation_lease_count == 0
        && root_cell.payload.is_none()
        && typed_disposer.terminal_is_empty()
```

The registry may remove its fixed slot only when that predicate holds. A `Blocked` result identifies the exact nonterminal root/operation key; it must not erase authority or detach the root cell.

### B. Borrowed public reads, owned non-clone operation lease

Replace the three public owning capabilities outright; no compatibility layer:

```rust
pub struct SnapshotRead<'lease, T: ?Sized> {
    value: &'lease T,
}

pub struct ErasedSnapshotRead<'lease> {
    value: &'lease (dyn Any + Send + Sync),
}

pub struct ChildContentView<'lease> {
    root: &'lease ChildContentRoot,
}

#[must_use]
pub struct ChildContentOperationLease {
    receipt: ChildRootLeaseReceipt,
    root: Arc<ChildContentRootCell>,
}
```

None derives or implements `Clone`, `Serialize`, `Deserialize`, `OpBinary`, or `OpText`. `typed_read` becomes synchronous and returns `SnapshotRead<'lease, S>`; `ErasedSnapshotRead::typed` returns the same borrow-tied type. Raw `snapshot_root` is deleted from the public API. `snapshot_owner` stays internal only until every cache adopts a root cell.

For immediate render/metadata callbacks, `ArtifactView<'lease, P>` carries `ChildContentView<'lease>` borrowed from the app’s live root. For a worker, admission consumes one capacity-reserved `ChildContentOperationLease`; `BoundedFirstStepCommandJob` stores that non-clone lease between worker turns, then lends it to `ArtifactView` only inside `step`. A handler can borrow a read across an `await` only while it also retains the lease. It cannot return that read, clone it, serialize it, or outlive the job. App-owned long-running jobs receive the same lease explicitly; the present `ArtifactOwnedToolJobRequest` intentionally receives no children and must stay that way until it accepts this governed lease.

Lease `Drop` is O(1): it atomically marks its fixed receipt `Returned` and enqueues no payload. Cancellation and normal completion both transfer/observe that receipt in the host’s bounded cleanup stage; the root-cell registry retains the cell first, so `Drop` cannot be the payload’s last owner. The cleanup stage does not scan operation maps: it removes one exact operation/root receipt and advances one registry cursor under the supplied item/byte grant.

### C. Schema-bound child construction, with no optional installer

Delete the optional `ArtifactStore.snapshot_retirement_factory`, its public installer, blanket `SpaceMember for ArtifactStore`, raw `MemberFactory::create/open`, and `retire_snapshot_read_erased`. A raw `ArtifactStore` is a parent/document store and cannot be registered as a child.

Define a `ChildMemberSchema` row as part of each `space_members!` variant. It names the closed-set match key, artifact schema, concrete snapshot root-retirement implementation, and root registry capacity. The macro emits a private bound member wrapper around `ArtifactStore` plus that declared root lifecycle. Its public factory surface returns only this bound wrapper:

```rust
space_members! {
    pub enum SemioMembers {
        Flow("flow", "stdio.semio", SemioFlowRootLifecycle) =>
            ChildArtifactStore<SemioFlowSnapshot, SemioFlowMutation>,
        // 17 sibling rows
    }
}
```

Generated `create_bound/open_bound` constructs the store, initializes the declared root lifecycle before exposing the enum, and returns `Result<Self, VcsError>`. `CompositionCoordinator::dispatch_group`, `VcsArtifactApp::open_child`, and `register_child` consume `BoundSpaceMember`; direct `ArtifactStore::new` is no longer type-compatible. This removes every installer timing race, includes all 18 stdio variants in one schema table, and makes the factory binding an invariant rather than a convention. The existing stdio `create_semio_member/open_semio_member` wrappers disappear rather than shadowing a second lifecycle.

## Exact migration sequence

1. **Store schema and root ownership** — `store/🦀️component.rs`: add root-cell/receipt state adjacent to the existing snapshot retirement contracts; redirect every assignment to `current` and `tail_undo_cache` (including `resolution_candidate`) through one `publish_root`/`retire_root` transition. Delete `snapshot_root`, owning read constructors, and optional retirement installation.
2. **Bound child member schema** — same store module: replace `SpaceMember` snapshot methods and raw `MemberFactory` with bound capture/factory traits; update `space_members!` expansion so every variant has a lifecycle declaration. Update the test-only single-member wrapper in the same module.
3. **Borrowed plugin surface** — `plugin/🦀️component.rs`: parameterize `ArtifactView`/`ChildContentView` by the read lifetime; replace `ChildContentEntry.snapshot` with a store root key/cell reference; turn `ChildContentRetirement` into index-reference release only; retain the fixed 64-slot pre-admission and its terminal checks.
4. **Operation lease bridge** — same plugin module: capture one lease at typed-operation admission; store it instead of cloned `ChildContentView` in `BoundedFirstStepCommandJob`; add exact completion/cancel transfer before a session can be removed. Do not activate the full typed route until its P8 full-operation packet applies the identical controlled lease to every async job.
5. **Consumers** — Flow duplicate-widget’s two `typed_read` calls become non-async borrowed reads. Update stdio’s macro declaration to the 18 lifecycle rows and delete its separate install helper/wrappers. Update all composition tests to construct bound members, never raw `ArtifactStore`.

There is an intentional serialization conflict: operation leases, root cells, read borrows, and terminal witnesses are local ephemeral capabilities. Only `SnapshotRootKey`/`ChildSnapshotRootKey` may travel in a continuation as invalidation evidence; they must never serialize the payload, `Arc`, receipt state, or cursor. A resumed continuation reacquires a fresh lease only if its exact key remains current; otherwise it is stale/cancelled.

## Required acceptance tests and gates

### Store and type ownership

- A compile-fail/API test proves `SnapshotRead`, `ErasedSnapshotRead`, `ChildContentView`, and `ChildContentOperationLease` are not cloneable or serializable, and that a read cannot outlive its lease.
- An async compile/runtime fixture borrows `typed_read` across an `await` while the non-clone lease remains held, then confirms release after the borrow ends. A separate compile-fail fixture tries to return that borrow after dropping/moving the lease.
- A root replacement with `current`, tail undo, resolution candidate, page reference, and one operation lease confirms the typed disposer does not start early; releasing each exact owner lets it finish under bounded grants. No `Arc::strong_count` is used as a completeness proof.
- A 64-slot saturation test rejects admission before publishing a root/operation; completion/cancellation frees only the exact slot and cannot evict another generation.

### Construction and lifecycle

- Macro expansion tests exercise every stdio row through generated `create_bound/open_bound`, `CompositionCoordinator` child genesis, `VcsArtifactApp::open_child`, and `register_child`.
- A negative fixture establishes that raw `ArtifactStore` cannot implement the bound child-member input to `register_child`, and no child member can have a missing lifecycle binding.
- Unchanged child + replacement child is the key regression: releasing the old `ChildContentRoot` must finish without trying to retire the still-live child `current` snapshot. A subsequent child mutation transfers that earlier store root exactly once into the child-store retirement cursor.
- A lying typed disposer, a missing child-root reference, and a nonterminal operation receipt each keep the registry resident and fault rather than reporting `Complete`.

### Async/cancel/runtime

- Dispatch an operation, replace its child root while it waits for a worker, then cancel before/after its first step. The exact lease receipt must return; maintenance must reclaim only after the root/index/disposer predicate is terminal.
- Exercise success, worker fault, cancellation, app close, and stale-generation rejection with 1/2/4/default workers. Assert no payload drop occurs on worker/session/lease drop and no step exceeds the item/byte budget.
- Run the existing focused plugin/stdio project targets through `bun nx` plus native/release/Wasm and browser timing gates after concurrent source settles. They are **not run by this audit**; Cargo/Nx were explicitly prohibited here.

## Relationship to existing P8 reports

`📓️p8-child-snapshot-retirement-domain-bindings-2026-08-22.md` correctly identifies the clone and generated-factory failures. This audit confirms them from the live call graph and adds the missing ownership split: a child-content view must release an index reference, not attempt to retire a payload still owned by `ArtifactStore.current`. That split, borrow-tied reads, and a non-clone operation lease are jointly required for a bounded terminal witness.

No implementation or verification command was run in this read-only scout.
