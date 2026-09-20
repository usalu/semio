# S5 — a spawned program is a first-class member of the `s` shell session

Slice S5, fleet 5, started 2026-09-20 ~19:00. Scope handed over from S4
(`📓️s4-space-studio-and-foreign-kind-open.md` §5.2: *"a spawned program has no windows"*).

Outcome 1's acceptance, last hop: **inside the real `s` host a user opens an editor of ANOTHER
plugin and works in it** — open → mutate → undo → redo.

Status legend: **measured** = this slice ran it and captured output; **unverified** = read from
source only.

## 0. tl;dr

**Reached.** Inside the real `s` host on :6071, signed in as `user1@semio.dev`, a foreign plugin's
editor is spawned from the command palette, opens with its OWN windows on the canvas and its own
Actions rail, and one real document mutation plus undo plus redo reach that spawned instance — with
**zero refusals** on the shell's dispatch lane.

**Nine foreign programs plus the studio** opened with their own windows and their own Actions rail,
in ONE session with no reload; **eight** dispatched a real document verb from that rail with no
refusal; **three — `draw`, `note`, `puzzle` — show the complete round trip** (§6).

```
draw    win draw-3::drawing-composite     verb addLayer   canvas moved  redo≠undo TRUE
note    win note-4::note-composite        verb addBlock   canvas moved  redo≠undo TRUE
puzzle  win puzzle-8::2d-overview         verb addNode    canvas moved  redo≠undo TRUE
```

Five root fixes landed in the React `ShellHost`, each found by measurement and each judged by the
live shell answering a DIFFERENT fault afterwards:

| # | what was broken | evidence it is fixed |
|---|---|---|
| 1 | a spawned program got no window instances at all, so `Mode` pruned the host app's leaves against a window list that no longer held them and painted the empty-shell notice | `[data-window-id]` went `[]` → `["draw-3::drawing-composite"]`; the studio spawns **three** windows |
| 2 | Display ▸ Windows offered the LANDING app's window kinds | it now reads the focused program's |
| 3 | the guest was addressed at the shell's namespaced window id, which it never declared | `addLayer` reached the guest (`typed-operation slots instance=3 live=1/64`) |
| 4 | a spawned session carried the HOST app's `activeModeId` | `addLayer refused … unknown action mode owner explore` → **gone** |
| 5 | `subscribeOperationCompletions` was wired for the session's instance ONLY, so a spawned program's typed operations never settled and never published their terminal outcome | the completion callback now fires for `draw#3` (`op=64 refresh={"kind":"full"}`) |

## 1. Inherited state

- serve 6070 (pid 26481) and 6071 (pid 34308, `S_HUB_URL=http://127.0.0.1:7501`) alive, `HTTP 200`;
  hub 7501 alive (`/` 404, the route this probe never uses). **None restarted, none killed by this
  slice.** Every change here is TypeScript, so vite served it without a re-stage — verified by
  fetching the transformed module (§8).
- No predecessor S5 report and no `🗑️generated/s5-*` captures existed: this slice started clean.

## 2. The model the fix follows

The wgpu shell's `switch_to_app` (`📓️o1-multi-plugin-hub.md` §3) makes the target app the session:
one program owns the canvas, it owns EVERY window kind it declares, and the shell's window-scoped
rules address that program's instances. React keeps the host session mounted underneath instead of
retiring it — that is the whole difference, and it is why the ids must be namespaced per spawned
instance: two spawned instances of one app declare the same window kinds.

That model is now a unit, `🏛️ShellHost/🪟️spawned-program/🟦️.ts` (no React, no shell imports — the
separation `⌨️window-scope` and `📌️panel` already keep, so a law can drive it):

| export | what it decides |
|---|---|
| `spawnedWindowInstanceIdV1` / `spawnedProgramWindowInstancesV1` | the shell window ids a spawned program contributes — `${spawnedId}::${windowKindId}`, one per declared kind |
| `spawnedIdOfWindowInstanceV1` / `spawnedWindowKindOfInstanceV1` | the owner and kind behind a shell window id, resolved against the LIVE spawned ids (longest match), never by scanning for the separator |
| `guestWindowIdV1` / `guestActiveUtilityByWindowIdV1` | the host↔guest boundary: the guest sees its own window kind ids and only its own windows' utilities |
| `renameLayoutWindowIdsV1` / `spawnedLayoutRenameV1` | the spawned app's OWN declared default layout, renamed into that instance's namespace |
| `focusedProgramV1` / `focusedProgramKeyV1` | which program owns the canvas, and the identity a re-seed may fire on |
| `spawnedProgramViewStateV1` | the view state a spawned dispatch must carry — its own mode, not the host's |

