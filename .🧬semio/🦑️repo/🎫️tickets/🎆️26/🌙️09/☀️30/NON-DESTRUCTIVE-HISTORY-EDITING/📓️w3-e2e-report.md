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

---

## Session 4 — 2026-10-04

Successor S4-E2E (Opus). Repair-first check (rule 34): the probe `🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (3896 lines, mtime
10-03 11:04) differs from HEAD only by a peer's wording sweep of the temporary-log tag in step 9's docstrings and regex
(5 lines, compile-atomic, kept). No half-finished edit. Phase A' (no serve up): bring the probe to the current runtime
(N1 paged windows, N2, N15, `HistoryPatch.reprojection`, notices, `editCount`, L4, §19.1, stepped load), a goal-coverage
table, strict `tsc`. Phase B on "SERVE UP": Run 4 batches A–H, React then wgpu, en + de.

### S4.1 Status (kept current)

- 02:09 — Phase A' started; reading the runtime (history body, React band, wgpu mirror) for the exact attributes.
- 02:20 — runtime read (history body `ui_history_panel` PLG ≈12455, TT 🔖️Panel/🔖️MutationPages, React band, React Tree
  window attributes, wgpu mirror/row actions, kernel notices). Baseline strict `tsc` of the unchanged module: exit 0, 0 errors.
  Editing the probe now (findings that change verdicts are listed in S4.2).
- 02:50 — **Phase A' done** (WRITTEN, TYPE-CLEAN, TAXONOMY-CLEAN, IMPORTS; NOT YET RUN — no serve is up). Module 4.2 k lines,
  178 docstrings, every leading emoji unique. "E2E PROBE READY" sent to `main`.

- 06:55 — Phase A'' (coordinator: S4-UI contract while activation waits for the describe wave after the channel bump 20→21).
- 07:08 — **Phase A'' done** (WRITTEN, TYPE-CLEAN, TAXONOMY-CLEAN, IMPORTS; NOT YET RUN). Module 4.66 k lines, 181 docstrings,
  every leading emoji unique. "E2E PROBE READY" sent to `main` again. Section S4.7.

### S4.2 Probe changes for the current runtime (Phase A')

Read from the disk on 10-04 (line numbers drift): history body `ui_history_panel` (`🔌️plugin/🦀️.rs` ≈12455), its mutation row
`history_panel_mutation_row` (≈12670, `RowAction::disabled_because`), TT `🔖️Panel` (band, `time_travel_reprojection_section`,
editor sections, `time_travel_input_row`), TT `🔖️MutationPages` (`history_row_mutation_extent`, window path), React
`TimeTravelBand` (`🛠️ShellHelpers/⏪️time-travel`), React Tree windows (`🖱️ui/🧱️elements/🌳️Tree` `treeWindowDomAttributes`:
`data-tree-window-key|path|total|offset|length`, rows `data-tree-window-row`, spacers `data-tree-window-spacer=leading|trailing`),
React row actions (Tree `TreeRowActionButton`: `aria-disabled` + `aria-describedby` → an `sr-only` reason; no tooltip),
wgpu mirror (`🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility` row actions `<row>::row-action::<i>` named `"<label>: <row>"`,
description = the disabled reason), kernel notices `🎠️kernel/🧫️fixtures/🧫️history-notices`.

| Area | What changed in the probe | Verdicts (new or changed) |
|---|---|---|
| N1 windows (W1E-6) | `treeWindow(key)` reads React's window attributes; `historyCommandsTotal()`; `reachWindowEnd(key)` scrolls a window until `offset + length = total`; `pageHistory` stops when every row the Commands `total` counts was seen (wgpu: no total in the mirror → paging as before). Step 16 drops the stale "+N more" detection (no such row exists; a row is a tree window over ALL its mutations) and derives reachability from the row window's `total`, then opens the LAST mutation for editing. wgpu pages the body keeping, in reading order, the rows between the entry and the next entry (survives the entry scrolling off). | `n1-commands-window-total-grows-by-one-per-edit` (2), `g9-every-mutation-of-the-long-transaction-is-reachable` (16, rewritten), `n1-the-last-mutation-of-the-long-transaction-opens-for-editing` (16), `n1-one-drag-adds-exactly-one-row-to-the-window-total` (19) |
| N2 editor | Reference row is now a tree window: `….targets.row`, first row `….targets.useSelection` (button + `.row`), chips `….targets.chip.<i>.row` (label = entity label; the button `….chip.<i>` reads "Remove <label>"). Editor reading and `pressUseSelection` rewritten. List inputs: `….node.handles.row` names "Items: n", `….add`, item rows `….handles.<i>.row` with `….remove`. `revealWindowRow` scrolls the inputs window slice by slice; `pressAuthored(…, false)` presses deep rows without scrolling back to the section. Long option rows (> 32 options) do not exist in any puzzle 2d leaf → note. | `editor-shows-the-targets-reference-list` (3, now label-based: the old check for raw ids contradicted `g3-chips-show-labels-not-raw-ids`), `n2-duplicate-adds-a-create-node-row`, `n2-edit-opens-the-create-node-session`, `n2-add-item-drafts-one-more-list-item`, `n2-remove-item-drops-the-drafted-item`, `n2-exit-leaves-zero-trace` (19) |
| N15 Edit refused | `rowActionState(row, Edit)`: disabled, AT reason (`aria-describedby` / mirror description) and the reason a sighted person sees on hover (React tooltip or `title`; wgpu mirror description). The replay arm records the reason of the Edit it presses. Edit regex accepts wgpu's `"Edit: <row>"`. | `n15-edit-is-refused-while-a-changed-draft-is-open-naming-why` + `n15-the-refusal-reason-is-visible` (4), `n15-edit-is-refused-while-choosing-naming-why` (5, React; wgpu note: modal projects only the prompt), `n15-edit-is-disabled-while-replaying-naming-why` (16) |
| N17 reprojection | `reprojection()` reads `framework.history.reprojection` (title, status, kind by the localized progress line, Cancel/Replay again); `armReprojection(observe | cancel | undo-then-cancel)` acts the frame the status appears (Undo `framework.history.undo.run`, then — after the refusal notice or 1.5 s — Cancel replay); `reprojectionSettled`. | reload check: `stepped-load-shows-its-progress-in-the-history-body` (or note); 17: `g10-the-second-peer-shows-what-it-replays` (or note); 18: see below |
| §20.8 stepped load | New step 18 page B attaches a grown folder archive with History open the moment Attach is pressed (`attachFolder(path, afterPress)`). | `n17-the-stepped-load-shows-its-progress-in-the-history-body`, `n17-a-command-during-the-load-is-refused-naming-the-load` (`document.loading`), `n17-cancel-replay-keeps-the-previous-document` / `n17-attaching-the-folder-loads-the-shared-document` |
| History steps | New step 18 page A: second example load → edit → finalize New alternative → switch to Main line (deferred local step) armed. | `n17-history-grows-by-a-second-example-load`, `n17-a-mutation-of-the-first-example-opens-with-a-draft`, `n17-the-edit-is-kept-as-a-new-alternative`, `n17-a-history-step-shows-its-progress-in-the-history-body`, `n17-a-second-history-step-is-refused-while-one-replays` (`history.replaying`), `n17-cancel-replay-drops-the-history-step-with-zero-trace`, `n17-the-history-step-completes-on-the-main-line` |
| Notices | The notice trace keeps code AND words (React `[data-notice-code]` text, wgpu `shell.notice` label); `shownNotices()`. | 9: `history-lane-notices-speak-the-locale` (when shown), `history-full-notice-names-the-edit-count` (when shown; else note: the paged ledger never fills in a run, `editCount` is visible only as that `{n}`) |
| L4 (§20.13) | New step 19: wheel zoom + middle-button pan + zoom; then a drag, a zoom, ⌘Z, ⌘⇧Z. | `l4-camera-moves-are-never-history-rows` (no new row at all, Commands `total` unchanged, camera moved), `l4-undo-takes-back-the-drag-not-the-camera`, `l4-redo-re-applies-the-drag` |
| §19.1 | Row label = intent leaf, never a support leaf riding with it. | `s19-the-transaction-row-reads-its-intent-leaf` (2), `s19-the-drag-row-reads-its-intent-leaf` (19) |
| Order | `ORDER = 1…8, 10…13, 19, 14…18, 9`; the reload check still runs before 14 (or 9); `STEP_NUMBERS` drives the report. | — |

Readings made while updating (not probe findings, for routing if they show live):
- React tree row actions render the disabled reason only as an `sr-only` description (`🌳️Tree` `TreeRowActionButton`); table row
  actions also put it in `title`. Expect `n15-the-refusal-reason-is-visible` to FAIL on React until W1E-1 lands (owner S4-UI).
- Neither host publishes a window size to assistive technology (no `aria-setsize`/`aria-posinset`/row count); the wgpu mirror
  publishes no window total at all, so wgpu counts are paged (noted per run, not a verdict).
- Observation outside the probe: the history panel's filter select dispatches `withoutMutations`/`onlyMutations` (and the
  handler at `🔌️plugin/🦀️.rs` ≈30379 accepts only those), while the injected `setHistoryCommandFilter` action declares the options
  `withoutOperations`/`onlyOperations` (`🛂️manifest/🦀️.rs` ≈2857): a palette/agent dispatch with a declared option is ignored.
  Owner guess: S4-RUNTIME (manifest + handler vocabulary).

### S4.3 Goal coverage (dev goal, `🧭️plan.md` Session 4) — every sentence → steps → verdicts

Both renderers (`--renderer react|wgpu`), both locales (`--locales en,de`; chords in both), desktop for every step, phone (375 ×
812, step 14) and tablet (768 × 1024, step 15) for the journey edit → input → accept → finalize. wgpu differences are notes, named in
the table.

| # | Goal sentence | Steps | Verdicts | Phone / tablet |
|---|---|---|---|---|
| 1 | every mutation in history is editable | 3, 8, 10, 12, 13, 16, 19 | `edit-opens-the-band-in-editing`, `editing-the-create-node-previews-the-clone-before-its-drag`, `g6-edit-opens-the-rotate-session`, `g6-edit-opens-the-scale-session`, `g4-edit-opens-the-upstream-unlock`, `g9-every-mutation-of-the-long-transaction-is-reachable`, `n1-the-last-mutation-of-the-long-transaction-opens-for-editing`, `n2-edit-opens-the-create-node-session` | `<device>-edit-opens-the-band` |
| 2 | starting an edit enters time-travel mode | 3, 14, 15 | `edit-opens-the-band-in-editing`, `band-is-a-polite-status-region`, `band-names-stage-and-target`, `windows-wear-the-time-travel-indicator`, `history-panel-reveals-on-session-start`, `focus-moves-to-the-editor` | `<device>-band-inside-the-viewport` |
| 3 | the edited mutation is shown with downstream NOT applied | 3, 8, 17 | `preview-is-state-before-target-plus-draft`, `downstream-not-applied-while-editing`, `downstream-row-reads-not-applied`, `g10-the-remote-edit-stays-downstream-and-unapplied-while-editing` | (preview via `<device>-editor-input-reachable-by-keyboard`) |
| 4 | the user accepts or discards the input change | 4, 11, 16 | `accept-replays-then-reviews-ready` (chord or button by `--chords`), `keep-editing-discard-returns-to-the-review-keeping-the-accepted-draft`, `keep-editing-second-review-is-ready-with-two-accepted-drafts` (the other way), `g9-exit-leaves-zero-trace` | `<device>-accept-by-touch-reviews-ready` |
| 5 | every input carries UI metadata (slider, stepper, min, max, snaps …) | 3, 4, 8, 10, 12, 19 | `editor-shows-dx-dy-steppers-with-grid-snap-step`, `arrow-keys-step-dx-by-the-snap-step`, `stepper-below-its-hard-minimum-is-refused-naming-the-bound`, `stepper-home-reaches-the-hard-minimum`, `g6-angle-is-a-dial`, `g6-angle-dial-shows-ticks-at-0-90-180-degrees` (React; wgpu note), `g6-angle-reads-degrees`, `g6-page-up-goes-to-the-next-detent`, `g6-factor-slider-has-ticks-at-its-snaps`, `g6-factor-ticks-sit-on-a-log-axis` (React), `g6-out-of-bounds-factor-is-refused-naming-the-bound`, `editor-shows-the-targets-reference-list`, `n2-add-item-drafts-one-more-list-item`, `n2-remove-item-drops-the-drafted-item` | `<device>-editor-input-reachable-by-keyboard` |
| 6 | accept replays all downstream mutations | 4, 16, 17, 18 | `reviewed-head-at-plus-120-with-downstream-reapplied`, `g9-the-replay-shows-progress-over-the-long-history`, `g9-cancel-stops-the-replay`, `g9-replay-again-is-offered-after-cancel`, `g9-the-replay-completes-the-review`, `g10-accept-replays-the-remote-edit-too`, `n17-*` (history step / load progress + Cancel) | `<device>-accept-by-touch-reviews-ready` |
| 7 | each downstream mutation succeeds / warns / errors | 4, 8, 12, 13 | `review-line-reads-ready`, `failing-row-reads-error-target-missing`, `g3-the-downstream-drag-reads-error-target-missing`, `g4-the-drag-row-reads-warning-partially-applied`, `g4-the-band-names-the-warning` | — |
| 8 | new warnings are visible in history | 13, reload | `g4-the-warning-is-marked-new-since-this-edit`, `g4-the-warning-stays-visible-after-finalize`, `warning-row-survives-the-reload` | — |
| 9 | fatal errors must be edited first, repeated until error-free | 8, 12 | `replay-review-is-blocked`, `finalize-disabled-while-blocked`, `next-problem-control-reachable`, `withdrawing-the-failing-drag-makes-the-review-ready`, `g3-next-problem-opens-the-failing-drag`, `g3-use-selection-replaces-the-targets`, `g3-editing-the-targets-makes-the-review-ready` | — |
| 10 | then the final result; finalize or keep editing other mutations | 4, 11 | `reviewed-head-at-plus-120-with-downstream-reapplied`, `keep-editing-begins-another-mutation-from-a-ready-review`, `keep-editing-finalize-overwrites-both-in-one-row`, `keep-editing-head-carries-both-edits` | — |
| 11 | finalize prompts "new alternative" vs "overwrite" | 5, 7, 18 | `finalize-opens-the-dialog`, `dialog-offers-destructive-overwrite`, `dialog-offers-new-alternative-with-a-name-field`, `overwrite-row-appears`, `alternative-row-appears`, `switching-*`, `trunk-listed-after-new-alternative` path, `n17-the-edit-is-kept-as-a-new-alternative` | `<device>-finalize-prompt-inside-the-viewport`, `<device>-finalize-prompt-is-keyboard-reachable`, `<device>-overwrite-by-touch-closes-the-session` |
| 12 | puzzle 2d: one drag mutation whose selection AND offset stay editable | 2, 3, 4, 7, 12 | `exactly-one-new-history-row`, `row-labelled-from-the-drag-mutation`, `editor-dx-dy-read-the-original-input`, `dx-120-updates-the-preview`, `dy-edit-updates-the-preview`, `g3-use-selection-replaces-the-targets`, `g3-preview-moves-the-new-targets`, `g3-head-equals-the-expectation` | `<device>-tap-selects-and-a-drag-moves-the-node` |
| 13 | tools are state machines yielding mutations inside a transaction | 2, 10, 19 | `exactly-one-new-history-row`, `s19-the-transaction-row-reads-its-intent-leaf`, `g6-rotate-is-one-row-labelled-from-its-leaf`, `g6-scale-is-one-row-labelled-from-its-leaf`, `n1-*-grows-by-one-*`, `s19-the-drag-row-reads-its-intent-leaf` | — |
| 14 | tools are not editable, their mutations are | 3, 8, 19 | (edits always target the yielded leaf) `editor-shows-dx-dy-steppers-*`, `exit-leaves-no-new-rows`, `l4-camera-moves-are-never-history-rows`, `l4-undo-takes-back-the-drag-not-the-camera` | — |
| 15 | clean artifact-agnostic mechanisms | all | the probe drives only framework surfaces (history body ids, band, dialog, notices, windows) — the board is the one app surface; the cross-plugin proof is the G12 harness (S4-AGNOSTIC), not this probe | — |
| 16 | end to end: React + wgpu, en + de, accessible | all, 9 | every verdict per renderer × locale; `aria-band-editor-and-history-have-no-structural-findings`, `aria-finalize-prompt-has-no-structural-findings`, `<device>-aria-*`, `focus-*`, `n15-*` reasons, `<device>-band-controls-are-touch-sized`, `no-uncaught-page-errors`, `no-hard-guest-faults`, `console-is-debug-free`, `rejection-notices-carry-their-code`, `history-lane-notices-speak-the-locale` | steps 14, 15 |

Gap features the coordinator listed, by step: N1 → 2, 16, 19; N2 → 3, 12, 19; N15 → 4, 5, 16; N17 `reprojection` →
16 (session replay), 17 (remote/load), 18 (step + load), reload check (load); `history.replaying` / `document.loading` → 18, 9;
`editCount` → 9 (`history.full`'s `{n}` when shown); L4 → 19; §19.1 → 2, 19; stepped archive load (no `LoadDocument`) → 18, reload.
Not provable live here (recorded as notes): long option rows (no puzzle 2d leaf has > 32 options), positive presence (needs a
hub-backed space serve, coordinator decision), wgpu window totals (the mirror publishes none).

### S4.4 Run 4/5 plan (batches; one foreground call ≤ 9 min each, `curl` = 200 before each)

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript && \
bun ./📜️script.ts verify time-travel --serve <url> --renderer <r> --locales <l> --chords en,de --only <batch> \
  --out <T>/🗑️generated/s4-e2e/run5 > <T>/🗑️generated/s4-e2e/run5-<r>-<l>-<batch>.txt 2>&1; echo exit=$?
```

