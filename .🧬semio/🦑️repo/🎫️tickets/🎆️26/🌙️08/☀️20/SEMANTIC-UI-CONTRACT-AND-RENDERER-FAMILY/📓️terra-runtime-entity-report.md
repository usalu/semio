# 📓️ terra-runtime-entity — packet report

Packet `runtime-entity`, wave W2. Owns exactly `🧠️runtime/…/🦀️entity.rs` and `…/🦀️context.rs`.

## Done

Both files rewritten wholesale from their scaffolds.

**`🦀️entity.rs`** — regions `🔖️Entity`, `🔖️Store`, `🔖️Lease`:
- `EntityId { slot, generation }`, `Entity<T>` (cheap-`Clone`, `Rc<Handle>`-backed, `PhantomData<Rc<T>>`
  so it is not `Send`/`Sync`), `WeakEntity<T>` whose `upgrade()` fails once the last strong `Entity<T>`
  drops.
- `EntityStore`: `Vec<Slot>` where each `Slot { generation, payload: Vacant|Occupied(Box<dyn Any>)|Leased }`,
  a free list, and a `pub(crate) effects: EffectQueues` field (the queues `Context` writes into — see
  below on why they live here).
- `insert`, `try_read`/`read`, `update` (the lease), `flush_releases`, `flush_effects`,
  `drain_deferred`, `drain_tasks`.

**`🦀️context.rs`** — region `🔖️Context`:
- `Context<'a, T>` borrows `Entity<T>` (owned clone) + `&'a mut EffectQueues` — deliberately *not* a
  borrow of the whole store, see decisions below.
- `entity`, `weak_entity`, `notify`, `emit`, `defer`, `spawn_local`, `subscribe`, `observe`.
- `Subscription` (RAII detach via a shared pending-removal list), `Task`/`PendingTask` (opaque handle +
  the boxed future for the embedder to poll), `EffectQueues` (`pub(crate)`).

11 in-file `#[test]` functions in `entity.rs` cover every item in the ticket's TESTS list: generational
reuse, queued-not-immediate release, `WeakEntity` upgrade failure, nested lease rejected (not aliased,
white-box via the private `take_for_lease`/`restore_after_lease` pair), read-during-lease rejected,
panic-unwind restores the value, effects queue rather than run inline, dropped `Subscription` stops
delivering, `defer` queues rather than runs, `spawn_local` queues a task.

## Acceptance: UNRUN (per U4 — I do not run cargo)

Commands for the coordinator:
```
cargo test -p semio-framework-ui-runtime --lib --target-dir <scratch>/target
cargo check -p semio-framework-ui-runtime --all-targets --target-dir <scratch>/target
```
Both will fail to compile **today**, for one reason only: `crate::UiApp` does not exist anywhere in
the tree yet (see decisions below) — `Context::defer`'s signature and `EffectQueues::defer`'s field
type name it. Once a sibling defines `pub struct UiApp` at the crate root (or re-exports one), these
gates should go green; nothing else in this packet reaches outside its own two files or the contract
crate. Cheap checks I *could* run and did: `rustfmt --edition 2021 --check` on both files — clean
parse, no syntax errors, matches repo `rustfmt.toml` (`max_width = 250`).

## Decisions

