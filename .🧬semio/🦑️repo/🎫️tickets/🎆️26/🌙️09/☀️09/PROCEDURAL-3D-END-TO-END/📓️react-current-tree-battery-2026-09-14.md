# React Current-Tree Battery — lane `react-current-tree-battery` (2026-09-14, session 5)

Port 6018 (React dev serve). This lane's job: prove the generation3d editor works end to end on the
CURRENT tree, turn the battery's three `undecided` keyboard steps into real verdicts, and fix the
product defects that stand in the way.

- Evidence: `🗑️generated/react-s5/` (restage logs, boot console, native laws) and
  `🗑️generated/react-verify/scoreboard.json` (the battery's own root).
- Probes changed: `🐍️editor-verbs-keyboard-probe.mjs`, `🐍️react-battery.mjs` (`keyboard-verbs` verdict).
- Script added: `📜️restage-retry-s5.sh`.

---

## 1. What was stale

| thing | state at 17:05 | why it mattered |
|---|---|---|
| served guest wasm | 15:31 | one file of its dependency closure was newer |
| `🗣️dsl/✨️derive/🦀️.rs` | 16:57 | a PROC MACRO in the closure — a change there invalidates every crate downstream, so the whole guest was formally stale (the change itself, a Windows `\\?\` canonicalization strip, is a no-op on macOS) |
| `🔌️plugin/⏯️tool-run/🦀️.rs` | 17:14, 17:28 | the peer's `recent_trace` de-duplication and the new ready group — landed AFTER the served wasm |
| host TS on 6018, 17:00–19:40 | fresh | `curl` of `/@fs/<abs path>` for the modules edited at 14:55–15:08 and 18:3x matched disk: vite re-transforms on change even with `SEMIO_VITE_HMR=0` |
| host TS on 6018, 23:10 | **STALE** | the serve (up since 13:22) held a `📃️UiDocumentStore/🟦️.tsx` transform with no `publishGuestPresenceV1` export while disk had it, so the page died on `SyntaxError: … does not provide an export named 'publishGuestPresenceV1'` and mounted nothing. Recycled by pid (listener + its `bun … serve` parent), `node_modules/.vite-temp` cleared, restarted under `screen -dmS g3dreact 📜️serve-generation3d-react.sh`, `curl` re-checked, boot clean. A long-lived vite serve DOES eventually miss an edit; check the served module, never the file date |
| served guest wasm, 23:03 | **STALE, reported green** | see §4.1 — an nx cache hit skipped the rebuild |

**Method note for the next reader:** `grep` treats these source files as BINARY (emoji + long lines) and
prints nothing without `-a`. A `grep -rn` that finds no hit in `🟦️.tsx`/`🦀️.rs` is very likely a false
negative — always `grep -a`. Half an hour went into a "the serve is stale, the symbol is gone from disk"
conclusion that was only a suppressed grep.

Restage command (foreground, retried through peer churn): `📜️restage-retry-s5.sh` →
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`.
First restage 17:29 → 18:12 green. Two later attempts died on shared-tree failures that were NOT this
lane's: `No space left on device` writing the wasm32 incremental cache (239 GB under
`⚡️cache/cargo/build`), and a peer's `DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE` half-landed in
`♾️infinite/🎲️board`. Both cleared by themselves; the retry loop is the right shape for this tree. The
battery's own stage is the later 23:04 → 23:07 run of `📜️restage-nocache-s5.sh` (§4.1).

Two ceilings owned by lane `contributions-ingress-ceiling` blocked every run between 19:00 and 19:39 —
`command ingress exceeds 64 pages`, then `setContributions … rejected 273136 raw bytes before decoding;
maximum is 262144` — so the editor booted with no `brep` extension at all. Both are theirs and both are
gone; this lane only measured them (`🗑️generated/react-s5/boot-1905`, `boot-1923`, `boot-1934`).

---

## 2. The defect that stopped every evaluation

**Symptom** (boot console, `🗑️generated/react-s5/boot/console.txt`, 17:14 run on the 15:31 wasm): 17
lines of

```
typed-operation completion effects failed SemioFaultError: 1:framework.panel.toolRun: DuplicateSiblingKey
    at retainedUiRefreshEffects (🔌️PluginRuntime/🟦️.tsx)
[DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-brep, capability: tessellate, req: 6n, …}
```

one per extension round trip (`flow-extension-math evaluate req 1`, `flow-extension-brep evaluate`
and `tessellate` req 2..6). The preview pill stayed `Running · Evaluating nodes (1/2)` and no example
ever converged.

**Chain.** Three layers, each wrong on its own terms:

1. **The guest published a panel with duplicate sibling keys.** `ToolRunEntry::apply_tick` pushed every
   `ToolRunTraceOp::Upsert` key onto `recent_trace` without removing an earlier copy, and ignored
   `Retire` entirely. A trace key is re-upserted whenever a node's verdict changes
   (testing → success/danger — the normal life of every evaluated node), so the panel's Attempts tree
   emitted `framework.toolRun.<run>.trace.<key>` twice and the retained-UI producer refused the whole
   surface. **Fixed by the `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` peer at 17:14**, with a law
   (`tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position`) — not by this lane.
   This lane confirmed the cause from their diff and verified the law runs green (§5).

2. **One surface's render fault aborted every other operation's completion effects.** THIS lane's fix.
   The guest reports a mounted surface's terminal through `shell_fault_effect` on whatever turn its
   reconciler happened to terminate on, which is almost never the turn that caused it. The React host
   read EVERY `AppFrame::Error` in a turn's effects as that turn's answer and threw, so
   `applyHostEffects` rejected and the `flow-extension-brep` completion lost its effects — the
   evaluation result never landed. A surface-render fault is now scoped to the surface it names:
   reported (`console.error`, still matched by the battery's fault regex), the turn keeps its other
   effects, and every other `Error` frame still throws.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
     — `SURFACE_RENDER_FAULT` + the `retainedUiRefreshEffects` law.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — `SURFACE_RENDER_FAULT_CODE`,
     the Rust half of the same wire constant; `⚛️reactor/🔄️turn/🦀️.rs` now emits it by name.
   - Law: `🧪️tests/🔌️plugin-runtime/🟦️.tsx` — "scopes a surface-render fault to its surface and keeps
     the same turn's other effects" (the pre-existing "reports a refresh fault frame" throw law is
     untouched and still green).

3. **The fault named a surface and nothing else.** A `DuplicateSiblingKey` in a hundred-node panel gave
   an author no way to find the node. `ComponentTreeProducer::duplicate_sibling` now reads the refused
   child out of its frame (the producer already retains both), and `PatchTracker::take_render_fault`
   reports `<surface> [producer]: DuplicateSiblingKey parent=<key> key=<key>`. It also names WHICH
   authority read the fault, because the two know their surface differently: a producer terminal carries
   its own `SurfaceId`, while a reconcile terminal is matched back to a slot BY GENERATION and degrades
   to `unknown surface` once that slot is gone — a reader could not previously tell an exact attribution
   from a derived one.
   - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs`,
     `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`,
     fixture `⚛️reactor/🩹️patches/🧫️fixtures/🗂️catalogue-surface.json`,
     law `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`.

**A continuation dispatched every extension answer with no view.** THIS lane's fix, and the next
thing that stopped every evaluation once the duplicate key was gone. A peer's wave made
`flowEvalResolve` and `flowTessellateResolve` window-addressed routes
(`ArtifactApp::retained_window_transient_target`, so they publish the wave they continue into the
addressed preview window's transient). But an extension answer is redispatched by the GUEST's own
reactor — `Event::Completed` → `plugin_dispatch_response_action` — and that door built its
`ActionMeta` with `view_state: None`. Every answer therefore faulted
`targeted window transient capture requires an exact ViewModel roster`, `invokeExtension dispatch
failed` for `flow-extension-math evaluate` and every `flow-extension-brep evaluate`/`tessellate`, and
the preview sat at `phase: "faulted" — Geometry extension unavailable` with zero meshes
(`🗑️generated/react-s5/boot-now/console.txt`, 19:40). A continuation now carries the instance's LAST
HOST VIEW, exactly as `ArtifactApp::pending_effects` already did
(`SurfaceRegistry::view()` — "the attached-window roster an app needs to address window-scoped
background work outside a render pass").
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `plugin_dispatch_response_action`.

**The ToolRun pill's frozen label.** Reported by the coordinator on 6021 at 18:28: a `Finalized` run
still read `Evaluating nodes (1/2)`. `tool_run_panel_group` published the last ANNOUNCED string rather
than the current one — the announcement throttle was implemented by refusing to change the text, and a
sighted reader watches the same string a screen reader hears. The text is now always current and only
`Liveness` is rationed (`Off` between announcements, `Polite`/`Assertive` on a state change or once per
`TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`.

---

## 3. The three `undecided` keyboard steps, now decided

`🐍️editor-verbs-keyboard-probe.mjs` used to press every chord in edit mode on a quiet shell and the
battery exempted the three rows that could not possibly do anything there. Both sides are fixed: the
probe presses each chord in the state that OWNS it and publishes its own `ok` plus the `oracle`
sentence it was decided by; the battery's `keyboard-verbs` verdict has no exemption list left and
treats a row without an `ok` as red.

| row | pressed in | oracle |
|---|---|---|
| `baseline` | boot, no chord | the preview window published a status AND the shell invoked nothing |
| `add-generation` (`mod+shift+g`) | GENERATE mode, after `Meta+Alt+ArrowRight` mounts the generate preview | `addGeneration` is invoked AND the Generations roster grows by exactly one row |
| `cancel-preview-eval` (`mod+.`) | on an evaluation in flight — the slowest example loaded, polled until the preview publishes `cancellable: true` | `toolRunAbort` is invoked AND the preview stamps `phase: "cancelled"` with `cancellable: false`, which is `🧫️fixtures/🛑️preview-cancel.json`'s own cancellation law |

Two new rows come with them (`generate-mode`, `cancellable-evaluation-armed`), so a failure can say
whether the chord missed or the STATE was never reached.

---

### 3.1 Every file this lane changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — `SURFACE_RENDER_FAULT`; `retainedUiRefreshEffects` scopes it instead of throwing.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — `SURFACE_RENDER_FAULT_CODE`, the Rust half of that wire constant.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — emits the code by name.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs` — `ComponentTreeProducer::duplicate_sibling`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` — `take_render_fault` names the authority, the parent and the refused key.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` — the ToolRun status text is always current; only `Liveness` is throttled.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `plugin_dispatch_response_action` carries the instance's last host view.

Laws and fixtures:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` and `⚛️reactor/🩹️patches/🧫️fixtures/🗂️catalogue-surface.json`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`.

Ticket:
- `🐍️editor-verbs-keyboard-probe.mjs` (rewritten), `🐍️react-battery.mjs` (`keyboard-verbs` verdict),
  `🐍️example-picker-probe.mjs` (new), `📜️restage-retry-s5.sh` and `📜️restage-nocache-s5.sh` (new),
  this report.

---

## 4. Battery

One stage, one port. Guest wasm restaged 23:07 from the tree (`NX_SKIP_NX_CACHE`, see §4.1), 6018
recycled at 23:09, `bun 🐍️react-battery.mjs` 23:13–23:54 plus two `--only` re-runs (`keyboard-verbs`
after this lane's oracle fix, `journey` after its click timeout). Scoreboard:
`🗑️generated/react-verify/scoreboard.json`; this run's stdout `🗑️generated/react-s5/battery-run.txt`.

**19 probes, 13 green, 3 page errors (all in `flow-wire`).**

| probe | verdict | steps | seconds | pageerrors |
|---|---|---|---|---|
| `boot` | green | 3/3 | 103 | 0 |
| `journey` | RED | 24/25 | 195 | 0 |
| `interact` | green | 58/58 | 137 | 0 |
| `generate-mode` | green | 13/13 | 42 | 0 |
| `flow-window` | green | 3/3 | 71 | 0 |
| `flow-wire` | RED | 2/5 | 155 | 3 |
| `flow-reorganize` | green | 3/3 | 129 | 0 |
| `cancel-preview` | green | 4/4 | 17 | 0 |
| `keyboard-verbs` | green | 12/12 | 30 | 0 |
| `io-surface` | green | 8/8 | 61 | 0 |
| `export-encoding` | green | 4/4 | 55 | 0 |
| `status-parity` | green | 12/12 | 201 | 0 |
| `role-switch` | green | 9/9 | 38 | 0 |
| `i18n-a11y` | RED | 5/7 | 75 | 0 |
| `customization-persistence` | green | 4/4 | 55 | 0 |
| `gaps` | RED | 11/13 | 238 | 0 |
| `preview-chrome` | green | 18/18 | 53 | 0 |
| `status-states` | RED | 10/15 | 386 | 0 |
| `menus` | RED | 5/6 | 57 | 0 |

**The end-to-end walk itself is whole.** `journey` converged **23 / 23 rows** — all nine edit picks
(`No example` + eight examples), generate mode, a generation added, back to edit, the viewer role and
all nine viewer picks — at 3–9 s each except `edit:Sphere Box Fuse` (79 s). Its ONE red step is the
suite's `no shell faults` assertion catching a single late
`interactionSelect … no actor for instance 1 (createApp not called, or already destroyed)` raised
during role teardown, not a row that failed to converge. `interact` is 58/58 (six gestures × all eight
examples), `preview-chrome` 18/18, `status-parity` 12/12.

The eight examples are all offered and all pick: `🐍️example-picker-probe.mjs` (new) lists the picker's
own options — `["No example", "Hexagonal Mushroom Column", "Rectangle Extrude Volume",
"Sphere Cut With Torus", "Box Fillet Preview", "Sphere Box Fuse", "Face Sweep Extrude",
"Rectangle Wire Preview", "Box Shell Preview"]`. The first `journey` attempt died on
`click: Timeout 4000ms … [role="option"] … 'Box Shell Preview'` after seven successful picks, which
that probe answers as a transient click timeout under a loaded machine, not a missing example.

### 4.1 The nx cache serves a stale guest

`activate-generation3d-react-dev` reported `procedural-plugin:component-dev [local cache]` and left the
20:14 wasm in place while `🔌️plugin/⏯️tool-run/🦀️.rs` had changed at 21:34 — the project's nx hash does
not cover the framework crate's own sources, so a cache hit served a guest OLDER than the tree while
claiming success. `📜️restage-nocache-s5.sh` (new) runs the same target with `NX_SKIP_NX_CACHE=true`;
that produced the 23:07 guest this battery ran against. **Always verify the served wasm's mtime against
the crate closure after a restage — a green nx run is not evidence.**

### 4.2 `keyboard-verbs`: what the first run found

The rewritten probe was red twice before it was green, and both reds were worth having.

1. `baseline` failed on 23 `toolRunPace` invocations. The app's own run driver paces itself at boot;
   "the shell invokes nothing" was the wrong sentence. The oracle now names the six chord-bound verbs
   under test and forbids exactly those without a gesture — strictly stronger than the old exemption,
   and blind to the app's self-driven evaluation.
2. `cancel-preview-eval` failed once because the press landed when the run had `unitsDone 5/6,
   inFlight 1` and that last unit finished inside the 30 s window. The re-run pressed the chord with
   real work left and the oracle held: `toolRunAbort` + `flowEvalRelease` invoked, the preview stamped
   `phase: "cancelled"`, `cancellable: false`, frozen at `nodesDone 4/6`. To keep a future red
   readable, the probe now falls back to a `cancel-affordance-discriminator` row that clicks the
   status's OWN `cancelAction`/`cancelArgs` affordance on a re-armed evaluation: the chord carries no
   run address and the button does, so a run where the button cancels and the chord does not names the
   chord's addressing rather than cancellation itself. That row did not run — the chord cancelled.

---

## 5. Laws run

| law | where | result |
|---|---|---|
| `mounted_catalogue_reports_producer_failure_once_before_cleanup` (extended: the report names the authority and `parent=…, key=…`) | `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` + fixture `🗂️catalogue-surface.json` | pass |
| `tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position` (the peer's duplicate-key law) | `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` | pass |
| `tool_run_panel_offers_start_for_the_active_run_tool_until_its_run_exists` | same | pass |
| `tool_run_panel_of_a_running_run_is_the_shell_fixture` (covers this lane's `Liveness` change) | same | pass |
| "scopes a surface-render fault to its surface and keeps the same turn's other effects" (new) | `📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | pass |
| "reports a refresh fault frame instead of returning an unchanged surface" (pre-existing, must still throw) | same | pass |
| `cargo check -p semio-framework-ui-runtime`, `cargo check -p semio-framework-plugin` | — | clean (warnings only, i.e. expansion really ran) |

Command for the Rust four: `cargo test -p semio-framework-plugin --lib -- mounted_catalogue_reports_producer_failure tool_run_panel`
(`🗑️generated/react-s5/native-tests.txt`, 4 passed). For the two TS laws:
`SEMIO_TEST_LEVEL=long vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx --testNamePattern="surface-render fault|refresh fault frame"` (2 passed).

---

## 6. Not claimed

- **The six red probes are NOT this lane's.** `flow-wire` (wire redraw after a cut, 3
  `FlowMessageRejected` page errors), `i18n-a11y` (the graph's own node/wire/port labels and the `ok`
  status word stay English under `de`), `gaps` (`export-after-generate` export dialog click times out;
  `doc-panel-select`), `status-states` (`Stale`/`Error`/`Veraltet`/`Fehler` never painted), `menus`
  (`exportDocument` opens no format list). They are characterised above and in the scoreboard, and
  nothing here was changed to make them pass.
- **`journey`'s one red step is a teardown race, not a converged-row failure.** A single
  `interactionSelect` reached instance 1 after it was destroyed. The same class of error appears in
  `status-states` (`no actor for instance 1` on `setActiveExample`). Not diagnosed.
- **The ToolRun pill's `7 nodes evaluated, 2 meshes tessellated` body** (coordinator, 6021 18:28) was
  NOT investigated. The frozen PHASE LABEL beside it is fixed (§2); the body is a step-log row and a
  step log legitimately shows the value at that step, so whether "2 vs 3 meshes" is stale or correct
  is undecided here.
- **The duplicate sibling key itself was fixed by the `INTERACTIVE-TOOLS-VISIBLE-PROCESS` peer**, not
  by this lane. This lane proved the cause from their diff, proved the fault is gone on the current
  stage (0 `DuplicateSiblingKey` in `🗑️generated/react-s5/boot/console.txt`), and closed the two
  layers around it (host isolation, fault diagnosability).
- **The `Liveness::Off` change is not proven against a screen reader.** It is proven against the panel
  fixture law and the published `accessibility.live` value; nobody listened to it.
- **`historyJsonPublished: false`** — no surface carries `data-history-json`, so the keyboard probe's
  undo/redo rows are decided by the invocation the chord mints, never by the framework's history
  cursor. The probe's own docstring claims the history chrome; it does not deliver it.
- **The battery ran against ONE stage.** Peers landed edits during the run (`🔌️plugin/🦀️.rs`,
  `⏯️tool-run/🦀️.rs`, `♾️infinite/🎲️board/🦀️.rs` all moved between 18:00 and 23:00); the numbers above
  describe the 23:07 guest and the 23:09 serve, not whatever the tree is when you read this.
- **wgpu is untouched.** This lane owned port 6018 only.
