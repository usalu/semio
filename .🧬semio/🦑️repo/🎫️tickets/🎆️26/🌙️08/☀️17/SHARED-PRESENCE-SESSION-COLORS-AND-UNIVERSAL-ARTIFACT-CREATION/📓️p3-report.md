# P3 report — channel + plugin ABI + wrapper + UiPresence

## Summary

On starting this lane, essentially all of contract-freeze §C7.6 was **already implemented and
committed** in the live tree (git log shows the most recent commit touching my leased files at
`2026-08-18 12:22:00`, well before I started). I did a line-by-line audit of every numbered item in
the worker-brief's "Work" list and every required test against the actual code (not just names), found
the implementation matches the contract closely and correctly, fixed one real compile bug and one
documentation gap inside my lease, and flagged two out-of-lease staleness issues as
`sharedFileRequest`s. Gate commands were started but could not be confirmed complete — see "Gates" below.

## What was already done (verified, not written by me)

All verified by reading the actual code, not by trusting comments/names:

1. **Channel**: `CHANNEL_VERSION = 12` in `📡️spr/🧵️channel/🦀️component.rs` and in
   `🧫️fixtures/📡️channel/channel-version.json` / TS `APP_CHANNEL_VERSION` (component.ts:2593) — all
   three agree. `AppFrame::Ephemeral` carries the trailing `interaction: Vec<u8>` field (Rust + TS
   codec, round-trip tests, tag=13 pinned). `AppCommand::Presence { seq, own_color, peers }` exists
   end-to-end (Rust encode/decode, TS encode/decode, `AppChannelClient.pushPresence`,
   `ProgramBridge::push_presence`, `plugin_exchange`'s dispatch arm replying `AppFrame::Done`).
   **Tag is 28, not 33** — see "Contract deviation" below.
2. **Object-safe app trait**: `EphemeralSnapshot { presence, presence_generation,
   transient_generation, interaction }` and `fn adopt_presence(&mut self, own_color: Option<u8>,
   peers: &[PresencePeer], now_ms: i64) -> Result<(), Fault>` both present on `PluginApp` exactly as
   specified (`🔌️plugin/🦀️component.rs:9512-9607`).
3. **`VcsArtifactApp`**: `own_color: Option<u8>` and `peer_presence: BTreeMap<String, PeerPresence>`
   fields exist; `adopt_presence` calls `presence_store.adopt_peer` (its first ever caller) only when
   `presence_pack` is present, unconditionally upserts `peer_presence`, and drops actors absent from
   the roster from BOTH maps (`🔌️plugin/🦀️component.rs:12122-12138`).
4. **Zero app code**: `ephemeral_snapshot().interaction` builds `hover_specs`/`selection_specs` from
   `self.registry.interactions()` (i.e. `AppDefinition.interactions`), calls
   `assemble_presence_interaction`, and short-circuits to empty bytes when `domains.is_empty()`
   (`🔌️plugin/🦀️component.rs:12093-12111`).
5. **`InteractionView`**: `peers: &BTreeMap<String, PeerPresence>` field plus `peers_selecting`/
   `peers_hovering` returning `Vec<PeerMark { actor, color }>`, sorted by actor via `BTreeMap`
   iteration (`🔌️plugin/🦀️component.rs:7862-7924`).
6. **`UiPresence`**: `color: Option<u8>` + `peers: Vec<UiPeerMark>` fields exist in
   `🖱️ui/…/🎯️targets/🧊️wgpu/🦀️component.rs:96-119`; it derives `Clone` (not `Copy`);
   `UiNode::presence()`/`UiControlNode::presence()` return `&UiPresence` (lines 2419, 3949). All 4
   Copy-dependent call sites the 0-h1 inventory flagged (`Interpreter/🧊️component.rs:1747`,
   `reconcile.rs:97,199`, `paint.rs:259`) already read through the reference correctly. Re-grepped the
   whole repo for `.presence()` call sites to confirm the inventory's "4 total" is still accurate — it is.
7. **`ui_tree_stamp_presence`**: signature is `(sections, selected, previewed, own_color,
   peer_marks_for: &dyn Fn(&str) -> Vec<UiPeerMark>)`, stamps `own_color` unconditionally and
   `peers` per item (`🖱️ui/…/🧊️wgpu/🦀️component.rs:2626-2652`). `stamp_and_cache_interaction_ui`
   (`🔌️plugin/🦀️component.rs:11430-11454`) builds `marks_for` by merging `peers_selecting`/
   `peers_hovering` per item id, exactly as specified.
8. **Scene `domain_id`**: `TableScene`, `BlockListScene`, `DiffViewScene`, `EventFeedScene` all carry
   `domain_id: Option<String>` mirroring `World3dScene.domain_id`
   (`🖱️ui/…/🧊️wgpu/🦀️component.rs:3494-3500, 3799-3803, 3822-3826, 3852-3856`).
9. **TS channel plumbing**: `AppChannelCodec` region has the `Ephemeral.interaction` field and the
   `presence` command (tag 28) fully wired (encode/decode/round-trip/tag-assert tests);
   `AppChannelClient.pushPresence(ownColor, peers)` exists (`🟦️component.ts:2776`);
   `PluginRuntime/🟦️component.tsx`'s `adaptPluginHandle` exposes `pushPresence(instanceId, ownColor,
   peers)` wired to the channel (line 725) and a typed `ephemeralSnapshot?` field including
   `interaction` (lines 116-118); `ProgramBridge::push_presence(instance_id, own_color,
   &[PresencePeer])` exists and is wired the same way natively (`🧊️component.rs:213-221`).
10. **Required tests**, all present and substantively correct (not just present by name — read every
    body): `adopt_presence_fills_presence_store_and_peer_marks_and_drops_left_peers` (17471),
    `ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs` (17429),
    `interaction_view_peers_selecting_returns_actor_and_color` (17495, checks sort-by-actor AND
    color), `ui_tree_stamping_replaces_app_supplied_presence_from_interaction_state` (18606, already
    extended with the peer-mark assertion the brief asked for). All in
    `🔌️plugin/🦀️component.rs`. TS side has round-trip tests plus explicit tag assertions for both
    `presence`/`Ephemeral` in `🟦️component.ts`'s existing `describe` blocks (no dedicated
    `app-command-presence.json`/`app-frame-presence.json` shared-fixture file exists, and none was
    needed — that shared-JSON-fixture pattern is reserved for the merge/opening/transaction tag
    ranges; presence follows the same per-language pinned-hex + round-trip pattern already used for
    every other tag beyond that range).

## What I actually changed

1. **`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️reconcile.rs:452`** — real bug:
   a test helper (`tree_ui`) still called `ui_tree_stamp_presence` with the OLD 3-argument signature
   (`sections, selected, previewed`), which would not compile against the current 5-argument
   signature (`+ own_color, peer_marks_for`). Fixed to
   `ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new(), None, &|_id: &str| Vec::new())`,
   matching the identical pattern already used correctly at `🔌️plugin/🦀️component.rs:5275`. Re-grepped
   the whole repo for `ui_tree_stamp_presence(` afterward — this was the only remaining stale call site.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx:721-727`**
   — doc gap, not a bug: `ephemeralSnapshot: undefined` had no comment explaining why, unlike its
   neighbors `attachBackbone`/`detachBackbone`. Traced the reason (confirmed by reading
   `ProgramBridge/🧊️component.rs:249-250`'s identical native-side stub): channel v12's "A4-channel"
   packet retired the `exchange(id, [])` empty-command poll outright — there is no synchronous
   on-demand ephemeral-state fetch left in the ABI; `AppFrame::Ephemeral` now only arrives unsolicited,
   appended to every `exchange()` reply (`plugin_exchange`, `🔌️plugin/🦀️component.rs:16203-16208`).
   Added a comment documenting this and pointing a future implementer at the correct fix (cache the
   last-seen `Ephemeral` frame per instance rather than resurrecting the retired poll) instead of
   leaving an unexplained `undefined` that reads as an oversight. No behavior change.

Both diffs are minimal (2 lines / 7 lines). `git diff --stat` for both files: 2 files changed, 8
insertions(+), 1 deletion(-).

## Contract deviation (sharedFileRequest)

**File**: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION/📋️contract-freeze.md`
§C7.6, "New `AppCommand::Presence` … — **tag 33**".

**Actual**: tag **28**. `AppCommand`'s tags are assigned "sequentially in match-arm declaration order,
NOT the enum's own discriminant" (the file's own header comment,
`📡️spr/🧵️channel/🦀️component.rs:9-10`) — the enum currently has 29 variants (0-28), so the next free
sequential tag is 28, not 33. Padding tags 29-32 with nothing to reach 33 would violate that file's own
established convention and gain nothing. This looks like a pre-existing, deliberate correction (every
doc comment, the golden-hex tests, and the TS twin all consistently use 28 — this isn't a half-done
edit). Flagging per brief rule 2 since the contract text itself still says 33; recommend the coordinator
amend the contract text to 28 rather than requesting a renumber.

## sharedFileRequests

1. **`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts`** (1-A's exclusive lease, ts-rs
   codegen output) — **stale**: `export type UiPresence = { state, status, hover, selected }` (line
   1159), missing the `color`/`peers` fields C7.6 added to the Rust source (`🖱️ui/…/🧊️wgpu/🦀️component.rs`,
   my lease, already correct and has `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`).
   `🛂️manifest/🟦️component.ts:151`'s `export type UiPresence = GeneratedUiPresence;` (my lease's only
   exception inside that file) forwards straight through, so the manifest-crate-wide `UiPresence` TS
   type is currently missing both new fields anywhere it's used outside `UiTreeItemNode`/`UiPeerMark`
   (which ARE hand-written twins in my lease and ARE already correct). Fix: 1-A re-runs
   `bun nx run @semio-tech/framework:generate` (already required for their own C8.1 work) — this is a
   pure codegen regen, not a manual edit, so it should fall out of their existing C8.1 task for free;
   flagging so it isn't missed as "someone else's problem" on both sides.
2. **`🧰️framework/🔨️modules/🔺️mesh/🟦️component.ts`** (R-A's Wave-2 lease) — `TableScene`,
   `BlockListScene`, `DiffViewScene`, `EventFeedScene` TS twins (lines 449, 555, 563, 581) do not yet
   carry `domainId`. This is expected — R-A's lease starts in Wave 2, after this wave's audit — noting
   it here only so the coordinator's W1→W2 handoff notes it as a known, not-yet-due gap rather than a
   surprise.

## Deliberately NOT touched (confirmed correctly out of scope)

- `ProgramBridge::ephemeral_snapshot` (native, `(Vec<u8>, u64, u64)` tuple, no `interaction`) — an
  "honest stub" returning `Err(...)`, explicitly and correctly retired by the
  MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME ticket's channel-v12 "A4-channel" packet (its own doc
  comment says so). Not part of C7.6; implementing a working poll here would resurrect a design the
  other ticket deliberately removed.
- `🖱️ui/…/🧊️wgpu/🦀️component.rs`'s `NodeGraphScene.presence_peers_json` field — still present, but its
  deletion is `📌️important.md`'s checklist item "the ad-hoc `presence_peers_json` scene field" under
  contract §C8.4 (space plugin, lane 2-A, Wave 2), not §C7.6. My lease over this wgpu crate is scoped
  to the itemized list (`UiPresence`, `UiPeerMark`, scene `domain_id`, `presence()` Copy call sites),
  not the whole file, and this field's removal ripples into React `NodeGraph.tsx`/`ShellHost.tsx`/the
  space plugin, all outside my lease.
- `Shell/🧊️component.rs` — not in my lease (Wave-2 "2-C wgpu shell"). It still constructs
  `store_sync::PresencePeer { cursor: None, viewport: None, .. }` (old C7.1 fields) — a real,
  pre-existing gap, but explicitly Wave 2's file, not mine.
- `PluginBuilder`/testkit regions inside `🔌️plugin/🦀️component.rs` (1-A's) — confirmed only one
  `PluginApp` implementor exists (`VcsArtifactApp<A>`), so the trait's two new methods needed no
  additional mock implementations anywhere.

## Gates — commands run, results

Ran one at a time per rule 8, never `--workspace`.

- **`cargo test -p semio-framework-os-kernel --lib channel`**, **`cargo test -p semio-framework-plugin
  --lib`**, **`cargo check -p semio-framework-ui`**, **`cargo check -p
  semio-framework-os-renderer-wgpu`**: **BLOCKED, not run to completion.** The workspace root
  `Cargo.toml` lists member `✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust`, which does not exist on disk
  right now — `git status` shows another live session mid-move of the `draw` plugin's `🔄️fsm` tree
  (renames into `🗿️artifacts/🖍️draw/…/🔄️fsm/…`, uncommitted) without having updated `Cargo.toml`'s
  member list yet. Any `cargo` invocation, even `-p`-scoped, fails at workspace-manifest load before
  reaching package resolution: `error: failed to load manifest for workspace member
  .../🖍️draw/🔄️fsm/📦️packages/🦀️rust`. This is the documented "Concurrent Cargo Workspace Churn" hazard
  (not my bug, not my lease, not `--workspace` — a single scoped `-p` command still needs the whole
  member list to resolve). Per rule 3 I waited and retried (checked repeatedly over ~15 minutes,
  including confirming the directory was still absent immediately before writing this report) rather
  than editing the unrelated `Cargo.toml` or killing/working around anything. Log:
  `🧪️p3-cargo-check-ui.txt` (the one attempt that got far enough to error, before I started polling
  instead of re-running blind). **Coordinator/next session: re-run these four once
  `✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust` exists again (or `Cargo.toml`'s member list is
  updated) — I have no reason to expect them to fail once the workspace loads, given the exhaustive
  manual read above, but they were NOT executed and I am not claiming they pass.**
- **`bun nx run @semio-tech/framework-os:test`**: **started, did not complete within the session.**
  Backgrounded (task id shows as `by9orcts5` in this session's tooling); `ps aux` showed 70-80+
  concurrent `nx run @semio-tech/framework-os:test`/`tsc`/`vitest` processes from other lanes' sessions
  running at the same time. The log (`🧪️p3-bun-framework-os-test.txt`) shows the repo's own
  `📜️script.ts nx run …` wrapper reprinting the same "`$ bun nx run @semio-tech/framework-os:test`"
  banner dozens of times with no further progress — an nx-daemon contention retry loop, not a crash or
  a hang specific to my change, consistent with 70-80+ concurrent invocations of the same target
  fighting over the same nx daemon. Left running per rule 3 (never kill). **Not claiming this passes —
  it was not observed to complete.**

Baseline from the brief: `semio-framework-plugin --lib` = 225 passed / 5 failed (pre-existing, not
mine). I could not confirm "no sixth failure" because the command did not run to completion in this
session — this must be re-checked once the workspace unblocks.

## What is NOT done

- The two gate-command results above (blocked by external live-session state, not by my code).
- `🤖️generated/🟦️manifest.ts` regeneration (1-A's, sharedFileRequest above).
- `🔺️mesh/🟦️component.ts` scene `domainId` twins (R-A's Wave 2, sharedFileRequest above, not yet due).
- `Shell/🧊️component.rs`'s stale `cursor`/`viewport` `PresencePeer` construction (Wave-2 2-C's file,
  noted above for visibility only).

## Changed files (this session)

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️reconcile.rs` (1-line fix, real bug)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
  (comment only, no behavior change)
- `🧪️p3-cargo-check-ui.txt`, `🧪️p3-bun-framework-os-test.txt` (logs, this ticket folder)
- `📓️p3-report.md` (this file)