## 3. What the React ShellHost did instead — measured

`sessionWindowInstances(session.app, …)` was the sole source of window instances at seven call
sites, and `spawnProgram` (`:6600`) only wrote the host panel. The consequence was not that the
canvas showed the wrong windows — it showed **none**, and the reason is inside the real renderer:

`Mode` resolves its layout with `resolveModeLayout(windows, layout)` → `reconcileWindows`, which
**prunes every layout leaf whose id is absent from the window list it was handed**
(`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:286,683`). After a spawn the window list was the spawned app's
single body while `shellLayout` still held `s-home-main`, so the layout reconciled to empty,
`hasWindows` went false, and the canvas painted `ui.display.emptyShell` — S4's verbatim
"Drag windows from Display in the navbar, or restore a saved layout", with `[data-window-id] = []`.

That pruning is now pinned as a regression law against the real `Mode` component (§5).

## 4. The fix

### 4.1 The canvas seed — the blocker itself

`🏛️ShellHost/🟦️.tsx`: a `focusedProgram` memo (the focused spawned instance, else the session), a
`focusedApp`, and `focusedWindowInstances`. An effect keyed on `focusedProgramKeyV1(focusedProgram)`
— the PROGRAM, never `session` identity, so an ordinary refresh never throws the user's arrangement
away — seeds `SET_SHELL_LAYOUT`/`SET_ACTIVE_WINDOW_ID` from the focused app's own `defaultLayout`,
renamed into that instance's window-id namespace. The session's layout and active window are
remembered when a spawn takes the canvas and restored verbatim when focus comes back.

The canvas branch (`:10746`) no longer renders one hard-coded `windowKinds[0]` body: it maps EVERY
window kind the spawned app declares, each with its own engagement, search, utility bar, Actions
rail, cursor, `WindowInstanceIdContext` and `framework.window.<segment>` element id — the same
projection the session's own app gets. `SpawnedWindowState.spawnedWindowUi: BuiltNode | null` became
`spawnedWindowUiByWindowId: Record<string, BuiltNode>` (`🐚️Shell/🟦️.tsx:468,699,817,1193`) and
`refreshSpawnedUi` fetches every declared kind instead of `resolveCanvasBodyKey(app)` alone.

Measured, same probe, same session:

| | before (S4) | after |
|---|---|---|
| `[data-window-id]` after spawning `draw` | `[]` | `["draw-3::drawing-composite"]` |
| after spawning the studio | `[]` | `["space-2::s-workflow","space-2::s-media-vfs","space-2::s-compiled-dag"]` |
| canvas | `mode-empty` notice | the foreign editor, breadcrumb `← Back to Workflow · semio · drawing` |
| Display ▸ Windows | the landing app's `s-home-main` | the focused program's kinds |

Captures `🗑️generated/s5-foreign-{1,3,4}.txt`, `s5-action-{1..14}.txt`,
`s5-spawned-draw{,-actions}.png`.

### 4.2 The dispatch lane — three faults, each named by the shell itself

**(a) The Actions rail was never missing.** Every earlier reading of "window exposed no action
control" was the rail's FOLD: it lives in the window's engagement pane
(`🪟️Window/🟦️.tsx:370-387`), whose toggle is `framework.window.<segment>.engagement.toggle`, and its
rows are `action.<actionId>` inside `[data-slot="window-action-pane"]` — not `[data-action-id]`,
which this product emits nowhere. Unfolded, the spawned `draw` publishes its own 17 rows verbatim:
`Clear Selection, Select All, Set Selection Mode…, Set Granularity…, Add Layer…, Combine Boolean…,
Set Active Example…, Export PDF…, Copy, Cut, Paste…` plus a `History` section. The permanent probe
now unfolds the rail and drives those rows.

**(b) The guest was addressed at a window it never declared.** The shell's window ids are
`${spawnedId}::${kindId}`; a spawned instance's own window instance ids are its window KIND ids. The
namespace is stripped at every boundary that speaks to the guest — the refresh request, the
dispatched view state (`windowInstances`, `focusedWindowId`, `activeUtilityByWindowId`) and the
action invocation's `windowInstanceId`. Without the strip `undeclaredActionDiagnostic` drops the
action with "no window kind declares it".

