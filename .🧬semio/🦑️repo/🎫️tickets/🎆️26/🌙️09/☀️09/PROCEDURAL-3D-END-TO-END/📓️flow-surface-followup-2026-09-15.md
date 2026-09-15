# 🧾️ Flow surface follow-up — lane `flow-surface-followup`, 2026-09-15

The four items `📓️flow-scroll-render-perf-2026-09-15.md` §9 handed on, plus its gate re-run.

Stage: `http://127.0.0.1:6022/?plugin=generation3d`, direct vite serve
(`📜️serve-generation3d-react-6022.sh`), **recycled by pid at 16:19** after the flow wasm rebuild —
`rm -rf node_modules/.vite-temp`, script restarted, freshness guard confirmed in the served document
(`semio dev · transform freshness: stat-guard … serve pid 83980`). Headless Chromium with
`--use-angle=metal`. Examples `hexagonal-mushroom-column` (7 nodes, 6 edges) and
`sphere-cut-with-torus` (6 nodes, 5 edges), both booting the real example graph.

wgpu untouched; :6118 never started. No `git commit/stash/checkout/worktree`, no ticket
open/close/reopen, nothing under `🗑️generated` deleted, no peer process killed.

---

## 1. Result

| item | before | after |
|---|---|---|
| 1 · the three Rust laws | written, **never executed** (crate test binary would not build) | **4 laws green**, run in the foreground |
| 2 · hops for a four-click sequence | 8 (`emitInteractionState` × 4, both lanes each) | **1** — the one click that moved the selection |
| 3 · board attaches per document | reported as 2 | **1** — and the "2" was a probe artifact, proven |
| 4 · flow-surface payload bytes per document | **224 314 B** | **122 817 B**; the 98 KB operator table crosses **once**, not twice |
| 5 · `flow-scroll` gate, both examples | — | **GREEN**, median 5.2–6.3 ms, p95 ≤ 8.2 ms, 0 guest hops during, 1 publication at settle |
| 5 · battery, 6 rows | — | **5/6 green, 0 page errors**; `interact` 54/58, every red attributed |

---

## 2. Item 1 — the three Rust laws, executed

### 2.1 The blocker, and the repair

`cargo test -p semio-framework-os-flow --lib` could not build its test binary: **21 × E0425**, all in
`🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs`. A peer's `fixture` → `host_snapshot` sweep had renamed the
**uses** of local bindings without renaming the bindings themselves — `fn retained_fixture()` still
opened `let mut fixture = FlowHostSnapshot::default();` and then pushed onto `host_snapshot`, and
three `let fixture: Value = parse(include_str!(…))` sites were read back as `host_snapshot["initial"]`.
The memory note `feedback-name-keyed-edits-need-region-guard` is exactly this shape.

Repaired forward, 11 sites, by the sweep's own intent rather than by reverting it
(`🔧️repair-flow-vcs-host-snapshot.py`): a local holding a `FlowHostSnapshot` takes the new name at its
binding; a local holding a parsed JSON fixture **file** keeps `fixture` and the stray use goes back.
`♾️infinite` and `🗺️surface` had already settled — they build and test clean.

### 2.2 The two node-graph laws

```
cargo test -p semio-framework-surface --lib graph_host_
test node_graph::tests::graph_host_wheel_ticks_accumulate_on_the_board_without_a_publication ... ok
test node_graph::tests::graph_host_pan_ticks_accumulate_on_the_board_without_a_publication  ... ok
test result: ok. 26 passed; 0 failed; 218 filtered out
```

### 2.3 The two flow-credit laws — and the precondition that was wrong

`one_poll_spends_its_whole_byte_credit_instead_of_one_byte` **failed on its first execution**, on its
own precondition:

```
the document payload must be large enough to measure: 612 crossings at credit 1
```

The law drove `documentJson` over the app's own starter document, which carries three widgets; 612
`Progress` events is too small a sample to separate a spent credit from an unspent one, and the
`> 1_000` bound was written against an assumed payload that was never measured. The law now seeds a
real document through the bridge's own `synchronizeDocument` first (`MEASURED_ROWS = 24` retained
rows, ~12 KB of JSON) and reports what it measured:

```
cargo test -p semio-framework-os-flow --lib one_poll_spends -- --nocapture
flow poll credit: 24 retained rows crossed in 5140 progress events at credit 1 and 1 at credit 4096
test result: ok. 1 passed
cargo test -p semio-framework-os-flow --lib the_credit_does_not -- --nocapture
test wasm_session::domain_laws::spending_the_credit_does_not_change_the_operation_result ... ok
test result: ok. 1 passed
```

