# DIN V 18599 route: `native-return-refused: semantic`

## Update 08:20 (supersedes the verdict below)

The stale-guest diagnosis for din18599 is not sufficient: `norm-eurocode-structural`, rebuilt from the current tree (07:10, which contains the mounted-owner fix), fails en1992, en1993 and en1994 with the same `semantic` refusal, and the 05:11 build passed. So there is a second, newer regression in the current tree. Status: root cause not yet isolated; no fix made by me because another agent is editing exactly these files right now.

### What is established

- Comparing the build provenance (source sizes and hashes) of the 05:11 run `wU4qvo3` and the 07:10 run `wjo9Pp4` gives the exact changed set: 118 source files. Almost all lie in the retained-decode and retirement refactor (mechanical `effect.reborrow(..)` to `effect.nested(..)` and `native_header_release` additions at 05:30, store, `value/decode`, `job/receiving`, `job/context`, `io/failure-owner`).
- Structural's failing poll runs the full store replay (`ValidateEdit` through `BuildCandidate` and `Complete`, all printed by the `[DEBUG] phase` lines) and fails immediately after `Complete`, in the same poll. The failure is therefore in the post-replay close or retirement or outcome admission path, not in decode. The published guest prints only `OriginalTurnFailure` (type name), not its message; the newer `dbg_cause` print (reactor `call` file, 07:08/07:50) is not in any published guest yet.
- The active owner is the agent working in `🧵️job/⏱️context/🛬️owned/🦀️.rs` (receiving scope: `recv_dbg` tags, `native_scope_quantum` with `maximum_depth==0` now falls back to the retained grant, depth `min` in the scope plan) and in `🌱️value/♻️retirement/🎟️tickets/🔒️close/🦀️.rs`, `🧭️support`, `🔗️arc-custody/🔒️sealed`, plus the `[DEBUG] rel` print in `🧵️job/⏱️context/🦀️.rs` (`post_retained`, whose `job child receipt exceeds original remaining retained grant` invariant is a candidate `semantic` source). Their prints were left untouched.
- Targeted run of the replay crate on the 08:10 tree: `semio-os-store-initialization-replay --lib` 1 passed, 3 failed: `retained_empty_initialization_catalog_releases_one_exact_native_page_without_ungranted_heap_work` (unwrap_err on `Ok(Progress(0))`: a quoted release step does nothing under the grant), `native_replacement_restores_same_payload_and_preserves_original_receipts_at_every_close` and `native_supersession_scope_and_cancellation_match_json_oracle_and_original_physical_receipts` (heap requested/released `(200, 24)` and `(136, 24)` against receipt `(0, 0)`: the receiving admission step allocates without posting to the original receipt; `recv refuse tag=1`). These are the same receiving-scope accounting family as the route failure and are the cheapest red tests for the fix.

### Rebuild list once the owner's fix lands

Every guest compiled since 05:30 contains the regression: `norm-eurocode-structural` (07:10), `norm-energy` (rebuild started 07:27 on the current tree, includes the same retained-decode code, so expect din18599 to fail with the same message unless the fix lands first), and `norm-eurocode` (00:51) and `norm` (19:21) are older and unaffected only because they predate it. After the fix: rebuild `norm-eurocode-structural` and `norm-energy`, then rerun en1992, en1993, en1994 and din18599 on a private Play server (my `din18599-console-probe.mjs` plus `PLAY_TEST_OBSERVE_POLL_REFUSAL=1` works against any port).

## Verdict (earlier, din18599 only; mounted-owner part still valid but insufficient)

Stale guest. The published `norm-energy` core (2026-11-22 20:44) was compiled from a snapshot of the framework that predates the mounted-owner grant fix. The current tree already contains the fix, so a rebuild of `norm-energy` is the fix. No source change was made and no build was started.

## Where the message comes from

