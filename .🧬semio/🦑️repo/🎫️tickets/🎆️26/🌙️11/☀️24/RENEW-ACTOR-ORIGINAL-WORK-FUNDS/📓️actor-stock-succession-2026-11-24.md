# Actor stock succession (2026-11-24)

## Problem
A browser plugin actor draws every turn, command and job from one finite stock issued at activation (`🔨️canvas/🧵️renderer-work/🔣️stock.json`). At the measured rate one 2^31 capacity axis lasts about 9 minutes of continuous heavy editing (`DRAW-PLUGIN-END-TO-END/panel-add-stall-2026-11-24.md`); wasm32 cannot hold more than 4 GiB per axis. An empty axis was answered with `backpressure` while idle.

## Decisions
1. **The successor renews funds, not the Source.** Resident owners (`OriginalSourceBinding`, `OriginalConstructionAdmission`, return places, payload ledger) compare operation, generation, clock binding, clock function, payload identity and absolute deadline. Rebinding any of those would refuse every document owner that is still open. So the kernel issues a fresh operation identity (the *tenure*), and only the funds it conveys are bound to the unchanged Source.
2. **No new WIT export.** The succession crosses as the actor-input operation `succeed` (`🔌️plugin/🧬️schema/🎟️actor-input`) and only `admit-actor-context` consumes it. Two crossings, both paid by the predecessor, both receipted.
3. **Capacity stays cumulative.** Live accounting (retained minus released) would make a grant grow between turns, which the receipt law forbids (`issued.grant == previous.remaining`), and releases are not proven to match charges (measured 308 MB charged, 156 MB released in one run). Memory residency has its own owners (payload ledger, wasm memory). Succession removes the end that cumulative accounting caused.
4. **The deadline is identity.** No successor extends it, so `stock.json` now admits 365 days instead of 24 h. The pool row is 2^52 per axis and 2^52 fuel (4,096 operations at 2^40 fuel, 2,097,152 by the byte axes); the operation row is unchanged.
5. **Unspent predecessor stock is forfeited**, as at close. Nothing returns to the pool.

## Laws (schema `🎟️retained-turn/🔁️succession/🧬️schema/🔣️.json`, fixture `🧪️tests/🧫️fixtures/🔣️.json`)
- A succession names the exact predecessor turn (operation, generation, epoch) and a successor identity above every earlier tenure. Replays, foreign identities and empty stock are refused without spending.
- Its receipt is exactly one item, the record's bytes and one work unit, no capacity and no release. The host accepts nothing else: a guest that answers `ready` without it is held as `original-turn.succession-unproven` (a guest built before this change).
- A plain turn with a renewed grant is still refused (`renewed its physical source authority`); `validate_currency` is untouched.
- Quiescence: host side no outstanding turn, no held output, no job it knows is live, not cancelled; guest side no job slot, no job source or output, no admitted or staged input, no retained failure, no decoder in flight, no return origin being published.
- A refusal defers the next attempt (doubling, at most 1,024 crossings; a settled job resets it) and keeps the drawn successor. The kernel offers the same one until a receipt names it.
- Policy (`📋️policy.json`): low water at 1/4 of the tenure's stock, floor at 1/16. Below the floor new work stops with the typed fault; cancel and close may still spend it, so cancelling the job that held the actor renews it.
- **Second ledger.** The Source's return book (`🧵️job/📬️outcome/🪙️budget`) keeps its own stock and only resets per turn while no return claim is live. It is carried over in the same step; live holds stay reserved and are funded again by the successor. The native run found this, the TypeScript model could not.

## Typed fault
`plugin.original-stock-exhausted` (`OriginalStockExhausted`, `{axis, remaining, floor}`, retryable) is thrown by the worker bridge, carried as `fault` on the shard reply, attached to the host error, and has an EN and a DE notice. It is also the fault of `OriginalSelfIssuedTurns` (was the untyped `original-turn.stock-exhausted`, reserve now 1/16 of the grant, i.e. the same 4 MiB on the byte axes). `PAGE_IDLE_REFUSAL_LIMIT` in `handle-turn-drive` stays as the bound for a guest that refuses for another reason.

