# Preview Re-Arm After An Inspector Edit — lane `preview-rearm-after-inspector-edit` (2026-09-14)

Closes the one red `📓️react-generate-chord-restage-2026-09-14.md` §9 handed on: the React battery's
`gaps` step `inspection-preview-rearm`. A slider edit in the Inspection panel landed as
`patchFlowWidgets`, the document's `height` really moved 6 → 7 — and the edit preview's published
payload stayed byte- and digest-identical for the rest of the session.

- Focused probe: `🐍️preview-rearm-probe.mjs` (new)
- Shared fixtures: `🪆️subsets/✳️any/🧫️fixtures/⏯️preview-eval-run.json`,
  `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json`
- Rust laws: `🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`, `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`
- TS twins: `🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts`,
  `🔌️plugin/🧪️tests/🔢️action-arg-carriers/🟦️.ts`, `🔌️plugin/🧪️tests/📖️read-only-run/🟦️.ts`
- Runtime logs: `🗑️generated/preview-rearm/`

---

## 1. TL;DR

**Six defects in one chain, across three layers, and the deepest one meant NO tool run of ANY app
could ever run twice in a session.** Four of them are below; the start storm and the superseded-job
cancellation are §2 rows 5 and 6.

1. **`toolRunFinalize` was always rejected as stale.** `generation` crosses the wire as
   `Number(Float(1.0))` — the shell's own action arguments build it with `UiValue::Number(f64)`
   (`⏯️tool-run/🦀️.rs:1454`) and a guest that mints it with `DslValue::uint` arrives the same way — but
   `tool_run_arg_u64` accepted only an exact `u64` carrier or a numeric string. It answered `None`, and
   `(_, None)` is `ToolRunRejection::Stale`. Measured live:
   `toolRunFinalize args=…("generation", Number(Float(1.0)))… outcome=Rejected(Stale) slot=ToolRunSlot { run: 1, generation: 1, state: Complete }`.
   Every run therefore reached `Complete` and stayed there, and a `toolRunStart` against a non-terminal
   run is `ToolRunRejection::Busy` — so the generation3d preview evaluated exactly ONCE per session.
2. **A read-only run has nothing to finalize and should not wait to be told.** `previewEval` declares
   `mutating: false`, authors no provisional edit, and there is nothing for a finalize to publish or for
   a user to review — yet it parked in `Complete` holding the tool's single run slot, waiting for an
   action its owner had to dispatch on the host's own refresh cadence. **Fixed** — a completed
   non-mutating run finalizes itself in the same driver turn.
3. **`ArtifactApp::pending_effects` could not see the run at all.** The four render and measure passes
   bind the run view with `.with_tool_run(tool_runs.view())`; the poll did not, so `doc.tool_run()`
   answered `None` on every poll — and the whole start/finalize ladder in `preview_eval_run_effects` is
   written over exactly that value. Measured: 27 consecutive polls in one session, every one of them
   `run=None`, while the run was demonstrably running and ticking
   (`🗑️generated/preview-rearm/run5/console.txt`).
4. **No gesture on the generation3d editor's session route owed the previews anything, and no route
   asked for the work it owed.** Only a hand-kept roster of tool ids (`setActiveExample`, the six
   generation commands, `setContributions`, a closing `importDocument`) ever re-armed the preview
   latches, so `patchFlowWidgets` — and `addWidget`, `removeWidget`, `nodeGraphEdit`, `deleteSelection`,
   `reorganize` and every transform with it — moved the document and left the preview showing the old
   geometry. **Fixed** — the rule is read off the gesture's own emit. And recording the debt was only
   half of paying it: the host polls `pending_effects` once per `refreshUi` and refreshes on ACTIVITY,
   so a gesture whose guest work leaves nothing further to say produces no poll at all. Measured:
   `owedAfter=["procedural-preview", "generation3d-generate-preview"]` and then 60 s of silence.
   **Fixed** — every route that owes the previews now carries, on its own emit, the run start that debt
   needs when no live run can be woken.

