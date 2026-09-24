# U5 — os `s` frontend: localized, accessible, interaction-friendly as a user experiences it

Slice U5, session 11, 2026-09-25. Continues U1 (progress/cancel/connection), U2 (touch/tablet/contrast/diagram a11y),
U3/U3b (`LocalizedLabel` mutation labels), U4 (pinch/diagram nav/contrast), AP1 (approval affordance). Sibling S15 owns the
per-plugin matrix, lazy install and the default-palette WCAG fixes. Ports: serves 6580–6589, hubs 8080–8089.
Captures: `wp-u5/generated/`.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Status

| # | task | state | evidence |
|---|---|---|---|
| 1 | mutation labels: German per plugin + live en/de history rows | **measured**: 1649 leaves / 105 crates handcrafted, native check sweep 14/14 green; live History rows en+de for dag, block, gis, note, layout, draw, forms, puzzle (post-restage) and writer, energy (pre-restage); raster one row "Hover" untranslated → fixed in source (framework label), pending describe | §1 |
| 2 | TaskManager real window + agent tool-call cancel | **measured**: Tasks window live en/de (sections, real actor row, named Suspend/Resume/Cancel); job ledger + guest cancel covered by an integration test (no live spawned job observed); agent Cancel on an approval-parked call now ends it **CANCELLED in 364 ms** (was: waited out the 120 s deadline) — gateway fix + gate `live-agent-loop` **22/22** | §2 |
| 3 | ShellSync localized + persistent hub indicator | **measured**: badge online → reconnecting 5.1 s after hub SIGSTOP, rAF max gap 33 ms (no freeze), back online 3.3 s after SIGCONT; de texts live | §3 |
| 4 | one approval affordance (modal retired) | **measured**: gate (e1) Approve Once in the conversation affordance (countdown 120), (e2) Deny → PERMISSION_DENIED channel=shell, (i18n) en — 22/22 on serve 6580; de proven by G10's run of the same gate (`wp-g10/generated/live-agent-loop-de-2.txt`) | §4 |
| 5 | tablet auto breakpoint, 3 viewports | **measured**: desktop/mobile/tablet detected, no horizontal overflow, every footer control reachable (tablet footer labels compacted) | §5 |
| 6a | dag horizontal two-finger pan | **measured**: two-point touch pan +90 px → camera x −82.6 (zoom 1.1), vertical +70 px → y −63.6 = −70/1.1 | §6 |
| 6b | Home table empty after sign-in | **two causes**: (i) C10 F3 — Home's unauthenticated `foldDirectoryEvents` retired, sealed pages are the only writer: remote create → row 244 ms after the hub answered, own create 513 ms, reload → rows in 4.3 s, 0 refusals (**measured**); (ii) ≥ 9 hub spaces faulted the window (`nodes 129 > 128`) → windowed table (UI contract `Table`/`TableRow`, SDK kit on the tree-window ledger, React grid on the tree-window observer) **landed + tested**, live 30-space proof pending W2's space rebuild | §6 |

## 0. Inherited state (truth vs audit)

The S11 audit (`📓️audit-s11-os-frontend.md`) repeats G5's 2026-09-2x findings; several are stale against the tree:

| audit claim | truth in the tree (read 2026-09-25 00:5x) |
|---|---|
| `MutationKind::label()` is 2690 hard-coded English strings; history panel English-only | **stale**: U3/U3b moved all 2795 sites to `LocalizedLabel::native(en, de)`; React `🏛️ShellHost` resolves `historyEntryLabelText(entry.label, uiTerminology, uiLocale)` at render time. The real gap is the *quality* of the German (generated from a term glossary) — §1 |
| TaskManager built but never mounted | **stale**: U1 mounted it as `os.task-manager` (bottom-right dock + palette `os.openTaskManager`) — but it only showed actors and said "no runtime attached" because no host passed a registry. The React plugin runtime **does** own an `ActivationRegistry` (module-private `getActivationRegistry`) — U1's "no ActivationRegistry" note was wrong. Jobs/installs/tool calls were not listed anywhere — §2 |
| no cancel on agent tool calls | **stale**: U1 added `cancelToolCall` (bridge frame tag 10 → gateway job registry) |
| `ShellSync` status text English; no persistent hub indicator | **mostly stale**: U1 localized the status words and mounted `HubConnectionIndicator` in the footer. Residual: the popover title `{cardKind} backbone` was an English literal, and the badge ignored the shell's own session link (a signed-in shell with no attached document read `offline`; a hub shortage was invisible until a document socket dropped) — §3 |
| tablet is a manual override only | **stale**: U2 added `UI_TABLET_MAX_WIDTH_PX = 1023`, `useUiDevice()`, `selectUiDevice(state, measured)` — needs a live 3-viewport check — §5 |
| two approval surfaces | **true**: AP1 §6.5 — the modal `AgentApprovals` veil blocked the inline chat decision — §4 |