## Changed
- `🎭️actor/🎟️retained-turn`: `🟦️.ts`, `🔁️succession/*` (new), `🦀️.rs`, `🏛️bootstrap/🟦️.ts`, `🏛️bootstrap/📮️port` (TypeScript and schema), `🎟️context/🦀️.rs`, `🎟️context/🧪️tests/🔁️succession` (new), `🧪️tests/💼️job-funds`; `🎭️actor/🦀️.rs`, `🎠️kernel/🦀️.rs` (re-exports).
- `🧵️job`: `📬️outcome/🪙️budget` (`succeed`), `⏸️continuation/🎟️source`, `⏱️context/🎟️source` (`succeed_return_stock`).
- `🔌️plugin`: `⚛️reactor/🎟️context` (`🌉️call`, `🎟️admitted-input`, `🎭️exports`, tests), `🧬️schema/🎟️actor-input`, `🌐️browser-bundle/🏗️materialization` (component bridge, worker, `🧪️tests/🔁️succession` new).
- `📮️shard-client` (messages, activation, request transport), `📤️return` inbox fixture, public facets roster, taxonomy roster, notice labels (TypeScript, Rust, fixture), `stock.json`, wgpu frame-worker test doubles.
- `⚛️reactor/🎟️context/🧪️tests/🟦️.ts`: four stale expectations (admission phases 4 to 6, `borrow_context_for_turn`, admission copy) aligned with the General twin, which already had them.

## Measured
| Check | Result |
| --- | --- |
| `🎟️retained-turn` TypeScript (21 files, `repo test-file`) | all pass; `🔁️succession` 11 of 11: real kernel stock, Source and driver, 6 stocks, every edit and job kept, per-tenure conservation checked in SQLite |
| Generated bridge + generated worker + real `ShardRequestTransport` + real kernel stock (`🏗️materialization/🧪️tests/🔁️succession`) | 2 of 2: 6 stocks with jobs interleaved, nothing lost; typed fault with EN and DE notice; cancel renews. The component is a JavaScript model of the guest's accounting |
| Plugin actor transport suite | 183 of 183 |
| Native `semio-framework-actor-retained-context` succession laws | 4 of 4: 6 real stocks on the real Source with a return claim held from the first to the last stock and an unchanged `OriginalSourceBinding` |
| Native `semio-framework-job-return-budget` | 11 of 11 (one new) |
| Native `semio-plugin-reactor-context` | 10 of 10 (two new, real guest entry), then 9 of 9 in the filtered rerun after the clock fix |
| Native `semio-plugin-abi` actor input | 2 of 2 (Rust writes and reads the pinned bytes) |
| `cargo check -p semio-plugin-reactor-context --target wasm32-wasip2` | passes, export bindings included |

## Not measured, not done
- At the time of this section no component was built and nothing ran in a browser; see "Browser run on guest r37" below. A guest must be rebuilt before the new bridge reaches its low water: an older guest answers `ready` without the succession receipt and is held with a message that says to rebuild. The app-channel version was not bumped.
- Rate after renewal (cost of two extra crossings per stock, about 7 minutes apart at the measured rate) was not timed.
- **No job slot, as asked:** an actor whose job slots are never all empty between low water and floor (12 % to 25 % of a stock) still gets the typed fault; it is not deferred. Because the Source identity is kept, a parked job looks renewable in principle; that was not tested and is not claimed.
- The wgpu frame Worker (`OriginalBrowserFrame`) and the document child (`OriginalSelfIssuedTurns`) own separate sources and do not renew. The child faults typed and is re-reserved as before.
- The owned (interpreter) ABI refuses the `succeed` input; the native kernel issues one operation per request and needs no renewal.