- `nativeRefusalError` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts` prints `original-turn.native-return-refused: <tag>` from `failure.failure.tag`.
- `semantic` is the WIT variant `actor-call-failure::semantic` (`🧬️schema/📜️reactor.wit`). `poll_wit` in `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` returns it when the guest's `dispatch_value_step` produced an `OriginalActorEntryFailure::Semantic`, that is a retained `ValueError` fault (not an authority refusal). The component world publishes no failure frame, so the cause stays in guest custody and the browser sees only the tag.

## Evidence

1. Reproduced on a private Play server (port 6047, scope `din18599`) with `PLAY_TEST_OBSERVE_POLL_REFUSAL=1`: `observed archive poll refusal {"category":"semantic","events":[],"operation":"1","epoch":"18853","spent":{all 0},"workUnitsSpent":"10", remaining grant 8,337,821 items / 2.1 GB copy / 521 MB capacity / 527 MB release}`. The turn is a plain `events: []` poll. The host stock is nowhere near exhausted, so this is not a host wallet or authority problem.
2. A bridge patch (`din18599-console-probe.mjs`, this folder) logged 9,422 successful `events: []` polls (about 2 items and 89 copy bytes each) driving the document transfer, then the 9,423rd poll fails. The failing turn spends no physical axis, which is the signature of an indivisible step that the per-step child grant cannot fund.
3. Build provenance of the 20:44 build (`.cache/b/r/m/components/runs/wRjtWve/r`): the build ran 20:04 to 20:44 on a tree that other agents kept editing ("Another process edited ... Taking a new snapshot"). `semio_plugin_mounted_owner` (wasm) was compiled at 20:08:46. Its source `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🦀️.rs` was edited at 20:12:13 and differs in size from the snapshot recorded in the build provenance.
4. That edit (committed in `508f457ff1e`) is exactly the fix for this failure class:
   - `LARGEST_INDIVISIBLE_STEP_COPY_BYTES` 33,976 to 377,696, because a store-replacement member open (377,696 B) and a member record receive (277,538 B) exceed the old per-turn copy grant. Its docstring says such a grant "leaves archive loads, undo and time travel unfundable forever".
   - New `covering_grant` plus the maintenance ladder widening (`🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`, 20:56) that raises a child grant to the covering power of two within the ceiling. Test: `covering_grant_covers_an_indivisible_demand_only_within_the_ceiling`.
   - The energy guest therefore has the old 33,976 B step grant and no ladder widening. This matches the 11-21 energy audit (`energy-authority-refusal-audit-2026-11-21.md`: a semantic refusal whose child grant was far below the demand).
5. Host contract is unchanged across builds: `norm` (19:21), `norm-eurocode` (00:51) and `norm-eurocode-structural` (05:11) all pass against the same current host. Only the build that straddled the mounted-owner edit fails. The din18599 example assets were last edited at 19:52, before the build window, and the domain crate tests pass (15/15, `din18599-cooled-office-example-2026-11-22.md`).

## Confidence

High that the guest is stale in exactly the mounted-owner grant code. The precise `ValueError` text is not observable (no failure frame in the component world, and the ownership/ledger guest exports are not present in this build), so the link between the old 33,976 B step grant and this specific refusal is inferred from the provenance, the matching docstring and the zero-spend signature, not from a captured message. The rebuild plus the strict route probe confirms it.

## Fix

Rebuild `norm-energy` from the current tree, then rerun the strict route probe for din18599. Nothing in the din18599 or energy sources needs to change. Note that the energy viewer and editor sources were edited after 20:44 (locale parameter for `TableWindowKit::render` and `scene_surface`), so the rebuild must also see the framework side of that change; if the rebuild fails to compile there, that is a separate in-flight refactor.

## Changed files

- Created: this note and `din18599-console-probe.mjs` (diagnostic Play probe, ticket-local).
- No source files changed. The temporary Play server (port 6047) was stopped and the generated logs of this investigation were deleted.