**The storm is the same latch, read once per REFRESH instead of once per ANSWER.** A run action is a
REQUEST; the run view keeps answering what it answered before until the host turns one into a run
state. `preview_eval_run_effects` re-read `owed` on every poll and asked again — three `toolRunStart`
invocations in 500 ms on a quiet boot, 1 053 in the busy session §9 measured. **Fixed** — the link
holds its request until the view ANSWERS it.

`inspection-preview-rearm` is green, under a predicate this lane made STRONGER than the one it
inherited (the payload must change AND still carry geometry, not merely change).

---

## 2. Root cause, file:line

| # | layer | site | what was wrong |
|---|-------|------|----------------|
| 1 | framework | `🧰️framework/…/🔌️plugin/⏯️tool-run/🦀️.rs` `tool_run_arg_u64` | read only `as_u64()`/numeric string; `generation` arrives as a JSON float, so every `toolRunFinalize` was `Stale` |
| 2 | framework | `🧰️framework/…/🔌️plugin/⏯️tool-run/🦀️.rs` `complete_tool_run_job` | a completed `mutating: false` run parked in `Complete`, holding the tool's only run slot |
| 3 | framework | `🧰️framework/…/🔌️plugin/🦀️.rs` `VcsArtifactApp::pending_effects` | built its `ArtifactView` without `.with_tool_run(tool_runs.view())`, so the poll's whole run ladder read "no run" |
| 4a | app | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` `Generation3dSessionCommandWork::step` | never owed the attached previews anything, whatever the gesture did to the document |
| 4b | app | same file, every owing route | recorded the debt and asked nothing for it; the host had no reason to poll again |
| 5 | app | `✏️s/…/🧵️preview-eval/🦀️.rs` `preview_eval_run_effects` | asked once per refresh rather than once per answer — the start storm |
| 6 | app | `✏️s/…/🧵️preview-eval/🦀️.rs` `PreviewEvalRunJob::close_step` | a SUPERSEDED job read the new run's markers as its own and quiesced the fresh evaluation (`phase: "cancelled"`, empty payload) |

### The order the evidence came in

1. `🐍️preview-rearm-probe.mjs` on a clean boot: `valueMoved: true`, `digestBefore == digestAfter`,
   `startsForThisEdit: 0`, `ticksForThisEdit: 0`. The gesture owed nothing (defect 4a).
2. With 4a fixed: `owedAfter=[…both previews…]` and then no poll at all for 60 s (defect 4b).
3. With 4b fixed: one `toolRunStart` went out and settled — and no run appeared. Every poll answered
   `run=None` (defect 3).
4. With 3 fixed: the run view appeared, the ladder ran — and every `toolRunFinalize` came back
   `Rejected(Stale)` while the slot's generation matched the argument exactly (defect 1).
5. With 1 fixed: the preview re-evaluated, and the generate preview began coming back `cancelled`
   (defect 6), and a Complete run still blocked the gesture's start until a read-only run stopped
   parking in it (defect 2).

---

## 3. The fixes

### 3.1 Framework — `🔌️plugin/⏯️tool-run/🦀️.rs`

- `tool_run_arg_u64` accepts a JSON number that is an exact non-negative integer, alongside the exact
  `u64` carrier and the decimal string. A number that is an identity IS that identity, whichever
  carrier minted it; refusing one shape is not strictness, it is a silent misread.
- `complete_tool_run_job` finalizes a completed `mutating: false` run in the same driver turn.
  `Complete` means "the job is done and the run awaits the finalize that publishes its provisional
  edits"; a read-only run authors none.

### 3.2 Framework — `🔌️plugin/🦀️.rs`

- `pending_effects` binds the run view exactly as the four render/measure passes above it do. The
  document itself stays the COMMITTED snapshot: a poll decides about work over what has landed, never
  over a run's own provisional overlay.

### 3.3 Guest — `🧵️preview-eval/🦀️.rs` (the peer lane's file, forward onto their contract)

- `owe_attached_previews` takes `&mut PreviewEvalRunLink` and releases the request latch.
- New `owe_attached_previews_carrying` — owes every attached window AND puts the run start on the
  gesture's own emit when no UNSETTLED job is there to be woken. New
  `owe_attached_previews_for_mutations` keys that on the gesture's emit rather than on a tool id.
- `PreviewEvalRunLink::requested: Option<(PreviewEvalRunRequest, Option<(u64, u32)>)>` — what was asked
  and about which run identity. Released by the ANSWER (`Start` by a run that is no longer startable,
  `Finalize` by a run that is complete no longer) or by the identity moving past it, so a request the
  ledger refused as stale cannot deadlock the latch.
- The `Complete` arm finalizes as soon as it sees a complete run, owed or not.
- `PreviewEvalRunLink::job_run` + `close_step`'s ownership guard: only the job the link belongs to may
  quiesce the session. A start retires the previous entry, so the old job's close runs AFTER the new
  job installed its port and reset `settled`.

### 3.4 Guest — `✏️editor/🦀️.rs`

Every route that owes the previews now also carries what asks for the work: the session command route
(keyed on the emit's artifact mutations), the preview command route (`setActiveExample` + the six
generation commands), the document-IO route (a closing `importDocument` chunk) and the contributions
route.

### 3.5 Fix-forward for the peer lane `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`

One, landed while this lane was running (their 09:08 edit, verified stable for six minutes before it was
touched). Their `drive_preview_run` / `owed_run_actions` harness replaced the viewer test context's
manual tick drain, and `armed_ticks`, `arm`, `armed_window_ids` and `drain_armed_flow_eval_ticks_from`
went with it — while three files still called the last two
(`👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs` × 6, `👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs` × 5).
The crate's `--lib` test binary did not compile, so no law in it could be run at all.

**Fixed forward onto their contract, reverting nothing**: `drain_armed_flow_eval_ticks_from` is now a
shim over their `drive_preview_run(app, view, initial).hops`, exactly as their own
`drain_armed_flow_eval_ticks` is; `armed_window_ids` is restored as a pure reading of an effect list,
with no dependency on the harness they replaced. The app crate itself always built — only the test
binary was broken — so the restage and the runtime proof below were never affected.

---

## 4. Laws

Language-agnostic, Rust + a TypeScript twin over the same fixture, in both files.

| fixture table | rows | Rust | TS twin |
|---|---|---|---|
| `⏯️preview-eval-run.json` → `gestureRearm` | 9 | `every_gesture_rearm_row_owes_one_tick_per_preview_and_at_most_one_start` | `🧪️tests/🩹️gesture-rearm/🟦️.ts` |
| `⏯️preview-eval-run.json` → `jobSupersession` | 4 | `every_job_supersession_row_lets_only_the_owning_job_quiesce_the_session` | same file |
| `⏯️preview-eval-run.json` → `runEffects` | 10 (generalized from one action to a sequence, two rows renamed) | `every_run_effects_row_starts_finalizes_or_leaves_the_run` | — |
| `⏯️tool-run/🔣️.json` → `actionArgCarriers` | 7 | `every_action_arg_carrier_of_an_exact_integer_reads_the_same_identity` + `a_finalize_in_the_wire_carrier_finalizes_the_run_it_names` | `🧪️tests/🔢️action-arg-carriers/🟦️.ts` |
| `⏯️tool-run/🔣️.json` → `readOnlyRun` | 1 | `a_read_only_run_finalizes_itself_and_frees_its_slot_for_the_next_start` | `🧪️tests/📖️read-only-run/🟦️.ts` |

The `gestureRearm` rows state the whole law: a landed document mutation owes every attached preview
window exactly one re-armed tick and carries the start that debt needs; a gesture that authored no
artifact mutation owes a settled run nothing; an unservable graph carries nothing; an UNSETTLED live
job is woken rather than restarted while a SETTLED one is not; and one gesture costs at most ONE
`toolRunStart` however many polls the host takes to notice (`startLagPolls` up to 9).

Each law was red before its fix. The first `gestureRearm` run reported `starts=6` for six polls — one
per poll, the storm in miniature — against an expectation of 1; the carrier law's float row read `None`;
and the runtime rejection `outcome=Rejected(Stale)` is the finalize law's failing-first evidence.

**Counts**

- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- preview_eval::tests`
  → **12 passed, 0 failed**