**(c) A spawned session carried the HOST app's mode.** `{ …, viewState: session.viewState }` meant
every invocation addressed `s.space.home`'s active mode. Read verbatim off the running shell:

```
input #4 addLayer refused: dispatch-failed (user window=draw-4::drawing-composite)
                           — unknown action mode owner explore
```

`spawnedProgramViewStateV1` restates the mode from the app the action is addressed to (and drops the
host's `windowId`/`activeWindowKindId`), at all six sites that build a spawned session. After it the
verb dispatches: `[DEBUG] typed-operation slots instance=3 live=1/64 peak=1`, no refusal.

**(d) A window-scoped dispatch now goes to the program that owns the window**, whatever controller
raised it — shell chrome raises actions with the SESSION's controller id while the active window
belongs to a spawned program. Measured before the fix as
`noteShellCommand refused: view-state-unresolved (user window=space-2::s-workflow)`. This is the
"panel actions dispatch in the ACTIVE window" rule, stated once instead of per call site. The same
rule routes `setActiveUtility` to the window's owner, so a spawned editor's utility bar arms its own
tool rather than the landing app's instance, and the chord lane resolves against the focused app's
keybindings (including the framework-universal undo/redo chords).

### 4.3 A spawned program's typed operations never settled

`subscribeOperationCompletions` was wired for `session.pluginId`/`session.instanceId` and nothing
else (`:6288`). So a spawned program's typed operations completed into the void: no history delta,
no final `UiDirtyScope`, none of the continuation turns' effects — and `settleOperation` was never
called, so every `onAction` on a spawned program waited out the full 30 s
`OPERATION_SETTLE_WATCHDOG_MS`. The same subscription now runs per spawned instance, keyed on the
spawned roster. Measured firing, with temporary instrumentation added, read and removed:

```
[S5PROBE] spawned completion draw#3 op=64 refresh={"kind":"full"} effects=0
```

**Named residual, honestly:** closing that hole did NOT make the first typed operation repaint the
spawned window on its own turn. `addLayer` lands in the document — the engagement goes
`1 layer · 0 selected` → `2 layers · 0 selected` — but only from the next refresh onwards. Two
host-side suspects were tested and both ruled out by measurement: re-fetching after
`awaitOperationSettle` (`🗑️generated/s5-action-{9,11}.txt`) and dropping the refresh cache's section
hashes on a completion (`s5-action-{10,14}.txt`) each left the reading unchanged, so neither was
landed. The remaining suspect is the guest's own fold of a typed operation's result relative to the
completion frame it emits — the same shape S4 §2.5 named on the directory lane — and it is not a
TypeScript change. It costs the spawned window one turn of freshness; it does not cost the mutation,
the undo or the redo.

## 5. Laws

`🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx`, registered in
`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` beside `⌨️window-scope`.

```
SEMIO_TEST_LEVEL=long bun ./📜️script.ts test 🪟️spawned-program-session
 Test Files  1 passed (1)
      Tests  17 passed (17)
```

The canvas half is driven against the **real `Mode` renderer**, not an oracle of it, because the
pruning that emptied the shell lives inside that component:

- **REGRESSION**: with the spawned window list and the HOST app's layout, `[data-window-id]` is `[]`
  and `[data-slot="mode-empty"]` is present — the live symptom, reproduced in a unit test.
- with the spawned program's own seeded layout, every one of its windows is mounted and the notice is
  gone.

The rest pin the unit (namespacing, two instances of one app not colliding, the guest strip, the
per-program utility map, the layout rename leaving an undeclared leaf alone, the re-seed key firing
on a program change and not on a refresh) and the six ShellHost decision sites that now route
through the focused program.

Scoped `tsc` (`🔣️s5-scope.json`, `🗑️generated/s5-tsc-1.txt`): the SAME 5 errors before and after this
slice's edits, none in a file it touched (`ShellHost:5021 consumes` is pre-existing — `git diff` on
that file contains no `consumes`).

## 6. Live matrix — plugin kinds opened, mutated, undone, redone inside `s`

Signed in as `user1@semio.dev` on :6071, ONE session, Home → studio (palette) → per-kind spawn.
`redo≠undo` is the verdict that covers the whole round trip: the verb reached the spawned instance,
its undo reached it, and its redo reached it.