**5 140 → 1.** The §4.3 fix is now measured, not argued.

### 2.4 Found on the way, not fixed: the RETIREMENT ladder still moves per byte

Sizing that law meant measuring the session close, and the close is the same defect §4.3 fixed for
`FlowProgramFeature::step`, one layer over:

| retained document | close turns | declared bound |
|---|---:|---:|
| 24 rows (~12 KB) | **2 626** | 4 096 |
| 64 rows (~32 KB) | **5 718** | 4 096 |

The bound is `🧫️fixtures/🧹️session-close/🔣️.json` `close.maximumTurns`. A 32 KB document cannot be
closed inside its own declared bound, and the turn count is linear in the payload — the retirement
ladder is spending one step per byte the way the program ladder used to. `MEASURED_ROWS` is sized to
fit the declared bound rather than to raise it; raising a close contract to hide a per-byte ladder
would be the wrong repair. **Handed on.**

---

## 3. Item 2 — `emitInteractionState` publishes only what changed

### 3.1 Root cause

`🧱️elements/🕸️NodeGraph/🟦️.tsx`'s `emitInteractionState` read the session's selection domains, hovered
widget and hovered channel, and then dispatched `interactionSelect` **and** `interactionHover`
unconditionally — whatever the values were. Two further facts make that worse than it reads:

- **One pointer-up reaches it twice.** `useCanvasPickInteraction.onCanvasPointerUp` calls
  `onSelectTarget` when the click resolved exactly one target, and the surface's own `onPointerUp`
  calls `emitInteractionState()` again straight after. A click on a node therefore published four
  times; a click on empty board, twice.
- **Each publication is a whole interaction-scope refresh.** `interactionSelect` re-renders three
  window bodies and five panel bodies, `interactionHover` three window bodies
  (`📓️interaction-scope-narrowing-2026-09-15.md` §4.1), each behind a `performInvocation` →
  `refreshUi` → React commit.

The hover half had the same shape: `onHoverFocus` published on every focus the pick hook resolved.

### 3.2 The rule, as one unit

`createNodeGraphInteractionLedger()` (exported from `🕸️NodeGraph/🟦️.tsx`, region
`🫱️InteractionPublication`) holds what this surface last told the plugin and answers whether a new
reading owes it a hop:

- `publishSelection(marks) → boolean` / `publishHover(mark) → boolean` — records the mark and says
  whether the plugin owes a dispatch.
- `adoptSelection` / `adoptHover` — marks that arrived **from** the plugin on a scene set the baseline
  without owing anything. This is what stops a plugin-side change (a keyboard `selectAll`, an outline
  pick) from being shadowed by a stale note of what the surface last sent; `applyScene` adopts
  `scene.selection` / `scene.hover` on every sync.
- `nodeGraphSelectionMarkKey` / `nodeGraphHoverMarkKey` are the comparison, exported so the law
  compares the same values the dispatch would carry.

Both `emitInteractionState` and `onHoverFocus` go through it, so the double consultation per
pointer-up collapses by construction: the second reading finds the baseline the first just set.

### 3.3 Measured, live

`🐍️flow-surface-followup-probe.mjs`, after quiescence, drives four clicks per example — a node, the
same node again, empty board, the same empty point again — and counts `semio.hop.invoke` by
`actionId` in each window. The before-column is measured in the same boot: the host logs
`[DEBUG] flow interaction publish select=… hover=…` on every consultation of the ledger, and **every
consultation dispatched both lanes under the rule this replaces**, so `2 × consultations` is the
old-rule cost of that same gesture.

| example | node click | node again | empty click | empty again | hops now | hops under the old rule |
|---|---:|---:|---:|---:|---:|---:|
| hexagonal-mushroom-column | **1** (`interactionSelect`) | 0 | 0 | 0 | **1** | 8 |
| sphere-cut-with-torus | **1** (`interactionSelect`) | 0 | 0 | 0 | **1** | 8 |

A click on empty board costs 0 because it moved nothing the plugin does not already hold — whether
the BOARD clears its selection on an empty pick is the board's own rule, and the law here is "no hop
unless the mark moved", not "a hop".

### 3.4 Law

`🧱️elements/🕸️NodeGraph/🧪️tests/🫱️interaction-publication/🟦️.ts`, registered in the react renderer's
vitest config, drives the real exported ledger:

```
SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts \
  -t "flow node-graph interaction publication|flow shared payload coalescing"
      Tests  13 passed | 1160 skipped (1173)
```

Nine of those thirteen are this item: a selection that moved is published and one that did not is
not; **one pointer-up consulted twice costs one hop and a repeat costs zero**; a plain click that
changed neither lane owes nothing; selection and hover are independent lanes; a mark the plugin made
is never published back at it; a plugin-side move makes the surface's own next reading due again; a
domain the plugin never hears about is a different mark; the keys are exactly what a publication
would carry; a retired ledger owes both lanes again.

---

## 4. Item 3 — the board attaches exactly once per document, and the "second attach" was the probe

### 4.1 Root cause: one console, two documents

The handed-on symptom was `node-graph surface ready` twice on one page, ~14 s into a boot. It is a
**measurement artifact of `🐍️flow-scroll-render-perf-probe.mjs`**, and its own evidence says so
(`🗑️generated/flow-scroll/after-fresh/console.txt`):

```
12844 node-graph host mount    surface=window:procedural-main
14221 node-graph attach called surface=window:procedural-main 1075 907 1
14321 node-graph surface ready surface=window:procedural-main nodes=7 edges=6
37879 node-graph host mount    surface=window:procedural-main
46890 node-graph attach called surface=window:procedural-main 1075 907 1
46942 node-graph surface ready surface=window:procedural-main nodes=6 edges=5
```

`nodes=7 edges=6` is `hexagonal-mushroom-column`; `nodes=6 edges=5` is `sphere-cut-with-torus`. The
scroll probe drives both examples with one `page.goto` each and never reset its `lines` buffer, so one
console file spans **two documents** — each of which mounted once, attached once and reported ready
once. There was never a second attach of one document.

### 4.2 Measured, per document

`🐍️flow-surface-followup-probe.mjs` resets its console per example, counts `surface ready` **per
surfaceId**, and watches 45 s past boot — well past the 14 s the report named:

| example | host mount | host unmount | attach called | surface ready | re-attached surfaces |
|---|---:|---:|---:|---:|---|
| hexagonal-mushroom-column | 1 | 0 | 1 | 1 | — |
| sphere-cut-with-torus | 1 | 0 | 1 | 1 | — |

Gate `attachesOnce` is "no surfaceId reported ready twice", which is the claim the item asked for.

### 4.3 What changed, and what is left standing

- `🐍️flow-scroll-render-perf-probe.mjs` clears its console per document and writes
  `console-<example>.txt`, so the artifact cannot be produced again.
- `🐍️flow-surface-followup-probe.mjs` gates on the per-surface count.
- **No product change was needed, and none was made.** The attach effect's dependencies were audited
  and are all stable across a document's life (`paintOverlays` `[]`, `syncSurfaceSize` `[]`,
  `renderFlow` `[surfaceId]`, `isGestureActive` `[]`, `sessionReady` one flip); only
  `canvasGeneration` can re-run it, and it is bumped exactly once per unpresentable canvas.
- **A real re-mount does exist, and it is named, not fixed** (§7).

---

## 5. Item 4 — the app-static payloads cross once per generation

### 5.1 Measured before

`sendFlowPayloadOnce` now logs `[DEBUG] flow payload <feature> bytes=<wire> body=<body> parts=<n>
named=<n>` per delivery, so the census is the host's own line. On `hexagonal-mushroom-column`, before
this lane's fixes (`🗑️generated/flow-surface/after/console-…` of the 16:24 run):

```
3057  setNeuronKindInfosJson   1 168 B     ← the framework starter table
3058  setCatalogueJson         1 648 B     ← the framework starter sections
3921  setNeuronKindInfosJson  98 642 B     ← the app catalogue arrives
3926  setCatalogueJson        22 309 B
3929  setNeuronKindInfosJson 100 447 B     ← 8 ms later: the SAME 98 KB again, to append 1 805 B
```

**224 314 B for one board on one document.** The §9 premise — "once per surface attach" — is not what
this app does: it has one node-graph surface per document, and the table crossed three times because
its *content* changed three times.

### 5.2 Two root causes

- **One body for two sources.** `syncFlowOperatorInfos` built `[...catalogue.operators,
  ...scene.operators]` and sent the concatenation. The two sources arrive milliseconds apart, so
  appending 1 805 B of scene-derived records re-crossed the whole 98 642 B app table.
- **The app catalogue itself arrives in stages**, 8 ms apart: 98 642 B, then 100 447 B.

### 5.3 The fix, on both sides of the ABI

