# 1030 review: TypeScript diff outside repo tooling (lane reviewts)

STATUS: DONE (read-only, nothing built or run). WIT-lowering mismatch in the browser host is lane `tshost`'s and is not repeated. Ordered most severe first.
Short paths: RT = `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`, OS = `🧰️framework/🛍️products/💻️os/🔨️modules`, BB = `OS/🔌️plugin/🌐️browser-bundle`.

## 1. Self-issued document actor stock is cumulative and never replenished (MEDIUM-HIGH, confidence medium; owner runtime)
- RT:67 (`ORIGINAL_SELF_ISSUED_GRANT` = 1M items, 64 MiB copy/capacity/release) and RT:97 (`OriginalSelfIssuedTurns` ctor).
- `OriginalOperationSource.receive` subtracts every receipt's `spent` from `#remaining` (RT:55-56) and nothing ever adds back. The browser document actor child is one source for its whole life.
- Scenario: a long gis/cad session whose turns copy patches/pages (up to 2 MiB per turn, `browserActorTurnShape`) exhausts the 64 MiB copy axis after tens of large turns. The guest then sees `maximum_items==0` or no copy budget and answers `pending`/`more-work` forever (Rust `read_job_output_step_with` and sibling guards), or `receive` throws `receipt-refused`, and every later `issue()` throws `source-not-ready`. The actor silently stops, and the render loop spins until `DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT`.
- Kernel-admitted shard actors have the same finite-stock rule by design, but the shard bridge reports receipts to the kernel; the child has no refill path.
- Fix direction: either re-admit a fresh source epoch/generation per turn (per-turn loan, like the native relay), or surface exhaustion as a typed fault that triggers child re-reservation.

## 2. Any non-receipt poll failure wedges the self-issued source with an opaque error (MEDIUM, confidence medium-high; owner runtime)
- RT:99 (`OriginalSelfIssuedTurns.poll`). `issue()` marks `#outstanding`; on a thrown error without `input/spent/workUnitsSpent` (JS lift/lower TypeError, trap, abort), the catch only rethrows. `#outstanding` stays set.
- Every following `poll`/`#context` throws `original-turn.source-not-ready`, hiding the root cause. The shard bridge (`component-bridge-source:176-190`) at least holds the refusal (`holdNativeRefusal`, `originalReturn.hold`); this class does neither.
- Also: if the receipt branch itself throws (`foreign-receipt`, `foreign-grant`), it replaces the original failure.
- Fix: in the catch, `cancel()`/hold the source and rethrow the first failure; never replace it.

## 3. Job output read has no recovery from a stall and moves at most 64 bytes per crossing (MEDIUM, confidence medium; owner runtime/tshost)
- RT:91 (`#collect`) against Rust `OS/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs:76` (`.min(64)` per page; `pending` page = no bytes, `outputClosed` false).
- (a) A 1 MiB job result needs about 16 400 awaited WASM crossings, each with a fresh `BigInt` receipt settle. This sits on the poll lane, so UI turns queue behind it (the lane serializes polls and jobs: `poll-lanes:20-33`).
- (b) Any `pending` page (yield on `should_yield()`, deadline, multi-step retirement of the body at the end) hits `if(page===undefined)throw "output-stalled"`. The guest keeps its offset, the host's `parts` are lost, and the retried step then fails `output-offset` (`chunk.offset!==length`). One wall-clock overrun makes the job unrecoverable instead of resumable.
- (c) No cancel signal or progress callback inside the loop (AGENTS.md: progress and cancellation for expensive operations); the only bound is the guest's wall deadline.
- Fix: treat "no bytes and not closed" as "call again" (bounded by the wall deadline), keep `parts` on the instance across calls, thread an `AbortSignal`/progress callback.

## 4. Job crossings are paid from a fresh 64 MiB / 16M-fuel stock per request, outside the actor's kernel admission (MEDIUM, confidence medium; owner runtime)
- RT:85 (`OriginalJobFunds` ctor) and `component-bridge-source:133` (`jobFunds`). Every `startJob/stepJob/cancelJob` builds new funds (`ORIGINAL_JOB_WORK_FUEL=1<<24`, self-issued grant) and never calls `originalReturn.receipt(...)`.
- A guest job can therefore spend up to that stock per call regardless of what `KernelOriginalResourcePool` admitted to the actor, and the kernel never sees the spend. Unbounded in aggregate. Needs a deliberate decision (charge the actor source, or report the receipts), not a silent bypass.
- Note `clockBinding:1n` here (RT:85) versus `clockBinding: input.operation` in `originalTurnWit`; the Rust guard compares it only where the job borrows the parent (`retained-turn/🦀️.rs:97`), so this is low-confidence drift, not a proven fault.

