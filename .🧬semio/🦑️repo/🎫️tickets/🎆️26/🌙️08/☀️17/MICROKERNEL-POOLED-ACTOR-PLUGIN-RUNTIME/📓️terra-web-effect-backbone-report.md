# terra-web-effect-backbone report

Executor: terra-web-effect-backbone. Ticket: `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.
No cargo builds were run (Rust read-only, per binding rule 4). No git-modifying commands were run
(per binding rule 1). `ticket_close`/`ticket_reopen` never called (per binding rule 2, this ticket is
coordinator "sol"'s).

## delivered

New file (only owned source path): **`🧰️framework/🛍️products/💻️os/🟦️effect-backbone.ts`**.

- `MessageEndpoint`, `EffectSendMessage` (`Effect::SendMessage`), `EventMessage` (`Event::Message`) —
  hand-written TS mirrors of the live Rust kernel enums, field-for-field, machine-diffed against
  `🎠️kernel/🦀️component.rs` by an in-source parity test (`#region 🧬️ParityTests`) read fresh on every
  test run — the exact pattern `terra-web-shardframe`'s `ShardFrame` parity test established, applied
  here to a different Rust source file.
- `EffectBackbone` — the per-PLUGIN-INSTANCE class (never a module-level singleton): `registerEndpoint`/
  `send`/`dispatchSendMessage` (outbound, capability-gated), `subscribe`/`unsubscribe`/`fanoutDelta`/
  `deliverMessage`/`drain`/`nextRevision` (inbound), `dispose`. Built on `createBoundedMailbox`
  (`🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts`, already landed/reused, not reimplemented) for both
  the coalesced-delta queue and the lossless-send queue.
- `CapabilityChecker`/`AllowAllCapabilities`/`backboneCapabilityScope` — TS twin of Rust
  `CapabilityChecker`/`AllowAllCapabilities`.
- `BackboneError`/`BackboneSendOutcome`/`BackboneDispatchOutcome`/`DeliveryOutcome` — TS twins of Rust
  `BackboneError`/`PublishOutcome` (both in-process-only on the Rust side too, not wire types).
- `BackboneOverflowEvent`/`BackboneOverflowReporter`/`ConsoleBackboneOverflowReporter`/
  `RecordingBackboneOverflowReporter` — the "reject-and-report, never silently drop" contract, mirroring
  `🟦️backbone-worker.ts`'s own `rejectMutationQueueOverflow` shape (finding 5 in that file), plus a
  `RecordingMetrics`-style test double (Rust twin of that pattern too).
- `BackboneGuestMessage`/`encodeBackboneGuestMessage`/`decodeBackboneGuestMessage` — the base64
  host↔guest wire shape the Rust `BackboneRegistry` doc comment explicitly specifies "for the
  TypeScript counterpart."
- `BackboneWorkerLike`/`createBackboneWorkerTransport`/`bridgeBackboneWorkerInbound` — routes through
  `🟦️backbone-worker.ts`'s EXISTING `BackboneWorkerRequest`/`Response` wire (open/send,
  `publishPreview`/`preview`) instead of opening a second path to the hub. Neither `🟦️backbone-worker.ts`
  nor `🟦️component.ts` was edited.

Also touched: **`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️vitest.config.ts`** — added
`../../🟦️effect-backbone.ts` to both `includeSource` and `coverage.include` (the ONLY way a new
in-source-test file gets collected in this package's config, per the mission brief and that file's own
`🩹️` comment warning against listing it in `include` instead).

## Rust↔TS wire parity table

Read fresh off `🎠️kernel/🦀️component.rs` by the in-source parity test on every run (not hand-copied
once and left to drift) — 3 separate assertions, one per Rust source construct:

| Rust construct | Rust fields | TS shape |
|---|---|---|
| `enum MessageEndpoint` (5 variants, all camelCase-tagged, externally tagged) | `Shell{instance}`, `Backbone{uri}`, `PluginInstance{id}`, `Extension{id}`, `Topic{name}` | `{shell:{instance}}` \| `{backbone:{uri}}` \| `{pluginInstance:{id}}` \| `{extension:{id}}` \| `{topic:{name}}` |
| `Effect::SendMessage` | `target: MessageEndpoint`, `payload: Vec<u8>` | `{sendMessage:{target,payload}}` |
| `Event::Message` | `source: MessageEndpoint`, `payload: Vec<u8>` | `{message:{source,payload}}` |

