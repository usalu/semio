# Browser E2E v8: load progress and performance

## Update (scripts, finer progress, boot blocker)
- `repo run @semio-tech/framework-os-dev:activate-process3d-react-prototype` exists but did not leave the `door.script` phase in 17 min (same as `repo-tooling-recheck.md`), so it cannot be used here. Thin native scripts now live at the ticket root: `hub-build.sh` (cargo wasm build, retries while the shared tree is mid-edit), `publish-hub.ts` (transpile and publish only the hub module; extensions and shard/vendor stay as published), `rebuild-and-boot.sh` (rebuild, publish, boot check, repeat), `progress-law.sh`, `run-e2e.sh`, `process-interactions.json`, `process-load-trace.json` (banner trace for a Concrete Forest load). Activation was not re-run: the extension modules and the activation receipt survive in `.cache`.
- Finer progress signal written: the load now also adds the members opened plus the candidate views prepared (`ActiveArtifactStoreReplacement::member_progress`, `members_done`, `document_archive_load_completed`), with `total` growing by two passes per archive member when the replacement begins. Laws (green): `stage_laws` and `progress_laws` (`progress-law.sh`, 2 passed). The closure validation (`drive_closure`) exposes no counter, so that stage still moves only as one step; the member-open and candidate-view passes (the brep snapshot opens) now move per member. No file of the brep agent was touched.
- Hub rebuilt and republished with these changes twice, but the page does not boot: `Framework OS boot failed ... UnsupportedOwner: field owner has no retained source projection` (the default `DslField::projection_view` in `🗣️dsl/🧬️schema/🪆️binding`, reached from the load path). It reproduces on both rebuilds over about two hours of tree changes, so some field type in the document load path lost or never got its retained projection in the in-flight projection-quoting work. That is the brep agent's area; the stage counter, Cancel replay and the re-timing could not be run. After that agent lands: `rebuild-and-boot.sh`, then `STEPS="$(cat process-load-trace.json)" ./run-e2e.sh` (banner trace over the load) and a Cancel replay step.

## Status (before this update)
Measurement done; one progress fix written and unit-tested but NOT verified in the browser; cancellation NOT verified. The browser run was blocked for the last hours by in-flight edits of the brep agent (boot failed with `UnsupportedOwner: field owner has no retained source projection`, then a compile break in `NativeReceiveSlot`), and the ticket's `🗑️generated` folder (my hub build, the publish and activate scripts, the wasm target) was deleted by ticket housekeeping while the last hub build ran. Re-running needs the publish scripts recreated and a full hub rebuild (about 45 min).

## 1. Measured breakdown (Firefox, hub v18, machine load average 14-17 from concurrent builds, so absolute times are inflated; ratios and counts are reliable)
Method: timing around every `reactor.poll` in the generated bridge (temporary `[DEBUG]`, removed) plus BiDi console timestamps.

| Quantity | Value |
|---|---|
| Guest turns (`poll`) for the whole 280 s session (boot, Demo, Concrete Forest) | 1,015,045 |
| of which first canvas (about 55 s): turns / items / bytes copied | 70,538 / 287,193 / 45 MB |
| Concrete Forest load alone | about 700,000 turns, about 3 min |
| Wall per turn (all-in) | about 0.28 ms |
| Guest wasm time per turn | about 0.17 ms average, 0.26 ms in the steadiest window |
| Share of wall spent inside `reactor.poll` (guest wasm) | about 78 %; the other 22 % is host JS between turns (extra guest crossings `admitActorContext`/`admitActorInput` per turn, the macrotask yield, React) |
| Idle gaps between turns (sleeping/cadence) | none; the worker drives turns back to back (`driveYield` is a MessageChannel hop, about 30 µs); first sample 23 % of wall was host work, not sleeping |
| Work per turn | about 1.8 items, about 450 B copied, about 400 work units (units track bytes) |
| Cost model fitted over the windows | about 65 µs fixed per turn + about 63 µs per item |

Reading: about 1.8 items per turn and about 1 M turns: the turn count follows the item count, so the step-count cut by the brep agent shrinks both parts, because each turn carries about 1.8 items (64 % per-item, 36 % per-turn at this ratio). Cutting only the number of items (steps) per record reduces the per-item part proportionally; merging steps into fewer turns would also remove the per-turn 65 µs plus about 40 µs host overhead. The turn count is the quantity to report to the brep agent: roughly 700 k turns for Concrete Forest, about 0.17 ms in the guest and 0.28 ms wall each. A 10x step cut gives about 10x on the 3 minutes only if turns drop with it; if per-turn items stay at 1.8 it does.
Not the cause: poll cadence (no sleeping), `driveYield` (an experiment skipping the yield for 4 ms slices showed no change in the guest share, reverted), the host JS (22 %), idle time.

## 2. Progress
Finding: `Loading document: N of N` reads N of N from the first moment (125 of 125 for the initial Demo load, observed for 57 s; 127 of 127 for Concrete Forest) because `completed`/`total` count only the folded operations, and the long phase is after the fold: the store replacement (`AwaitingMembers` to `RetiringCommittedStore`), which reported nothing.
Fix (guest, `🔌️plugin`):
- `🗃️document/🧾️store-replacement-vocabulary/🦀️.rs`: `STORE_REPLACEMENT_STAGES = 8` and `ActiveArtifactStoreReplacementState::stage()` (monotonic ordinal along the success path).
- `🗃️document/🗄️document-archive-load/🦀️.rs`: the load carries `stage`; `status()` adds it to `completed`.
- `🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`: `total` grows by `STORE_REPLACEMENT_STAGES` when the replacement begins; `stage` follows the replacement state in `AwaitingMembers` and `AwaitReplacement`.
- Law (green): `stage_laws::the_success_path_walks_monotonic_stages_ending_at_the_announced_count` (`cargo test -p semio-plugin-document-media --lib stage_laws`): stages never decrease and end at the announced count, so `completed` stays monotonic and at most `total`.
- Limit: the counter now moves in 8 steps after the fold. A long stage (closure validation and candidate views, the likely 2 minutes) still shows no movement inside the stage; a finer signal needs items-done and items-total from the candidate view driver (`drive_candidate_views`, `drive_closure`), which sits next to the brep agent's area.
Not verified in the browser: needs the hub rebuild above.

## 3. Cancellation
The status bar offers `Cancel replay` while a load runs; the guest cancel command (`CancelDocumentArchiveLoad`) exists per the document-load API. I did not exercise it in this round.

## 4. Product changes kept from earlier rounds (unchanged)
Bridge Work-overrun latch fix and law; contributions-refresh retirement handling; example pick feedback.

## Cleanup
No `[DEBUG]` of mine remains in sources (guest `replacement stage` print, bridge poll stats removed). Published modules in `.cache` still contain the build with the guest debug print until the next hub rebuild.
