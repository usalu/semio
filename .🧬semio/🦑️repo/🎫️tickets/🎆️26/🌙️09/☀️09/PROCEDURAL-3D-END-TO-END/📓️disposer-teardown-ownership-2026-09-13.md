# Disposer Teardown Ownership — Generation3d `--lib` Teardown Stall (2026-09-13)

Lane `disposer-teardown-ownership` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
`repo`/`semio` MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket lifecycle call, no `📓️status.md` / `🎫️ticket.json` edit, no
modifying git command, no dev server started or stopped, no plugin restage, nothing in
`🗑️generated` that another lane created was read-modified or deleted.

## TL;DR

The teardown family reported in `📓️fix-forward-set-contributions-hang-2026-09-13.md` §5 is **fixed
at the owning layer** — this app's own codec. `cargo test -p semio-s-artifact-procedural-generation3d
--features component-app-assembly --lib -- --test-threads=1 unit_tests::` goes from

| | passed | failed | wall |
|---|---|---|---|
| before | **45** | **29** | 247.32 s |
| after | **68** | **6** | 56.60 s |

Every one of the 23 newly-green laws is a teardown failure of this family; **no law regressed** (the
6 that remain are a strict subset of the 29 that failed before). Two new laws pin the contract and
were **red before the fix and green after**.

**Root cause.** Commit `5b6f77afcf` (`git log --date=iso` → **2026-09-13 11:27:07 +0200**) rewrote
`semio_framework_artifact_flow_flow::retained::FlowRetirement` from a `LinkedList` into a
**reserve-then-close `PagedList` frontier**: `close_step` now answers `Blocked` — never an error —
while `next_allocation_bytes()` still names a page the current owner's decomposition needs
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:543-546`). Every
driver that only ever *closed* that frontier became a silent infinite spinner. This app owned **two**
of them, both in
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`:

- `Generation3dRetainedSnapshotRetirement::close_step` — **line 698 before the fix**:
  `if !self.flow.is_empty() { return self.flow.close_step(maximum_items, maximum_bytes); }`
- `Generation3dReplayRetirement::close_step` — **line 535 before the fix**:
  `if !self.domain.is_empty() { return self.domain.close_step(maximum_items, maximum_bytes); }`

`FlowOwner::Fixture` declares 3 continuation slots (`owner_continuation_slots`), so the very first
close of a document snapshot demands a frontier page — which nobody ever paid. The `Blocked` then
travelled up unchanged: `ReturnedSnapshotReadRetirement` → `SnapshotReadReturnPump::drive`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8576`) →
`PluginCloseStep::Blocked { reason: "returned snapshot-read disposer is waiting on external
ownership" }`, and the fixture close ladder yielded and asked again with the same grant until its
deadline. The brief's suspicion of a leaked retained session / window-transient reader / pool-worker
checkout / testkit ordering defect is **refuted**: nothing external owned the value at all.

## 1. Method

1. Static localisation: followed the `8576` reason string down through
   `store::ReturnedSnapshotReadRetirement::close_step` → `owned_factory.retire_owned` →
   `Generation3dRetainedSnapshotRetirementFactory` → `Generation3dRetainedSnapshotRetirement`, which
   drives a bare `FlowRetirement::close_step`.
2. Commit evidence: `git log -S "if self.next_allocation_bytes().map_err(str::to_owned)?.is_some()"`
   on `🧵️retained/🦀️.rs` returns exactly one commit, `5b6f77afcf` 2026-09-13 11:27:07. The peer's
   already-landed fix for the *same* defect at `FlowHostRetirement::close_page`
   (`🌊️flow/🖥️host/🦀️.rs:~2395`, commit `d8dce87ca0` 2026-09-13 14:41:31, diff comment
   "⚠️ `FlowRetirement` is a RESERVE-then-CLOSE frontier") is the precedent this fix follows.
3. Wrote the two laws (§2) — **red**, with the exact panic text they were written to produce.
4. Fixed both drivers (§3) — laws **green**, suite 45/29 → 68/6.

## 2. The laws

`…/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs`, region
`🧹️FlowFrontierOwnership`. `drive_under_the_frameworks_fixed_page_grant` is the exact driver every
framework close ladder is — one item, one 4 KiB page (`GENERATION3D_OWNER_BYTES` =
`store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`), **no demand channel** — and it fails the law on
`Blocked`, because for a value this app owns outright `Blocked` is not backpressure, it is a
permanent stall.

- `every_document_retirement_pays_its_own_flow_frontier_under_the_fixed_page_grant` — drives both
  document routes (`generation3d_retire_owned_snapshot`, and the `Arc` route the store's own
  `store::SnapshotRetirementFactory` hands out) and asserts they release **byte-for-byte the same**
  backing. The second route is the oracle for the first.
- `every_displaced_replay_owner_pays_its_own_flow_frontier_under_the_fixed_page_grant` — every
  displacement `generation3d_apply_initialization_mutation` hands back for the 14 retained mutation
  fixtures, plus the replayed document afterwards.

