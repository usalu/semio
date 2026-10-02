# 📓️ W3-E2E — Time-travel browser probe (puzzle 2d React, en + de)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W3-E2E. The probe ran against the coordinator's serve at
`http://127.0.0.1:6012/?plugin=puzzle2d`. I did not start any server. Nothing was committed, and no ticket or goal was
opened or closed. I changed no source code; only the probe was edited.

**Status:**
- **Probe:** written, run and calibrated.
- **Flow:** the history-editing flow works end to end in the browser in both locales:
  - drag → row → Edit → preview → Accept → replay → review;
  - Finalize (overwrite and new alternative) → Undo/Redo;
  - fatal path: blocked → withdraw → ready → Exit with zero trace.
- **Missing UI:** the draft editor is not rendered in React (finding 1). Steps 4, 7 and 8 therefore send their draft
  inputs through a probe-only bypass, which is labelled on every use.
- **Other open findings:**
  - the alternatives switcher (expected-pending, W2-A);
  - the step-5 reload route (finding 7);
  - an uncaught error in the hub rejection notice (finding 9).

## 1. Verdicts per step

The evidence runs are all in `🗑️generated/e2e/`:
- en: `probe-2026-09-30T15-22-33` (final probe);
- de: `probe-2026-09-30T15-22-33` for steps 1–6, 8 and 9;
- de step 7: `probe-2026-09-30T15-14-58` and `probe-2026-09-30T15-06-47`. In the 15-22-33 run the dev serve reloaded
  the page 7 times during de step 7; the probe logs "main frame navigated #2…#8", so those verdicts measure a fresh
  document.

Each run writes `.md` (step lines, verdicts, notes, timeline, console digest), `.ndjson` (records and inventories) and
png/json per step.

| Step | en | de | What passed | What failed and why |
|---|---|---|---|---|
| 1 Boot | PASS 4/4 | PASS 4/4 | `data-board-fixture-parsed=true`, 180 nodes and 179 edges; the history panel is in the UI locale (COMMANDS / BEFEHLE) | — |
| 2 Select 2 + drag (+80,+40) | PASS 9/9 | PASS 9/9 | Shift-click selects 2 nodes. Both move by one common offset (80.00003, 40), and no other node moves. Exactly ONE new document row appears, "Drag 2 items by (80, 40)" / "2 Elemente um (80; 40) ziehen". It expands to its `drag-selection` mutation. The downstream drag of C is its own row, "Drag 1 item by (-60, 30)". | — |
| 3 Edit → band / preview / editor | FAIL 8/11 | FAIL 8/11 | Edit (row action) opens the band: `role=status`, `aria-live=polite`, stage `editing`, "Editing a mutation · Editing: Drag 2 items by (80, 40)", with accept/discard/exit and chords Alt+Enter, Alt+Backspace, Alt+Shift+Backspace. All 3 windows show the indicator. Preview = pre-drag state + draft (design §4). The downstream drag is not applied, and its row reads "Not applied while editing" / "Beim Bearbeiten nicht angewendet". | The editor section is not rendered: no dx/dy steppers, no targets list (**finding 1**). |
| 4 dx→120, Accept, review | FAIL 5/6 | FAIL 5/6 | With dx=120 (bypass), the preview moves A and B to pre + (120, 40) with C unapplied. Accept (en button, de chord Alt+Enter) gives reviewing / `ready` in about 100 ms: "Ready to finalize · Accepted changes: 1", Replay again disabled ("Not possible right now"), Finalize… enabled. The head is at +120 with C re-applied. | Only the missing dx control (**finding 1**). The arrow-key snap-step check is skipped because there is no control. A `replaying` frame never renders for a 2-mutation replay (note, not a FAIL). |
| 5 Finalize → Overwrite, reload | FAIL 9/11 | FAIL 9/11 | The dialog reads "Finish editing history" / "Verlaufsbearbeitung abschließen". Overwrite is `data-destructive=true`, `data-tone=danger`. "New alternative" is the submit, and the name field defaults to "Edited history" / "Bearbeiteter Verlauf". The band shows `choosing`. Overwrite closes the session, adds the row "History edited — overwrite: 1 mutation" / "Verlauf bearbeitet — überschrieben: 1 Mutation" and relabels the drag row "Drag 2 items by (120, 40)". The head keeps +120. The folder attach keeps the head and writes the archive. | `positions-persist-after-reload` and `overwrite-row-survives-the-reload` fail (**findings 2 and 7**). |
| 6 Undo / Redo | PASS 5/5 | PASS 5/5 | The ⌘Z chord takes the finalize back to +80 in about 0.8 s, and ⌘⇧Z redoes it to +120. Rows "History edit undone — …" and "History edit redone — …" appear (de: "Verlaufsbearbeitung rückgängig / wiederhergestellt — überschrieben: 1 Mutation"). | — |
| 7 Second session → new alternative | FAIL 7/10 | FAIL 7/10 | The second Edit opens. With dy+30 (bypass), the preview is at (120, 70). Review `ready`. The dialog name field takes the name, and "New alternative" closes the session. The row "History edited — alternative <name>: 1 mutation" / "Verlauf bearbeitet — Alternative <name>: 1 Mutation" appears, and the head is on the edited alternative at (120, 70). | The editor controls (**finding 1**, ×2) and `alternative-switcher-present` (**finding 3**, expected-pending W2-A). |
| 8 Fatal path | FAIL 13/14 | FAIL 13/14 | See the step-8 detail below this table. | Only `withdraw-control-reachable` (**finding 1**). |
| 9 Console | FAIL 1/2 | FAIL 1/2 | No hard guest faults. | Uncaught page errors raised in the step-5 reload check: en shows finding 9; de shows finding 9 in earlier runs, and in 15-22-33 an HMR-time error (**finding 4**). |

Step 8 detail (fatal path, both locales):
- mod+d clones node "node-1", and dragging the clone gives the row "Drag 1 item by (70, 70)".
- Edit on `Create node "node-1"` previews the clone at its pre-drag position.
- Withdraw (bypass) previews the document without the clone.
- Accept gives review `blocked`: "Errors in later mutations block finalizing · Worst outcome: Error" (de: "Fehler in späteren Mutationen verhindern den Abschluss").
- Finalize is disabled with the title "Blocked: resolve the pending change or the errors first".
- The failing row reads "Error: Target missing" / "Fehler: Ziel fehlt".
- Edit on the failing drag, then Withdraw, then Accept gives `ready` with "Accepted changes: 2".
- Exit closes the session with 0 position drift and 0 new rows.

Totals, final run 15-22-33: PASS 117, FAIL 27, uncaught 3, hard 0. Of the 27 FAILs:
- 14 are finding 1;
- 2 are finding 3;
- 4 are the reload part (findings 2 and 7);
- 2 are step 9 (findings 9 and 4);
- 5 are de step 7, measured under dev reloads (finding 4).

Earlier runs 15-06-47 (de) and 15-11-13 (en) each had 62 PASS, 10 FAIL and 1 uncaught (finding 9).

## 2. How to run

`bun T/🔍️time-travel-probe.ts --port=6012 [--only=1,…,9] [--locales=en,de] [--chords=de] [--explore]`

- Headless Chromium runs one fresh context per locale, and each context has exactly one tab. The UI locale comes from `navigator.language` (en-US / de-DE).
- `--chords=<locales>` drives Accept and Exit through the band chords in those locales (default: de), and through the band buttons elsewhere.
- The step-5 reload check runs after step 8 and before step 9, so steps 6–8 keep the edited document. It is attributed to step 5.
- `--explore` boots, opens the history panel and the sync card, dumps the DOM and exits.
- Each step ends with a `STEP <locale>/<n> PASS|FAIL (k/n)` line. A main-frame navigation the probe did not cause is recorded as the note `page-reloaded-under-the-step`.
- A full two-locale run takes about 8–10 min under peer load.

## 3. Findings (root causes with evidence)

### Finding 1 — React drops the time-travel band section and the draft editor (blocks steps 3, 4, 7, 8)

While the session is `editing`, the rows carry the overlay and the history-lane actions are frozen, so `time_travel` is
`Some`. Even so, the panel DOM holds only ACTIONS and COMMANDS: there is no `…/framework.history.timeTravel*` and no
`…/framework.history.editor*` id. The inventories are in the `-s3-failure.json` dumps.