| Batch | `--only` | Covers |
|---|---|---|
| A | `1,2,3,4,5,6,7,9` | boot + folder, drag (N1 total, §19.1), Edit/preview/editor/ARIA, stepper + N15 Blocked + Accept, Finalize + N15 Choosing + prompt ARIA, Undo/Redo, alternatives |
| B | `1,2,3,4,5,reload,9` | the G5 folder reload (+ stepped-load observation) |
| C | `1,8,10,9` | fatal path + hard minimum, G6 dial/log slider + rotate/scale edits |
| D | `1,2,11,12,9` | keep editing (Discard, the other way), G3 Use selection |
| E | `1,13,reload,9` | G4 warning + warning after reload |
| F | `1,14,15,9` | phone + tablet |
| G | `1,16,9` | long history: N1 window total + last mutation, progress, Cancel, Replay again, N15 while replaying |
| H | `1,17,9` | two peers over one folder (+ the peer's reprojection) |
| I | `1,18,9` | N17 history step (progress, `history.replaying`, Cancel zero trace, completion) + stepped load (progress, `document.loading`, Cancel) |
| J | `1,19,9` | L4 camera/undo, N1 total, §19.1, N2 list add/remove |

Order: React :6012 A–J `en`, then `de`; wgpu :6112 `--explore --locales en` (calibrate mirror keys), then A–J `en`, `de`.

### S4.5 Phase A' verification (commands and counts)

| Command | Result |
|---|---|
| `bunx tsc --noEmit -p T/🗑️generated/s4-e2e/probe-tsconfig.json` (strict; the S3 config) | baseline before edits: exit 0, 0 errors; after edits: run 1 exit 2 (1 redeclared `replayed` in step 17 + 5 follow-ons, mine), fixed → run 2 exit 0, **0 errors**; run 3 exit 0, 0 errors; run 4 (after the last edit, docstring emoji renames) exit 0, **0 errors** |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel` (repo root) | `clean=true errors=0 warnings=0` |
| `bun -e 'await import(<module>); await import(<router ✅️verification/🟦️.ts>)'` | both load: `[ "runTimeTravelCli" ] [ "VerifyScript" ]` (5.9 s) |
| docstring emoji census (python over the module) | 178 docstrings, 0 duplicate leading emojis, 0 without an emoji |

### S4.7 Phase A'' — S4-UI contract (coordinator 06:5x)

Read from the disk at 07:00: React `HistoryReprojectionStatus` + `historyReprojectionControlV1` (`🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`
≈369–408, mounted by ShellHost ≈13410, outside the History panel), kernel copy `historyReprojectionStatus` / `🧫️history-reprojection`,
wgpu `HISTORY_REPROJECTION_STATUS_ID = "shell.history.reprojection"` (a polite status, a progressbar while it replays, named by the
kernel status line — no title prefix, no controls), `DisabledReasonHint` (`🖱️ui/🧱️elements/💡️ChromeControlHint`, delay 400 ms,
`[data-slot=row-action-reason][data-revealed]` portal under the same id as the button's `aria-describedby`), band fault text
(`timeTravelBandTextV1`: unknown codes read `ui.timeTravel.refusal.replayFaulted`), channel handshake
(`📡️spr/🧵️channel` `CHANNEL_MISMATCH_CODE`, channel 21; notice copy in `🎠️kernel/🧫️fixtures/🧫️framework-notices`).

| Item | Probe change | Verdicts |
|---|---|---|
| (a) W1E-6 | The transaction row's window: scrolled to its end, the last mutation row must carry `data-tree-window-row = total − 1`; wgpu pages until the create-node rows (by label) reach the node count and the rows seen reach nodes + edges. | `g9-every-mutation-of-the-long-transaction-is-reachable` (tightened) |
| (b1) reprojection status | `armReprojection` now samples BOTH surfaces each frame: the shell status (React `[data-semio-history-reprojection]` kind, `-phase`, `data-notice-code`, the `role=status` `aria-live=polite` announcement, `-control`; wgpu mirror `shell.history.reprojection` role/live/label) and the body section, plus whether a session band is open; Cancel prefers the shell's `cancelReplay` control. `reprojectionSettled` waits for both to be gone. `reprojectionShellVerdicts(summary, suffix)` judges every shell sample. Step 17 also arms peer A during its session (the base move). Body-only verdict names lost their `-in-the-history-body` suffix. | `reprojection-status-is-announced-outside-the-history-panel<-load|-peer|-during-a-session|-step>` (React: kind + phase + polite status reading "<title>: <text>" in the kernel copy; wgpu: polite/progressbar node named by the kernel line), `reprojection-controls-show-only-while-no-session-is-open<…>` (React; wgpu note: controls live in the body), `a-refused-history-change-names-its-reason-in-words<…>` (when refused), `stepped-load-shows-its-progress`, `n17-a-history-step-shows-its-progress`, `n17-the-stepped-load-shows-its-progress` |
| (b2) reason visible | `rowActionState` runs the reveal sequence on React: hover (900 ms) → revealed; leave → hidden; keyboard focus → revealed; Escape → hidden; press → revealed; blur → hidden; the revealed element's id ∈ the button's `aria-describedby`. Step 5 (behind the modal) reads without the sequence. `tapRevealsReason` taps a refused Edit on phone/tablet. | `n15-the-refusal-reason-is-visible` (step 4, every phase of the sequence), `<device>-a-tap-on-a-refused-edit-tells-its-reason` (steps 14/15, React; wgpu note) |
| (b3) long option rows | > 32 options (`UI_FIXED_LIST_ITEMS`; corrected from 24) → `role=treeitem`, `aria-selected`, chosen `data-selected`. No puzzle 2d leaf offers > 32 options. | note `n2-long-option-rows-not-offered-by-puzzle-2d` (states the contract and the laws that prove it) |
| (b4) band fault words | The band trace keeps, for the page's life, every fault line (React code + words; wgpu any status line carrying a `<scope>.<code>` token), harvested across reloads and fresh pages. | step 9 `band-never-shows-a-raw-fault-code` (known codes in their own words, unknown ones "Replay failed: later mutations could not be checked" / de; the code only in `data-semio-time-travel-fault`) — exercised by step 16's Cancel (`timeTravel.cancelled` → "Replay cancelled") |
| (b5) channel handshake | Notice code `plugin.channel-mismatch` and the host's console line are watched. | step 9 `no-plugin-is-refused-for-a-channel-mismatch` (negative; PASS on a fresh activation), `channel-mismatch-notice-is-localized` (only when one shows); on a failed boot the same negative verdict FAILs with the console lines (coordinator action: re-describe / re-activate) |

Goal coverage (S4.3) additions: sentence 6 (accept replays downstream) and 16 (accessible, both renderers) gain the
`reprojection-status-*` verdicts; sentence 16 gains `n15-the-refusal-reason-is-visible`, `<device>-a-tap-on-a-refused-edit-tells-its-reason`,
`band-never-shows-a-raw-fault-code` and `no-plugin-is-refused-for-a-channel-mismatch`.

Verification (Phase A''): `bunx tsc --noEmit -p T/🗑️generated/s4-e2e/probe-tsconfig.json` → exit 0, **0 errors** (run 5, after the
last edit); `bun ./📜️script.ts verify taxonomy report --scope …/🧪️time-travel` → `clean=true errors=0 warnings=0`; `bun -e` import of
the module and the router → `[ "runTimeTravelCli" ] [ "VerifyScript" ]`; docstring census 181, 0 duplicate emojis; the module holds
no temporary-log tag.

### S4.6 Run 5 — results

Pending: waiting for "SERVE UP: 6012" / "SERVE UP: 6112" from `main` (activation is blocked on a peer refactor; S4-INFRA).
02:52 — `curl` 6012 → 000, 6112 → 000: Phase B not started. Resume S4-E2E (SendMessage) with "SERVE UP: <port>" to run the batches.

---

## Session 5 — 2026-10-05

Successor S5-E2E (Opus). Repair-first check (rule 46): the probe `🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (4660 lines, mtime 10-04 18:27)
has no diff against HEAD; no half-finished edit. Serve: React `http://127.0.0.1:6012/` UP since 00:58 (supervised, puzzle 2d,
build **B0** = the tree as activated at 00:58, before any session-5 change). Task 1 = React Run 5 on B0 (batches A–J, `en` then `de`),
Task 2 = adapt the probe to the session-5 contract (staged under `T/🗑️generated/s5-e2e/`, landed on "… LANDED"), Task 3 = wgpu arm on
"SERVE UP: 6112". Outputs: `T/🗑️generated/s5-e2e/run5/` (probe files) and `T/🗑️generated/s5-e2e/run5-react-<locale>-<batch>.txt`.

### S5.1 Status (kept current)

- 01:01 — read rules 1–54, S4.1–S4.7, W.1–W.4, audit clause 13 and the parity gap list; `curl` 6012 → 200; load average 78–90.
- 01:05 — first live run of the rewritten probe: 14/23, every FAIL a PROBE fault (S5.3 P1). 01:34 — **batch A `en` green 72/0**.
- 01:37 — **batch B `en` green 67/0** — the Run-2 folder-reload failure (R2-2) is gone live.
- 01:42–02:25 — batch C `en` 48/1 (F1). 02:33 — **batch D `en` green 44/0**. 02:40 — fleet cut by the usage limit (rule 62).
- 04:22 — resumed; probe file intact, `tsc` clean. 04:27 — **batch E `en` green 35/0**. 04:28 — batch F 35/2 (F2).
- 04:37 — batch G 19/1 (F3). 04:46 — batch H 18/4 (F4, two peers diverge; first live two-peer run). 04:55 — batch I 19/1 (F5; F6
  candidate). 05:18 — batch J 19/1 (F7). Coordinator's change of plan: `en` only on this build, `de` + wgpu on build B1.
- 05:22 — `REACT RUN 5 DONE (en only)` sent. 05:27 — Task 2 patch LANDED in the probe (S5.6); smoke on "B0 guest + s1 host":
  A 76/0, C 52/2, F 35/5 (new F8). 05:41 — batch H on the `attach` wave: **27/0, all four F4 verdicts PASS**, peers converge.
- 07:26 — "SERVE UP (B1)" (channel 22; React 6012 + wgpu 6112). Run 6 started; 07:45 fleet cut; 09:36 resumed (rules 64/65).
- 09:52 — batch B diagnosed: folder persistence dead on React B1 (F9). 10:11 — the React serve is saved under my lock by a peer
  outside the fleet; the probe now attributes every reload and `batch.sh` re-runs an invalidated batch.
- 10:26 — FIRST wgpu contact: F10 (fresh-profile boot refused), F11 (empty board). 10:39 — wgpu `en` A 7/12.
- 11:17 — React `en` on B1 done except I: PASS 380 / FAIL 23 (S5.8). React is off-limits from "WAVE C GO" until "SERVE UP (B2)".
- 11:34 — the machine rebooted (every helper died); 15:39 activation B2 relaunched; 16:06 "SERVE UP (B2)" for React.
- 16:10–16:18 — Run 7 React `en` A, B, E, H, I: **F9, F4, F6 closed live** (S5.10). 16:18 — "SERVE UP (B2 wgpu)".
- 16:27–16:40 — wgpu `en` on B2: **F10, F11 closed live**; F14 (the first board click kills the renderer) blocks everything
  behind the board; F15–F18 (S5.10). 16:40–16:50 — React `en` C, D, F, G, J, K, L: **goal sentence 9/9**, F12 closed, F3 open.
- 16:50–17:15 — React `de` A–L: 447 PASS / 1 FAIL (F3); F13 and F5 closed live by host saves at 17:01 / 17:11.
- 17:23–17:31 — new step 22 (§22.13 `mutation.precondition-drifted`): batch M 18/0 in `en` and `de`. React Σ 465 / 1 per locale.
- 17:34 — "SERVE UP (B2w)" (waves 10 + 11). 17:36–18:25 — wgpu arm calibrated against the mirror DOM; **first live history edit
  on wgpu**: goal sentence 5 PASS / 4 FAIL / 1 not reached (S5.11); every clause works functionally, each editor input takes
  45–90 s to reach the preview (F22). 18:26 — React panel DOM saved for S5-WGPU (`react-panel-dom/`).
- Next: wgpu `en` A–K and wgpu `de` — NOT RUN: not feasible inside the 10-minute cap until F22 is fixed or the machine is quiet.
- 20:16 — "SERVE UP (B3)" for React (guest rebuilt: §22.32 (a) board tools on the gesture slot, F21 / F16 guest halves). 20:16–20:31
  — Run 8 regression A–M `en`: **472 / 1** (only F3), no B3 regression. 20:35–21:10 — new step 23 "tools are machines"
  (batch N): **27 / 0**, 17 `m32-*` verdicts (S5.12). wgpu not probed (B3w not announced).
- 23:05 — both serves restarted (app quit ~23:00); `:6012` = B3 guest + hot host waves (S5-UI `f3` 21:33, S5-LOAD `f13` + `detach`).
  23:35–23:46 — Run 9: G `en` 22 / 0 (**F3 CLOSED LIVE**), L `de` 20 / 0, N `de` 26 / 1 (the FAIL is a probe baseline fault,
  S5.13). wgpu not probed (B3 wgpu lane not activated).
- 2026-10-06 00:22 — new goal (design §23): the feature for every single editor. 00:27–00:40 — probe batch U, the universal live
  journey (step 24, `--universal`), written and run: **puzzle 2d React 8 / 8, draw React (peer serve :6064) 8 / 8** (S5.14).
- 02:03–02:34 — batch U extended (two rows, every control role, conflict clause, `de`, controls JSON): **puzzle 2d 21 / 0 in
  `en` and `de`, draw 20 / 0 in `en` and `de`**; two product faults on draw's inputs (U1, U2) — S5.15.

### S5.2 Run 5 — results (React :6012, locale `en`; `de` moved to build B1 by the coordinator)

Build identity: guest wasm = **B0** (activated 00:58) for every run. Served TypeScript saved after 00:58 and hot-loaded (each batch
log starts with the list): `⏪️time-travel/🟦️.ts` + `🎠️kernel/🟦️.ts` 01:16 / 02:26 (S5-RUNTIME twins), `🛠️tool-machine/🟦️.ts` 01:25,
`🛂️manifest/🟦️.ts` 02:25, and from 04:56:38 `🏛️ShellHost`, `🐚️Shell`, `⚛️react` (a peer's refactor of the introduction auto-start,
saved under my serve lock; band / Tree / Slider files unchanged since 10-04). Batch A's first green pass (01:10–01:16, steps 1–6)
was pure B0.

| Batch | `--only` | When | PASS | FAIL | uncaught | hard | Failing verdicts → finding | Output (`T/🗑️generated/s5-e2e/`) |
|---|---|---|---|---|---|---|---|---|
| A | `1,2,3,4,5,6,7,9` | 01:33 | **72** | 0 | 0 | 0 | — | `run5-react-en-A.final.txt`, `run5/probe-react-2026-10-04T23-33-12.md` |
| B | `1,2,3,4,5,reload,9` | 01:35 | **67** | 0 | 0 | 0 | — | `run5-react-en-B.txt`, `run5/probe-react-2026-10-04T23-35-21.md` |
| C | `1,8,10,9` | 02:23 | 48 | 1 | 0 | 0 | `g6-out-of-bounds-factor-is-refused-naming-the-bound` → F1 | `run5-react-en-C.final.txt` (step 10 alone: `run5-react-en-C10.txt`, 31/0) |
| D | `1,2,11,12,9` | 02:31 | **44** | 0 | 0 | 0 | — | `run5-react-en-D.final.txt` |
| E | `1,13,reload,9` | 04:26 | **35** | 0 | 0 | 0 | — | `run5-react-en-E.final.txt` |
| F | `1,14,15,9` | 04:28 | 35 | 2 | 0 | 0 | `mobile-band-controls-are-touch-sized`, `tablet-band-controls-are-touch-sized` → F2 | `run5-react-en-F.final.txt` |
| G | `1,16,9` | 04:36 | 19 | 1 | 0 | 0 | `n15-edit-is-disabled-while-replaying-naming-why` → F3 | `run5-react-en-G.final.txt` |
| H | `1,17,9` | 04:43 (twice, identical) | 18 | 4 | 0 | 0 | `g10-a-remote-edit-arrives-while-editing`, `g10-the-remote-edit-stays-downstream-and-unapplied-while-editing`, `g10-accept-replays-the-remote-edit-too`, `g10-the-second-peer-sees-the-finalized-edit` → F4 | `run5-react-en-H.final.txt` |
| I | `1,18,9` | 04:51 (reference run) | 19 | 1 | 0 | 0 | `reprojection-status-is-announced-outside-the-history-panel-load` → F5 (+ F6 candidate) | `run5/probe-react-2026-10-05T02-51-10.md` / `.ndjson`; later runs `run5-react-en-I.run4-reloaded.txt` |
| J | `1,19,9` | 05:17 | 19 | 1 | 0 | 0 | `l4-camera-moves-are-never-history-rows` → F7 | `run5-react-en-J.final.txt` |
| **Σ `en`** | | | **376** | **10** | 0 | 0 | 7 product findings (F1–F7) | |

No uncaught page error, no hard guest fault, no temporary-log line and no channel mismatch in any batch.

What is proven LIVE on React (`en`), by goal sentence (S4.3 numbering):

| # | Goal sentence | Proven by (batch → steps) | Open |
|---|---|---|---|
| 1 | every mutation is editable | A 3, C 8/10, D 12, E 13, G 16 (the 374th mutation of one transaction opens), J 19 | — |
| 2 | editing enters time-travel mode | A 3 (band, indicator ×3, focus, panel reveal), F 14/15 | F2 (control height), O1/O5 (band overlaps other text) |
| 3 | edited mutation shown, downstream not applied | A 3 (preview = before + draft; downstream row "Not applied while editing"), C 8 | two-peer case: F4 |
| 4 | accept or discard | A 4 (chord), D 11 (button, Discard chord), F (touch), G 16 (Exit zero trace) | — |
| 5 | inputs carry UI metadata | A 3/4 (steppers, snap step), C 8 (hard minimum refusal, Home), C 10 (dial ticks/degrees/detents, log slider ticks), D 12 (reference list, Use selection, label chips), J 19 (list add/remove) | F1 (typed slider value lost when the control remounts) |
| 6 | accept replays downstream | A 4, G 16 (progress over 766 mutations, Cancel, Replay again) | F3 (Edit disabled without a reason while replaying) |
| 7 | each downstream mutation ok / warning / error | A 4 (ready), C 8 + D 12 (Error: Target missing), E 13 (Warning: Partially applied) | — |
| 8 | new warnings visible in history | E 13 ("New since this edit", stays after finalize, survives the reload) | — |
| 9 | fatal → edit first, repeat until clean | C 8 (blocked → Next problem → Withdraw → ready), D 12 (blocked → Next problem → Use selection → ready) | — |
| 10 | final result; finalize or keep editing | A 4/5, D 11 (two accepted drafts → one overwrite row "2 mutations") | — |
| 11 | prompt: new alternative vs overwrite | A 5 (Overwrite), A 7 (New alternative, switch both ways), B (reload restores rows, alternatives, current), I 18 | — |
| 12 | puzzle 2d drag: selection AND offset editable | A 2–4, 7 (offset), D 12 (targets through Use selection) | — |
| 13 | tools yield mutations in a transaction | A 2, C 10 (rotate/scale = one row from its leaf), J 19 | F7 (a middle-button pan logs "Apply Board Events") |
| 14 | tools not editable, their mutations are | A 3, C 8, J 19 (undo skips the camera) | — |
| 16 | end to end, accessible | ARIA oracle clean in A 3/5 and F; N15 reasons in A 4/5; phone + tablet journeys in F | wgpu, `de`; multi-user: F4; loads: F5/F6 |

Not provable on this build / history size (notes in the runs): the N17 history STEP (switching alternatives over 766 mutations is
adopted within one refresh in all four I runs → progress, `history.replaying` refusal and Cancel-zero-trace were never judged);
`replay-stage-rendered` for two-mutation replays; presence (needs a hub); long option rows (no puzzle 2d leaf has > 32 options).

### S5.3 Probe faults found by running it (fixed in the probe; none weakens a verdict)

| # | Fault | Evidence | Fix |
|---|---|---|---|
| P1 | DOM ids of windowed GROUP rows: `<namespace>/<windowPath>␟<rowKey>` (U+241F, `TREE_WINDOW_PATH_SEPARATOR`); leaf rows keep `/`. The probe matched `/` only → no document row was ever read (first run 14/23). | `dom/dom-en-2-expanded.json` | every id matcher accepts `/` and `␟` (wgpu also U+001F) |
| P2 | N1 total verdicts (steps 2, 19) expected `total + 1`; the Commands window also counts command rows without mutations (the gesture's "Apply Board Events", the probe's own "Toggle Panel"). | total 8 → 13 with one document row | `total` grows by exactly the rows newer than the read before the drag AND exactly one of them is a document row |
| P3 | "Use selection" is an activatable ROW (`….targets.useSelection.row`). | step 3 inputs list | reader accepts the row |
| P4 | Reads right after the History panel opens raced the windowed body (`total` set, `length` 0; an expanded row's child window materialises later; a body mid-refresh lists nothing). | failure dump `commands: {total: 2, length: 0}` | `openHistory` waits for `length > 0`; `pageHistory` waits until every expanded row shows a child; `allHistoryRows` re-reads an empty body; `findMutationRow` re-expands |
| P5 | N15-while-choosing read the row behind the prompt before the body refreshed (286–446 ms). | A runs 3 vs 4 | ≤ 5 s settle, wait recorded |
| P6 | The create-node `index` input sits below 22 rows of the windowed Inputs section. | step 8 inputs list | `revealWindowRow` first |
| P7 | Rotate / Scale / Set Active Example are window ACTIONS reached through the window's Actions rail on both renderers; the probe drove the palette ("No results found."). | `dom/palette-en.json`, `dom/actions-en.json` | React arm: `framework.window.<id>.engagement.toggle` → `action.<id>` → `framework.window.<id>.action.<id>.execute`; wgpu arm to be calibrated in Task 3 |
| P8 | The editor rows arrive with the History body 0.3–1.5 s after the band reads `editing`. | D run 1 (28/8) | every Begin waits for its editor (`editorArrived`) |
| P9 | The Alternatives section sits at the top of the body and exists only once the document holds a checkpoint (automatic ≈ 20 s after an edit when folder-bound). | E runs 1–2 | read at the top after a settle; the reload check waits ≤ 40 s for the first listing, else a note |
| P10 | Single reads of a state that settles a refresh later (review head, relabelled row after an overwrite, a peer's row, rows after Exit). | E, C, H, J | `waitUntil` with the wait in the detail |
| P11 | On 374 mutations a Cancel pressed in the first rendered frame can arrive after the replay finished (≈ 430 ms). | G run 3 | the history grows by one example load and the round retries |
| P12 | The mutation-row search stopped once all ENTRY rows were seen and never paged through a 374-row window. | G run 1 | `pageHistory(…, deep)` |
| P13 | The body's Cancel replay / Replay again are button ROWS (`framework.history.reprojection.cancelReplay.row`). | I run 1 (`cancel: absent`), run 2 `ids` | selector accepts the row |
| P14 | Step 19 counted the probe's own panel rows as camera rows. | J run 1 | shell chrome rows are told apart by their `monitor` icon; rows are read after each camera gesture |

Added for diagnosis: failure dump with rows + Commands window; `diagnose(tag)`; typing refusals with the field's state; notices
with run-clock time, attributed to their step; slider readout focus/refusal traces; the Edit action's state per frame through a
replay (`how` it is disabled); both peers' final states + the folder PUT/GET timeline (step 17); the reprojection section's ids;
navigations of fresh pages; the shell's restore alert as notice `shell.document-restore` + step-9 verdict
`no-document-restore-fails`; post-cancel verdicts `n17-a-cancelled-load-clears-its-status` and
`n17-the-document-is-editable-after-a-cancelled-load` (written, not yet judged — see F6).
Strict `tsc` after every probe edit: `T/🗑️generated/s5-e2e/tsc-1…29.txt`, last exit 0 / 0 errors. Module: 4925 lines.

### S5.4 Findings for owners (product)

| # | Verdict(s) | Observed | Evidence | Owner (coordinator's routing) |
|---|---|---|---|---|
| F1 | `g6-out-of-bounds-factor-is-refused-naming-the-bound` (C 10) | The slider's typed readout editor does not stay open after step 8 ran in the same document: focus trace t0 on the Inputs section row → +84 ms editor open → +245 ms focus back on the section row, editor closed (3 double-clicks in a row); the typed value or its refusal is lost. Alone (step 10 only) the refusal is correct ("Must be greater than 0", `aria-invalid`). Root cause found by S5-UI: `🌳️Tree` renders a section's rows from a state copy one render behind, a body refresh renumbers node ids and React remounts the control. | `run5-react-en-C.final.txt` line 49 (`opening`), `run5/probe-react-2026-10-04T23-52-37-en-s10-factor-refused.png` (first observation; the final run's screenshots were pruned) | S5-UI (Tree fix staged) |
| F2 | `mobile-…` / `tablet-band-controls-are-touch-sized` (F 14/15) | Band controls Accept / Discard / Exit are 22.39 px high (WCAG 2.5.8: ≥ 24 px). | `run5-react-en-F.final.txt` lines 17, 33 | S5-UI |
| F3 | `n15-edit-is-disabled-while-replaying-naming-why` (G 16) | Band `replaying` at t0 with the row's Edit still enabled; +69 ms the pressed Edit is `disabled` (native attribute + `aria-disabled`) with NO `aria-describedby`; a press in the first frame is refused by the notice `timeTravel.illegal`. Per the coordinator: the host's pending state of the pressed button hides the guest's reason and is not cleared after `{rejected}`. | `run5-react-en-G.final.txt` line 21 (`target` samples) | S5-UI |
| F4 | 4 × `g10-…` (H 17) | Two peers on one folder DIVERGE SILENTLY when one edits history: B's write is fetched by A (GET 200 0.2 s after the PUT) but A's read-back load is REJECTED — shell alert "Document restore failed: actor-document-control.receipt-count", no console line — A's body never lists B's edit, A's Accept replays without it (one `timeTravel.stale` notice), A's Finalize is never written (no PUT after it), B never sees it. Final: A = n1 +80 / n2 original; B = n1 +60 / n2 moved. | `run5-react-en-H.final.txt` (note `g10-final-states-of-both-peers`), `run5/probe-react-2026-10-05T02-43-27-en-s17-g10-peer-a-final.png` | S5-STORE (merge of a fetched pair), S5-LOAD (no PUT after a finalize; retired port), S5-RUNTIME (base move in a session) |
| F5 | `reprojection-status-is-announced-outside-the-history-panel-load` (I 18) | While a folder archive loads, the History body shows "DOCUMENT LOAD / Loading document: 0 of 1" (19.8 s uncancelled; ≥ 180 s after a Cancel) but the shell status `[data-semio-history-reprojection]` is never rendered. | `run5/probe-react-2026-10-05T02-51-10.md`, `…T02-47-13.md` | S5-UI (host status) / S5-LOAD |
| F6 (candidate, one run) | — (`n17-a-cancelled-load-clears-its-status` written after it) | After Cancel replay of a load the previous document stays (PASS) but the body's "Loading document: 0 of 1" section stayed for 180 s. Whether the document is editable afterwards is not judged yet: the two later I runs were reloaded at +196 s / +200 s (below). Also: an uncancelled load of the two-example archive takes 19.8 s as ONE step ("0 of 1"). | `run5/probe-react-2026-10-05T02-51-10.ndjson` (`document-load`) | S5-LOAD / S5-STORE |
| F7 | `l4-camera-moves-are-never-history-rows` (J 19) | Wheel zoom adds no row (both directions); a MIDDLE-BUTTON PAN adds one Commands row "Apply Board Events" (no mutations). | `run5-react-en-J.final.txt` line 11 (`byGesture`) | S5-PUZZLE (board events) |

Observations (no verdict): O1/O5 the band has no opaque surface — over the footer status text at 1600 px and over the merged
panel's rows at 375 px (`run5/probe-react-2026-10-05T02-28-03-en-s14-mobile-editing.png`); O2/O3 transient notices on legal
presses (`timeTravel.invalid-input` once in A; `timeTravel.illegal` + `timeTravel.stale` in D 11, probably a stepper's
commit-on-blur after the Discard chord); O4 an automatic checkpoint fired against a retired document port after a detach
(`commitCheckpoint refused … actor-document-port.retired`); O6 the Alternatives section is absent until the first checkpoint;
O7 a Cancel that arrives after the replay finished answers "Not possible right now".

Environment: page reloads under a batch — `🛠️tool-machine/🟦️.ts` 01:25:55 and an icon + `🔣️shortcodes.json` 02:01:08 saved
without `serve` (rules 57 / 61 followed); in I runs 3 and 4 all pages of the batch reloaded 200 s / 196 s after the run started
with no Vite invalidate line and no bundled non-Rust file saved in that minute (unexplained; the transform guard counts 485
modules since the 04:56:38 host save).

### S5.5 Owed

- React `de` (all batches) and the whole wgpu arm — on build B1, after the Task-2 adaptation (coordinator's plan).
- F6: the two post-cancel verdicts of step 18; the N17 history step needs a longer history than 766 mutations to show a frame.
- wgpu arm of the Actions-rail runner (`runPaletteCommand`), wgpu labels of shell chrome rows, wgpu restore alert.

### S5.6 Task 2 — probe adapted to the session-5 contract (LANDED 05:27, `tsc` clean)

Staged first as an anchored patch script — `T/🗑️generated/s5-e2e/task2-apply.py` (every replacement must match exactly once) —
applied to a staged copy (`probe-staged.ts`, strict `tsc` exit 0: `tsc-staged-1.txt`), then to the module on the coordinator's
"S5-UI s1 ON DISK" relay (`tsc-30.txt` / `tsc-31.txt` exit 0; taxonomy report `clean=true errors=0 warnings=0`; module + router
import; 189 docstrings, unique leading emojis; module 5.1 k lines). Pre-Task-2 copy: `probe-before-task2.ts`. Contract notes:
`task2-contract-notes.md`.

| Contract (owner) | Probe change | Verdicts (new or changed) | First live result |
|---|---|---|---|
| Band controls never `disabled`/`title` (S5-UI) | `reactBand` reads `aria-disabled`, the reason through `aria-describedby` → `[data-slot=row-action-reason]`, `native`, `id` | `refused-band-controls-stay-reachable-and-name-why` (8), `band-controls-carry-their-control-ids` (3; React id = corpus `controlId` = wgpu key) | PASS on s1 |
| Bands in `[data-slot=layout-superfooter] > [data-semio-bottom-bands]`, in flow, opaque (S5-UI) | `bandOverlaps` (real geometry: the band's text on no other text; every band control and text box is the top element at its own points) | `band-overlaps-no-other-text` (3), `…-while-blocked` (8), `<device>-band-overlaps-no-other-text` (14/15), `band-sits-in-the-superfooter-bands` (3) | PASS desktop + phone on s1; tablet → F8 (covered controls; the check was tightened after it) |
| `nextProblem` band control first in a blocked review (S5-UI + guest `session.nextProblem`) | reader + wgpu key `shell.time-travel.next-problem` | `band-offers-next-problem-first-while-blocked` (8) | FAIL on the B0 guest (no `nextProblem {mutationId}` on the wire) — expected until B1 |
| Reveal + scroll to the first blocking row (S5-RUNTIME / host) | `mutationRowRevealed` (no probe scroll) | `the-first-blocking-row-is-revealed-on-a-blocked-review` (8) | FAIL on the B0 guest — expected until B1 |
| "History editing" / "Verlaufsbearbeitung" copy (S5-UI corpus `labels`) | presence regex `bearbeitet .* im Verlauf|bearbeitet den Verlauf`; no other compared string used the old words | — | A 76/0 on s1 |
| Busy / refused row action = `aria-disabled` + `aria-busy` (S5-UI) | `rowActionState` reads `native` + `busy`; N15 verdicts require `native !== true`; N15-while-replaying is judged on every sampled replay frame (aria-disabled with the reason, never the native attribute), the press before the body's refresh by `g9-edit-during-replay-is-refused` | `n15-*` (4, 5, 16) | steps 4/5 PASS on s1; batch G owed on B1 |
| Row actions `[Edit, Withdraw | Restore]`, Edit disabled with its reason for a non-editable mutation (S5-RUNTIME wave C) | COPY `withdraw` / `restore` / `refusalNotEditable` / `refusalReadOnly`; new step 20 (batch K `1,20,9`) | `r22-every-mutation-row-offers-edit-and-withdraw`, `r22-withdraw-from-the-row-opens-a-session-with-a-withdrawn-draft`, `r22-the-review-shows-the-document-without-the-withdrawn-node`, `r22-a-row-with-an-accepted-draft-offers-restore-in-place-of-withdraw`, `r22-withdrawing-the-blocking-mutation-from-its-row-makes-the-review-ready`, `r22-restore-from-the-row-drops-the-accepted-draft`, `r22-exit-leaves-zero-trace` | WRITTEN, TYPE-CLEAN, NOT RUN (needs the B1 guest) |
| Two peers by design §22.25 (S5-STORE / S5-LOAD) | convergence judged, the second peer's drag recorded | `g10-both-peers-converge-after-the-finalize`; note `g10-the-second-peers-drag-after-the-finalize` | PASS on the `attach` wave; the drag survives |
| Restore alert (S5-LOAD) | notice `shell.document-restore` | `no-document-restore-fails` (9) | PASS |
| wgpu mirror (S5-WGPU wave 1) | `MirrorNode` gains `invalid`, `setSize`, `posInSet`, `step`, `min`, `max`, `tone`; refusal = the CONTROL node's description when `aria-invalid`; typed slider values go to `<key>::editor`; band / arm progress from `shell.time-travel.progress`; Commands total = `aria-setsize` of the entry rows (N1 verdicts judge on wgpu too) | wgpu arm of `stepper-*`, `g6-*`, `n1-*`, `g9-*` | WRITTEN, TYPE-CLEAN, NOT RUN (needs 6112 on B1) |

OWED in Task 2: the `mutation.precondition-drifted` step (S5-PUZZLE: edit a drag's offset far away → its recorded proximity connect
shows a NEW warning → withdraw it → ready) — the probe needs a drag that records a proximity `connect-handles` (a node dropped
beside a free handle), which must be calibrated on the live board (handle positions are not in the board vitals);
`mutation.inverse-refused` (Fatal) has no user-visible step yet; the wgpu arm of the Actions-rail runner, of shell chrome rows
and of the restore alert (Task 3 `--explore`).

### S5.7 Runs on the hot-reloaded serve after `REACT RUN 5 DONE` ("B0 guest + s1 host", then + `attach`)

| Batch | When | PASS | FAIL | Result |
|---|---|---|---|---|
| A | 05:27 | 76 | 0 | step 3 = 18/18 with the three new band verdicts; `run5-react-en-A-s1.txt` |
| C | 05:29 | 52 | 2 | F1 CLOSED (step 10 after step 8 = 22/22: the readout editor stays open, refusal "Must be greater than 0"); the 2 FAILs are the two B1-guest verdicts above; `run5-react-en-C-s1.txt` |
| F | 05:35 | 35 | 5 | F2 CLOSED (controls 28.8 px), O5 CLOSED on the phone (15/15); **F8 new**: at 768 px the bottom-right panel tab bar (Settings / Marketplace / History / Tasks) is painted over the band's first row, "Accept draft" / "Discard draft" cannot be tapped (`tablet-accept-by-touch-reviews-ready` + 4 follow-ons); `run5/probe-react-2026-10-05T03-35-29-en-s15-tablet-prompt.png` → S5-UI |
| H | 05:39 | 27 | 0 | **F4 CLOSED on the `attach` wave**: A lists B's drag "Not applied while editing" (12.1 s), Accept replays it, A's finalize reaches B (11.1 s), both peers hold the same document and rows, B's drag survives, no restore alert, no `timeTravel.stale`; the shell announces the read-back ("Document load: Loading document: 377 of 377"); leftover O4: an automatic checkpoint refused by the load shows the person a `document.loading` notice; `run5-react-en-H-attach.final.txt` |

### S5.8 Run 6 — build B1 (activation 07:19: channel 22, S5-RUNTIME waves A–E, S5-STORE, outcome codes, S5-WGPU waves 1–3; host = S5-UI s1 + r2 + r3, S5-LOAD attach)

Outputs: `T/🗑️generated/s5-e2e/run6-<renderer>-<locale>-<batch>.txt` (+ `.invalidated-<n>.txt` for attempts a foreign save reloaded),
`run6/probe-<renderer>-<stamp>.md|ndjson`, scratch dumps `wgpu/`. Each batch log starts with the served TypeScript saved since
07:19 (the host half of the build under test): among them S5-STORE 09:37 (`🏪️store/👷️worker`, `💻️os/🟦️.ts`), S5-LOAD 09:53 +
10:17 (`📡️backbone/🔗️binding`, `🏛️ShellHost`, `🔌️PluginRuntime`; wave `attach-told`), S5-PUZZLE 09:52 (mutation labels).

**React `en` (valid runs)**

| Batch | When | PASS | FAIL | Failing verdicts → finding |
|---|---|---|---|---|
| A | 07:27 | 75 | 1 | `no-uncaught-page-errors` (unhandled `actor-document-control.receipt-count` in the port retire at folder attach) → F9 |
| B | 09:48 | 63 | 7 | `folder-attach-writes-the-document-archive`, `folder-reconnect-offered`, `positions-persist-after-reload`, `edit-ids-survive-the-reload`, `document-rows-survive-the-reload`, `overwrite-row-survives-the-reload`, `no-uncaught-page-errors` → F9 |
| C | 09:57 | 52 | 2 | `the-first-blocking-row-is-revealed-on-a-blocked-review` → F12; `no-uncaught-page-errors` → F9 |
| D | 10:47 | 45 | 0 | — |
| E | 11:04 | 29 | 6 | 5 × reload check + `warning-row-survives-the-reload` → F9 (step 13 itself 13/14) |
| F | 10:51 | 40 | 0 | — (F2, F8 closed) |
| G | 10:59 | 20 | 1 | `n15-edit-is-disabled-while-replaying-naming-why` → F3 |
| H | 11:08 | 18 | 6 | `g10-second-peer-opens-the-shared-document` + 5 × `g10-…` → F9 |
| I | — | — | — | not recorded: 11:13 run invalidated by a foreign save (`🚪️io/🪶️sqlite-snapshot/🟦️.ts`), then the wave-C window |
| J | 11:02 | 20 | 0 | — (F7 closed: wheel zoom, middle-button pan, zoom out add no row) |
| K | 11:01 | 18 | 0 | — (step 20 = 8/8, first live run of the row actions) |
| **Σ** | | **380** | **23** | F9 × 21, F3 × 1, F12 × 1 |

**wgpu `en`**: A (10:37) PASS 7 / FAIL 12 — `wgpu-boots-in-a-fresh-browser-profile` (F10), `wgpu-loads-the-example-at-boot`,
`wgpu-navbar-example-loads-the-board`, `board-has-nodes` (F11), `history-panel-speaks-the-locale` (probe key mismatch, fixed after
the run: the open tab is the chrome button `framework.panel.history` with `aria-pressed=true`), 6 × `blocked-by-the-empty-board`,
`no-uncaught-page-errors` (the boot fault). The probe's own folder attach PASSES on wgpu (`.semio/documents.db` written).

**Findings of Run 6**

| # | Owner | Finding | Evidence |
|---|---|---|---|
| F9 | S5-LOAD (cause: the peer's pack encoder stopped sorting object keys → the guest's control receipt is noncanonical for the TS grammar; guest fix on disk 10:18, live with B2) | Folder persistence is dead on React B1: Attach → 2 × GET 204 → the bind fails (`actor-document-control.noncanonical` in `decodeDocumentBackboneControlV1`, before LOAD's 09:53 save: unhandled `receipt-count` in `retire`) → no PUT ever, no reconnect offer after a reload, re-attach loads the plain example, two peers never share. Since `attach-told` (10:17) the person is told: console `[os-shell] sync attach failed Error: actor-document-control.noncanonical`, notice `sync.attach.failed` "The document could not be attached", 0 uncaught. The wgpu host attaches the same folder fine (GET 204 → PUT 200) — not guest-side on both hosts. | `run6-react-en-B.txt`, `run6-react-en-B-attach.txt`, `run6/probe-react-2026-10-05T08-34-29.md`, `run6/probe-react-2026-10-05T08-02-06-en-s1-sync-card-after-attach-probe-1.png`, `wgpu/wgpu-contact.json` |
| F3 | S5-UI | Edit pressed while replaying: `aria-disabled` now (the native attribute is gone), 66 ms after the band reads `replaying`, but still without `aria-describedby` — no reason named. | `run6-react-en-G.txt` line 22 |
| F12 | S5-UI / S5-RUNTIME | On the edge into a blocked review the History panel is open but the first blocking row is not in the DOM / viewport (one valid run, read at once; the 4 s settle was added afterwards and its two runs were invalidated). | `run6/probe-react-2026-10-05T07-57-58.md` |
| F10 | S5-WGPU | The wgpu shell does not boot in a fresh browser profile: "worker-boot-failed: language-authority: missing explicit shell terminology authority" (`shell_language_axes`: terminology only from `SEMIO_LOCKED_TERMINOLOGY` or the stored preference). The probe continues as a returning visitor (a stored `setTerminology native` event). | `run6/probe-wgpu-2026-10-05T08-25-33.md`, `wgpu/wgpu-boot-console.txt` |
| F11 | S5-WGPU / S5-PUZZLE | The wgpu board is empty: 0 nodes / 0 edges in all three windows after boot, no Commands row; choosing the example in the navbar (`shell.example.nakagin-capsule-tower`, `setActiveExample branch=catalog`, typed operation 1.2 s) leaves it empty; no console error. Every board step is blocked. | `wgpu/wgpu-contact.json`, `wgpu/wgpu-contact-after-example.png`, `wgpu/wgpu-contact-console.txt`, `run6-wgpu-en-A.final.txt` |
| O-w1 | S5-WGPU | First ARIA-oracle pass over the mirror: `aria-expanded` on `role=group` section nodes (History ×2, Inspection), `aria-valuetext` on `role=combobox` (filter, search). | `run6/probe-wgpu-2026-10-05T08-25-33-en-s1-explore.json` (`aria`) |
| O-w2 | S5-WGPU | Painted panels at the bottom anchors differ from React: History lists Commands first / Actions last, labels right-aligned, row-button icons drawn over their labels; the sync chip stays "Remote: detached" after a successful attach. | `wgpu/wgpu-contact-history.png` |
| E1 | coordinator (a peer outside the fleet) | The React dev serve is saved without the `serve` lock (`🗣️Interpreter`, `🛠️ShellHelpers` incl. a temporary `[DEBUG] Draw Actions dispatch` log, `🔌️PluginRuntime` ×4, `🚪️io/🪶️sqlite-snapshot`): 8 batch attempts reloaded. `console-is-debug-free` FAILs from that log are FOREIGN, not product faults. | HMR frames in the `.invalidated-*.txt` logs |

Closed live on B1 (React): F1 (C step 10 = 22/22), F2 + F8 + O1/O5 (F 40/0; `band-overlaps-no-other-text` on desktop, phone, tablet;
band in `layout-subfooter`), F7 (J), `band-offers-next-problem-first-while-blocked`, `refused-band-controls-stay-reachable-and-name-why`,
`band-controls-carry-their-control-ids`, the row actions of design §22.1 (K), O3 (`timeTravel.stale` no longer shown in D).
F4 cannot be re-judged on B1 (no folder); it passed on the B0 guest + attach wave (S5.7). F5 / F6 not judgeable on B1.

**Probe changes in Run 6** (strict `tsc` exit 0 after each: `tsc-32…42.txt`): `layout-subfooter` seat; HMR `full-reload` frames
recorded → `navigationCause`; an unexpected reload aborts the batch (`INVALIDATED by …`) and `batch.sh` re-runs it (≤ 3, inside
the 10-min call budget); step 9 notes `folder-requests-of-the-run` + `console-errors-in-full`; the sync card's text / alerts after
Attach; `the-first-blocking-row…` with a 4 s settle + window evidence. wgpu arm: `wgpu-boots-in-a-fresh-browser-profile`
(judged without any seed; `--wgpu-seed-terminology` skips the fresh attempt in later batches), stored-terminology seed,
`wgpu-loads-the-example-at-boot`, `wgpu-navbar-example-loads-the-board`, `blocked-by-the-empty-board`, `mirrorAwait`
(panel nodes reach the mirror up to ~3 s after the press; Attach enables 2.8 s after the path is typed), panel-open reader by
`aria-pressed`, Actions-rail runner through the mirror (`<window>/framework.section.engagements/action.<id>`; the staged form's
Execute key is still matched by suffix — no form opened on the empty board).

**OWED after Run 6**: Run 7 on B2 (React `en` A–K incl. I, wgpu `en` A–K, `de` on both); the `mutation.precondition-drifted`
step; wgpu calibration of everything behind the board (drag aim, editor keys, `::editor` readouts, the action form's Execute).

### S5.9 Prepared for Run 7 (build B2, activation building since 11:21) — WRITTEN, TYPE-CLEAN, NOT RUN

Order (coordinator): React `en` A–L (F9 must be green in B, E, H, I — H = two-peer convergence through the merge route, I = the
stepped load) → wgpu `en` A–L WITHOUT the stored-terminology seed (`wgpu-boots-in-a-fresh-browser-profile`,
`wgpu-loads-the-example-at-boot` are judged; the seed is taken only if the fresh boot is refused) → `de` on both.
Command: `RUN=run7 zsh T/🗑️generated/s5-e2e/batch.sh <react|wgpu> <6012|6112> <en|de> <batch> <only> 510`.

| Batch | `--only` | New since Run 5 |
|---|---|---|
| A…J | as S4.4 | Task-2 verdicts (S5.6), `no-document-restore-fails`, navigation attribution |
| K | `1,20,9` | row actions (design §22.1) — green on React B1 |
| L | `1,21,9` | **new step 21, the goal's own sentence as one verdict set**: `goal-one-drag-of-the-selection-is-one-history-row`, `goal-edit-opens-the-drag-mutation`, `goal-the-editor-offers-the-selection-as-a-reference-list` (Use selection + a chip with Remove per target), `goal-the-editor-offers-the-offset-as-steppers-with-the-grid-snap`, `goal-stepping-the-offset-previews-it-and-keeps-downstream-unapplied`, `goal-removing-a-target-previews-the-drag-without-it`, `goal-use-selection-replaces-the-targets`, `goal-accept-re-applies-the-downstream-drag`, `goal-exit-leaves-zero-trace` — same function on both renderers |

To re-judge on B2: F3 (the sample now also records `aria-busy`), F12 (4 s settle + window evidence), F5 / F6 (batch I: load
status outside the panel; `n17-a-cancelled-load-clears-its-status`, `n17-the-document-is-editable-after-a-cancelled-load`), F4
(batch H: `g10-both-peers-converge-after-the-finalize`; the second peer's drag is recorded), F10 / F11 (wgpu boot + example).

§22.13 (`mutation.precondition-drifted`) — NOT written yet, on purpose: the outcome needs a drag that RECORDS a proximity
`connect-handles` with a `tolerance` (`🪢️connect-handles/🔺️diff`: warning when the two handles lie farther apart than the
tolerance; runtime fixture `a-drifted-proximity-connect-warns-and-is-withdrawn`: translate → row = drag + connect → dx edited
far away → the connect reads the NEW warning, review `ready`, worst warning → withdraw the connect → ready → overwrite "2
mutations"). The live example has no free handle (358 handles = 2 × 179 edges); a duplicated node has one. Which pair of free
handles the gesture auto-connects (kind compatibility, proximity radius) must be read from the live board first
(`data-board-handle-positions-json`, React) — a one-minute scratch read on B2, then the step (words: "Warning: Precondition
drifted" / "Warnung: Vorbedingung nicht mehr erfüllt").

Verification of the probe after the last edit: strict `tsc` exit 0 (`tsc-45.txt`), taxonomy report `clean=true errors=0
warnings=0`, module + router import, 193 docstrings with unique leading emojis, no temporary-log tag; module ≈ 5.3 k lines.

### S5.10 Run 7 — build B2 (activation 15:39–16:02: channel 23, F9 guest fix + key-ordered typed-value funnel, merge route, RUNTIME H, PUZZLE §22.13, TOOLS press identity, STORE AA / renumber / CL, UI rows / icons; wgpu lane 16:18: waves 4–9)

Logs: `T/🗑️generated/s5-e2e/run7-<renderer>-<locale>-<batch>.txt`; probe files `…/run7/probe-<renderer>-<stamp>.ndjson`
(screenshots of passing React batches deleted). Batch command as S5.9 with `RUN=run7`.

**React (`:6012`), 0 uncaught page errors, 0 hard guest faults in every batch**

| Batch | `--only` | `en` | `de` | Notes |
|---|---|---|---|---|
| A | `1,2,3,4,5,6,7,9` | 76/0 | 76/0 | |
| B | `1,2,3,4,5,reload,9` | 71/0 | 71/0 | F9 closed: GET 204 ×2 → PUT 200 → GET 200, 5 PUTs, `/.semio/documents.db`, reconnect offered + taken, drift 0, a detached folder is not offered |
| C | `1,8,10,9` | 54/0 | 54/0 | F12 closed (`the-first-blocking-row-is-revealed-on-a-blocked-review`, 814 ms); `de` run 1 invalidated by a full-reload without a file (16:53) |
| D | `1,2,11,12,9` | 45/0 | 45/0 | |
| E | `1,13,reload,9` | 36/0 | 36/0 | reload keeps edit ids / rows / the overwrite row |
| F | `1,14,15,9` | 40/0 | 40/0 | `de` run 1: 39/1 (`tablet-tap-selects-and-a-drag-moves-the-node`: the tap selected nothing in 10 s at load 47), re-run 40/0 — not reproduced |
| G | `1,16,9` | 20/1 | 20/1 | **F3** |
| H | `1,17,9` | 24/0 | 24/0 | F4 closed: both peers converge, the second peer's drag survives; `en` run 1 invalidated by the generated playgrounds registry (wgpu activation) |
| I | `1,18,9` | 21/1 → 24/0 | 24/0 | pure B2: **F13**; after the host saves of 17:01 / 17:11: F13 + F5 closed |
| J | `1,19,9` | 20/0 | 20/0 | |
| K | `1,20,9` | 18/0 | 18/0 | row actions |
| L | `1,21,9` | 19/0 | 19/0 | **the goal's own sentence, 9/9 in both locales, green on its first run** |
| M | `1,22,9` | 18/0 | 18/0 | **new step 22, design §22.13 `mutation.precondition-drifted`, 8/8 in both locales** (17:28 / 17:30) |
| **Σ** | | **465 / 1** (462 / 2 on pure B2) | **465 / 1** | |

Build identity: `en` A–L and `de` A–G ran on pure B2; `de` H–L and the `en` I re-run ran on B2 + the React host saves of
17:01:59 (`🏛️ShellHost`, `🗨️dialog-origin/🛂️admission/📄️document`, `os/🟦️.ts`) and 17:11:09 (+ `🛠️ShellHelpers`), landed
under the serve lock.

Goal sentence, clause → verdict (batch L unless named):

| Clause | Verdict(s) | `en` | `de` |
|---|---|---|---|
| after a board drag History shows ONE drag row | `goal-one-drag-of-the-selection-is-one-history-row` ("Drag 2 items by (60, 40)" / "2 Elemente um (60; 40) ziehen", one mutation) | PASS | PASS |
| Edit → time-travel band | `goal-edit-opens-the-drag-mutation` | PASS | PASS |
| the editor offers the SELECTION as a reference list (Use selection / remove) | `goal-the-editor-offers-the-selection-as-a-reference-list`, `goal-removing-a-target-previews-the-drag-without-it`, `goal-use-selection-replaces-the-targets` | PASS | PASS |
| dx / dy as steppers with the grid snap | `goal-the-editor-offers-the-offset-as-steppers-with-the-grid-snap` (60.00 / 40.00, + / −, step 1) | PASS | PASS |
| preview = the document as of that mutation, downstream NOT applied | `goal-stepping-the-offset-previews-it-and-keeps-downstream-unapplied` | PASS | PASS |
| Accept re-applies downstream | `goal-accept-re-applies-the-downstream-drag` (review "Ready to finalize") | PASS | PASS |
| fatal → Next problem → resolve | batch C step 8: `replay-review-is-blocked`, `finalize-disabled-while-blocked`, `band-offers-next-problem-first-while-blocked`, `next-problem-control-reachable`, `the-first-blocking-row-is-revealed-on-a-blocked-review`, `withdrawing-the-failing-drag-makes-the-review-ready`; batch K step 20: `r22-withdrawing-the-blocking-mutation-from-its-row-makes-the-review-ready` | PASS | PASS |
| warnings | batch E step 13: `g4-the-band-names-the-warning`, `g4-the-drag-row-reads-warning-partially-applied`, `g4-the-warning-is-marked-new-since-this-edit`, `g4-the-review-is-ready-not-blocked`, `g4-the-warning-stays-visible-after-finalize` | PASS | PASS |
| Finalize prompts Overwrite vs New alternative | batch A step 5: `finalize-opens-the-dialog`, `dialog-offers-destructive-overwrite`, `dialog-offers-new-alternative-with-a-name-field`, `overwrite-closes-the-session`, `overwrite-row-appears`; step 7: `name-field-takes-the-alternative-name`, `new-alternative-closes-the-session`, `alternatives-section-lists-the-new-alternative` | PASS | PASS |
| leaves no trace on Exit | `goal-exit-leaves-zero-trace` | PASS | PASS |

**React findings**

- **F9 CLOSED LIVE** (S5-LOAD): folder persistence works on B2 (batch B, E, H, I).
- **F4 CLOSED LIVE**: two peers converge through the merge route; B's edit reaches A in 1.9 s while A edits, is listed "Not
  applied while editing", Accept replays it, after A's finalize both peers hold the same two nodes and the same four rows.
- **F12 CLOSED LIVE**, **F6 CLOSED LIVE** (a cancelled load clears its status in 1.6–2.7 s, the document is editable).
- **F13** (S5-LOAD / S5-UI), found on pure B2 and **CLOSED LIVE by the 17:01 / 17:11 host saves**: a load the person cancelled
  raised `[data-semio-bootstrap-status][role=alert]` "Document restore failed: AppChannelClient.loadDocumentArchive(…):
  cancelled" + a console error (`run7-react-en-I.b2.txt` L27, L33, L39, L40). After the saves: notice
  `shell.documentTransfer.load-cancelled` ("Laden von „Editor“ abgebrochen; das bisherige Dokument ist unverändert.", `de`
  17:09) and, after the second save, `shell.documentTransfer.folder-detached` ("The folder was detached because its document
  was not loaded; nothing is saved to it. Reconnect the folder to load its document.", `en` 17:12 — no PUT follows).
- **F5 CLOSED LIVE** (same saves): the stepped load shows a shell status (`role=status`, polite, with Cancel) besides the
  History body's section; batch I grew from 22 to 24 verdicts.
- **F3 OPEN** (S5-UI / S5-RUNTIME), settled read: the band turns `replaying` at 157 ms; 166 ms later (`en`; 180 ms `de`) Edit on
  a mutation row still reads ENABLED; the press is refused by the notice `timeTravel.illegal` ("Not possible right now" /
  "Derzeit nicht möglich"); the row's own `aria-disabled` + reason reaches the DOM at 982 ms — after the 751 ms replay ended
  and the band is back in `reviewing`. Frames in between read `aria-disabled` + `aria-busy` (the host's pending state of the
  probe's own press; listed, not judged). Evidence: `run7/probe-react-2026-10-05T14-49-00.ndjson`, `run7-react-en-G.txt` L21.
- **§22.13 PROVEN LIVE** (batch M, step 22, `d13-*`, 8/8 in `en` and `de`). The live example has no free handle, so the step
  makes a compatible free pair: a one-handle capsule is duplicated (⌘D; `node-1:link`, kind "door capsule right", free), the
  original is deleted from the Actions rail (its peer handle `f28b4ee5…:sl1_d0`, kind "door tambour right", is free again —
  read as the one published handle row whose flag flipped), and the clone is dropped with its handle 7.49 world units beside
  it. Result: ONE row "Drag 1 item by (-72, -50) (+1)" with the leaves "Drag 1 item by (-72, -50)" and "Connect "node-1:link"
  to "f28b4ee5…:sl1_d0"". Edit the drag → dx 128 → Accept: review "Ready to finalize", "Worst outcome: Warning", the connect
  row reads "New since this edit · Warning: Precondition drifted" ("Neu durch diese Bearbeitung · Warnung: Vorbedingung nicht
  mehr erfüllt"), 179 edges kept. Withdraw on the connect's row → "Ready to finalize, Accepted changes: 2", no warning, 178
  edges → Finalize overwrite → "History edited — overwrite: 2 mutations" / "Verlauf bearbeitet — überschrieben: 2 Mutationen".
  A precondition the step cannot establish is a NOTE (`d13-not-provokable-*`), never a FAIL.
- O6 (S5-PUZZLE / S5-UI, outside the goal) — running Delete Selection from the Actions rail resets the overview camera: (38.39,
  77.58, zoom 1.4641) before, (0, 0, zoom 1) after; the duplicate and the re-select before it keep the camera (`cameras` in
  `d13-a-drop-beside-a-free-handle-…`, both locales).
- O7 — `data-board-handle-positions-json` is capped at 128 rows in id order while 178 handles are on screen at zoom 1, so a
  clone's handle (`node-1:…` sorts last) is cut; the step zooms in until the cap no longer cuts (zoom 1.46–1.61).
- Not judgeable: the history-step replay (switch of alternative) still finishes within one refresh
  (`n17-the-history-step-adopted-within-one-refresh`), so its refusal and Cancel cannot be pressed.

**wgpu (`:6112`), fresh browser profile, NO stored-terminology seed**

| Run | Result |
|---|---|
| `en` A, attempt 1 (16:18) | 5/2 then cancelled at 510 s — PROBE faults P15, P16 (below) |
| `en` steps 1, 2, 9 (16:31, `run7-wgpu-en-A12.txt`) | step 1 **7/1**, step 2 1/7 (F14), step 9 5/0 |

- **F10 CLOSED LIVE** — `wgpu-boots-in-a-fresh-browser-profile` PASS (booted after 6–12 s, three board windows).
- **F11 CLOSED LIVE** — `wgpu-loads-the-example-at-boot` PASS (180 nodes, 179 edges, navbar "Nakagin Capsule Tower");
  History lists "Set Active Example" with its row action "Backwards: Set Active Example".
- Green on wgpu B2 too: History panel in `en` ("Commands"), folder attach (card, path typed, Attach, `folder:///…`, drift 0).
- **F14** (S5-WGPU, BLOCKER) — the first pointer click on a board node kills the renderer: "wgpu renderer fault:
  worker-frame-failed: board retained authority faulted", surface quarantined. Deterministic, isolated repro: boot → move to
  (480,465) (hover dispatches `applyBoardEvents`, fine) → click → PointerDown + PointerUp at 18.8 s → fault at 19.5 s. Text
  recorded at `🧊️wgpu/🧊️renderer/🦀️.rs:15787`. Evidence: `wgpu/b2-click/wgpu-contact-console.txt` L183–259,
  `wgpu-contact.json`, `run7-wgpu-en-A12.txt` L15–22.
- **F15** (S5-WGPU) — `aria-pressed=true` on the chrome tabs Artifact and Inspection (also Display, Tool, Settings) at boot
  while no panel is shown or mirrored (`wgpu-pressed-panel-tabs-show-their-panel` FAIL; `wgpu/b2-panels/wgpu-contact-booted.png`).
- **F16** (S5-WGPU) — Actions rail → Select All selects nothing: the pointer click dispatches `action=selectAll` 2.2 s later,
  `dumpBoard2d().selection` stays empty for 14 s, nothing is highlighted (`wgpu/b2-rail2/`).
- **F17** (S5-WGPU) — Actions rail → Move… (and Rotate…) opens no form: no arg field, no Execute in the mirror or on the canvas.
- **F18** (S5-WGPU) — rail rows cannot be activated by an assistive technology: `treeitem` without `data-actionable`; a mirror
  click is not forwarded, focus + Enter dispatches the KeyDown and runs nothing (`wgpu/b2-rail/`).
- O-w2 open — the History panel paints bottom-up and draws each action row's label twice, overlapping
  (`wgpu/b2-panels/wgpu-contact-history-open.png`). O-w3 — 5 s after Select All the shell dispatched `commitCheckpoint` on its
  own and History lists a "Commit Checkpoint" row nobody asked for (`wgpu/b2-rail2/wgpu-contact-console.txt` L231).
- **No goal clause is proven on wgpu yet**: the bounded attempt to reach a history edit without a board click (rail: Select
  All → Move…) ended at F16 / F17.

**Probe faults found and fixed in Run 7 (none weakens a verdict)**

- P15 — the wgpu boot wait answered before the mirror carried the chrome (boards at 6 s, chrome 3–6 s later), so the first tab
  press found nothing. `waitForBoot` now also requires mirrored `shell.chrome` nodes.
- P16 — open panels were read from `aria-pressed` (B1 calibration); on B2 five chrome buttons read pressed at boot, so
  `closePanels` OPENED five panels. Open panels are now the tabs whose body window the mirror carries; the pressed state is
  judged on its own (F15).
- P17 — the Edit-during-replay press fired in the first frame the band read `replaying`. It is now a settled press (once Edit
  reads disabled, else 160 ms after `replaying`, timer beside the observer); `aria-busy` frames are listed, not judged.
- New evidence only: `slow page read` log for a page read over 5 s; `underTheTap` + `tapWaitedMs` on the tablet-tap verdict.
- New step 22 (`step22`, `handleRows`), registered in `ORDER` after 21 and in `STEP_NUMBERS`; copy keys `drifted`,
  `overwriteTwo` in both locales. Three probe faults on its way (no rows published right after the rail closes → wait; the
  128-row cap → zoom in; the camera reset after the delete → zoom in again around the clone).
- Scratch `wgpu-first-contact.ts` got the modes `click` and `rail` and no longer seeds a terminology unless asked.
- Verification: strict `tsc` exit 0 (`tsc-56.txt`), 197 docstrings with unique leading emojis, no temporary-log tag.

**Owed after Run 7**: wgpu `en` A–L after "SERVE UP (B2w)", wgpu `de`; calibration of everything behind the board on wgpu
(drag aim, editor keys, `::editor` readouts, the action form's Execute — the rail route must use the pointer on
`tree.label.<key>` hits, not the mirror); step 22 on wgpu needs handle positions from `dumpBoard2d` (it publishes none).

### S5.11 wgpu on B2w (re-activation 17:34:39: S5-WGPU waves 10 + 11) — first live history edit on wgpu

Logs: `run7-wgpu-en-L.txt` (last run 18:16, load average 50–56), earlier runs `run7-wgpu-en-L.run3.txt` (17:50, load ≈ 20) and
`run7-wgpu-en-L.run5.txt` (18:06); screenshot of a wgpu session in review `run7/probe-wgpu-2026-10-05T15-50-41-en-s21-goal-reviewed.png`;
mirror dumps `wgpu/b2w-aim/`, `wgpu/b2w-expand/`.

Closed live on B2w: **F14** (a click selects a node, no renderer fault; a drag moves it by (80, 40)), **F18** part (the
mutation row and its row actions are `data-actionable`).

Goal sentence on wgpu `en` (batch L, 10 verdicts since the on-grid one was split out):

| Verdict | wgpu | What the run shows |
|---|---|---|
| `goal-one-drag-of-the-selection-is-one-history-row` | PASS | "Drag 2 items by (60, 40)", one row, one mutation leaf (3 runs) |
| `goal-edit-opens-the-drag-mutation` | PASS | band "Editing a mutation · Editing: Drag 2 items by (60, 40)" (3 runs) |
| `goal-the-editor-offers-the-selection-as-a-reference-list` | PASS | chips t_f5_b_c1, t_f7_b_c1, each "Remove …", "Use selection" |
| `goal-the-editor-offers-the-offset-as-steppers-with-the-grid-snap` | PASS | mirror: `<input type=number step=1>` "Offset X" / "Offset Y"; canvas "+ 60.00 −" |
| `goal-the-recorded-drag-offset-lies-on-the-grid-snap` | **FAIL** | dx = 59.99996 (F21) |
| `goal-stepping-the-offset-previews-it-and-keeps-downstream-unapplied` | **FAIL** | the step lands on 60 (a = 146.132) but later than the 45 s wait at load 56 (17 s at load 20); downstream stays unapplied |
| `goal-removing-a-target-previews-the-drag-without-it` | **FAIL** | still two chips after 46 s; one chip at the next read ≈ 60 s later |
| `goal-use-selection-replaces-the-targets` | **FAIL** | pressed; the new target is not moved in the preview within 45 s; it is at Accept |
| `goal-accept-re-applies-the-downstream-drag` | PASS | review "Ready to finalize · Accepted changes: 1", downstream drag re-applied exactly, new target placed |
| `goal-exit-leaves-zero-trace` | not reached | 520 s cap (one earlier run: band still `reviewing` 20 s after Exit) |

Mirror DOM of the open editor (note `wgpu-editor-mirror-nodes`): `framework.history.timeTravel.status` "Editing a mutation:
…", `framework.history.editor.target` "Draft: …", `…editor.accept|discard|withdraw` buttons, `…input.targets.row`,
`…targets.useSelection` button, `…targets.chip.<n>.row` + `…chip.<n>` button "Remove <label>", `…input.dx|dy`
`input[number] step=1`, band `ui.timeTravel.accept|discard|exit` + `shell.time-travel.status` (status).

Findings (owner S5-WGPU unless named):

- **F21** (S5-WGPU / S5-PUZZLE) — a 60-unit board drag records dx 59.99996 (a node dragged by 80 ends at 166.13196; React:
  166.132): f32 pointer math and no grid snap on the recorded offset. The row label rounds it to "(60, 40)", the editor shows
  59.99996, the first step snaps to 60. Caveat: the probe's pointer x is fractional (486.132); React is exact with it.
- **F22** — while a session is open the page's main thread stalls: page reads of 5–44 s (15 in one run), growing through the
  run, even for a plain `querySelectorAll` over the mirror; each editor input takes 45–90 s to reach the preview at load 56
  (≈ 17 s at load 20). React at the same load: < 1.5 s per clause.
- **F23** (was O-w2) — the History panel is unreadable while editing: the editor's steppers, Remove, Use selection, Withdraw /
  Discard / Accept and the Actions buttons are painted over the Commands rows (screenshot above). For the diagnosis:
  `react-panel-dom/react-panel-history-bottom-right.html` (panel root `data-anchor="bottom-right"`, `flex-direction:
  column-reverse`, rect [1297, 708, 300, 288]) and `react-panel-inspection-top-right.html` (`data-anchor="top-right"`,
  `column`, rect [1297, 3, 300, 144]), each with a `.png` and `react-panel-dom.json`.
- **F20** — an expandable History row cannot be expanded through the mirror (click and focus + ArrowRight keep
  `aria-expanded=false`); a pointer press on the row's right end expands it, the `tree.chevron.<row>` hit exists only once it
  is open, and the mutation row with "Edit: …" / "Withdraw: …" reaches the mirror 4–6 s later.
- F16 (rail Select All selects nothing) — a guest fault on every renderer per the coordinator; F15 — not a fault (matches
  React); O-w3 — the automatic check-in, not judged.
- Observations: `dumpBoard2d().rect` is window-local (page rect = the `ScrollRegion` hit of the window id); every hover /
  select adds an "Apply Board Events" row to History (8 around one drag); the History mirror fills over > 4 s after opening.

Probe faults fixed for wgpu (rule 49, none weakens a verdict): P18 aim through the page rect; P19 History read only once the
mirrored rows are stable; P20 expand by the pointer; P21 no needless panel scroll when every row is mirrored; P22 a stepper is
also `<input type=number>` with its `step`; P23 wgpu waits of 45 s on the editor, chips and previews, with the wait in the
detail; P24 the off-grid step is judged to 1e-5 so 59.99996 does not pass for 60. New note `history-rows-at-the-missed-mutation`.
`wgpu-pressed-panel-tabs-show-their-panel` became the note `wgpu-pressed-panel-tabs-without-a-mirrored-panel` (F15 is not a
fault by the coordinator's decision). Verification: strict `tsc` exit 0 (`tsc-66.txt`), 198 unique docstring emojis, no
temporary-log tag; React `en` L re-run 20/0 with the 10-verdict set (React `en` Σ 466 / 1, `de` Σ 465 / 1).

### S5.12 Run 8 — build B3 (React recycled 20:16: S5-PUZZLE guest wave §22.32 (a) + F21 + F16, STORE LO, RUNTIME H2, LOAD f13 + detach, GRAPHS tool-run-settle); React `en` only

Logs: `T/🗑️generated/s5-e2e/run8-react-en-<batch>.txt`, probe files `…/run8/probe-react-<stamp>.ndjson` (screenshots of passing
batches deleted). wgpu was not probed (`:6112` still served B2w).

**(a) Regression A–M — 472 PASS / 1 FAIL, 0 uncaught, 0 hard faults; no B3 regression**

| A | B | C | D | E | F | G | H | I | J | K | L | M | Σ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 76/0 | 74/0 | 54/0 | 45/0 | 39/0 | 40/0 | 20/1 | 24/0 | 24/0 | 20/0 | 18/0 | 20/0 | 18/0 | **472 / 1** |

Against the Run 7 baseline (466 / 1): B + 3 and E + 3 — more verdicts are judged, not fewer faults. L's first run was
invalidated by a full-reload from `📐️Canvas2dHost/🟦️.tsx` (20:30, an outside peer's save) and re-run clean at 21:09.

- **F3 OPEN, unchanged** (batch G, `n15-edit-is-disabled-while-replaying-naming-why`): band `replaying` at 194 ms; Edit pressed
  167 ms later still reads enabled; refused by the notice `timeTravel.illegal`; the row's own `aria-disabled` + "Not possible
  right now" reaches the DOM at 1039 ms, after the 770 ms replay ended. `run8/probe-react-2026-10-05T18-24-22.ndjson`.
- **F21 guest half CLOSED LIVE**: the drop of step 22 records exactly "Drag 1 item by (-72, -50)" with offset `[-72, -50]`
  (Run 7: `[-71.999985, -50]`); the row, the editor and the node position agree.
- **O6 STILL PRESENT, and wider than Delete Selection** (S5-PUZZLE / board host): note `m32-camera-trace` of batch N — the probe
  zooms to 1.331 to aim at handles; the camera is back at (0, 0) zoom 1 after the upstream history-edit session, after the
  duplicate, after the wire, after the second delete and after the brush (not yet 2.5 s after a delete, but before the next
  aim). Batch M shows the same after the rail's Delete Selection (`afterTheDelete`).

**(b) New batch N (`1,23,9`) — step 23, design §22.32 (a) "tools are state machines that yield mutations within a transaction": 27 / 0**

Figures are those of the final run, 21:07 (`run8/probe-react-2026-10-05T19-07-28.ndjson`); earlier runs of the step are kept
as `run8-react-en-N.run2…run5.txt` and `.invalidated-1.txt`.

| Verdict (`m32-…`) | Result | What the run shows |
|---|---|---|
| `a-streamed-drag-preview-paints-in-the-other-windows` | PASS | Detail AND Selection panes: pixel-stable before the press and while held, repaint once the node is dragged (mode `dragNodes`) |
| `an-aborted-drag-leaves-no-row` | PASS | Escape mid-drag: node back, no document row |
| `a-press-and-release-that-changes-nothing-leaves-no-row` | PASS | click on empty board, nothing selected: zero rows of any kind |
| `a-history-edit-begun-mid-drag-leaves-no-row` | PASS | Edit pressed in page while the pointer holds a node: band `editing` after 132 ms, release commits nothing, node unmoved |
| `select-all-from-the-actions-rail-selects-every-node` | PASS | 180 / 180 in 768 ms (F16 closed live) |
| `use-selection-takes-the-select-all-selection` | PASS | the open edit of a drag takes all 180 as targets (47 chips materialised), preview drags the sampled nodes by the offset |
| `deleting-a-node-with-its-edge-is-one-history-row` | PASS | Delete on a one-handle capsule: nodes 180 → 179, edges 179 → 178, ONE row "Delete node "…"", 1 mutation |
| `an-upstream-edit-previews-without-the-later-delete` | PASS | editing the earlier drag shows the deleted node again |
| `accept-of-the-upstream-edit-replays-the-delete` | PASS | Accept → review ready, node gone again, drag at its new offset |
| `a-wire-dragged-handle-to-handle-is-one-history-row` | PASS | clone's free "door capsule left" handle → freed "door tambour left" handle (`linkDragSnap`): edges + 1, ONE row "Connect "node-1:link" to "…:sl0_d1"", 1 mutation |
| `a-brush-placed-node-is-one-history-row` | PASS | brush armed, Alt + sweep over the freed handle: node + 1, edge + 1, ONE row "Create node "puzzle2d.brush.1" (+1)" holding 2 mutations (Create node + Connect; the Connect label was read in the 21:04 run) |
| `a-placed-node-selected-at-once-stays-selected` | PASS | clicked 1037 ms after the placement, still selected 3.5 s later |
| `edit-of-the-placed-node-offers-its-inputs` | PASS | editor lists 14 inputs (id, node kind, shape, x, y, radius, …) |
| `withdraw-then-restore-of-the-placed-node-works` | PASS | Withdraw → review ready without the node; Restore → node back, session closes |
| `withdraw-then-restore-leaves-the-document-as-it-was` | PASS | 180 nodes, no drift, 7 document rows before and after |
| `creating-a-region-is-one-history-row` | PASS | area brush drag (`regionPaint`): region 90 × 70, ONE row "Create target region "…"", listed in the untouched open History panel within 4 ms of the first read |
| `resizing-a-region-is-one-history-row` | PASS | grip drag (`regionDrag`): 90 × 70 → 131 × 101, ONE row "Resize target region "…"" |

Not a verdict: `edgeDelete` (a wire cut) — no gesture found for it in the time; a node delete takes its edge with it (judged).

Gestures, as found live (scratch `tools-explore.ts`, outputs `tools/`): utility bar = `#ui.utilities.2d-overview` with
`button#select|brush|areaBrush` behind `…2dOverview.utilityBar.unfold`; delete = select + Delete key; brush = armed + Alt held
over an open handle (ghost) and leaving the slot; area brush = armed + drag; region grip = select utility, drag the corner;
wire = press on a free handle, drop on a compatible free handle; Select All = the rail row itself, no Execute.

Observations (not judged):
- Every selection / hover flush lists a non-expandable "Apply Board Events" command row; Delete pressed with nothing selected
  lists a "Delete Selection" command row (seen in the explorations).
- Not reproduced: in two exploration runs a painted region was on the board while the open History panel listed no row for
  4–8 s, until the next board input (`tools/tools-explore.json`: `allRowsAfterCreate` has entries 1–3 only; rows 4 + 5 arrive
  with the resize). In the four runs of step 23 that reached the region the row was listed untouched at once (4–80 ms after the first read).

Probe faults on the way (rule 49, none weakens a verdict): the clone for the wire must share the deleted node's handle KIND
(a "capsule right" clone does not connect to a freed "tambour left" handle — a correct refusal, zero rows); the placed node
is aimed at by the board's own `hoveredId`; a row's mutation count is the tree window's `total`, not the materialised rows;
document-row counts are read once two reads agree. Three runs were invalidated by outside saves (`📐️Canvas2dHost`).
Probe: `step23` + helpers `boardInteraction`, `targetRegions`, `setUtility`, `revealHandles`, `documentRowsSince`, `paneShot`;
strict `tsc` exit 0 (`tsc-72.txt`), 205 unique docstring emojis, no temporary-log tag.

**Owed**: batch N in `de`; wgpu `en` A–N on B3w (not announced); `edgeDelete` gesture.

### S5.13 Run 9 — React on B3 guest + hot host waves (S5-UI `f3`, S5-LOAD `f13` + `detach`), serves restarted 23:05

Logs: `run9-react-en-G.txt`, `run9-react-de-N.txt` (+ `run9-react-de-N.cut-1.txt`), `run9-react-de-L.txt`; probe files
`run9/probe-react-2026-10-05T21-35-26.ndjson` (G), `…T21-42-37.ndjson` (N), `…T21-40-47.ndjson` (L). Served files saved
between the restart and the runs: `🐚️Shell`, `🛠️ShellHelpers`, `🛂️manifest`, `🏛️ShellHost/🪟️spawned-program`,
`🛂️manifest/🪛️utilities/🌅️initial` (23:25–23:32); none saved under a run, no run was reloaded.

| Batch | Locale | Result |
|---|---|---|
| G `1,16,9` | `en` | **22 / 0** |
| N `1,23,9` | `de` | **26 / 1** — run 2; run 1 was cut by my own 215 s limit |
| L `1,21,9` | `de` | **20 / 0** |

- **F3 CLOSED LIVE** (S5-UI `f3`). Settled read `n15-edit-is-disabled-while-replaying-naming-why` PASS and the new unsettled
  read `n15-edit-reads-disabled-with-its-reason-in-the-first-replaying-frame` PASS: in the DOM commit in which the band first
  reads `replaying` (243 ms) the row's Edit reads `aria-disabled` with `aria-describedby` → "Not possible right now"; the
  probe's press lands on the disabled action (`settledMs` 0) and nothing opens. Frames: 0 ms reviewing / enabled → 243 ms
  replaying / disabled + reason → 1163 ms reviewing / enabled → 1236 ms reviewing / disabled + reason.
- Observation O9 (not judged): 73 ms after the replay ended the same Edit reads disabled "Not possible right now" again while
  the band is `reviewing` (last frame above; the trace stops there). It looks like the guest's late row publication of the
  replay window arriving after the host re-enabled the action.
- **Batch L `de` 20 / 0** — the goal sentence holds after `f3` (10 `goal-*` verdicts).
- **Batch N `de`**: 16 of the 17 `m32-*` verdicts PASS in German ("Knoten "puzzle2d.brush.1" erstellen (+1)", "Zielregion
  "target-region-2" erstellen", …). The one FAIL, `m32-creating-a-region-is-one-history-row`, is a PROBE fault, not a product
  fault: the region's row was listed in the untouched History panel 19 ms after the first read with one mutation, but the
  verdict counted the rows after a stale sequence baseline and so also counted the brush, delete and wire rows. The History
  read that takes the baseline returned no entry rows for ≈ 12 s at that point in both `de` runs and in none of the `en` runs;
  the cause is not found. Guard written (the baseline never goes below the last known sequence, and it is now in the
  verdict's detail), type-clean, NOT RUN.
- Not reproduced: in the cut first run, Withdraw pressed on the placed node's create row ≈ 2 s after leaving an edit session
  opened no session within 20 s (`run9-react-de-N.cut-1.txt`); in run 2 it opened in 241 ms. The offer's state before the press
  is now recorded (`withdraw.offered`, `bandBeforeThePress`).
- Probe: new verdict above, `settledNewestSeq` + monotonic baseline in step 23. `tsc` over the probe's closure reports ONE
  error, in a peer's file saved at 23:27 (`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts(2003,83)` TS2322), none in the probe
  (`tsc-74.txt`); 205 unique docstring emojis, no temporary-log tag.

**Owed**: N `de` once more for the region verdict; wgpu `en` A–N on a B3 wgpu lane; `edgeDelete` gesture.

### S5.14 Batch U — the universal live journey (design §23.3), 2026-10-06 00:27–00:40

Step 24 (`stepU`), artifact-agnostic: framework selectors only — `[data-slot=window]`, the Actions rail (`[id$=".engagement.toggle"]`
→ `[data-slot=window-action-pane]` → `action.<verb>` rows under `action.category.<name>` → `….action.<verb>.execute`), the History
panel, the time-travel band, the history editor's `framework.history.editor.input.<pointer>.row` rows and the finalize prompt.
No window id, verb name or fixture of an editor is named in it.

**Command line (any React dev serve; the URL is opened verbatim, steps default to 24 + 9):**

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
bun ./📜️script.ts verify time-travel --universal --serve "<url, e.g. http://127.0.0.1:6012/?plugin=puzzle2d>" --renderer react --locales en --out <dir>
```

or, with the serve lock and an automatic re-run after an outside reload: `zsh T/🗑️generated/s5-e2e/batch-u.sh "<url>" <tag> en [seconds]`.
The routed command ran the journey at 00:30 and 00:32; from 00:33 to ≈ 00:39 it could not start (a peer's in-flight taxonomy
edit: `loadCatalogTaxonomy` → "semanticDirectoryMemberKinds collide for owner "commands" and member "📄️load-document-json""),
so the counted runs went through the scratch runner `run-universal.ts` (`RUNNER="bun …/run-universal.ts …/runU"`), which
calls the same entry point; at 00:40 the routed script started again.

How the journey takes each clause:

| # | Verdict | How, without editor knowledge |
|---|---|---|
| u1 | `u1-boots-and-lists-its-actions-and-its-history` | ≥ 1 window; rail verbs listed with their categories; History opens |
| u2 | `u2-one-mutation-row-without-editor-knowledge` | rail rows in manifest order (create, transform, then the rest; transfer / file / view / selection categories and clipboard / delete / import-like verbs skipped): open the row, read the staged args, press Execute when enabled, wait for a document row; else a row of the booted example |
| u3 | `u3-edit-opens-an-editor-with-contract-controls` | first mutation whose Edit is offered enabled → band `editing` → inventory: role (slider, spinbutton, radiogroup, listbox, combobox, switch, textbox, reference list) + name + value + min / max / step / options |
| u4 | `u4-changing-one-input-is-acknowledged-as-a-draft` | first changeable control by one step (ArrowRight / ArrowUp / next radio or option / click / one typed character + Tab); the editor republishes the value, band stays `editing`, later rows read "not applied" |
| u5 | `u5-accept-replays-to-a-review-and-blockers-resolve` | Accept → review; while `blocked`: Next problem → the editor's Withdraw (else the blocker's row action) → Accept, up to four rounds; the path is recorded |
| u6 | `u6-finalize-offers-overwrite-and-new-alternative-and-overwrites` | prompt has Overwrite AND New alternative with its name field → Overwrite → session closed, history-edit row listed |
| u7 | `u7-withdraw-then-restore-then-exit-leaves-zero-trace` | Withdraw on a row (a second row is made by the same rail action when none is left) → Restore on it → Exit → same document rows |
| u8 | `u8-no-faults-and-labels-in-the-locale` | 0 uncaught, 0 hard faults; History section and band speak the locale |

What an editor does not offer is a NOTE `u<n>-NOT-OFFERED-…` / `u<n>-NOT-REACHED-…` with its evidence.

**Results (`en`)**

| Editor | Serve | u1 | u2 | u3 | u4 | u5 | u6 | u7 | u8 | Controls | Log |
|---|---|---|---|---|---|---|---|---|---|---|---|
| puzzle 2d | `http://127.0.0.1:6012/?plugin=puzzle2d` (B3 guest + hot host) | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS | 13 | `runU-puzzle2d-en.txt`, `runU/probe-react-2026-10-05T22-39-32.ndjson` |
| draw ("semio · drawing", window `drawing-composite`) | `http://127.0.0.1:6064/` (a peer's build; boots the editor on the bare URL, no `?plugin=`) | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS | 9 | `runU-draw-en.txt`, `runU/probe-react-2026-10-05T22-38-53.ndjson` |

Both runs: 14 PASS / 0 FAIL including step 9 (no uncaught error, no hard fault, no temporary-log line, no channel mismatch).

- **puzzle 2d** — 32 rail verbs; route: "Add Node…" (`addNode`) executed with its staged defaults after seven verbs that produce
  nothing without a selection (Duplicate, Connect… with Execute disabled, Connect Nearby, Move…, Rotate…, Scale…, Set
  Selection Mode…); row "Create node "node-1"". Inventory 13: textbox × 4 (id, node kind, text, icon), combobox × 1 (shape),
  spinbutton × 6 (x, y, radius, width, height with step 1; index with min 0), slider × 1 (scale 0.1–10), radiogroup × 1 (anchor,
  2 options). Change: id "node-1" → "node-1x". Accept → `ready`. Overwrite → "History edited — overwrite: 1 mutation".
  Withdraw → Restore on the create row → zero trace.
- **draw** — 22 rail verbs, empty History at boot; route: "Add Layer…" (`addLayer`, staged `kind=Path`) after two forms whose
  Execute is disabled without a domain (Set Selection Mode…, Set Granularity…); row "Create layer "layer-973a…"". Inventory 9:
  reference list (parent group), spinbutton × 2 (index min 0 step 1; stroke width step 0.5), textbox × 2 (layer kind, blend
  mode), slider × 1 (stroke colour, 0–1), radiogroup × 3 (line cap 3, line join 3, fill rule 2). Change: Index 1 → 2.
  **Conflict path taken live:** Accept → review `blocked`, "Worst outcome: Fatal" (index 2 is beyond the one layer) → Next
  problem → the editor opens on the create-layer mutation → the editor's Withdraw → Accept → `ready` → Finalize → Overwrite
  AND New alternative offered → Overwrite → "History edited — overwrite: 1 mutation". Withdraw → Restore on a second layer
  row (made by the same rail action) → row actions [Edit, Restore] → zero trace.

Limits of what these two journeys prove (stated, not hidden):
- In both the edited row is the NEWEST row, so u4's "later rows read not applied" is vacuous (0 downstream rows) and no
  `replaying` stage frame was seen in u5. The journey should make a second row before the edit; not done yet.
- u4 changes ONE control per editor (a textbox on puzzle, a spinbutton on draw); the other roles are inventoried, not operated.
- A composite control is listed by its first primitive (draw's stroke colour appears as one slider 0–1).

Observations for owners (from the inventories; none is a journey fault):
- U-o1 (framework editor, both editors): the generic `index` spinbutton publishes min 0 but no max, so a step beyond the
  collection is accepted as a draft and only the replay reports it — as Fatal, which the conflict path then resolves.
- U-o2 (draw): `layer.kind` ("path") and `layer.blendMode` ("normal") are free textboxes although both are enumerations;
  puzzle 2d's `node.nodeKind` and `node.iconKind` are free textboxes too (icon reads "null").

Probe faults on the way (rule 49): draw's first run failed u5 and u7 — both probe timing (the editor's Withdraw and the row's
Restore were read before the rows were materialised); after a settled read both pass and the DOM shows the controls. Two
puzzle runs were invalidated by outside reloads (`Board2dHost`, `scene`, the Vite styling builder saved 00:31–00:32).
Probe: `stepU`, `universalControls`, `universalVerbs`, `universalChange`, `--universal` (verbatim route, boot = ≥ 1 window,
90 s boot cap); strict `tsc` exit 0 (`tsc-78.txt`), 209 unique docstring emojis, no temporary-log tag.

**Owed**: a second row before the edit (downstream / replay); every control role operated once; the wgpu arm of batch U
(through the ARIA mirror); `de`; further editors as their serves come up.

### S5.15 Batch U, extended — two rows, every control role, the conflict clause, `en` + `de` (2026-10-06 02:03–02:34)

Command (the `--editor` slug names the controls file; the URL is opened verbatim):
`bun ./📜️script.ts verify time-travel --universal --editor <slug> --serve "<url>" --renderer react --locales en,de --out <dir>` —
or `zsh T/🗑️generated/s5-e2e/batch-u.sh "<url>" <slug> <en|de> [seconds]`. Runs of this section went through
`RUNNER="bun …/run-universal.ts …/runU"` (same entry point). Logs: `runU-puzzle2d-en.txt`, `runU-puzzle2d-de.txt`,
`runU-draw-en.txt`, `runU-draw-de.txt` (earlier runs `*.run1…`, `*.cut-1`, `*.s15-run1`). Machine-readable inventories:
`T/🗑️generated/s5-e2e/runU/puzzle2d-controls.json`, `…/runU/draw-controls.json` — per locale: route, second row, edited
mutation, rail verbs (verb, category, label), controls (pointer, role, name, value, min, max, step, options, readOnly), what
was operated per role (attempts with before / shown after / refusal notice), the conflict path.

What changed in the journey: u2 makes TWO rows (the second by trying the other rail verbs after the first row, then after
Select All, then the same action again) and the OLDER row is edited; one control of every inventoried role is operated on a
clean draft (`u-control-<role>`; a change counts only when the value is republished, the band stays `editing` and no refusal
notice is raised; up to three controls of a role are tried); u4 requires a later row reading "not applied"; u5 records the
replay stage and its progress frames; `u-conflict-…` is the blocked → Next problem → Withdraw → ready path (taken in u5 when
the edit itself blocks, else by withdrawing the older row); u8 also fails on a rail verb or control labelled by its raw key.

**Verdicts** (step 24; totals include boot + the five console verdicts of step 9)

| Verdict | puzzle 2d `en` | puzzle 2d `de` | draw `en` | draw `de` |
|---|---|---|---|---|
| u1 boot, rail verbs, History | PASS (32 verbs) | PASS | PASS (22 verbs) | PASS |
| u2 two rows without editor knowledge | PASS | PASS | PASS | PASS |
| u3 Edit → editor with contract controls | PASS (12) | PASS (12) | PASS (9) | PASS (9) |
| u-control-textbox | PASS | PASS | PASS (2nd textbox; the 1st is refused — U1, U2) | PASS (same) |
| u-control-combobox | PASS | PASS | not inventoried | not inventoried |
| u-control-spinbutton | PASS | PASS | PASS | PASS |
| u-control-slider | PASS | PASS | PASS | PASS |
| u-control-radiogroup | PASS | PASS | PASS | PASS |
| u-control-reference-list | not inventoried | not inventoried | PASS (Use selection 0 → 1 chip) | PASS |
| u4 one input changed, later row "not applied" | PASS | PASS | PASS | PASS |
| u5 Accept → review, blockers resolved | PASS | PASS | PASS | PASS |
| u5 replay shown as a stage with progress | PASS ("61/182" … "182/182") | PASS | NOTE: ended within one refresh (1 later row) | NOTE (same) |
| u-conflict blocked → Next problem → Withdraw → ready | PASS | PASS | PASS | PASS |
| u6 Finalize: Overwrite AND New alternative → Overwrite | PASS | PASS | PASS | PASS |
| u7 Withdraw → Restore → Exit, zero trace | PASS | PASS | PASS | PASS |
| u8 no faults, labels in the locale, none a raw key | PASS | PASS | PASS | PASS |
| **Total** | **21 / 0** | **21 / 0** | **20 / 0** | **20 / 0** |

No switch and no listbox is inventoried by either edited mutation; puzzle's `index` spinbutton (13th control in S5.14) was not
materialised at these reads, so 12 are recorded.

- **puzzle 2d** — rows: "Add Node…" (`addNode`) → "Create node "node-1""; then "Force Layout" (`forceLayout`, 181 "Move node …"
  mutations, found by trying the other verbs after the first row). Edit of the create: id "node-1" → "node-1x"; the later row
  reads "Not applied while editing" / "Beim Bearbeiten nicht angewendet". **Conflict:** Accept → `replaying` (progress n / 182)
  → review blocked, worst outcome Error (the layout's "Move node "node-1"" lost its node) → Next problem → the editor opens
  on that mutation → its Withdraw → ready, "Accepted changes: 2" → Overwrite. In `de` the route reads "Knoten hinzufügen…" /
  "Kraftbasiertes Layout".
- **draw** — rows: "Add Layer…" twice (no other verb produces a row with its defaults: Combine Boolean… is refused with
  `app.command.invalid-args`). Edit of the first create-layer: Index 1 → 2. **Conflict (a cascade):** Accept → blocked, worst
  outcome Fatal → Next problem → Withdraw the first layer → blocked again (the second layer's index is now out of range) →
  Next problem → Withdraw the second → ready, "Accepted changes: 2" → Overwrite.

**Control inventories** (role, name `en` / `de`, UI metadata the DOM carries)

| Editor · mutation | Pointer | Role | Name `en` | Name `de` | min / max / step / options |
|---|---|---|---|---|---|
| puzzle 2d · Create node | `node.id` | textbox | Node · ID | Knoten · ID | — |
| | `node.nodeKind` | textbox | Node · Node kind | Knoten · Knotenart | — |
| | `node.shape` | combobox | Node · Shape | Knoten · Form | (options not in the DOM until opened) |
| | `node.x`, `node.y`, `node.radius`, `node.width`, `node.height` | spinbutton | Node · X / Y / Radius / Width / Height | Knoten · X / Y / Radius / Breite / Höhe | step 1 |
| | `node.text` | textbox | Node · Text | Knoten · Text | — |
| | `node.iconKind` | textbox | Node · Icon | Knoten · Symbol | — |
| | `node.scale` | slider | Node · Scale | Knoten · Skalierung | min 0.1, max 10 |
| | `node.anchor` | radiogroup | Node · Anchor | Knoten · Verankerung | 2 options |
| draw · Create layer | `parentId` | reference list | Parent group | Übergeordnete Gruppe | — |
| | `index` | spinbutton | Index | Index | min 0, step 1 (no max) |
| | `layer.kind` | textbox | Layer · Kind | Ebene · Art | — |
| | `layer.attributes.stroke.color` | slider (first primitive of the colour control) | Layer · Attributes · Stroke · Color | Ebene · Attribute · Kontur · Farbe | min 0, max 1 |
| | `layer.attributes.stroke.width` | spinbutton | … Stroke · Width | … Kontur · Breite | step 0.5 |
| | `layer.attributes.stroke.cap` | radiogroup | … Stroke · Line cap | … Kontur · Linienende | 3 options |
| | `layer.attributes.stroke.join` | radiogroup | … Stroke · Line join | … Kontur · Linienverbindung | 3 options |
| | `layer.attributes.fillRule` | radiogroup | Layer · Attributes · Fill rule | Ebene · Attribute · Füllregel | 2 options |
| | `layer.blendMode` | textbox | Layer · Blend mode | Ebene · Füllmethode | — |

`de` labels: every rail verb label and every control name differs from its `en` text on both editors, except draw's
"Index" (the same word in German); no label is a raw key or empty.

**Product faults**

- **U1 draw / framework history editor (S5-UI, S5-RUNTIME)** — a text input the session REFUSES keeps showing the refused
  text: "x" appended to `layer.kind` ("path" → "pathx") raises `timeTravel.invalid-input` "Invalid value: the input keeps
  its previous value" / "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert", yet the field still reads "pathx" two
  seconds later. Evidence: `runU/draw-controls.json` → `locales.en|de.operated.textbox.attempts[0]`
  (`shownTwoSecondsAfterTheRefusal`).
- **U2 draw (draw plugin owner; UI metadata)** — two enumerations are published as free text boxes: `layer.kind` refuses
  everything but an exact literal (see U1), and `layer.blendMode` ACCEPTS "normalx" into the draft with no refusal
  (`attempts[1]`, `en` and `de`). Both need their options published (combobox / radiogroup).
- Observations carried from S5.14: the generic `index` spinbutton has no max; puzzle 2d's `node.nodeKind` and
  `node.iconKind` are free text boxes for what looks like enumerations.

Probe faults on the way (rule 49; each fixed with a settled read, none weakens a verdict): a control read right after the
change found the row re-rendering (`settledControl`); a textbox's shown text was taken for a draft although the session had
refused it (now the refusal notice decides — this is how U1 was found); a mutation row read before its actions rendered
looked withdraw-only and sent the edit to the wrong row; the Restore offer was read before the row re-rendered. Verification:
strict `tsc` exit 0 (`tsc-83.txt`), 209 unique docstring emojis, no temporary-log tag.

**Owed**: a switch and a listbox (no edited mutation inventoried one — other mutations or editors will); the wgpu arm of
batch U; further editors as their serves come up.

