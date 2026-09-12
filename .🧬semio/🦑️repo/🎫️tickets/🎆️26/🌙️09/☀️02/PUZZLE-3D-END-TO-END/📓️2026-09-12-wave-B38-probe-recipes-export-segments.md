# Wave B38 — the six probe recipes, the segmented export lane, and the clipboard red on #53

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · wasm #53 on `:6013` (host vite-live).
Written incrementally while the wave ran. Every command below ran in the FOREGROUND with its tail quoted.

Inputs read first: `📓️2026-09-12-wave-B36-full-run-bisect-2.md` (§7 recipes + §5 export),
`📓️2026-09-12-wave-B35-probe-ring-brush-gate.md`, `📓️2026-09-12-wave-B20-world-surface-identity.md` §1.3,
`📓️2026-09-12-wave-B26-residual-lanes.md` §1, `📓️2026-09-11-wave-B13-export-history-locale.md` §2.

---

## 0 Headline

| # | item | outcome |
|---|---|---|
| 1 | the six probe recipes | all six applied and measured. `locale-control-present`, `catalogue-add-selects-new-object`, `duplicate-reselects-clone`, `locked-flag-row` **FAIL → PASS** on the unchanged #53 guest; `projection-repaints-camera` and `delete-selection` became HONEST product reds; and every remaining red in the lane set traces to ONE product cause — **switching the UI language revokes the plugin actor** (§1.7) |
| 2 | export of large fixtures | **product, fixed.** `export_fixture` picks its lane by the framework's own `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`: inline at or under one wasm page, the segmented-download lane above it. New `PuzzleCommandWorkStep::Download` carries it through the shared puzzle command session. The drain DID lack something — its only sink fails closed without `showSaveFilePicker` — so the host now assembles and delivers through the inline lane's own blob-and-anchor. Law drains a 145 714 B Nakagin export chunk by chunk and reassembles it byte-for-byte (rides #55 for the guest) |
| 3 | the clipboard red | **probe read.** With the precondition proven through `data-selection-json`, `mod+c` → `ClipboardWrite` → `mod+v` → `create-object` runs end to end on #53: `clipboard` **FAIL → PASS**, `delta=1`, history `Copy` / `create-object object-1` / `Paste`. No product change needed or made; two reserved-clipboard silences handed over with their hops |

One gate was found reading green while measuring nothing: the whole segmented-download drain corpus
(`🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts`, seven laws) was in no runner's include list at all (§2.6).

---
## 1 The six probe recipes — all six applied, all six measured on #53

`🔍️browser-probe.ts` only. Every step name and every pre-existing verdict name is unchanged; one verdict is
ADDED (`selection-precondition`), because a precondition that cannot be established must be reported as one
rather than as the command's failure.

### 1.1 `ensurePanel(tabId, bodySelector?)` — a tab click is never used as "ensure open" again

`🔍️browser-probe.ts` new helper (after `dismissChrome`), plus `PANEL_BODY_SELECTORS`: the body a panel tab
reveals, per tab id, because a panel body carries the SURFACE's id prefix (`panel:puzzle3d-play-inspector/…`),
which is not derivable from the tab id. The rule it encodes, and the one B36 §7.1 asked for:

- **the body is the authority** whenever one is known — a panel rendering its rows IS open; the tab's own
  `aria-selected`/`data-state` is the fallback for a panel whose body nobody has measured yet. `framework.settings`
  needs exactly this: it is a BRANCH tab that mounts its `order: 0` child and never carries `aria-selected` itself.
- already open → **no click at all** (`clicked=false`), so neither a predecessor nor the shell's own auto-reveal
  on selection can be inverted;
- closed → one click, then POLL for the body (8 s) instead of sleeping.

Migrated call sites: `readHistoryEntryIds` (B36 §1.2's own defect — the reader was closing the panel it measured),
`openHistory`, `openPanel` (now click-free: it resolves the tab by regex and delegates), the `selection-surfaces`
inline Inspection click, the `locked-refusal` inline Inspection click, the `brush-stroke` trailing History click,
plus body selectors for the outliner (`[id^="panel:puzzle3d-play-document/"]`), the catalogue
(`[id^="puzzle3d-play-kinds"]`) and the app Settings panel (`[id*="puzzle3d-play-settings."]`).

Measured live — the recipe does exactly what it claims:

```
[84.6s] ensurePanel framework.panel.inspection clicked=true  active=true body=4 waitedMs=60
[97.3s] ensurePanel framework.panel.inspection already-open active=true body=4 clicked=false
[125.8s] ensurePanel framework.panel.inspection already-open active=true body=4 clicked=false
[27.9s] ensurePanel framework.panel.artifact  clicked=true  active=true body=7 waitedMs=19
[35.1s] ensurePanel framework.panel.artifact  already-open active=true body=7 clicked=false
```

Three reads of the Inspection panel across two steps, one click total, `body=4` throughout. Under the old
helper the second and third of those clicks are the `inspectorRows=0` of B36 §1.1.

### 1.2 The language control by exact id

`setLanguage` now calls `ensurePanel("framework.settings")` (body `[id="framework.settings.language"]`), with
`framework.settings.general` as the explicit descend fallback, and addresses the select as
`[id="framework.settings.language"][role="combobox"]`. `openPanel(/settings/i)` could never reach it: the app's
own `puzzle3d.panel.settings` precedes the shell's `framework.settings` in the roster and BOTH render the text
"Settings" (B13 §3a, B36 §7.2).

```
[29.6s] ensurePanel framework.settings clicked=true active=true body=2 waitedMs=369
[29.7s] locale language control panel={"opened":true,...,"body":2} triggers=1
[23.3s] verdict locale-control-present PASS
```

**`locale-control-present` FAIL → PASS**, in both runs. The switch really lands — the whole shell roster comes
back German one poll later (`"Inspektion"`, `"Dokument"`, `"Katalog"`, `"Einstellungen"`, `"Verlauf"`,
`"Allgemein"`, `"Tastenkürzel"`), and `locale-switch-back-en` PASSes.

### 1.3 `projection-options` filtered to real controls

The step now enumerates `[id^="puzzle3d-measure-projection"], [id^="framework.worldOrbit.projection"]` WITH each
hit's `data-slot`/`role`, marks a hit a control only when its slot is one of
`select-trigger|slider|tree-action-checkbox|toggle-group-item|numberStepper` (or its role is
`combobox|slider|checkbox|switch|spinbutton`, or it is an `input`), records whether it sits in the perspective
window's subtree, and prefers `puzzle3d-measure-projection-orthographic-view`. When every hit is chrome, BOTH
verdicts report the locator miss instead of scoring the shell.

```
[11.2s] verdict projection-measures-present PASS
[12.6s] verdict projection-control-flips PASS
[12.7s] verdict projection-repaints-camera FAIL id=puzzle3d-measure-projection-orthographic-view slot=select-trigger
        before={"position":[9.17,-5.67,4.2575],"target":[3.5,0,0.005],…,"projection":"…
        after={"position":[9.17,-5.67,4.2575],"target":[3.5,0,0.005],…,"projection":"…
[16.7s] step projection-options: ok windows=2 canvases=2 newFaults=none
```

`projection-repaints-camera` is now an HONEST red and no longer B36 §7.3's pane nudge: the target is the
orthographic-view **`select-trigger`**, it flipped, the step raised no fault, and `data-camera-json` came back
bit-identical. That is a §3 product statement for whoever owns the projection lane — not a probe defect.

### 1.4 `selectionState()` reads `data-selection-json`

Two halves, both repaired:

- world half: `[data-selection-json]` → `selectedIds` + `targetVolumeIds`, keyed by `data-window-instance-id`.
  It used to fold `data-interaction-json`'s `view.selection.ids` — a field `WorldInteractionRecord` has never
  carried (B20 §1.3 says so twice: that record is the utility/brush/fill record). The world half was dead code.
- ARIA half: scoped to the OUTLINER's rows (`[id^="panel:puzzle3d-play-document/"]`). Unscoped it answered with
  Display-panel chips (`puzzle3d-play-distribution=Distribution`, battery #53).

Plus one reader defect found on the way: `dumpInstances()` capped `ids` at the first **16**, and
`catalogue-add-selects-new-object` asks whether the selection names one of those ids — on a 180-object document
the object just added sits far past index 16, so that verdict was unfalsifiable. It now returns every id.

Effect, run 1 (`🗑️generated/b38-probe-six-lanes.txt`):

```
[229.3s] verdict catalogue-add-selects-new-object PASS
[288.4s] verdict duplicate-reselects-clone PASS
```

Both were FAIL in #53 with the dead reader; both PASS on the same #53 guest with the reader fixed. B36 §7.4's
prediction ("that verdict is measuring the reader, not the guest") is confirmed for `duplicate-reselects-clone`,
whose Rust law `duplicate_selection_reselects_the_created_clones` is green.

### 1.5 The precondition, and the census that names its arrivals

New `ensureWorldSelection(label, spot)` — shared by `selection-surfaces`, `locked-refusal` and
`selection-keybindings`: three pane-fraction presses, then the OUTLINER row as the fallback, and the result
proven through the SAME `data-selection-json` read every verdict uses, with the route named
(`via=pane-fraction` / `via=outliner`). `censusTrace` logs one census per poll WITH ids, so an arrival names
itself instead of turning up as an unexplained `after=n+1` (B36 §1.4's unattributed `+1`).

```
[334.9s] keybindings precondition attempt=0 via=pane-fraction ids=["seed-left-001","seed-left-001"]
         state=["puzzle3d-main-top=seed-left-001","puzzle3d-main-perspective=seed-left-001"] waitedMs=67
[335.1s] delete census count=5 +[] -[]
   … 58 more polls, every one count=5 +[] -[] …
[365.7s] verdict delete-selection FAIL selected=["seed-left-001","seed-left-001"] before=5 after=5 removed=[] arrived=[] waitedMs=30566
```

`delete-selection` is now a real product red: a selection PROVEN non-empty on both panes, 30.6 s of polling, no
object removed and **none arrived** — so B36's unattributable `+1` cannot recur, and this red is not a
precondition artifact. `duplicate-selection` PASSes in the same lane with the same route, so `Meta+d` reaches
the guest and `Delete` does not.

`locked-flag-row` FAIL → **PASS** once the precondition is real (run 2, `[126.9s]`).

### 1.6 The verdicts, both runs

`bun 🔍️browser-probe.ts --only=selection-surfaces,locked-refusal,selection-keybindings,catalogue-panel,locale-switch,projection-options --port=6013`

| verdict | #53 (B36) | run 1 `b38-probe-six-lanes.txt` | run 2 `b38-probe-six-lanes-2.txt` |
|---|---|---|---|
| `locale-control-present` | FAIL `switched=false` | **PASS** | **PASS** |
| `locale-switch-back-en` | — | PASS | PASS |
| `locale-no-english-leak` | — | PASS | PASS |
| `locale-flips-document-labels` | — | FAIL (§1.7) | FAIL (§1.7) |
| `locale-de-document-section-label` | — | FAIL (§1.7) | FAIL (§1.7) |
| `projection-measures-present` | PASS | PASS | PASS |
| `projection-control-flips` | PASS (a pane) | PASS (a `select-trigger`) | PASS (a `select-trigger`) |
| `projection-repaints-camera` | FAIL (pane nudge) | FAIL (honest, §1.3) | FAIL (honest, §1.3) |
| `selection-precondition` | new | PASS | PASS |
| `inspection-object-fields` | FAIL | FAIL (§1.7) | FAIL (§1.7) |
| `inspection-locked-flag-row` | FAIL | FAIL (§1.7) | FAIL (§1.7) |
| `locked-flag-row` | FAIL | FAIL (§1.7) | **PASS** |
| `locked-refusal-notice` | FAIL | FAIL `locked=false` | not reached |
| `catalogue-panel-opens` | PASS | PASS | PASS |
| `catalogue-kind-rows-present` | PASS | PASS | PASS |
| `catalogue-add-object-kind` | PASS | PASS | FAIL (§1.7) |
| `catalogue-add-selects-new-object` | FAIL | **PASS** | FAIL (§1.7, `added=0`) |
| `catalogue-drag-drop` | PASS | PASS | PASS |
| `duplicate-selection` | FAIL | PASS | PASS |
| `duplicate-reselects-clone` | FAIL | **PASS** | **PASS** |
| `focus-selection` | FAIL | PASS | PASS |
| `delete-selection` | FAIL | FAIL (honest, §1.5) | FAIL (honest, §1.5) |

Run 1 `PASS=14 FAIL=12 FAULTS=12 first-hard-fault-at=32.3 guest-death-faults=9`; run 2 `PASS=14 FAIL=12
FAULTS=15 first-hard-fault-at=20.6 guest-death-faults=12`.

### 1.7 One product cause owns every remaining red in this lane set: the locale switch REVOKES the actor

Both runs, same hop, measured with `newFaults` per step:

```
[16.7s] step projection-options: ok windows=2 canvases=2 treeItems=0  newFaults=none
[32.3s] step locale-switch:      ok windows=0 canvases=0 treeItems=24
        newFaults=[DEBUG] typed-operation completion effects failed
                  Error: plugin-ui.intake-rejected:intake:actor-activation.revoked
[32.7s] verdict guest-alive-read FAIL recovery=[] canvases=0 surfaces=[] guestDeathFaults=12
[110.4s] step selection-surfaces: … newFaults=pageerror: Error: [DEBUG] program puzzle: no channel for
         instance 1 (createApp not called, or already destroyed)   ×3
```

`projection-options` raises no fault at all; the step that changes the UI language ends with **zero windows and
zero canvases** and 12 guest-death faults, and the next mutate step still finds "no channel for instance 1". So:

1. `locale-flips-document-labels` / `locale-de-document-section-label` are not a translation gap in the
   outliner — the guest that authors those labels is DEAD by the time they are read. (B13 §3b fixed the
   shell's own half; the roster really does come back German, §1.2.)
2. `inspection-object-fields` / `inspection-locked-flag-row` fail behind a panel that is demonstrably OPEN
   (`body=4`, `clicked=false`) and, in run 2, behind a PASSING `selection-precondition` — they are downstream of
   the same revoke, which is why `locked-flag-row` (one step later, after the actor came back) PASSES in run 2.
3. `catalogue-add-object-kind`'s run-2 red (`before=1 after=1 waitedMs=30145`, `activatableRow:"true"`,
   `covered:false`) is the same: the row is wired and un-covered, the actor is not there to answer.

**Handover, not fixed here (host-side, and adjacent to the file wave B37 owns):** a UI-locale change must not
revoke the plugin actor's activation. The signature to look for is
`plugin-ui.intake-rejected:intake:actor-activation.revoked` on a typed-operation completion, immediately
followed by `program puzzle: no channel for instance 1 (createApp not called, or already destroyed)`. Until it
is fixed, `--only=…,locale-switch,…` poisons every step after it, and the ordering alone decides which of the
reds above appear — exactly the variance between run 1 and run 2.

---
## 2 Export of large fixtures — the segmented lane, and the sink that made it silent

### 2.1 Root cause, `file:line`

| hop | where | state |
|---|---|---|
| 0 | `✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` `export_fixture` | pushed `Effect::DownloadMediaExport { data, encoding: Some("utf-8") }` with the WHOLE JSON, unconditionally — **the defect** |
| 1 | `✏️editor/🦀️.rs:8425` / `:8608` | `exportFixture` is `ActionKind::Shell` + `action_interactive_job(…, Migrated)`, and `:7876` builds `Puzzle3dWindowCommandWork` for it — so it already runs as a typed operation, i.e. the lane that CAN publish a download |
| 2 | `✏️editor/🦀️.rs:7149` | its publication contract is `[HostOnly]` — no store lane, which is exactly what a download is |
| 3 | `🎮️commands/🧵️retained/🦀️.rs:34` `PuzzleCommandWorkStep` | had only `Progress` / `Complete(Emit)`: the shared puzzle command session could not express a download at all |
| 4 | `🔌️plugin/🦀️.rs:13455` `ArtifactToolCompletion::complete_download` + `:19090` `segmented_downloads` + `:24144` the `Download` result page + `:26439` the ACK that admits the chunks | the whole lane, fully implemented, and reached by NOTHING in puzzle3d |
| 5 | `📤️SegmentedDownload/🟦️.ts:60` `createSegmentedDownloadSink` | **fails closed** without `showSaveFilePicker` (`segmented-download-streaming-sink-unavailable`) and is the drain's DEFAULT sink — so even a correct producer would have delivered no file outside Chromium, and never one a probe could drive |

The threshold is the framework's own wire constant, not a literal:
`semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` = 65 536 ("Largest CONTIGUOUS block a routine,
per-command or per-turn guest path may request… one wasm page", `🧮️memory/🦀️.rs`). B36's measured pair sits
exactly astride it — Concrete Forest 7 542 B downloads, Nakagin Capsule Tower 145 714 B does not.

### 2.2 The fix — guest

`✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` is now the one place the lane decision lives:

- `puzzle3d_export_inline_budget_bytes()` — `const fn` returning `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`.
- `puzzle3d_export_json(&Puzzle3dFixture)` — the exported bytes, unchanged projection.
- `puzzle3d_export_inline_effect(...)` — the inline `DownloadMediaExport`, for a payload at or under the budget.
- `puzzle3d_export_segmented(...)` — the payload sliced by `ArtifactOutputChunks::CHUNK_BYTES` (the framework's
  own per-chunk cap, newly EXPORTED for exactly this reason — see §2.4) into an `ArtifactDownloadOutput` with
  `identity` encoding, capped by this command's declared contract output budget `PUZZLE_COMMAND_OUTPUT_BYTES`,
  so a fixture larger than the tool declared it may produce faults in the producer instead of being truncated
  on the wire.
- `puzzle3d_export_publication(...)` → `Inline(Effect)` | `Segmented(ArtifactDownloadOutput)`.

`✏️editor/🦀️.rs` `Puzzle3dWindowCommandWork::step`, `Scene` stage: `exportFixture` resolves there (it reads the
document and publishes a download — it owns no mutation, no precompute session and no placement scene) and
returns `PuzzleCommandWorkStep::Complete(Emit::effect(...))` or the new `…::Download(...)`. Same shape as
`openImportFixture`, which already has both a shell-only arm and a leftover reducer arm.

`🎮️commands/🧵️retained/🦀️.rs` (shared by puzzle 2d/3d/5d): `PuzzleCommandWorkStep::Download(ArtifactDownloadOutput)`,
a `download` owner on `RetainedPuzzleCommandJob` retired by `close_step`/`terminal_is_empty` like every other
owner, and a `Publish` phase that calls `completion.complete_download(...)` instead of `complete(...)`. The
variant carries NO mutation on purpose — a download is a HostOnly publication, and a work object that owes
both must publish them as two steps.

The LEFTOVER (non-interactive) arm keeps the inline effect and, above the budget, pushes one notice naming the
route that streams: it holds no typed-operation completion authority, so it cannot publish a segmented output,
and silence is the one answer a download must never give.

### 2.3 The fix — host half (the drain DID lack something)

`📤️SegmentedDownload/🟦️.ts`: `createBufferedDownloadSink(filename, mimeType, deliver)` assembles the drained
chunks in order and hands the finished bytes to `deliver` on `close` — and delivers NOTHING on `abort`, so a
partially drained payload never reaches the user as a file that looks complete. `segmentedDownloadSinkFactory(deliver)`
is the factory a shell passes in. `createSegmentedDownloadSink` (the File System Access stream) stays exported
for a caller that genuinely wants it, but it can be neither the default for a plain "Export" press (it opens a
native Save-As dialog) nor the sink an automated check can drive. Everything the drain admits is bounded by
`MAX_SEGMENTED_DOWNLOAD_BYTES` (32 MiB), so assembling is bounded too.

`🛠️ShellHelpers/🟦️.tsx`: `downloadMediaExportBytes(filename, mimeType, bytes)` — the blob-and-anchor half of
`downloadMediaExport`, i.e. the segmented lane now delivers through the SAME mechanism as the inline lane —
and `shellSegmentedDownloadSinkFactory`, wired to it.

`🏛️ShellHost/🟦️.tsx:~5254`: the drain call passes `sinkFactory: shellSegmentedDownloadSinkFactory`. (Nothing
near `refreshUi`/the coalescing region wave B37 owns was touched.)

### 2.4 One framework export

`🔌️plugin/🦀️.rs` `impl ArtifactOutputChunks`: `pub const CHUNK_BYTES = ARTIFACT_OUTPUT_CHUNK_BYTES` — the exact
byte cap `push` already enforces. Every app that fills a segmented output re-declared it as its own literal
(`📤️export/🦀️.rs:45` `const OUTPUT_CHUNK_BYTES: usize = 4_096` in the Layout plugin), and a literal that drifts
from `push`'s check turns a correct producer into a `segmented-output-limit` fault at runtime. The Layout
duplicate is left to its owner; it is now derivable.

### 2.5 Laws

**Guest, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`** — new
`export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture`: asserts the budget IS
the wire constant, that Concrete Forest stays inline with the whole payload and opens NO handle, that Nakagin
publishes exactly one handle with no inline effect at all, that the handle keeps the per-example filename and
declares `identity` + the full byte count, and then DRAINS it one bounded chunk at a time exactly as
`drainSegmentedMediaExport` does and compares the reassembled bytes to the export byte-for-byte plus
object-for-object against the live document.

Testkit support (`✏️editor/🧪️tests/🔬️testkit/🦀️.rs`): `Puzzle3dSettled.downloads`, `Puzzle3dDownload::from_page`
(the Download lane's flat 4-element array, read exactly as the renderer reads it) captured in `settle` where the
host's own `consumeTypedOperationEffects` captures it, and `drain_segmented_download`.

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 export
test editor::puzzle3d::component::tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::tests::export_fixture_names_the_download_after_the_active_example ... ok
test editor::puzzle3d::component::tests::export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture ... ok
test editor::puzzle3d::component::tests::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok
test editor::puzzle3d::component::tests::import_fixture_reproduces_the_exported_document ... ok
test editor::puzzle3d::component::tests::leftover_export_fixture_downloads_the_boot_example_json ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 725 filtered out; finished in 9.78s
```

Measured payload, from the law's own run before the temporary print came out:
`segmented export filename=nakagin-capsule-tower.json bytes=145714 chunkCap=4096` — 36 chunks, reassembled
identical.

**Real coverage, proven** — with the split reverted (`if data.len() <= …` → `if true`) and nothing else changed:

```
test editor::puzzle3d::component::tests::export_over_the_inline_budget_streams_one_segmented_download_carrying_the_whole_fixture ... FAILED
panicked at …🔬️unit/🦀️.rs:5784:5:   ← "an over-budget export must publish no inline download effect"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 730 filtered out; finished in 0.45s
```

`export_fixture_names_the_download_after_the_active_example` is UPDATED, not weakened: it now reads the filename
from whichever of the two lanes published, and asserts that exactly one of them did — the naming contract is
lane-independent, the lane is chosen by size.

**Host half, `🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts`** — `segmented download assembled sink`: the shell's
factory turns a two-page chunked producer into ONE delivered file with the payload in order, and an aborting
drain delivers nothing. Its sibling law one `describe` up already pins the other half —
`createSegmentedDownloadSink` rejects with `segmented-download-streaming-sink-unavailable` wherever the picker
is absent, which is WHY the factory has to exist.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts --testNamePattern='assembled sink' --reporter=verbose
 ✓ …/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts > segmented download assembled sink > assembles every chunk in order and delivers the payload once 2ms
 ✓ …/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts > segmented download assembled sink > delivers nothing when the drain aborts 1ms
 Test Files  1 passed | 29 skipped (30)
      Tests  2 passed | 993 skipped (995)
```

### 2.6 A gate that was reading green while measuring nothing

`🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts` — the WHOLE segmented-download drain corpus, seven laws — appeared in
**no runner's include list**: it sits under `🧱️elements/…/🧪️tests/`, which this vitest package's default `include`
glob never reaches, and it was named nowhere (`rg` for its path across the repo: zero hits outside the file
itself). Fixed in `…/🎯️targets/⚛️react/vitest.config.ts` with an `elementSuite(element, name, ext)` resolver
beside the existing `engineSuite`, and `elementSuite("🛠️ShellHelpers", "🧩️component")` added to
`engineTestSuites`. First run after inclusion: `Test Files 1 passed | Tests 9 passed` (the seven that never ran,
plus this wave's two).

### 2.7 What is NOT proven, and why

The browser verdicts `export-only` / `export-names-the-example` on Nakagin **cannot flip on `:6013` this wave**:
the split is guest Rust, so it **rides #55**. The host half IS vite-live and is covered by §2.5's vitest laws.

One premise correction on B36 §5, stated because the fix stands on the framework's own budget rather than on it:
B36 attributed the drop to "payload size alone", but its two lanes were `--only=export-import` (Concrete Forest,
no predecessor) and `--only=example-switch,export-import` (Nakagin, behind a switch) — Nakagin is not reachable
WITHOUT an example switch, so size and the predecessor were never separated in the browser. The native evidence
is unambiguous about the byte counts (7 542 / 145 714) and the framework's own ceiling is 65 536, so the lane
split is right either way; but if `export-only` is still red on #55, the example switch — which this wave
measured revoking the actor outright (§1.7) — is the next suspect, not the payload.

---

## 3 The clipboard red on #53 — the probe read, and it is GREEN now

### 3.1 The route, traced and verified hop by hop

| hop | where | state |
|---|---|---|
| 0 | `🛂️manifest/🦀️.rs:1047-1070` `clipboard_action_definitions` | `copy` (`mod+c`), `cut` (`mod+x`), `paste` (`mod+v`, args `anchor`/`position` only) — framework-reserved, injected into every app |
| 1 | `🔌️plugin/🦀️.rs:5285` / `:5340` | injected into the `AppDefinition` and synthesized into `session.app.keybindings` |
| 2 | `🏛️ShellHost/🟦️.tsx:8208-8236` `handleAppKeydown` | matches the chord; `copy` has no args → fires directly; `paste` HAS args → staged-form path |
| 3 | `🏛️ShellHost/🟦️.tsx:8217-8220` | `pasteActionWithRetainedFragment` injects `args.fragment` from `clipboardFragmentRef` and executes; with NO retained fragment it opens the staged form instead — a dispatch that never happens |
| 4 | `✏️editor/🦀️.rs:7937-7942` | puzzle3d overrides the reserved route with `Puzzle3dClipboardJob` for all three verbs |
| 5 | `✏️editor/🦀️.rs:7645-7649` `Puzzle3dClipboardJob::emit` `"copy"` | `Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }] }`, `Err(_) => Emit::default()` |
| 6 | `🏛️ShellHost/🟦️.tsx:5180-5182` | `clipboardFragmentRef.current = clipboardWriteFragmentFromEffect(effect)` |
| 7 | `✏️editor/🦀️.rs:7657-7664` `"paste"` | `args.get("fragment")` → `puzzle3d_paste_operations_on` → `artifact_mutations` |

Every hop is intact. The three guest laws are green on this code:

```
RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1 copy
test editor::puzzle3d::component::tests::copy_then_paste_clones_selection_as_one_mutation ... ok
test editor::puzzle3d::component::tests::leftover_copy_paste_clones_object_from_selected_vortex_uuid ... ok
test editor::puzzle3d::component::tests::leftover_copy_paste_clones_selected_object ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 728 filtered out; finished in 0.17s
```

### 3.2 Root cause of the red: the step's own precondition, plus two destructive gestures

`🔍️browser-probe.ts`'s `clipboard-copy-paste` (before this wave):

1. established its selection with `clickForestTable()` + one canvas click at the pane fraction and **never
   verified it** — the same precondition that §1.5 showed lands on empty space. Hop 5 answers an empty
   selection with `Emit::default()` — no effect, no notice, no fault — so the ref stays `undefined`, hop 3
   opens the staged paste form, nothing is dispatched, and the verdict reads `delta=0
   historyHasCopyPaste=false`: identical to a broken clipboard.
2. pressed *the last button labelled "actions"*, which **folds** the Actions pane (B26 §1a), and
3. typed `copy` into the command palette — two gestures that could only add state to a step whose whole
   question is whether one hotkey reaches the guest. (`copyBtn=0 copyById=0` was never a defect: no Copy
   control exists anywhere — the three verbs are hotkey-only.)

### 3.3 The probe fix, and the measurement

Route is now the hotkey alone. `clipboard-precondition` (new verdict) establishes and PROVES the selection
through `data-selection-json`; the control census is kept as an observable and clicks nothing; `clipboardHops`
reads each hop off the host's own console — the `copy` invocation and its effect count, the `paste` invocation
and its effect count — so a red names its hop. `copy` pushes exactly one effect and answers a refusal with
zero, so `"effects":1` on a settled `copy` IS the fragment, measured host-side.

`bun 🔍️browser-probe.ts --only=clipboard-copy-paste --port=6013` (`🗑️generated/b38-probe-clipboard-2.txt`):

```
[14.1s] clipboard precondition attempt=0 via=pane-fraction ids=["seed-left-001","seed-left-001"]
        state=["puzzle3d-main-top=seed-left-001","puzzle3d-main-perspective=seed-left-001"] waitedMs=1
[14.1s] verdict clipboard-precondition PASS
[14.6s] clipboard inspection-wait populated=true empty=false id=Id seed-left-001 lock=true
[14.6s] clipboard census before: {"count":1,"ids":["seed-left-001"]}
[14.6s] clipboard controls=[] (none is expected: copy/cut/paste are reserved hotkey-only verbs)
[15.4s] clipboard copy hops={"copyTurns":1,"copyEffects":["1"],…} waitedMs=1
[15.4s] verdict clipboard-copy-writes-a-fragment PASS
[16.2s] clipboard paste hops={…,"pasteTurns":1,"pasteEffects":["0"],…} waitedMs=0
[16.2s] verdict clipboard-paste-reaches-the-guest PASS
[16.2s] clipboard staged-paste execute controls=0
[17.0s] clipboard census after: {"count":2,"ids":["seed-left-001","object-1"]} delta=1
[17.0s] verdict clipboard PASS
[17.0s] clipboard history: […"framework.history.entry.12=Copy","…entry.14=create-object object { id=object-1 …",
        "…entry.15=Paste↶","…entry.16=create-object object { id=object-2 …","…entry.17=Paste↶"…]
```

**`clipboard` FAIL → PASS on the unchanged #53 guest**, with the whole chain named: one `Copy`, one retained
fragment, one `paste` dispatch that never needed the staged form (`execute controls=0` — it executed on the
hotkey), `create-object object-1`, one `Paste` history row. B6's "paste creates an object" holds in the browser.
No product change was needed, and none was made.

### 3.4 Handover — the silence that made this red unattributable

`✏️editor/🦀️.rs:7645-7649` / `:7653` / `:7660`: `Puzzle3dClipboardJob::emit` answers EVERY refusal —
empty-selection copy, empty-selection cut, paste with no fragment — with a bare `Emit::default()`. A user
pressing `mod+c` with nothing selected gets no notice and no fault; so did five browser batteries.

It cannot be repaired in place: `ArtifactReservedToolInput::Action { args, interaction, hover }` carries **no
`ViewModel`**, and `puzzle3d_notice_emit`/`Puzzle3dActionCtx::notice` resolve their text through
`puzzle3d_labels(view_state)` — with no view state the only reachable message is
`PUZZLE3D_LOCALIZATION_UNSUPPORTED`, which would trade silence for a default-language violation. The clean fix
is to carry the `ViewModel` on the reserved-tool request (framework, `🔌️plugin/🦀️.rs`
`ArtifactReservedToolJobRequest`), then refuse through the app's own localized `nothing_selected` label exactly
as `refuse_without_selection` does. Not raced here.

Second handover, same family: `paste`'s declared args are `anchor`/`position` only (`🛂️manifest/🦀️.rs:1064-1067`),
so the staged paste form the shell opens when no fragment is retained has **no field for `fragment`** and
submitting it can never paste (`✏️editor/🦀️.rs:7659` returns `Emit::default()`). A staged form that cannot
succeed should not be the fallback for a missing clipboard; the honest fallback is a refusal notice.

---
## 4 Gates

All foreground, tails quoted.

| command | tail |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings …` / `Finished dev profile [unoptimized] target(s) in 2.48s` — **0 errors**, 88 warnings (B35/B36's baseline count, so the expansion really ran) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests` | `warning: … (lib test) generated 111 warnings` / `Finished dev profile … in 0.38s` — 0 errors |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `warning: semio-s-artifact-puzzle-5d (lib) generated 1 warning` / `Finished dev profile [unoptimized] target(s) in 0.80s` — 0 errors |
| `cargo check -p semio-framework-plugin` | `warning: semio-framework-plugin (lib) generated 5 warnings` / `Finished dev profile … in 9.93s` — 0 errors |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1 export` | `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 725 filtered out; finished in 9.78s` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1 copy` | `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 728 filtered out; finished in 0.17s` (`clipboard` matches no test name in this crate — the laws are spelled `copy_then_paste…` / `leftover_copy_paste…`) |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1` (whole lib suite) | `test result: FAILED. 717 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.47s` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts --testNamePattern='segmented download'` | `Test Files 1 passed | 29 skipped (30)` / `Tests 9 passed | 986 skipped (995)` |
| `bun build 🔍️browser-probe.ts --target=bun --external '*'` | `🔍️browser-probe.js 166.1 KB (entry point)` |
| `bun 🔍️browser-probe.ts --only=boot --port=6013` | `verdict boot PASS` · `battery PASS=3 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0` |

**Renderer-react vitest lane, whole corpus at `long`** (the host half was touched, so this ran in full):

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts
 Test Files  4 failed | 26 passed (30)
      Tests  32 failed | 965 passed (997)
```

**No NEW failure.** The four failing files are ones this wave never touched, and every failure names a cause
outside the download lane: `🧪️tests/🔬️engine-contract/🟦️.ts` — 7, all `extension invocation completion
ownership` / `noteShellCommand` argument diffs; `🧪️tests/🧩️package-integration/🟦️.ts` — 6, every one
`ReferenceError: Bun is not defined` (the wgpu worker-render laws need the Bun global, absent under this
runner's environment); `🧪️tests/🔌️plugin-runtime/🟦️.tsx` and `🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the rest,
peer churn on shapes this wave does not write (`readAppDocumentPack` gained an `"ops"` field, a packed window
projection gained a `details`/`details-body` pair). The file this wave ADDED to the corpus
(`🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts`, §2.6) is among the 26 that pass, with all 9 of its laws green.

**The 14 lib-suite reds are B36's exact 14** (B36 §9: "716 passed, 14 failed"); this wave's count is
`717 passed; 14 failed` — one MORE pass (the new export law) and the same failure set, none of it in an edited
region: `every_context_menu_row_dispatches_a_declared_action`,
`gumball_active_only_for_transform_utilities_with_object_selection`,
`open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`,
`selected_object_inspector_renders_that_object_field_group`,
`settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on`,
`the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`,
`the_settings_panel_renders_the_focused_panes_own_value_not_a_default`,
`two_instances_converge_disjoint_object_edits_via_backbone`,
`world_pick_null_clears_without_reselecting_first_object`,
`world_vortices_reveal_in_selected_mode_only_for_the_selected_object`, and the four
`panels::catalogue::tests::*` (catalogue/settings/context-menu/gumball/world-pick lanes — all peer lanes, live
this night).

No temporary `[DEBUG]` log survives this wave: the one print inside the new export law (quoted in §2.5) was
removed and the law re-run green afterwards.

---

## 5 Files

**Product — guest (rides #55):**
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` — the budget, the two lanes, the leftover notice.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `exportFixture` resolves in the `Scene` stage and picks its lane.
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` — `PuzzleCommandWorkStep::Download`, the `download` owner, `complete_download` in `Publish`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — the new variant's arm in the 5d board-events work (a download is not a board mutation).

**Product — framework:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `ArtifactOutputChunks::CHUNK_BYTES`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload/🟦️.ts` — `createBufferedDownloadSink`, `segmentedDownloadSinkFactory`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `downloadMediaExportBytes`, `shellSegmentedDownloadSinkFactory`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — the drain passes that factory (one call site, ~5254; nothing near the coalescing region).

**Laws / harness:**
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the new segmented-export law, the updated naming law, `Download` arms.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `downloads` channel, `Puzzle3dDownload`, `drain_segmented_download`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` — `Download` arms.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts` — the assembled-sink laws.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` — `elementSuite` + the ShellHelpers suite (§2.6).

**Ticket:**
- `🔍️browser-probe.ts` — `ensurePanel` + `PANEL_BODY_SELECTORS`, `openPanel`/`openHistory`/`readHistoryEntryIds`
  migrated, `ensureWorldSelection`, `censusTrace`, the repaired `selectionState()`, the uncapped
  `dumpInstances().ids`, the framework-settings language route, the filtered `projection-options`, the rebuilt
  clipboard step. No step renamed, no pre-existing verdict renamed; three verdicts ADDED
  (`selection-precondition`, `clipboard-precondition`, `clipboard-copy-writes-a-fragment`,
  `clipboard-paste-reaches-the-guest`).
- `🗑️generated/b38-probe-boot.txt`, `b38-probe-six-lanes.txt`, `b38-probe-six-lanes-2.txt`,
  `b38-probe-clipboard.txt`, `b38-probe-clipboard-2.txt`, plus the runs' own
  `probe-2026-09-12T06-48-*.md`/`.ndjson`.

---

## 6 For the fleet

1. **`clipboard` is GREEN on #53** (§3.3) — probe read only, no product change. Any battery that still reports it
   red is running a pre-B38 probe.
2. **A UI-locale change revokes the plugin actor** (§1.7). Signature:
   `plugin-ui.intake-rejected:intake:actor-activation.revoked` on a typed-operation completion, then
   `program puzzle: no channel for instance 1`. Until fixed, `locale-switch` poisons every step after it and the
   step ORDER decides which reds appear — do not attribute a red that follows it to its own lane.
3. **`projection-repaints-camera` and `delete-selection` are honest product reds now** (§1.3, §1.5): a real
   `select-trigger` flipped with a bit-identical camera, and a proven non-empty selection with a 30 s census in
   which nothing left and nothing arrived.
4. **Never use a panel tab click as "ensure open"** — use `ensurePanel`. The body, not the tab state, is the
   authority for a branch tab.
5. `export-only` / `export-names-the-example` on Nakagin ride **#55**; the host half is live and law-covered now
   (§2.5). If they are still red on #55, suspect the example switch (item 2), not the payload (§2.7).
6. Two reserved-clipboard silences are handed over with their exact hops (§3.4): a refusal with no notice
   because the reserved-tool request carries no `ViewModel`, and a staged paste form with no `fragment` field.
