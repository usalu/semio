# Packet `runtime-reconcile` — report

## Done

Replaced the `🦀️reconcile.rs` scaffold wholesale with `SurfaceReconciler`, in three regions
(`🔖️Identity`, `🔖️Reconciler`, `🔖️Diff`) plus `🧪️Tests`:

- `SurfaceReconciler { surface, revision, allocator, retained, key_index, root }` exactly per the
  packet brief's field list.
- `reconcile(&mut self, tree: &ComponentTree) -> Option<UiPatch>` — keyed diff, `None` when nothing
  changed, otherwise one `UiPatch` with `base_revision` = current, `revision` = current + 1.
- `snapshot(&self) -> UiSnapshot` — full current state for a fresh subscriber.
- `mark_rejected(&mut self)` — resets retained state so the next `reconcile` re-sends in full.
- Every `fn` tagged `// 🚫️async: U1 …` per ruling U1; no `async fn`, no `dyn` on a first-party trait
  (U3 — this file defines none).
- 10 `#[test]` functions covering every bullet in the packet's TESTS list, including the round-trip
  property against the contract's own `apply_patch`/`validate_snapshot`.

## Acceptance: UNRUN

Per U4 I do not run cargo. Exact commands for `sol`, target dir in scratchpad, both `--lib` and
`--all-targets` (U-program rule 26), 600000 ms timeout each (U-program rule 19):

```
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --lib
CARGO_TARGET_DIR=<scratchpad>/target cargo check -p semio-framework-ui-runtime --all-targets
CARGO_TARGET_DIR=<scratchpad>/target cargo test  -p semio-framework-ui-runtime --lib
```

`semio-framework-ui-runtime`'s only dependency is `semio-framework-ui-contract`, which is itself
dependency-free (serde + `ui_styling` only — see its own `📦️glue.rs` header) and outside U5's named
failing set (`semio-framework-number`, `semio-framework-actor`). This packet's crate should therefore
be unaffected by the U-program's asyncify churn; if a gate comes back RED anyway, that is
`blocked-external` with the exact error, not a bug in this file, per U2.

## Decisions

1. **Field-group → op mapping.** `component` → `SetComponent`; `layout` → `SetLayout`;
   `activity`+`disabled` → one combined `SetActivity` (matches the op's own two-field shape);
   `children` (id list only) → `SetChildren`. **`style`/`accessibility`/`bindings`/`menu` have no
   dedicated setter in the landed `UiPatchOp` enum** (`Upsert | SetComponent | SetLayout | SetActivity
   | SetChildren | Remove | SetRoot` — seven variants, but only four are field-targeted). A change to
   any of those four therefore falls back to one full-node `Upsert`, which is still far narrower than
   the old `PatchTracker` stub's every-turn root replace, but not as narrow as it could be. Concrete
   case this costs: a pure theme/tone flip (`StyleSpec.tone`) on a leaf with unchanged component/layout
   still emits a full `Upsert` instead of a one-field op. **Feedback for a future contract revision:**
   a `SetStyle`/`SetAccessibility`/`SetBindings`/`SetMenu` (or one narrower `SetProps`-style op) would
   close this gap.
2. **Duplicate-sibling-key detection lives in this file too, not only upstream.** `present.rs`'s
   `ComponentTree::new`/`TreeNode::with_children` already assert uniqueness, but `ComponentTree`'s
   `root` field is `pub`, so `ComponentTree { root }` bypasses both constructors entirely. This
   reconciler runs its own `assert_unique_child_keys` inside `diff_children`, independent of whatever
   the caller did upstream — proven by a test that constructs a tree via the bypass and asserts the
   reconciler itself panics.
3. **Identity is exactly `(parent_id, key)`, never position** — `key_index: HashMap<(Option<UiNodeId>,
   String), UiNodeId>`. Reorder/insert/remove tests all confirm ids survive independent of where a key
   moves among its siblings.
4. **Removal purges both maps recursively** (`purge_subtree`), so `retained`/`key_index` never
   accumulate an orphan and a removed id is never handed back to the allocator (ids are per-surface
   monotonic and never reused, verified by a dedicated test).
5. **`mark_rejected` also resets `revision` to `UiRevision::default()`** (0) in addition to clearing
   `retained`/`key_index`/`root` — this mirrors the receiver being assumed back to an empty document
   after a rejection-driven resync, so the next patch's `base_revision` matches what the receiver
   actually has. The `UiNodeIdAllocator` is deliberately left untouched across this reset, so re-sent
   nodes get ids that continue monotonically rather than risking reuse of an id a stale renderer
   reference might still hold.
6. **`transition` has no source in `ComponentTree`/`TreeNode`** (see `present.rs`'s own module doc —
   builder-side, never diffs). This file is therefore the only place `UiNodeRecord::transition` is
   populated: `None` for a freshly inserted node, the previously retained value carried forward
   unchanged for an existing one. Never derived here. Driving `Introducing`/`Celebrating` from presence
   data is master.md's separate `PresenceUpdate` channel, out of this packet's scope — flagged here for
   whichever packet wires that up.
7. **`UiSnapshot::layout_epoch` is left at `0`** by `snapshot()` — no layout engine exists in this
   crate to source a real epoch from.
8. **`snapshot()` before any `reconcile` call** returns `root: UiNodeId::default()` (0) with an empty
   `nodes` table. This sentinel never resolves to a real record, and both `validate_snapshot` and
   `apply_patch`'s root walk (`if nodes.contains_key(&root_id)`) already treat a root with no matching
   record as a no-op traversal rather than a violation — no contract-side special-casing was needed.

## Registrar-requests

None — no `Cargo.toml`/`project.json`/other registrar-only file needed a change.

## Deviations

None from the packet brief. `🦀️dispatch.rs` and `🦀️transaction.rs` were read for context only, never
edited, and are referred to by path only (not defined/stubbed) — they remain packet `runtime-transact`'s
in-flight work.
