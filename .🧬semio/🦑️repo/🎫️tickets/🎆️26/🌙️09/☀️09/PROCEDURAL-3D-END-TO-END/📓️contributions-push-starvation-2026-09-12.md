# Contributions Push Starvation — the push is now its own unit, and a not-contributed tick stops re-arming

Lane `contributions-push-starvation`, 2026-09-12, Opus execution agent. Repo MCP was down all session
(`-32602 invalid initialize params`); bookkeeping is on disk and no ticket was opened, closed or reopened.
Command outputs under `🗑️generated/push-starvation/` and `🗑️generated/push-starvation-host/`.

**Restage required: YES** (guest Rust changed). The host half is live on `:6018` already and is verified below.

---

## 1. What was wrong

### 1.1 Host — the push was a passenger in `refreshUi`'s generation race

`🏛️ShellHost/🟦️.tsx`'s `refreshUi` bumps `refreshGenerationRef` on entry and abandons itself whenever the
generation moves under one of its awaits. The contributions push lived INSIDE that guard and its own await
was a guest call (`readAppDocumentPack`) queued behind the guest's slow tick commands. The served guest
re-arms a faulting `flowEvalTick` every 3–14 s; every settle drives a `refreshUi`; every `refreshUi` bumps
the generation — so every push was superseded inside its document read and the closure never crossed.

Measured (`🗑️generated/console-dump/console.txt`, 45 s): **zero** `[DEBUG] contributions …` lines,
preview `phase: faulted` / `flow.extension-not-contributed` (`brep`), `meshes=0`.

This is structural, not a race window to widen: a contributions push is not a projection of one refresh's
UI state, so a superseded refresh has no business aborting one.

### 1.2 Guest — the fault re-armed itself, and the poll re-armed it too

`flowEvalTick`'s continuation was `if more { rearm }`. An uncontributed operator kind is not slow work:
`Evaluator::dispatch` answers `EvalError::UnknownKind`, the node publishes an error dictionary, **the error
is never cached** (`cache.seed` only on `Ok`), so the next tick recomputes the identical miss. A graph that
outruns one step budget therefore re-arms forever.

Worse, `Generation3dPlayApp::pending_effects` polls on a **scratch** `FlowEvalSession`
(`with_scratch_session` → `FlowEvalSession::new()`, its own empty `NeuralCache`), so its budget-0 probe
always reports pending and it arms one tick per refresh regardless of the retained session's state. Both
sources fed the same spin, and the spin is what starved the host push (§1.1) — chicken and egg.

### 1.3 A latent framework leak the new law exposed

