# Replica stale overwrite (2026-11-22 night)

## Symptom
Two source files were silently reset to an old base (about HEAD):
1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts` (~02:13)
2. `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` (~02:14)

## Evidence (store journals under `/Volumes/semio-store/v1/origins/*/segments`, every event carries `hlc`, `sha256`, `base`)
- Both writes come from one origin, **g1-m04** (Linux workstation `ueli-workstation-lin-1`, `/home/deck/semio`), in a burst of 871 `put` events between 02:12 and 02:31, after m04 published nothing since 2026-11-21 00:xx.
- `📥️load/🟦️.ts`: m04 `put` hlc `1795309982651-000076-g1-m04` (02:13:02), sha `7adf4e64…`, 47296 bytes, **base `a9f66f57…` = the content of git HEAD**. The shared content at that moment was `a08804d5…` (51747 bytes, m02 at 00:59), six fleet versions later. `diff HEAD m04-content` is three lines: m04's content is HEAD plus a small local edit, so it dropped every fleet change since HEAD (sealedOnly, closure handling, ...).
- `🧵️interaction/🦀️.rs`: m04 `put` hlc `1795310067921-000010-g1-m04` (02:14:27), base `7643b045…` = the content of the server's put of 11-21 10:54, i.e. m04's register had stopped at its last follow; the fleet content was newer.
- Fold of all journals (74 746 events): 263 puts have a `base` different from the then-current shared content; 62 of them are m04 at 02:xx (others: the 10:00 batch of 11-21 and old noise). m04's 62 stale-base puts include `Cargo.lock` files and ~60 `🦀️.rs` files across plugin/renderer/store modules, so more files than the two reported were affected; list reproducible with the script in "Reproduce".
- `origins/g1-m04/refused.json`: `"the worktree code does not start: the daemon was not ready after 900 s"`, wallMs 1795309843310 = 02:10:43. m04's daemon was down (its `ready.json` shows `disk-collecting`: 2.5 GB free, 6 GB needed) from ~18:00 on 11-21 (its register held content up to 17:30, nothing later) and came back at ~02:11; its first full walk published at 02:12:21.
- m02 (this machine) log `~/Library/Logs/semio-fleet-replica.log` has no event for either path: not a local agent and not this daemon. No `overwritten`/`stale-republish` was logged for them anywhere on m02 either: the replica did not see anything wrong, because the events were valid last-writer-wins updates.
- The only agent-side part: agents on m04 kept editing stale files while the daemon was down. That is expected; the replica must not let such edits replace newer fleet content.

## Root cause (replica design, `🔁️replica/🟦️.ts`)
1. `publishOnceV1` takes `base` from the local register (`replica.files`). After a long stop the register is hours behind the store and the first publish round (full walk) runs after a follow that is bounded (`followBudgetMilliseconds=500`, 1024 events per slice) while 14 origins produced tens of thousands of events. Paths whose newer events are not read yet are not in the queue, so they are published with a stale `base` and a fresh hlc, which wins the fold.
2. The existing safety net `staleRepublishV1` only recognises a republish of content the path already had earlier. Here the content is new (HEAD plus edit), so it is an ordinary edit.
3. At apply time `conflictWinnerV1` let any local edit with `mtime > remote hlc` win, no matter how much later. A machine that wakes up with an edit hours newer than the fleet event therefore overwrote the fleet (`kept`, `newer-than-remote`) and nothing was kept in `lost/`.

## Fix
- `conflictWinnerV1(localMtimeMs, remoteHlc, tolerance)` + `staleEditV1`: a local edit wins only when simultaneous (after the remote event, within `concurrentEditMilliseconds`, default 30 s, option `concurrentEditMilliseconds`). Later edits were written on a stale view: the fleet content stays, local bytes go to `.cache/f/lost`, event `stale-base` (en/de text, schema enum). Remote deletes keep the old rule (they take no content).
- Fence in `publishOnceV1` (`catchUpV1`): before an edit or delete of a path outside `.git/` is published, check the other heads; if the oldest unread event is older than the tolerance, ingest up to the heads (no apply) so queued paths are left to the apply, and hold all edits/deletes (`blocked: store-ahead`, retried) when the heads cannot be reached within 1 s.
- README of the module documents both rules; tests and the scenario fixture were updated to the new policy (offline edits newer than the remote by more than the window no longer win).

## Tests (bun via repo's pinned runtime)
- `🔁️replica/⚖️clash/🧪️tests/🟦️.ts`: new "the 02:13 incident" test (fails before the change: m02-style stale edit replaced `version two`), unit test for `conflictWinnerV1`/`staleEditV1`, simultaneous-edit test.
- `🔁️replica/🧪️tests/🟦️.ts`: scenarios `offline-edit-newer…` and `both-machines-offline…` now expect fleet-wins plus one file in `lost` (Python oracle still compares store and tree); new tests for the stale offline edit and the in-window edit; "a restarted daemon …" is the incident as an end-to-end run (daemon restart, stale edit, nothing published, edit in lost).
- Not covered by a test: the `store-ahead` hold (needs a backlog larger than the 1 s catch-up budget).

## Reproduce the evidence
Read all `origins/*/segments/*.jsonl`, drop `.cache`/`.git` paths, sort by `hlc`, track the current `sha256` per path and list `put`s whose `base` differs from it.

## Recovery note
The files overwritten at 02:13/02:14 can be rebuilt from the journal predecessors: the previous shared sha is in the blob store (`/Volumes/semio-store/v1/blobs/<2hex>/<sha>`), for load: `a08804d528d7…` (git index copy), already restored.

## Repair (2026-11-22 ~04:30)

Method: stale-base m04 puts (02:12-02:31) listed from journals by `stale-repair-list.py` (base != fleet head in fold, head = latest event by hlc before the put): 62 puts with a head, plus 94 puts on paths absent from the retained journals (compacted into snapshot, head unknown, base never equal to a journal head: not repaired, listed in `🗑️generated` only). Blobs: `/Volumes/semio-store/v1/blobs/<2hex>/<sha256>`. Merge: `git merge-file -p` on temp copies, `current base head` (fleet delta base->head applied onto the current file, equals `head base new` when current is m04 new); `head base new` when current is the fleet head. Written only if clean, sha re-checked right before the write. Scripts: `stale-repair-list.py`, `stale-repair-merge.py`, `stale-repair-apply.py`.

Counts: lockfile 8, already-recovered 12, conflict 26, merged 16

| path | m04 base | m04 new | fleet head | action |
|---|---|---|---|---|
| `🌎️hub/🧩️compositions/📸️remodel/🧩️component/🔣️.json` | e489df71 | b44d20ef | 8828bf34 | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🧳️receiving/🔢️binary/🦀️.rs` | fc2bcc6b | b2cf3630 | 82e52810 | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🔁️spawn-recovery/🦀️.rs` | d48f704d | 42807352 | 68514bfc | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🖌️paint-editor-and-composite/🦀️.rs` | c6aa8b86 | 2731ce50 | 2731ce50 | already-recovered (fleet head == m04 new) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs` | 84c06411 | d5024b34 | 87e4ef8a | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🪣️packet-image-and-blend/🦀️.rs` | 94d8d9a6 | 1824d8b8 | 584dbb12 | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🧪️tests/🖱️pointer-gestures/🦀️.rs` | f3a66f86 | 8f6c7ed1 | b73b8187 | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs` | dd06e3f3 | 761ddd28 | 8f6b21ed | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs` | 1cf1843b | cf707b29 | 11b8678f | already-recovered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` | 857419c9 | 5ba0d52b | e637729c | already-recovered |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts` | a9f66f5d | 7adf4e64 | a08804d5 | already-recovered (repaired by hand earlier) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` | 7643b045 | 7095211d | e62d41df | already-recovered (repaired by hand earlier) |
| `🌎️hub/🧩️compositions/📸️remodel/🧩️component/🛂️.descriptor.semio` | 55c0f50c | 7c7dcdf7 | dc4bc709 | conflict (git merge-file rc=255) |
| `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` | 9efed311 | b610f62b | 51ee10dd | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts` | f12bbe15 | 67f14734 | 5c5a416d | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` | 52f0085a | b11cdaec | 9ad147cd | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs` | 7c0a0ef3 | f323c11b | 76bf3ba8 | conflict (git merge-file rc=3) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs` | 4a9829e1 | e15fa3fb | eeedd3ad | conflict (git merge-file rc=1) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🔌️in-port-and-lane-verbs/🦀️.rs` | b7a5a505 | d0bad725 | fca726f1 | conflict (git merge-file rc=6) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️native/🦀️.rs` | b0d8d5c5 | a22ef81f | c41c78cd | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs` | cd1c38c5 | cd14095a | 8b0628f3 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🌉️program-bridge-entry/🦀️.rs` | 3c3c873a | c9c481ac | 86a97930 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🧩️session-and-topology/🦀️.rs` | ffc84910 | 0be73b5a | 7748786c | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs` | 2a764870 | 80e3bdca | effe7330 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs` | 512c98c8 | ac5b9f88 | 72ea1d5c | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/📨️dispatch-app-switch/🦀️.rs` | 5f91e90a | d37d9d3e | 6205d103 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🧾️dump-types-paths-and-walk/🦀️.rs` | 8c610b73 | a9a6c180 | b40cd2d8 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/📒️time-travel-ledger/🦀️.rs` | b6cdfbd8 | d5b3d0d7 | 9545fe15 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🗄️store-state/🦀️.rs` | 035640c8 | 993e5f5e | c6b3870d | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧱️store-owners-and-member-visitors/🦀️.rs` | f58cc6ae | 93aae4c7 | 3fe862c3 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` | 788e939c | 030fc19f | fc452c79 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs` | f58b506f | 9cdaec35 | 4e55ec58 | conflict (git merge-file rc=1) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs` | 271f0305 | 55bee4c2 | 327dc947 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` | a2de625c | f2e19029 | 1813d01a | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs` | cbba0893 | b0d01b4d | f51afa59 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📥️staging/📮️action-ingress/🦀️.rs` | d19ae47d | e7ed02c4 | b8e8eb60 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs` | 3f50f900 | 32d7e782 | e2d4f5d4 | conflict (git merge-file rc=2) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | a6ef827d | 11d857bb | 8a6264e7 | conflict (git merge-file rc=3) |
| `Cargo.lock` | 90cb9b89 | f029de97 | 371de950 | lockfile-pending (regeneration blocked) |
| `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.lock` | 4256ab42 | 42177e85 | ad633616 | lockfile-pending (regeneration blocked) (git merge-file rc=33) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock` | 30aab5b1 | c6ff6014 | 2e360373 | lockfile-pending (regeneration blocked) (git merge-file rc=1) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock` | 126d8903 | a2c2fddd | fd8a9427 | lockfile-pending (regeneration blocked) (git merge-file rc=47) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.lock` | e84145b7 | 44b00dfd | d59f3813 | lockfile-pending (regeneration blocked) |
| `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock` | 518ac198 | 4ce93671 | bc3b59d2 | lockfile-pending (regeneration blocked) (git merge-file rc=34) |
| `🌎️hub/Cargo.lock` | 0190fcd9 | 02652903 | 8e137be0 | lockfile-pending (regeneration blocked) (git merge-file rc=2) |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.lock` | ce51bc80 | a37c16a9 | e73a7dd7 | lockfile-pending (regeneration blocked) (git merge-file rc=8) |
| `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🦀️.rs` | 7696e409 | 895acbe2 | 2c776d72 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs` | ff8edf6f | b6c0bce4 | 8ac1aea4 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/📎️registry-and-node-graph/🦀️.rs` | 86e3c6d9 | b1fd7af1 | c67ac9ec | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` | 088fd7c0 | fb4019d9 | 55b40a2a | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs` | 41d394ab | 30c346db | 4b6de255 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state/🦀️.rs` | 41d5294b | 04e56b2b | f6dbb5d3 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📤️ink-event-pages-and-edit-routing/🦀️.rs` | fc0475f5 | 8d19d426 | 5eeb7550 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs` | bced146e | 8c4ea69b | 1440261f | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs` | 99da448b | 0fdcbc4b | e29dbf92 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs` | 33b3d27f | c8e2e7fb | e4b21cce | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs` | 19363169 | 5dbe979f | 2b1d1c1a | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` | 883105b8 | c47ec9b1 | 169c1d62 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs` | 4e0a4bcc | 8c033547 | fe35b9b8 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs` | 2072bf02 | e135f42d | 09e4c4b9 | merged |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs` | ba6f655d | 893f6244 | aeafbd7f | merged |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 8ec89129 | 13302b90 | 2aaf34c3 | merged |

Conflicts (real overlap: m04 edited the same hunks the fleet changed; resolve by hand from the three blobs): see the `conflict` rows. m04 base->new is often 40-150 lines (local refactors), not a small edit.

Blocked: `repo cargo` (source admission) fails for every workspace with `ENOENT lstat .../derivative/index/stamps` (another agent is mid-edit in `📥️source/🔗️derivative/🔏️index/🟦️.ts`; earlier `build/🧷️format-stamps/🟨️.mjs` was also being rewritten and was syntactically broken for a moment). The 8 Cargo.lock files are therefore not yet regenerated (planned command per workspace: `repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --manifest-path <ws>/Cargo.toml`, which updates the lock minimally instead of upgrading) and no `cargo check` ran.

## Hand resolution: job-context, plugin, store replay, editor unit, artifact-app-laws (17 paths)

Rule: fleet delta = base to head, m04 delta = base to new, plus peer edits since. Where the current tree has a design (APIs, callers, sibling files) that supersedes a side, current wins; the fleet side is kept only where its APIs exist in the tree.

| path (last two segments) | resolution |
|---|---|
| `job/⏱️context/🦀️.rs` | current + fleet delta; peer duplicate `charge_retained_item`/`charge_retirement_retained_item`/`charge_retained_item_after` and `renew_pinned_turn_fuel` dropped in favour of fleet `charge_item`/`renew_pinned_turn` (tests and fixture use those); m04 effect-work refactor kept |
| `plugin-runtime-runtime-close-budget/🟦️.ts` | fleet head + m04 two rewritten lines; m04's deletion of the cleanup-parent test not applied (source and fixture still exist) |
| `instance/🔌️plugin-app-impl/🦀️.rs` | current kept (already evolved past both sides); m04's two close-ordering hunks (command lease release returns terminal, action ingress close demand) applied; m04 stale diagnostics not applied |
| `reserved-jobs/🦀️.rs` | current kept: m04 contract design plus peer picker/natural; fleet `ReservedWirePhase`/`original_picker` design needs a supporting file that exists nowhere in the tree (unrecoverable, not in journals) |
| `stall-witness-and-unit-advance/🦀️.rs` | current (m04) kept; fleet delta was only `[DEBUG]` logging on the stall witness m04 removed |
| `in-port-and-lane-verbs/🦀️.rs` | fleet mount/finish import flow and rewrite tests, m04 other hunks; m04 `settle_kit_import` helper dropped as unused |
| `store/initialization/replay/native` | fleet head formatting and owner step; m04's three small edits merged |
| `time-travel-ledger/🦀️.rs` | current (m04) kept; fleet delta was debug diagnostics only |
| `time-travel/store-state/🦀️.rs` | current (m04) kept; fleet edited a function m04 removed (no callers) |
| `store-owners-and-member-visitors/🦀️.rs` | current (m04) kept; fleet edited functions m04 removed (no callers) |
| `fixture-dispatch-steps-and-document-loading/🦀️.rs` | fleet head (renew_pinned_turn, original_turn_ceiling) + m04 history_snapshot hunk; fleet `[DEBUG]` archive eprintln block dropped |
| `publication-retirement-authority/🦀️.rs` | fleet head (admit_clone effect API) + m04 stall-streak field rename and removal of the stall-witness test; drains-closing-store test kept |
| `mounted-run-and-pump/🦀️.rs` | current (m04) kept; fleet delta built on `note_paid_publication_progress`, which no longer exists |
| `typed-operation-unit/🦀️.rs` | m04 quote-based receiving + fleet window-config `admit_store_batch` branch (progress via `publication_progress`) and picker/natural commit hooks; fleet `note_paid_publication_progress` calls dropped |
| `retained-command/mounted/frontier/🦀️.rs` | fleet head (`blank_mounted_operation`, `job` alias) + m04 removal of `note_paid_publication_progress` and `publication_stall_streak` rename |
| `action-ingress/🦀️.rs` | current kept; fleet delta (`original_press_selected`) already redone by a peer |
| `producers/🦀️.rs` | current kept (m04 + peer picker/natural) + fleet `prepare_reserved_media_cache` before media and natural-source advance |

Known gap: the fleet `ReservedWirePhase` / picker-owner design for `reserved-jobs` is not reproducible; its supporting files were never listed in the journals.

## Repair part 2 (2026-11-22 ~06:20): compacted paths and lockfiles

Snapshot format: `/Volumes/semio-store/v1/snapshots/<origin>/<hlc>.json` = `{version, hlc, manifest: {path: {hlc, operation, sha256, bytes, executable}}, cursors}`, the fold state at `hlc` (compaction.json: segments before the snapshot are deleted). Snapshot `g1-m04/1795309823435-000000-g1-m08.json` (02:10:23, 451 012 entries) precedes m04's burst (first put 1795309941302 = 02:12:21), and no retained journal event for these paths lies between snapshot and burst.

Result for the 94 compacted paths: in all 94 the snapshot manifest entry (op `put`) has `sha256 == m04 base` (0 differ, 0 missing, 0 equal to m04 new). The fleet head at m04's put was therefore exactly the content m04 based its edit on: these were ordinary non-stale edits (the list script flagged them only because the path has no retained journal event). Nothing was lost, nothing to apply: 94 no-op, 0 merged, 0 conflicts. Total conflicts remain the 26 listed above (manual).

Lockfiles (`repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --manifest-path <ws>/Cargo.toml`): ran OK (exit 0, Cargo.lock rewritten minimally) for root, puzzle/bridge, puzzle 2d, 3d, fixture-sweep, remodeling; 5d OK on retry. hub: five attempts failed (peer edits of `📥️load`/`unit-cache` restarting source admission, then host-capacity policy `maximumPhaseMegabytes` schema mismatch, then `Undeclared original Repo build selection`); `🌎️hub/Cargo.lock` mtime 05:22 but its regeneration is unconfirmed, retry later.

`repo cargo check` of `procedural/🗿️artifacts/🧊️generation3d --workspace`: FAILED, 150 errors in crate `semio-s-artifact-procedural-3d-retained-commands` (`🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/📦️packages/🦀️rust/../../📥️provisional/🎛️input/…🦀️.rs`): `begin_granted` expects `&mut RetainedAllocationEffect`, found `RetainedCloneGrant`; work `u64` vs `u128`; missing `advance_borrowed_original`/`admit_lookup_borrowed_original`. This is an API mismatch between that crate and `semio-framework-value-retirement`/`value-clone-source`/`value-ordered` (a peer mid-refactor, not touched by the repair: none of these files are in the repair tables). Not fixed here.

## Hand resolution of the renderer-engine conflicts (2026-11-22 ~06:30)

Finding: m04's `base -> new` is NOT a pure edit. m04 worked from a checkout that lacked several fleet features, so besides its real additions its diff also contains stale deletions (native file open, image-source-page, media export, native original media, `base64`-crate revert). Rule applied: keep every m04 addition, keep every fleet change (incl. `fitGraph`), and restore every m04 deletion whose feature is still defined/used elsewhere in the tree.

| path (short) | m04 intent kept | fleet change kept | m04 stale deletions undone / notes |
|---|---|---|---|
| `frame-transaction` | `get_mut_at` world3d stepping, `log_debug` terrain fault | `AppPresenter::for_window(..)` | conflict only on presenter construction: fleet form wins (m04 `new(gpu, runtime)` no longer exists) |
| `program-bridge-entry` | `testkit` cfg widening, `pub observe_native_modification`, peer `exchange_folder_archive` added since | `native_consumes`, `import_native_file`, `import_native_pdf_source`, `deliver_original_media` | restored `declared_native_media_save` / `save_native_media`; new `testkit` cfg on the fleet `#[cfg(test)]` |
| `session-and-topology` | `suggestion_holds`, `workspace_clipboard`, `scroll_thumb_drag`, route/runtime fields | `deferred_document_load`, `held_host_effects`, `native_original_media` | restored `native_media_save` (struct still declares it) |
| `dispatch-action` | richer task-manager block (actor forward + install cancel), `bind_workspace_paste` | `fitGraph` -> `framework_fit_node_graph`, `NAVIGATE_SHELL_URI_ACTION` | restored `mod native_file_open` (`open.rs`), `setImageSourcePage`, `OPEN_ARTIFACT_FILE`, media-export block, `cancel_media_save`/`cancel_original_media`; also restored `framework_set_image_source_page` in `dispatch-action/layouts` (auto-merged row, same m04 deletion) |
| `directory-and-hub` | agents panel verbs, agents reset on close/sign-out, `pump_hub_check_in(&mut directory)` | `REPLAY_FIRST_RUN`, native-transfer progress in `advance_document_opening` | restored the `NativeFile` answer arm; agent verbs moved into `DirectoryState::run_hub_agent_action` (shell-directory `command-dispatch`) because the merged crate hit the 4000 active-line source boundary |
| `dispatch-app-switch` | `not_found` local, error/to_string tweaks | `note_shell_route` early return | semantically identical; m04 form kept |
| `dump-types-paths-and-walk` | `pub DumpNodeState`/`pub state`, mesh `position/rotation/scale`, `Tree` arm split | `graph_hover` field + extraction | none |
| `World3dHost/🟦️.tsx` | `relocateDragActive` gating, new `CatalogueDropGhost` props, removed `CatalogueDropPreviewInvalidate` | `parseTerrainBrushAction`, stroke brush, move-position, `reservePrimaryTouch`, `interactionDomainId` | old per-cell terrain brush dropped (superseded by fleet parser) |
| `remodel/component/🛂️.descriptor.semio` | - | - | binary, NOT hand-merged: still the m04 version (`7c7dcdf7`); regenerate via the `component` target once the tree compiles (its paired `🔣️.json` is already `189cc41e`, newer than both) |

Extra repair outside the subset: `🖌️canvas2d/🪣️packet-image-and-blend/🦀️.rs` ("already-recovered") still carried m04's stale `base64` crate import (breaks `semio-renderer-wgpu-scene-canvas2d`, E0432/E0433, and violates the no-external-runtime-dependency rule); restored to the fleet head (`semio_framework_io_base64`, `pub(super)` struct).

Verification: NOT achieved. `repo cargo check` of the wgpu crates repeatedly blocked by peer in-flight edits: `semio-ui-render` (`WORLD3D_POINTS_SHADER` import), source-boundary overruns (shell-view-chrome, shell-actions before the agent move), `semio-renderer-wgpu-scene-tables` (`render_widget` missing, `BlockListRole` mismatch) and `engine-attach` `registry-and-node-graph` (tuple type mismatch, line 210/232), finally a missing registry file for `mio` in the shared cargo home. React `framework-renderer-react:test` blocked by `BUILD_SOURCE_UNPARSEABLE` (`@semio-tech/framework-os/.../tutorial-video` unresolved import by a peer).