## 1. Mutation label localization

### 1.1 Measured truth (`🐍️` → `wp-u5/u5-label-audit.py`, capture `generated/u5-label-audit.json`)

2800 trait-anchored `fn label(&self) -> …LocalizedLabel` sites in `✏️s` + `🧰️framework`; 2659 distinct (crate, en, de)
templates (`generated/u5-label-pairs.txt`). Every site carries a German cell — no `String` path, no English fallback exists.
But the German was produced by U3's term glossary + compounding rules, and reading all 2659 pairs found it is frequently
**not German a domain expert would write**:

| defect class | examples (before) |
|---|---|
| symbols glued into compounds (58) | `Bemessungsaxialkraftn_Ed [kN] auf {} ändern`, `Referenzprimärenergienbedarfq_p`, `NetzuRL`, `DokumentuRI` |
| wrong domain term | Eurocode `action` → `Aktion` (is *Einwirkung*), `shear force V_Ed` → `Schubkraft` (is *Querkraft*), pile `shaft/base resistance` → `Schacht-/Basiswiderstand` (is *Mantelreibung/Spitzenwiderstand*), table `column` → `Stütze` (is *Spalte*), DTP `story` → `Geschoss` (is *Textfluss*), machining `stock` → `Bestand` (is *Rohteil*), mp4 `sample` → `Probe`, JSON `member` → `Glied`, LiDAR `points by return` → `Punkte durch Rückluft`, SHGC → not `g-Wert` |
| verbs parsed as nouns | `Seal run` → `Siegellauf`, `Switch space alternative` → `Schalterraumalternative`, `Touch artifact` → `Berührungsartefakt`, `collapse page size` → `Einklappenseitengröße`, `Sign out` → `aus signieren` |
| missing genitive / word order | `Griff "{}" Griffart auf "{}" ändern`, `Ebene "{}" sichtbar auf {} setzen`, `Kante "{}" gesperrt ändern` |
| `(s)` plural hack (10) | `{} Objekt(s) verschieben`, `Gruppe {} Knoten(s) in Ebene #{}` |
| English side not prose | 484 stdio sites were kebab op ids (`set-snapshot`, `remove-topic`); en1992/en1995/din4108 English was a field-name dump (`Change a c mm2 to {:?}`) |
| wrapper labels | 18 `stdio.semio` envelope leaves (`apply-cad` …) labelled every edit with the bare subset name (`cad`/`cad`, `graph`/`Graph`) |

### 1.2 What landed (01:15)

* **Handcrafted table** `wp-u5/u5-de-corrections.txt` — ~930 template rewrites across 60 crates, written by reading every
  pair (energy/EnergyPlus, Eurocode/DIN norms, OOXML/PDF/glTF/media codecs, architecture programming, DTP layout, FEM, …).
  Where the English was a field dump it carries the English too (`⟸`).
* **Codemod** `wp-u5/u5-apply-de-corrections.py` (one-off, ticket folder): region-anchored exactly like U3, span-keyed,
  literal-only; asserts every rewritten literal keeps its placeholder sequence and is a valid Rust string body; a
  correction matching no site aborts the run. Kebab English → sentence case (`set-snapshot` → `Set snapshot`, acronym
  table). Run: `sites 2774, corrected 1313, kebabEnglish 523, changedFiles 1649, errors 0` (`generated/u5-apply-run.json`).
  One stale row (`splice`) was dropped because a peer renamed that leaf to `replace-byte-range` minutes earlier.
* **Manual edits** `wp-u5/u5-manual-label-edits.py`: the 18 `apply-*` envelope leaves now delegate to the wrapped
  subset mutation's own localized label (`protocol::SemanticMutation::label(&self.mutation)`); the 10 `(s)` labels pick
  singular/plural per count (`1 Objekt verschieben` / `{count} Objekte verschieben`).
* **Rebuild request** appended to `wp-w1/requests/u5.txt` (W2's post-H9 full rebuild picks the literals up for every crate
  it compiles after 01:15).

### 1.3 Compile-atomic proof

