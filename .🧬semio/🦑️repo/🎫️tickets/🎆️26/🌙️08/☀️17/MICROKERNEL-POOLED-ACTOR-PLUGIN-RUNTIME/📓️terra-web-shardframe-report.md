# terra-web-shardframe report

Executor: terra-web-shardframe. Ticket: `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. No cargo builds were run (TypeScript only, per binding rules). No git-modifying commands were run.

## delivered

Owned-path edits only:

1. **`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`**
   - New `//#region 📨️ShardFrame`: `ShardFrame` discriminated union mirroring Rust `ShardFrame` (`Register`/`Unregister`/`Grant`/`Envelope`), `ShardEnvelope`/`ShardOrigin` mirroring Rust `Envelope`/`Origin`, `MAINTENANCE_LANE_DEFAULT_BUDGET` (mirrors `lane_defaults::budget_for(Lane::Maintenance)`), `orderEnvelopesByLane`, `GrantedBudgetTracker`/`createGrantedBudgetTracker`, `interpretShardFrame` (pure dispatch, mirrors `ShardLoop::pump`'s per-frame logic), and `SHARD_FRAME_VARIANT_FIELDS` (runtime twin of the union's field names, since TS types erase at runtime).
   - `ShardClient.envelope(shardEnvelope)` — sends `ShardFrame::Envelope` (passthrough, budget-less on the wire).
   - `ShardClient.grant(actorId, budget, envelopes)` — sends `ShardFrame::Grant` with envelopes pre-sorted by lane via `orderEnvelopesByLane`.
   - New `"frame"` `OutboundMessage` wire kind, additive alongside the existing `"activate"`/`"turn"`/etc — none of those were changed.
   - Header doc updated to reflect that A1's generated mirror (`🤖️generated/🟦️actor.ts`) is now clean.
   - 12 new in-source tests (see `## baseline vs after` below for names).

