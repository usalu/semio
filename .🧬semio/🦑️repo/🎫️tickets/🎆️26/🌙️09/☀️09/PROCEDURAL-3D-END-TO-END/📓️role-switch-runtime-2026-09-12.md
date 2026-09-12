# Role-Switch Runtime — Transactional Surface Switch + Generate-Mode Entry State (2026-09-12)

Lane: `role-switch-runtime` (continuation of a stalled agent). Owned surface:
`🏛️ShellHost/🔀️surface-switch/`, `switchToPluginApp`, `applyModeChange`, generate-mode entry.

Two live defects from `🗑️generated/journey-3/` (`console.txt`, `results.json`):

| | defect | status |
|---|---|---|
| **A** | `Meta+Alt+V` fired mid-chain retired editor instance 1 underneath its own in-flight work; the DOM kept the EDITOR windows and the roles group still read `editor`, and every later action failed `no actor for instance 1` (20 of them) plus one `pageerror` | **fixed**; proven on 6018 (§5.3): `window:procedural-view-preview` mounted, roles `viewer` pressed, `no actor for instance` 20 → **0**, `pageerror` 1 → **0** |
| **B** | `Meta+Alt+ArrowRight` gave `hosts=[]` / `windows=[]` for 189 s — no `[data-status-json]` host anywhere in generate mode | **implemented + natively proven** (`meshes=1` after `addGeneration`); needs a **wasm restage** before it can be seen on 6018 |

---

## 0. State found on disk

The stalled agent had already written the transactional module
`🏛️ShellHost/🔀️surface-switch/🟦️.ts` (17 277 B, 04:56) — `SessionAppSwitchQuiesceV1`,
`SealedInstanceLedgerV1`, `SessionAppSwitchStatusV1`, the gate, and a `runSessionAppSwitchV1` whose
port type demands `quiesce`/`seal`. Nothing else: `ShellHost/🟦️.tsx` still built the OLD ports object
and still typed `switchToPluginApp` as returning `ActiveSession | null` straight out of the unit, and
both the fixture (`🧫️fixtures/🔀️surface-switch/🔣️.json`, 03:17) and the suite
(`🧪️tests/🔀️surface-switch/🟦️.ts`, 03:28) were the pre-transactional versions. The generate-mode
windows under `…/🎭️modes/🧬️generate/` were untouched since 09-08.

