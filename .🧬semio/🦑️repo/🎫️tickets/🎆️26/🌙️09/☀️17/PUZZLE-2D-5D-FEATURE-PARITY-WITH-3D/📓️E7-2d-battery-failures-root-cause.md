# E7 — root cause of the 2026-09-17 puzzle 2d battery's 4 FAILs + 2 FAULTs

Source battery: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated/probe-2026-09-17T02-14-35.{md,ndjson}`
(PASS=33 FAIL=4 FAULTS=2), probe source `…/🔍️browser-probe.ts`, history `…/📓️status.md`.

All four failures and both faults are traced to source. Three are real product defects (one is a
**framework-wide** defect, not 2d-specific); one FAIL is primarily a probe robustness bug; the 2
"FAULTS" are a probe fault-classification artifact, not runtime defects.

---

## 1. FAIL `8-transform/engagement-move` — the engagement line cannot carry numeric arguments (framework bug)

**Symptom:** typing `move 50 25` into the window's Action line and pressing Enter never moves the
selected node (`before === after`, 20 s wait). Node id `puzzle2d.fill.1` stayed at
`[325.33, 50.33]`.

**Mechanism (file:line):**

- The Rust command that parses the typed line is correct:
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📨️engagement-submit/🦀️.rs:12-15,39-42`
  — it lower-cases the value, splits on whitespace, and requires `verb == "move"` with
  `numbers.len() >= 2` before calling `puzzle2d_transform_selection(.., Translate{dx,dy})`.
- The **shared** window Action-line component that feeds this command strips every space before the
  Rust code ever sees the text. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`:
  - `normalizeEngagementActionText` (line 10217-10230) removes every non-alphanumeric character
    (except a protected decimal point) and PascalCases what's left: `"move 50 25"` →
    `"Move5025"`.
  - Every `onChange` from the real `<input>` goes through this normalizer:
    `Input … onChange={(event) => applyDraft(event.target.value)}` (line 10851), and `applyDraft`
    (line 10771-10782) calls `normalizeEngagementActionText(value)` before ever calling
    `input?.onChange?.(normalized)` — there is no path that preserves the literal typed text.
  - On Enter (line 10888-10893): `input!.onSubmit?.(draft)` submits the *normalized* `draft`
    (`"Move5025"`), not the literal keystrokes.
- `engagement_submit` therefore receives `args.value == "Move5025"`. `.to_lowercase()` →
  `"move5025"`, a single whitespace-free token. `words.next()` consumes the whole thing as the verb,
  leaving no words to parse as numbers, so `verb` never equals the literal string `"move"` and the
  `_ => false` arm is taken. No mutation is queued; `engagement_input_by_pane` is also never cleared
  (see engagement-submit `🦀️.rs:53-55`, only reset when `applied == true`), so the stale text lingers
  in the field for later steps too.
- `normalizeEngagementActionText` was built for the CAD-style single-token verb grammar
  (`PlaceColumn`, `2Points` — see memory `feedback-shell-action-line-echo-race.md`), where collapsing
  whitespace is the whole point. It is wrong for any grammar (puzzle 2d's own placeholder text is
  literally `select, brush, fill <n>, clear, move <dx> <dy>, rotate <deg>, scale <factor>`) that needs
  space-delimited numeric arguments.
- **This is not puzzle-2d-specific** — `Search` (same file, line 10741 on) is the one component every
  app's Action line renders through. Puzzle 3d's own battery (`…/PUZZLE-3D-END-TO-END/📓️2026-09-12-wave-B29-dock-chrome-engagement.md:371-372`) only ever exercised bare-verb engagement commands
  (`engagement-brush-verb`, `…-fill-verb`, `…-abort`, `…-clear-is-a-noop`) — it never drove a
  space-separated numeric verb (`translate`/`move`) through the Action line, so this defect has never
  been exercised or fixed for 3d either; 3d simply never hit it because its battery didn't try.

**Fix:**
- *Product (framework, shared):* `normalizeEngagementActionText`/`applyDraft` must not destroy spaces
  for apps whose engagement grammar needs them. Minimal options: (a) stop PascalCasing/space-stripping
  entirely and let each app's own parser own tokenization (puzzle 2d/3d already lower-case and
  `split_whitespace` on the Rust side); or (b) make the normalizer opt-in per `SearchInput`/`EngagementSpec`
  (a `grammar: "verb" | "free-text"` flag) so CAD keeps PascalCase collapsing and puzzle keeps literal
  space-delimited args. Given the CAD grammar is name-token-only and puzzle's is verb+args, (b) is safer.
- *Probe:* no probe fix needed once the product is fixed — `input.fill("move 50 25")` correctly
  exercises the real UI path (it goes through the same controlled `onChange`, so the current probe
  already reproduces the real defect faithfully). If the product fix is deferred, the probe should at
  least assert `input.value` after fill to show `"Move5025"` instead of silently waiting 20 s.

---

## 2. FAIL `20-history/undo-changes-document` — likely per-placement (not per-gesture) commits blow the 64-edit ledger

**Symptom:** pressing Undo (`#action.undo`, confirmed present and clicked) never changes the entity
count (`before === after == 1297`), 20 s wait. This follows a 100-placement Fill run + a
`delete-selection` + a catalogue `click-adds-node`.