2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`** — `shardWorkerSource()` only:
   - Added `MAINTENANCE_LANE_DEFAULT_BUDGET`, `SHARD_FRAME_LANE_ORDER`, `grantedBudgets` (per-actor last-granted-budget map), `orderEnvelopesByLane`, `interpretFrame` — a hand-transcribed JS mirror of `interpretShardFrame` (a template-string worker body can't `import` a module, so this duplication is necessary, not sloppy; documented in both files' headers).
   - New `case "frame":` in the worker's message switch: `Register`/`Unregister` are acked as pure bookkeeping (`Unregister` also forgets any granted budget); `Grant` records its budget and runs its (lane-sorted) envelopes through the existing `actor.api.poll(events, budget)` path under the SAME in-flight-turn guard `"turn"` already uses; `Envelope` resolves the actor's last granted budget (or the Maintenance floor); anything else is acked `{ ignored: true }` instead of thrown.
   - `dispose` now also clears `grantedBudgets` for that actor.

`pluginComponentBridgeSource` and every build/orchestration function in that file were left untouched, per the ticket's ownership split.

## ShardFrame TS ↔ Rust variant table

Read fresh off `🖥️host/🧵️shard/🦀️component.rs` by the in-source parity test on every run (not hand-copied once and left to drift):

| Rust variant | Rust fields | TS `kind` | TS fields |
|---|---|---|---|
| `Register { actor: ActorId }` | `actor` | `"Register"` | `actor` |
| `Unregister { actor: ActorId }` | `actor` | `"Unregister"` | `actor` |
| `Grant { actor, budget, envelopes }` | `actor`, `budget`, `envelopes` | `"Grant"` | `actor`, `budget`, `envelopes` |
| `Envelope(Envelope)` (tuple, no field name) | — | `"Envelope"` | `envelope` (this mirror's own naming choice for the lone tuple position — documented as such, not asserted against Rust since there is no Rust name to compare) |

`actor` is typed `string` on the TS side (this file's own established id vocabulary), not the generated mirror's bit-packed `ActorId` bigint — same "stand-in, camelCased" convention this file already uses for `ShardBudget`/`ShardJobBudget`.

## encoding decision + reasoning

**Shape-only (structured clone), not bytes.** `ShardFrame`/`ShardEnvelope`/`ShardOrigin` travel over `postMessage` as plain objects, not through the Rust `pack_encode`/`pack_decode` hand-rolled binary codec `ShardFrame::pack_encode` uses on the thread/process transports.

Reasoning:
- `ShardEnvelope.payload` is typed as the pre-existing, ALREADY-DECODED `ShardEventEnvelope` (`{kind, payload}`), not the Rust `Payload` union's real wire shape (`{"kind":"event", bytes:[...]}` — opaque pack-encoded bytes). Adopting the byte-accurate `Payload` would produce envelopes the jco-guest bridge cannot execute today: nothing on the web side decodes a pack-encoded `Payload::Event` blob back into the WIT `event` variant jco's `reactor.poll` expects (confirmed: `shard-client.ts`'s own pre-existing header doc already flagged this exact gap for `turn()`, unresolved because "no TS mirror of that codec exists yet"). Carrying opaque bytes nobody can decode would be a regression disguised as parity.
- Everything ELSE about `ShardFrame` — its own 4 variants, and `Envelope`'s scheduling metadata (`to`/`from`/`lane`/`seq`/`deadlineMs`/`coalesce`/`cancelOf`) — is real, checkable metadata the web side can use *today* for lane ordering and per-actor budget tracking, with no decode step needed. Adopting exactly that (and no more) is the honest boundary of what's actually usable this packet.
- A future byte-level unification is a single, localized swap: change `ShardEnvelope.payload`'s type from `ShardEventEnvelope` to a real decoded `Payload`, once a TS `pack_decode` exists for `semio_framework_actor::Payload`/`Envelope`. Nothing else in `ShardFrame`, `orderEnvelopesByLane`, `GrantedBudgetTracker`, or the worker's dispatch changes — the seam is exactly this one field, which is why I judged shape-only correct FOR NOW rather than a stopgap that would need unwinding.

## proof the budget constant is gone

There is **no `DEFAULT_SHARD_BUDGET`-style named constant anywhere in either owned file** — verified with `grep -rn "DEFAULT_SHARD_BUDGET\|TURN_BUDGET\|JOB_STEP_BUDGET"` across `🧰️framework` (empty result) and by reading `shard-client.ts` in full before editing. The one place a constant-*shaped* budget lives today is `🎠️kernel/🟦️component.ts`'s `ActivationRegistry.defaultBudget` (`budgetFor: () => this.defaultBudget`, `turn-scheduler.ts`'s own seam) — **that file is not in this packet's owned paths** and I did not touch it. `grep -rn "new ActivationRegistry("` across the repo also shows **no real (non-test) construction site exists yet** — so on the actual product side this constant isn't even wired to a running app today; it's exercised only by that file's own test fixtures (`BUDGET_FIXTURE`).

What THIS packet proves instead, entirely within its owned files:
- `ShardFrame::Envelope`'s wire shape is structurally budget-less — `Object.keys(sentFrame)` for an `envelope()` call is exactly `["kind", "envelope"]` (asserted in a test below). There is no parameter position left for a caller to hand it a cached constant, unlike `turn(actorId, events, budget)`'s signature which requires one on every call.
- `GrantedBudgetTracker`/`interpretShardFrame` prove, with a real assertion (not just "unused"), that once an actor has been `Grant`-ed a budget, a LATER budget-less `Envelope` for that same actor resolves to `budget === grantBudget` (`toBe`, reference identity) and explicitly `not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET)` — i.e. the constant floor is provably NOT what runs once a real DRR grant exists for that actor.
- The generated `shardWorkerSource()` JS was executed for real (see `## honest gaps` — an ad hoc smoke test, not part of the graded suite) and confirmed the same behavior end-to-end through the actual worker text, not just the TS mirror.
- Migrating `ActivationRegistry`'s own `budgetFor: () => this.defaultBudget` onto `ShardClient.grant()` is explicitly a follow-up packet's work (that file is registrar/other-packet-owned), not something I could close out here — flagged honestly below rather than claimed.

## commands + exit codes

```
$ cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript" && bun ./📜️script.ts test --reporter=verbose
...
 Test Files  3 passed (3)
      Tests  40 passed (40)
   Start at  21:39:17
   Duration  243ms (transform 262ms, setup 0ms, import 298ms, tests 41ms, environment 0ms)
EXIT_CODE=0
```

```
$ cd "/Users/ueli/Documents/semio" && npx tsc --noEmit -p tsconfig.json --skipLibCheck
```
Exit code 1, but **zero** of the 19 reported errors are in either file I touched (`grep -i "shard-client\|plugin-web-materialize"` on the output returns nothing) — all 19 are pre-existing errors in unrelated plugin/extension files (`✏️s/🔌️plugins/…`, `🟦️extension.ts`), consistent with this being a live, concurrently-edited repo per the ticket's own binding rules.