- `cargo test -p semio-framework-plugin --lib -- tool_run_tests` → **18 passed, 0 failed**
- `bun 🔍️gesture-rearm.ts` → 9 `gestureRearm` rows + 4 `jobSupersession` rows, all green, agreeing with
  Rust row for row
- `bun 🔍️tool-run-arg-carriers.ts` → 7 rows green
- `bun 🔍️read-only-run.ts` → green, through the framework's OWN TypeScript state machine
  (`⏯️tool-run/🟦️.ts`'s `ToolRunMachine`), which independently reproduces `toolRun.busy` for a run
  parked in `complete`

---

## 5. Runtime proof on 6018

All after `activate-generation3d-react-dev` (`🗑️generated/preview-rearm/restage-*.txt`).

### 5.1 The focused probe

`SEMIO_PROBE_OUT=preview-rearm/run14 bun 🐍️preview-rearm-probe.mjs`

```
[DEBUG] boot            digest=169:2868d98b   meshes=1
[DEBUG] inspector-open  digest=3644:305583bb  meshes=3   selected=height
[DEBUG] inspector-patched digest=596:1836f633 meshes=2
  {"valueMoved":true,"digestBefore":"3644:305583bb","digestAfter":"596:1836f633",
   "previewMoved":true,"previewArmed":true,"startsForThisEdit":3,"ticksForThisEdit":19}
[DEBUG] PREVIEW REARM PROBE DONE {"rearmed":true,…}
```

`startsForThisEdit: 3` counts console LINES (a request logs `performInvocation`, `command ingress lane`
and `performInvocation settled`): **one** `toolRunStart` for the edit — two `performInvocation` requests
in the whole session, one at boot and one for the edit. Before this lane the same
measurement read `digestAfter == digestBefore`, `startsForThisEdit: 0`, `ticksForThisEdit: 0`
(`🗑️generated/preview-rearm/run2/`).

### 5.2 The gap probe step

`SEMIO_PROBE_OUT=preview-rearm/gaps6 bun 🐍️react-gap-probe.mjs` → **`GAPS DONE 11/11`**, with

```
inspection-preview-rearm ok=true {"previewArmed":true,"rearmed":true,"valueMoved":true,"typed":7,
  "bytesBefore":{"window:procedural-preview":"3644:cd770756"},
  "bytesAfter":{"window:procedural-preview":"596:e7852e37"},"previewMoved":true}
```

### 5.3 The full battery

`cd T && bun 🐍️react-battery.mjs` → **`BATTERY DONE green=16/16 red=[] 1203s pageerrors=12`**
(`🗑️generated/preview-rearm/battery-1.txt`, scoreboard at `🗑️generated/react-verify/scoreboard.json`).
`gaps` is `ok=true exit=0 140s steps=11/11` — every one of its eleven steps green, including
`export-after-generate` (`genInvoked: ["addGeneration","toolRunStart","flowEvalTick"]`,
`meshes: 1`) and `catalogue-add`, whose `addWidget` now also re-arms
(`invoked: ["addWidget","toolRunStart","flowEvalTick"]`) where before it re-armed nothing.

The twelve `pageerror` lines are spread over the sixteen probes and are the same pre-existing ones the
previous lane's runs carry (`FlowMessageRejected`, `Error: cancelled`, two
`Cannot read properties of null (reading 'addEventListener')`); the battery's own fault gate passed.