Cause:
- `⏪️time-travel/🦀️.rs` `time_travel_band_section` and `time_travel_editor_section` build their roots with
  `section(label)`, which is `Component::Container(role: Section)` (`🖱️ui/🧬️contract/🏗️builder/🦀️.rs:995`).
- `ui_history_panel` pushes these as children of `tree()`.
- React `🗣️Interpreter/🟦️.tsx` `TreeView` (about line 2177) keeps only
  `record.children.filter(r => r.component.type === "treeSection")`, so every other child is dropped silently.

Consequences:
- There are no dx/dy steppers, targets list, Accept/Discard/Withdraw editor buttons, Next problem, Replay again (panel copy)
  or Clear buttons in React.
- There is no UI path to change a draft or to withdraw, and Accept of an unchanged draft closes the session (the correct
  law: `accept` → unchanged → `resume(Inactive)`).

Fix: owner W2-A (Rust body) or W2-B (React interpreter). Either build both sections as `tree_section` with tree-item rows
(one vocabulary on both hosts), or render non-treeSection tree children. When it lands, the probe's bypass must be
deleted (section 4).

### Finding 2 — nothing persists across a plain page reload on the playground route (by design)

After a reload, A is back at its Nakagin origin, even the plain drags are gone, and the only row is Set Active Example.
`resolveDocumentOpeningBindings` (`🏛️ShellHost/🧭️opening/🟦️.ts:46`) returns `[]` without a `spaceId`, and
`?plugin=puzzle2d` opens no space. There is therefore no hub or folder binding, and a reload re-runs Set Active Example.
The probe records this as the note `bare-reload-reruns-the-example`. W2-A's in-process law
`a_history_edit_is_its_own_row_locally_remotely_and_after_reload` covers the fold.

### Finding 3 — no React control switches alternatives (expected-pending, W2-A)

The Rust history body renders no alternatives list. `switchAlternative` declares no `alternativeId` argument, so neither
the palette nor a dialog can dispatch it. Only the tutorial driver does (`🏛️ShellHost/🟦️.tsx`
`applyTutorialSliceToShell`). The DOM scan for `alternative|switchAlternative` controls found none. The probe already
drives a `select`/`combobox` that lists the alternative's name, when one exists, and asserts the original positions.

### Finding 4 — dev-serve page reloads under the probe (environment)

