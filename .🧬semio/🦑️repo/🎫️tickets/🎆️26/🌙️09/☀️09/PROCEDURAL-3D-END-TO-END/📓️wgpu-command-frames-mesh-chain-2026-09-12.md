# ⛓️ wgpu COMMAND FRAMES → MESH CHAIN — one shell endpoint, two wire languages, and the chain that follows

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu command frames + mesh chain", 2026-09-12.
Resumes `📓️wgpu-raster-witness-effects-2026-09-12.md` §7 (`decodeAppFrame: unknown tag 115`).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. Evidence under
`🗑️generated/wgpu-chain/run-1 … run-11`; nothing under any `🗑️generated` folder was swept. The react
serve on 6018 was not touched, the procedural guest was NOT rebuilt or restaged, and no git-state
modifying command was run.

---

## 1. TL;DR

| question | answer |
|---|---|
| **1 — what was `unknown tag 115`?** | **A framing error, and neither of the brief's two hypotheses.** `115` is `'s'` — the first byte of `semio.typed-operation-page.v1\0`. The guest multiplexes a mounted typed operation's RESULT PAGES onto the very `Effect::SendMessage{Shell{instance}}` endpoint that carries `AppFrame` replies, and the wgpu host claimed every payload on that endpoint as a frame (§2). The Rust/TS tag tables were already in sync (§2.1). |
| **1 — fixed where?** | At the one demux both renderer targets read (`shellFrameBytes`, `🖼️wire-turn`), which now discriminates by magic instead of relying on an ordering rule; React's private copies of the page codec are deleted and point here (§3). Pinned by a neutral fixture driven by a Rust law on the NATIVE host's own demux (2/2) and a TS twin (5/5), plus a wgpu-bridge vitest for the multi-frame reply (6/6) (§6). |
| **2 — does the chain reach the extensions?** | **Yes, and it converges.** `http://127.0.0.1:6118/?plugin=generation3d`: **5 `flowEvalTick` commands, 6 extension round trips — 1 `flow-extension-math evaluate`, 2 `flow-extension-brep evaluate`, 3 `flow-extension-brep tessellate` — every one `status: ok`, the last carrying 2265 B of mesh payload**, zero faults, zero `invokeExtension faulted`. That is byte-for-byte the shape `📓️tick-arming-latch` §4 derives for the NATIVE chain (§8). |
| **2 — how many more hops were missing?** | **Six**, each root-caused and fixed (§4): the typed-operation acknowledgement/drain the host owed the guest; a drain that stole a host call's effects; a `bigint` that two of four bridge verbs refused to serialize; a `dispatch_command` that dropped every effect it did not name, `InvokeExtension` included; a detached extension task that threw its answer away; and a fixed `UiValue` arena the browser host never returned. |
| **3 — is the hexagonal column on the wgpu World3d surface?** | **No, and it is not claimed.** One hop remains, named with console evidence in §8: the wgpu dock never LAYS OUT the `procedural-preview` window instance, so no World3d engine surface exists in the frame for the meshes to reach. The Flow window paints (`drawCalls 2, quadCount 625, glyphCount 580`, 34 arena nodes, all 7 flow nodes and 6 wires, `Evaluated` on `Vector`/`ExtrudeCurve`/`Preview`) and `procedural-preview` renders its document every round — it is simply not in the dock's layout. |

---

## 2. Deliverable 1 — the framing error, root-caused

### 2.1 Both of the brief's hypotheses are disproved, on the source

**Hypothesis A — a peer added Rust `AppFrame` variants ahead of the TS table.** No. Both tables run
`0…26` with the same names, `DocumentArchive: 26` included, and `APP_COMMAND_TAGS` already carries the
peer's `LoadDocumentArchive: 32`/`ReadDocumentArchive: 33`:

```
🧰️framework/🛍️products/💻️os/🟦️.ts:2674            APP_FRAME_TAGS  Done 0 … DocumentArchive 26
📡️spr/🧵️channel/🦀️.rs:2986 encode_app_frame       out.push(0) … out.push(26)   — same 27, same names
```

**Hypothesis B — the reply stream is split at the wrong boundary.** Also no: nothing splits. Each
`Effect::SendMessage{Shell}` payload is ALREADY one whole frame (`route_app_frame` pushes one effect
per frame), and `AppChannelClient.pumpOutcomes` decodes each `outcome.frames[i]` independently.