### 5.4 The storm

`toolRunStart` REQUESTS per session (`performInvocation {"invocationKind":"action",…"toolRunStart"}`):

| session | starts |
|---|---|
| §9's measurement, 242 s of `gaps` | **1 053** |
| this lane's `gaps6`, eleven steps, 140 s | **9** |
| this lane's focused probe (`run14`), boot + one edit | **2** (one at boot, one for the edit) |
| this lane's own worst intermediate state, with the request latch keyed on the wrong action (`run8`) | **3 568 in 45 s** — removed by §3.3's `Finalize` key |

---

## 6. Files

Created:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔢️action-arg-carriers/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📖️read-only-run/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts`
- `🐍️preview-rearm-probe.mjs`, `🔍️gesture-rearm.ts`, `🔍️tool-run-arg-carriers.ts`, `🔍️read-only-run.ts`
- `📓️preview-rearm-after-inspector-edit-2026-09-14.md` (this report)

Updated:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` — the carrier reader, the read-only
  auto-finalize
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `pending_effects` binds the run view
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json` — `actionArgCarriers`,
  `readOnlyRun`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` — two laws, the read-only
  toy tool
- `✏️s/…/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` — the request latch, the emit-keyed owe, the carrying
  owe, the prompt finalize, the job-ownership guard
