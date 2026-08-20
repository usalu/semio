# 📓️ terra — packet `dispatch-group-split` — report

## 🎯 Outcome: **`cargo check -p semio-framework-plugin --lib` → EXIT 0.**

The last 5 (then 2, after fixing a ripple bug — see below) errors blocking `semio-framework-plugin`
are cleared. The guest SDK compiles. The fleet fan-out is now measured for the first time in this
program (see §5 below) — large numbers there are the expected payoff, not a regression.

## What changed

### `🏪️store/🦀️component.rs` — the ruled split, applied verbatim

Per the packet's binding ruling, `CompositionCoordinator`/`TransactionCoordinator`'s composite-dispatch
surface now takes **two** type parameters instead of one:

- `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>(parent: &mut Mp, children: &mut
  [(&mut Mc, ChildDispatch)], …) -> Result<GroupReceipt<Mc>, VcsError>` (was `<M: SpaceMember +
  MemberFactory>` for both) — line ~8319.
- `dispatch_peer_group<Mp, Mc>` — same split, line ~8336.
- `dispatch_relation_group<Mp, Mc>` (the shared two-phase engine both delegate to) — same split, line
  ~8399. Internal uses of the old single `M` updated: `created_children: Vec<(ArtifactRef, Mc)>`,
  `Mc::create(...)` for genesis (only children are ever constructed — the rationale the packet gave for
  why `MemberFactory` never belonged on the parent).
- `compensate<Mp, Mc>` (the rollback helper `dispatch_relation_group` calls on a late failure) — same
  split, line ~8296. All five internal `Self::compensate(...)` call sites are inference-driven (no
  turbofish), so no further edits needed there.

**Also split `undo_group`/`redo_group`** (the packet's second call site, ~11699 in the plugin crate,
needed this too — see below). These previously took ONE homogeneous
`members: &mut [(&ArtifactRef, &mut M)]` list mixing parent and children, which is exactly the same
unification failure `dispatch_group` had. Per the packet's own fallback instruction ("if separating
cleanly means the parent travels as its own argument rather than as list element 0, do that"):

```rust
pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &ArtifactRef, parent: &mut Mp, children: &mut [(&ArtifactRef, &mut Mc)], group_id: &str) -> GroupUndoReport
pub async fn redo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &ArtifactRef, parent: &mut Mp, children: &mut [(&ArtifactRef, &mut Mc)], group_id: &str) -> GroupUndoReport
```

Both doc comments already said each member is checked against `group_id` **independently** —
list/argument order never affected correctness, only the returned report's ordering — so the split
is behavior-preserving. Factored a private `undo_one<M: SpaceMember>` helper (parent pass + children
pass now share one tail-group-check/undo/skip code path instead of duplicating it). `redo_group` keeps
children-first-then-parent internally to match `dispatch_group`'s apply order, `undo_group` keeps
parent-first-then-children, exactly as the pre-split doc comments described.

**Test fallout in `🏪️store`'s own `#[cfg(test)] mod tests`** (all homogeneous-type call sites — the
local test-fixture `ArtifactStore` wrapper plays both `Mp` and `Mc`, which the new bounds allow since
`Mp` only needs `SpaceMember`):

- 3 `undo_group`/`redo_group` test call sites updated from one `members` array to separate
  `parent`/`children` arguments (`undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest`,
  `redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest`,
  `undo_group_reverses_both_members_of_a_real_peer_transaction`).
- 1 new type-inference fix: `dispatch_group_mints_genesis_child_ids_deterministically_across_replicas`
  used an untyped empty array literal (`let mut children_1 = [];`) for the zero-live-children case.
  Under the old single-`M` signature, `Mc` inference piggybacked on `parent`'s concrete type; under the
  split it can't, since `Mc` only appears in the (empty) `children` argument. Gave it an explicit
  element type (`[(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0]`) — caught by
  `--all-targets`, not `--lib` (rule 26: run both).
