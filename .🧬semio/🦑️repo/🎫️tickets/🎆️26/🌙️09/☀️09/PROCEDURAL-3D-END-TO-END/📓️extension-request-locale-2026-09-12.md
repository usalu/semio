# Extension Request/Completion Wire — `missing field \`locale\`` Decoded, And What The 154 Fault Bytes Actually Say

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "extension request/completion wire + fault surfacing", 2026-09-12.
Repo MCP was down all session (`invalid initialize params`); no ticket was opened, closed or reopened.
Probe outputs: `🗑️generated/extension-locale/` (`probe-diag-1`, `probe-diag-2`, `probe-guard-1`, `probe-fix-1`, `probe-fix-2`).

---

## 1. TL;DR

Two DIFFERENT defects were hiding behind one console, and neither is where the brief expected.

| symptom | actual cause | side | status |
|---|---|---|---|
| `pageerror SemioFaultError: missing field \`locale\`` ×2 per boot | the host dispatched a recursively requested effect with the shell's **raw `ActiveSession.viewState`** (`{ activeModeId }`), which carries no `locale`/`terminology`; the guest's `ViewModel` declares both non-optional | **host (TypeScript)** | **fixed, live-proven: 0 pageerrors** |
| `extension completion submitted … status: fault, bytes: 154` on every `evaluate` | the React `PluginWasmHandle` **has no `invoke` at all** — `extension.invoke-unavailable` is raised before `flow-extension-math` is ever entered | **host (TypeScript), missing door** | **decoded and proven; the door itself is a sized next package (§5)** |
| preview status stuck on `flow.extension-not-contributed` after install | the status object could only ever name the ADDRESSING miss; a contributed-but-refusing extension had no representation | **guest (Rust)** | **fixed, live-proven** |

**The brief's hypothesis — that `locale` was added to the extension invocation request schema, the
flow-extension-math component's request struct, or the host's TS decoding — is wrong on all three
counts.** `evaluate_invoke_json`'s `EvaluateRequest` (`🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:126-136`) has
exactly `operatorId`/`inputJson` and never had a `locale`. The 154 fault bytes never came from the
extension at all. `ViewModel.locale` has been non-optional since at least 2026-09-08 (checked across
five commits) — nothing was "added recently"; what drifted is that several host dispatch paths never
resolved a view context in the first place, and the ONE path that also `void`-dispatches
(`scheduleDispatchAction`) is the one whose rejection reached the page unnamed.

---

## 2. Deliverable 1 — where `missing field \`locale\`` is raised, and the fix

### 2.1 The raiser