## Failures that are not from this change
- `semio-framework-actor-retained-context`: 11 older tests fail. They expect four admission epochs and a decoder loan born during admission; `context/🦀️.rs` at HEAD already admits prepare, backing and publish separately. Not run on an untouched tree, but none of them reaches a changed line other than the added `tenure` field.
- `⚛️reactor/🎟️context` TypeScript source-text tests: `🌉️call/🧪️tests` (2), `💼️job/📤️receipt` (1), `📤️failure` (1), `📤️output` (1), `⚖️failure-debit` (9) look for text that moved to other owners.
- The `semio-plugin-guest-contract` test target exceeds the 32-file source boundary with or without this change, so its tests cannot be built; the entry law lives in the reactor context crate for that reason.

## Browser run on guest r37 (evening of 2026-11-24)
**Cause of "never renewed":** the playground served `🧵️shard/🟨️shard-worker.js` published on 21 Nov 15:51 (43,087 bytes, no `requestSuccession`) with a bridge of 24 Nov 20:27. The kernel link therefore had no `succeed`; `advance()` ran at low water with every gate clear (no jobs, nothing held, nothing outstanding), got `TypeError: this[#kernel].succeed is not a function`, and stored it as a kernel refusal. The native chain (`component`, `prepare`, `activate`) has no `support` stage, and `prepare` and the playground launch only checked that the worker file exists. None of the four suspects was the cause.

**Fixed**
- `OriginalStockSuccession` refuses a kernel link without `succeed` when the actor is built: `original-turn.succession-unoffered … run repo run @semio-tech/framework-plugin-web:support`. Fixture rows `offers`.
- `shardWorkerPublicationV1` classifies a published worker as missing, stale or current by comparing its text with what the sources generate. `prepare` refuses a stale one; the playground launch republishes it. Fixture and schema in `🏗️materialization/🧪️tests/🔖️worker-freshness`.
- A refused `startJob` no longer stays in the host's job set (found by reading, suspect 1; not the cause here).
- Test doubles that build an actor now offer `succeed`.

**Browser runs** (`probe-succession-gates.mjs`; worker, bridge and stock routed into the page, nothing republished; guest r37)
| Run | Result |
| --- | --- |
| r37 bridge, served worker, 2^28 | `succeed is not a function` at low water; no request sent |
| r37 bridge, current worker, 2^28 | a job was running at low water; the guest then faulted `plugin.internal.zero-progress` in the cleanup pump with 18.9 M capacity left (floor 16.8 M) |
| r37 bridge, current worker, 2^29 | one succession (tenure 2) at 75.7 s, next two shapes added; guest out of memory at 91 s |
| new bridge, served worker | boot fails with `succession-unoffered` and the command to run |
| new bridge, current worker, 2^29 | one succession (tenure 2) at 81.3 s, two more shapes added and one more job settled; guest out of memory at 94.7 s |
| r37 bridge, current worker, 2^31 (full stock, no succession) | guest out of memory at 97.8 s, with the same tenth shape |

**Open**
- **Guest memory.** Linear memory grew from 231 MiB after boot to 512 MiB at the tenth shape and never shrank. The full-stock control has no succession, so this is not from renewal. It ends the session before a full stock reaches low water (1.70 G of 2.15 G capacity left). Not diagnosed.
- **A single crossing larger than the floor** faults in the guest (`zero-progress`, or `original close demand exceeds the remaining Source grant` in the 2^28 runs) and not as `plugin.original-stock-exhausted`. Seen only at 2^28, where one Draw job costs as much as the window from low water to floor.
- More than one succession in a browser session was not reached, because of the memory limit.
- A worker published by one runtime and compared by another was not tested; both publishing and checking run in Bun.
- The staged worker was not republished here, because the running playground serves that folder.
- `🧑‍💻os-dev-composition-ownership` (7 fail) and `🧪️ticket-owned-browser-host-staging` (syntax error in `🚚️distribution/🏗️compiler/🟦️.ts:45`, dated 19 Nov) fail on files this change does not touch; the second one's test kit was edited but could not be run.
