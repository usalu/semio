# Wave B14 — settle semantics: a turn ends on progress, never on one acknowledgement-free round trip

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B14, 2026-09-11. Removes the `empty-required stop` the
2026-09-10 coordinator added ("22:53 … kills the 4096-spin fault") and replaces it with the quiescence
rule the fault it guarded actually needs, now that W-B2 (retirement pricing + zero-progress fast-fail)
and W-B8 (slot retirement) removed the spin at its source. No git write, ticket not closed,
`🗑️generated` untouched, no wasm build. All runs foreground on wasm #46 / `:6013` (vite-live host).

---

## 1 The settle loop, as it is (file:line)

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

| symbol | line (post-wave) | meaning |
| --- | --- | --- |
| `hasRequiredUiPatches` | 1335 | `undefined` required = "any patch"; **empty set = satisfied by definition**; a named set = every surface must have published at least one patch |
| `PLUGIN_UI_CONTINUATION_LIMIT` | 1342 | 4 096 — the round-trip backstop |
| `PLUGIN_UI_CONTINUATION_BATCH_SIZE` | 1343 | 8 — macrotask yield cadence |
| `PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT` | 1358 | W-B2's contract-derived 128 (`maxPatchBytes/maxTextBytes × batch`) |
| `PLUGIN_UI_QUIESCENT_CONTINUATIONS` | 1371 (new) | the same bound read as "quiesced", not "stalled" |
| `pluginTurnStalledError` | 1408 | W-B2's attributable fault; now also names the owning call (`operation=puzzle#1:command#1`) |
| `settlePluginTurn` | 1428 | the loop |
| `settleAcknowledgedPluginTurns` | 1477 | what `runQueuedTurn` uses after command ingress completes — `required=new Set()`, `drainOperations=true` |
| `drainTypedOperations` | 2274 | the poll that advances retained operations when no host call is left; one settle per macrotask |
| `runQueuedTurn` | 2190 | command ingress pages → `command-complete` → `settleAcknowledgedPluginTurns` → `routeHostEffects` → frames |

Per continuation the loop holds four things:

- **`required`** — what the CALLER needs published (`refreshUi` passes the surfaces whose retained view
  is missing; every mutation path passes `new Set()`, i.e. *nothing is required of this turn*).
- **`published`** — surfaces seen across `results` (`wirePatchSurfaceId`).
- **`acknowledgements`** — what `acceptPatches` (production: `acceptUiPatches`) plus
  `typedOperationAcknowledgements` produced for the turn just collected; they are the events submitted
  with the NEXT continuation.
- **`status`/`hasWork()`** — `hasWork()` is `(drainOperations || required still missing) && status === "more-work"`.