Before the fix (run against the 17:38 binary, `🗑️generated/disposer-teardown/before-laws-direct.txt`):

```
test result: FAILED. 0 passed; 2 failed; 0 ignored; 425 filtered out
… answered Blocked under the framework's exact one-page close grant — nothing external owns this
value, so paying the Flow frontier's own reserve-then-close demand is this retirement's business
and the app close ladder spins here forever
```

After (`🗑️generated/disposer-teardown/final-laws.txt`): `test result: ok. 2 passed; 0 failed`.

## 3. The fix

One new private helper plus four call-site lines, all in
`…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`:

- `generation3d_close_flow_frontier(flow, maximum_items, maximum_bytes)` (new, line 541) pays the
  frontier's own `next_allocation_bytes()` demand with `reserve_allocation` and only closes once the
  demand is `None` — the same protocol `FlowRetirement::retire_cold` runs cold, and the same shape as
  the peer's `FlowHostRetirement::close_page` fix. It reports a paid reservation as
  `Pending { released_items: 1, released_bytes: 0 }` (progress, no release), and stays honest with
  `Blocked` only when the grant genuinely cannot cover the exact demand.
- Both drivers now call it, and — critically — gate on `terminal_is_empty()` rather than
  `is_empty()`. Those two are no longer the same predicate: after the rewrite a frontier can be
  logically empty while still holding **allocated** pages, and `FlowRetirement::drop` panics on
  exactly that, so both `close_step` guards and both `terminal_is_empty()` witnesses had to move.

Secondary (diagnosis, kept): `Generation3dAppFixture::drop` and `Generation3dViewerFixture::drop`
swallowed the close ladder's own reason (`.is_err() { break }`) and asserted only that terminal-empty
was not reached — the one thing that is never the cause. They now share
`context::close_ladder_witness`, which names the last pending authority the way the framework's own
`artifact_app_laws::close_registered_fixture_app` does. That string is what identified the single
remaining teardown failure in §5.

## 4. Runs, in the foreground of my own Bash calls

Driver: `📜️disposer-teardown-suite.sh` (ticket root). Logs under
`🗑️generated/disposer-teardown/`. `RUST_MIN_STACK = 67108864` already lives in `.cargo/config.toml`
`[env]` — a peer landed it today, so the 2 MiB libtest-stack concern from
`📓️editor-verbs-cancel-undo-2026-09-13.md` is already answered and I changed nothing there.

| Run | Command | Result |
|---|---|---|
| `before-laws-direct.txt` | 17:38 binary, `--test-threads=1 pays_its_own_flow_frontier` | **0 passed / 2 failed** |
| `before-suite-direct.txt` | 17:38 binary, `--test-threads=1 unit_tests::` | **45 passed / 29 failed**, 247.32 s |
| `after-laws.txt` | `cargo test … --lib -- --test-threads=1 pays_its_own_flow_frontier` | **2 passed / 0 failed** |
| `after-suite.txt` | `cargo test … --lib -- --test-threads=1 unit_tests::` | **68 passed / 6 failed**, 126.65 s |
| `after-isolated.txt` | each remaining failure `--exact`, alone | all 6 fail in isolation — none is an ordering artifact |
| `final-laws.txt` / `final-suite.txt` | re-run after the fixture-witness change | **2 passed / 0 failed**; **68 passed / 6 failed**, 56.60 s |
| `flowhost-laws.txt` | `cargo test -p semio-framework-os-flow --lib -- --test-threads=1 retirement` | **5 passed / 2 failed** — see §5 |

Peer compile breaks I waited out rather than authored: the viewer's
`active_example_id: String → Option<String>` refactor (4 errors, 17:46 → fixed by its lane 17:55) and
`export_document::emit`/`document_io::mesh_bridge_semio_mesh` (2 errors, 18:12 → fixed by its lane
18:13). I fixed neither and reverted no hunk of theirs.

## 5. The 6 that remain, and who owns them

All six also failed before my change, and all six fail in isolation.