Baseline typecheck of `@semio-tech/framework-renderer-react` (`🗑️generated/role-switch-runtime/typecheck-baseline.txt`):
**875 `error TS` lines tree-wide** (a peer's `Command is not generic` refactor dominates). Seven of
them were this lane's:

```
🧱️elements/🏛️ShellHost/🟦️.tsx(4930,7): error TS2739: Type 'SessionAppSwitchOutcomeV1<AppDefinition, PluginViewState>' is missing … pluginId, instanceId, app, viewState
🧱️elements/🏛️ShellHost/🟦️.tsx(4931,9): error TS2345: … is not assignable to parameter of type 'SessionAppSwitchPortsV1<AppDefinition, PluginViewState>'
🧪️tests/🔀️surface-switch/🟦️.ts(263,68) (265,27) (266,27) (292,68) (293,27)
```

After this lane: **867**, and **zero** referencing `surface-switch` or the switch region of
`ShellHost` (`typecheck-after-2.txt`). The five ShellHost errors that remain (1967, 1979, 8170, 8171,
8882) and `🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts(86,5)` are all in the baseline, at peers'
lines (`LeftoverWorldSelectionOverlayV1`, `InteractionState`).

---

## 1. Defect A — root cause as the evidence actually supports it

`🗑️generated/journey-3/console.txt`, after 8 example picks and a generate/edit mode round trip:

```
347181 [DEBUG] command ingress lane {"instanceId":1,"actionId":"interactionSelect","seq":63,"lane":"Interactive"}
341031 [DEBUG] extension completion submitted {instanceId: 1, req: 9n, status: ok, …}
        ← ⌘️⌥️V pressed at ~350.2 s, with BOTH still in flight
351352 error [DEBUG] typed-operation completion effects failed Error: plugin-ui.intake-rejected:intake:actor-activation.revoked
351352 error [DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-brep, capability: evaluate, req: 9n, error: … no actor for instance 1 …}
351352 error [DEBUG] action failed interactionSelect … Error: actor-activation.revoked   (at CapturedShardActivation.assertActive)
351378 pageerror Error: actor-activation.revoked
354312 … 391918  19 further `[DEBUG] action failed … no actor for instance 1 …`
```

Three facts pin it down:

1. `no actor for instance 1` is only reachable from `requireActorId`
   (`🔌️PluginRuntime/🟦️.tsx:2006`) when `closingInstances.has(1)` or the actor map entry is gone —
   both of which only `destroyApp(1)` produces. **The close ladder ran.**
2. The console never mentions instance **2** at all, and every later `performInvocation` still carries
   `"instanceId":1`, while `results.json` records `roles: {editor:"true", viewer:"false"}` for the
   `viewer-role` step and the same `['procedural-preview']` window list. **`SET_SESSION` never
   landed.**
3. The two errors at 351352 are a typed-operation completion pass and an extension round trip that
   were **started before the chord and finished after the retire**, on a revoked activation.

So the switch tore the predecessor down while its own work was mid-turn, the revocation propagated
back into the switch's own await chain, and the successor was never published — a half-applied
switch, which is precisely what a transaction must make impossible.

## 2. Defect A — what was built

### 2.1 `🏛️ShellHost/🔀️surface-switch/🟦️.ts` (owned unit, no React, no shell imports)

Added to what the stalled agent left:

| export | role |
|---|---|
| `SessionWorkKindV1` | `"typed-operation" \| "extension-invocation"` — the two things that hold a captured activation for a whole round trip |
| `createSessionWorkLedgerV1()` | per-`(pluginId, instanceId, kind)` counter; `pending` is the sum, `outstanding` names the split so a refusal can say what it waited on; `begin` answers an **idempotent** release, so a `finally` that runs twice cannot drive it negative |
| `SealedInstanceLedgerV1.unseal` | the rollback a failed `createApp` needs — see the order below |
| `SESSION_SWITCH_QUIESCE_BUDGET_MS = 12_000`, `…_POLL_MS = 40` | sized off the measured worst case: the `interactionSelect` + `flow-extension-brep evaluate` round trip above took **4.2 s** (347.2 s → 351.4 s) |
| `quiesceSessionWorkV1(pending, {budgetMs, pollMs, now, sleep})` | polls until quiet or the budget runs out; pure over its clock and its sleeper |
| `SEALED_INSTANCE_DROP_CODE`, `sealedInstanceDropV1`, `sealedInstanceDropTextV1` | the ONE typed diagnostic a sealed instance's late work may produce |
| `SURFACE_SWITCH_BUSY_LABEL`, `surfaceSwitchBusyTextV1(locale)` | the refusal text, en + de, authored in the unit that decides the refusal — no shell dictionary, no default language |

The order `runSessionAppSwitchV1` enforces:

```
quiesce(current) → [draining, nothing touched]   if it will not go quiet
seal(current)    → every late effect is stale from here
createInstance   → [create-failed: unseal(current), rethrow, session untouched]
retire(current)  → the close ladder (whose own first step revokes the actor)
publish → seedLayout → refresh
```

**`seal` moved in front of `create`** on the evidence of the first 6018 run (§5.1): `createApp` for the
second instance of the same plugin **itself revokes the predecessor's captured activation** — four
`actor-activation.revoked` failures landed 26 ms after the `create` trace and 0 ms after where `seal`
used to sit. So the window the seal exists to close was open for exactly one `createApp`. The
predecessor is kept, not lost, when the create then fails: `unseal` puts it back and the error is
rethrown, so the surface the user is still looking at is never permanently sealed.

### 2.2 `🏛️ShellHost/🟦️.tsx`

New region `//#region 🔀️SurfaceSwitch` (beside `🏁️OperationSettle`):
`sessionWorkRef`, `sealedInstancesRef`, `sessionSwitchGateRef`, `surfaceSwitchBusy` state, and
`dropForSealedInstance(target, what, detail)` → one `console.warn` line, `true` when it dropped.

Wiring:

| site | change |
|---|---|
| `switchToPluginApp` (`:4950`) | runs through `sessionSwitchGateRef.current.run(…)`; new `quiesce`/`seal` ports; `trace` prints `[DEBUG] surface-switch <step> <detail>`; `draining`/`busy` show `surfaceSwitchBusyTextV1(uiLocale)` through `showTransientNoticeRef` and return `null`; `setSurfaceSwitchBusy` around the whole thing |
| `onAction` guest dispatch (`:6355`) | `sessionWorkRef.begin(target, "typed-operation")`, released in the existing `.finally` — the entry spans `handleAction` → host effects → `awaitOperationSettle`, i.e. the whole round trip a switch must outlive |
| `onAction` head (`:6252`) | `dropForSealedInstance(targetSession, "action", action.action)` → return |
| `onAction` catch | a failure on a sealed instance is a drop, not an `[DEBUG] action failed` stack |
| `applyHostEffects` head (`:5150`) | `dropForSealedInstance(baseSession, "host effects", …)` → return |
| `invokeExtension` branch (`:5414`) | `sessionWorkRef.begin(requester, "extension-invocation")` released in a new `.finally`; the answer callback and the catch both drop when the requester is sealed |
| `roleSwitcherElement` (`:8736`) | `aria-busy` on the `ButtonGroup`, `disabled` on the inactive item while a switch runs |

`switchToPluginApp`'s two `applyShellUri` call sites already guarded `null` (`:5643`, `:5653`), so the
`draining`/`busy` return needed no caller change.

## 3. Defect A — the laws

`🧫️fixtures/🔀️surface-switch/🔣️.json` gained `quiesce` per switch row, `expected.status`,
`expected.trace`, `expected.pending`, and five new sections: `gate`, `work`, `quiesce`, `sealed`,
`busyLabel`. `🧪️tests/🔀️surface-switch/🟦️.ts` reads all of them, each row at least twice:

- **switch rows (8)** — port-call order AND the typed `trace` vocabulary, against an independent
  in-file oracle (`oracleSwitchTrace`) that derives the owed steps from the request alone, plus four
  explicit orderings: `quiesce < retire`, **`seal < create`**, `create < retire`, `retire < publish`,
  and "a refused switch retires nothing". Two new rows:
  `a-predecessor-that-will-not-go-quiet-refuses-the-switch` → `draining`, `pending: 2`, `retired: []`;
  `a-successor-that-cannot-be-created-unseals-the-predecessor-it-keeps` → the call REJECTS,
  `trace = [quiesce, seal, create-failed, unseal]`, `created: 0`, `retired: []`, and the predecessor is
  the only thing ever unsealed.
- **gate** — a second chord arrives while the first switch is parked inside its quiesce pass: it is
  answered `busy` with the mounted session intact, the first still completes, exactly one tear-down
  and one `createApp` ran.
- **work (3)** — both kinds count together, instances count apart, a double release is a no-op;
  oracle = `begins − distinct releases`.
- **quiesce (3)** — settles on the last round trip (3 sleeps), never sleeps when already quiet,
  refuses inside the budget — driven with a fake clock.
- **sealed** — the bounded ledger (slots 3, five seals incl. one duplicate → size 3, oldest evicted),
  `unseal` taking exactly one back out, and both drop-text shapes.
- **busyLabel** — `en` / `de`.
- **node twin** — the twin now transpiles and runs `sessionInstanceKeyV1`,
  `SEALED_INSTANCE_LEDGER_SLOTS`, `createSealedInstanceLedgerV1`, `SEALED_INSTANCE_DROP_CODE`,
  `sealedInstanceDropV1`, `sealedInstanceDropTextV1`, `SURFACE_SWITCH_BUSY_LABEL`,
  `surfaceSwitchBusyTextV1` in a bare `node -e` process over the SHIPPED source text, alongside the
  four resolution laws it already ran. 33 twin rows.

Run (foreground, `🗑️generated/role-switch-runtime/surface-switch-2.txt`):

```
> nx run @semio-tech/framework-renderer-react:surface-switch-check
[DEBUG] surface-switch boot=10 group=4 roleTargets=4 switch=8 work=3 quiesce=3 sealed=5 busyLabel=2 modeSteps=6 keybindings=4 twin=33 PASS
 ✓ surface switch > resolves the boot role, gates the role group, retires the predecessor on every switch, and keeps both axes on the keyboard 656ms
 Test Files  1 passed (1)      Tests  1 passed (1)
```

---

## 4. Defect B — generate-mode entry state

### 4.1 What was actually missing

`addGeneration` was already reachable from the UI: `generation_tree`
(`🫀️core/🖼️semantic-ui/🦀️.rs:56`) renders an "Actions" section with an **Add Generation** tree item
bound to `factory.action("addGeneration", None)`, and the form window already hints
(`generate_form_hints_without_a_selected_generation`). The hole was the preview:

```rust
if payload.meshes_json == "[]" && payload.instances_json == "[]" {
    let scene = TextEditorScene::base(text.to_string(), Some("json".into()), None);
    return crate::scene_surface(…, SurfaceKind::TextEditor, &scene);   //  ← no status host
}
```

`data-status-json` / `data-meshes-json` are published only by `🌐️World3dHost` and `🕸️NodeGraph`, so a
text-editor fallback means **no host at all** — which is exactly the `hosts: []`, `windows: []` the
probe recorded for the whole 189 s of generate mode.

### 4.2 Change (`…/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`)

The window is now a `World3d` surface in **every** state. New
`generate_preview_status_json(session, eval_json, payload, preview_status, hint)` merges
`preview_scene_status_json` (phase / phaseLabel / progress / cancellable / cancelAction, and
`preview_status_json` over the patched generation fixture once one is selected) with the `debug`
counters the browser probe reads, and adds `"hint"` **only** while there is nothing to show. The
`TextEditorScene` fallback and its import are gone (no compat path).

### 4.3 Laws (native, foreground)

`…/🎭️modes/🧬️generate/🧪️tests/👁️preview/🔬️unit/🦀️.rs` —
`generate_preview_entry_state_is_an_idle_status_host_with_a_hint` (replaces the old text-editor
assertion):

```
[DEBUG] generate preview entry status={"phase":"faulted",…,"debug":{"evalLen":0,"meshesLen":2,"instancesLen":2},"hint":"(evaluate a generation to preview output)"}
test result: ok. 1 passed; 0 failed; … 373 filtered out
```

(`faulted` is correct for the bare `app()` harness — no plugin contributes the `brep` flow extension
there. The registry-backed law below pins `idle`.)

`…/🧪️tests/🔬️tick-addressing/🦀️.rs` —
`add_generation_from_the_generations_window_drives_the_generate_preview_to_a_rendered_mesh`, both
halves of the brief in one law: the entry state under a contributed geometry extension, then
`addGeneration` → the addressed tick chain → real tessellated geometry.

```
[DEBUG] addGeneration generate-preview chain: ticks=2 meshes=1 status={"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},"progress":{…,"ratio":1.0},"cancellable":false,"cancelAction":"cancelPreviewEval","debug":{"evalLen":1209,"meshesLen":1089,"instancesLen":223}}
```

Whole module and whole generate mode green:

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- editor::generation3d::component::tick_addressing::
  only_a_preview_addressed_tick_passes_the_retained_preflight ... ok
  add_generation_from_the_generations_window_drives_the_generate_preview_to_a_rendered_mesh ... ok
  generate_preview_eval_emits_extension_or_rearms_flow_eval_tick ... ok
  set_active_example_drives_the_self_dispatched_tick_chain_to_a_rendered_mesh ... ok
  every_armed_tick_names_a_preview_window_that_is_actually_attached ... ok
test result: ok. 5 passed; 0 failed

cargo test … -- modes::generate
  the_generate_layout_lists_all_three_windows / generate_form_hints_without_a_selected_generation /
  generate_mode_renders_surfaces / generate_preview_entry_state_is_an_idle_status_host_with_a_hint
test result: ok. 4 passed; 0 failed
```

### 4.4 Restage required

**This is a guest change.** `http://127.0.0.1:6018` serves the last staged `generation3d` wasm, so the
generate preview on 6018 keeps returning the text-editor fallback until the procedural / generation3d
guest is restaged. Per the lane brief I did **not** start a wasm build; the proof above is native and
stops there. The `switch-3` probe's own generate step confirms the staged guest is still the old one —
`windows=[] hosts=[]` after `Meta+Alt+ArrowRight`, exactly the journey-3 symptom — which is what a
restage is expected to turn into one `window:…generate-preview` host with `phase: "idle"` and the hint.

### 4.5 Pre-existing red in the same crate (NOT this lane)

The full lib run is 369 passed / 5 failed. All five fail identically with this lane's generate-preview
change reverted to `HEAD` (`🗑️generated/role-switch-runtime/rust-head-baseline.txt`, the two touched
files swapped out and restored by md5):

```
add_generation_records_an_undoable_generation_operation          — "undo did not revert to the expected snapshot" (left 1, right 0)
generation_preview_is_one_app_transient_shared_by_two_generation_windows — "Generation3d preview operation did not finish" (30 s typed-operation publication timeout, before any render)
two_instances_converge_disjoint_widget_moves                     — module.vcs "remote snapshot merge is fail-closed until …"
undo_redo_round_trips_flow_graph_edits
vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed
```

---

## 5. Browser verification (6018)

Probe: `🐍️role-switch-runtime-probe.mjs` (new, kept in the ticket folder) — boots, converges, fires an
example pick and presses `Meta+Alt+V` **150 ms later** so the switch lands mid-chain, then does a quiet
`Meta+Alt+E` / `Meta+Alt+V` pair and a `Meta+Alt+ArrowRight`. It records the roles group, its
`aria-busy`, the mounted windows, every `[data-status-json]` host, every `surface-switch` trace line,
every `shell.surface-switch.sealed-instance` drop, and every `no actor for instance` /
`actor-activation.revoked` / `pageerror`.

Run: `cd <ticket> && SEMIO_PROBE_OUT=role-switch-runtime/switch-3 SEMIO_PROBE_SETTLE_WAIT=100 bun 🐍️role-switch-runtime-probe.mjs`
→ `🗑️generated/role-switch-runtime/switch-3/{steps.json,summary.json,console.txt,*.png}`.
Three runs are recorded (`switch-1`, `switch-2`, `switch-3`); **`switch-3` is the authoritative one** —
see §5.2 for why the first two ran against a stale served module.

### 5.0 Conditions

Peers were editing the same live-reloaded tree throughout (`🛠️ShellHelpers/🟦️.tsx`, `🕸️NodeGraph/🟦️.tsx`,
`🌐️World3dHost/🟦️.tsx` all rewritten at 05:52 / 06:00 / 06:04 / 06:05 while probes ran). A first
attempt with the long `🐍️journey-probe.mjs` had to be abandoned: every example step sat at the full
180 s without converging and the flow graph screenshot showed `Polygon Queued` / `Vector Computing` /
`ExtrudeCurve Blocked` — an evaluation stall from a peer's half-written module, not from this lane
(nothing this lane touches runs before a switch is requested). The focused probe below boots and
converges normally, so the readings are usable; they are readings of a tree several lanes are writing.

### 5.1 First run — `switch-1` (before `seal` moved in front of `create`)

Trace and counts (`switch-1/summary.json`):

```
126742 [DEBUG] surface-switch quiesce settled:0
127452 [DEBUG] surface-switch create s.procedural.generation3d@1/*#viewer:2
127452 [DEBUG] surface-switch seal procedural#1
195707 [DEBUG] surface-switch retire-failed procedural#1
195707 [DEBUG] surface-switch publish / seed / refresh  s.procedural.generation3d@1/*#viewer
181162 [DEBUG] surface-switch outcome busy …#viewer  pending=0      ← gate refused an overlapping chord
199410 [DEBUG] surface-switch outcome busy …#editor  pending=0      ← and another
204958 [DEBUG] surface-switch outcome switched …#viewer pending=0
noActor: 0      pageErrors: []      drops: []      revoked: 4
```

DOM at `post-viewer` (`switch-1/steps.json`):

```
roles = {editor: "false", viewer: "true"}      windows = ["procedural-view-preview"]
hosts = [("window:procedural-view-preview", …)]
```

and during the switch, `mid-chain` / `post-editor`: `rolesBusy = "true"` — the group's `aria-busy`.

**Against journey-3: `no actor for instance` 20 → 0, `pageerror` 1 → 0, and the switch actually
completed** (journey-3 kept `editor` pressed and the `procedural-preview` editor window forever).

Two things this run exposed:

1. The four remaining `actor-activation.revoked` lines are a single burst **26 ms after `create`**:
   ```
   127478 [DEBUG] typed-operation completion effects failed Error: plugin-ui.intake-rejected:intake:actor-activation.revoked
   127479 [DEBUG] history snapshot failed Error: actor-activation.revoked
   127479 scheduled dispatch of "flowEvalTick" failed Error: actor-activation.revoked   (×2)
   ```
   i.e. **`createApp` for the second instance of the same plugin revokes the first instance's captured
   activation**, before the close ladder is anywhere near. That is what moved `seal` in front of
   `create` (§2.1). All four are handled `console.error`s, not page errors.
2. `retire` took **68 s** and then failed with `plugin-ui.owner-close-budget-exhausted`
   (`🔌️PluginRuntime` `retireInstanceLifecycle`). The transaction behaved exactly as designed — the
   failure was reported through `onRetireFailed`, never blocked the successor, and the two chords that
   arrived during those 68 s were refused `busy` with the mounted session intact — but a 68-second
   close ladder is its own defect, in `🔌️PluginRuntime/🟦️.tsx`, **outside this lane's files**. Flagged,
   not touched.

### 5.2 Second run — `switch-2`

Same counts — `noActor: 0`, `pageErrors: []`, `drops: []`, `revoked: 4` — and this time the
**mid-chain** step itself caught the completed switch (the 45 s window in run 1 expired inside the
68-second close ladder; run 2 waited 90 s):

```
114506 mid-chain-immediate  roles={editor:"true",  viewer:"false"}  rolesBusy="true"   windows=["procedural-preview"]
185664 mid-chain            roles={editor:"false", viewer:"true"}   rolesBusy="true"   windows=["procedural-view-preview"]
                            hosts=[("window:procedural-view-preview", …)]
188516 [DEBUG] surface-switch outcome busy    …#editor  pending=0
192241 [DEBUG] surface-switch outcome switched …#viewer pending=0
```

That is the DOM the brief asks for — `window:procedural-view-preview` present, the roles group's
`viewer` button `aria-pressed="true"`, `aria-busy="true"` for the duration, and **zero
`no actor for instance` errors** — reached by a chord pressed while the shell was mid-chain.

But the trace still read `create` → `seal`, i.e. **the page was running the pre-reorder module**. Cause,
proven directly rather than guessed: the dev server's transform cache for that file had gone stale
under the peers' edit burst. Fetching the served URL bare returned the old text while the same URL with
a cache-busting query returned the new one:

```
curl …/🔀️surface-switch/🟦️.ts            → 119: const instanceId = await ports.createInstance(…)
                                             122: trace("seal", …)              ← pre-reorder
curl …/🔀️surface-switch/🟦️.ts?t=<now>    → 135: let instanceId;
                                             139: trace("create-failed", …)
                                             141/142: trace("unseal", …) / ports.unseal(current)
```

A `touch` on the two files (content unchanged; no server restart — the lane brief forbids one)
invalidated the cache and the bare URL then served the current text.

### 5.3 Third run — `switch-3` (fresh modules) — the authoritative reading

Full trace, both directions (`switch-3/summary.json`):

```
114181 [DEBUG] surface-switch quiesce settled:0
114181 [DEBUG] surface-switch seal    procedural#1          ← seal now precedes create
114726 [DEBUG] surface-switch create  …#viewer:2
176470 [DEBUG] surface-switch retire-failed procedural#1     (plugin-ui.owner-close-budget-exhausted, 62 s)
176470 [DEBUG] surface-switch publish / seed / refresh …#viewer
180990 [DEBUG] surface-switch outcome switched …#viewer pending=0

181625 [DEBUG] surface-switch quiesce settled:0
181625 [DEBUG] surface-switch seal    procedural#2
183774 [DEBUG] surface-switch create  …#editor:3
204272 [DEBUG] surface-switch retire  procedural#2           ← this one retired cleanly
204272 [DEBUG] surface-switch publish / seed / refresh …#editor
209222 [DEBUG] surface-switch outcome busy     …#viewer pending=0
387566 [DEBUG] surface-switch outcome switched …#editor pending=0

noActor: 0     pageErrors: []     drops: []     revoked: 4
```

DOM (`switch-3/steps.json`):

```
114302 mid-chain-immediate  roles={editor:"true",  viewer:"false"}  rolesBusy="true"
179583 mid-chain            roles={editor:"false", viewer:"true"}   rolesBusy="true"
                            windows=["procedural-view-preview"]   hosts=[("window:procedural-view-preview", …)]
314451 post-viewer          roles={editor:"true",  viewer:"false"}
                            windows=["procedural-preview"]        hosts=[("window:procedural-main", …), ("window:procedural-preview", …, "idle")]
```

So: the chord pressed mid-chain now completes the whole editor → viewer transaction and the shell
shows `window:procedural-view-preview` with the roles group's `viewer` button `aria-pressed="true"`;
the return trip viewer → editor completes too; a chord that arrives while one is running is refused
`busy` with the mounted session intact; the group carries `aria-busy` throughout; and **the twenty
`no actor for instance 1` failures and the `pageerror` from journey-3 are both zero**.

**The four `actor-activation.revoked` lines that remain**, all at 114749 — 23 ms after `create`,
568 ms *after* the seal:

```
[DEBUG] typed-operation completion effects failed Error: plugin-ui.intake-rejected:intake:actor-activation.revoked
[DEBUG] history snapshot failed Error: actor-activation.revoked
scheduled dispatch of "flowEvalTick" failed Error: actor-activation.revoked        (×2)
```

These are three SHELL-owned background passes (a completion's effect pass, a history-snapshot read,
two `scheduleDispatchAction` timers) that were **already in flight before the seal**, so no entry guard
can stop them — only cancellation could, and neither the completion subscription nor
`scheduleDispatchAction` (`🛠️ShellHelpers`, another lane's file) carries a cancellation token today.
They are handled `console.error`s on an instance that is being torn down: no page error, no action
failure, no user-visible consequence. Registering them in `sessionWorkRef` so `quiesce` waits for them
is the clean follow-up, and it needs `scheduleDispatchAction` to answer a handle — a change in the
`dispatch-timer-throttle` lane's file, not this one.

Two further findings, both **outside this lane's files**, recorded rather than fixed:

- **`plugin-ui.owner-close-budget-exhausted`** — the first close ladder took 62 s and then failed
  (`🔌️PluginRuntime` `retireInstanceLifecycle`). The transaction handled it exactly as designed, but a
  minute-long retire is its own defect.
- **The 6018 transform cache goes stale under a peer edit burst** (§5.2): a bare module fetch returned
  pre-edit text while `?t=<now>` returned the current one. `touch` on the file invalidates it. Worth
  knowing before trusting any browser reading taken during a multi-lane session.

---

## 6. Files

Changed:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🧪️tests/👁️preview/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs`

Added:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️role-switch-runtime-probe.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️role-switch-runtime-2026-09-12.md`

Outputs (delete with the ticket): `🗑️generated/role-switch-runtime/`.
