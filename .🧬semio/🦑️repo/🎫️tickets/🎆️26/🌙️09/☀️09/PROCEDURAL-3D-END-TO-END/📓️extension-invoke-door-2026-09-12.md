# 📥️ The Extension Invoke Door — `Event::Request` In, `Effect::Respond` Out, Both Halves Built

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "host half of the extension `request`/`respond` seam",
2026-09-12. Repo MCP was down all session (`invalid initialize params`); no ticket was opened, closed or
reopened. Evidence: `🗑️generated/invoke-door/` (`probe-1`, `probe-2`, `build-math-component.txt`,
`build-math-materialize.txt`, `dev-plugin-math.txt`, `cargo-plugin-lib.txt`). Nothing under any
`🗑️generated` folder was swept.

---

## 1. TL;DR — `meshes` is no longer 0

| hop | before | after |
|---|---|---|
| host `PluginWasmHandle.invoke` | did not exist → `extension.invoke-unavailable` on every evaluate | `Event::Request` submitted on the extension's own request actor |
| `wireEffectToFriendly` `respond` | no case; effect dropped as "unmapped" | decoded and routed by `req`, both arms |
| **guest** `Event::Request` | **`=> {}` — silently dropped, caller parked forever** | served from the installed `ExtensionBundle`, answered on the SAME turn |
| `flow-extension-math` `evaluate` | never entered | `status: ok, bytes: 199` |
| `flow-extension-brep` `evaluate` | never entered | `status: ok, bytes: 122 / 124` |
| `flow-extension-brep` `tessellate` | never entered | `status: ok, bytes: 393 / 131 / 2265` |
| `window:procedural-preview` `meshes` | **0** | **3**, `progress 44/44 units, 8/8 faces, ratio 1.0` |
| `window:procedural-main` node statuses | `profile: stale`, `extrusion-axis: stale`, `extrude: blocked` | **every node `ok`** |
| `pageerror` / `extension invocation refused` | present | **none** |

The previous lane's decode (`📓️extension-request-locale-2026-09-12.md` §3) was right that the host half
was missing. It was **half** the hole: the guest's turn loop matched
`Event::Request { .. } => {}` (`⚛️reactor/🔄️turn/🦀️.rs:690`), so even a perfect host door would have
parked forever. `grep -rn 'extension_invoke\b'` over the repo returned **zero runtime callers** — the
capability table (`plugin_runtime::extension_invoke`, `🔌️plugin/🦀️.rs:34722`) was reachable only from
four extension crates' own unit tests. Both halves are built here.

---

## 2. What the door is

### 2.1 One protocol, one implementation, two renderer targets

`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` gains the whole protocol:

- **`wireRespondAnswer(effect)`** — decodes `respond-effect` (`📜️.wit:496`, `{ req, outcome:
  respond-result }`) without narrowing its u64 identity. The outcome keeps the WIT arm names
  (`ok`/`fault`) rather than Rust's `RequestOutcome::{Ok,Err}`, because this is the wire the host reads
  AND it is byte-for-byte the shape `captureExtensionCompletion.complete` already takes — one
  vocabulary for both directions of the same door.