### 2.2 The actual cause: one endpoint, two wire languages

`route_exchange_output` (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1443`) emits BOTH onto the same endpoint:

```rust
if let Some(page) = output.typed_operation_result.as_ref() {
    effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { .. }, payload: page.renderer_exchange_bytes() });
}
for frame_bytes in output.frames { route_app_frame(instance, &frame_bytes, effects); }
```

`renderer_exchange_bytes` opens with `TypedOperationResultPage::RENDERER_PAGE_MAGIC =
b"semio.typed-operation-page.v1\0"`. Its first byte is `0x73` = **115**. `flowEvalTick` is a `Migrated`
interactive job: it mounts a typed operation, so its reply carries pages beside its `Invocation`
frame — and the wgpu bridge handed all of them to `decodeAppFrame`.

**The NATIVE host already discriminates** (`🧊️renderer/🦀️.rs:7548`, `apply_turn_result`):
`decode_guest_message` first, `decode_app_frame` second. **React survives by an ORDERING accident**:
`consumeTypedOperationEffects` runs inside `settlePluginTurn` and strips the pages before
`shellFrameBytes` ever sees the turn. The browser wgpu host has neither. That is the defect.

### 2.3 A second, quieter defect on the same wire

The native `decode_guest_message` capped its lane check at `lane > 12`, while the guest's own emitter
writes `WindowTransient = 13` and `WindowConfig = 14` — **lane 13 is the mesh publication lane.** A
window-transient page therefore failed that demux, fell through to `decode_app_frame`, failed there
too, and was silently carried out as an unparsed effect. The neutral fixture agreed with the bug
(`"invalidTags": [13, 255]`), so nothing caught it. Both are corrected here.

---

## 3. Deliverable 1 — the fix, at the owning layer

`🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` (the ONE demux both renderer targets import) gains
region `📬️TypedOperationPage`:

* `TYPED_OPERATION_PAGE_MAGIC` / `_ACK_MAGIC` / `_PAGE_HEADER_BYTES` / `_TOKEN_BYTES` /
  `_RESULT_PAGE_BYTES` / `_RESULT_LANE_MAX` / `_LANE_TERMINAL` / `_LANE_FAULT`.
* `typedOperationResult(effect)` — the page codec, twin of `decode_guest_message`/`renderer_exchange_bytes`.
* `typedOperationAcknowledgements(turn)` — the ACKs a turn's pages owe.
* `scanTypedOperationPages(effects)` → `{kept, pages, acknowledgements, terminal, faults}` — a PURE
  scan that names a fault and a terminal without choosing a policy for either, so React's
  owner-attributing router and the wgpu ladder read one decoder rather than two.
* `drainTypedOperationTurns(...)` — moved here from `PluginRuntime` (it was already injected and pure).
* **`shellFrameBytes` declines a payload bearing either magic**, and a new total
  `shellMessageKind(effect, instanceId) → "app-frame" | "typed-operation-page" | "typed-operation-ack" | null`
  makes the discrimination a verdict rather than an ordering rule. A third language added to this
  endpoint now has to name itself here.

`🔌️PluginRuntime/🟦️.tsx` deletes its private `shellFrameBytes`, `TYPED_OPERATION_*_MAGIC`,
`TYPED_OPERATION_RESULT_LANE_MAX`, `typedOperationResult`, `typedOperationAcknowledgements` and the
body of `drainTypedOperationTurns`, and imports all of them. Its policy (`TypedOperationRouter`,
`consumeTypedOperationEffects`) stays where it is — that is renderer-owned, the codec is not.

`🧊️renderer/🦀️.rs` names its ceiling `TypedOperationResultPage::LANE_MAX = 14` and checks against it.

---

## 4. Deliverable 2 — six more hops, each measured

Every fix below moved the failure exactly one hop further, on a rebuilt bundle each time.

### 4.1 The host never acknowledged the pages it collected (run-1)

The guest PARKS a mounted operation until each result page is acknowledged over the same
`Shell{instance}` channel. The wgpu bridge collected none. `WgpuTypedOperationDrive`
(`🐚️plugin-bridge/🟦️.ts`) is the whole obligation in one type: scan a turn once, keep its non-page
effects, hand back the ACKs the next submission owes, name a fault page, and never observe a turn
twice. `runQueuedTurn` now carries the ACKs into every submitted turn, settles while the drive still
owes one, and — when the turn ends `more-work` — starts `drainTypedOperations`, the wgpu twin of
`PluginRuntime`'s standing poll, on the shared `drainTypedOperationTurns`.

```
run-1:  unknown tag 115 → 0        typed-operation drain polls=4 stopped=idle pages=3 terminal=true
```

### 4.2 …and then the drain stole the host call's own effects (run-1 → run-2)

`submitActorWork` only keeps two TURNS from overlapping. A drain poll slipping between two of a
render's turns collected that render's effects, so the `dispatchAction` the guest armed for the host
was reported as `effects=0` and the chain stopped before it started:

```
run-1:  renderSurface surface=procedural-main turn=1 effects=0 tags=-          ← regression
```

`serializeWgpuActorCall` adds per-actor serialization at CALL granularity — React's
`serializeCommandIngressForActor` — held by `runQueuedTurn`, `renderSurface`, the extension-completion
turn and each drain poll. `renderSurface` additionally CLAIMS the instance's leftover ledger when it
starts, so an effect a drain poll parked between two host calls still reaches the shell:

```
run-3:  renderSurface surface=procedural-main turn=0 effects=1 carried=1 tags=dispatchAction
```

### 4.3 `JSON.stringify` refuses a `bigint`, and two of four verbs had no replacer (run-2)

```
run-2:  wgpu-shell deferred action flowEvalTick failed: handleCommand promise failed:
        Do not know how to serialize a BigInt