`#[derive(FromValue)]` (`🌱️value/✨️derive/🦀️.rs:517/526/626`) emits `missing field \`{wire_name}\``.
The struct is **`ViewModel`** (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4272-4330`), whose `locale: Locale`
and `terminology: Terminology` carry no `#[value(default)]` — deliberately, per their own docstring
("the shell always resolves one … so 'nobody set the locale' is unrepresentable").

It is decoded on the guest at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33612`:

```rust
protocol::AppCommand::Command { seq, command, view_state } => 'dispatch: {
    let view_state = if view_state.is_empty() { None } else {
        match decode_wire_serialized::<ViewModel>(&view_state).await {
            Ok(view) => Some(view),
            Err(fault) => { push_app_fault(&mut frames, Some(seq), fault).await; break 'dispatch; }
```

An EMPTY view state is tolerated (`None`); a NON-EMPTY one missing a required field becomes an
`AppFrame::Error`, which `invocationFromFrames` (`🔌️PluginRuntime/🟦️.tsx:2776-2780`) rethrows as a
`SemioFaultError`.

**Pinned as a law, not as prose** — `🛂️manifest/🧪️tests/🪟️resolved-host-context/🦀️.rs`:

```
test manifest::resolved_host_context_tests::an_unresolved_context_names_the_field_it_lost ... ok
test manifest::resolved_host_context_tests::the_resolved_context_decodes_in_the_guest ... ok
test result: ok. 2 passed; 0 failed; … 228 filtered out
```

`error.to_string()` for the `missing-locale` row is byte-for-byte the string the browser printed.

### 2.2 The producer

`ActiveSession.viewState` is the shell's own partial projection — constructed as
`{ activeModeId: … }` at `🏛️ShellHost/🟦️.tsx:3390`, `:7273`, `:8216` — and nothing in the reducer ever
stamps `locale`/`terminology` onto it. The single admission that does is `resolvedTargetViewState`
(`:4119-4131`, `parseResolvedPluginViewState` + `uiLocale`/`uiTerminology`).

Three dispatch families crossed the raw projection:

- `makeEffectDispatchOne` (`🛠️ShellHelpers/🟦️.tsx:715-729`) → `baseSession.viewState`, used by
  `dispatchOpenedFiles` (`:5167`), **`scheduleDispatchAction` (`:5181` — the `flowEvalTick` re-arm)**
  and `requestMediaFrames` (`:5306`).
- `scheduleDispatchAction` then `void`-dispatches, so the guest fault had no awaiting caller: an
  unhandled rejection, printed with neither the action nor the field's owner. That is exactly the
  two `pageerror` lines per boot, and why the count (2) was so much lower than the flowEvalTick
  count (hundreds) — every OTHER chain originated from an action path
  (`:6248-6260`, `{ ...targetSession, viewState: dispatchViewState }`) whose view state IS resolved.

### 2.3 The fix (schema-first, no optional-field shim)

1. **`makeEffectDispatchOne` now REQUIRES a resolver** (`resolveViewState: (session) => ViewModel`),
   dispatches with it and hands `applyEffects` the session carrying the resolved context, so a
   recursive chain cannot re-lose it. All three `🏛️ShellHost` call sites pass `resolvedTargetViewState`
   (added to `applyHostEffects`' dependency list).
2. **One host→guest gate, in `🔌️PluginRuntime/🟦️.tsx`** — `admitCrossingViewContext` on every place a
   view context is encoded for the guest: `performInvocation` (action AND command), `refresh-ui`
   (`uiRefreshSurfaceEvents`), `context-menu`. An unresolved context is now a loud host-side throw
   naming the crossing and the missing field, never an anonymous guest fault.
   *Scope note*: this is a required-FIELD gate, deliberately not the full
   `parseResolvedPluginViewState` capacity gate — see §6.1, that is a live, separately-owned lane.
3. **`scheduleDispatchAction` names its own rejection** (`console.error("scheduled dispatch of \"…\" failed", …)`).
4. **`runCapturedExtensionEffect` prints the DECODED fault** (`[DEBUG] extension invocation refused`
   with `origin`/`code`/`message`) — the only reason §3 could be decoded at all; the completion
   itself reaches the guest as opaque bytes.

### 2.4 Live proof

`SEMIO_PROBE_SECONDS=85 SEMIO_PROBE_OUT=extension-locale/probe-fix-2 bun 🐍️console-dump-probe.mjs`

| probe | `pageerror` count |
|---|---|
| `probe-restage-2b` (before) | **2** (`SemioFaultError: missing field \`locale\``) |
| `push-starvation-host` (before) | **2** (same) |
| `probe-fix-2` (after) | **0** |

---

## 3. Deliverable 4 (brought forward) — what the 154 fault bytes actually are

With the host-side decode log live, one reload answers it:

```
[DEBUG] extension invocation refused {"extensionId":"flow-extension-math","capability":"evaluate",
  "origin":"os","code":"extension.invoke-unavailable","message":"extension.invoke-unavailable"}
[DEBUG] extension completion submitted {instanceId: 1, req: 1n, status: fault, bytes: 154, pages: 1}
```

The request that was refused is well-formed and correctly addressed:

```json
{"operatorId":"math.vector","inputJson":"{\"z\":{\"$schema\":\"number\",\"value\":6.0}}",
 "nodeHash":17647890481751977919,"windowId":"procedural-preview",
 "windowKindId":"procedural-preview","extensionId":"flow-extension-math"}
```

**`extension.invoke-unavailable` is raised at `🏛️ShellHost/🟦️.tsx:1678`**, i.e.
`typeof extensionEntry.handle.invoke !== "function"`. And it is not a function, because **nothing
defines it**:

- `adaptPluginHandle` (`🔌️PluginRuntime/🟦️.tsx:2835-2930`) builds the handle field by field — no `invoke`.
- `loadPluginModule` returns `{ ...richHandle, refreshUi, captureExtensionCompletion, bindDocumentPort }`
  (`:2604`) — no `invoke`.
- the kernel handle it wraps (`🎠️kernel/🟦️.ts:220-235`) is `manifest`/`createApp`/`destroyApp`/
  `takeSegmentedDownloadChunk`/`enqueue`/`outcomes`/`dispose` — no `invoke`.

So **no extension invocation has ever reached a guest in the React renderer.** The fault was never
`flow-extension-math`'s; the extension is loaded (`hot-swap flow-extension-math … version: 0.2.0`),
resolved (no `invokeExtension unresolved` warning), and simply never called. Every boot's
`0 output bytes` seed failure and `meshes=0` follow from this one missing door.

The reason it is missing is a real ABI move, not an oversight in one file — `🔌️plugin/🧬️schema/📜️.wit:768`:

> "Replaces `extension.invoke`, `artifact-compose`, the guest `io-run`/`io-sniff` exports … every
> 'someone else calls INTO this actor' seam is now one inbound `request`, answered with the `respond`
> effect within a bounded number of turns, or by spawning a job."

The host half of that seam does not exist in TypeScript: `grep -rn '"respond"'` over
`🧰️framework/**/*.ts{,x}` (non-dist) returns **0 hits**, and `wireEffectToFriendly` has no `respond`
case. §5 sizes it.

---

## 4. Deliverable 2 — the preview publishes the fault it is actually living with

### 4.1 What changed

- **`ExtensionEvaluateFault`** (new, `🌊️flow/🖥️host/🦀️.rs`, next to `FlowExtensionAddressMiss`):
  `{ extension_id, capability, code, message }`, `CODE = "flow.extension-evaluate-failed"`, `labels()`
  giving an en/de pair with no default language. Deliberately distinct from the addressing miss: a
  miss means nothing was invoked, this means the extension answered and refused.
- **`FlowEvalSessionState.extension_evaluate_fault`** + `note_`/`clear_`/reader, and it is cleared
  inside `invalidate_for_flow_extension_registry` — so **a contributions install clears the previous
  fault before the first evaluation settles**, which is the brief's second requirement.
- **`FlowEvalResolve` gains `extension_id`, `ok`, `fault_code`, `fault_message`** (all
  `#[value(default)]`). These are not invented: `reactor::extension_response_args`
  (`🔌️plugin/⚛️reactor/🦀️.rs:731-777`) has always appended `ok` plus `faultCode`/`faultMessage` to
  the response action's args, and always echoed the request's own correlation fields back — the
  generation3d payload simply never declared them, so the decoded fault was dropped on the floor.
  `evaluate_tick` now puts `extensionId` on the request so the echo can name WHICH extension refused
  (the tick's geometry address is a different extension from the one an operator hop is routed to —
  the first draft of this fix published `brep` for a `flow-extension-math` refusal and the law caught it).
- **`resolve_eval` records or forgets it** (`note_eval_answer_fault`, publication only — no arming
  touched), and **`preview_progress_status_json_for` publishes it**, outranking the addressing miss,
  with `phase: "faulted"`, the Faulted `phaseLabel` pair and `cancellable: false`.

### 4.2 Live proof (`probe-fix-2`, `window:procedural-preview` `data-status-json`)

```json
"fault":{"code":"flow.extension-evaluate-failed","extensionId":"flow-extension-math",
 "capability":"evaluate","faultCode":"extension.invoke-unavailable",
 "faultMessage":"extension.invoke-unavailable",
 "message":{"en":"Geometry extension 'flow-extension-math' could not evaluate",
            "de":"Geometrie-Erweiterung 'flow-extension-math' konnte nicht auswerten"}}
```

The stale `flow.extension-not-contributed` is gone. `meshes` is still 0 — that is §3's missing door,
not this.

**One restage still owed for this deliverable**: that capture came off the 03:41 wasm, which predates
the `phaseLabel`/`cancellable` correction (the JSON above still reads `phaseLabel: {"en":"Idle"}`
beside `phase: "faulted"`). The corrected projection is green natively and needs one restage of the
procedural plugin to be visible. Nothing else here is guest-side-pending.

---

## 5. The next package: the host half of the `request`/`respond` seam

Not attempted here — it is a new door, not a repair, and half-landing it would break every boot.
What it needs, in the order it has to be built:

1. `PluginRuntime` gains `invoke(capability, request)` on the handle it returns: capture the
   extension actor's activation, submit one `Event::Request { req, params: { origin, capability,
   payload } }` shard event (the envelope kind already exists — `🎭️actor/📮️shard-client` tests drive
   `{ kind: "request", … }`), then drain that actor's turns.
2. `wireEffectToFriendly` learns `respond` (`📜️.wit:595`, `respond-effect` = `{ req, outcome:
   respond-result }`) and routes it by `req` instead of dropping it as an unmapped effect.
3. The answer crosses back through the SAME paged contract `captureExtensionCompletion.complete`
   already uses (`guestAnswerPages` + `GUEST_HOST_ANSWER_CEILING_BYTES`), for the same
   `cabi_realloc` reason boot #12 documented.
4. `runCapturedExtensionEffect`'s `extension.invoke-unavailable` branch then becomes genuinely
   unreachable for a loaded extension, and `🧫️fixtures/🔁️extension-invocation-wire/🔣️.json`'s `ok`
   row becomes drivable end to end instead of only against the SDK.

The wgpu bridge (`🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts:906`) reads `invoke` off its handle exactly the
same way and is therefore in the identical state — one door serves both.

---

## 6. Not mine, observed live, worth someone's attention

### 6.1 The view-context capacity lane is live and was moving under this one

My first gate used the full `parseResolvedPluginViewState`, and the browser answered
`view context rejected at refresh-ui: view context: invalid panel data` on every refresh: the shell
puts its **248 635-character** `contributionsJson` into the refresh view state, against the schema's
65 536-character bound (the gap `📓️extension-addressing-2026-09-10.md` §6 already sized). I relaxed my
gate to the required-FIELD check rather than take that package on by breaking the app — and while I
was writing this, a peer extended my own new fixture/laws with `contributions-in-view-state` and
`oversized-panel` rows and removed `contributionsJson` from the view-context schema. That is the right
fix and it is theirs; the two gates compose.

### 6.2 A contributions blackout between 02:57 and ~03:45

Three consecutive boots (`probe-diag-1`, `probe-diag-2`, `probe-guard-1`) published **zero**
contributions: `contributions push skipped unresolved document operators {reason: "no-operator-graph"}`,
with the examples branch never reached. Cause: `exampleArtifactSources` (`🎠️kernel/🟦️.ts:335`) was
switched from `appId` matching to `dialect` matching at 02:57, while the served wasm's manifest
examples still carried no `dialect` (`ExampleDefinition.dialect` is new in the same lane). The 03:41
restage cleared it — `probe-fix-1` onward installs 16 kinds / 248 635 chars at 3.3 s. Flagged because
it silently makes the whole extension lane unobservable and reads exactly like a regression in it.

### 6.3 Pre-existing failures, verified not mine

- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib`
  (full, `--test-threads=1`): **366 passed, 6 failed**. All six are typed-operation retirement
  timeouts (`"registered fixture typed operation did not retire within 30 seconds"`,
  `"Generation3d preview operation did not finish"`) or `module.vcs` fail-closed assertions, in files
  the sibling arming/latching lane edited at 03:20/03:23 — after this session started.
- React engine vitest (full, 27 files): **915 passed, 9 failed** — 6 × `🧩️package-integration` worker
  bytes, `noteShellCommand` (`inverseArgs`/`inverseCommandId` added to the impl, expectation not yet
  updated), `surface render ViewModel` (an extra `["window","canvas-body"]` binding), and
  `readAppDocumentPack` (impl now returns `ops`, expectation not). None touch anything in this lane;
  each was inspected individually.
- `UnlinkedFlowExtensions::take()` (`🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs:127`) asserts both
  linked packs are registered, and the registry is process-wide — so **any** filtered run of two laws
  that both take it fails under the default thread pool. `--test-threads=1` is required. Worth making
  the guard re-entrant; not done here.

---

## 7. Tests — all run, results verbatim

### 7.1 View context (the actual defect): one fixture, two implementations

Fixture `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json`
gained a `guestDecode` section pinning the exact guest fault per removed field.

| implementation | command | result |
|---|---|---|
| Rust (guest decoder) | `RUST_MIN_STACK=33554432 cargo test -p semio-framework --lib resolved_host_context` | **2 passed / 2** |
| TypeScript (host admission) + Ajv oracle | `bun ./📜️script.ts verify resolved-host-context` | `resolved-host-context cases=9 guest-rejections=2 schema=valid explicit-preferences=required` |

### 7.2 Extension invocation request/completion rows (ok, fault, empty output)

New fixture `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json`
— four rows: `ok`, `empty-output`, `operator-fault` (the extension answered `{"error": …}`, which is
still an **ok** completion on the wire) and `bad-request` (undecodable request → **fault** completion).

| implementation | command | result |
|---|---|---|
| Rust (`evaluate_invoke_json`, the SDK that produces the bytes) | `cargo test -p semio-framework-os-flow --lib the_evaluate_wire_answers_every_fixture_row` | **1 passed / 1** |
| TypeScript (`runInvokeExtensionEffect`, the shell that classifies and packs them) | react vitest, `-t "evaluate answer"` | **4 passed / 4** (one per row) |

### 7.3 The evaluate fault the preview publishes

New fixture `…/🧊️generation3d/…/✏️editor/🧫️fixtures/💥️extension-evaluate-fault.json`.

| implementation | command | result |
|---|---|---|
| Rust law (drives `resolve_eval` + the status projection + the install clear) | `cargo test … --lib -- --test-threads=1 an_evaluate_fault_outranks` | **1 passed / 1** |
| TypeScript twin (the shape and both-languages rule the surface renders) | react vitest, `-t "evaluate-fault"` | **1 passed / 1** |

### 7.4 Required regression gate

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib -- --test-threads=1 a_late_contributions_install
test editor::generation3d::component::tests::a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted ... ok
test viewer::generation3d::component::eval_chain_tests::a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 370 filtered out
```

Re-run after every edit in this lane, including the last one. Green throughout.

### 7.5 React engine vitest, full

`SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts` → **915 passed, 9 failed**, every
failure listed and attributed in §6.3. Three call sites that fed the new gate an unresolved context
were corrected in place (`🔬️engine-contract/🟦️.ts` `handleAction` fixture; `🔌️plugin-runtime/🟦️.tsx`
`refreshUi`, `registerBrushMesh`, `commitFixture`) — they were asserting a contract the guest never
accepted.

---

## 8. Files

**New**
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🪟️resolved-host-context/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/💥️extension-evaluate-fault.json`

**Changed — host (TypeScript, live on reload)**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (`admitCrossingViewContext` + its three crossings)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` (`makeEffectDispatchOne` resolver, `scheduleDispatchAction` rejection)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (three call sites + dep list, decoded-fault log)

**Changed — guest (Rust, needs one restage for §4.2's label correction)**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (`ExtensionEvaluateFault`, session field, clear-on-install)
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (test module declaration only)
- `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` (`FlowEvalResolve` fields, `extensionId` correlation, `note_eval_answer_fault`)
- `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs`, `…/👁️viewer/🦀️.rs` (argument parsing + the status projection)

**Changed — tests**
- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json`, `…/🧪️tests/🪟️resolved-host-context/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, `…/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
- `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `…/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs`

Nothing under any `🗑️generated` folder was swept. No wasm build of the procedural plugin was started
from this lane, no dev server was started or restarted, and no ticket state was changed.