**Content-addressed, multi-part payloads.** `🌊️flow/🖥️host/🦀️.rs`, region `📦️SharedPayloads`:

```
payload := part ('' part)*
part    := '@' digest '\n' body   — CARRIES the body and registers it under that digest
         | '@' digest             — NAMES a body this guest process already holds
         | body                   — a bare body (every in-process and native caller)
```

`resolve_flow_shared_payload` / `resolve_flow_shared_payload_parts` answer
`FlowCoreError::UnknownSharedPayload` for a reference this process cannot resolve, so a sender whose
note is wrong carries the bytes again instead of installing an empty catalogue.
`FlowHost::set_neuron_kind_infos_payload` composes the table part by part in order (a later id wins,
exactly as it did when the host concatenated the sources itself);
`FlowHost::set_host_catalogue_payload` stores a single part verbatim and concatenates several.
Operations 2 505 / 2 506 call the payload entry points and map the fault through `domain_error`.

The registry is a `thread_local` `HashMap<String, String>` of **bodies**, not parsed catalogues: a
parsed `OperatorInfo` map carries `Value`s that fail closed on a bare drop, and a process-wide
registry has no owner to retire it. It is process-wide because every flow session in a page lives in
one wasm module with one linear memory.

Host side (`🕸️NodeGraph/🟦️.tsx`, region `📦️SharedFlowPayloads`): `flowContentDigest` (FNV-1a in two
lanes, tagged with the body length — a content address, not a security digest), a module-scoped
`flowSharedPayloadDigests` of what the guest process has confirmed, and `sendFlowPayloadPartsOnce`,
which names the parts already held and carries only the new ones. A send that does not land clears
both the digest and this session's delivery record.

**A coalescing window.** `createFlowSharedPayloadCoalescer` (exported, injected ports, default
`FLOW_SHARED_PAYLOAD_COALESCE_MS = 24`) holds an app-static payload for one settle window so a
catalogue that arrives in stages crosses once, with its final content. Each feature settles on its own
clock; the coalescer is disposed with the session in `cancelFlowTasks`.

### 5.4 Measured after

| | before | after |
|---|---:|---:|
| `setNeuronKindInfosJson` crossings | 3 (1 168 + 98 642 + 100 447 B) | **2** (1 168 + 100 449 B) |
| `setCatalogueJson` crossings | 2 (1 648 + 22 309 B) | 2 (1 648 + 22 309 B) |
| **flow-surface payload bytes, one document** | **224 314 B** | **122 817 B** |

The 98 642 B crossing is gone: the append that caused it now names the part the guest holds. The two
remaining `setNeuronKindInfosJson` crossings are 900 ms apart (2 124 ms and 3 026 ms) and carry
genuinely different tables — the framework starter and the app catalogue — which no content address
and no settle window can or should merge.

Reference naming is visible in the log when the content does repeat: the run before the coalescer
landed shows `setNeuronKindInfosJson bytes=97514 body=98644 parts=2 named=1` — the scene part named,
the app part carried.

### 5.5 Laws

**Rust — `🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs`, five:**

```
cargo test -p semio-framework-os-flow --lib shared_payload -- --nocapture
a_shared_payload_crosses_once_and_a_second_session_names_it ...
  shared flow payload: carried 209 B, named 11 B for the same catalogue
a_shared_payload_reference_the_guest_lost_fails_closed ... ok
shared_payload_envelope_carries_names_and_fails_closed ... ok
test result: ok. 3 passed

cargo test -p semio-framework-os-flow --lib composed -- --nocapture
a_composed_operator_table_installs_every_part ... ok
a_composed_payload_carries_only_the_part_the_guest_has_never_seen ... ok
test result: ok. 2 passed
```

The first drives two real sessions through the real `FlowBridge` — session A carries the catalogue,
session B names it, and both read back the identical `catalogueJson` — at **209 B against 11 B**. The
second proves a reference the process lost answers non-OK and installs nothing. The third is the
envelope itself. The fourth and fifth are the composition: both parts install, in order, whether
carried or named, and a retired registry holds no part.

**TypeScript — four, in the same file as item 2's law** (`flow shared payload coalescing`, 13/13
above): a catalogue offered in three stages crosses once with its final content; each feature settles
on its own clock; two genuinely separate generations cross separately; a disposed coalescer crosses
nothing.

---

## 6. Item 5 — the gates

### 6.1 `🐍️flow-scroll-render-perf-probe.mjs`, both examples, `SEMIO_PROBE_GATE=1`