`instance`/`id` stay plain `string` (this codebase's established actor/plugin-id stand-in, same
convention `🧵️shard-client.ts`'s `ShardFrame`/`GrantedBudgetTracker` already use) — not parity-tested
since Rust's own `PluginInstanceId` isn't a primitive the regex can diff against.

`BackboneError`/`DeliveryOutcome` (→ Rust `BackboneError`/`PublishOutcome`) are declared analogous but
**not** machine-parity-tested: neither Rust type derives `Serialize` (both are in-process-only, exactly
like `PublishOutcome`'s own doc note — "not on the wire"), so there is no wire contract to diff against,
only a naming-consistency choice.

`BackboneGuestMessage`'s `{kind:"send"|"delta", uri, payload, revision?}` shape is transcribed from the
Rust `BackboneRegistry` module's own doc comment (`⚡️effects/🦀️component.rs`, `#region 📡️EffectBackbone`,
"Wire shape for the TypeScript counterpart") — spec-conformant by inspection, **not** machine-diffed
(it's English prose in a doc comment, not code a regex can parse); flagged honestly in `## honest gaps`.

## overflow + loss semantics

Every backbone queue enqueues under a single `Lane` (`"Background"`) — deliberately the ONLY lane in
use, so a full queue can **never** cross-lane-evict something else (mailbox eviction only ever targets
a lower-priority nonempty lane; with exactly one lane populated there is none to evict). This makes
overflow always resolve to a clean `{kind:"rejectedFull"}`, never an incidental silent `droppedLane`:

- **Direct sends** (`deliverMessage`) are lossless: no coalesce key, so every message queues distinctly
  up to `sendQueueCapacity` (default 256, configurable). Once full, the NEXT message is rejected and
  reported through `BackboneOverflowReporter` — the two already-accepted messages stay put, in order
  (tested: `## delivered`'s overflow test asserts both surviving entries drain in original order after
  a third is rejected). This is the same "reject-and-report, never drop" contract
  `🟦️backbone-worker.ts`'s `rejectMutationQueueOverflow` already uses for its own outbound mutation
  queue (finding 5 in that file) — consistent by design, not by coincidence.
- **Deltas** (`fanoutDelta`) coalesce latest-wins per uri: every delta for the same `(actor, uri)`
  enqueues under the SAME coalesce key (the uri itself) into a dedicated capacity-1 mailbox, so a burst
  collapses to the latest — capacity 1 is provably sufficient (the first delta accepts, every later one
  replaces it in place), matching `backbone_delta_fanout_coalesces_a_burst_for_the_same_uri` exactly.
- `deliverMessage`/`fanoutDelta` against an actor that never subscribed to that uri resolves to
  `{kind:"noSuchSubscriber"}` (mirrors Rust `PublishOutcome::NoSuchSubscriber` from
  `EventRouter::send_message`/`publish`), never a crash and never a queued-then-lost message.

## state classification

Every field this file owns is **ephemeral-local-only** (CLAUDE.md: persisted-local-only /
persisted-shared / ephemeral-local-only / ephemeral-shared):

- `EffectBackbone.endpoints` (uri → transport) — in-memory only, rebuilt at plugin activation.
- `EffectBackbone.byActor`/`byUri` (subscription index) and their `BoundedMailbox` queues — in-memory
  delivery queues, never persisted; lost on reload by design (mirrors the Rust `EventRouter`'s own
  in-memory-only `Mutex<HashMap<...>>`).
- `EffectBackbone.revisions` (per-uri monotonic counter) — in-memory, reset on instance construction.
- `RecordingBackboneOverflowReporter`'s recorded events — diagnostic, in-memory, test-only in practice.

Nothing **persisted** or **shared** is introduced by this file. The only persisted/shared data any of
this touches lives entirely inside `🟦️backbone-worker.ts`'s own `ArtifactState` (already classified in
that file: `outbox`/`pendingMutations` etc.) — `createBackboneWorkerTransport`/
`bridgeBackboneWorkerInbound` only ever CALL INTO that pre-existing state via the existing
`BackboneWorkerRequest`/`Response` wire; they never duplicate or reclassify it.

## commands + exit codes

```
$ cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && bun ./📜️script.ts test --reporter=verbose
...
 Test Files  1 failed | 2 passed (3)
      Tests  1 failed | 206 passed (207)
   Start at  00:18:38
   Duration  1.08s (transform 1.94s, setup 0ms, import 2.29s, tests 280ms, environment 0ms)
EXIT_CODE=1
```
The 1 failure is `@semio-tech/framework-os workflow > matches the Rust plan_workflow across shared
fixtures decoded via wasm`, `Cannot find module '.../pkg/semio_framework_os.js'` — the SAME pre-existing,
unbuilt-wasm-artifact failure the acceptance brief's baseline names (one of the original two; the second
named failure, `decodes the Rust-generated binary wire fixtures byte-identically`, is already GREEN in
this live tree — traced to a concurrent session's fix, not this packet's work, consistent with binding
rule 6/the "live, concurrently-edited repo" context this ticket already documents elsewhere).