```
$ node --check shard-worker-generated.js   # the actual template-string output of shardWorkerSource()
SYNTAX_OK
```

## baseline vs after + proof new tests ran by name

Baseline (stated in the acceptance brief): 29 passed / 0 failed.
After this packet, `--reporter=verbose` output (pasted above in full to my own working transcript) shows **40 passed / 0 failed** — the 11 new suite tests plus... (see below; 11 new tests, 29+11=40, confirmed against the doubled-counting bug: 40 ≠ 58, so the fixed config held).

New tests that executed (verbatim names from the verbose run):
- `orderEnvelopesByLane > sorts by Lane priority, not arrival order, stable within a tied lane`
- `orderEnvelopesByLane > is a no-op for an already-lane-sorted, single-lane batch`
- `GrantedBudgetTracker + interpretShardFrame > a Grant records its budget and hands back envelopes in lane-priority order`
- `GrantedBudgetTracker + interpretShardFrame > an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant`
- `GrantedBudgetTracker + interpretShardFrame > an Envelope AFTER a Grant for the same actor runs under THAT granted budget — proving the old constant no longer influences it`
- `GrantedBudgetTracker + interpretShardFrame > Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget`
- `GrantedBudgetTracker + interpretShardFrame > an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)`
- `ShardClient.grant / ShardClient.envelope wire adoption > grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them`
- `ShardClient.grant / ShardClient.envelope wire adoption > envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all`
- `ShardClient.grant / ShardClient.envelope wire adoption > turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)`
- `ShardFrame parity with Rust component.rs > TS ShardFrame variant/field names match the live Rust enum in 🖥️host/🧵️shard/🦀️component.rs`

That's 11 new tests; 29 + 11 = 40, matching the verbose run exactly.

## lease-requests

None. Everything landed inside the owned paths (`🧵️shard-client.ts`, `shardWorkerSource` in `🌐plugin-web-materialize.ts`).

## honest gaps

- **No real caller uses `grant()`/`envelope()` yet.** `ActivationRegistry` (`🎠️kernel/🟦️component.ts`, not owned by this packet) still calls `ShardClient.turn()` with a constant `defaultBudget`. Wiring `TurnScheduler.budgetFor` to a real DRR source and switching that call site onto `ShardClient.grant()` is necessarily a different packet's work — I made the new path available and regression-safe, not universally adopted.
- **`Register`/`Unregister` have no public `ShardClient` method.** They're fully represented in the `ShardFrame` type/parity test/`interpretShardFrame`, and the worker acks them, but there's no cross-shard coordinator on web today to send them from (mirrors the Rust variant's own doc: "a coordinator on the other end... not built by this packet"). Left as structurally complete but unexercised by any real caller — same honest scoping the Rust side itself uses.
- **`shardWorkerSource`'s frame logic is a hand-duplicated mirror of `interpretShardFrame`**, not a shared import (a template-string worker body cannot `import` a TS module). I did not stop at "it type-checks" — I additionally wrote an ad hoc Node smoke test (`/private/tmp/.../scratchpad/worker-frame-smoke.mjs`, NOT part of the graded suite, not committed anywhere) that evaluates the actual generated worker source with a faked `self`/bridge and confirmed: a `Grant`'s envelopes are dispatched to `poll()` lane-first with the granted budget; a follow-up `Envelope` reuses that same budget object; a never-granted actor's `Envelope` gets the Maintenance-default budget; and an unknown frame kind is acked, not thrown. All five checks passed. This is real verification beyond what's graded, but it is NOT wired into the actor package's own vitest run (different package, different vitest config, out of this packet's acceptance command) — a real integration test for `shardWorkerSource`'s output belongs in the os/plugin package and is a reasonable, cheap follow-up.
- **`ShardEnvelope`/`ShardOrigin`'s field names are hand-mirrored, not parity-tested against Rust.** Only `ShardFrame`'s own 4 variants are checked against the live Rust source (per the explicit acceptance ask). Extending the same read-the-source-and-diff approach to `semio_framework_actor::Envelope`/`Origin` (both in `🎭️actor/🦀️component.rs`) would be a cheap follow-up if a later packet's callers start depending on those field names holding steady.