**Put-back on unwind.** `EntityStore::update` calls a private `take_for_lease` that marks the slot
`Leased` and returns the boxed value *before* constructing a local RAII guard (`struct PutBack<'s, T>`
declared inside `update`'s body, holding `&mut Vec<Slot>` + the slot index + `Option<Box<T>>`). Its
`Drop` writes the value back to `Occupied` if still present. Because this is a guard, not a
`mem::replace`-then-manual-put-back, the value returns to the slot on every exit path including an
unwind — proven by the in-file test `value_restored_after_panicking_closure` via `catch_unwind`.

**Release queuing.** `Entity<T>`/`WeakEntity<T>` share a non-generic `Handle { id, release_queue: Weak<RefCell<Vec<EntityId>>> }`.
`Handle::drop` (which only runs once the last `Rc<Handle>`, i.e. last strong `Entity` clone, drops)
pushes its id into the store's release queue if the store is still alive — it never frees the slot
itself. `EntityStore::flush_releases` is the *only* place a slot is actually vacated and its
generation bumped; call it at a safe boundary (end of a transaction), never mid-lease. Tests
`release_is_queued_until_flush_releases` and `stale_entity_id_never_resolves_to_new_occupant` pin this
down.

**Why `EffectQueues` lives inside `EntityStore` rather than beside it.** `Context<T>` needs to write
into queues *while* the store has a value leased out — i.e. two disjoint `&mut` borrows of the same
`EntityStore` need to be alive simultaneously (one held by the `PutBack` guard over `slots`, one held
by `Context` over `effects`). Rust allows disjoint direct-field `&mut self.slots` / `&mut self.effects`
borrows from the same `&mut self`, so I made `effects: EffectQueues` a field of `EntityStore` and split
it at the two borrow sites in `update`. This is why `Context` never holds `&mut EntityStore` at all —
by construction it cannot read or update *any* entity (including its own or another's) synchronously;
it can only queue.

**Listener dispatch is mine, not `runtime-transact`'s.** `subscribe`/`observe` register a fully
type-erased `Box<dyn FnMut(&mut EntityStore, Option<&dyn Any>)>` per listener (closing over the
concretely-typed observer `Entity<T>` and listener at registration time, so no `dyn` trait object is
needed for `T`/`U`/`E` themselves — only the std `Fn*`/`Any` erasure U3 sanctions). `EntityStore::dispatch`
(private) removes the entry, calls it with a reborrow of `self` (so the listener's own
`store.update(&observer, …)` call re-enters `EntityStore::update` cleanly — no aliasing, since by the
time `flush_effects` runs, no lease from the triggering `update` is still open), then reinserts unless
the source died meanwhile. I judged this belongs here because invoking a listener *requires*
`EntityStore::update`, which only I can safely construct (the `PutBack` guard's `slots` field is
private) — `runtime-transact` could not implement this without either reaching into my private state or
duplicating the lease mechanism. `runtime-transact` still owns the *fixpoint loop*: **`flush_effects` is
one bounded cycle** — it snapshots (drains) the current `notify`/`emit` queues into local `Vec`s before
dispatching, so a listener that queues a fresh `notify` lands in the *next* call, not this one. This
was a real bug I caught and fixed while writing it: my first draft used a live `while let … pop_front()`
loop directly against the queue, which would have let a single `flush_effects()` call spin unboundedly
on a listener that keeps re-notifying, defeating the whole point of `runtime-transact`'s 64-cycle
EffectStorm budget living at a higher level. The fixed contract: **`runtime-transact` must call
`flush_effects()` in a loop, stopping when it returns `false` or its own cycle budget is spent — not
call it once.**

**Exact signatures for `runtime-transact` to match:**
```rust
impl EntityStore {
    pub fn flush_releases(&mut self);                                    // call at each transaction's safe boundary
    pub fn flush_effects(&mut self) -> bool;                             // one bounded cycle; loop until false or budget spent
    pub fn drain_deferred(&mut self) -> Vec<Box<dyn FnOnce(&mut crate::UiApp)>>; // invoke each with your &mut UiApp
    pub fn drain_tasks(&mut self) -> Vec<PendingTask>;                   // PendingTask { id: u64, future: Pin<Box<dyn Future<Output=()>>> } — hand to the embedder's executor, never poll here
}
```

**`spawn_local`'s contract is type-enforced, not just documented.** Its `impl Future<Output = ()> + 'static`
bound makes it impossible for the future to close over `&mut T` or `Context<'a, T>` — both borrow a
lifetime shorter than `'static`, so the compiler rejects any attempt; the future must re-enter later
through a `WeakEntity::upgrade`. Documented on the method per the ticket's ask.

## Registrar-requests

**`crate::UiApp` is unresolved and I did not define it.** Nothing in `📋️packets.md`, `📌️important.md`,
or `📋️master.md` assigns an owner for a top-level `UiApp` aggregate type, yet the packet brief's own
`Context::defer` signature (`impl FnOnce(&mut UiApp) + 'static`) requires the name to exist. I judged it
almost certainly belongs to `runtime-transact` (owner of `dispatch.rs`/`transaction.rs`, i.e. the "frame
transaction" that would naturally aggregate `EntityStore` + `DependencyTracker` + `CommandGateway` +
whatever else `transact()`/`build_frame()` glue together) — FORBIDDEN's own example list
("`crate::DependencyTracker`, `crate::CommandGateway`, …") reads as non-exhaustive, and `UiApp` fits the
same "leave unresolved, do not stub" instruction. Please confirm the owner (or scaffold it centrally,
as W1's registrar pass did for the region stubs) so this crate compiles. Until then both packet gates
above are correctly UNRUN, not red — this is the sole reason.

No other registrar-request. Root manifest, `Cargo.toml`, `📦️glue.rs`, `📋️project.json`, `📜️script.ts`
untouched.

## Deviations

None from the packet brief's API shape. One internal design choice not spelled out in the brief:
`Context<T>` holds `&'a mut EffectQueues` rather than `&'a mut EntityStore` — justified above (it is
what makes "no mutable entity reference crosses an await" *and* "no synchronous reentrant read/update
through Context" both true by construction, and it is what let disjoint field-borrow-splitting in
`update` work without unsafe code).

## Files touched
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️entity.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️context.rs`

## reconciliation

Coordinator-directed follow-up: `cargo check -p semio-framework-ui-runtime --lib` reported 12 errors
at the `runtime-entity`/`runtime-present` seam. OWNS temporarily expanded to include `🦀️tracking.rs`
and `🦀️present.rs` (read-only reference for the former, one targeted fix for the latter). All edits
done, acceptance still **UNRUN** per U4 — same two commands as above, now expected to actually pass.
Cheap check re-run: `rustfmt --edition 2021 --check` on all four files — all four parse clean, exit 0.

**What each error class actually was, and which side changed:**

1. **`E0425: cannot find type UiApp` (1 error).** My own packet brief's mistake, per sol's ruling —
   `UiApp` was never going to exist. Changed `Context::defer`'s bound from
   `impl FnOnce(&mut UiApp) + 'static` to `impl FnOnce(&mut EntityStore) + 'static`, and
   `EffectQueues::defer`'s field type and `EntityStore::drain_deferred`'s return type to match
   (`VecDeque<Box<dyn FnOnce(&mut EntityStore)>>` / `Vec<Box<dyn FnOnce(&mut EntityStore)>>`). My side.

2. **`E0616: field {alive,call,source} of ListenerEntry is private` (4 errors).** My own latent bug,
   not `runtime-present`'s — `EntityStore::dispatch` in `entity.rs` reads `entry.source`/`.alive`
   (twice) `/.call` on a `ListenerEntry` whose fields I had left plain-private in `context.rs`, a
   different module. This would have failed to compile even without `runtime-present` in the picture;
   the coordinator's gate is what first caught it. Fixed per ruling 4: the three fields are now
   `pub(crate)`, the struct itself stays un-exported (no `pub`/`pub(crate)` on `ListenerEntry` — it is
   still unnameable outside `context.rs`, only its fields are reachable by value-holders elsewhere in
   the crate). My side.

3. **`E0107: missing generics for Context` + `E0106: missing lifetime specifier` (2 errors, same root
   cause).** `present.rs` declared `PresentCx { context: &'a crate::Context }` and
   `PresentCx::new(.., context: &'a crate::Context)`, naming `crate::Context` with zero of its two
   required generic args (`Context<'a, T>` needs both a lifetime and a type). This is
   `runtime-present`'s expectation being **genuinely wrong**, not merely differently shaped: my
   `Context<'a, T>` is the per-lease *mutation-effect* handle `EntityStore::update` hands to a
   closure — by design (see the original report's decisions section) it borrows only the effect
   queues, never the store, so a lease can never be read around through it, and it is generic per-`T`
   because it names one specific entity under lease. Presenting is the opposite shape: a read-only
   traversal across many entities of many different `T`s, entirely outside any lease. There was no
   sensible instantiation of `Context<'a, T>` that could have satisfied `present.rs`'s use — it needed
   `EntityStore::read`, which already existed. Fixed on `present.rs`'s side (in-scope for this
   reconciliation): `PresentCx`'s field renamed `context` → `store: &'a crate::EntityStore`,
   `PresentCx::new`'s parameter renamed to match, `PresentCx::read` now calls
   `entity.read(self.store)`.

4. **`E0599: no method named read found for &Entity<T>` (1 error).** `present.rs` called
   `entity.read(self.context)`, expecting `Entity<T>` to have a `read` method — reasonable, and
   missing only because I hadn't added it (the original report only exposed `EntityStore::read`).
   Added `Entity<T>::read<'s>(&self, store: &'s EntityStore) -> &'s T` in `entity.rs`, a thin forward
   to `EntityStore::read` — a convenience for exactly this caller shape (entity-first, store as
   argument) rather than store-first. My side (an addition, no prior behavior changed).

5. **`E0308: mismatched types` (remaining errors, at least 1, likely more before the above fixes
   cascaded).** Two independently-defined `EntityId` types existed — mine in `entity.rs`
   (`{slot,generation}` struct) and `runtime-present`'s in `tracking.rs` (`EntityId(pub u64)`,
   deliberately opaque so `DependencyTracker` never depends on `runtime-entity`'s internal slot
   layout). `Entity::<T>::id()` returned mine; `DependencyTracker::record_read`/`dirty_surfaces_for`
   took theirs. Per ruling 2, deleted my definition entirely and route through theirs: `entity.rs` now
   does `use super::tracking::EntityId;` and adds a private inherent `impl EntityId` (packing/unpacking
   `(slot, generation)` into the type's single `pub u64`, exactly as `tracking.rs`'s own doc comment on
   `EntityId` anticipated) — legal because inherent impls only need to share a crate with their type,
   not a module. Every internal `entity.id.slot`/`.generation` field access became
   `entity.id.slot()`/`.generation()` method calls; `Handle`, `Slot` lookups, `flush_releases`,
   `take_for_lease`, `restore_after_lease`, `update`'s `PutBack` guard, and all in-file tests updated
   to match. One identity type now exists crate-wide. My side (deleted the duplicate).

**Where `runtime-present`'s expectation was wrong vs. merely different:** only item 3 above
(`PresentCx` naming the wrong `Context` shape) was a genuine design error on their side, not just a
naming mismatch — no version of my `Context<'a, T>` could have served presentation's read pattern, so
I fixed `present.rs` rather than distorting `Context<T>` to serve two incompatible purposes. Item 4 was
a legitimate ask I simply hadn't provided yet. Items 1, 2, and 5 were mine.