```
$ cd "/Users/ueli/Documents/semio" && npx tsc --noEmit -p tsconfig.json --skipLibCheck
EXIT_CODE=2
```
9981 total lines of pre-existing repo-wide output. `grep -i "effect-backbone"` on the output returns
exactly 5 lines, ALL `TS5097: An import path can only end with a '.ts' extension when
'allowImportingTsExtensions' is enabled` — the SAME pre-existing, repo-wide pattern (2025 total TS5097
occurrences across the tree, e.g. `.storybook/scopes.ts`, `compose/client/lib/sketchpad/js/index.ts`,
dozens more) that every other file using this codebase's own established "explicit `.ts` extension in a
relative import" convention already triggers under the ROOT tsconfig (which lacks
`allowImportingTsExtensions`) — `mailbox.ts`'s own `import type { Lane, CoalesceKey } from
"../../🤖️generated/🟦️actor.ts"` and `shard-client.ts`'s identical import both already do this too. Not a
regression: `grep -c "TS5097"` before this packet's edits would show the same class of error at every
OTHER `.ts`-suffixed relative import site in the repo; vitest/vite's actual bundler resolution (which is
what really matters — proven by all 22 new tests passing) does not enforce this TS-specific strictness.
No error of any OTHER kind appears against `effect-backbone.ts`.

## baseline vs after + proof tests ran by name

Baseline, measured fresh in this live tree just before writing any code (NOT copied from the acceptance
brief, which stated 184/2 — this tree had already moved to 184/1 by the time this packet started, a
concurrent-session fix unrelated to this work):
```
Test Files  1 failed | 1 passed (2)
     Tests  1 failed | 184 passed (185)