## 5. Spawned jobs are never cancelled, never abortable, and run sequentially on the turn lane (MEDIUM, confidence medium-high; owner e2eprep/tshost)
- `OS/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts:49-56` builds the `SpawnedJobPort` with only `startJob`/`stepJob`. `driveSpawnedJob` (`🎭️actor/🖼️wire-turn/🧵️spawned-job/🟦️.ts:69-82`) supports `signal` and `onStep`, but neither is passed; there is no `cancelJob` in the port.
- Scenario: the user closes the document or the step ceiling is reached while a job is `running`. The guest job keeps running in the (still reserved) child, and its `job-completed` is never delivered or reaped. `driveSpawnedJobs` loops `for (spawned of jobs)` serially inside `reconcileUiPatches` (`ui-patch-reconciliation:103`), so a second job waits for the first and no UI patch or event gets through (turn lane held). No progress is reported while waiting.
- Fix: plumb the session `AbortSignal` (cancel the job on abort), report `onStep` as progress, drive independent jobs concurrently.

## 6. Record codec oracle: unbounded allocation and prototype-key hazards on untrusted packs (LOW-MEDIUM, confidence high; owner e2eprep)
File: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts`.
- :441 `inflateRawSync(stored)` has no `maxOutputLength`; `rawLength` is compared only after inflating. A 1 KiB deflate bomb allocates GiBs before the check.
- :1075 `Array.from({ length: rowsCount }, () => new Map())` with an attacker `rowsCount` (varint up to 2^53) allocates before any `take` bounds check.
- :1129 `entries[key] = ...` on a plain `{}`: a map key `__proto__` assigns the prototype instead of an own property, so the key silently disappears from the decoded value and its content is reachable as inherited members. Use `Object.create(null)` or `Map`.
- :1019 `this.chunks[id]!` for an out-of-range chunk id gives `undefined` into `concat` (TypeError `reading 'length'`), not the RangeError every other refusal throws; tests asserting `RangeError` miss it.
- :1115 and the `TAG.INT` case below it: `dslValue` integers go through `Number(...)`, silently losing precision above 2^53, while record fields use `narrow()` (bigint beyond). Rust i64/u64 seeds or ids inside a `value` field would round-trip differently. Schema drift between the two views of the same wire.
- Only tests import it today (grep), so impact is test-oracle only; fix before it is reused as a loader.

## 7. Cosmetic / consistency (LOW)
- `component-bridge-source:204` and `OS/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/📥️segmented-download-and-multiplexing/🟦️.ts:75`, `🎭️actor/📮️shard-client/📤️segmented-download/🟦️.ts:46` still document `unwrapOption`, which no longer exists (now `optionValue`).
- `shard-worker-source/📨️message-handler/🟦️.ts:96` ("turn") throws `actor-poll.event-count` for more than one event, while the frame case (:227-233) iterates per envelope. If any caller of `ShardClient.turn` can pass two events, they fail only on the browser target. I found no such caller, but the two paths differ.
- `🧩️actor-module/🟦️.ts` is untracked and new while its content moved out of `📜️script.ts`: make sure it is added with the change (otherwise `script.ts:19` fails to resolve on a clean checkout).

## Checked, nothing found
- Poll-lane serialization (`pollTail`) keeps `OriginalSelfIssuedTurns.poll` from racing itself; `close()`/`onAbort` rejections are handled (`closing.catch`).
- `activate` abort path, `phase` checks, `active` capacity (32) are consistent with the previous inline code.
- `♻️mit-bestand/package.json`/`bun.lock`: the dropped `🕸️bindings` workspace is gitignored generated output; `@semio-tech/flow-core` is still served by the vite playground stub, nothing else depends on it by workspace.
- No React host shell, vite activation plugin or demonstrator TS source is part of the uncommitted diff, so there is no i18n (English/German) or accessibility change to review; none found.
- `test-poll-input-*` targets: no leftover references.