| example | gesture | events | paints | median ms | p95 ms | max ms | guest during | settle | refresh d/s | commits d/s | long > 50 ms |
|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---:|
| hexagonal-mushroom-column | wheel-zoom | 30 | 82 | **6.3** | 7.5 | 8.2 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 0 |
| hexagonal-mushroom-column | drag-pan | 63 | 164 | **5.5** | 6.0 | 6.3 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 0 |
| sphere-cut-with-torus | wheel-zoom | 30 | 90 | **5.5** | 8.2 | 8.3 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 0 |
| sphere-cut-with-torus | drag-pan | 63 | 142 | **5.2** | 5.5 | 5.7 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 0 |

**GREEN** — 4/4 gestures, and slightly faster than the 15:40 reading (7.2–11.2 ms medians), so nothing
here regressed the scroll lane.

### 6.2 `🐍️flow-surface-followup-probe.mjs`, both examples, `SEMIO_PROBE_GATE=1`

**GREEN** — 1 attach per document, 1 hop for the click that moved the selection, 0 for every click
that moved nothing, 0 page errors. Evidence `🗑️generated/flow-surface/final/`.

### 6.3 Battery — `SEMIO_BATTERY_ROOT=react-flow2` on :6022

| row | verdict | steps | seconds | page errors |
|---|---|---|---:|---:|
| `flow-window` | **green** | 3/3 | 72 | 0 |
| `flow-wire` | **green** | 5/5 | 124 | 0 |
| `flow-reorganize` | **green** | 3/3 | 53 | 0 |
| `graph-keyboard` | **green** | 13/17 declared steps | 43 | 0 |
| `outline-selection` | **green** | 11/11 | 36 | 0 |
| `interact` | **red** | 54/58 | 125 | 0 |

**Total page errors across the six rows: 0.** Scoreboard
`🗑️generated/react-flow2/scoreboard.json` (and `react-flow2b/` for the `interact` re-run).

Two rows needed a fix to their own reading, not to the product:

- **`flow-reorganize` was reading nothing.** It resolved the widget layout through
  `hostSnapshotJson()` or a `data-fixture-json` DOM attribute; the first is a scene field this app's
  plugin does not send, and the second was renamed to `data-host-snapshot-json` by the peers' sweep.
  Both answered `null`, so the probe scored `moved: []` for a reorganize that moved everything — and
  it had scored exactly that at 15:00 too (`🗑️generated/react-scroll/flow-reorganize/`
  `{"moved":[],"before":null,"after":null}`), i.e. the row's own earlier "3/3" was never this
  assertion passing. The surface probe registry now publishes `nodeIds()` and `nodeLayout()` — the
  positions the surface is actually painting — and the row reads
  `moved: ["height","radius","sides","profile","extrusion-axis","extrude","column-preview"]`.
- **Its 120 s idle window was long enough for a dev hot-swap to retire the app instance.** Twice the
  run recorded `hot-swap procedural` → `actor-activation.revoked` → `node-graph host unmount`, after
  which the Flow window has **no node-graph host at all** and the reorganize lands on nothing. The row
  is 45 s now, which is all this mode needs (boot plus one keypress). `flow-wire` hit the same
  hot-swap once (2/5) and is 5/5 on a run that did not.

---

## 7. Not claimed

- **The retirement ladder was measured, not fixed** (§2.4): a 32 KB retained document needs 5 718
  close turns against a declared bound of 4 096, and the count is linear in the payload. This is the
  same per-byte-ladder family as `📓️flow-scroll-render-perf-2026-09-15.md` §4.3, one layer over, and
  it is a live contract violation for any document over ~20 KB.
