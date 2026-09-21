# 📓️ FL2 — child-lane replication, window transient reset, window-config command lane, batch progress

Slice FL2 of ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, 2026-09-21.
**Native laws only. No wasm32 build, no `activate`/`serve`, no browser run — wasm/live verification of all
four items is deferred to the next `s` rebuild.**

Sources read in full: `26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📓️flow.md` (§5.2, §5.3, §5.4, §6.3),
`📓️s10-s-host-studios-and-sweep.md` §4, `📓️pb3-raster-live-fem3d-probe-flow-lanes-chrome.md` §3.6.

## 1. Child-lane edits replicate across the backbone

**Root.** `ArtifactStore` is the ONLY holder of a `Backbones`, and a composed document's children are
separate `ArtifactStore`s held in `VcsArtifactApp::children` with **no transport at all**:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` `announce_history` and `flush_outbound` /
  `flush_apply_outbound` announce `self.envelope`'s events only — the PARENT lane.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` `VcsArtifactApp::attach_backbone` /
  `tick_backbone` / `detach_backbone` forward to `self.store` alone and never touch `self.children`.

So every `ChildEmit` (`dispatch_emit_group`) lands in a child store that can neither announce nor fold,
which is exactly `📓️flow.md` §5.3: the parent document converges, the `content` child does not.

**Fix — one more lane identity on the existing transport, not a second transport.**

