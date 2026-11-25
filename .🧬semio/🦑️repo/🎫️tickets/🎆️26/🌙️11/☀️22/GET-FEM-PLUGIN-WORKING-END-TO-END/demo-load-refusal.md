# Demo Load Refusal: "Editor could not be loaded" (2026-11-25)

## Symptom
In the FEM 2D editor (http://127.0.0.1:6086/), picking the **Demo** example showed "“Editor” could not be loaded…". The console showed `original-turn.native-return-refused: authority (invariant-violated)`. After that, the typed-operation progress refresh, the completion effects and the tree window refresh failed too.

## Cause (guest, the wasm published at 08:16)
I instrumented the bridge (`🌉️bridge.js`), the main-thread worker `postMessage` and `handle-turn-drive` `driveSpawnedJob`. I then ran fresh browser contexts, picking Demo once per context.

1. Loading Demo spawns the `semio.fem2d.mounted-analysis` job. The host calls `OriginalJobFunds.start` in the bridge, which runs two rounds:
   - round 1: admit `{operation:"start-job", source}`, then `jobs.startJob` answers `admitted=false` (the guest takes the source);
   - round 2: admit `{operation:"start-job", source:null}` (38 bytes), then `jobs.startJob` is expected to answer `admitted=true`.
2. In every failing run, round 2 was refused with `authority (invariant-violated)` and zero spend. The bridge then held a native refusal, so every later call failed. That produced the load failure and all the follow-on errors.
3. Discriminator, 9 of 9 runs on the 08:16 wasm: a failing run's round-1 `startJob` receipt spent `copiedItems 7, copiedBytes 1022, work 1`. A working run's spent `1, 0, 1`.
   - The extra 6 items / 1022 bytes are deferred guest return housekeeping. It costs copy bytes only, with no capacity and no work.
   - The same 7/1022 receipt shows up on whichever call comes next after certain command turns. Seen on polls at epochs 17, 115/117, 121, 127, 129 and 799.
   - Whether it lands on `startJob` round 1 depends on timing, which explains the roughly 50% failure rate.
4. On the 08:16 build, a round 1 that also paid this housekeeping left the actor context in a state where round 2 was refused before any spend.
   - The refusal comes from the context machinery, not from the job body: a body error is reported as Semantic, never as Authority.
   - So the refusal comes from `validate_currency` / `begin_dispatch` / `borrow_context_with_delivery` in `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`, or from the guard in `dispatch_job_start_step` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs`).
5. The host is not the cause. Every host change in the 05:18–06:20 window was checked:
   - document-load-gate, document-load-effects, surface-switch, shell-core-state, typed-result, module-loader `refreshUi`, handle-turn-drive cancel paths, archive-transfer, apply-named-layout, program-dispatch.
   - The guest call sequence was the same in failing and working runs.
   - `driveSpawnedJob` waits in `serializeCommandIngressForActor` until the archive-load command turn finishes. That is the designed lane order, and it is the same in both outcomes.
   - Host changes only shift when the housekeeping lands.

The exact guest refusal message could not be captured. The 08:16 wasm was built at 07:42 from source older than `report_authority` (`🎭️exports/🦀️.rs`, 06:54), so it logged no `original-turn.authority …` line. The wasm has since been overwritten, so the failing artifact can no longer be inspected.

## Fix
This is already fixed in current guest source. No new code change was needed, and no host change was made.

- `wasm-retry.sh` (started 08:23 in this ticket) built the current source and republished at 09:39:
  - `semio_hub_fem_component.wasm`, sha256 `03c89a6360e67d65adaeb3cfc75d308c25cc7c893593b7ca15c7a70b6131e7fb`;
  - the core wasm contains `original-turn.authority`.
- On that build, the exact hazard still occurs (round 1 receipt 7/1022, `admitted=false`). Round 2 is now admitted every time.
- The fixing delta is one of the guest edits between the 07:42 build's crate compiles and the 08:24–08:38 compiles. The likeliest is `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` (07:21), the decoder owner whose `has_pending_receiving` gates `retained_dispatch`.
  - Other candidates: `⚛️reactor/🔄️turn/*` (07:18–07:36), `⚛️reactor/🔌️app`, `🧾️typed-operations` (07:55).
  - There is no intra-day history, so it cannot be pinned further.

Latent guest defect, not changed here: `start_job_wit` (`🎭️exports/🦀️.rs`) drops `reply.returned_source`. If round 1 ever returns the source untaken (yield, or an empty grant after housekeeping), the source is lost and round 2 gets `invalid-value`. Fixing that needs a guest edit and a wasm rebuild.

## Law
`demo-load-law.mjs` (this ticket) runs a language-agnostic browser law through Playwright (third-party).

Usage: `node demo-load-law.mjs [url] [runs]`

- Each run uses a fresh context, picks Demo, and records every `startJob` round's receipt with a bridge probe.
- A run fails on any `original-job.call-refused`, `load-failed`, `native-return-refused`, page error or "could not be loaded" text, or if no round is admitted.
- The law also fails if no run exercised the housekeeping round (`copiedItems > 1`).

## Test result (09:40–09:50, the 09:39 build)
- 4 instrumented repro runs: 4/4 loaded, 4/4 hit the 7/1022 round 1.
- `node demo-load-law.mjs http://127.0.0.1:6086/ 5`: `{"runs":5,"passed":true,"exercised":4}`, exit 0.
- Before (the 08:16 build): 9/9 runs that hit the 7/1022 round 1 failed, and every run that did not hit it passed.

## Files
- Created: `demo-load-law.mjs`, `demo-load-refusal.md` (this ticket).
- No source files changed. My temporary `🗑️generated/load` captures were deleted, and my page driver on port 6199 was stopped.