**Mechanism:**

- Every `ArtifactStore` (document, config, draft, …) has one fixed, non-compacting history ledger:
  `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:186`).
  `ArtifactHistoryLedger::reserve_slot` (same file, lines 335-347) returns
  `ArtifactHistoryReservationFault::Capacity` once `slots.len() == 64` and there is no free (evicted)
  slot — insertion is refused unless something is retired first.
- The framework *does* support folding N mutations from one user gesture into ONE ledger slot — proven
  by `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:3083-3119`
  (`artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step`)
  — but that requires the caller to publish the whole gesture as one batch/`ArtifactCommand::Apply`
  call (or one coalesced `amend`, the same pattern already used for CAD's engagement drafts — see memory
  `project-engagement-history-coalescing-and-64-edit-ledger.md`).
- Puzzle 2d's fill tool does **not** do this. Each placement is committed individually through
  `commit-slot`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️commit-slot/🦀️.rs`
  — `commit_slot()` calls `ctx.host.borrow_mut().brush_commit_slot()` then
  `apply_host_events(..)` once per slot, and this command is driven once per placement by the retained
  fill job (matches the battery's own live progress readout: *"Searching an open handle (2/5) …
  100 placements reached"*, 98.8 s for 100 placements — a streaming, per-placement commit, not one
  batched finalize). 100 individual document commits in one interactive session is well past the
  64-slot ceiling, with none of the coalescing machinery the CONFIG lane already has for exactly this
  problem.
- Corroborating live evidence: the 2d unit suite is *currently* failing 198/724 tests
  (`…/PUZZLE-2D-END-TO-END/📓️status.md:51`), one failure class being exactly
  *"fold / `edit history insertion requires its exact mutation retirement factory`"* — the same panic
  message the store's `puzzle3d_store`/binary-codec doc comment
  (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:32-37`)
  documents as the exact failure mode of an `ArtifactStore` whose ledger/owner bookkeeping is wrong —
  i.e. the fold/retirement path this exact scenario (many rapid document edits) exercises is
  independently known-broken right now.
- Net effect once the ledger is exhausted (whether it hard-refuses new inserts that then get retried by
  evicting the oldest slot, or the fold/retirement bookkeeping documented above is simply broken): by
  the time Undo is pressed, either there is nothing left in the applied ledger that maps to the visible
  delta, or the cursor/`canUndo` bookkeeping the UI reads is desynced — Undo dispatches but has nothing
  correct to pop.