The Vite reconnects and supervisor recycles come from peer edit bursts. The serve pid and document stamp change, and the
page is replaced. Steps that run across such a reload measure a fresh document, as de step 7 in 15-22-33 did (main-frame
navigations #2–#8).

In the same run's de reload window, two page errors were raised:
- `Error: useShellScope called outside a ShellScopeProvider at FrameworkOsShellInner`
  (`🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx:47`);
- one pageerror with an empty message.

These fit a module-identity split while Vite hot-replaces the ShellScope context module during full reloads. That is
unverified, and the error never occurred in a run without dev reloads.

### Finding 5 — Undo/Redo of a finalize works through the chords

Step 6 passes in both locales. My pre-run concern, that a newer panel-toggle shell row with an inverse would capture plain
Undo first (`supersede_undo_target`), did not happen in this flow.

### Finding 6 — Behaviours met on the fatal path (probe-side handling, one open observation)

- **Board, UX note:** while a node whose only handle is free is the sole selection,
  `sole_indirect_handle_hit_idle_selected_node` (board `➕️normal/🦀️.rs:9344`, called from `resolve_hit_world`)
  resolves any press inside it to that handle. The press then starts a link drag instead of a node drag. That is what a
  fresh clone is right after mod+d, so the probe clears the selection before grabbing it.
- **Open observation (W2-A/W2-B):** the band's `data-time-travel-generation` stayed at 3 across Begin-from-Reviewing.
  In one run, 4 of 4 bypass withdraws dispatched the instant the band flipped to `editing` were silently dropped. With an
  800 ms settle, they land. The real editor buttons carry their own generation, but it is worth checking whether the band
  can briefly show a stale generation.

### Finding 7 — the local folder route writes and reads back the archive, but re-attaching does not apply it

The coordinator's decision was to run the reload verdict only on a local binding, without a hub.

- **Route:** sync chip `s-sync-status` → group `ui.utilities.group.sync` → `framework.sync.folder` →
  `framework.sync.folder.path` → Attach. This calls `openSyncTarget` with a `persistedLocalOnly` folder binding. It does
  not re-open by itself after a reload, because `syncBackboneUri` resets per session.
- **Write:** nothing is written until the bound document sends an outbound mutation (`archivePersistence` in
  `bindDocumentBackbone.send`), so the probe makes one edit after attaching. Then
  `PUT /semio-backbone?uri=folder://<T>/🗑️generated/e2e/folder-…&documentId=puzzle-1` returns 200, and
  `.semio/documents.db` gets `document_archive` row `puzzle-1` (133–146 KB).
- **Read-back:** after the reload, re-attaching the same folder issues `GET … documentId=puzzle-1`, which returns 200.
- **Result:** the head stays the fresh example, and the history holds only chrome rows. In the same window the worker
  locally rejects a 68–70-envelope batch, which is finding 9. That rejection is the likely reason the restore does not
  stick; the crash hides the reason string.
- **Where to look:** ShellHost around line 3876, `documentArchiveReplaced` → `loadDocumentArchive` (line 3233) →
  `entry.replacements.replace` / `lane.replace(exact)`, and the worker's local rejection reasons.
- **Verdict:** FAIL, outside the history-editing WPs. The coordinator decides whether to classify it as N/A.

### Finding 8 — the "Warning ·" on "Set Active Example" (open question, answered)

That edit contains a `Replace kind catalogs` / "Artkataloge ersetzen" mutation whose recomputed outcome is
"Warning: No change" (`mutation.no-op`): the example's catalogs equal the existing ones. The row only shows the warning
after outcomes are recomputed, for example after an attach re-open or a replay. The note is
`example-row-warning-after-attach`. It is benign; the example loader could skip the no-op replace.

### Finding 9 — uncaught `JSON.parse` in `hubCommandRejectionNoticeV1` (1 per locale)

The error is `SyntaxError: Unexpected token 'D'|'E'|'F' … at JSON.parse at hubCommandRejectionNoticeV1
(🛠️ShellHelpers/🟦️.tsx) at worker.onmessage`, raised during the step-5 folder re-attach.

`CommandAckOutcome.rejected.messages: readonly number[]` (`💻️os/🟦️.ts:738`) has two producers that disagree:
- The hub ack path (`🏪️store/👷️worker/🟦️.ts` about line 4406) passes `outcome.Rejected.messages`, the bytes of a JSON
  message list.
- Every local rejection passes counters: `[envelopes.length]` or `[count, bytes, limit]`. This covers the read-only
  target, queue overflow, backbone capacity, canonical-pair and the scope-mismatch reject at line 7501.

W2-B's `hubCommandRejectionNoticeV1` (ShellHelpers line 2322, called at ShellHost line 3911) decodes the counters as
UTF-8 JSON: `[68|69|70]` becomes "D"/"E"/"F" and throws. The throw escapes `worker.onmessage`, no notice is shown, and
the reason is lost, because the `console.warn` comes after the parse. A count from 1 to 9 parses as a JSON number and
then throws "invalid messages", so every local rejection crashes this handler.

Fix: owners W2-B and the store worker. Use one encoding (JSON message lists for local rejections too), and never throw
from the event handler.

### Finding 10 — cosmetic and hygiene observations

- The entry-row description shows the raw op text (`op_lines`) next to the label, e.g. "Drag 2 items by (80, 40)
  drag-selection dx=80.00003051757813 …". This is the existing description design.
- Interaction and chrome verbs appear as command rows ("Hover", "Clear Selection", "Toggle Panel", "Apply Board Events").
- About 5.4k `[DEBUG] unit verb=setActiveExample … / advance verb=…` lines per locale are logged at console.error level.
  This is leftover peer instrumentation.
- The hub trusted catalog answers `GET /_semio/hub/trusted-catalog/plugin-modules` → HTTP 500 at every boot and
  periodically.

## 4. Probe-only bypass (delete when finding 1 is fixed)

`bandDispatch(action, args)` walks the React fiber of `[data-semio-time-travel]` up to the `TimeTravelBand` props. It
then calls that band's own `onAction({controllerId, action, args: {generation, …}})`. Every use is emitted as an ndjson
`note`, and the missing control keeps its own FAIL verdict. The uses are:

| Where | Verb | Replaces the missing control |
|---|---|---|
| step 4 | `historyEditInput{path:"/dx", value:120}` | `framework.history.editor.input.dx` stepper |
| step 7 | `historyEditInput{path:"/dy", value:dy+30}` | `framework.history.editor.input.dy` stepper |
| step 8 (twice) | `historyEditWithdraw` | `framework.history.editor.withdraw` |

Step 8's "Next problem" (`framework.history.timeTravel.nextProblem`, also missing) falls back to a real UI path: Edit on
the first row with Error/Fatal text.

## 5. Design readings recorded by the probe

- **Preview in step 3:** the brief said the board should show the pre-drag positions. Design §4 and W2-A's ShowPreview say
  the preview is the state before the target plus the draft, and Begin drafts the original input. So the dragged nodes
  read pre-drag + (80, 40) until the draft changes. The probe asserts the design, and adds a downstream drag (C) that
  proves "downstream not applied" (C at its pre-drag position, the row reads "Not applied while editing").
- **Replay frame:** a 2- or 3-mutation replay completes inside one turn, so the band goes `editing` → `reviewing` with no
  `replaying` frame. An in-page MutationObserver trace sees no intermediate state. Progress frames are throttled to at
  least 100 ms or 5 %.

## 6. Probe-side fixes made while diagnosing

These were fixed in the probe, not in the product:
- CSS `text-transform` makes the COMMANDS label read in upper case.
- The dragged clone was grabbed on a neighbour's or its own sole free handle.
- The windowed Commands list does not materialise every row, so new rows are now "newer than the newest seq", not
  "missing from an earlier read".
- Row entry seqs renumber after re-open, so the drag row is found by its stable mutation id.
- The sync utilities sit behind a grouped toggle.
- The folder archive is written only on an outbound mutation.
- Every `/semio-backbone` request is logged, together with HTTP ≥ 400 responses.

## 7. Files

- **Created:**
  - `T/🔍️time-travel-probe.ts`
  - `T/📓️w3-e2e-report.md`
- **Scratch:** `T/🗑️generated/e2e/`, with probe runs (`probe-<stamp>.md|ndjson|*.png|*.json`), `probe-tsconfig.json` and
  folder-attach folders `folder-<stamp>-<locale>/`. All of it is safe to delete at close. `activate-react*.log` and
  `supervisor.stdout` there belong to the coordinator.

---

# Run 2 — W2-A fixes, real controls only (2026-09-30, about 16:30–16:55)

This run is against a fresh coordinator activation on `:6012`, with W2-A's fixes:
- the band and draft editor are `tree_section`s, with windowed inputs;
- window-transient persistence;
- `[DEBUG]` lines removed and no interaction rows;
- op-lines moved under labelled mutation children;
- the generation is published in the same frame;
- the Alternatives section is present.

**The probe-only bypass is deleted.** Every step now drives the real controls the way a user does:
- The windowed history body is scrolled until the section holds the row.
- Steppers take keyboard input (`fill` + Enter, ArrowUp/ArrowDown).
- Editor and band buttons are pressed as their rows (`framework.history.editor.withdraw.row`,
  `framework.history.timeTravel.nextProblem.row`).
- Alternatives are switched through their Switch row action.

The document is bound to a fresh local folder at boot (`--folder-at=1`), so step 5's reload check measures a document
whose every edit is persisted.

Evidence runs (in `🗑️generated/e2e/`), with no dev reloads under either:
- en: `probe-2026-09-30T16-49-01`, PASS 85, FAIL 3, uncaught 0, hard 0;
- de: `probe-2026-09-30T16-51-11`, PASS 85, FAIL 3, uncaught 0, hard 0.

## R2.1 Verdicts per step (en and de identical)

| Step | en | de | Evidence |
|---|---|---|---|
| 1 Boot + local folder | PASS 5/5 | PASS 5/5 | Fixture parsed; 180 nodes. The panel is in the UI locale. The folder attach (sync chip → `ui.utilities.group.sync` → `framework.sync.folder` → path → Attach) keeps the document. |
| 2 Select 2 + drag | PASS 9/9 | PASS 9/9 | Exactly one row, and its label is now clean: "Drag 2 items by (80, 40)" / "2 Elemente um (80; 40) ziehen", with no op text. |
| 3 Edit → band / preview / editor | PASS 11/11 | PASS 11/11 | The band is `role=status`, polite, in stage `editing`. The preview is before + draft, and downstream is not applied. `framework.history.editor.input.dx` and `.dy` are stepper inputs with value "80.00"/"40.00" and `step="1"` (the grid-factor snap), with plus/minus. `…input.targets` is a reference list with 2 chips and "Use selection". |
| 4 dx→120, Accept | PASS 7/7 | PASS 7/7 | The stepper takes the keyboard `fill("120")`, then Enter, and the preview moves to +120. ArrowUp gives 121 and ArrowDown gives 120: the snap step is 1. Accept (en button, de chord Alt+Enter) gives `ready`, and the head is +120 with the downstream drag re-applied. |
| 5 Finalize → Overwrite, reload | FAIL 9/11 | FAIL 9/11 | Everything passes (dialog, destructive Overwrite, overwrite row, head) up to the reload. After reload and re-attach, the head is restored with 0 drift over every node, the clone included. The document rows are lost (**R2-2**). |
| 6 Undo / Redo | PASS 5/5 | PASS 5/5 | ⌘Z → +80, ⌘⇧Z → +120, with the undone/redone rows. |
| 7 New alternatives + switching | FAIL 22/23 | FAIL 22/23 | See the step-7 detail below this table. The only FAIL is `trunk-listed-after-new-alternative`, expected-pending (W1-G). |
| 8 Fatal path | PASS 15/15 | PASS 15/15 | Withdraw goes through the real row. Review `blocked`, Finalize disabled ("Blockiert: zuerst die offene Änderung oder die Fehler auflösen"), and the failing row reads "Fehler: Ziel fehlt". Next problem (`framework.history.timeTravel.nextProblem.row`) opens the failing drag; Withdraw then Accept gives `ready`. Exit leaves 0 drift and 0 new rows. |
| 9 Console | PASS 2/2 | PASS 2/2 | 0 uncaught, 0 hard. Findings 9 and 10's `[DEBUG]` spam are gone. |

Step 7 detail:
- Session 1 changes dy to +30 through the stepper and reaches `ready`. The name field takes the name, and "New alternative"
  adds the row "History edited — alternative <name>: 1 mutation".
- The Alternatives section lists `framework.history.alternative.<id>` = "<name> Current · Branched by client-…"
  ("Aktuell · Abgezweigt von …" in de).
- The trunk is not listed (expected-pending, W1-G). Session 2 therefore changes dy to +60 and creates a second
  alternative, and both are then listed.
- Switching to the first alternative shows A at +120,+70; switching to the second shows +120,+100.

## R2.2 Resolved since Run 1

- **Finding 1 (editor not rendered):** fixed.
- **Finding 3 (no alternatives switcher):** fixed for listed alternatives. Trunk listing is pending (W1-G).
- **Finding 6's open observation:** the band generation now moves on Begin-from-Reviewing (14 → 15).
- **Finding 9 (uncaught `JSON.parse`):** fixed. The local reject now arrives as a warning notice.
- **Finding 10:** the `[DEBUG]` spam and interaction rows are gone, and op text no longer appears in row labels.

## R2.3 New and open findings (routing per coordinator)

- **R2-1, layout (→ W2-A).** The History body orders ACTIONS → COMMANDS → ALTERNATIVES → HISTORY EDITING (band) → the
  editor section. With a real history, the band and editor sit below the whole Commands list. The editor's inputs are
  windowed, so the dx/dy rows are not in the DOM until the user scrolls down. In one run only the Targets row was
  materialised (`probe-2026-09-30T16-43-17-en-s7-no-editor-2.png|.json`). Design §7 says the body leads with the session
  band. The probe now scrolls the section into view, as a user must.
- **R2-2, history lost on the folder reload (→ W2-B).** After a reload and a re-attach of the same local folder, the head
  is fully restored, but only the newest one or two edits come back as rows:
  - Before: Set Active Example, both drags, "History edited — overwrite", "… undone", "… redone", both
    "History edited — alternative …", Duplicate Selection, "Drag 1 item by (70, 70)".
  - After: "Drag 1 item by (70, 70)", or also a row whose label is raw op text (`create-node node { id=node-1 … }`,
    having lost its "Duplicate Selection" description).
  - The supersede rows and the alternatives therefore do not survive.
  - Evidence: `probe-2026-09-30T16-49-01` / `16-51-11` verdicts `document-rows-survive-the-reload`, and
    `folder-…/.semio/documents.db`.
- **R2-4, related to R2-2 (→ W2-B / store worker).** While the document is folder-bound, every local edit batch is
  refused by the backbone worker: `[backbone-worker] refused a local batch puzzle-1 local.backbone-scope-mismatch
  {envelopes: 1|2|3}`, plus the shell warning `[os-shell] a command batch was refused …`. That is 12 per run, one per
  edit, from the first drag onward. The archive still lands, because `archivePersistence` writes on every outbound send,
  so the state persists while the worker's log never accepts an edit. This is the most likely root of R2-2.
- **Trunk not listed (→ W1-G).** After finalize-as-new-alternative, only the new alternative(s) are listed. Switching back
  to the trunk cannot be tested yet.
- **R2-5, `hostEvent` undeclared (→ W2-D / puzzle 2d).** Three times per run during step 2's gestures, the console shows
  `semio: app "s.puzzle.puzzle2d@1/*#editor" dropped action "hostEvent" dispatched from window kind "2d-overview": no
  window kind declares it` / `input #N hostEvent refused: undeclared-action`. The gestures still land, but the host event
  is dropped.
- **R2-6, check-in at New-alternative submit (→ W2-B, low).** At the moment of the step-7 "New alternative" submit, the
  shell logs `input #48 commitCheckpoint refused: dispatch-failed — timeTravel.frozen`, once per run. The probe never
  presses Commit Checkpoint or Check In, and `commitCheckpoint` is dispatched only by the shell's check-in path
  (`dispatchCheckpoint`, ShellHost around line 10955). Something routes the dialog submit, or its Enter, to check-in.
  The refusal is harmless, but the dispatch should not happen.
- **R2-3, environment.** Peers' Vite full reloads hit two earlier Run 2 attempts (4 to 7 main-frame navigations during
  step 7). The probe records `page-reloaded-under-the-step`, and those runs were discarded.