| file:line (post-edit) | change |
|---|---|
| `🏪️store/🦀️.rs` `BackboneMessage` | new variant `Member { slot, child_id, envelopes }` appended LAST (ordinals of `Genesis`/`Mutations`/`Ack` unchanged, so the frozen wire hex in `🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone` still holds) |
| `🏪️store/🦀️.rs` `ArtifactStore::member_inbox` | new buffer: a pump inside an ordinary `dispatch` can never silently drop a member-addressed message; folded into `close_structural_owners_terminal_is_empty` and drained by `close_take_backbone_retirement` through a new `ArtifactStoreBackboneRetirement::from_queue` |
| `🏪️store/🦀️.rs` `pump_with_reports` | `Member` messages are buffered, never ingested into the parent lane |
| `🏪️store/🦀️.rs` | new `event_log_payload`, `announce_tail_edit_payload` (seeds the member's own causal dag exactly as `flush_apply_outbound` does), `ingest_remote_payload`, `send_member_mutations`, `take_member_inbound` |
| `🏪️store/🦀️.rs` `SpaceMember` | three new object-safe methods (`event_log_payload`, `announce_tail_edit_payload`, `ingest_remote_payload`), implemented for `ArtifactStore`, `NoMembers` and the `space_members!` delegation |
| `🔌️plugin/🦀️.rs` `VcsArtifactApp` | `announce_member_event_logs` (on attach), `announce_member_tail_edits` (called from `dispatch_emit_group`, the one choke point child edits land at), `fold_member_inbound` (on tick; an unknown lane is a **named fault**, never a silent drop) |

**Config lane:** deliberately NOT replicated. `config_store`/`draft_store` are authority-local state
(`ArtifactStore`'s own `merge_policy` doc says the same of merge policy); replicating a user's local
window/app config across replicas would be a different product decision. Recorded as a gap, not done.

**Laws** (both peer-authored, neither changed by me — the proof is the product change alone):
`two_instances_converge_on_disjoint_edits` (`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:638`) and
`two_instances_converge_disjoint_edits_via_backbone`
(`✏️s/🔌️plugins/🎬️sequence/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:187`, which moves a step on sequence's
`s.stdio.semio@v1/flow` CHILD and measures the child, not the parent projection).

## 2. Window transient across a same-byte `load_document_pack`

**Measured contract, not a survival bug.** `VcsArtifactApp::load_document_pack` already calls
`prepare_document_window_reset` / `commit_document_window_reset`
(`🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`), which replaces the WHOLE
`window_transient_store` with a fresh `WindowTransientOwnerRegistry` at `document_generation + 1`. Every
partition is therefore reborn at generation 0 with `Default` state.

`WindowTransientOwnerRegistry::capture` (`🫧️transient/🦀️.rs:338` → `TypedWindowTransientStoreOwner::capture`
`:197`) **materialises the partition on demand** (`self.partition(window_id)` is an
`entry().or_insert_with()`), so it answers `Some` for every REGISTERED window kind. `None` means only
"this window kind registers no transient owner at all" (that is why `🌍️gis`'s law asserts `None` and
`🗒️note`'s asserts `Some(0)`).

So flow's assertion `window_transient_generation(&generation).is_some()` could never hold — it was the
wrong predicate for a correct product, not a silent survival. Restated precisely as the product's real
semantics, and strengthened to cover the state, not just the generation.

## 3. Two back-to-back window-config commands on the same window

**Reproduced first, then rooted.** New law `flow_two_window_config_commands_in_one_turn_both_land`
dispatches `setGridVisible(false)` and `setGridFactor(20)` at the SAME window inside one turn and
settles once. Pre-fix measurement (`🗑️generated/fl2-window-config-before.txt`):

```
[DEBUG] Flow one-turn window-config lanes: [WindowConfig, Ui, Ui, Terminal, Terminal]
grid (false, 10.0) instead of (false, 20.0)
```

ONE window-config page for two commands, both operations terminating cleanly, **no Fault lane** — the
silent drop, exactly as `📓️flow.md` §5.2 reported it.

**Root, exact — TWO defects, both in `🔌️plugin/🦀️.rs`'s retained emission ladder
(`publish_mounted_typed_operation_unit`), and the second is what made it silent.** The window-config arm
took the authority captured at DISPATCH time and handed it straight to `begin`:

```
} else if let Some(mutation) = emit.window_config_mutations.pop() {
    let authority = mounted.window_config_authority.as_ref().ok_or_else(…)?;   // stale
    let publication = self.window_config_store.begin(…, authority, …)?;
```

1. **Stale base.** `WindowConfigOwnerRegistry::begin` (`🪟️window/🎚️config/🦀️.rs`) calls
   `partition.store.begin_apply_batch(operation, authority.generation, authority.revision, …)`, which is
   **exact-base**. A window's config partition is its own document, so the first command of the turn
   moves its generation/revision on and the second command's captured authority is stale by the time its
   batch begins. The window-TRANSIENT arm four lines below always re-read its authority
   (`self.window_transient_store.refresh(authority)?`); the config arm did not.
2. **A refusal that ate the mutation.** `emit.window_config_mutations.pop()` removes the mutation from
   the operation's `Emit` BEFORE `begin` runs. On a publication fault,
   `advance_typed_operation_publication_unit` grants the operation up to
   `TYPED_OPERATION_MAXIMUM_RETRIES` retries — and the retry walked the same ladder over an `Emit` that
   no longer held the mutation, fell through to "nothing to publish", and the operation completed clean.
   That is why the refusal never reached a Fault lane. Every sibling lane of that ladder already hands
   its owners back on rejection (`emit.config_mutations = mutations`, `ephemeral.transient.push(mutation)`);
   the window-config lane was the only one that did not.

**Fix (both halves, so the outcome is "applied", not "refused"):**
- refresh the captured authority before `begin`, exactly as the window-transient arm does;
- `WindowConfigOwnerRegistry::begin` now returns a new `RejectedWindowConfigEmission { mutation, fault }`
  instead of a bare `Fault`, and the ladder pushes the refused mutation back into `emit` before
  returning the fault — so a retry is a real retry and an exhausted one lands on the Fault lane **by
  name** instead of vanishing.

**What the framework owes here, stated precisely.** Both commands now publish, in dispatch order — the
same thing the NON-retained window-config route has always done (one
`WindowConfigOwnerRegistry::dispatch` per emitted mutation, in order). It does **not** merge them:
flow's `flow_direct_store_emit` (`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:724`) emits a WHOLE-record
`FlowMainWindowConfig` cloned from the config it read at DISPATCH time, so the later record necessarily
supersedes the earlier one and the turn ends at `(true, 20.0)`. Making both FIELDS survive a burst is a
flow-side change (field-level window-config mutations), recorded as a gap below — it is not something the
framework can do for an app whose mutation is a whole-state replacement.

## 4. `progress()` reports zeros between batch items

`ArtifactStoreBatchPublication::progress()` (the line the peer named, §5.4) **was already fixed in the
working tree before this slice started** — HEAD answers `stage.checkpoint() + item`, and the working tree
adds `retained_checkpoint`, refreshed in `take_stage`, plus the `close_started` early return, so gesture
progress now survives folding, publication, acknowledgement and owner retirement. Verified by reading
`git diff HEAD` on `🏪️store/🦀️.rs`; not my change, not claimed as such.

**The same defect was still live in its twin**, which is the one that actually sits at the reported line
number: `ArtifactEphemeralOneItemPublication::progress()` answered
`ArtifactStoreOneItemCheckpoint::default()` (all zeros) the moment its preparation owner was released in
`close_step`. Every window-transient, presence and transient publication polls through it.
**Fixed** with the same discipline: a `retained_checkpoint` field, written from `self.progress()` on the
turn the preparation is released, returned by `progress()` once the owner is gone.

## Laws and results

Every run below: foreground, one cargo at a time, `RUST_MIN_STACK=67108864`,
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fl2` (shared build-dir untouched).

| law | crate | before | after |
|---|---|---|---|
| `two_instances_converge_on_disjoint_edits` | `semio-s-artifact-flow-flow` | **FAILED** — `left: [("inputNote", 40, 41), ("inputSlider", 0, 0), …]` vs `right: [("inputSlider", 0, 0), ("inputSlider", 300, 301), …]` (`🗑️generated/fl2-child-lane-before.txt`, byte-identical to `📓️flow.md` §5.3) | **ok** |
| `two_instances_converge_disjoint_edits_via_backbone` | `semio-s-artifact-sequence-sequence` | **FAILED** — `left: [("step-1", 111.0), ("step-2", 280.0)]` vs `right: [("step-1", 0.0), ("step-2", 222.0)]` | **ok** |
| `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` | `semio-s-artifact-flow-flow` | FAILED on `Flow generation transient survived same-byte document reload` (`📓️flow.md` §5.2) | **ok** with the restated contract (`Some(0)` + `Default` state) |
| `flow_two_window_config_commands_in_one_turn_both_land` (**new**) | `semio-s-artifact-flow-flow` | **FAILED** — 1 window-config page, grid `(false, 10.0)` (`🗑️generated/fl2-window-config-before.txt`) | **ok** — 2 pages, grid `(true, 20.0)` |
| `an_ephemeral_one_item_publication_reports_monotone_progress_across_owner_release` (**new**) | `semio-framework-os-kernel` | **FAILED** — `ephemeral progress went backwards at close: 0 items / 0 bytes after 1 items / 1 bytes` (`🗑️generated/fl2-ephemeral-progress-before.txt`) | **ok** |

The two "before" columns for item 1 were measured by disabling ONLY the three relay call sites
(`announce_member_event_logs` / `announce_member_tail_edits` / `fold_member_inbound`) and rebuilding —
nothing else changed, so the delta is the fix and nothing else.

**Checks.** `cargo check -p semio-framework-os-kernel -p semio-framework-plugin
-p semio-framework-replication --all-targets --keep-going` → **0 errors**, 393 warnings
(`🗑️generated/fl2-check-all-targets.txt`); warnings are the proof the type-check really ran.

**Filters the brief named.** `cargo test -p semio-framework-replication --lib -- convergence` → **0 tests
match** (that crate holds no convergence law; the convergence laws live in the composed plugins, run
above). `cargo test -p semio-framework-os-kernel --lib -- publication | batch | backbone` and the full
`--lib` run: **1068 passed / 48 failed**. **None of the 48 are mine** — they are pre-existing
fleet-in-flight breakage spread across `os_directory` (schema fixture mismatch), `os_spr` (paged channel
decoders), `🎒️pack`, and `os_store` owner-authority work that is UNCOMMITTED in the tree and not mine
(`git diff --cached` on `🏪️store/🦀️.rs` carries a peer's `retained_checkpoint` work, `git diff` carries a
peer's `authority_retirement` rework). Representative messages: `a freshly constructed member store must
not carry preinstalled or terminal owner authority` (`install_document_store_owners_exact`),
`attempt to add with overflow` in `bump()`, `mutation dag reached Drop before every exact envelope …`.
I have no clean baseline to subtract, so this is stated as an observation, not as a clearance — but every
one of them is in code this slice never touched, and my own new law plus the four plugin laws are green
on the same binaries.

## Files changed
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — `BackboneMessage::Member`, `member_inbox` + its retirement, `pump_with_reports` routing, the five member-lane store methods, the three `SpaceMember` methods (+ `NoMembers` and `space_members!` delegation), `ArtifactEphemeralOneItemPublication::retained_checkpoint`/`progress`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` — the test-local `SpaceMember` delegating wrapper gains the three methods; new ephemeral-progress law
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — window-config command lane (refresh + return the refused mutation) and the member-lane relay (`announce_member_event_logs`, `announce_member_tail_edits`, `fold_member_inbound`, three call sites); `ChildMemberRegistry::entries/len/is_empty` widened to `pub(crate)`; `RejectedWindowConfigEmission` re-export
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` — `RejectedWindowConfigEmission` and the `begin` signature through the erased owner trait, the typed owner and the registry
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs` — restated transient-reload contract; new one-turn window-config law
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — 5 sites `positions(&live_host_snapshot(…))` → `positions(&*…)`. **A pre-existing compile break, not mine**: a peer moved `live_host_snapshot` to `ColdOwner<SequenceHostSnapshot>` and closure calls get no deref coercion, so the whole crate's `--lib` test target did not build. Repaired mechanically because it blocked the second composed-plugin proof.

## Gaps and honest limits
1. **Native only.** No wasm32 build, no `activate`, no serve, no browser. Every claim here is a native
   law. The guest behaviour of all four fixes is unverified and needs the next `s` rebuild.
2. **The config lane is deliberately NOT replicated.** The brief named "child/config-lane edits"; I
   replicated child lanes only. `config_store`/`draft_store` are authority-local (per-user) state, and
   pushing them across replicas is a product decision, not a bug fix.
3. **A child opened AFTER `attach_backbone`** does not get its event log announced (only genesis children
   present at attach time, plus every later child-lane EDIT, cross). `open_child`/`register_child` should
   announce the new member's log when a backbone is already attached; not done.
4. **An inbound child-lane fold does not refresh the parent's `ChildContentView`.** `fold_member_inbound`
   invalidates `self.cache`, which is what the convergence laws read through, but the immutable
   `child_content_root` a render pass observes is only rebuilt by `dispatch_emit_group`. A remote child
   edit may therefore need a local gesture before the rendered projection moves. This is the same root
   PB3 §3.6 named ("a `Child`-lane edit is invisible to the parent's edit count and to flow's rendered
   projection") and it is NOT fixed here.
5. **No child genesis verification on the wire.** The parent lane verifies `Genesis` against its own
   initial digest; a member lane trusts the `(slot, child_id)` tag and the child's causal dag. An unknown
   lane is a named fault, but a lane naming a DIFFERENT document with the same id would not be caught.
6. **Item 3 leaves a flow-side gap**: two window-config commands in one burst now both publish, but
   flow's whole-record reducer means the earlier command's field does not survive. Field-level flow
   window-config mutations (or a settle between dispatches) are the flow-side follow-up.
7. **48 pre-existing reds in `semio-framework-os-kernel --lib`** (see above) — observed, classified as
   not mine by module, but not baselined against a clean tree.
8. `semio-framework-plugin --lib` was NOT run: FP7 owns that number.