```

`pluginHandleForBridge` projected the SAME `InvocationResponse` two different ways:
`dispatchInvokeExtension`/`pushScopedContributions` carried a bigint replacer, `handleAction`/
`handleCommand` used a bare `JSON.stringify`. The first command whose reply requested an
`invokeExtension` (`req` is a `u64`) died on it. One exported `invocationResponseJson`, all four verbs.

### 4.4 `dispatch_command` dropped every effect it did not name (run-3)

```
run-3:  wgpu-bridge effects leftover 1 tags=invokeExtension      ← and nothing after it
```

`ShellState::dispatch_command` hand-rolled a SECOND, shorter effect fold beside
`queue_host_effects` — and `InvokeExtension` was not in it. React has one funnel:
`makeEffectDispatchOne` hands both `handleAction` and `handleCommand` results to `applyHostEffects`.
`dispatch_command` now does the same; only the two effects `queue_host_effects` cannot own (a
`Navigate` that must also RESOLVE its uri, and the native replay relay — both `async`, and that funnel
is deliberately a plain `fn`) stay in the caller.

```
run-4:  wgpu-shell invokeExtension dispatch extension=flow-extension-math capability=evaluate req=1
        wgpu-bridge extension request answered {…, turns: 1, bytes: 333}
        wgpu-bridge extension completion submitted {…, status: ok, bytes: 295, pages: 1}
```

### 4.5 The extension answer was thrown away (run-4)

The `InvokeExtension` arm fired a DETACHED `crate::spawn_app_task` and discarded the returned
`InvocationResult` — so the `flowEvalResolve` that answer arms never reached the shell and the chain
stopped after exactly one evaluate. React's `dispatchInvokeExtensionEffect` publishes the response
through `applyHostEffects`.

The arm now PARKS a `PendingExtensionInvocation`, and `flush_deferred_actions` became ONE convergence
loop over the two halves of a guest chain — the actions an effect deferred, and the extension calls
those actions asked for — bounded by `SHELL_DEFERRED_CHAIN_ROUNDS = 512`.
`run_extension_invocation` folds each answer's own `requested_effects` back through
`queue_host_effects` and applies its mutations.

```
run-5:  six round trips, all ok — math evaluate ×1, brep evaluate ×2, brep tessellate ×3
```

### 4.6 A fixed `UiValue` arena the browser host never returned (run-5 → run-9)

The converged chain then stopped the whole shell painting:

```
run-7:  renderDocument result parse failed: data did not match any variant of untagged enum UiValue
        at line 1 column 17497  headroom collections=0 items=139
        near …"args":{"kind":"neuron|brep.eval.curveDomain"},"capability":null}…