`🌊️flow/🌉️bridge/🦀️.rs` `build_channel_eval_json` builds `channels.inputs…cloned().merge(params)` per
widget. `Dictionary::merge` MINTS a dictionary (`ColdDictionaryBuilder::finish`), so any neuron carrying
inline params left this projection owning the only copy and `Dictionary::drop` fail-closed on it. Proven:
without the fix, `cargo test -p semio-framework-os-flow --lib` **aborts the whole binary** (SIGABRT, "panic
in a destructor during cleanup") at `host::tests::set_neuron_params_merges_into_eval_input`.

---

## 2. What changed

### 2.1 Host

| file | change |
|---|---|
| `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts` | **new** `createContributionsPublisher<E>` — the push as an owned unit: keyed `(pluginId, instanceId)`, concurrent callers JOIN one run on an unmoved registry generation, operator scope resolved once per session, scoped pack cached per `(pluginId, generation, kinds)`, `setContributions` installed keyed `(instanceId, content)` and **claimed before the guest crossing** (restored on throw so a failure retries), `retire(instanceId)` abandons a resolving unit on a session switch. Own module (no React, no shell imports) so a law can drive it — importing `🛠️ShellHelpers/🟦️.tsx` from a test hits the `ShellHelpers → Shell → ShellHost` cycle and dies with `Cannot access '__vite_ssr_import_8__' before initialization`. |
| `🏛️ShellHost/🟦️.tsx` — new `//#region 🧩️ContributionsPush` | the publisher instance (a `useRef`, created once), `readDocumentOperatorScope` (document read → typed scope, with the published-examples fallback), `publishContributions` (captures the environment: loaded plugins, disabled extensions, host mode, `resolvedTargetViewState`, and the deferred-effects dispatcher), and the session-switch `retire` effect. |
| `🏛️ShellHost/🟦️.tsx` — `refreshUi` | starts the unit BEFORE its first await (`const contributionsInstalled = publishContributions(nextSession, loadedPlugins)`) and only `await`s it just before `pendingRefreshEffects` — so `pendingRefreshEffects` deferral is unchanged while the push itself is outside the generation race. The ~110-line inline push block, `contributionsJsonRef` and `documentOperatorKindsRef` are gone (the unit owns both keys). |

Every `[DEBUG] contributions …` line is kept verbatim, plus one new `[DEBUG] contributions publish` naming
the unit's decision (`installed` / `unchanged` / `unresolved` / `empty` / `retired` / `failed`).

### 2.2 Guest

A peer landed the surface-neutral `🧵️preview-eval` extraction mid-session; the gate went in there, so both
surfaces get it.

| file | change |
|---|---|
| `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` | **new** `unserved_flow_operator_kinds(&FlowFixture) -> Vec<String>` — the graph's `Widget::Neuron` kinds the live registry cannot serve. Domain-neutral, read-only, no allowlist. |
| `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | **new** `may_rearm(fixture)` = that list is empty; `evaluate_tick`'s continuation becomes `if more && may_rearm(fixture)`. |
| `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | re-exports `may_rearm` beside `rearm`. |
| `…/✏️editor/🦀️.rs` `pending_effects` | arms nothing while the graph is unserved (the poll's scratch session cannot see the chain that gave up). The viewer's `pending_effects` carries the same guard (mirrored by the peer's refactor). |
| `🧰️framework/…/🌊️flow/🌉️bridge/🦀️.rs` | `input_dict` is retired per widget — the §1.3 leak. |

### 2.3 The re-arm design decision, and what it supersedes

**A fault nothing in this process can clear owes no continuation.** The only thing that may resume the
chain is `setContributions`, whose route already invalidates the retained session against the moved
`flow_extension_registry_generation` and re-arms one tick per attached preview window
(`📓️contributions-rearm-2026-09-10.md` §4.2) — measured again here at exactly 1 re-arm per attached preview.

This **supersedes** `📓️fault-arm-symmetry-2026-09-10.md`'s "the fault arm is symmetric with the ok arm"
where that reading has been taken to mean continuation symmetry. That report is about how a fault VALUE is
encoded on the `completion-result.fault` / host-async wire (pack, not JSON) and is silent on who owes the
next tick. **Symmetry of encoding is not symmetry of continuation.** Nothing in it changes.

The gate is deliberately narrow: it blocks only ARMING. A tick that is already armed still runs and still
publishes whatever it could compute (the numbers, the per-widget `unknown kind` errors), so a partially
served graph loses no information — it just stops paying for the same miss forever.

---

## 3. Tests (all run; commands and results verbatim)

### 3.1 Host — vitest, JSON-fixture driven

New suite `🧰️framework/…/🧑‍🎨engine/🧪️tests/🧩️contributions-push/🟦️.ts`, registered in the react target's
`vitest.config.ts`, driven by `🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json` (language-neutral:
scenarios, refresh counts, when the document read resolves, expected reads/pushes/keys/outcomes).

```
cd 🧰️framework/…/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bunx vitest run "🧩️contributions-push"
→ Test Files 1 passed (1) | Tests 6 passed (6)
```

Scenarios: `superseded-mid-document-read` (4 refreshes all superseded mid-read → 1 read, **1** push),
`repeat-refresh-installs-once`, `session-switch-retires-the-resolving-unit`,
`unresolved-scope-pushes-nothing`, plus registry-generation re-run and install-throw-restores-the-key.

**Discrimination measured**, not argued: with the in-flight join disabled the suite fails
`document reads for superseded-mid-document-read: expected 2 to be 1` (2 of 6 red). Restored → 6/6.

```
SEMIO_TEST_LEVEL=long bunx vitest run "🔬️engine-contract" "🩺️window-fault"
→ 557 passed | 1 failed — `inverseArgs`/`inverseCommandId` on `shell.windowClose`, a peer's
  in-flight window-config row, nothing this lane touches.
```

### 3.2 Guest — Rust law + third-party twin

Fixture `…/✏️editor/🧫️fixtures/🚧️contribution-gated-arming.json`; laws in
`…/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs`; twin `…/🔬️unit/contract.ts` (the neighbouring
`🔬️tick-addressing` does the same).

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- flow_eval_tick::tests --test-threads=1 --nocapture
→ test result: ok. 3 passed; 0 failed
[STATS] uncontributed-graph-arms-nothing: kind=witness.contribution-gate.absent
        unserved=["witness.contribution-gate.absent"] pending=true armed=0
[STATS] served-graph-keeps-its-chain: kind=math.add unserved=[] pending=true armed=1
[STATS] setContributions resumed the chain with 1 re-arm(s) across 31 page(s)

bun …/⏱️flow-eval-tick/🧪️tests/🔬️unit/contract.ts
→ generation3d contribution-gated arming blocked=0 served=1 resume=setContributions budget=512
```

Both arms are the SAME oversized graph (600 neurons > the real `FLOW_EVAL_TICK_STEP_BUDGET`, asserted
against the constant; each neuron carries its own params or the node cache would collapse them into one
miss), so `pending=true` on both and only the registry's answer differs.

**Discrimination measured**: with `&& may_rearm(fixture)` removed from `evaluate_tick`, the law fails at
`uncontributed-graph-arms-nothing: armed 1 ticks, fixture says 0`. Restored → green.

### 3.3 Regression

```
nx test @semio-tech/procedural-generation3d-rs -- contribution      → 11 tests run, 11 passed (nextest)
cargo test … --lib -- contribu --test-threads=1                      → 12 passed; 0 failed
    (includes `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
     and the viewer twin — the gate does not break the late-install recovery)
cargo test … --lib -- tick_addressing --test-threads=1               → 4 passed; 0 failed
cargo test … --lib --test-threads=1 (full)                           → 358 passed; 6 failed
cargo test -p semio-framework-os-flow --lib -- --test-threads=1      → 153 passed; 57 failed
  same run with the §1.3 bridge fix reverted                         → binary ABORTS (SIGABRT)
  …reverted + --skip set_neuron_params_merges_into_eval_input        → 152 passed; 57 failed
```

So the bridge fix is **+1 pass, 0 new reds, and no more binary abort**.

The 6 generation3d reds are all attributed elsewhere: `two_instances_converge_disjoint_widget_moves`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_…`, `generation_preview_is_one_app_transient_…`,
`refresh_pending_effects_arms_flow_eval_tick_chain` (the four listed in
`📓️contributions-rearm-2026-09-10.md` §5.1), plus `add_generation_records_an_undoable_generation_operation`
and `undo_redo_round_trips_flow_graph_edits` ("undo did not revert to the expected snapshot"), which a peer
lane is already probing in `🗑️generated/viewer-lane/isolated-failures.txt`.
`refresh_pending_effects_arms_flow_eval_tick_chain` was additionally re-run with this lane's
`pending_effects` guard removed and fails **identically** (`typed operation did not retire within 30
seconds`) — pre-existing, not this lane's.

### 3.4 Typecheck

```
nx run @semio-tech/framework-renderer-react:typecheck
→ 862 errors, ZERO in this lane's code: no row in `🛠️ShellHelpers`, none in
  `🛠️ShellHelpers/🧩️contributions`, none in the new `🧪️tests/🧩️contributions-push`, and none in
  ShellHost's new region (lines ~4307–4460 / the `await contributionsInstalled` site).
  The five ShellHost rows (1945, 1957, 7945, 7946, 8615 — `brushPreviewJson`, `InteractionState`,
  `{action: string}`, an `unknown` argument) are the peers' in-flight rows this file already carried.
```

---

## 4. Runtime evidence (live, `http://127.0.0.1:6018/?plugin=generation3d`, 60 s)

`cd <ticket> && SEMIO_PROBE_SECONDS=60 SEMIO_PROBE_OUT=push-starvation-host bun 🐍️console-dump-probe.mjs`
→ `🗑️generated/push-starvation-host/console.txt`.

```
3290 [DEBUG] contributions document sources {"packBytes":873,"sprBytes":280,"opsChars":92,…,
       "status":"unresolved","reason":"no-operator-graph","kinds":[]}
3290 [DEBUG] contributions scoped from published examples {"plugin":"procedural",
       "app":"s.procedural.generation3d@1/*#editor","kinds":[16 kinds],"examples":8}
3300 [DEBUG] contributions scoped pack {"chars":248635,"hasManifestJson":true,"hasPolygon":true,…}
7068 [DEBUG] setContributions deferred effects {"plugin":"procedural","instanceId":1,
       "effects":[4 × dispatchAction flowEvalTick → procedural-preview / generation3d-generate-preview]}
7068 [DEBUG] contributions publish {"plugin":"procedural","instanceId":1,
       "outcome":{"status":"installed","chars":248635,"kinds":[…]}}   ×4 joined refreshes
```

Line counts over the whole 60 s boot: `contributions document sources` **1**, `scoped from published
examples` **1**, `scoped pack` **1**, `setContributions deferred effects` **1**, `contributions publish`
**4** (the four refreshes that joined the one run, each reporting its outcome).

Before this lane, the same probe produced **0** contributions lines in 45 s. The push now lands at 3.3 s and
the install settles at 7.1 s, deterministically.

(The first run of the probe showed `scoped pack` **71×** — the 248 kB cut was being recomputed on every
refresh. That is what the per-`(pluginId, generation, kinds)` pack cache in the publisher fixes; the numbers
above are after it.)

**Still faulted, and not this lane's**: the preview remains `phase: "faulted"` with `meshes=0` and the flow
status still reads `profile: queued / extrusion-axis: computing`, and 225 `flowEvalTick` invocations still
fire in 60 s. Both are expected here: the served component is the **2026-09-11 wasm**, so neither the
re-arm gate nor anything else from §2.2 is in it, and the guest's own post-install `unknown kind` blocker
(`📓️contributions-example-scope-2026-09-11.md` "Next blocker", `📓️unknown-kind-after-restage-2026-09-11.md`)
is a different lane. No wasm build was started from here.

---

## 5. Files

**New**
- `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts`
- `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json`
- `🧰️framework/…/🧑‍🎨engine/🧪️tests/🧩️contributions-push/🟦️.ts`
- `✏️s/…/🧊️generation3d/…/✏️editor/🧫️fixtures/🚧️contribution-gated-arming.json`
- `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/contract.ts`

**Changed**
- `🧰️framework/…/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` (publisher moved out; nothing else)
- `🧰️framework/…/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts`
- `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`, `🧰️framework/…/🌊️flow/🌉️bridge/🦀️.rs`
- `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs`
- `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs`, `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`,
  `…/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs`

---

## 6. Open

1. **Restage.** Every §2.2 change is guest-side. The spin and the gate cannot be observed in the browser
   until the coordinator restages the procedural component.
2. **`pending_effects` still polls a scratch session.** Even fully contributed, it arms one `flowEvalTick`
   per refresh (the retained session's `tick_scheduled` latch is invisible to it), so a settled app still
   pays one idle tick per refresh. The gate removes the pathological case, not the idle one; the real fix is
   a `pending_effects` that can see the RETAINED session, which the framework trait signature
   (`fn pending_effects(doc, cfg, view)`, no owner handle) does not currently allow.
3. **`generation2d` is untouched.** Its editor has the same `pending_effects`/tick shape on the app-wide
   `HostOnly` lane and would spin the same way against an uncontributed registry.
4. **Cluster inner kinds are not walked** by `unserved_flow_operator_kinds` — a graph blocked only inside a
   cluster still falls back to the old behaviour. Deliberate: cluster boundary kinds are structural
   (`core.input`/`core.output`) and would produce false blocks that no contribution could lift.
5. The guest's post-install `unknown kind` (contributed registry vs the guest's own operator catalogue) and
   the preview fault it leaves behind are still open and owned elsewhere.