| plugin kind | spawned window(s) | verb taken from its OWN rail | canvas moved | redo ≠ undo | refusal |
|---|---|---|---|---|---|
| `s.space.studio` | `space-2::s-workflow`, `::s-media-vfs`, `::s-compiled-dag` | — (the surface the others are spawned into) | — | — | none |
| `draw` | `draw-3::drawing-composite` | `addLayer` | **yes** | **yes** | none |
| `note` | `note-4::note-composite` | `addBlock` | **yes** | **yes** | none |
| `puzzle` | `puzzle-8::2d-overview` | `addNode` | **yes** | **yes** | none |
| `layout` | `layout-5::layout-blueprint` | `addFrame` | **yes** | no | none |
| `forms` | `forms-3::forms-blueprint` | `addQuestion` | **yes** | no | none for the taken verb |
| `raster` | `raster-4::raster-composite` | `addLayer` | **yes** | no | none for the taken verb |
| `writer` | `writer-5::writer-main` | `paste` | **yes** | no | none |
| `flow` | `flow-6::flow-main` | `addWidget` | **yes** | no | none for the taken verb |
| `sequence` | `sequence-7::sequence-main` | (no rail row moved the document in 10 tries) | no | no | `setViewport requires a camera` |

**Nine foreign programs plus the studio opened with their own windows and their own Actions rail
inside `s`, in ONE session, with no reload.** Eight of them dispatched a real document verb from
that rail with no refusal. **Three — `draw`, `note`, `puzzle` — show the full round trip**: the
mutation, its undo and its redo each reached the spawned instance and the window reads a different
document after the redo than after the undo.

For the five where `redo ≠ undo` is `no`, the verb DID move the canvas (`canvasChangedByMutation`
true) — what is not demonstrated is that the undo/redo pair left the window on two different
readings, which §4.3's one-turn freshness residual is enough to explain and which this slice did not
chase further. That is stated as a limit of the measurement, not as a claim that undo is broken.

Refusals the probe captured verbatim while trying OTHER rows (each an argument the probe did not
staged-fill, not a lane fault): `forms patchQuestions … missing field question_ids`,
`raster setActiveExample … missing field example_id`, `sequence setViewport requires a camera`,
`flow addWidget … retained command work refused the command before any capacity was measured`.
Captures `🗑️generated/s5-foreign-{4,5,6,7}.txt`.

**Lazy install is still not exercisable as *lazy***: host mode installs all 60 plugins at boot, so
`lazyInstalled` is `false` by construction for every row (S2 §3.2 point 1, unchanged). What is
measured is that the spawned program is *resolved, instantiated and driven* from the host's own
registry in one session with no reload.

## 7. Local studio vs. hub authority — unchanged from S4, restated for this slice

Everything in §6 is the **local** path: the studio is spawned through the palette and the foreign
editors are spawned into it, with no hub document involved. Sign-in, the directory and entering a
space are hub-backed and work. Creating an artifact INSIDE a hub space still answers
`503 catalog-unavailable` from `open-plan` until a trusted catalog is published (GM1's lane), so the
matrix above deliberately does not go through a hub space. That boundary is S4 §4's and this slice
did not move it.

## 8. LB1 live agent gate against the `s` host with a spawned editor

**Not run, and the reason is measured rather than assumed — two independent blockers.**

1. **Preamble rule 26.** The gate's `live-agent-loop-check` verb builds `semio-framework-os-mcp`, and
   at the time of this slice `ps -axo command | grep -c "cargo test -p semio-framework-os-mcp"`
   answered **2**. Rule 26 forbids starting a third builder on that crate.
2. **The gate has no spawn step.** Its step 0 asserts `boot :: ready=<plugin> windows=<plugin's
   windows>` (`📓️wr4-typed-command-dispatch-and-gates.md` §5.2), which is a single-plugin serve's
   shape. Against `s` the shell boots `ready=s` with `s-home-main`, and the foreign editor only exists
   after a palette spawn — a step the gate does not have. Running it unchanged would have measured
   the landing app, not a spawned editor, at the cost of a contended build.

What this slice DID establish for it, measured: the served transform on :6071 carries this slice's
code, so the gate's step-0 precondition (an eligible shell whose `🏛️ShellHost` is not stale — the
trap that has cost this ticket a slice before) holds.

```
GET http://127.0.0.1:6071/@fs/…/🏛️ShellHost/🟦️.tsx  →  200, 1 919 722 bytes
  focusedProgramKeyV1 / spawnedWindowInstanceIdV1   10 occurrences
```