```

Two additions made that legible and they stay: the parse error now carries a 240-char window of the
payload around the column serde refused, and `ui_value_headroom()`. **`collections=0`** is the whole
story: every `UiList`/`UiMap` in a published node is a handle into a FIXED process-wide arena, and
`UiValue`'s untagged enum fails EVERY variant once the arena refuses an admission.

Two holes, both fixed:

* `render_with_document_js` parsed a document's records and dropped them without ever running the
  `UiValue` retirement pump — the reactor drives exactly that pump after its own patch intake, and
  this host drove only the DOCUMENT pump. `retire_browser_ui_values()` now returns them per render.
* `refresh_ui` retired its previous documents into `closing_documents`, which `render_chrome_step`
  drains **one page per FRAME**. A converging chain runs many refreshes back to back inside ONE
  `flush_deferred_actions` and no frame happens in between, so the registry only grew.
  `drain_retained_document_arenas()` returns the previous refresh's documents AND their `UiValue`
  pages before the shell asks the guest for the next one.

```
run-9:  nodeCount 34, drawCalls 2, quadCount 625, glyphCount 580, alert null — painting again,
        with the full six-round-trip chain in the same run
```

---

## 5. Deliverable 2 — the neutral oracle

`🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json` is extended
from a bare lane list into the whole exchange contract, and is read by three implementations:

* `page` — the two magics (` `-terminated, exact), `headerBytes 30`, `tokenBytes 25`,
  `laneOffset 25`, `lengthOffset 26`, `maxPayloadBytes 4096`, and `pageMagicFirstByte: 115` — the
  reported `unknown tag`, declared as a fact so the defect cannot come back nameless.
* `lanes` — **15 rows, `artifact 0 … interaction 12, windowTransient 13, windowConfig 14`**
  (was 13 rows ending at 12); `invalidTags: [15, 255]` (was `[13, 255]`).
* `shellMessageStream` — one preview chain turn as `route_exchange_output` emits it: an `Invocation`
  frame, a `windowTransient` page, an `interaction` page, a `Done` frame, a `terminal` page, and one
  frame addressed at ANOTHER instance. `expect: {appFrames: 2, pages: 3, acknowledgements: 3,
  terminal: true, faults: 0}`.

---

## 6. Deliverable 1 — the laws, all run, verbatim

### 6.1 Rust, driving the NATIVE host's production demux

`📺️renderer/🧑‍🎨engine/🧪️tests/🗞️typed-result-page/🦀️.rs` (new law appended beside the existing one).

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- typed_result_page
running 2 tests
test kernel_runtime::typed_result_page_tests::renderer_result_lane_vectors_decode_and_reject_unknown_tags ... ok
test kernel_runtime::typed_result_page_tests::one_shell_message_stream_splits_into_app_frames_and_typed_operation_pages ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 500 filtered out
```

The new law asserts the fixture declares the production magics and layout, that
`pageMagicFirstByte == PAGE_MAGIC[0]`, and then drives `decode_guest_message` over the declared
stream: two app frames for this instance, three pages, three acknowledgements owed, the terminal seen,
each page's payload byte-exact — and it fails if the stream stops exercising lane 13.

### 6.2 The TypeScript twin, same fixture, same stream

`🎭️actor/🧪️tests/🗞️typed-operation-page/🟦️.ts` (new, mounted from `🖼️wire-turn/🟦️.ts`).

```
$ bunx vitest run --config vitest.config.ts -t "two wire languages"    # @semio-tech/framework-actor
 Test Files  1 passed | 10 skipped (11)
      Tests  5 passed | 244 skipped (249)
```

Five laws: the production layout equals the declared one; every declared lane decodes and every
declared invalid tag throws; **`shellFrameBytes` returns `null` for a page AND for an ack, and
`shellMessageKind` names both**; the declared stream splits into exactly its app frames, pages and
acknowledgements (with the foreign-instance frame owed to nobody); and a fault page is NAMED without a
policy being chosen for it.

### 6.3 The wgpu-bridge law for the multi-frame reply

`📺️renderer/🧑‍🎨engine/🧪️tests/🗞️wgpu-typed-operation-reply/🟦️.ts` (new, registered in the wgpu
`vitest.config.ts`).

```
$ bunx vitest run --config vitest.config.ts 🧪️tests/🗞️wgpu-typed-operation-reply/🟦️.ts
 Test Files  1 passed (1)
      Tests  6 passed (6)
```