**Fix:**
- *Product:* coalesce fill's per-slot commits into one gesture — either buffer all placements and
  publish them as one `ArtifactCommand::Apply` batch at Finalize (mirrors the 200-mutations-one-slot
  test above and matches how Finalize already behaves as a single UI transition), or `amend` each
  placement under one shared coalesce key the way CAD's engagement drafts do, so one Fill run costs the
  ledger exactly one undo step. Also worth fixing the underlying fold/retirement-factory panic class
  independently (it's failing 198 2d unit tests right now, well beyond this one probe).
- *Probe:* the existing battery already captures `[DEBUG] history patch applied
  {"currentCursor":…, "canUndo":…}` lines for other actions (seen for `Toggle Panel`/`Switch Panel Tab`
  in this same run) — the undo step should log/assert on that line (and on `applied_edit_ids().len()`
  if exposed) around the Fill step instead of inferring purely from node-count deltas, so a future
  regression shows the ledger state directly instead of a bare `before === after`.

---

## 3. FAIL `24-import/import-round-trip` — Actions panel never reopened after the document reload (probe robustness, not proven product)

**Symptom:** `import row not found`; no `filechooser` event ever fired
(`{"chooser":false,"before":101,"after":101,"edges":96,...}`).

**Mechanism:** `probe-2026-09-17T02-14-35-import.png` (captured right after the failed click) shows
all three windows' Actions chrome collapsed (`▷ Actions`, not expanded) — the row genuinely wasn't in
the visible DOM when Playwright tried to click it; this rules out a bad selector.

- The `import` step (`…/🔍️browser-probe.ts:632-654`) starts with `await selectExample(other)` — a full
  document reload (Nakagin → Concrete Forest in this run) — then `closePanels()`, then
  `await setActions(true)`, then races `waitForEvent("filechooser")` against clicking
  `[id="action.openImportFixture"]`.
- `setActions()` (`…/🔍️browser-probe.ts:250-259`) only ever clicks
  `[id="framework.window.2dOverview.engagement.toggle"]` and checks readiness with a **global, not
  window-scoped** query: `actionsOpen()` (line 251) is
  `document.querySelector('[id="action.undo"], [id="puzzle2d-engagement"]')?.offsetParent !== null` —
  it does not scope to the Overview window, so with 3 windows mounted it can be fooled by a stale or
  differently-scoped element and skip the click on its very first check (`for` loop returns `true`
  before ever clicking). Separately, the click itself is `.catch(() => {})`-swallowed on a 3 s timeout
  with no diagnostic, and the caller (unlike the `undo` step, which checks `setActions`'s return value)
  doesn't check it here — `…/🔍️browser-probe.ts:645` calls `await setActions(true);` and discards the
  result.
- `selectExample()` triggers a full example reload; window chrome (and the
  `framework.window.2dOverview.engagement.toggle` button specifically) can be transiently unready right
  after such a reload. With only up to 2 retries × ~1 s settle, a slow re-render after loading a
  100+-node scene is enough to silently miss the click.
- The command-side wiring looks correct and unrelated to the failure: `open-import-fixture`
  (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️open-import-fixture/🦀️.rs`)
  pushes `Effect::RequestFileOpen{..., import_action: "importFixture", ...}` exactly like the pattern
  documented for 3d; there's no evidence the `RequestFileOpen` → chunked `importFixture` path itself is
  broken — the row was simply never clicked, so the effect was never even requested.

**Fix (probe-first, since there's no evidence yet of a product bug here):**
- Scope `actionsOpen()` to the active window (`[data-slot="window"][data-active="true"] …`), matching
  the pattern the framework's own `shouldRouteKeysToWindowSearch` already uses
  (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10432-10445`).
- After `selectExample()`, wait for the Overview window's chrome to be interactive (e.g. poll for the
  engagement-toggle button's `offsetParent !== null` and enabled) before calling `setActions(true)`,
  and log (don't swallow) a failed toggle click.
- Only after confirming the row genuinely can't be opened with correct scoping/timing should this be
  escalated to a product investigation of `RequestFileOpen`.

---

## 4. FAIL `16-inspection/inspector-fresh-on-open` — not an inspector bug; the board transiently renders blank (recurrence of the event-credit-refusal family)

**Symptom:** test B ("pick with panel closed, then open") reports `selection=[]` and the inspector's
document-summary panel shows `Nodes 0 / Edges 0`, even though Nakagin (180 nodes / 179 edges) was the
active document.

**Mechanism:**

- `…/🔍️browser-probe.ts:795-812` (test B): click node `visible[5]`, wait ≤10 s for selection to include
  it — **fails** (`B first click picked=false waited=10230`); retries once at the *same, unrefreshed*
  screen coordinate — **fails again** (`B second click picked=false waited=10194`); then opens the
  Inspection panel regardless. With `selection=[]`, `render()`
  (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:165-170`)
  falls through `selected_section` returning `None` to `summary(envelope, labels)`.
- `summary()` (same file, lines 132-139) reads counts directly off the **live** document:
  `fixture_nodes(&envelope.fixture).len()` / `fixture_edges(&envelope.fixture).len()` — there is no
  separate, stubbed data source. `"Nodes 0 / Edges 0"` can only render if `envelope.fixture` itself had
  zero nodes/edges at that instant — i.e. the panel is being honest about a board that had gone empty,
  not misreading a populated one.
- The step's own screenshot, `probe-2026-09-17T02-14-35-inspector-timing.png`, confirms this directly:
  captured at the end of the step (right after test B), all three windows' canvases are **completely
  blank** — no nodes rendered anywhere — while the title bar still correctly reads "Nakagin Capsule
  Tower". Test A, earlier in the *same* step, had already succeeded (`ok=true`, selection round-tripped
  through the inspector normally), so the document was rendering fine at the start of the step and went
  blank during test B (between `closePanels()` after test A and the panel reopen in test B).
- This exact symptom — an edged document's board rendering empty ("three blank panes") after a
  re-parse/re-sync inside the same session — was already root-caused for the **drag** interaction in
  this same ticket: `…/PUZZLE-2D-END-TO-END/📓️status.md:52` — `parse_fixture_json → clear_scene →
  sync_descriptor` announces every document edge as a fresh `edgeCreate` event; on a 179-edge document
  that's the whole event-credit queue (256 slots) in one shot, and a second parse in the same session
  gets refused (`data-board-fixture-parsed=false`). The fix landed was `sync_descriptor_with(desc,
  announce_new_edges)`, called with `false` **only from the fixture-parse path**; status.md explicitly
  notes *"the JS authoring sync keeps announcing"* — i.e. any OTHER code path that resyncs the
  descriptor (a panel close/reopen forcing a re-render, or the click-handling/hit-test re-sync a second
  select attempt goes through) is not covered by that fix and can still exhaust the same queue. The
  probe's own refusal detector (`__semioBoard2dRefusedFixture`, checked after every step at
  `…/🔍️browser-probe.ts:857-858`) did **not** fire for this step (nothing in the ndjson), consistent
  with a different call site than the one the earlier fix targeted, or a refusal not caught by that
  narrow detector.
- The blankness is transient: `guest-alive` (immediately after, `recovery:[]`) and `drag-node`
  (immediately after that, PASS with a real position change) both succeed, so the board self-recovers
  by the next fixture parse — matching "one session's worth of event credits exhausted, next reparse
  starts fresh."

**Fix:**
- *Product:* extend the `announce_new_edges=false` treatment (or an equivalent event-credit-aware sync)
  to whatever non-fixture-parse code path test B exercises — most likely the descriptor resync that
  runs on panel open/close or on a failed-then-retried pick. Audit every `sync_descriptor` call site for
  the same unconditional edge announcement the fixture-parse path already had fixed.
- *Probe:* this FAIL is correctly detecting a real defect, but its own diagnosis is currently
  mislabeled as an "inspector" problem. Two probe improvements would make future recurrences obvious
  without re-deriving this chain: (a) re-query `nodeScreen(second)` before the retry click instead of
  reusing a possibly-stale coordinate, and (b) assert `data-board-fixture-parsed` (already read
  elsewhere, e.g. the `undo` step) immediately after a failed click, so a blank-canvas episode surfaces
  as its own explicit signal instead of a downstream inspector-count mismatch.

---

## 5. The 2 "FAULTS" — a probe fault-classification artifact, not runtime defects

**What they are:** both are the *same* benign dev-environment boot warning, logged twice, misclassified
as a "fault" by an over-broad regex.

- `FAULT_RE` in `…/🔍️browser-probe.ts:44-45` includes the bare alternative `puzzle2d-` (intended to
  catch `unknown Puzzle 2D action`-style errors, but written as an unanchored substring match).
- At boot (and again once more later in the run), the dev host logs one console warning:
  *"semio dev · 58 staged plugin module(s) are behind their source …"* followed by 58
  `[stale] <plugin>: … run: bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` lines —
  every one of those 58 lines contains the literal substring `puzzle2d-` (from the recipe name
  `activate-puzzle2d-react-dev`), so the *entire multi-line console message* matches `FAULT_RE` and gets
  pushed into `faults[]` as one fault (`…/🔍️browser-probe.ts:49-54,72`). It fires twice in this run
  (`faults.length === 2` at the final summary, and the identical 58-line block appears twice in the
  `## faults` section of the `.md`), giving `FAULTS=2`.
- This is a **dev-activation staleness advisory** (the served plugin bundles are behind their source;
  `bun nx run …:activate-puzzle2d-react-dev` was not re-run before this battery), unrelated to any of
  the four FAILs above — the code that emits it lives in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` (dev-server only, not
  shipped).
- There's already a precedent for excluding a similarly-noisy-but-benign line:
  `…/🔍️browser-probe.ts:70-72` explicitly excludes `"contributions document sources"` from
  `FAULT_RE` for exactly this reason ("quotes the document ops text, which carries words the fault regex
  reads as faults; it is a census, never a fault").

**Fix (probe only):**
- Either re-run `bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` before batteries
  (removes the warning at the source), or narrow `FAULT_RE`'s `puzzle2d-` alternative to something that
  can't match the recipe name — e.g. anchor it to an actual action-id shape
  (`unknown Puzzle 2D action ['"]puzzle2d-`) instead of a bare substring — and/or add the same kind of
  exclusion `"contributions document sources"` already gets, keyed on `"staged plugin module"`.

---

## Summary table

| # | Step | Locus | Kind |
|---|------|-------|------|
| 1 | `8-transform/engagement-move` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10217-10230,10851,10771-10782,10888-10893` | **Product, framework-wide** — engagement-line space-stripping normalizer, wrong for verb+args grammars |
| 2 | `20-history/undo-changes-document` | `…/🎮️commands/✅️commit-slot/🦀️.rs` + `🌿️vcs/🦀️.rs:186,335-347` | **Product** — fill commits per-placement, not per-gesture; blows the 64-edit ledger |
| 3 | `24-import/import-round-trip` | `…/🔍️browser-probe.ts:250-259,632-654` | **Probe** — un-scoped `actionsOpen()` + no readiness wait after a document reload |
| 4 | `16-inspection/inspector-fresh-on-open` | board-host `sync_descriptor` (non-fixture-parse call site); inspector code itself is correct | **Product** — recurrence of the event-credit "blank panes" refusal outside the already-fixed fixture-parse path |
| 5 | 2 FAULTS | `…/🔍️browser-probe.ts:44-45,70-72` | **Probe** — `FAULT_RE`'s bare `puzzle2d-` alternative matches the benign stale-plugin-module boot banner |