- 4 stale doc comments corrected (`dispatch_group_phase1_rejects_under_normal_…`,
  `…accepts_the_same_error_scenario_under_laissez_faire`, `…rejects_under_vigilant_…`,
  `group_receipt_messages_contains_the_union_…`) — they explained why the test used the local wrapper
  type by citing "`dispatch_group<M: SpaceMember + MemberFactory>` requires the SAME `M` for `parent`
  and every `children` entry", which the split falsifies. Reworded to say `MemberFactory` is now only
  required on `Mc`.

No other `dispatch_group`/`dispatch_peer_group`/`compensate` test call sites needed changes — they're
all homogeneous-type and compile under the relaxed bounds via ordinary inference.

### `🔌️plugin/🦀️component.rs` — both call sites, plus one ripple bug the split exposed

- **Call site 1** (`dispatch_emit_group`, ~11525): the call itself (`self.composition.dispatch_group(
  &parent_ref, &mut self.store, &mut dispatches, …)`) needed **no code change** — `self.store`
  (`ArtifactStore<A::Snapshot, A::Mutation>`) unifies as `Mp`, `dispatches` (`Vec<(&mut M, ChildDispatch)>`,
  this composition's child-kind type) unifies as `Mc`, both by ordinary inference. Replaced the 13-line
  `🚧️ BLOCKED` doc comment (which had correctly diagnosed the exact fix needed) with a short `RESOLVED`
  note.
- **Call site 2** (`dispatch_group_history_action`, ~11681): this one DID need a real edit. The old code
  built one `members: Vec<(&ArtifactRef, &mut M)>` by pushing `(&parent_ref, &mut self.store)` alongside
  every live child — which never actually type-checked (`self.store`'s type and `M` don't unify), hence
  the `🚧️ BLOCKED`. Rewrote to build only a `children: Vec<(&ArtifactRef, &mut M)>` (children only) and
  pass `&parent_ref, &mut self.store` as separate arguments to the new `undo_group`/`redo_group`
  signature. This also deleted the old `if action == "undo" { … } else { … }` ordering dance entirely —
  no longer needed since `undo_group`/`redo_group` now order parent-vs-children internally.
- **Ripple bug, same function, immediately downstream of the fix**: once the type error above stopped
  masking it, two more errors surfaced (E0382 "use of moved value: `result`") at the two `return`
  points of `dispatch_group_history_action`:
  ```rust
  let mut result = Self::empty_result(…);      // binds the FUTURE, not awaited
  result.await.diagnostics = diagnostics;       // awaits once, mutates a temporary (discarded)
  return Ok(result.await);                      // awaits AGAIN — E0382, already moved
  ```
  This is a pre-existing bug (not something I introduced — it was simply unreachable by the type
  checker before, since the compiler had already given up on `members` two lines earlier), and it's
  inside the exact function this packet owns fixing. Corrected both sites to await once into `result`,
  mutate the resolved value, then return it un-awaited-again:
  ```rust
  let mut result = Self::empty_result(…).await;
  result.diagnostics = diagnostics;
  return Ok(result);
  ```

## Acceptance — every command run in the foreground, this turn, `CARGO_TARGET_DIR=` the session
## scratchpad (`…/scratchpad/target-dgsplit`), full output saved to `.txt` files in this folder

1. **`cargo check -p semio-framework-plugin --lib` → EXIT 0.** (`terra-dgsplit-plugin-lib-check.txt`)
   115 warnings, no errors. **This is the packet's headline deliverable.**
2. 🚨 `cargo check -p semio-framework-os-kernel --lib` → **EXIT 0**, 57 warnings, all
   `async_fn_in_trait` (R7-sanctioned) — unchanged from the stated baseline
   (`terra-dgsplit-oskernel-lib.txt`).
   `cargo check -p semio-framework-os-kernel --all-targets` → **EXIT 0** after the one test-file fix
   above (`children_1`/`children_2` explicit typing) — first run was EXIT 101 on exactly that one
   E0283, fixed, re-ran clean (`terra-dgsplit-oskernel-alltargets.txt`).
   `cargo test -p semio-framework-os-kernel --lib` → **EXIT 0, `test result: ok. 779 passed; 0 failed;
   0 ignored`** — the exact required count, unchanged (`terra-dgsplit-oskernel-test.txt`).
3. `cargo check -p semio-framework --lib` → **EXIT 0**, 27 warnings, unchanged from baseline
   (`terra-dgsplit-framework-lib.txt`).
4. `cargo test -p semio-framework-plugin --lib` → **EXIT 101, cannot compile — 1373 errors, all in
   `#[cfg(test)]` code, NONE in code this packet touched.** This is the SAME pre-existing residue
   already recorded in `📌️important.md`'s rule 27 (`sdk-final` finding, same day): "`semio-framework-
   plugin --all-targets` surfaces a separate 1,381-error residue, almost entirely `#[cfg(test)]` …
   two `__semio_dispatch_PluginApp` ambiguous-import errors that look macro-related, not
   await-insertion residue … **Needs its own dedicated packet**". Confirmed by direct comparison: the
   errors I hit (`unresolved import crate::app::__semio_dispatch_PluginApp`,
   `__semio_dispatch_PluginApp is ambiguous`, `cannot find type HybridLogicalTimestamp in module
   $crate::os_store`) are word-for-word the ones that report already named, at unrelated files/lines
   (`🏗️builder/🦀️component.rs:945`, `🦀️component.rs:15089/15091/18361` — none of them
   `dispatch_group`/`undo_group`/`redo_group`). The 5-known-failures-by-name baseline from the packet
   brief cannot be re-measured until that separate residue is cleared — **not a regression from this
   packet, and not fixed by it either; out of this packet's scope per the ticket's own rule 25 (atomic
   packets, not partially eaten).** Full log: `terra-dgsplit-plugin-test.txt`.
5. **The payoff — first fleet compile of the program:**
   `cargo check -p semio-s-plugin-stdio --lib` → EXIT 101, **44102 errors** (`terra-dgsplit-stdio-lib.txt`).
   `cargo check -p semio-s-plugin-note --lib` → EXIT 101, **44152 errors** (`terra-dgsplit-note-lib.txt`).
   Both now reach `semio-framework-plugin` and compile against it (they did not before — they aborted
   earlier on the unrelated `semio-framework-number`/`semio-framework-3d` crates per the ticket's
   cross-packet findings). The large counts are exactly the fleet fan-out the acceptance criteria said
   to expect ("large numbers are fine and expected — they scope the fleet fan-out") — this is the first
   time in the program these two crates have compiled far enough to measure that number at all.

## Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` —
  `compensate`/`dispatch_group`/`dispatch_peer_group`/`dispatch_relation_group`/`undo_group`/
  `redo_group` signatures (`Mp`/`Mc` split) + their internal uses; 4 stale test doc comments; 3
  `undo_group`/`redo_group` test call sites; 1 empty-array type-inference fix in a genesis test.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` —
  `dispatch_emit_group`'s `dispatch_group` call site (comment only, code already correct once the store
  side split); `dispatch_group_history_action`'s `undo_group`/`redo_group` call site (real rewrite,
  parent/children split, dropped the dead ordering branches) + a pre-existing double-`.await` bug in
  the same function's two return points (E0382, unmasked by the type-error fix, corrected).
- Ticket folder scratch: `terra-dgsplit-plugin-lib-check.txt`, `terra-dgsplit-oskernel-lib.txt`,
  `terra-dgsplit-oskernel-alltargets.txt`, `terra-dgsplit-oskernel-test.txt`,
  `terra-dgsplit-framework-lib.txt`, `terra-dgsplit-plugin-test.txt`, `terra-dgsplit-stdio-lib.txt`,
  `terra-dgsplit-note-lib.txt` — all raw command output, no `.log`.

## Not touched

`🗣️dsl/**`, `💡️inference/**` (peer-active, per the packet's own exclusion). No git-modifying commands
run. No `ticket_close`/`ticket_reopen` called.