It pins the regression by NAME — `expect(() => decodeAppFrame(page)).toThrowError("decodeAppFrame:
unknown tag 115")` — then drives the real `WgpuTypedOperationDrive` over a `Migrated` job's reply
(page, `Done` frame, page, terminal page): exactly one decodable `AppFrame`, three acknowledgements
handed over exactly once, `owesASettle` true on `more-work` and false on idle, a turn observed twice
acknowledged once, agreement with the shared scanner on the fixture's own stream, and §4.3's bigint
projection (`JSON.stringify` throws, `invocationResponseJson` answers `req: 7`).

### 6.4 The whole wgpu suite

```
$ bunx vitest run --config vitest.config.ts        # @semio-tech/framework-renderer-wgpu
 Test Files  1 failed | 11 passed (12)
      Tests  113 passed (113)
```

The one failed FILE is `🧪️tests/🧩️package-integration/🟦️.ts`, which cannot resolve
`…/📦️packages/🦀️rust/📜️script` — a peer split that package into `🟦️typescript` + `🦀️rust` at 17:45
today, mid-session (§10). Zero tests failed.

---

## 7. What was proven on 6118, and how

`http://127.0.0.1:6118/?plugin=generation3d`, `🐍️wgpu-chain-probe.mjs` (new — it follows the chain PAST
the command reply and classifies the console itself, so a run's verdict is the log, not a reading of
it). Eleven runs; `run-9`/`run-10`/`run-11` are the converged ones and agree.

```
run-11, 90 s
alert            null
nodeCount        34
dumpFrameStats   { windowId: "procedural-main", drawCalls: 2, quadCount: 625, glyphCount: 580 }
counts           unknownTag115 0 · decodeAppFrameFailures 0 · deferredActionFailed 0
                 flowEvalTick 5 · invokeExtensionDispatch 12 (6 shell + 6 bridge)
                 extensionRequestAnswered 6 · extensionCompletionSubmitted 6 · invokeExtensionFaulted 0
```

The six round trips, in order, from `run-10/console.txt`:

```
 5715  wgpu-shell command flowEvalTick settled effects=1 mutations=0
 6192  extension request answered {pluginId: flow-extension-math, capability: evaluate,    req: 1, turns: 1, bytes:  333}
 6192  extension completion submitted {instanceId: 1, req: 1, status: ok, bytes:  295, pages: 1}
10706  wgpu-shell command flowEvalTick settled effects=1
13689  extension request answered {pluginId: flow-extension-brep, capability: evaluate,    req: 1, turns: 1, bytes:  232}
15660  wgpu-shell command flowEvalTick settled effects=1
15635  extension request answered {pluginId: flow-extension-brep, capability: evaluate,    req: 2, turns: 1, bytes:  234}
20420  wgpu-shell command flowEvalTick settled effects=2
23160  extension request answered {pluginId: flow-extension-brep, capability: tessellate,  req: 3, turns: 1, bytes:  363}
23510  extension request answered {pluginId: flow-extension-brep, capability: tessellate,  req: 4, turns: 1, bytes:  116}
23660  wgpu-shell command flowEvalTick settled effects=1
24380  extension request answered {pluginId: flow-extension-brep, capability: tessellate,  req: 5, turns: 1, bytes: 2236}
24380  extension completion submitted {instanceId: 1, req: 6, status: ok, bytes: 2265, pages: 1}
24431  wgpu-shell invokeExtension answered req=6 extension=flow-extension-brep capability=tessellate effects=0
```

**5 ticks, 6 round trips, 1 math evaluate + 2 brep evaluate + 3 brep tessellate, every one `status:
ok`, the last carrying 2265 B of mesh payload, and `effects=0` on the final answer — the chain
QUIESCED.** `📓️tick-arming-latch-2026-09-12.md` §4 derives exactly "5 ticks and 6 extension round
trips per preview window (1 math evaluate, 2 brep evaluate, 3 brep tessellate)" for the native chain.

`flowEvalResolve` / `flowTessellateResolve` never appear as host commands, and that is correct rather
than missing: they are the guest's own interactive-job completions, driven by the `completed` events
`captureExtensionCompletion.complete` submits. The host-visible half is the five `flowEvalTick`
commands above, each settling with the next `invokeExtension`.