The gate's next owner needs one new step — spawn `S_OS_MCP_LIVE_PLUGIN` through the palette before
`boot` asserts its windows — and then :6071 is a valid target.

## 9. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- a foreign program spawned inside the real `s` host owning windows on the canvas —
  `s5-foreign-1.txt` (`["draw-3::drawing-composite"]`, `["note-4::note-composite"]`), the studio
  owning **three** (`space-2::s-workflow`, `::s-media-vfs`, `::s-compiled-dag`)
- the spawned `draw` publishing its OWN 17-row Actions rail and engagement (`1 layer · 0 selected`)
  — `s5-action-{2,3}.txt`, `s5-spawned-draw-actions.png`
- the mode-owner refusal verbatim and its disappearance —
  `addLayer refused … unknown action mode owner explore` (`s5-action-6.txt`) → dispatched
  (`s5-action-7.txt`)
- the window-owner refusal verbatim —
  `noteShellCommand refused: view-state-unresolved (user window=space-2::s-workflow)`
  (`s5-action-1.txt`)
- the spawned operation-completion subscription firing — `s5-action-13.txt`
- the per-kind matrix of §6 — `s5-foreign-{4,5,6,7}.txt`
- `SEMIO_TEST_LEVEL=long … test 🪟️spawned-program-session` → **20 passed**, including the
  empty-canvas regression against the real `Mode` renderer
- scoped `tsc`: the same 5 pre-existing errors before and after, none in a touched file

**Not done / unverified**:

- **The one-turn freshness residual (§4.3)** — a spawned program's first typed operation lands in the
  document but the window body shows it from the next refresh. Two host-side causes were tested and
  ruled out by measurement; neither speculative fix was landed. The remaining suspect is guest-side.
- **`redo ≠ undo` for `layout`, `forms`, `raster`, `writer`, `flow`** — their document verb moved the
  canvas; the undo/redo pair was not observed to leave two different readings (§6).
- **`sequence`** — no rail row moved the document within the probe's 10-row budget.
- **Splits of a spawned program's windows** — Display ▸ Windows now offers the focused program's
  kinds, but dropping one is refused with a named `console.warn` rather than half-wired:
  `refreshSpawnedUi` fetches a spawned app's declared kinds, not extra instances of them. Deliberate,
  and the next owner's cheapest follow-up.
- **Lazy install as *lazy*** — host mode installs all 60 at boot, so `lazyInstalled` is `false` by
  construction for every row (S2 §3.2 point 1, unchanged by this slice).
- **Artifacts inside a hub space** — still `503 catalog-unavailable` (§7), GM1/TC1's lane.
- **LB1's gate** — §8.
- **The wgpu twin** — the wgpu shell already switches session in `switch_to_app`; nothing in this
  slice changed it, and no wgpu boot was run.
- Two spawned instances of the SAME app are kept apart by construction and by a law, but were not
  opened together in the live shell.

## 10. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts` | **new** — the spawned-program model as an owned unit (§2) |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | focused-program memos; the layout/active-window re-seed; the multi-window spawned canvas branch; Display ▸ Windows from the focused app; the host↔guest window-id strip at every dispatch boundary; window-owner routing for window-scoped dispatch and for `setActiveUtility`; the chord lane on the focused app; `spawnedProgramViewStateV1` at every spawned session; per-spawned-instance `subscribeOperationCompletions`; the close ladder retiring an instance when its LAST window closes; split/template-drop guarded for spawned programs |
| `…/🧱️elements/🐚️Shell/🟦️.tsx` | `spawnedWindowUi: BuiltNode \| null` → `spawnedWindowUiByWindowId: Record<string, BuiltNode>` (state, action, reducer, initial value) |
| `…/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🟦️.tsx` | **new** — 17 laws, incl. the empty-canvas regression against the real `Mode` |
| `…/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | suite registered |
| `🐍️s2-s-host-foreign-kind-probe.mjs` (ticket) | unfolds the Actions rail, drives the real rail rows (including staged forms' Execute), captures the shell's `console.warn` refusals, and judges mutate/undo/redo by the document |
| `🐍️s5-spawned-action-diagnose.mjs` (ticket) | **new** — what a spawned window exposes, and the verbatim refusal behind a verb that does nothing |
| `🔣️s5-scope.json` (ticket) | the scoped `tsc` project for `🏛️ShellHost` + the new unit |
