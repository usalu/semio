# CW1 — Composition children made live end-to-end

Status: **landed and verified**. `semio-framework-plugin` 153/153 (was 150/150 + 3 new laws),
`semio-framework-os-kernel --lib` 843/1 (the 1 is the pre-existing `fixture_sweep`
`all_non_stdio_grammars_reject_each_others_shipped_fixtures`, unrelated — it fails finding 0 usable
grammar+fixture pairs while sibling sessions restructure plugin `📚️examples`).

## What was actually broken

The composition machinery existed as types but the central promise — "children with their own
version history" — was unreachable. Two real defects, both found by writing a test that asserted the
documented behaviour rather than by reading the code.

### 1. The durable `.spr` form persisted none of a child's composition facts

`HistoryLog` carried no `owner`, no `dialect`, and no `composition_pins`; all three were dropped to
`None`/`Vec::new()` on parse with comments deferring them to "a later wave". Consequences:

- A reloaded child forgot it was owned, so ownership was **not** queryable from the child side —
  the exact property `OwnerRef`'s own doc comment says it exists to provide.
- A reloaded child forgot its dialect, so a parent could not type it.
- Checkpoint pins evaporated on reload, making the cascade unpersistable.

Fix: a new NON-CRITICAL extension record `REC_COMPOSITION = 0x41` (neighbour of `REC_CURSOR` in the
caller-defined `0x40..=0x7E` range) carrying the document's whole composition overlay — owner triple,
dialect triple, and per-checkpoint pins — dictionary-coded like every other identifier. Chosen over
adding fields to `REC_DOC`/`REC_CHECKPOINT` because those are format-frozen critical records, and a
reader that does not understand composition must be able to skip the overlay and still read a valid
document. A document with no composition writes no record at all, so leaf documents gain zero bytes.

Files: `📡️spr/📜️history/🦀️component.rs` (`HistoryComposition`, `encode_composition`/
`decode_composition`, encode+decode passes), `🏪️store/🦀️component.rs`
(`history_composition_from_envelope` / `apply_history_composition`).

### 2. `DerivedArtifactComposer::reads()` shared ONE memo across every artifact kind

```rust
fn reads() -> &'static [Dialect] {
    static UNION: OnceLock<Vec<Dialect>> = OnceLock::new();   // ← NOT per-Spec
```

A `static` declared inside a **generic function is not monomorphized per type parameter** — every
instantiation shares the same storage. The doc comment claimed "memoized per-`Spec` monomorphization",
which is false. Whichever artifact kind called `reads()` first in the process won and handed its
answer to every other kind forever: a composing artifact would silently report a leaf's empty reads,
or vice versa, purely by call order. This surfaced as two order-dependent test failures.

Fix: a `TypeId`-keyed table with the per-kind slice leaked once (the `&'static [Dialect]` return type
admits no alternative; the count is bounded by the number of artifact kinds). Verified stable across
3 consecutive full runs.

## What was built

| Item | Where |
|---|---|
| `TypedChildStoreFactory<P, M>` + `register_typed_child_store_factory` — the one production factory; one line per composable kind | `🏪️store/🦀️component.rs` `🔖️Composition` |
| `SpaceMember::{document_pack_bytes, envelope_pack_bytes, pack_at_checkpoint}` — the object-safe READ surface that was missing (nothing could get a member's content out without downcasting to types a parent cannot know) | `🏪️store/🦀️component.rs` `🔖️Space` |
| `MemberDirectory` + `MemberLinkResolver` — the first production `LinkResolver`; `Head` → live tip, `Checkpoint` → historical content via `pack_at_checkpoint`, `Snapshot` → blob or `PinnedOnly` | `🏪️store/🦀️component.rs` `🔖️Composition` |
| `AppCommand::{LoadChildren, ReadChildren}` + `AppFrame::Children` + `ChildPackEntry`, `CHANNEL_VERSION` 5→6 | `📡️spr/🧵️channel/🦀️component.rs` |
| `PluginApp::{load_child_pack, child_packs}` + guest exchange-loop handlers | `🔌️plugin/🦀️component.rs` |
| `ChildContentView` (`pack`/`typed`/`dialect`/`slots`) on `ArtifactView.children` | `🔌️plugin/🦀️component.rs` |
| Checkpoint cascade: `commit_children_for_checkpoint` (leaves-first) → `stamp_checkpoint_composition_pins` → `cascade_checkout_to_children` + `pending_child_pins` queue | `🔌️plugin/🦀️component.rs` `🔖️CheckpointCascade` |
| `ArtifactStore::set_checkpoint_composition_pins` | `🏪️store/🦀️component.rs` |

### `ArtifactView` gained a constructor

140 struct literals across 81 files were swept to `ArtifactView::new(snapshot, history)` /
`ArtifactView::with_children(..)`. Done as a constructor rather than by adding a field to every
literal so the NEXT lane addition (presence, transient) costs zero call-site churn.

## Design decisions

- **Reads through the live store, not a cache.** `ChildContentView` holds a borrow of the live child
  stores. The `thread_local!`/session `HashMap<child_id, content>` caches it replaces went stale
  whenever anything moved a child's history without going through `ArtifactApp::handle` — which
  store-level undo/redo and checkout both do. Reading through the live member cannot go stale
  *by construction*; that is strictly stronger than the fail-closed staleness checks it supersedes,
  which could only detect divergence after the fact.
- **Fail-closed `owner ⇒ dialect`.** `TypedChildStoreFactory::open` rejects an owned envelope with no
  dialect, and `create` rejects an empty genesis pack rather than substituting a default. Flipping
  `ArtifactEnvelope.dialect` to non-`Option` repo-wide (106 files) stays deferred — every composition
  seam already carries dialect explicitly, so the invariant buys what the flip would.
- **Pins sorted, child packs sorted.** Map iteration order is not stable; an unsorted persisted child
  list would make every save look like a change to anything diffing it.

## Verification actually run

```
RUSTC_WRAPPER="" cargo check -p semio-framework-os-kernel --all-targets   → 0 errors
RUSTC_WRAPPER="" cargo check -p semio-framework-plugin  --all-targets     → 0 errors
RUSTC_WRAPPER="" cargo test  -p semio-framework-plugin  --lib            → 153 passed, 0 failed (×4 runs)
RUSTC_WRAPPER="" cargo test  -p semio-framework-os-kernel --lib          → 843 passed, 1 failed (pre-existing)
```

New laws: `typed_child_store_factory_round_trips_a_child_through_create_persist_open`,
`typed_child_store_factory_rejects_empty_genesis_and_dialect_less_owned_child`,
`pack_at_checkpoint_reads_history_without_moving_the_live_cursor`,
`member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pins`,
`composition_overlay_round_trips_through_the_binary_log`,
`a_log_without_composition_writes_no_composition_record`,
`child_pack_commands_round_trip`,
`a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`,
`a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them`,
`the_child_content_view_never_goes_stale_across_undo_and_redo`.

## Next (CW2+)

- Register `TypedChildStoreFactory` for the 19 stdio `🧿️semio` subsets, invoked from each composing
  plugin's app-creation fn (the registry is per-wasm-component).
- Delete the `thread_local!` child caches in the 11 composed plugins, routing reads through
  `ArtifactView.children` (writes already go through `Emit.child_emits`).
- Host side: implement `resolve_artifact_link` behind the WIT import using `MemberLinkResolver`, and
  add `resolveArtifactLink` to the web shim (`🌐plugin-web-materialize.ts` has no such export today).
- Wire `LoadChildren`/`ReadChildren` into the host's actual persistence path.