```

After this packet, `--reporter=verbose` shows **206 passed / 1 failed** (207 total) — 184 + 22 new = 206,
confirmed against the doubled-counting bug the coordinator's own config comment warns about (22 ≠ 44, so
the fixed `includeSource`-only config held).

All 22 new tests, verbatim names from the verbose run, all in `../../🟦️effect-backbone.ts`:
- `EffectBackbone capability gating > mirrors backbone_send_is_rejected_without_the_capability: send is rejected without the messaging.backbone:<uri> capability`
- `EffectBackbone capability gating > mirrors backbone_send_reaches_the_registered_transport_once_granted: send reaches the registered transport once granted`
- `EffectBackbone capability gating > send against an unregistered uri fails noSuchEndpoint even when granted`
- `EffectBackbone capability gating > dispatchSendMessage routes a Backbone target through send, and reports a non-Backbone target as the documented no-op gap`
- `EffectBackbone delta fan-out > mirrors backbone_delta_fanout_coalesces_a_burst_for_the_same_uri: a burst for the same uri collapses to the latest, not queued`
- `EffectBackbone delta fan-out > fanoutDelta only reaches actors subscribed to that specific uri`
- `EffectBackbone delta fan-out > fanoutDelta against a uri with no subscribers is an empty, error-free no-op`
- `EffectBackbone delta fan-out > nextRevision is monotonic per uri and independent across uris`
- `EffectBackbone queue overflow > a lossless direct-send queue rejects-and-reports once full, rather than silently dropping (consistent with backbone-worker's outbox contract)`
- `EffectBackbone queue overflow > deliverMessage against a uri actor never subscribed to is noSuchSubscriber, not a crash`
- `EffectBackbone per-instance isolation > an inbound Event::Message reaches only the subscribed actor, not other actors`
- `EffectBackbone per-instance isolation > two EffectBackbone instances for the SAME plugin never share endpoints or subscriptions`
- `BackboneGuestMessage wire shape > round-trips a send message through base64, matching the Rust doc's {kind,uri,payload} shape`
- `BackboneGuestMessage wire shape > round-trips a delta message with its revision, matching the Rust doc's {kind,uri,payload,revision} shape`
- `BackboneGuestMessage wire shape > refuses to encode a non-Backbone source`
- `createBackboneWorkerTransport / bridgeBackboneWorkerInbound > send lazily opens the document once, then posts publishPreview through the existing send request kind`
- `createBackboneWorkerTransport / bridgeBackboneWorkerInbound > send throws if used for a different uri than it was bound to`
- `createBackboneWorkerTransport / bridgeBackboneWorkerInbound > bridgeBackboneWorkerInbound turns an inbound preview event into a fanoutDelta reaching a subscribed actor, chaining any prior onmessage`
- `createBackboneWorkerTransport / bridgeBackboneWorkerInbound > bridgeBackboneWorkerInbound ignores non-event and non-preview responses without throwing`
- `EffectBackbone Rust↔TS wire parity > MessageEndpoint variant/field names match the live Rust enum in 🎠️kernel/🦀️component.rs`
- `EffectBackbone Rust↔TS wire parity > Effect::SendMessage fields match the live Rust variant in 🎠️kernel/🦀️component.rs`
- `EffectBackbone Rust↔TS wire parity > Event::Message fields match the live Rust variant in 🎠️kernel/🦀️component.rs`

That's 22 new tests; 184 + 22 = 206, matching the verbose run exactly. No real sleeps anywhere in the
new suite (confirmed by re-reading the file: zero `setTimeout`/`vi.advanceTimers`/`await new
Promise(...)` calls) — every assertion is synchronous state inspection.

## lease-requests

None. Everything landed inside the owned path (`🟦️effect-backbone.ts`) plus the one explicitly permitted
companion edit (`📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `includeSource`/`coverage.include`
arrays — additive, one line each, no existing entry touched). `🟦️backbone-worker.ts` and `🟦️component.ts`
were read (both already export everything this file needed: `ArtifactActorConfig`, `ArtifactActorMsg`,
`BackboneWorkerRequest`/`Response`, `encodeBackboneWorkerRequest`, `decodeBackboneWorkerResponse`,
`PersistenceBinding`) but never edited. `🎠️kernel/🦀️component.rs` and
`🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` were read-only (Rust, per binding rule 4).

## honest gaps

- **`createBackboneWorkerTransport`/`bridgeBackboneWorkerInbound` reuse `publishPreview`/`preview`**,
  the only generic, arbitrary-key-and-payload, EPHEMERAL pub/sub primitive
  `🟦️backbone-worker.ts`/`🟦️component.ts` already expose — not a dedicated backbone wire kind (adding
  one would require editing those files, which are outside this packet's owned/leased paths; a
  `lease-request` for a real `BackboneSend`/`BackboneDelta` `ArtifactActorMsg`/`ArtifactEvent` pair is
  the natural follow-up if `publishPreview`'s semantics prove insufficient in practice).
- **The worker bridge only reaches `EffectBackbone.fanoutDelta` (coalesced), never
  `deliverMessage` (lossless).** `publishPreview`/`preview` carries no "is this a direct send or a
  delta" distinction on the wire, so `bridgeBackboneWorkerInbound` cannot honestly route inbound worker
  traffic into the lossless queue — flagging this rather than silently mislabeling a `preview` event as
  a lossless send it structurally cannot guarantee (a hub-side `Preview` frame is itself explicitly
  ephemeral/collapsible, matching `fanoutDelta`'s own semantics far better anyway).
- **`createBackboneWorkerTransport` opening the SAME uri twice from two independent `EffectBackbone`
  instances sharing one physical `Worker`** (a realistic pooled-actor scenario) each independently posts
  its own `kind:"open"` for that `documentId` — whether `🟦️backbone-worker.ts`'s real `ArtifactState` map
  tolerates a duplicate open for the same `documentId` gracefully is unverified (that file was read-only
  this packet; its `openArtifact` handler wasn't traced end-to-end for this specific case). Each
  `EffectBackbone` instance's OWN state stays correctly isolated regardless (proven by the isolation
  tests) — this gap is specifically about a possible redundant/wasted `open` request on a shared
  transport, not about instance isolation.
- **No real `Worker`/browser was exercised.** Every worker-transport test uses a fake
  `BackboneWorkerLike` (structural `postMessage`/`onmessage`, no real `postMessage` serialization
  round-trip through structured clone or a real thread boundary) — appropriate for this packet's unit
  scope (mirrors how `createBackboneWorkerTransport`'s own Rust counterpart, `BackboneTransport`, is
  ALSO only ever exercised via a `RecordingTransport` test double in `⚡️effects/🦀️component.rs`'s own
  tests, never a real network socket), but a real end-to-end browser exercise remains a later
  integration concern, not verified here.
- **This file is not re-exported from `🟦️glue.ts`** (the `@semio-tech/framework-os` package's own `.`
  entry) — `glue.ts` is outside this packet's owned paths, so a future consumer must import
  `effect-backbone.ts` by relative path (same pattern this file itself uses for `mailbox.ts`) until a
  barrel export is added by whoever owns that lease.
- **`BackboneGuestMessage`'s wire shape is spec-conformant by inspection of an English Rust doc
  comment, not machine-parity-tested** the way `MessageEndpoint`/`Effect::SendMessage`/`Event::Message`
  are — there is no Rust CODE for a regex to diff against (the doc comment IS the entire spec on the
  Rust side; no Rust struct/enum implements this shape today). Flagged plainly rather than claimed as
  machine-verified.