## R2.4 Re-probe list for the coordinator

After re-activation, re-run `bun T/🔍️time-travel-probe.ts --port=6012 --locales=en,de`; it needs about 5 min per locale.
The fixes each item needs:

| Fix | Owner | Verdicts it should turn green |
|---|---|---|
| Section order: band and editor lead the body (R2-1) | W2-A | none fail today; check that `…-no-editor*` dumps stop appearing |
| Folder-bound edits accepted, and history persisted and reloaded (R2-2, R2-4) | W2-B / store worker | step 5 `document-rows-survive-the-reload`, `overwrite-row-survives-the-reload`; the step 9 digest loses the scope-mismatch lines |
| Trunk listed after New alternative | W1-G | step 7 `trunk-listed-after-new-alternative`, then `switching-to-the-trunk-shows-original-positions` and `switching-back-shows-the-edited-positions` |
| `hostEvent` declared (R2-5) | W2-D | the step 9 digest loses the `hostEvent` lines |
| No check-in on the finalize submit (R2-6) | W2-B | the step 9 digest loses `commitCheckpoint refused` |

## R2.5 Probe changes in Run 2

- The bypass `bandDispatch` and all its uses are deleted.
- `pressAuthored` presses `<id>` or `<id>.row` (the row activation), and reads `aria-disabled` on rows.
- `revealHistory(section)` scrolls the section into view before reading or pressing, because the body is windowed.
- Step 1 attaches the local folder (`--folder-at=1`; `5` keeps Run 1's late attach).
- The reload check compares every pre-reload document row, not only the overwrite row.
- Step 7 is rebuilt around `editDragAsAlternative`, `readAlternatives` and `switchAlternative`: trunk ↔ alternative when
  the trunk is listed, else a second alternative with switching between the two.
- Step 8 adds a `next-problem-control-reachable` verdict.

---

# Plan — `--renderer=wgpu` (puzzle 2d wgpu shell, launch entry `🛠️dev🧩️puzzle◻️2d🧊️wgpu🌐️wasm`, port 6112)

**Status.** Implemented in `T/🔍️time-travel-probe.ts` (region `🔖️Wgpu`, plus a renderer branch in every helper the steps
call). Strict `tsc` is clean (exit 0). It has not run yet, because 6112 is not served (curl 000).
- A React regression run after the refactor passed: `--only=1,2,3 --locales=en` scored PASS 26, FAIL 0, including the new
  `presence-roster-shows-no-editing-peer`.
- Run it with `bun T/🔍️time-travel-probe.ts --renderer=wgpu --port=6112 --locales=en,de` (default chords: de).
- Start with `--explore`. It dumps the mirror keys (at boot, with History open, and after the sync card), the chrome hits,
  the window ids and the `dumpBoard2d` answer into `probe-wgpu-<stamp>-<locale>-s1-explore.json`, so the first real run can
  calibrate keys.

## W.1 How the wgpu shell reaches the DOM (what a user or an assistive technology can drive)

- **ARIA mirror** `#semio-wgpu-accessibility` (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`):
  - It has one element per projected node, in reading order, with `data-node-key` (the node key or chrome control id),
    `data-window`, `role`, `aria-label`, `aria-describedby` (the description), `aria-expanded|pressed|selected|disabled`,
    `aria-valuenow`, `aria-keyshortcuts` and `aria-live`/`aria-busy`.
  - A `click` on an actionable element is forwarded as `accessibility-activate`, which is the AT activation path.
  - An `input` on a mirrored input is forwarded as `accessibility-value`.
  - `focus`/`blur` become `accessibility-focus`/`accessibility-blur`.
  - It refreshes at most every 400 ms.
- **Chrome projection** (`🐚️Shell/🎯️targets/🧊️wgpu` `chrome_accessibility_nodes`): every chrome hit target with a control id
  appears in the mirror, plus these status nodes:
  - `shell.time-travel.status` (polite `status`, or a `progressbar` with done/total while replaying; `aria-busy` while
    replaying or finalizing);
  - `s-presence-peers`;
  - `s-hub-connection`.
  While a modal dialog is open, **only the dialog's nodes** are projected.
- **Chrome hit registry**: `semioWgpuIntrospection.dumpChrome()` gives every pointer target with its page rect, and the
  dispatched-action ledger when `SEMIO_RUNTIME_DIAGNOSTICS=1` is set. The probe sets that in `localStorage` before load.
- **Keyboard** (`🎮️input-wire`): keys are admitted only when their target is the canvas or a mirror element.
  - Mirror inputs keep native editing.
  - A mirror button swallows Enter and Space.
  - So the probe focuses the largest canvas before any chord (`prepareChord`).
- **Board**: the Board2d surface projects no accessibility nodes, and no dump carries its positions, camera or selection
  (see W.3 P1).

## W.2 Mapping (same steps, same verdict names)

| Probe need | React | wgpu |
|---|---|---|
| Boot | 3 `[data-slot=window]` and canvases | `semioWgpuIntrospection` attached, `dumpStructure().windowIds` ⊇ three `2d-*`, the mirror holds nodes |
| Board positions, camera, selection, pane rect | Board2dHost `data-board-*` | `dumpBoard2d` (P1) |
| Select, drag, clone, wheel | pointer and keyboard on the pane | the same pointer on the canvas; keys after `prepareChord` |
| Panel tabs | `[data-slot=panel-tab-button]` | `shell.panel.tab.<anchor>.<tabId>`, open when `aria-pressed`/`selected` |
| History rows | `…/framework.history.entry.<seq>`, `…mutation.<id>` | the same keys in the mirror; the text is label + description |
| Expand a row | disclosure chevron | activation, else focus + ArrowRight |
| Row action (Edit, Switch) | button in the row | `<rowKey>::row-action::<n>` named by its label |
| Editor rows (dx/dy, targets, Withdraw, Next problem) | `…input.dx` stepper, `….withdraw.row` | the same keys; a stepper is a `spinbutton`; values go in by `fill` (→ `accessibility-value`); ArrowUp/ArrowDown step natively |
| Band | `[data-semio-time-travel]` + controls | `shell.time-travel.status` (stage and review read from the announced message, en/de); controls `ui.timeTravel.accept|discard|exit`, `shell.time-travel.{cancel-replay,rerun,finalize,back}` |
| Window indicator | `[data-semio-time-travel-indicator]` | `framework.window.<id>.timeTravel.indicator` (`note`) |
| Finalize dialog | `ui.dialog.choice.overwrite`, `ui.dialog.submit`, `input#name` | `shell.dialog.<id>` with `.choice.overwrite`, `.confirm`, `.cancel`, `.field.name`. Destructiveness is told by the choice description on both renderers. |
| Sync-card folder attach | `s-sync-status` → `ui.utilities.group.sync` → `framework.sync.folder` → path → Attach | the same keys, with the attach button `framework.sync.attach` |
| Alternatives, switching | `framework.history.alternative.<id>` + Switch | the same keys + row action |
| Presence roster (⏪ badge) | `#s-presence-peers` | `s-presence-peers` status node |
| Rejection notice by code | `[data-notice-code]` (observed for the page's life) | not observable (P2) |

Two verdicts differ by renderer, and each case is recorded as a note:
- On wgpu, `dialog-offers-destructive-overwrite` checks the choice's label and description, because the mirror carries no
  tone.
- On wgpu, `band-reads-choosing-while-the-dialog-is-open` becomes a note, because a modal dialog hides the band from the
  projection.

## W.3 Prerequisites and scope (for routing)

- **P1 — `dumpBoard2d` (owner: wgpu renderer / W2-C).** It is required for steps 2–8. Without it the probe records
  `wgpu-board-introspection-present` FAIL plus `blocked-by-missing-board-introspection` for steps 2–8, instead of noise.
  The contract, the wgpu twin of React's Board2dHost `data-board-*` vitals:
  - signature: `semioWgpuIntrospection.dumpBoard2d(windowId?) → {surfaces:[{surfaceId, windowId, rect:[x,y,w,h], camera:{x,y,zoom}, positions:{nodeId:[x,y]}, selection:[id], nodes, edges, handles, parsed}]}`;
  - `rect` is in page CSS px;
  - it is read-only, built from the Board2d scene the frame worker already holds (`fixture_json`, `camera_json`,
    `selection_json`, the surface rect);
  - it goes beside `dumpMeshStats` in the `🔬️IntrospectionExports` region, with a probe kind `board2d` and a shim entry in
    `🚀️browser-boot` `attachIntrospectionBindings`.
- **P2 — notice code (owner: wgpu shell / W2-C).** The transient notice is painted only: its message and
  `ShellTransientNotice.code` never reach the mirror (only `shell.notice.close` is a hit). Project it as a polite status
  node, with key `shell.notice`, the message as label and the code as description. Until then, step 9's
  `rejection-notices-carry-their-code` FAILs on wgpu whenever the console shows a refused batch. That happens today on
  every edit of the folder-bound document (R2-4).
- **P3 — a second peer (⏪ badge, "is editing" notes).** This is not cheap here. Presence travels through a hub (presence
  bit 13), and a local folder binding carries no presence. The probe asserts the one-tab case on both renderers: no ⏪ and
  no "is editing" in `s-presence-peers`. The positive case stays covered by W2-C's
  `the_roster_badges_and_announces_an_editing_peer_and_notes_its_rows` and the shared `🧫️time-travel-peers` corpus. A
  browser test needs a hub-backed space serve.
- **Trunk switch.** Step 7 already tests trunk ↔ alternative when the trunk is listed (W1-G), and otherwise two
  alternatives. This is the same on both renderers.

## W.4 What the first wgpu run should report

- With P1 present, every step 1–9 in en and de, with the same verdict names as React. The Run-2 expectations carry over,
  except for the renderer notes above.
- The mirror keys may differ from the React ids. `mirrorFind` matches exact keys, then a `/`- or `.`-delimited suffix, and
  never a bare substring. A missing key is logged as absent, so calibration from the `--explore` dump is a one-line change
  per key.

---

# Run 3 — Session 2 (2026-10-01)

Successor of W3-E2E (Session 2). Repair-first check (rule 21): the probe was complete (2198 lines, `🔖️Main` closed, strict
`tsc` clean at the start); S2-W2C had only adjusted the blocking-rule regex. No half-finished edit.

## R3.1 Probe changes (Phase 1)

`T/🔍️time-travel-probe.ts` keeps its structure (regions, the renderer branch in every helper, the same verdict names for
steps 1–9). Strict typecheck: `bunx tsc --noEmit -p T/🗑️generated/e2e/probe-tsconfig.json` → 0 errors (the probe is in
`--listFiles`, check time 5 s).

**Run order per locale:** 1 → 8, 10, 11, 12, 13, the reload check (attributed to step 5), 14, 9. Steps 10, 12, 13 and 14 need
no earlier step (they only exclude A/B/C); step 11 needs step 2.

| Step | Gap | What it drives (real controls only) | Verdicts |
|---|---|---|---|
| 3 (added) | G13 | After Edit: focus and the reveal, read before any probe scroll | `focus-moves-to-the-editor`, `history-panel-reveals-on-session-start` |
| 4 (added) | G13 | After Accept | `focus-moves-to-the-band-on-review` |
| 5 (added) | G13 | Finalize prompt open | `focus-moves-into-the-finalize-prompt` |
| 9 (added) | — | Console | `console-is-debug-free` (any line containing `[DEBUG] ` fails) |
| 10 | G6 | Select 2 nodes → palette `mod+p` → "Rotate…" → command panel form (angle default 90) → Execute → Edit the `rotate-selection` row → angle control; then the same for "Scale…" (factor default 1.5) | `g6-rotate-command-turns-the-selection-by-90-degrees`, `g6-rotate-is-one-row-labelled-from-its-leaf`, `g6-angle-is-a-dial`, `g6-angle-dial-shows-ticks-at-0-90-180-degrees` (React; a note on wgpu), `g6-angle-reads-degrees`, `g6-arrow-keys-step-the-angle-by-one-degree` (preview at 91° then 90°), `g6-page-up-goes-to-the-next-detent` (preview at 180°), `g6-exit-leaves-the-rotation-as-it-was`, `g6-scale-command-spreads-the-selection-by-1.5`, `g6-scale-is-one-row-labelled-from-its-leaf`, `g6-factor-slider-has-ticks-at-its-snaps`, `g6-factor-ticks-sit-on-a-log-axis` (React: tick 1 at 50 % of 0.1…10), `g6-out-of-bounds-factor-is-refused-naming-the-bound` (typed `-5` → "Must be greater than 0" / "Muss größer als 0 sein"), `g6-refusal-keeps-the-draft`, `g6-exit-leaves-the-scaling-as-it-was` |
| 11 | keep editing | Edit the A/B drag, dy+10 → Accept → `ready` → Edit the downstream C drag from the review, dx+15 → Accept → Finalize → Overwrite | `keep-editing-first-review-is-ready`, `keep-editing-begins-another-mutation-from-a-ready-review`, `keep-editing-second-review-is-ready-with-two-accepted-drafts` ("Accepted changes: 2"), `keep-editing-review-shows-both-drafts`, `keep-editing-finalize-overwrites-both-in-one-row` ("… overwrite: 2 mutations"), `keep-editing-head-carries-both-edits` |
| 12 | G3 | mod+d clone + drag → Edit the upstream `create-node` → Withdraw → Accept → Next problem → select two remaining nodes on the board → Use selection → Accept → Finalize → Overwrite | `g3-the-review-is-blocked`, `g3-the-downstream-drag-reads-error-target-missing`, `g3-next-problem-opens-the-failing-drag`, `g3-board-selection-works-while-editing`, `g3-use-selection-replaces-the-targets`, `g3-chips-show-labels-not-raw-ids`, `g3-board-highlights-the-referenced-nodes` (React `data-board-highlighted-ids-json`, wgpu `dumpBoard2d.highlighted`), `g3-preview-moves-the-new-targets`, `g3-editing-the-targets-makes-the-review-ready`, `g3-finalize-overwrite-closes-the-session`, `g3-head-equals-the-expectation` (head = pre-session head without the clone, the two nodes moved by the drag offset) |
| 13 | G4 | S2-W2D scenario `warning-from-an-upstream-edit` on the live example: select P → Inspection panel row `puzzle2d-play-inspector.node.locked` (lock) → again (unlock) → drag P+Q → Edit the unlock → Withdraw → Accept → Finalize → Overwrite | `g4-lock-in-the-inspector-is-a-history-row`, `g4-unlock-in-the-inspector-is-a-history-row`, `g4-the-review-is-ready-not-blocked`, `g4-the-band-names-the-warning`, `g4-the-drag-row-reads-warning-partially-applied`, `g4-the-warning-is-marked-new-since-this-edit` (§16.5 `introduced`), `g4-the-replay-skips-the-locked-member`, `g4-the-warning-stays-visible-after-finalize`, `g4-the-head-keeps-the-locked-member-in-place`; after the folder reload: `warning-row-survives-the-reload` (filed under 13) |
| reload (5) | G5 | Folder reload + reconnect, as Run 2, after step 13 | strict `document-rows-survive-the-reload` (the descriptionless exemption is gone), `no-row-reads-op-text-after-reload`, `alternatives-survive-the-reload`, `main-line-listed-after-the-reload`, `current-alternative-restored-after-the-reload`; on a build with no folder route (browser wgpu, S2-W2C decision) one plain reload and `reload-restores-the-edited-document` |
| 14 | G13 | A fresh 375 × 812 context (`isMobile`, `hasTouch`): tap + drag one node → Edit → dx by keyboard → Accept by touch → Finalize by touch → Tab/Shift+Tab in the prompt → Overwrite by touch | `mobile-boots-at-375-px`, `mobile-no-horizontal-page-scroll`, `mobile-tap-selects-and-a-drag-moves-the-node`, `mobile-edit-opens-the-band`, `mobile-band-inside-the-viewport`, `mobile-band-controls-are-touch-sized` (≥ 24 × 24 CSS px), `mobile-editor-input-reachable-by-keyboard`, `mobile-accept-by-touch-reviews-ready`, `mobile-finalize-prompt-inside-the-viewport`, `mobile-finalize-prompt-is-keyboard-reachable`, `mobile-overwrite-by-touch-closes-the-session` |

**Helper changes:**
- **History paging.** The Commands list now holds about 35 rows per locale. Reading only its start and end windows would miss
  the middle, so `pageHistory(down|up)` pages through the windowed body (0.8 viewport per page, stops at the edge or after
  two pages with nothing new). `allHistoryRows`, `documentEditIds` and the new `findMutationRow` (newest first, expanding the
  collapsed rows of each page) use it. `beginDragEdit` now finds the drag by its stable mutation id through
  `findMutationRow`; it no longer expands rows that the window has unmounted.
- **New helpers.** `beginEditOf(mutationKey)` (also Begin from a review), `newMutation(seq, matches)`, `selectNodes`,
  `runPaletteCommand`, `readNumberControl` (dial, ticks with their axis %, `aria-value*`, refusal), `typeSliderText`
  (double-click the readout, type, Enter), `pressUseSelection`, `focusRead`, `editorRevealed`, and the phone helpers
  `chromeBox`, `bandControlBoxes`, `tapBand`, `tapOverwrite` and `tabsStayInThePrompt`.
- **Reach.** `openTab` / `closePanels` reach the merged mobile panel through `ui.mobilePanel.toggle`, and `waitForBoot`
  takes a window count (1 at phone width).
- **wgpu.** `MirrorNode` gains `focused`, `valueMin`, `valueText`; `Board2dSurface`/`Vitals` gain `highlighted`; the contract text
  for `dumpBoard2d` names `highlighted:[id]`. `--explore` now also dumps the Inspection panel and the palette (query "Rotate"
  / "Drehen") so the first wgpu run can calibrate those keys.

**Readings recorded in the verdicts (not product failures):**
- `history-panel-reveals-on-session-start`: in one tab a session can only begin from the History panel's own Edit row action.
  The verdict therefore checks that the panel is open with the editor's first input in view and uncovered, without a probe
  scroll. A closed-panel begin needs the agent/MCP gateway, which this serve does not run.
- Rotate and scale have no board gesture that both shells expose: the gumball has move and rotate flags only, and wgpu
  publishes no gumball geometry. Step 10 therefore uses the palette command, which yields one `rotateSelection` /
  `scaleSelection` ToolTransaction (tool `#editor#rotateSelection`) like any gesture.
- On wgpu the slider ticks are painted only: the mirror projects `<input type=range>` with `aria-value*`. The tick verdicts
  are React's; wgpu records a note with a screenshot, and the detent law is proven by PageUp.

**Expected FAILs from reading the current source (predictions, not run results):**

| Verdict(s) | Source fact | Owner |
|---|---|---|
| `g6-angle-is-a-dial`, `…ticks-at-0-90-180-degrees`, `…reads-degrees`, `g6-page-up-goes-to-the-next-detent` | `time_travel_input_row` (`🔌️plugin/⏪️time-travel/🦀️.rs` 🔖️Panel, Dial arm) builds a track slider without `appearance`, `snaps`, `display_factor`/`display_unit` | S2-W1E (S2.4 "time-travel mapping", NOT STARTED in its report) |
| `g6-factor-ticks-sit-on-a-log-axis`, `g6-out-of-bounds-factor-is-refused-naming-the-bound` | the Slider arm ignores `scale: log` and passes no `limits`; React `Slider.commitTyped` then refuses `-5` with a null message (no visible text) | S2-W1E |
| `g3-chips-show-labels-not-raw-ids` | puzzle 2d implements no `ArtifactApp::entity_label`, so the framework default `None` shows the raw id | S2-W2D |
| `g3-board-highlights-the-referenced-nodes` (wgpu only) | `DumpBoard2dSurface` has no `highlighted` field | S2-W2C |
| `reload-restores-the-edited-document` (wgpu only) | the browser wgpu build serves no folder transport (`📓️w2-c-report.md` S2.1) | S2-W2C / coordinator decision |

## R3.2 Phase 2 — runs

Pending: neither 6012 nor 6112 was served when Phase 1 closed (`curl` → 000). The commands, in order:
1. `bun T/🔍️time-travel-probe.ts --port=6012 --locales=en,de`
2. `bun T/🔍️time-travel-probe.ts --renderer=wgpu --port=6112 --explore`
3. `bun T/🔍️time-travel-probe.ts --renderer=wgpu --port=6112 --locales=en,de`

Expect about 12–15 min per locale on React.

---

## Session 3 — 2026-10-02

Successor S3-E2E. Repair-first check (rule 28): the probe `T/🔍️time-travel-probe.ts` (3170 lines, 17:34 Oct 1) is older than
the last report section (17:35 Oct 1), so no half-finished edit was left. Phase A (no serve up): close the N14 probe holes,
give the probe its permanent home (N16), adopt the structural a11y oracle, strict `tsc`, plan Run 4. Phase B (after the
coordinator's "serve up: 6012 / 6112"): Run 4.

### S3.1 Status (kept current)

- 11:55 — Phase A started; the permanent home was created from the ticket probe.
- 12:30 — **Phase A done** (WRITTEN, TYPE-CLEAN, NOT YET RUN: no serve is up). The probe lives at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`. It is routed by `verify time-travel`, the nx
  target `@semio-tech/framework-os-dev:time-travel` and two seed rows. Every N14 hole has a step. The ticket copy
  `T/🔍️time-travel-probe.ts` is deleted, because the permanent module replaces it fully. Run 4 waits for "serve up: 6012 / 6112".

### S3.2 N16 — the permanent home

| Piece | Where | What |
|---|---|---|
| Test module | `🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (3.9 k lines, regions as before plus `🔖️AriaOracle`, `🔖️ReplayArm`, `🔖️LongHistory`, `🔖️Peers`) | exports `runTimeTravelCli(repoRoot, defaultOutDir, segments)`; there is no module-level `process.argv` and no top-level await. Playwright loads the way the other dev harnesses load it (`PLAYWRIGHT_MODULE_SPECIFIER` and `ensureParityPlaywrightBrowsersPath`, so the repo cache's chromium-1234 matches the root playwright 1.62.1). The serve comes from `ensureDevServe({variant: "puzzle2d", renderer})`: it reuses a serve that answers, or starts one and stops it after the run (zero-touch for a dev). An acceptance record `time-travel` is written through `withAcceptanceRecord`/`publishAcceptanceCheckResult`: pass, fail, or blocked when no serve is reachable; en + de summary; evidence = report `.md` + `.ndjson`. Ctrl-C stops after the running step and still writes the report. |
| Route | `🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts` | `verify time-travel …` → `runTimeTravelCli(repoRoot, 🧑‍💻dev/🤖️generated/🧪️time-travel, …)`, the same pattern as `verify two-human`. The package `📜️script.ts` was not touched: it already routes `verify` to this router. |
| nx target | `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` → `time-travel` | `bun ./📜️script.ts verify time-travel`, `cache: false`, inputs `default` + external `playwright` (the parity-journey shape), `forwardAllArgs`. |
| Launch rows | `.vscode/🧩️launch.seed.jsonc` (the seed; `.vscode/launch.json` is generated from it by `🔌️plugin/📇️registry/🚀️launch/🟦️.ts`) | `⚖️gate⏪️time-travel⚛️react` (411.2781, `--serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de`) and `⚖️gate⏪️time-travel🧊️wgpu` (411.2782, :6112, `--renderer wgpu`). Group `4_gate`, placed right after `⚖️gate⏱️interaction-latency⚛️react` (411.278). **Coordinator:** regenerate `.vscode/launch.json` (I did not hand-edit generated output). |
| Output | default `🧑‍💻dev/🤖️generated/🧪️time-travel/` (gitignored); my runs use `--out T/🗑️generated/s3-e2e/run4` | `probe-<renderer>-<stamp>.md/.ndjson/-<locale>-s<step>-<tag>.png/json`; the folder-attach folders sit under the same `--out` |

CLI: `verify time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,…,reload]
[--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--explore]`. Flags take the codebase's `--flag value` form, not the
old `--flag=value` form, so the coordinator's Phase B lines translate to `--serve http://127.0.0.1:6012/ --locales en,de
--chords en,de`. `--chords` now defaults to `en,de`. `--only` selects steps; each selection boots its own document. The token
`reload` selects the reload check, which is no longer tied to step 5, so step 13's warning-after-reload can run without
steps 2–7.

### S3.3 N14 — probe holes closed (Phase A)

| Hole | Step | Real controls driven | New verdicts |
|---|---|---|---|
| en chords never driven | all | `--chords` defaults to `en,de`. `pressBand(control, way)`: step 11's second Accept takes `otherWay()` (chord ↔ button), so both are driven per locale. Discard is now driven too (it never was before). | `keep-editing-discard-returns-to-the-review-keeping-the-accepted-draft`; the `via` in every Accept verdict |
| hard-minimum refusal on a stepper | 8 (`hardMinimumRefusal`) | In the open `create-node` session: type `-1` into the `/index` stepper (schema `minimum: 0`), Enter, then Home | `stepper-below-its-hard-minimum-is-refused-naming-the-bound` ("Must be at least 0" / "Muss mindestens 0 sein", `role=alert` + `aria-invalid`), `stepper-refusal-keeps-the-draft` (nothing dispatched, preview unchanged), `stepper-home-reaches-the-hard-minimum` (design §18) |
| rotate/scale edits (not only reads) | 10 | rotate: after the zero-trace Exit, Edit again → PageUp to 180° → Accept → Finalize overwrite. scale: after the refused `-5`, type `2` in the readout → Accept → Finalize overwrite | `g6-rotate-edit-finalizes-the-new-angle`, `g6-rotate-row-reads-the-edited-angle` ("Rotate 2 items by 180°"), `g6-typed-factor-previews-the-new-spread`, `g6-scale-edit-finalizes-the-new-factor`, `g6-scale-row-reads-the-edited-factor` (replaces `g6-exit-leaves-the-scaling-as-it-was`) |
| keep editing from a clean `ready` | 11 | as in Run 3, plus Discard and a re-Begin in the middle | see the first row |
| Use selection on `drag-selection.targets` | 12 | unchanged (G3 loop) | unchanged |
| warning flow + reload | 13 + `reload` | unchanged; the reload check can now run with 13 alone (`--only 1,13,reload,9`) | `warning-row-survives-the-reload` |
| tablet | 15 (`deviceJourney(TABLET)`) | Fresh 768 × 1024 touch context (`📱️device`: 768 is a tablet). Same journey as the phone (14), which is now the same function | `tablet-*` twins of the 12 `mobile-*` verdicts |
| long history ≥ 200, progress, Cancel, Replay again | 16 | Fresh document. Its example load is one transaction (`Puzzle2dActiveExampleWork`: one create-node per node, one connect-handles per edge; Nakagin has 180 nodes and 179 edges, so 360+ mutations). The step edits the transaction's first listed mutation with a real change: manifest id (text) > a node's x (+10) > Withdraw. Accept with the page armed: in the first frame that shows `replaying`, Cancel is pressed (in-page, the way a person reacts to the progress bar). Then Replay again with the page armed to press Edit on another mutation row while it replays. Then Exit. If no frame ever shows a replay, the history grows by one palette Set Active Example per round (up to 2) and the step retries; per-round timings go to the ndjson record `long-history`. | `g9-fresh-document-boots`, `g9-every-mutation-of-the-long-transaction-is-reachable` (acceptance item 1 / N1), `g9-the-replay-shows-progress-over-the-long-history` (total ≥ `--long-history`, default 200), `g9-cancel-stops-the-replay` (review `needsReplay`, "Replay cancelled"), `g9-replay-again-is-offered-after-cancel`, `g9-edit-during-replay-is-refused` (the Edit control disabled, or the `timeTravel.illegal` notice; never an `editing` frame after the press), `g9-the-replay-completes-the-review`, `g9-exit-leaves-zero-trace` |
| Edit-during-replay refusal | 16 | see the row above | `g9-edit-during-replay-is-refused` |
| second peer | 17 | Two fresh contexts, both attached to ONE fresh local folder. A drags one node, and the archive is written. B attaches and reads A's document. A begins a history edit. B drags another node. A receives it over the dev serve's `backbone.folder` change stream as a base move. A changes dx, Accepts and Finalizes. B converges. | `g10-first-peer-boots`, `…-attaches-the-shared-folder`, `…-edit-is-written-to-the-folder`, `g10-second-peer-boots`, `g10-second-peer-opens-the-shared-document`, `g10-first-peer-begins-a-history-edit`, `g10-second-peer-drags-another-node`, `g10-a-remote-edit-arrives-while-editing`, `g10-the-session-survives-the-base-move`, `g10-the-remote-edit-stays-downstream-and-unapplied-while-editing`, `g10-accept-replays-the-remote-edit-too`, `g10-finalize-closes-the-session`, `g10-the-second-peer-sees-the-finalized-edit` |

**Second peer — what the dev serve can and cannot do without a hub (read from source, 12:10):**

- **What syncs.** The dev serve does sync two browser contexts on the same document over a shared local folder.
  - The Vite plugin `semioBackboneVitePlugin` (`🧑‍💻dev/🔌️vite-plugins/🟦️.ts`) serves `GET|PUT /semio-backbone?uri=folder://…&documentId=…`.
  - Each client also gets the `backbone.folder` route on the dev stream mux. It is a debounced (200 ms) `fs.watch` of the folder's `.semio`, emitting `changed`.
  - On `changed`, the store worker (`🏪️store/👷️worker/🟦️.ts` `pollFolderOnce`) reads the archive and emits `documentArchiveReplaced` unless the bytes equal its own echo.
- **What that means for history.** The folder route is whole-archive, last-writer-wins replacement, not a merge. It carries the other peer's history and edits, so the base moves while editing, and step 17 asserts exactly that.
- **What does not sync: presence.** The roster's ⏪ badge and the "is editing" notes travel only in hub presence frames (presence bit 13). The worker sets `presenceAuthority` only from a hub socket (`🏪️store/👷️worker/🟦️.ts` around line 4140, `ServerFrame::Presence`), and a folder binding carries no presence.
- **Consequence for the probe.** The positive presence case needs a hub-backed space serve (`🚀️local-hub` plus serves joined with `?space=`), which only the coordinator can start. Step 17 records it as the note `g10-presence-travels-only-through-a-hub`. The one-tab negative stays as `presence-roster-shows-no-editing-peer` (step 3).

**Also changed in the probe:**

- **wgpu notices.** The step-9 wgpu `rejection-notices-carry-their-code` verdict is now observable. W2-C projects the transient notice as the polite `shell.notice` mirror node, described by its code (`🧯️wgpu-transient-notice`). `installNoticeTrace` observes it on both renderers.
- **Notice codes across pages.** Notice codes are harvested before every reload and before every fresh context closes (`harvestNotices`), so step 9 compares every code the locale showed.
- **`dumpBoard2d`.** Its `camera` may be null, and the probe now tolerates that.
- **Docstring emojis.** All 162 docstrings in the module start with a unique emoji (73 inherited duplicates were renamed).

### S3.4 a11y oracle (coordinator decision 3)

The structural oracle is the third-party pair the React laws already use, `aria-query` 5.3.0 and `dom-accessibility-api` 0.5.16.
Both are installed at the repo root, transitively through `@testing-library/dom`; no `axe-core`.

- **How the probe loads it.** It bundles the pair once per run with `Bun.build`: a virtual entry module, resolved from the test
  directory to the root `node_modules`, output as one 116 KB IIFE. It then evaluates the bundle into the live page through CDP,
  so no page CSP applies.
- **What it checks.** `ariaFindings` applies the W2-B law's rules in the live DOM:
  - an unknown `aria-*` attribute;
  - an attribute the role does not support;
  - a dangling `labelledby` or `describedby`;
  - a control without an accessible name — widened to tree items, tabs, menu items, options, checkboxes, switches and links;
  - a duplicate id.
- **Verdicts.**
  - `aria-band-editor-and-history-have-no-structural-findings` (step 3);
  - `aria-finalize-prompt-has-no-structural-findings` (step 5);
  - `<device>-aria-band-and-history-have-no-structural-findings` and `<device>-aria-finalize-prompt-has-no-structural-findings` (steps 14 and 15).
- **Roots.** React: `[data-semio-time-travel]`, the History panel and `[role=dialog]`. wgpu: the whole ARIA mirror, which shows
  only the prompt while one is open.
- **Self-check.** I proved the bundling path in Chromium on a static page first (12:05): names were computed, and an
  unsupported `aria-checked` on `role=link` was reported.

### S3.5 Phase A verification (commands and counts)

| Command | Result |
|---|---|
| `bunx tsc --noEmit -p T/🗑️generated/s3-e2e/probe-tsconfig.json` (strict; files: the module and, in the second run, the router plus a canary) | Run 1 (module only): exit 0, **0 errors**, the module confirmed in the 1221-file program (`--listFilesOnly`). Run 2 (module + router + canary `const canary: number = runTimeTravelCli`): 3 errors = the canary (expected, which proves the graph is checked) + 2 in peer files the router imports, none in mine (below). |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel` | `clean=true errors=0 warnings=0` |
| Run 3 (module only, after the last edits, 12:40) | exit 0, **0 errors** |
| `bun -e 'await import(<module>); await import(<router>)'` | both load: `[runTimeTravelCli]`, `[VerifyScript]` (7 s) |
| `Bun.resolveSync` from the test directory | `dom-accessibility-api`, `aria-query` and `playwright` (1.62.1, chromium-1234 present in the repo cache) resolve to the repo root |
| `Bun.JSONC.parse(.vscode/🧩️launch.seed.jsonc)` | valid; the 2 new rows with orders 411.2781 and 411.2782 (the 88 duplicate names are pre-existing placeholders) |
| `project.json` JSON parse | valid; `targets.time-travel.options.command = bun ./📜️script.ts verify time-travel` |

Peer type errors reached through `✅️verification/🟦️.ts` (not mine; bun strips types, so `verify` still runs):
- `📇️directory/🧪️testkit/📡️client-probe/🟦️.ts:144` TS2741: the `MutationEnvelope` literal lacks `line`. It is imported via two-human.
- `🔌️plugin/🏗️build/📥️installation/🟦️.ts:51` TS2345: `string` → `InstallationDirectoryV1`.

### S3.6 Run 4 plan

**Constraint.** A Bash call ends at 10 min, and I may not detach. A full two-locale run takes about 50–60 min per renderer
(steps 1–13 ≈ 12–15 min per locale, plus 14–17 ≈ 15 min). So I run Run 4 in batches of ≤ 9 min, each in the foreground:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript && \
bun ./📜️script.ts verify time-travel --serve <url> --renderer <r> --locales <l> --chords en,de --only <batch> \
  --out <T>/🗑️generated/s3-e2e/run4 > <T>/🗑️generated/s3-e2e/run4-<r>-<l>-<batch>.txt 2>&1; echo exit=$?
```

Before every batch I check that the port answers with `curl`, so `ensureDevServe` only ever reuses a serve.

| Batch | `--only` | Covers |
|---|---|---|
| A | `1,2,3,4,5,6,7,9` | boot + folder, drag, Edit/preview/editor/ARIA, stepper + Accept, Finalize + prompt ARIA, Undo/Redo, alternatives |
| B | `1,2,3,4,5,reload,9` | the G5 folder reload of the overwrite and of every row |
| C | `1,8,10,9` | fatal path + hard-minimum stepper, G6 dial/log slider + rotate/scale edits |
| D | `1,2,11,12,9` | keep editing (Discard, the other way), G3 Use selection |
| E | `1,13,reload,9` | G4 warning + warning after reload |
| F | `1,14,15,9` | phone + tablet |
| G | `1,16,9` | long history: progress, Cancel, Replay again, Edit refused while replaying |
| H | `1,17,9` | two peers over one folder |

Order:
1. React :6012 — batches A–H in `en`, then in `de` (the de pass drives the same chords).
2. wgpu :6112 — `--explore --locales en`, calibrate the mirror keys (a one-line change per key), then batches A–H in `en` and `de`.

Every FAIL is routed with this table shape: verdict → evidence → owner (S3-W2A runtime/history body, S3-W2B React, S3-W2C wgpu
shell, S3-W2D puzzle board, S3-W1E controls). The table goes to `main` and into S3.7.

For a single unbatched run, the coordinator can start it detached from the main session (≈ 1 h per renderer):

```
bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react \
  --locales en,de --chords en,de --out <T>/🗑️generated/s3-e2e/run4-full
```

Predictions from reading the current source (to compare against Run 4):

| Verdict | Source fact | Expected | Owner if red |
|---|---|---|---|
| `g9-every-mutation-of-the-long-transaction-is-reachable` | `HISTORY_PANEL_MUTATION_ROWS = 8` (`🔌️plugin/🦀️.rs` ~12383), no "more" row (N1) | FAIL | S3-W2A |
| `g9-edit-during-replay-is-refused` | `TimeTravelSession::begin_refusal` now disables the Edit action while replaying, choosing or finalizing (`🔌️plugin/🦀️.rs` ~12433, N15 fixed in source) | PASS via `disabled` | S3-W2A |
| `g9-the-replay-shows-progress…`, `g9-cancel-stops-the-replay` | progress frames at ≥ 5 % or ≥ 100 ms of a replay stepped in 4 ms turns; a 360-mutation replay may finish inside a few frames | unknown, which is why there are growth rounds | S3-W2A (G9) / S3-W1G (N17) |
| `g6-angle-*`, `g6-factor-*` | `time_travel_slider` carries appearance Dial, snaps, display factor, precision and limits | PASS | S3-W1E |
| `g3-chips-show-labels-not-raw-ids` | puzzle 2d implements `entity_label` (`✏️editor/🦀️.rs` ~5072) | PASS | S3-W2D |
| `g10-*` base move while editing | never tested anywhere: does the archive replacement reach an editing store as `BaseMoved`, or does the frozen artifact lane refuse it? | unknown | S3-W2A / S3-W2B |
| step 9 `rejection-notices-carry-their-code` (wgpu) | `shell.notice` projection exists | PASS when no unannounced refusal | S3-W2C |

### S3.7 Run 4 — results

Pending: no serve was up when Phase A closed.