`run-9/shot-120s.png` shows the Flow window: the outline tree with `Vector — Evaluated`,
`ExtrudeCurve — Evaluated`, `Preview — Evaluated`, all seven flow nodes (`height`, `radius`, `sides`,
`profile`, `extrusion-axis`, `extrude`, `column-preview`) and all six wires, the node-graph canvas with
its value chips `0.5 / 6.0 / 5.0`, the wires and the minimap.

---

## 8. What is NOT claimed

**The hexagonal column is not on the wgpu World3d surface, and no mesh count was observed.** The chain
delivers its 2265-byte tessellation to the guest, and the guest reports its nodes `Evaluated` — but the
frame has no World3d surface to paint into.

Evidence, `run-11`/`run-10` `console.txt` + `samples.json`:

* `dumpStructure` lists **34 nodes, every one under `stack[0]#procedural-play-main.body`**, and exactly
  one scene node: `…/stack[1]#procedural-play-main.canvas/componentScene[0]#procedural-main`. There is
  no node under `procedural-preview` at all.
* `dumpFrameStats` answers for `windowId: "procedural-main"` only.
* `procedural-preview` IS in the roster and IS rendered every round —
  `contributions slim view … windowInstances: [{id: "procedural-main"}, {id: "procedural-preview"}]`,
  and `wgpu-shell render begin/leave surface=procedural-preview body=procedural.play.preview` seven
  times in a 90 s run, each succeeding.

So the remaining hop is **the wgpu dock's layout, not the mesh chain**: `refresh_ui` renders the
preview window's document, but `self.dock.root` places only `procedural-play-main`, so no
`world3d_states` entry is ever created and the preview's transient meshes have no surface. That is the
next lane's first move, and it is independent of everything fixed here.

A second, smaller residue: `framework.panel.catalogue` still transiently reports
`renderDocument result parse failed … headroom collections=0` during the burst (8–9 times in a 120 s
run, always recovering on the next refresh, never faulting the shell). The `UiValue` arena's
COLLECTION slots are the binding capacity for that one document — the catalogue is one small `args`
map per operator row — so §4.6's per-refresh return is enough to keep the shell alive but not enough
to keep that one surface continuously available under a chain burst. Sizing
`UI_VALUE_ADMISSION_SLOTS` against a real catalogue is out of this lane's scope and is named here
rather than guessed at.

---

## 9. Checks and builds

```
$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings              # unchanged baseline
    Finished `dev` profile

$ bunx tsc --noEmit -p tsconfig.json      # @semio-tech/framework-renderer-react, filtered to touched files
  🔌️PluginRuntime/🟦️.tsx  2 errors — BOTH pre-existing (`FaultScope.req`, `AppFrameValue.Invocation`)
  🐚️plugin-bridge/🟦️.ts   8 errors — ALL pre-existing, identical set before and after, only line-shifted
  🖼️wire-turn/🟦️.ts       0
```

The renderer wasm was rebuilt and the served bundle re-verified **eight** times (every probe run
checked that the served `🎞️frame-worker.js` and `_bg.wasm` carried the build under test). Shared cargo
build dir throughout, `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`, every build in the
foreground.

Two build-loop facts worth keeping:

* `Trunk.toml`'s `[watch]` list did **not** include `🧑‍🎨engine/🧱️elements/*/🎯️targets/🧊️wgpu`, so an
  edit to the wgpu `Shell`/`ProgramBridge`/`Interpreter`/`Dock`/`Scenes`/`EngineCanvas`/`IconRenderHost`
  target compiled into the bundle but never triggered a serve rebuild — the served wasm silently stayed
  stale. All seven are now watched.
* `generate-frame-worker` still writes the artifact and THEN throws on `checkFrameWorkerCarrierCensus`
  (the census forbids the literal `localstorage`, which a live peer lane's
  `shardRuntimeDiagnosticsArmed()` in `🎭️actor/🧵️shard-runtime/🟦️.ts` still puts in the bundle). The
  file is correct; the gate is not this lane's. Same reason the serve script's `dev` path still falls
  back to `trunk serve`.

---

## 10. Observed, not mine

The workspace was red for peer refactors **seven** separate times during this lane, each for 3–20
minutes, each going green on its own. Nothing was reverted and nothing was fought:

| red | peer work in flight |
|---|---|
| `couldn't read …🧬️schema/⚛️component/🤖️generated.rs` | rename to `⚛️component/🦀️.rs` |
| `couldn't read …🖱️ui/🤝️contract.json` | move into `🖱️ui/🧬️contract/` |
| `couldn't read …♾️infinite/🖼️canvas/🌉️icon-name-value-bridge.rs` | rename to `🌉️icon-name-value/🦀️.rs` |
| `ArtifactActorMsg::LocalDocumentArchive` / `ArtifactEvent::SnapshotReplaced` / `try_fold` / `PackEncodeOptions.write_backwards_section` | the `📡️spr/🧵️channel` + `🏪️store` DocumentArchive lane |
| `Box<ArtifactStore<P, M>>: SpaceMember` (12 errors, `os-kernel`) | same wave |
| `📦️packages/🦀️rust/{vitest.config.ts,package.json,📜️script.ts}` vanished | the wgpu package split into `🟦️typescript` + `🦀️rust` (17:45) |
| `🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` → `🎭️actor/🖼️wire-turn/🟦️.ts` | the same taxonomy wave — **it carried this lane's edits and re-pointed every importer and all three test mounts correctly** |

`semio-framework-plugin --lib` is still red on the peer's `PackEncodeOptions.write_backwards_section`,
so the plugin-side half of the lane fixture
(`language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields`, which decodes each
fixture row's `name` with the real `TypedOperationResultLane` decoder) **could not be run** — the two
names added, `windowTransient` and `windowConfig`, are the crate's own camelCase spellings read
straight off the enum. `cargo check -p semio-framework-os-renderer-wgpu --lib` (native) is likewise
blocked in the same crate graph; the **wasm32** target, which is what 6118 runs, is green.

**One peer file was fixed forward, never reverted**: `🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs`
initialised `World3dScene` without `instances_delta_json`, which a peer added to
`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` at 14:23 today. That single missing field was the ONLY thing keeping
`semio-framework-os-renderer-wgpu --lib` from building its tests at all, so §6.1 could not run without
it; one `instances_delta_json: None` line.

---

## 11. Files

**New**
- `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🗞️typed-operation-page/🟦️.ts` — the TS twin (§6.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗞️wgpu-typed-operation-reply/🟦️.ts` — the wgpu multi-frame-reply law (§6.3).
- `<ticket>/🐍️wgpu-chain-probe.mjs` — the chain probe (§7).

**Changed**
- `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` — region `📬️TypedOperationPage`; `shellFrameBytes`
  discriminates; `shellMessageKind`; `drainTypedOperationTurns` lifted here; the new law's mount.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json`
  — the page layout, 15 lanes, and the shell-message stream (§5).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗞️typed-result-page/🦀️.rs` — the new Rust law (§6.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — private page codec deleted, shared one imported (§3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` —
  `WgpuTypedOperationDrive`, `serializeWgpuActorCall`, the drain, `stashLeftoverHostEffects`,
  `renderSurface`'s carried ledger, the reworked extension-completion turn, `invocationResponseJson` (§4.1–4.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` —
  `dispatch_command` on the one funnel; `PendingExtensionInvocation` + `run_extension_invocation`;
  `flush_deferred_actions` as one convergence loop; `drain_retained_document_arenas` (§4.4–4.6).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` —
  `retire_browser_ui_values`; the parse error's payload window and arena headroom (§4.6).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `LANE_MAX = 14` (§2.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/vitest.config.ts` — registers the new suite.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml` — the seven element wgpu targets are watched (§9).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs` — the peer field, fixed forward (§10).

No `launch.json` entry was added: `procedural3d-wgpu` already exists and is the row this lane ran, and
both new TS suites run inside existing targets (`@semio-tech/framework-renderer-wgpu:test`,
`@semio-tech/framework-actor:test`).

**Temporary logs left in the tree** — `[DEBUG] wgpu-bridge typed-operation …`,
`[DEBUG] wgpu-shell invokeExtension dispatch/answered/failed`,
`[DEBUG] wgpu-shell command <id> settled effects=…`, `[DEBUG] wgpu-bridge renderSurface … carried=…`,
`[DEBUG] wgpu-shell deferred chain exhausted …`. They are the entire evidence trail of §7 and the only
trace the chain leaves; the `renderDocument` parse error's payload window and `headroom` are NOT
temporary — that message named nothing but a column number before, and §4.6 was read straight off it.