`wp-u5/u5-check-sweep.sh` — native `cargo check -p … --keep-going` over all 105 touched crates in 14 batches of 8, one
cargo at a time (`generated/u5-check-batch<N>.txt`, summary `generated/u5-check-sweep.txt`): **14/14 EXIT 0, 0 errors**
(02:32). Post-sweep audit (`generated/u5-label-audit-after.json`): 2698 ok; the 76 "camel" hits are symbols/proper names
kept on purpose (`V_Ed`, `glTF`), the 26 "other" are the envelope delegations.

### 1.4 Live en/de history rows (`wp-u5/u5-history-locale-probe.mjs`)

Per plugin: spawn from Home, drive one real mutation + undo/redo through the Actions rail, read every History row, switch
the SHELL locale to German in Settings, read the same rows again without dispatching.

| plugin | row (en → de) | run |
|---|---|---|
| dag | Add Node → Knoten hinzufügen | r3 (restaged guests, `generated/u5-history-r3.json`) |
| block | Add Handle Kind → Griffart hinzufügen | r3 |
| gis | Add Feature → Objekt hinzufügen | r3 |
| note | Add Block → Block hinzufügen | r3 |
| layout | Add Page → Seite hinzufügen | r3 |
| draw | Add Layer → Ebene hinzufügen | r3 |
| forms | Add Step → Schritt hinzufügen | r3 |
| writer | Set Text → Text festlegen | r2 (`generated/u5-history-r2.json`; not spawned in r3) |
| energy | Create zone → Zone anlegen | r2 (not spawned in r3) |
| raster | Add Layer → Ebene hinzufügen, Set Composite Viewport → Komposit-Ansichtsfenster festlegen, **Hover → Hover** | r3 |
| puzzle | Add Node → Knoten hinzufügen (+ Undo/Redo, panel and window rows) | `wp-u5/u5-puzzle-history-probe.mjs`, `generated/u5-puzzle1.json` (07:3x; r3's miss was a Settings boot without a language control) |

Every other row (Toggle Panel, Activate Window, Clear Selection, Undo/Redo) re-rendered in German. The one untranslated
row is the framework's own hover interaction (`🛂️manifest/🦀️.rs` `INTERACTION_HOVER_ACTION_ID`): German "Hover" →
"Darüberfahren" (`cargo check -p semio-framework` green); descriptors carry it after W2's next describe (request filed).

## 2. Progress + cancellation UI

**Landed** (React host):

* `🔌️PluginRuntime/💼️job-ledger/🟦️.ts` (new, React-free) — the live spawned-job ledger `driveSpawnedJob` keeps: one row per
  job it drives (plugin, actor, kind, admitted steps, whether the guest reported progress, start time, `cancelling`),
  step updates coalesced to ≤ 1 publish / 250 ms, open/cancel/close immediate. `cancelSpawnedJobV1` sends `cancelJob`
  through the job's own actor ingress (cooperative: the guest frees the id, its next `step-job` answers `job.unknown`, the
  drive delivers that as the job's `job-completed` failure — the ledger only records that a human asked).
* `pluginRuntimeActivationRegistryV1()` exposes the runtime's real `ActivationRegistry` (never creates one).
* `🧵️TaskManager`: `TaskManagerWindow({ sources })` — **Running tasks** (spawned jobs, plugin installations, agent tool
  calls; each a `role=progressbar` with localized `aria-valuetext` "128 steps · 12 s" and a named Cancel button, disabled
  while cancelling) above the **Actors** table (suspend/resume/cancel). The window starts the registry's metrics publisher
  while mounted and stops it when closed. en + de labels (`os.taskManager.tasks.*`).
* `🏛️ShellHost`: stable `TaskManagerSourcesV1` (ledger + installs from `pluginStatusById` + running tool calls from the
  agent conversation); cancel routes job → `cancelSpawnedJobV1`, install → that plugin's `AbortController`, tool call →
  `agentBridge.cancelToolCall`. The unused `activationRegistry` prop (U1 placeholder nobody passed) is removed.

**Tests**: `🧵️TaskManager` 22 + `💼️job-ledger` 4 (new suite) — fixture `🧵️TaskManager/🧫️fixtures/🏃️running-tasks.json`
replayed, ARIA read back through Testing Library role queries (third-party oracle); `🔌️plugin-runtime` integration test
"lists a driven job in the task-manager ledger and routes a cancel to the guest's own cancel-job" (fake shard, 3/3 green,
`generated/u5-vitest-plugin-runtime-jobs.txt`). Renderer typecheck 0 errors (`generated/u5-typecheck-renderer-4.txt`).

**Live** (`generated/u5-live-tasks.json`, `u5-tasks-en.png`, `u5-tasks-de.png`): Tasks window sections
"Running tasks / Actors" ↔ "Laufende Aufgaben / Akteure", empty state "No task is running." ↔ "Es läuft keine Aufgabe.",
real actor row `space#1 | space | Maintenance | Active` ↔ `Wartung | Aktiv`, buttons "Suspend/Resume/Cancel: space#1" ↔
"Anhalten/Fortsetzen/Abbrechen: space#1". A spawned fem model listed its actor (`fem#3`) but ran no retained job during
the probe, so no live job row was observed (`generated/u5-live-jobs.json`).

**Agent tool-call cancel — defect found and fixed.** Live gate run 1 (`generated/u5-live-agent-loop-en.txt`): Cancel on the
running `action_invoke` row turned it `cancelling`, but the call — parked on a destructive-verb approval — ended only at the
120 s approval deadline as `APPROVAL_REQUIRED` (the gate passed it anyway). Cause: `ApprovalCoordinator::resolve_by_shell`
polled for a decision until its deadline and never looked at the tool call's own job, which `AgentCancel` flips.
Fix (gateway, `semio-framework-os-mcp`):

* `📣️notify` `active_request_cancel_requested()` — whether the `tools/call` running on this thread was asked to stop
  (`notifications/cancelled` for its request, or a cancel of any job minted under it — the chat Cancel);
* `🛡️policy` `ApprovalResolution::Cancelled`; the shell-lane wait checks it every poll;
* `🔀️dispatch` `SettledApproval::Cancelled` → decides the parked handle `false`, answers `CANCELLED` with the approval handle;
* tests: `a_cancelled_tool_call_ends_its_parked_shell_approval_instead_of_waiting_out_the_deadline`,
  `outside_a_request_scope_no_call_reads_as_cancelled` — policy suite 28/28 (`generated/u5-test-os-mcp-policy.txt`);
* gate (c) tightened: clicks only the `action_invoke` row (`data-semio-agent-chat-tool`, new attribute on the chat Cancel
  button — run 2 had clicked an already-answered search row) and requires `CANCELLED` within 30 s.

Gate run 3 (`generated/u5-live-agent-loop-en-3.txt`): **22 passed, 0 failed** — (c) `clicked=inv_8 state=cancelling
settledMs=364 code=CANCELLED`.

## 3. Connection status

* `🔄️ShellSync`: popover title `{cardKind} backbone` (English literal) → `ui.sync.backboneFile|Folder|Remote` (en + de).
* `hubConnectionSummaryV1(statuses, session, link)` now folds the shell's own session link (`verifying | reachable |
  unreachable` from the directory session refresh loop's `onDegraded`) with the documents: `local` (no hub configured),
  `signedOut`, `live · N peers`, `reconnecting` (a verified session that stops answering — a short shortage the loop rides
  out on bounded backoff, nothing else stops), `connecting`, `online` (signed in, hub reachable, nothing attached — used to
  read a false `offline`), `offline`. `ShellHost` tracks `hubSessionRefused` (hub pronounced the session over / human
  cancelled) so that case offers sign-in instead of a never-ending `reconnecting`.
* Law fixture `🔄️ShellSync/🧫️fixtures/📶️hub-connection-summary.json` (10 cases) replayed; `ShellSync` suite green.

**Live** (`generated/u5-live-hub.json`, `u5-hub-reconnecting.png`, `u5-hub-reconnecting-de.png`): signed in → "Hub
connection: online"; hub process SIGSTOP → "reconnecting…" after 5078 ms while the page kept painting (967 rAF frames, max
gap 33 ms); SIGCONT → "online" after 3313 ms; German: "Hub-Verbindung: online" / "verbinde erneut…" (4820 ms).

## 4. Agent approvals

* `🤖️AgentApprovals`: the modal `Dialog` is **retired**. `AgentApprovalAffordance` is the ONE affordance per approval,
  rendered as the approval's row in the agent conversation: verb, capability, description, target, change summary, who
  asked, risk, polite live countdown, three decisions with the shared `framework.approvals.<decision>.<id>` ids (the wgpu
  twin's scheme), accessible names carrying the capability; a resolved row keeps its record and says how it was decided
  (`decidedDeny|Once|Session`, en + de). `section tabIndex=-1 aria-labelledby` so focus can land on the group, never on a
  decision.
* `AgentApprovalsNotice` (footer, every device): "N agent approvals are waiting" + Review — decides nothing, reveals the
  chat panel and moves focus onto the oldest affordance; announces a new request in its live region.
* `ShellHost`: a newly arrived approval reveals the chat panel (without stealing focus); `revealAgentApproval` for Review.
* `💬️AgentChatPanel` renders the affordance instead of its own copy (duplicate countdown/decision code deleted).
* Gate `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` reads the unified affordance (`surface=conversation`).

**Live** (gate run 3 on serve 6580, note spawned in the real `s` host): (e1) `affordance=conversation countdown=120`,
Approve Once → the destructive `deleteSelection` commits; (i18n) en "Approve Once"/"Deny"; (e2) Deny →
`PERMISSION_DENIED channel=shell`; (e3) silent client → typed `APPROVAL_REQUIRED`. German: G10's 05:48 run of the same gate
on the same affordance, (i18n) de "Einmal genehmigen"/"Ablehnen" PASS.

## 5. Devices (`wp-u5/u5-live-probe.mjs devices`, `generated/u5-live-devices.json`, `u5-devices-*.png`)

| viewport | detected | touch | horizontal overflow | footer controls clipped / unreachable |
|---|---|---|---|---|
| 1440×900 | desktop | no | none (scrollWidth 1440) | 0 / 0 of 9 |
| 375×812 | mobile | yes | none (375) | 0 / 0 of 2 (mobile chrome) |
| 768×1024 | tablet | yes | none (768) | 0 / 0 of 9 |

Defect found at 768: the footer's panel-tab labels pushed Tasks/History off the bar. Fix: `PanelTabBar` gained
`compactLabels` (label becomes `sr-only`, the icon stays, the accessible name is unchanged), `ShellHost` sets it on tablet
and omits the empty presence bar there; measured after the fix as above.

## 6. Small defects

### 6a. dag horizontal two-finger pan

`🗺️surface/🕸️node-graph/🦀️.rs`: `GraphHost::wheel_screen/plan_wheel` take `delta_x` and pan `x − delta_x / zoom` (was
dropped); the wasm binding passes it; the wgpu renderer's call site updated. Native check + unit tests green
(`generated/u5-test-surface-wheel.txt`, new `graph_host_wheel_screen_pans_horizontally_by_delta_x_over_zoom`); wasm32
check green; `@semio-tech/framework-surface-rs:wasm` rebuilt through the wasm mutex (06:10–06:12, EXIT 0). Stale docstring
in `🕸️NodeGraph/🟦️.tsx` ("the dag binding currently drops deltaX") corrected.
**Live** (`wp-u5/u5-dag-pan-probe.mjs`, `generated/u5-dag-pan1.json`): a CDP two-point touch pan (constant finger distance)
over the dag canvas: +90 px horizontal → camera x 0 → −82.6 (one pinch notch to zoom 1.1 was recognised at touch start),
y unchanged; +70 px vertical → y −63.6 = −70/1.1, x unchanged.

### 6b. Home's own space table empty after sign-in

Probes: `wp-u5/u5-home-live-probe.mjs` (sign in, remote create by a second client of the same human, own create through the
Create Space dialog, reload), `wp-u5/u5-home-reload-probe.mjs`.

**Cause 1 — C10 F3, fixed.** Live directory events reached Home as `foldDirectoryEvents`, classified
`BatchOnlyPendingRewrite`, so every live fold was refused (`generated/u5-home-before2.json`: `input #3 foldDirectoryEvents
refused: dispatch-failed … BatchOnlyPendingRewrite`). Migrating it would have been wrong: the fold advances the projection
cursor past the sealed receipt, so the next authenticated page (`after == receipt cursor`) answers the non-retryable
`s.home.directory-event-page-frontier-race`, and the raw fold bypasses the hub's per-user filtering. The worker's
acknowledged stream already turns every live event into a wake → next sealed page → `applyDirectoryEventPage`. So the raw
writer is **retired** (one-off codemod `wp-u5/u5-retire-home-fold.py`): Home's `foldDirectoryEvents` action + command
module, `HomeConfigMutation::FoldDirectoryEvent`, `HomeViewCommand::FoldDirectoryEvents`, the retained-limits fixture row
and schema route (18 → 17), the taxonomy member; `ShellHost` and the wgpu shell relay raw events only to the Space index
(the one app that still folds them); the `home-host-panel-owner` gate in `📜️script.ts` asserts the action is gone (its stale
batch-only list — `applyDirectoryEventPage`/`createStudio` are migrated — corrected). Checks: `cargo check -p
semio-s-artifact-space-home --lib --tests` and `-p semio-s-plugin-space --lib --tests` green; `cargo test -p
semio-s-artifact-space-home --lib --features component-app-assembly` 92/94 — the 2 reds are peers' (below); wgpu renderer
check green. W2's 05:58 restage carries it (descriptor regenerated 05:15 without the Home action).
**Live on the restaged guests** (`generated/u5-home-restaged.json`): signed in → hub rows at once; a space created by a
second client → its row 244 ms after the hub answered; own Create Space → row in 513 ms; reload → rows back in 4.3 s; zero
fold refusals, frontier races or faults.

**Cause 2 — a table could not hold more than 8 author rows.** With ≥ 9 hub spaces the Home surface faults:
`retained surface render fault … s-home-main … nodes: 129, max_nodes: 128` and the window keeps its previous paint (only the
local demo row) — "empty table while the overlay lists spaces". Re-measured on a fresh hub with 30 spaces and the old guest
(`generated/u5-home-before.json`): same fault, 1 row. An author row cost 12 nodes (row + 6 cells + 5 action buttons), the
header 8, the chrome 5; `UI_DOCUMENT_NODES = 128`. The SDK kit also capped a table at 32 rows (`TABLE_WINDOW_ROWS`).

**Fix — one windowing mechanism for trees and tables** (landed; live proof waits on W2's space rebuild):

* **UI contract** (additive, both languages): `Component::Table(TableProps { label, columns, actions_label, window:
  Option<TreeWindow> })` and `Component::TableRow(TableRowProps { cells, row_actions })` — the header lives on the table,
  cells and row actions are PROPS, so a table costs `1 + materialised rows` records, the tree cost model exactly; the table
  carries the same `TreeWindow` stamp a tree section does. Rust: builders `table`/`table_row`, limits, accessibility (`grid`,
  `row`, rows focusable, table named), typed copy/compare/retirement rosters, reconciler census, wgpu mapping (a table is a
  stack, a row a button reading its cells joined, primary action = row activation). TS: generated projection
  (`@semio-tech/ui-contract-rs:generate` law 86 types, `typegen_export` green), retained wire decoder, text accounting,
  accessibility twin, `UiDocumentStore` text bytes.
* **Language-neutral fixtures**: conformance case `🧩️component/📊️table` (a window rows 12–13 of 40) read by the Rust
  conformance laws (26/26 with accessibility + limits) and by the Interpreter's corpus law (64/64); the typed-retirement
  roster (`🧩️components.json`, `componentVariants`) carries both new variants; Ajv validates the corpus catalog (64 cases).
* **Plugin SDK**: `TableWindowKit::render_rows(windows, label, columns, actions_label, entries, row)` builds only the host's
  slice of `entries` on the SAME `TreeWindows` ledger the tree panels spend (`claim_window`/`sliced`/`stamp`/
  `tree_window_rows`), `table_window_row(key, cells, actions, activate)` and `table_row_action(icon, label, action)` build one
  row record. The fixed 32-row `TableRowsView`, its retire arena and the reactor's arena ladder are gone (rows are ordinary
  built nodes retired by the built-node ladder). Laws: one node per row, row actions/activation as props, exactly the host's
  window (`offset 200 rows 20` of 500 → 20 rows, ledger debited 21), a 10 000-row first paint stays at one default viewport
  (5/5, `generated/u5-test-sdk-table.txt`).
* **Apps**: Home editor + viewer and the Space index editor + viewer render through the windowed kit (`HomeTableLabels.table_name`
  en/de); Home rows open on Enter/double-click (`openSpace` activation) and keep their role-gated actions. Home window tests
  21/21, Space window tests 7/7.
* **React host**: `TableView` (Interpreter) renders `role=grid` with `aria-rowcount`/`aria-colcount`, a header row of
  `columnheader`s, `row`s with `aria-rowindex` at their LOGICAL index behind the tree windows' own spacers
  (`treeWindowDomAttributes`, `treeWindowSpacerRows`), and reports through the SAME `useTreeWindowObserver` — so the body's one
  scheduler asks the guest for the rows the viewport shows. Keyboard (ARIA grid, one tab stop): Up/Down, PageUp/PageDown,
  Home/End over the whole logical extent (scrolls unstreamed rows into view and focuses them when they arrive), Enter/Space =
  row activation, Right/Left into the row's actions (named "open: Studio 07"), Escape back. A polite status reads "Rows 13–14
  of 40" / "Zeilen 13–14 von 40" (`ui.host.tableRowRange`, en + de). `ShellHost` now gives WINDOW bodies the tree-window
  channel too (`TreeWindowContext` per window `bodyKey`; the scheduler refreshes `windowBodies` for them). Interpreter suite
  125/125 incl. 4 table laws (Testing Library role queries as the oracle); renderer typecheck 0 errors.
* Native checks green: `semio-framework-ui-contract`, `-ui-runtime`, `-ui` (wgpu-engine), `semio-framework-plugin`,
  `semio-s-artifact-space-home`, `-space-space`, `semio-s-plugin-space`, `semio-framework-os-renderer-wgpu`. W2 request filed
  (space guest rebuild + describe); no WIT/owned-ABI, pack-schema or codec-hash change — but the UI contract crate is linked
  into every guest, so any guest compiled after ~07:10 has new wasm bytes.

## 7. Honest gaps

* **Windowed table live proof** (6b cause 2): implemented and tested; the ≥ 25-space scroll-to-last proof runs once W2 restages the space guest (baseline captured: 30 spaces fault with the old guest).
* **wgpu table**: renders the materialised rows as activatable rows (cells joined, primary action) — no column layout, no row-action strip, no window requests yet (WG lanes).
* **Pre-existing reds, not mine** (same set on the baseline with my contract edits reverted, `u5-baseline-toggle.py` in the
  scratchpad): `semio-framework-ui-contract --lib` binding-copy / component copy+compare / typed + built retirement / document
  assembly laws ("did not retire exact owners", arena poisoned) and the runtime's `runtime_tree_retirement_*` twins — the
  close ladders no longer finish under the laws' byte grants. The Rust conformance, accessibility, limits and typegen laws are
  green; the new fixture rows ride the same (currently red) retirement laws.
* **Approval affordance after a cancel**: the gateway ends the call `CANCELLED` but sends no `ApprovalResolved`, so the
  shell's affordance stays until its own countdown and a late decision is ignored. The bridge has no "withdrawn" outcome;
  adding one touches the gateway⇄shell wire and both shells (G10/WG lanes).
* **No live spawned-job row** in the Tasks window (no retained job ran during the probe); covered by the integration test.
* **History en/de not read live** for puzzle; raster "Hover" fixed in source, visible after W2's next describe.
* **MutationKind labels** reach History rows only for contributed mutations; ordinary rows show the action definition label
  (both localized).
* C10 F1 (Create Space under the Actions chip) left to S15 (chip hit-testing is theirs).
* Peer reds seen, not mine: `retained_config_cancel_and_cleanup_respect_the_production_grant` (test expects `Blocked` for an
  undersized close; S8 made `close_step` release per page), `direct_owner_descriptor_surfaces_and_catalog_correspond`
  (in-flight `change-catalog-generation` outcome classes).
* HubSignIn buttons read "Sign in Sign in" in `innerText` dumps — not investigated (likely visible text + a visually
  hidden label).

## 8. Files changed

* Labels: 1649 `fn label` sites in 105 crates (`generated/u5-apply-run.json`, `generated/u5-check-batches.json`), 18
  `✉️base/🧬️schema/🧬️mutations/*apply-*` delegations, 10 plural labels (`wp-u5/u5-manual-label-edits.py`);
  `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (Hover).
* Tasks/jobs: `🔌️PluginRuntime/💼️job-ledger/🟦️.ts` (new), `🔌️PluginRuntime/🟦️.tsx`, `🧵️TaskManager/🟦️.tsx`,
  `🧵️TaskManager/🧫️fixtures/🏃️running-tasks.json` (new), `🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx`,
  `🧑‍🎨engine/🧪️tests/💼️job-ledger/🟦️.ts` (new), `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`,
  `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`.
* Shell: `🏛️ShellHost/🟦️.tsx`, `🔄️ShellSync/🟦️.tsx` + `🧫️fixtures/📶️hub-connection-summary.json` + tests,
  `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, `🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx`,
  `🔗️AgentBridge/🟦️.tsx`, `🤖️AgentApprovals/🟦️.tsx` + tests, `💬️AgentChatPanel/🟦️.tsx` + tests, `🕸️NodeGraph/🟦️.tsx`.
* Gateway: `🌉️mcp/📣️notify/🦀️.rs`, `🌉️mcp/🛡️policy/🦀️.rs`, `🌉️mcp/🛡️policy/🧪️tests/🔬️quick/🦀️.rs`,
  `🌉️mcp/🔀️dispatch/🦀️.rs`, `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`.
* dag: `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`, `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`.
* Home: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️.rs`, `…/✳️any/✏️editor/🦀️.rs`, `…/✏️editor/🎚️config/🦀️.rs` + tests,
  `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `…/🎮️commands/🏷️rename-space/🧪️tests/🔬️unit/🦀️.rs`, `…/🎮️commands/🌱create-space/🦀️.rs`,
  `…/👁️viewer/🦀️.rs` + tests, `…/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`, `…/✳️any/🧬️schema/🔣️.json`,
  removed `…/🎮️commands/📇️fold-directory-events/`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`; `📜️script.ts`;
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
* Windowed table: `🖱️ui/🧬️contract/🧩️component/🦀️.rs`, `🏗️builder/🦀️.rs`, `🛡️limits/🦀️.rs`, `♿️accessibility/🦀️.rs` + `🟦️.ts`,
  `🧾️typed/🦀️.rs`, `🪞️copy/🦀️.rs`, `⚖️compare/🦀️.rs`, `♻️retirement/🌳️typed/🧱️component/🦀️.rs` + `🧩️components.json` + `🧫️fixtures/🔣️.json` +
  `🧬️schema/🔣️.json`, `🧬️schema/🦀️.rs`, `🧪️tests/🧬️typegen-export/🦀️.rs`, `🧪️tests/🔬️conformance-unit/🦀️.rs`,
  `🧪️tests/🔬️conformance-corpus/🟦️.ts`, `🧫️fixtures/🧪️conformance/📇️catalog.json` + new `🧩️component/📊️table/`,
  `🧵️retained/🟦️.ts`, `🧵️retained/📦️wire/🧾️typed/🟦️.ts`; `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, `🛂️manifest/🟦️.ts`;
  `🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs`; `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`; `🔌️plugin/🦀️.rs` (table kit),
  `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, `🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`, `🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`;
  space: `🫀️core/🦀️.rs`, Home editor + viewer main windows and tests, Space index editor + viewer main windows, roots and
  tests; React: `🗣️Interpreter/🟦️.tsx` + new `🧪️tests/📊️table/🟦️.tsx` + corpus law count, `📃️UiDocumentStore/🟦️.tsx`,
  `🏛️ShellHost/🟦️.tsx` (window-body tree-window channel), `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`.
  One-off codemods: `wp-u5/u5-table-window-{contract,sdk,apps,react}.py`; probes `u5-home-table-probe.mjs`,
  `u5-puzzle-history-probe.mjs`.
* Requests: `wp-w1/requests/u5.txt` (label pass, surface wasm, space guest + describe, Hover label, windowed table → space guest).

## 9. Processes started

| what | pid | port | capture |
|---|---|---|---|
| hub hold (`wp-u5/u5-hub-hold.ts`, catalog-less, credential sign-in on) | hold 67070, os-hub 67072 | 8080 | `wp-u5/state-8080/` |
| `dev s` serve, first (`S_OS_PORT=6580 S_HUB_URL=…8080`) | 67409 / 67450 | 6580 | `generated/u5-serve.txt` |
| `dev s` serve, restarted for W2's restage (+ `S_AGENT_BRIDGE_DIR=wp-u5/rendezvous`) | 19731 / 19749 | 6580 | `generated/u5-serve-2.txt` |
| local-only serve (stopped at once: it began building its own `os-hub`) | 31222 tree | 6581 | `generated/u5-serve-6581.txt` |
| cargo check sweep | 84658 → 24163 | — | `generated/u5-check-sweep.txt` |
| surface wasm build (wasm mutex) | — | — | `generated/u5-surface-wasm-build.txt` |

`wp-u5/target`, the copied hub binary and the hub data dir are deleted.
| hub hold, second run (fresh data, ada/bo provisioned with `os-hub credential set`, 30 spaces "Studio 01…30") | hold 71359, os-hub 71366 | 8080 | `wp-u5/state-8080/` |
| `dev s` serve, third run | 71361 tree | 6580 | `generated/u5-serve-3.txt` |

All stopped at the end of the turn; the hub data dir is deleted again (re-seed: `os-hub credential set` + `u5-hub-seed.ts`).