1. **`refresh_pending_effects_arms_flow_eval_tick_chain`** — witness: *"last pending close authority:
   Generation3d evaluation session awaits its exact close grant"* →
   `Generation3dInstanceOperationOwner::maintenance_step`
   (`…/✳️any/✏️editor/🦀️.rs:157`) forwarding `FlowEvalSession::close_step`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:3468`), which hands the caller's 4 KiB
   grant straight to `neural::ValueRetirement::close_step`. **The same commit `5b6f77afcf`
   (2026-09-13 11:27:07)** also rewrote that retirement to be capacity-exact and blocking —
   `Owner::Bytes/Strings/Channels/Fields` now answer `Blocked` when a single indivisible backing
   exceeds the grant (`🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:86,120,139,148`), and the session never
   queries the `next_close_byte_demand()` that same commit added.
   **This is framework-owned and already has its own red laws**: `semio-framework-os-flow --lib`
   `host::session_retirement_tests::{empty_reserved_text_does_not_require_capacity_sized_credit,
   session_semantic_bytes_larger_than_production_grant_retire_exactly_across_workers}` fail with
   *"positive session grant blocked"* (5 passed / 2 failed). Those laws date from `025ec86a42`
   (2026-09-08 20:58:18) and `6ad7b0e7bc` (2026-09-10 01:31:40) — i.e. they were green and commit 617
   turned them red. I did **not** fix it: the byte contract is pinned by a JSON fixture
   (`🧫️fixtures/🧹️session-retirement/🔣️.json`) whose expected `releasedBytes` encodes a design
   decision (an empty-but-reserved `String` must not need capacity-sized credit) that is not mine to
   guess, and the file is that lane's live area.
2. **`generation_preview_is_one_app_transient_shared_by_two_generation_windows`** — *"Generation3d
   preview operation did not finish"*. Same eval-session neighbourhood, but I did **not** establish
   that it is the same defect and make no claim either way.
3. **`declared_actions_bridge_to_commands`** — *"action nodeGraphViewport failed to bridge:
   nodeGraphViewport requires viewport"*, raised by `…/✳️any/✏️editor/🦀️.rs:202`. The editor file's
   last two commits are `d8dce87ca0` (2026-09-13 14:41:31) and `5b6f77afcf` (2026-09-13 11:27:07) —
   today's action/window lanes.
4. **`two_instances_converge_disjoint_widget_moves`** — `FaultCode("module.vcs")`, *"remote snapshot
   merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate
   transaction are terminal-authorized"*, from `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
   (guard introduced `21fbcd3538`, 2026-09-02 12:19:02). Framework store lane; the testkit helper
   `paired_registered_apps` cannot attach a backbone while that guard stands.
5. **`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`** —
   *"generation3d-publication.contended"* from the `try_lock` on the process-global publication lease
   table (`…/💾️binary/🦀️.rs:325,341`). In my crate's file but a different family (publication
   authority, not the close ladder); **not investigated**, no claim about its cause.
6. **`the_editor_binds_every_keyboard_verb_the_fixture_names`** — *"escape reaches clearSelection but
   the fixture never names it"*. The fixture `🧫️fixtures/⌨️keyboard-reachability.json` was last
   written by `d8dce87ca0` (2026-09-13 14:41:31); a binding landed without its fixture row.

## 6. Same defect, other crates — not touched

The reserve-then-close regression hits every `ErasedSnapshotRetirement` that wraps a `FlowRetirement`
and only ever closes it. Outside this lane's crate I found, and did **not** change:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:636` `FlowFixtureRetirement::close_step`
- the same file, `:715` `FlowMutationRetirementFrontier::close_step_with`
- `✏️s/🔌️plugins/🌊️flow/…/✏️editor/👥️presence/♻️retirement/🦀️.rs:35`

Also note that `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🧵️retained/🦀️.rs`
(mtime 06:03) calls `FlowRetirement::push` as a `Result` and uses `next_push_allocation_bytes` /
`reserve_push_allocation`, none of which exist in the source (mtime 12:50) — that package's own tests
do not compile right now. That is a live peer refactor, untouched by me.

## 7. What is NOT claimed

- **Not fully green.** 6 of 74 `unit_tests::` laws still fail; §5 attributes each. I did not fix any
  of them, and only §5.1 has a root cause I actually established.
- **No production/browser claim.** Everything here is the native `--lib` harness. The eval-session
  stall in §5.1 would hit a real app close the same way, but I did not measure that and did not
  restage or boot the plugin.
- **No framework edit.** I changed no file under `🧰️framework/**`. The `FlowRetirement` /
  `ValueRetirement` rewrite in commit `5b6f77afcf` is described, not modified.
- **Byte-grant ceiling untested at scale.** My laws prove the document and replay routes converge
  under the fixed 4 KiB page for `Generation3dSnapshot::default()` and the 14 retained mutation
  fixtures. A single `FlowOwner` backing larger than one page would still answer `Blocked`
  (`release_backing` is all-or-nothing); I did not construct one and make no claim that no such
  document exists.
- **Ticket lifecycle untouched**, per the coordinator's instruction to this lane.

## 8. Files

Source:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
  — new `generation3d_close_flow_frontier`; both retirement drivers and both `terminal_is_empty`
  witnesses moved from `is_empty()` to `terminal_is_empty()`.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs`
  — new region `🧹️FlowFrontierOwnership`, two laws + their driver.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — `context::close_ladder_witness`, used by `Generation3dAppFixture::drop`.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs`
  — `Generation3dViewerFixture::drop` uses the same witness.

Ticket: this report, `📜️disposer-teardown-suite.sh` (ticket root), and my own logs under
`🗑️generated/disposer-teardown/` (`before-build.txt`, `before-laws.txt`, `before-laws-direct.txt`,
`before-suite.txt`, `before-suite-direct.txt`, `after-build.txt`, `after-laws.txt`,
`after-suite.txt`, `after-isolated.txt`, `final-build.txt`, `final-laws.txt`, `final-suite.txt`,
`flowhost-laws.txt`, `probe-1.txt`).