- **`wireEffectToFriendly` learns `case "respond"`** — in BOTH copies (`🖼️wire-turn.ts`'s own and
  `🔌️PluginRuntime/🟦️.tsx`'s inline one), the PluginRuntime copy delegating to the shared helper so
  the "third divergent copy" that module's header warns about does not happen.
- **`wireTurnStatusTag`** moved here from `PluginRuntime` (it is a wire concern, and the drain loop
  needs it); `PluginRuntime` imports and re-exports it, so every existing call site is untouched.
- **`driveInboundRequest(drive)` + `INBOUND_REQUEST_TURN_BUDGET = 64`** — submits ONE `request` event,
  then drains EMPTY turns while the guest still reports `more-work`, scanning each turn's effects for a
  `respond` whose `req` matches. Returns a discriminated
  `{status: "answered"|"cancelled"|"unanswered"}`; fault VOCABULARY is deliberately not chosen here, so
  each target maps it onto its own typed refusal. `submit` is injected and typed on the two fields the
  protocol reads (`InboundRequestTurn = {effects, status?}`), not on `WireTurnResult` — the two targets
  carry structurally different turn records.

Cancellation and progress are read at **turn boundaries only**: a turn already handed to the worker is
never half-abandoned.

### 2.2 The React handle

`🔌️PluginRuntime/🟦️.tsx`:

- `PluginWasmHandle.invoke(capability, request, context?)` — new, returned by `loadPluginModule`.
- `PluginExtensionInvokeContext` — `originInstanceId` (becomes the request's
  `message-endpoint::shell` origin; `0` = the shell itself, owning no instance of the callee),
  `signal`, `onProgress`.
- **The callee's actor is this handle's own request actor** (`${pluginId}#request`), activated lazily
  on the first call through `registry.activate` and retired with the handle
  (`retireRequestActor`, folded into `dispose`'s `Promise.allSettled`). An extension program declares
  no app, so it never gets a `createApp` instance — and `Event::Request` needs none: the guest's turn
  loop serves it out of the actor-scoped `ExtensionBundle`, and `poll_kernel`'s lifecycle focus is
  simply `None`. A program nobody calls into costs no worker residency at all.
- Size gates on BOTH directions: the request is refused past
  `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` (64 KiB) with `extension.request-too-large` — it crosses as
  ONE contiguous `pack` and a block the guest allocator refuses is an `abort`, not a fault
  (`🧮️memory/🟦️.ts`'s own header) — and the answer past `GUEST_HOST_ANSWER_CEILING_BYTES` with
  `extension.answer-too-large`.
- The `fault` arm is decoded (`decodeFaultFromWire`+`decodePackValue`) and rethrown as a
  `SemioFaultError` carrying the GUEST's own fault, so `runCapturedExtensionEffect`'s existing
  typed-refusal branch stays the one door.
- `serializeCommandIngressForActor` wraps the whole drive; the captured `ShardActorActivationLease` is
  asserted before every submitted turn and after the answer.

### 2.3 ShellHost

`runCapturedExtensionEffect` now reads `const { invoke } = extensionEntry.handle` (typed, not a
duck-typed cast) and calls it with `{ originInstanceId: instanceId }`. Its
`extension.invoke-unavailable` branch is **unreachable for a loaded extension** and survives only for a
handle that genuinely has no door (a test double, or a target that never adapted one) — and
`extension.missing` for a genuinely absent extension is untouched. `raw` is now `Uint8Array`, so the
`typeof raw === "string"` tolerance is gone: the guest answers in the capability's own encoding
(`evaluate_invoke_json` emits JSON text) and the ONE re-encoding into a `pack` happens on the
completion, exactly where `📓️extension-result-realloc-2026-09-10.md` §4.5 put it.

### 2.4 The wgpu bridge — one door, not a second one

`🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts` gains the identical `invoke` (plus
`WgpuPluginInvokeContext`, the lazy request actor, and its retirement in `dispose`), built on the SAME
`driveInboundRequest`. `dispatchInvokeExtension` reads `invoke` off the typed `WgpuPluginHandle`
instead of an `as { invoke?: … }` cast.

### 2.5 The answer path is unchanged

The completion still crosses through the SAME paged contract `captureExtensionCompletion.complete`
already used (`guestAnswerPages` + `GUEST_HOST_ANSWER_CEILING_BYTES`, terminal page on the `completed`
event). Nothing in `📓️extension-result-realloc-2026-09-10.md` §4.2's paging was touched — the runtime
lines below show `pages: 1` for every answer, all under one 64 KiB page.

---

## 3. The guest half (found here, not in the brief)

`⚛️reactor/🔄️turn/🦀️.rs` — `Event::Request { .. } => {}` becomes:

```rust
Event::Request { req, capability, payload, .. } => {
    let result = match crate::plugin_runtime::extension_invoke(&capability, &payload).await {
        Ok(answer) => RequestOutcome::Ok(answer),
        Err(fault) => RequestOutcome::Err(store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&fault)?)),
    };
    inbound_request_effects.push(Effect::Respond { req, result });
}
```

Three deliberate choices:

1. **Served inline, answered on the SAME turn.** An `ExtensionBundle` handler is a pure
   `Fn(&[u8]) -> Result<Vec<u8>, Fault>` with nothing to await, so the ABI's "within a bounded number of
   turns, or by spawning a job" is satisfied at turns = 1. The host's 64-turn budget exists so a guest
   that parks a request instead of answering it fails loudly at the caller rather than leaving an
   outstanding completion forever.
2. **An actor with no bundle answers the bundle's own typed refusal** (`extension.inactive` /
   `extension.missing` / `extension.unknown-capability`), never silence. The procedural plugin itself is
   such an actor; it is never sent a request, and if it were it would refuse by name.
3. **The fault arm is a `pack`** (`store::pack_rt::encode_wire_value` of `dsl::to_dsl_value(&fault)`) —
   the shape `decodePackValue`/`decodeFaultFromWire` read on the host, matching
   `shell_fault_effect`'s own encoding rather than `dsl::encode_fault_bytes`' JSON.

`inbound_request_effects` is kept apart from `document_backbone_effects` and appended AFTER them, so a
request that also wrote the document publishes the write first — explicit ordering, not incidental.

---

## 4. Tests — all run, results verbatim

### 4.1 One language-neutral fixture, two implementations, third-party oracle

New fixture `🔌️plugin/⚛️reactor/🧫️fixtures/📥️inbound-request/🔣️.json` (seam shape + five rows:
`inactive`, `ok`, `empty-answer`, `handler-fault`, `unknown-capability`) and its strict schema
`🔌️plugin/⚛️reactor/🧬️schema/🔣️.json`.

| implementation | command | result |
|---|---|---|
| Rust (the real `poll_kernel` turn loop, real `ExtensionBundle`) | `RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib -- --test-threads=1 inbound_request` | `test component::reactor::turn::inbound_request_tests::every_inbound_request_row_is_answered_on_the_turn_it_arrives ... ok` — **1 passed / 1** |
| TypeScript (`driveInboundRequest` + the real `runInvokeExtensionEffect`) + **Ajv** as the independent oracle | react vitest, `🧪️tests/📥️inbound-request` | **16 passed / 16** |

The TS suite pins: the Ajv schema check; that the fixture's declared budgets equal
`INBOUND_REQUEST_TURN_BUDGET` / `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` /
`GUEST_HOST_ANSWER_CEILING_BYTES`; every row answered on turn 1 with the exact event field names
(`req`/`params`, `origin`/`capability`/`payload`) and `origin = {tag:"shell", val: originInstanceId}`;
`more-work` draining with per-turn progress; cancellation stopping at turn 3 of a 64-turn budget rather
than draining it; a quiet guest reported `unanswered`; a `respond` answering someone ELSE's `req`
ignored; and both arms of `wireRespondAnswer` plus its two rejections.

### 4.2 The `🔁️extension-invocation-wire` rows, now end to end

The brief's requirement. `🧪️tests/📥️inbound-request/🟦️.ts`'s second block builds an extension entry
whose `invoke` IS `driveInboundRequest` over a guest emulation — a REAL door, not a stubbed `invoke` —
and drives all four rows (`ok`, `empty-output`, `operator-fault`, `bad-request`) through
`runInvokeExtensionEffect` to exactly one completion each, asserting the completion's arm and decoded
payload. **4 passed / 4**, inside the 16 above.

The pre-existing twin of those rows in `🧪️tests/🔬️engine-contract/🟦️.ts` was corrected in place: its
`invoke` doubles now return BYTES (`extensionAnswerBytes`, a new helper with its own docstring) instead
of JS strings, and the two `toHaveBeenCalledExactlyOnceWith` assertions carry the new
`{ originInstanceId }` argument. `bunx vitest -t "extension"` → **30 passed**.

### 4.3 Regression corpora

| corpus | command | result |
|---|---|---|
| React engine, full | `SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts` | **937 passed, 10 failed** — see §6.1, none mine |
| `@semio-tech/framework-actor` (owns `🖼️wire-turn.ts`) | `bunx vitest run --config vitest.config.ts` | **219 passed, 12 failed** — all 12 are pre-existing Ajv `$defs/NonZeroU64` resolution failures in `🚪️lifetime`/`📤️return`/`🪪️activation`, files this lane never touched; `🖼️wire-turn.ts`'s own in-source suite is green |
| `semio-framework-plugin`, full lib | `cargo test -p semio-framework-plugin --lib -- --test-threads=1` | **525 passed, 144 failed** — see §6.2, none mine |
| `semio-framework-plugin`, check | `cargo check -p semio-framework-plugin --lib` | `Finished` with 5 warnings (warnings present ⇒ expansion completed) |
| TypeScript | `bunx tsc --noEmit -p tsconfig.json` (react target) | zero errors in any file this lane wrote or edited; the pre-existing errors in `PluginRuntime`/`ShellHost`/`plugin-bridge` are unchanged in count and location |

---

## 5. Runtime on 6018 — quoted

`SEMIO_PROBE_SECONDS=240 SEMIO_PROBE_OUT=invoke-door/probe-2 bun 🐍️console-dump-probe.mjs`
(`🗑️generated/invoke-door/probe-2/`). Host TypeScript is live on reload; `flow-extension-math` and
`flow-extension-brep` were rebuilt for the guest half (§7).

```
25549 [DEBUG] extension request answered {pluginId: flow-extension-math, capability: evaluate, req: 1, turns: 1, bytes: 194}
25549 [DEBUG] extension completion submitted {instanceId: 1, req: 1n, status: ok, bytes: 199, pages: 1}
51513 [DEBUG] extension request answered {pluginId: flow-extension-brep, capability: evaluate, req: 1, turns: 1, bytes: 121}
51513 [DEBUG] extension completion submitted {instanceId: 1, req: 2n, status: ok, bytes: 122, pages: 1}
74968 [DEBUG] extension request answered {pluginId: flow-extension-brep, capability: evaluate, req: 2, turns: 1, bytes: 123}
74968 [DEBUG] extension completion submitted {instanceId: 1, req: 3n, status: ok, bytes: 124, pages: 1}
99975 [DEBUG] extension request answered {pluginId: flow-extension-brep, capability: tessellate, req: 3, turns: 1, bytes: 363}
99975 [DEBUG] extension completion submitted {instanceId: 1, req: 4n, status: ok, bytes: 393, pages: 1}
113758 [DEBUG] extension request answered {pluginId: flow-extension-brep, capability: tessellate, req: 4, turns: 1, bytes: 116}
113758 [DEBUG] extension completion submitted {instanceId: 1, req: 5n, status: ok, bytes: 131, pages: 1}
118972 [DEBUG] extension request answered {pluginId: flow-extension-brep, capability: tessellate, req: 5, turns: 1, bytes: 2236}
118973 [DEBUG] extension completion submitted {instanceId: 1, req: 6n, status: ok, bytes: 2265, pages: 1}
```

Every completion is `status: ok` with `bytes > 0`, every answer arrives on `turns: 1`, every one fits
`pages: 1`. Zero `pageerror`, zero `[DEBUG] extension invocation refused`, zero
`extension.invoke-unavailable`.

`hosts.json`:

```json
{"surfaceId":"window:procedural-preview","meshes":3,
 "status":{"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
           "progress":{"unitsDone":44,"unitsTotal":44,"facesDone":8,"facesTotal":8,"inFlight":0,"ratio":1.0},
           "cancellable":false,"cancelAction":"cancelPreviewEval",
           "debug":{"evalLen":1042,"meshesLen":3641,"instancesLen":695}}}
{"surfaceId":"window:procedural-main","meshes":0,
 "status":{"height":{"status":"ok"},"radius":{"status":"ok"},"sides":{"status":"ok"},
           "profile":{"status":"ok"},"extrusion-axis":{"status":"ok"},"extrude":{"status":"ok"},
           "column-preview":{"status":"ok"}}}
```

The evaluation SETTLES: no `fault` key at all (the `flow.extension-evaluate-failed` /
`extension.invoke-unavailable` object the previous lane published is gone), `ratio 1.0`, `inFlight 0`,
44/44 units and 8/8 faces. `probe-2/final.png` shows the hexagonal column rendered in the Preview
window with every node in the Flow tree reading **Evaluated**.

An earlier 90 s probe (`probe-1`) caught the math evaluate and one brep evaluate and already read
`meshes: 1`; the 240 s window is what the three tessellate hops need (one evaluate→tessellate round is
~12-25 s of guest work on this build).

`window:procedural-main`'s `meshes: 0` is not a hole: that surface is the node-graph editor and
publishes no mesh list — its own status object is all-`ok`.

The view-context lane's `view context: invalid panel data` rejection did NOT appear on 6018 during
either probe, so no workaround was needed and the view-state cap was not touched.

---

## 6. Not mine, observed, attributed

### 6.1 React engine: 10 failures, all pre-existing or peer-owned

Nine are the exact set the previous lane recorded (`📓️extension-request-locale-2026-09-12.md` §6.3):
6 × `🧩️package-integration` worker bytes, `noteShellCommand`, `surface render ViewModel`,
`readAppDocumentPack`. The tenth, `🔀️surface-switch`, appeared BETWEEN two runs of mine 30 minutes
apart: `TypeError: ports.quiesce is not a function` at
`🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts:217`, a file mtime-stamped **04:56** — after my last
edit (04:29) — by the peer that owns it. The test double has not grown the `quiesce` port the
implementation started calling. Not touched by this lane (`invoke`/`respond` appear nowhere in it).

### 6.2 `semio-framework-plugin` lib: 144 failures, a live peer refactor

`grep -c "respond\|Event::Request"` over the failure log: **0**. Every one belongs to four families of
another lane's in-flight work — `app-definition.interactive-job-classification: unclassified
interactive command`, `interactive-job.missing-factory: typed command … has no exact
controller/owner/factory/tool/schema proof`, `artifact store reached Drop without its exact
terminal-empty shallow-shell witness`, and `app-definition.invalid: app id testkit-txn must be a
canonical surface id`. The assertion that raises the first lives in
`🔌️plugin/🦀️.rs:5662`, a file mtime-stamped **04:59** — after my last edit and never touched by this
lane. My own law passes on that same tree (§4.1).

### 6.3 `nx materialize-dev` does NOT reach the served module directory

The brief's suggested `component-dev` + `materialize-dev` pair builds and transpiles correctly, but
`materialize` stages into `🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/…` while the
6018 dev server serves `🧑‍💻dev/🔌️plugin-modules/` (`PLUGIN_MODULES_ROOT`,
`🔌️vite-plugins.ts:22`) and the catalog installs from `🧑‍💻dev/🧩️extension-modules/`
(`publishBuiltExtension`). After a clean `materialize-dev` the served `*.core.wasm` was still two days
stale, with no warning anywhere. The path that actually publishes is
`@semio-tech/framework-os-dev:plugin <filter>` (`buildPlugin` → `materializePlugin` →
`publishBuiltExtension`). Worth either teaching `materialize-dev` to publish or making the staleness
loud — it reads exactly like "the rebuild didn't take". Extends
`[[project-component-release-does-not-materialize]]`.

---

## 7. What was rebuilt

`bunx nx run @semio-tech/framework-os-dev:plugin --args="flow-extension-math"` (foreground, 3 m 30 s).
The filter matched the flow-extension family, so ten extension crates were rebuilt and republished;
the two this lane needed are:

```
built program flow-extension-brep (wasm32-wasip2, wasm-dev) -> …/🧑‍💻dev/🔌️plugin-modules/🧊️flow-extension-brep
published extension flow-extension-brep -> /🧩️extension-modules/🧊️flow-extension-brep/🌉️bridge.js
built program flow-extension-math (wasm32-wasip2, wasm-dev) -> …/🧑‍💻dev/🔌️plugin-modules/🧮️flow-extension-math
published extension flow-extension-math -> /🧩️extension-modules/🧮️flow-extension-math/🌉️bridge.js
```

They needed it because §3 changed the shared framework turn loop, which every guest links. **The
procedural plugin was NOT rebuilt** (the filter never matched it) and **no dev server was started or
restarted**. Earlier in the lane `@semio-tech/flow-extension-math-rust:component-dev` and
`:materialize-dev` were also run (logs retained) — see §6.3 for why they were not sufficient.

---

## 8. Files

**New**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧫️fixtures/📥️inbound-request/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️inbound-request/🟦️.ts`

**Changed — host (TypeScript, live on reload)**
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` (`wireRespondAnswer`, `respond` case, `wireTurnStatusTag`, `driveInboundRequest`, `INBOUND_REQUEST_TURN_BUDGET`)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (`Effect.respond` typed: `req: bigint`, `result: {ok}|{fault}`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (`invoke`, `PluginExtensionInvokeContext`, request actor + retirement, `respond` case, `wireTurnStatusTag` now imported)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (`runCapturedExtensionEffect` uses the typed door and passes the origin)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` (`invoke`, `WgpuPluginInvokeContext`, request actor + retirement, `dispatchInvokeExtension` typed)

**Changed — guest (Rust; the two flow extensions were rebuilt, §7)**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` (`Event::Request` served and answered, `inbound_request_effects`, test module declaration)

**Changed — tests / config**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (`extensionAnswerBytes`, byte-returning doubles, origin argument)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` (registers the `📥️inbound-request` suite)

No `launch.json` entry was added: the new suite runs inside the existing
`@semio-tech/framework-renderer-react:test-long` target, which is already registered.

---

## 9. Next blocker (none for this lane's goal)

`meshes` reaches 3 and the preview settles, so the seam is no longer the blocker. Two adjacent items a
follow-up should decide on:

1. **`phase`/`phaseLabel` still read `idle`/`Idle` on a settled evaluation** with `ratio 1.0` and
   `unitsDone == unitsTotal`. That is the guest-side projection the previous lane corrected natively and
   flagged as "one restage owed" (`📓️extension-request-locale-2026-09-12.md` §4.2); it needs a restage
   of the **procedural** plugin, which this lane was forbidden to build.
2. **A parked request has no host-side cancel route yet.** `driveInboundRequest` honours an
   `AbortSignal` and `PluginWasmHandle.invoke` accepts one, but `runCapturedExtensionEffect` passes
   none — `cancelPreviewEval` reaches the guest's own evaluation, not an in-flight extension request.
   Harmless while every capability answers on turn 1; it is the wire to pull the first time one spawns a
   job instead.
