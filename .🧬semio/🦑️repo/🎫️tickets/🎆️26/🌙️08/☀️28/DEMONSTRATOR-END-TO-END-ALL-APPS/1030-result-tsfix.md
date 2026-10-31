STATUS: GREEN for the codec, job drive, job funding and source-hold items; two items are deliberately narrower than the review proposed (see "Not done"). Not run against a compiled wasm guest or a real browser.

Short paths: RT = `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`, OS = `🧰️framework/🛍️products/💻️os/🔨️modules`, RD = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`.

## Verified findings
All 7 hold; 1, 2, 3, 4, 5, 6 fixed (partly for 1 and 3, see below), 7 untouched (cosmetic, out of this lane's list).

## 6 / (1) Record codec (`RD/🚪️io/💾️binary/🎒️record-document/🟦️.ts` and `🐍️.py`)
- Inflate: declared raw length capped (`MAX_SEGMENT_RAW` 64 MiB) and `inflateRawSync(..., {maxOutputLength})` / Python `decompressobj` bounded to the declared size; a deflate bomb no longer allocates before the length check.
- Allocation: new `Reader.count(unit)` refuses an item count the remaining bytes cannot back (lists, maps, statements, record fields, chunk ids, packed runs, symbols, dsl lists/maps); a table's `rowsCount` is bounded by `8 * remaining` before `Array.from`. Chunk id outside the pack is a `RangeError` (`Decoder.chunk`).
- `__proto__`: decoded dynamic maps are null-prototype objects (own `__proto__` key survives).
- Exact integers: `DslValue` gains `bigint`; dsl `INT`/`UINT` decode through `narrow()` (number while exact, bigint beyond) and the encoder takes bigint. `json()` for record `int/uint` fields still projects to `Number` (JSON snapshot view, unchanged).
- Tests (TS `RD/🧪️tests/🎒️record-document/🟦️.ts` +4, Py `🐍️.py` +3 `HostileLaws`): bomb capped, hostile row/list counts, bad chunk id, `__proto__` + 2^63-1 / 2^64-1 / -2^63 round trip. TS and Py produce the identical pack bytes (`HOSTILE_VALUE_HEX` in both).
- Run: `bun test ./<RD>/🧪️tests/🎒️record-document/🟦️.ts --timeout 120000` 17/17; `python3 <RD>/🧪️tests/🎒️record-document/🐍️.py` 16/16. Existing Rust-pack re-encode laws still byte-equal.

## 5 Spawned jobs (`🎭️actor/🖼️wire-turn/🧵️spawned-job/🟦️.ts`, `OS/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts`)
- `SpawnedJobPort` gains `cancelJob`; `driveSpawnedJob` cancels (quietly) any job it leaves running: abort, step ceiling, or a thrown step. Native reference: `cancel_job_funded` in `OS/🔌️plugin/🖥️host/🎭️guest-runtime/💼️job-crossing/🦀️.rs`, driven from `cancel_one` in `🧵️shard/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs`.
- `ReservationPollLanes.driveSpawnedJobs`: jobs now run concurrently (each crossing still queued on the single-threaded child's poll lane, like the native pump), with `session.abort.signal` combined with a sibling-failure signal, `onStep` as progress callback, `cancelJob` allowed on a closing lane, first failure (in time) rethrown.
- Child side: `stepJob` receives the activation `control.signal` (actor module) so the output collection aborts when the child closes.
- Tests: `OS/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🧪️tests/🟦️.ts` (5): interleave + order, progress, abort -> cancel, ceiling -> cancel, sibling failure.

## 3 Job output (RT `OriginalJobFunds`)
- `pending` page (no bytes, not closed) is now a retry, bounded by 1024 consecutive pending pages and the funds' wall deadline (child clock), then `original-job.output-stalled`.
- Collected pages live in a caller-held `OriginalJobOutputHold` (per bridge / per `OriginalSelfIssuedTurns`), so an aborted or stalled collection resumes at the guest's offset instead of faulting `output-offset`; `cancel` and a completed ack drop the hold.
- `OriginalJobControl`: `signal` (abort between pages), `onProgress` per page, `held`.
- Page size: the 64-byte cap is the guest's (`OS/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs` `read_job_output_step_with`, `.min(64)`); not changed (a limit raise in the guest, outside this lane). The native host also faults on a pageless unclosed page (`collect_job_output`), so the retry is a deliberate TS-side hardening, not a mirror.

## 4 Crossings through the actor's admission
- New `originalSourceJob(source, report, wallUs, run)`: a job request issues one turn from the actor's own `OriginalOperationSource`, runs on funds built from that turn (`OriginalJobFunds` 5th ctor arg), then receives and reports ONE aggregate receipt (also on failure, the first failure always wins). The shard bridge (`OS/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`) uses it for start/step/cancel and serializes polls and jobs on one `exclusive` chain so `issue()` never races a poll. Native: `ShardLoop::job_funds` funds from the actor's Kernel-issued retained turn (`🧵️shard/🏗️shard-loop-construction-and-replay-drive/🦀️.rs:42`).
- The document actor child (no Kernel grant) keeps fresh self-issued funds per request, as the native relay does (`relay_job_funds`, `🔀️post-turn-relay/🎟️instance-slots-leases-and-cancel-helpers/🦀️.rs:178`); the `clockBinding:1n` of self-issued funds is the native `self_issued_input` value.

## 1 / 2 Self-issued actor stock and failed polls (`OriginalSelfIssuedTurns`)
- Per-turn refill is NOT possible: the guest's receiving owner conserves the grant across turns (`receive_next_issued`, `🎟️retained-turn/🦀️.rs:92`: next epoch = previous + 1, grant = previous remaining), so a fresh epoch-1 grant would be refused (confirmed by tshost's epoch 2n/3n law). Native headless hosts get fresh stock only per fresh guest.
- Replenishment = a new source for a new child: exhaustion is now a typed `original-turn.stock-exhausted` fault (reserve 4 MiB per byte axis / 4096 items) raised before the turn, instead of the guest answering `more-work` forever; unconfirmed polls already retire the reservation and request recovery (`🎯️action-dispatch/🟦️.ts:100`), which re-reserves a child with a fresh source.
- Failed poll without a receipt: the source is held (`holdNativeRefusal`, like the shard bridge), the first failure is rethrown unchanged, a secondary receipt error cannot replace it, and later `issue()` errors carry it as `cause` (`OriginalOperationSource.issue`, `original-turn.source-not-ready`). A true "recoverable" source is unsound: after a trap/lift error the guest's owner stays held natively too.
- Tests: `🎟️retained-turn/🧪️tests/💼️job-funds/🟦️.ts` (10): pending retry, stall bound, wall stall, abort + resume + progress, cancel drops hold, Kernel source aggregate receipt/failure/over-spend, cause chain, stock exhaustion.

## Verification
- `bun test` explicit paths: record-document 17/17; job-funds 10/10; poll-lanes 5/5; wit-lowering 5/5 (real generated bridge incl. jobs + close); materialization bootstrap 3/3; retained-turn tests 2/2. `lifetime` 1/2 fails with "An original caller artifact directory is required" (environmental; not touched).
- tsc (`C/1030-tools/tsfix/tsconfig.json`, output `C/🗑️generated/1030/tsfix-tsc.txt`): only foreign `ArtifactOwnerSlots.browserActorReservation` error in `🧾️document-state`.
- Not run: `spec.sh` (cargo), vitest staging/GIS suites (tshost owns), compiled-wasm browser run.

## Not done / notes
- Jobs still run inside `reconcileUiPatches` on the turn lane (the completion event is delivered by that loop); true overlap with UI turns needs the completion delivered out-of-loop. Concurrency among jobs and crossing-level interleaving with polls is in.
- Progress has a sink parameter only (`driveSpawnedJobs(..., onStep)`); no existing UI progress channel was wired by the reconcile caller.
- 64-byte output pages remain (guest constant).
- Item 7 (stale `unwrapOption` docs, `actor-poll.event-count` asymmetry, untracked `🧩️actor-module`) untouched.
- Embedded worker source (`originalOperationWorkerSource`) now also carries `originalSourceJob`, `ORIGINAL_JOB_PENDING_PAGES`, `ORIGINAL_SELF_ISSUED_RESERVE_*`.
