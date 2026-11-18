# R139: the door tells who holds it, a stalled or stale holder gives it up, a stale read hands over

Scripts: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R139/` (`land.py`, `window.sh`, `inlock.txt`, `t.sh` + `overlay.ts` = staged test runner, `battery.sh`, `py/*.py` = the schema and code patches).
Landed 2026-11-17 17:04:03 to 17:04:50 UTC under `landing.lock` (entry in `landings.md`).

## The door in this code

The door of the graph read is `🕸️graph/🕰️generations/🔐️custody` (claims `<store>/generations/s/<token>.json`, one lease per claim, admission lease). A read queues there for the right to build; a holder that is `structural` (cold generation, stale environment, added or removed project) conflicts with everybody, others conflict when their owner sets intersect. Behind it the outermost command waits as "front door alone" (`🛌️yield`, exit 75 plus ticket) until the generation advances. The incidents: the 3 h 38 min read was a structural holder; waiters printed a count only; nothing ever asked it to stop; its code was captured at start.

## 1. Transparency (schema first: custody schema `Holder`, `Progress`, `Ask`, policy numbers, messages)

- A claim carries `holder {pid, command (script by name, 200 characters), startedMs, digest}`; the holder writes the sidecar `<token>.progress.json {phase, done, total, advancedMs}` (throttled by `progressWriteMilliseconds` 5 s, fed by the generation progress events and by every finished wave: `owners done of total`).
- Waiter notice (every `noticeMilliseconds` 20 s, refreshed with the current counters): `graph: 1 refresh(es) go first, ... (21 s); holder: pid 4711 (script.ts repo show projects), started 2026-11-17 16:00:00 UTC, code aaaaaaaaaaaa, owners 120 of 482, 12 s without progress` and `Halter: ... owners 120 von 482, seit 12 s ohne Fortschritt`.
- The front door (`🛌️yield`): the ticket carries `door` (the directory the child queued in, remembered by `🪧️door-board`); the front door reads the claims without loading the custody (only held claims of living pids), prints holder, phase, owners and the time since the FIRST start of the command (not since its own wait), at the yield and again every 30 s, EN and DE.
- New member `🪧️door-board` (pure decisions + a light reader): `askReasonV1`, `isDeposedV1`, `advanceProgressV1`, `holderTextV1`, `readDoorV1`, `doorSuffixV1`.

## 2. Bounded door (progress based)

- Progress ADVANCES when the phase changes or more owners are done; elapsed time never counts (test: a holder finishing one owner per 5 min is never asked over 20 min).
- A waiter asks the held blocker once (`<token>.ask.json`, reason `stalled`) when its last advance is older than `stallMilliseconds` (600 s, schema), or `stale-code` when the holder's digest differs from the tree's current one (see 4). The holder sees the ask at its next wave boundary (`handover()`), stops, keeps its finished waves (journal) and releases. A holder that has not answered `askGraceMilliseconds` (300 s) after the ask is DEPOSED: it no longer blocks admission, its `extend` answers not-held, its release later is harmless. "No progress" is only judged for holders that report progress (a holder of older code cannot be judged).
- Limit: the unit boundary is a wave (`ownerWave` owners), not a single owner; a holder hung inside one unit is deposed after the grace, not stopped (nothing kills a process).

## 3. Disjoint scopes

- Claims of disjoint owner sets already hold at once (kept, law test). New: a holder gives back the owners it finished (`settle(wave)` after each journaled wave): a claim of exactly those owners is admitted while the rest stays claimed (test `🚪️bounded-door`, and `🤝️handover` measures `[COUNT-wave, 7]` owners left after waves 1 and 2).
- Still exclusive, and why: (a) a STRUCTURAL claim (cold, environment stale, added/removed project): its final generation rewrites the environment, the owner table and every owner's edges, so `settle` refuses for it (tested) and it conflicts with all claims; (b) the short admission lease `graph-custody-admission:<root>` (decision of one admission); (c) the publication of a generation (serialized per store, merged by the multiwriter rules, existing test `👯️multiwriter` green); (d) the selected-read writer lease `build-graph-state:<store>:selected:<shard>`: two selections that hash to the same shard queue (not changed). The selected baseline read (`loadBuildGraphUnpinnedV1` with scope) takes only (d) and no custody, so it is not behind a whole-workspace holder by code reading; the machine capacity governor admits both (not measured).
- A selected read that goes through the generation custody and meets a structural cold holder still waits behind it; the owners that holder finished only become visible to a reader through the store records, a structural holder does not release them.

## 4. Hand-over of a read on stale code

- Holder digest = `hostImplementationV1` of the process (fixed at its first queue); tree digest = `hostReloadIdentityV1(...)` (the identity the reload of the service already uses; undefined when the tree is half edited, then nobody hands over).
- At every wave boundary the generation store asks `claim.handover()`: an ask of a waiter, or (own digest differs from the tree's AND a conflicting claim waits) stale-code. A hand-over throws an error with code `SEMIO_STALE_IMPLEMENTATION` (text EN+DE: `hands over after 40 of 87 owners, what it finished is kept ...`) after the wave was journaled. The outermost command treats that code as the declared restart: it starts again on the current code; the journal key (generation, identity, membership, owners) is unchanged, so the new run derives only the owners the waves did not.
- Resident service: `failureRefusalV1` maps that error (also as a cause) to the refusal `handover` (`host-graph-handover`, EN and DE, `RefusalCode` gains `handover`); the plan client (`requestInvocationPlanV1`) asks again up to `maximumHandovers` (3) times, saying so in both languages; the next ask reaches the service of the current code (identity keyed by digest since R138).
- Nobody waiting means no hand-over: a stale read runs on (law test).

## Proofs (all in-process; red before on the old tree, green after)

| test | red -> green |
|---|---|
| `🪧️door-board/🧪️tests` (6): closed documents vs Ajv, EN/DE text, progress advance, 600 seeded decisions vs the Python model `🐍️oracle`, front door reader | 1/5 -> 6/0 |
| `🔐️custody/🧪️tests/🚪️bounded-door` (8): holder text in both languages and refreshed, ask after the stall, grace and deposition, progress not time, stale code ask and own check, error text, settle and structural, sidecar cleanup | 1/7 -> 8/0 |
| `🛌️yield/🧪️tests/🪧️holder` (2): front door names holder, phase, owners and wait since first start, EN+DE, refreshed; ticket carries `door` | 0/2 -> 2/0 |
| `🧪️tests/🤝️handover` (2) LAW: build with a waiter hands over after wave 1 (code `SEMIO_STALE_IMPLEMENTATION`, no generation published), the restarted build computes `COUNT - wave` owners, none of the finished ones, union = all, graph equals the Python oracle; with nobody waiting it runs on | 0/2 -> 2/0 |
| `🏡️resident/🧪️tests/🤝️handover` (2): refusal mapping (direct and as cause), schema texts, plan client asks again bounded with both-language notice | 0/2 -> 2/0 |

Battery before landing (staged vs base, 28 + 47 dirs): nothing worse; identical pre-existing reds: `generation-protocol` 2, `idle-builder` 2, resident `plan-refusal` 1, `provision` 6, `restart` (3/2 base, 4/1 staged), `🌐️host` 1. Under the lock: 13 dirs, 0 fail, both standing preflights green. The existing custody test compiled `Claim` alone; it now passes the schema's definitions (one line). Harness (`genuine-birth --birth=setup`) after landing: door-board 6/0, handover 2/0.

## Not verified

- No slice-lab machine was started; the isolation of two lab machines and a real edit-during-read are covered by in-process law tests only (the lab needs the live service restarted on the new code).
- The real service path across the slice worker: the error crosses as `code` (whitelisted), its text is the first line; not exercised with a real worker.
- `hostReloadIdentityV1` cost and its `blocked` case in production; `stallMilliseconds` 600 s and grace 300 s are declared, not measured on a real 17 GB read.
- A structural cold holder still blocks every other custody user until it publishes (item 3 only frees non-structural holders as they finish waves).
- Running processes (the live service, reads on old code) keep the old code until restarted; they print no holder and cannot be asked.