- `✏️s/…/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` — the two new laws, `runEffects`
  generalized to sequences
- `✏️s/…/🪆️subsets/✳️any/🧫️fixtures/⏯️preview-eval-run.json` — `gestureRearm`, `jobSupersession`,
  `runEffects`
- `✏️s/…/🪆️subsets/✳️any/✏️editor/🦀️.rs` — every owing route carries the start
- `✏️s/…/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` — the peer fix-forward of §3.5
- `🐍️react-gap-probe.mjs` — `inspection-preview-rearm`'s predicate strengthened

---

## 7. What is NOT claimed

- **Nothing about the wgpu surface's runtime.** Every change here is surface-neutral (the framework
  reader, the poll's run view, `🧵️preview-eval`) or editor-side, and the wgpu shell must benefit from
  the finalize fix identically — but no wgpu browser run was taken in this lane. `6118` is
  `wgpu-end-to-end-verification`'s.
- **Nothing about the VIEWER surface's gesture routes.** `👁️viewer/🦀️.rs`'s two `owe_attached_previews`
  call sites still only OWE; they do not carry the start. They compile against the new `&mut` signature
  and are unchanged otherwise. The viewer's own view commands were not measured here, and if the same
  silence applies to them it is the same fix, not a different one.
- **`preview_eval_run_effects` no longer leaves a complete run standing.** `a-settled-complete-run-stays`
  is now `a-settled-complete-run-is-finalized`, and
  `a-complete-run-with-an-owed-window-is-finalized-and-restarted-at-once` is
  `…-is-finalized-first`. These are the peer lane's rows, changed forward with the reason on each row,
  not reverted. With defect 2 fixed a read-only run rarely reaches this arm at all; the arm stays
  because a MUTATING surface run would.
- **The read-only auto-finalize is a framework contract decision, taken from the defect rather than
  with the peer.** It says: a run that publishes nothing has nothing to finalize, so its completion is
  its finalization. If their intent is that a read-only run should still be reviewable in `Complete`,
  the place to say so is the run definition — a third policy beside `mutating` — not this call site.
- **The 1 053-start storm was never reproduced by this lane.** §9's own measurement is the record; the
  session that produced it was overwritten before this lane started. What this lane measured and fixed
  is the same latch at smaller scale (three starts in 500 ms on a quiet boot) and a 7 404-start storm it
  caused itself with a wrong latch key and then removed (`🗑️generated/preview-rearm/run8/`).
- **`@semio-tech/framework-renderer-react:typecheck` was not run.** No `.tsx` was touched; the three new
  `.ts` twins are run directly by their ticket runners.
- **No claim about tool runs of other apps beyond the two framework laws.** The carrier fix and the
  read-only auto-finalize are framework-wide by construction and the 18 `tool_run_tests` pass, but only
  generation3d was exercised in a browser.
- **The whole cargo workspace was not built.** `semio-framework-plugin`'s and
  `semio-s-artifact-procedural-generation3d`'s own lib test binaries both compile (that is what running
  their suites proves), and both framework edits are additive at their call sites — but no other crate
  was compiled against them in this lane.
- **The battery's twelve `pageerror` lines were not investigated.** They are spread across the sixteen
  probes, match the signatures the previous lane's runs already carry, and none of them is new to this
  lane — but this lane did not chase them.