Guest side, `more-work` is the fold of ten named sources (`⚛️reactor/🔄️turn/🦀️.rs:81-104`,
`:1171`): `executor_deadline`, `process_pool`, `close_cleanup`, **`typed_operation` ("a typed operation
continuation has not reached its terminal state")**, `reconcile`, `resumes`, `executor_pending`,
`command_ingress`, `lifecycle`. Only the FOLD crosses the wire (`TurnStatus::MoreWork`, `:1229`) — the
host cannot see which source is armed, so "is this actor busy or parked?" is only answerable by
whether it hands anything back.

A mutation's publication reaches the host as `MountedTypedCommandFullOperation` pages carried on
`send-message` effects: `consumeTypedOperationEffects` (653) turns lane 10 into
`TYPED_OPERATION_TERMINAL_SEEN`, lane 11 into a thrown fault, and leaves shell frames alone;
`shellFrameBytes` turns those into `AppFrame`s (`Invocation`, `Ephemeral`, `OperationCompleted`), and
`subscribeOperationCompletions` (2966) flushes `pendingCompletionEffects` when `OperationCompleted`
arrives. The publication unit itself refuses to run against a moved document root
(`🔌️plugin/🦀️.rs:23485-23495`, `typed-operation pending publication rejected a stale immutable
document root`; freshness is `canonical_revision == live_revision`, `:19019`).

### 1.1 Where the stop sat

```ts
if (requiredEmpty && acknowledgements.length === 0) { emptyRequiredStopped = true; console.warn(…); break; }   // HEAD, before W-B2's streak
zeroProgress = …; if (zeroProgress >= PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT) throw pluginTurnStalledError(…);
```

The stop ran **before** W-B2's zero-progress accounting and short-circuited it: with `required=[]` the
loop could never reach a second continuation unless the guest happened to produce acknowledgements on
every single one. Every mutation path passes `required=new Set()`.

---

## 2 The reproduced drop (browser, wasm #46, fresh page)

`bun 🔍️browser-probe.ts --only=catalogue-panel --port=6013`, with a temporary per-continuation
`[DEBUG] b14 settle …` tap in the loop (added, run, **removed**) →
`🗑️generated/probe-2026-09-11T15-38-44.md`:

```
warning: [DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"addObjectKind"}
warning: [DEBUG] command ingress lane {"instanceId":1,"actionId":"addObjectKind","seq":24,"lane":"Interactive"}
warning: [DEBUG] command ingress settled status=command-complete observed=command-pending,command-complete
warning: [DEBUG] b14 settle puzzle#1 call=puzzle#1:command#1 c=1 status=more-work acks=0 patches=0 effects=none required=[] drain=true
warning: [DEBUG] settle puzzle#1 empty-required stop continuation=1 status=more-work drain=true
warning: [DEBUG] performInvocation settled {…,"actionId":"addObjectKind","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":0}
```

**One** continuation. The guest said `more-work` while its typed operation was mid-flight, published
nothing on that round trip, and the host ended the turn — exactly B11 §6's reading, now with the
continuation index. What followed (same console, second run
`probe-2026-09-11T15-42-34.md`) is the shape of the whole session:

```
… 149 × [b14 settle puzzle#1 call=puzzle#1:operation-drain#1 c=1 status=more-work acks=0 patches=0 effects=none required=[] drain=true
        + settle puzzle#1 empty-required stop continuation=1 status=more-work drain=true] …
debug:   [DEBUG] typed-operation slots instance=1 live=14/64 peak=14   → live=18/64 peak=18
warning: [DEBUG] b14 settle puzzle#1 call=puzzle#1:operation-drain#1 c=1 status=more-work acks=1 patches=0 effects=send-message …
```

i.e. the drain poll became the ONLY driver, at one turn per macrotask, each poll cut after one
continuation; mounted operations accumulated (`live=14 → 18/64`, never retiring) and the document
never moved: `verdict catalogue-add-object-kind FAIL before=1 after=1`,
`verdict catalogue-drag-drop FAIL … before=1 after=1`. The coordinator's clean HEAD battery of the
same hour (`🗑️generated/battery-2026-09-11-46-6013.txt`, FAULTS=0) agrees:
`volume-brush-add-target-volume FAIL before=0 after=0`, `duplicate-selection FAIL before=1 after=1`,
`delete-selection FAIL before=1 after=1`.

---

## 3 The change

> A turn ends when the actor stops HANDING BACK, not when one round trip happens to carry no
> acknowledgement. `required=[]` states what the caller needs; it says nothing about what the actor
> still owes this turn.

`settlePluginTurn` (1428-1470):

- the `empty-required stop` and its ungated `console.warn` are **gone**, and so is the ungated
  `% 512` continuation warn (the only two per-turn console writers in the loop; diagnosis now lives
  in the two faults, which name actor, call, pending surfaces and the bound they hit);
- progress is measured over **everything the continuation added** — `acknowledgements.length > 0 ||
  results.slice(collected).some(turn => turn.uiPatches.length || turn.effects.length)` — so a patch
  that arrives on a supplemental acceptance turn counts too (it did not before);
- one publication-free streak, two outcomes:
  - `zeroProgress >= PLUGIN_UI_QUIESCENT_CONTINUATIONS && !outstanding()` → **quiesced**: return what
    was collected (no fault: nothing was requested of this turn);
  - otherwise `zeroProgress >= PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT` → W-B2's
    `pluginTurnStalledError`, now carrying `operation=${call?.label}` so a drain with no surfaces to
    name still names the call (`puzzle#1:command#1`, `puzzle#1:operation-drain#1`, …);
- `PLUGIN_UI_QUIESCENT_CONTINUATIONS = PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT` (1371) — deliberately
  ONE number, not a second magic constant: W-B2 derived 128 as "the most publication-free turns a
  HEALTHY guest may need"; past it the actor is either stalled (surfaces were requested) or has nothing
  more for this turn (they were not).

Termination is unchanged and total: quiescent stop ≤ 128 round trips, stall fault at 128, backstop
fault at 4 096. The spin the old stop guarded (W-G3's `interactionSelect` 4 096-continuation fault) is
W-B2's zero-progress fault now, and the clean #46 battery already showed FAULTS=0.

---

## 4 Laws — `🧪️tests/🔌️plugin-runtime/🟦️.tsx`

Written first, run at HEAD (production sources unmodified), all three **FAILED**:

| law | at HEAD |
| --- | --- |
| (a) `collects a mutation frame emitted several acknowledgement-free continuations after the command completed` | `AssertionError: expected 1 to be undefined` — one continuation, frame never collected |
| (b-drain) `treats an actor answering more-work with nothing to publish as quiesced only after a whole silent batch` (rewrite of `does not spin the continuation budget when drain-operations has an empty required set…`) | `expected 1 to be undefined` |
| (c) `prints no console warning while a healthy actor settles its continuations` | `AssertionError: expected 1 to be 3` — the settle stopped after one continuation and warned |

W-B2's own law (b) `fails fast and names the pending surface when a guest answers more-work without
publishing anything` is untouched and green.

After the change:

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts -t "settle"
 Test Files  4 passed | 20 skipped (24)
      Tests  12 passed | 871 skipped (883)
$ … -t "continuation"          →  Tests  6 passed | 877 skipped (883)
$ … -t "more-work"             →  Tests  3 passed | 880 skipped (883)
```

---

## 5 Probe verdicts, before → after (wasm #46, `:6013`, fresh page)

`bun 🔍️browser-probe.ts --only=catalogue-panel,volume-brush,selection-keybindings --reload-between-groups --port=6013`
→ `🗑️generated/b14-after-steps.txt`, `probe-2026-09-11T15-50-57.md`, `faults=0 hard=0 collateral=0`:

| verdict | HEAD | after |
| --- | --- | --- |
| `catalogue-drag-drop` | `FAIL … before=1 after=1` | **`PASS`** |
| `duplicate-selection` | `FAIL before=1 after=1` | `FAIL before=2 after=2` |
| `delete-selection` | `FAIL before=1 after=1` | `FAIL before=4 after=4` |
| `catalogue-add-object-kind` | `FAIL before=1 after=1` | `FAIL before=1 after=1` |
| `volume-brush-add-target-volume` | `FAIL before=0 after=0` | `FAIL before=0 after=0` |
| `volume-brush-arm` / `-voxel-dims` / `-target-volume-attribute` | PASS | PASS |

The `before=` counts are the finding: the document now ACCUMULATES the edits (1 → 2 → 4 objects over
one run) where at HEAD it never left 1. `history patch applied {"replace":false,"currentCursor":3,
"patchCursor":4,"upserts":1,"labels":["create-object object { id=puzzle3d.object.c05b8dbd798d5454
label=\"Hexagonal Cut Concrete Forest Left\" …"]}` — the created object, its history entry, its undo
cursor: none of that existed at HEAD. What remains is a **one-command lag**, §6.

`🔍️b4-main-thread-probe.ts` once, after (`bun 🔍️b4-main-thread-probe.ts "http://127.0.0.1:6013/?plugin=puzzle3d"`):

```
[14.7s] landed after 5.6s: [{"bytes":54254,"instances":180},{"bytes":54254,"instances":180}]
[14.8s] gap=5.6s longtaskCpu=0.2s tasks=2 workerPosts=1947
  page.workerMessage                15.8ms  ×3932
  page.workerPost                    8.9ms  ×1966
  page.console.warn                  1.3ms  ×17
```

against B4 §4/§5 at HEAD (`gap=86.3s longtaskCpu=14.2s tasks=55 workerPosts=4970`, **the switch did
not land**; `page.console.warn 108.1ms ×2505`, `page.workerMessage 72.5ms ×10006`,
`page.workerPost 45.2ms ×5002`). The switch **lands**, in 5.6 s with 0.2 s of long-task CPU and 2 long
tasks; the warn storm B4 priced at 108–244 ms is **17 calls / 1.3 ms**. (Caveat: peers land work in
this tree continuously; the warn count is attributable to this wave, the round-trip count only partly.)

---

## 6 Residual — handed over, with evidence

**A parked typed operation only advances on a turn carrying COMMAND INGRESS.** With the stop gone,
every settle now reaches `status=idle` (measured: 18 command settles 0–71 continuations, 10 refresh-ui
settles 10–23, 3 job-completion settles 2–4, exactly 2 quiesced at 128/132 — and **one** drain poll in
a whole session, against hundreds at HEAD). But the add's own turn ends idle with its operation still
parked, and the completion is carried by the NEXT command's turn:

```
warning: [DEBUG] b14 command routed instance=1 action=addObjectKind status=idle
         effects=send-message|send-message|send-message|send-message|typed-operation-terminal-seen
         frames=Invocation|Ephemeral|Invocation|OperationCompleted leftover=typed-operation-terminal-seen
warning: [DEBUG] b14 settled puzzle#1 call=puzzle#1:operation-drain#1 continuations=0 quiesced=false status=idle
…
warning: [DEBUG] b14 command routed instance=1 action=registerBrushMesh …
warning: [DEBUG] history patch applied {…,"upserts":1,"labels":["create-object object { id=puzzle3d.object.c05b8dbd798d5454 …
```

Tested and ruled out from the host side: arming `drainTypedOperations` unconditionally after every
command (not only on `more-work`) changed nothing — its poll submits an EVENTLESS turn and the actor
answers idle at continuation 0. The `refresh-ui` settles between the add and the next command carry
real events (`SurfaceVisible`) and 11–23 continuations, and they do not advance it either. Only a
command-ingress turn does. That is guest-side scheduling (`typed_operation_scan` versus the ingress
focus in `poll_kernel_turn`) and needs a wasm rebuild to confirm — forbidden to this wave. It is what
still keeps `catalogue-add-object-kind`, `volume-brush-add-target-volume`, `duplicate-selection` and
`delete-selection` red: the edit lands, one command too late for the probe's own `after=` read.

Also observed, for whoever takes that thread:

1. **W-B2's 128 is closer than it looks.** At HEAD a mounted operation went 149 consecutive
   publication-free drain polls before its first page. With the stop gone the streaks collapse (max
   132 observed, both quiescent), but a settle that DOES name required surfaces and sits behind such an
   operation would fault at 128. If that ever fires spuriously, the bound — not the stop — is what to
   revisit.
2. **Operation slots leaked at HEAD** (`typed-operation slots instance=1 live=14/64 peak=14 → 18/64`,
   monotonic). After the change the same trace shows `live=0/64` once operations complete.
3. **`catalogue-drag-drop`'s own dispatch** — in the HEAD console the drop step produced no
   `performInvocation` at all (`ran=true`, no action); its PASS above is the click's add landing during
   the drop step's window. B11's §2c thread, not this one.

---

## 7 Verification (foreground)

| command | tail |
| --- | --- |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` | `Test Files  3 failed \| 21 passed (24)` / `Tests  9 failed \| 874 passed (883)` — **the identical 9 failures B4 §verification recorded**: 6 `🧩️package-integration` (generated-worker byte/provisioning laws), 1 `🔬️engine-contract` (`buildNoteShellCommandAction`), 2 `🔌️PluginRuntime` (`binds two instances of one body…`, `readAppDocumentPack()` extra `"ops": ""`). None in this wave's region; **no new failure**. |
| `bun x tsc --noEmit -p tsconfig.json` (react target) | 1 083 errors repo-wide (peer baseline, dominated by generated `.d.ts`); in `🔌️PluginRuntime/🟦️.tsx` exactly two, both outside this wave's hunks and pre-existing peer code: `(2473,52) … 'req' does not exist in type 'FaultScope'` and `(2644,111) … Property 'Invocation' does not exist on type 'AppFrameValue'`. Zero in `🧪️tests/🔌️plugin-runtime/`. |
| probes | `--only=catalogue-panel,volume-brush,selection-keybindings --reload-between-groups --port=6013` → `done booted=true faults=0 hard=0 collateral=0 verdicts=32`; `🔍️b4-main-thread-probe.ts` → `landed after 5.6s`. |

Every `[DEBUG] b14 …` tap was removed (`rg -a "b14"` over both files: no match). No wasm build was run.

## 8 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — quiescence rule, `PLUGIN_UI_QUIESCENT_CONTINUATIONS`, call-named stall fault, both ungated settle warns removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` — three laws (quiescence, late mutation frame, silence).
- `🗑️generated/` — `b14-before-catalogue.txt`, `b14-before-catalogue2.txt`, `b14-after-catalogue1..3.txt`, `b14-after-steps.txt` and their `probe-2026-09-11T15-{38,42,45,47,48,50}-*.md/.ndjson`.