- **`interact` is 54/58 and every red is attributed, none to this lane.** Three —
  `Rectangle Wire Preview · select`, `Rectangle Wire Preview · inspector`,
  `Hexagonal Mushroom Column · inspector` — were already red at 13:05
  (`🗑️generated/react-sweep-closing/scoreboard.json`, 55/58, the same three). The fourth,
  `Hexagonal Mushroom Column · select`, is new since 13:05 and reproduces across two runs: the probe
  reads `selectedIdsBefore: ["extrude@solid"]` and its plain click therefore toggles the object OFF.
  The console names the source — an `interactionSelect` at **t = 2 538 ms, fourteen milliseconds
  BEFORE `node-graph host mount`**, straight after `setContributions`/`setActiveExample`, with no
  pointer event of any kind and **no `[DEBUG] flow interaction publish` line anywhere in that
  console**, i.e. the flow board's publication path was never consulted in that probe. A boot-time
  auto-selection upstream of this lane's surface. A fifth red, `Sphere Box Fuse · hover/select/
  inspector`, appeared in one run and not the other (35 swept points, no mesh) — the known world3d
  delivery flake.
- **A real re-mount of the node-graph host exists, and it is not fixed here.** After a dev hot-swap
  the app instance is revoked and the Flow window's node-graph host unmounts and **never comes back**
  — the shell then reports `program procedural: no channel for instance 1`. That is the hot-swap
  `no channel` fault the live `host-refresh-latency` lane owns; this lane only stopped its own probes
  idling into it.
- **`scene.hostSnapshotJson` is absent on this stage.** No `synchronizeSnapshotJson` payload crosses
  on most boots, so the flow session's document is never seeded from the scene — the board is driven
  entirely by `scene.nodes`/`scene.edges`. It appeared in one of the four measured boots (1 867 B), so
  it is conditional, not gone. Not investigated; it is why two ticket probes were reading `null`.
- **The starter catalogue still crosses separately** (1 168 B + 1 648 B, 900 ms before the app one).
  Different content, legitimately sent early; neither the content address nor the 24 ms window applies.
- **The content-addressed reference has no live second consumer on this app.** Its saving is proven by
  the Rust law (209 B carried against 11 B named, two real sessions on the real bridge) and by the
  `named=1` crossing in §5.4; this app opens one node-graph surface per document, so the per-attach
  duplication the item named does not occur here at all.
- **`WasmGraphSurface`'s own `emitInteractionState`** (the other node-graph host in the same file,
  used by the OS workflow window) still publishes both lanes unconditionally. The ledger is exported
  and would serve it verbatim, but this lane measured only the Flow board and does not claim a fix it
  did not run.
- **11 vitest FILES in the react renderer suite fail to import**, with
  `ReferenceError: Cannot access '__vite_ssr_import_18__' before initialization` through
  `🛠️ShellHelpers/🟦️.tsx:2102 → 🐚️Shell/🟦️.tsx:78 → 🛠️ShellHelpers/🟦️.tsx:180` — a `ShellHelpers` ↔
  `Shell` circular import, new since 15:40 (the scroll lane's run of the same config reported
  `1 passed | 49 skipped (50)`), and untouched by this lane. Both of this lane's suites and the scroll
  lane's own suite pass through it.
- **8 pre-existing reds in `semio-framework-os-flow --lib domain_laws`** (5 are
  `FlowDomainAdapter::default()` dropped with an unretired `OrderedMap`, 3 are other laws' own fixture
  assertions). They fail identically with and without this lane's change and none of them touches the
  code here; the 6 green include all four of this lane's.
- **Windows and Linux are reasoned, not measured.** The shared-payload envelope and the coalescer are
  platform-independent by construction; only macOS was exercised.

---

## 8. Files

Product:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` —
  `🫱️InteractionPublication` (the ledger), `📦️SharedFlowPayloads` (digest, registry, coalescer),
  `sendFlowPayloadPartsOnce`/`crossFlowPayload`, the two-part operator table, `nodeIds`/`nodeLayout` on
  the surface probe registry, the `[DEBUG] flow payload` and `[DEBUG] flow interaction publish` seams
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `📦️SharedPayloads`,
  `FlowCoreError::UnknownSharedPayload`, `set_host_catalogue_payload`,
  `set_neuron_kind_infos_payload`, `neuron_kind_ids`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs` — operations 2 505 / 2 506 take the
  content-addressed payload
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm` +
  `📦️packages/🦀️rust/🕸️bindings/` (rebuilt twice, `nx run semio-framework-os-flow-core:wasm`,
  `NX_SKIP_NX_CACHE=true`, 1 m 57 s and 1 m 44 s)

Laws:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🫱️interaction-publication/🟦️.ts` (new, 13)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (registration)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs`
  (5 new shared-payload laws; the credit law's fixture sized and its bridge helpers lifted)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs` (peer rename repair)

Ticket:

- `🐍️flow-surface-followup-probe.mjs` (new), `🔧️repair-flow-vcs-host-snapshot.py` (new)
- `🐍️flow-scroll-render-perf-probe.mjs` (console per document), `🐍️flow-window-probe.mjs` (layout read),
  `🐍️react-battery.mjs` (`flow-reorganize` window)
- evidence: `🗑️generated/flow-surface/{after,final}/`, `🗑️generated/flow-scroll/followup/`,
  `🗑️generated/react-flow2/`, `🗑️generated/react-flow2b/`
