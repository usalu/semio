# React Generate-Chord & Restage — lane `react-generate-chord-restage` (2026-09-14)

Closes both red steps `📓️react-end-to-end-verification-2026-09-13.md` left open —
`gaps/generate-chord` and `gaps/inspection-edit` — at the layers the coordinator named, with laws in
Rust and TypeScript over one shared fixture, plus every fix-forward the peer lane
`26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` needed before the generation3d app would restage at all.

It also separates a third failure that was hiding inside `inspection-edit` and gives it its own red
line (`inspection-preview-rearm`, §9) rather than a green one it had not earned. The battery therefore
finishes at **15/16**, not 16/16, and §9 says exactly whose the sixteenth is.

- Shared fixture: `🧰️framework/…/🧱️elements/🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json`
- Owned unit (TS): `🧰️framework/…/🧱️elements/🏛️ShellHost/⌨️window-scope/🟦️.ts`
- Rust twin: `🧰️framework/…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`//#region ⌨️WindowScope`)
- New probes: `🐍️generate-activation-recon.mjs`, `🐍️actions-pane-recon.mjs`, `🐍️preview-rearm-recon.mjs`
- Restage/battery logs: `🗑️generated/generate-chord/`

## 0. Files

Created:
- `🧰️framework/…/🏛️ShellHost/⌨️window-scope/🟦️.ts` — the owned unit
- `🧰️framework/…/🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json` — the shared fixture
- `🧰️framework/…/🧑‍🎨engine/🧪️tests/⌨️window-scope/🟦️.ts` — the TS law
- `🐍️generate-activation-recon.mjs`, `🐍️actions-pane-recon.mjs`, `🐍️preview-rearm-recon.mjs`
- `📓️react-generate-chord-restage-2026-09-14.md` (this report)

Updated:
- `🧰️framework/…/🏛️ShellHost/🟦️.tsx` — seven seed sites, `effectiveModeLayoutRef`, the keyboard loop
- `🧰️framework/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — the Rust twin + `dispatch_app_keybinding`
- `🧰️framework/…/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` — three Rust laws
- `🧰️framework/…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `📦️packages/🟦️typescript/📜️script.ts`,
  `📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — the `window-scope-check` gate
- `✏️s/…/✏️editor/🎭️modes/✏️edit/🦀️.rs`, `🎭️modes/🧬️generate/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/🦀️.rs`,
  `✏️editor/🦀️.rs` — the `previewEval` mode references
- `✏️s/…/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs`,
  `🧪️tests/🔬️status-contract/🟦️.ts` — the cancel→abort migration
- `✏️s/…/🧫️fixtures/🛑️preview-cancel.json`, `⌨️keyboard-reachability.json`, `🚪️io/📄️document-surface.json`
- `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the window-scoped-chord law
- `🐍️react-gap-probe.mjs` — the four precondition repairs and the `inspection-*` split

---

## 1. TL;DR

**`mod+shift+g` (Add Generation) was dead in BOTH modes for two independent reasons, one per layer,
and a third defect was hiding behind them.**

1. **The React dock never activated a window.** Every layout seed dispatched
   `SET_ACTIVE_WINDOW_ID: null` — at boot, on every mode switch, on every named-layout apply. Measured
   with the app untouched: `activeWindow: null`, `mod+shift+g` → `invoked: []`. The wgpu shell has
   always seeded `active_window_id` from the layout; React never did.
   **Fixed** — seven seed sites now route through `dockSeedActiveWindowIdV1`.
2. **An app-wide chord resolved against the focused window kind alone.** Even with
   `procedural-main` active, `addGeneration` is declared only on the Generations window, so the chord
   had no owner and died silently.
   **Fixed** — `resolveKeybindingTargetWindowV1` targets the window of the ACTIVE MODE that owns the
   verb, focused or not, activates it, and dispatches there; when no mounted window owns the verb the
   chord is a **hinted** no-op (`⌘⇧G · Add Generation — not available in this mode`, en + de), never a
   dead key.
3. **Exposed by (1): one chord, two owners.** `build_definition` mints `mod+alt+arrowright` for the
   injected `toolRunStep` of every app that declares a tool run — the SAME chord as the navbar's
   `ui.shell.mode.next`. With no window ever active the app half never resolved it; the moment a mode
   seeds an active window, the app half swallowed every mode switch (it folded the Actions pane and
   staged `toolRunStep` instead). **Fixed** — the shell's own accelerator-carrying chrome chords are
   now reserved from the app-keybinding loop, the rule the wgpu shell already applied.

`gaps/generate-chord` is green at runtime, run after run: `activeWindow: "generation3d-generations"`,
`chordInvoked: ["addGeneration", …]`. `gaps/inspection-edit` is green too — the number control mounts,
its edit dispatches `patchFlowWidgets`, and the document's `height` really moves 6 → 7 — and its law
(`inspector_slider_control_rides_on_a_tree_row`) RUNS for the first time, because the crate's `--lib`
test binary compiles again.

**The battery is 15/16, not 16/16, and the one red is named and owned.** `gaps` now reports eleven
steps; ten are green and the eleventh, `inspection-preview-rearm`, is the preview-evaluation pipeline
failing to re-evaluate after a document patch — a peer-owned defect their own unit laws already name
(§9). It was previously hidden inside `inspection-edit`, which conflated "the inspector edits the
document" with "the preview re-evaluates" and so reported a working inspector as broken.

---

## 2. Root causes, with file:line

### 2.1 The dock never activated a window (framework)

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1526` —
`resolveFrameworkLayoutSeed` is documented as resolving a layout "**without inferring window focus**".
Nothing downstream inferred it either: every one of ShellHost's seven seed sites dispatched
`SET_ACTIVE_WINDOW_ID: null`, and `Mode` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1233`) renders
`active={activeWindowId === activeId}`, so `[data-slot="window"][data-active="true"]` existed only
after a user click.

Measured (`🐍️generate-activation-recon.mjs`, `🗑️generated/generate-chord/recon2/recon.json`):

| reading | before |
|---|---|
| `boot` | `activeWindow: null` in EDIT mode |
| `edit-chord-untouched` | `mod+shift+g` → `invoked: []` |
| `generate-entered` | `activeWindow: null`, three windows mounted |

The recon also proves the dock's own activation path was never broken: a real click on a tree row, a
window body or a dock-tab label DOES move `data-active` in generate mode. The earlier
`focusedGenerations: true, activeWindow: null` reading came from a `click({force:true})` landing on an
overlay, not from a dock defect.

### 2.2 An app-wide chord resolved against the focused window only (framework)

`🏛️ShellHost/🟦️.tsx`, `handleAppKeydown`: `actionById` was built from
`session.app.windowKinds.find(kind => kind.id === focusedWindowKindId)?.actions` and a chord whose
action was absent did `continue` — no dispatch, no diagnostic, no hint. `addGeneration` is declared on
`GENERATION_3D_PLAY_WINDOW_GENERATIONS` alone (`✏️editor/🦀️.rs`, `window_kind_action_refs`) while
`.keybinding("mod+shift+g", "addGeneration")` is app-wide, so the chord had an owner that the resolver
could never reach.

### 2.3 One chord, two owners (framework)

`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1899` `TOOL_RUN_STEP_CHORD = "mod+alt+arrowright"` versus
`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:186`
`"ui.shell.mode.next": "mod+alt+arrowright"`. `build_definition`
(`🔌️plugin/🦀️.rs:5649`) mints a keybinding for every injected `toolRun*` action and copies the action
onto every window kind, so once a window is active BOTH owners answer the same keystroke.
`ui.window.focus` / `toolRunFinalize` (`mod+shift+enter`) collide the same way.

Measured after fix (1) and before fix (3) — `🗑️generated/generate-chord/actions2/recon.json`,
step `after-generate`: the mode hop left the flow window's Actions pane **unfolded with 37 rows**,
because the app half had handled `mod+alt+arrowright` as `toolRunStep` and expanded its staged form.

### 2.4 The peer's `previewEval` rename blocked the restage and the crate's test binary

`🔌️plugin/🦀️.rs:5429` refuses `app-definition.invalid: app … tool previewEval is not referenced by any
mode`. `✏️editor/🦀️.rs:2255` declared the tool with the comment "referenced by no mode because the
preview surface starts it" — a statement the builder does not accept. Three restage attempts had
failed on this since 02:12 (`📓️react-end-to-end-verification-2026-09-13.md` §6).

---

## 3. Fixes

### 3.1 Owned unit — `🏛️ShellHost/⌨️window-scope/🟦️.ts` (new)

React-free, shell-import-free, mirroring the `🔀️surface-switch` unit this ticket already established:

| export | what it decides |
|---|---|
| `modeLayoutStacksV1` | flattens a mode layout into its tab stacks, in layout order |
| `dockSeedActiveWindowIdV1` | the window a freshly seeded layout opens active, or `null` for "keep what you have" |
| `resolveKeybindingTargetWindowV1` | `focused` / `owner` / `unowned` for one chord in one mode |
| `chordCarriesAcceleratorV1`, `reservedShellChordsV1` | which chords the shell's chrome keeps out of the app loop |
| `KEYBINDING_UNOWNED_LABEL`, `keybindingUnownedTextV1` | the en/de hint, authored in the unit that decides the refusal |

`dockSeedActiveWindowIdV1` answers `null` when `activeWindowId` already names one of the layout's
windows — which is what keeps an explicit canvas-background deactivation from being re-seeded, and
makes the seed idempotent (asserted in both languages).

### 3.2 ShellHost wiring

- Seven `SET_ACTIVE_WINDOW_ID: null` sites → `seededActiveWindowId(seeded.modeLayout)`
  (boot/host seed, primary-app seed, session-switch refresh, role-switch `seedLayout`,
  `applyNamedLayout`, `applyModeChange`, `publishPreparedArtifactOpening`).
- `effectiveModeLayoutRef` carries the layout the dock is actually rendering into the keyboard loop,
  so `mounted` is the ACTIVE MODE's windows in layout order — not every declared window kind.
- `handleAppKeydown`: reserved-chord skip → `resolveKeybindingTargetWindowV1` → activate on `owner` →
  dispatch with `args.windowId = targetWindowId` (the channel `windowActionInvocation` and the
  undeclared-action gate already read) → `showTransientNotice(..., KEYBINDING_UNOWNED_CODE)` on
  `unowned`.

### 3.3 wgpu twin

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` gains `mode_layout_stacks_v1`, `dock_seed_active_window_id_v1`,
`resolve_keybinding_target_window_v1`, `chord_carries_accelerator_v1`, `reserved_shell_chords_v1` and
`keybinding_unowned_text_v1`. `dispatch_app_keybinding` now resolves the owner over
`self.dock.window_instances()` instead of assuming the active window declares the verb, activates the
owner, and sets `self.error` to the localized hint when no mounted window owns it. (The wgpu shell
already seeded `active_window_id` at boot, so it needed no dock-seed change.)

### 3.4 App half — the chord table names its owner

`🧫️fixtures/⌨️keyboard-reachability.json`: every `windowScoped` row now carries `ownerWindowKindId`
and `liveInModes`, and the note states the new rule (owner-reachable, hinted where unowned) instead of
the old "live only while that window kind has focus".

---

## 4. Fix-forwards for the peer lane `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`

Every one of these was a half-edit of theirs that blocked the generation3d app; all are forward onto
their contract, none reverts a hunk.

| # | what was broken | fixed forward as |
|---|---|---|
| 1 | `app-definition.invalid: … tool previewEval is not referenced by any mode` — the restage blocker | `🎭️modes/✏️edit`, `🎭️modes/🧬️generate` and the viewer's `🎭️modes/👁️view` now reference `previewEval` in their `tools`, because each mounts a preview window that starts the run; the `.tool()` comment at `✏️editor/🦀️.rs:2255` restated |
| 2 | `👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs` used the deleted `CancelPreviewEval` / `cancel_preview_eval_for` / `PREVIEW_CANCEL_ACTION_ID` | `FlowEvalRelease` + `release_invocations_for` + `FlowEvalSession::cancel_preview_evaluation`; `TOOL_RUN_ABORT_ACTION_ID` |
| 3 | `preview_progress_status_json_for` / `preview::render` arity (a `ToolRunView` argument) | `run_view(cancellable)` helper mirroring `🧵️preview-eval/🧪️tests/🔬️unit`'s own; `None` where a run is not implied |
| 4 | three viewer laws asserted a plugin-declared `cancelPreviewEval` COMMAND that no longer exists | rewritten to the new truth: the published verb is the framework-reserved abort the viewer EARNS by declaring the run, and the plugin declares no cancel command; the retained-route/lane assertions moved onto `flowEvalRelease`, which still owns them |
| 5 | `🧫️fixtures/🛑️preview-cancel.json` still said `cancelPreviewEval` / `declaresCancelCommand` | `toolRunAbort` / `frameworkReservedVerb`; the TS twin `🧪️tests/🔬️status-contract/🟦️.ts` updated with it |
| 6 | `🧫️fixtures/⌨️keyboard-reachability.json` bound `mod+.` to the deleted `cancelPreviewEval` | `toolRunAbort`, `framework: true`, with the minting rule spelled out |
| 7 | `🧫️fixtures/🚪️io/📄️document-surface.json` still expected `Export Document…` after the 2026-09-13 ellipsis fix (§5.3 of the previous report) | `Export Document` / `Dokument exportieren` — the shell appends the ellipsis |

---

## 5. Laws

| law | language | count | result |
|---|---|---|---|
| `⌨️ window scope` (`🧪️tests/⌨️window-scope/🟦️.ts`) | TS + Ajv oracle | 5 stack projections, 8 dock seeds, 12 chord targets, 3 hint locales, 11 reserved-chord rows, 4 hostile fixtures | ✅ |
| `a_seeded_mode_layout_always_opens_with_one_window_active` | Rust | 5 + 8 rows, each also asserted idempotent | ✅ |
| `an_app_wide_chord_resolves_to_the_window_that_owns_its_verb_in_the_active_mode` | Rust | 12 rows (7 owner hops, 3 hinted no-ops), 3 hint locales | ✅ |
| `the_shells_own_accelerator_chords_are_reserved_from_the_app_keybinding_loop` | Rust | 11 rows, 6 reserved; pins the live `TOOL_RUN_STEP_CHORD` collision | ✅ |
| `every_window_scoped_chord_names_its_owning_window_kind_and_the_modes_that_mount_it` | Rust (app) | 5 window-scoped chords × 2 modes | ✅ |
| `inspector_slider_control_rides_on_a_tree_row` | Rust (app) | 1 — **first execution ever** (§6 of the previous report) | ✅ |
| `document_rows_bind_both_framework_interaction_verbs` | Rust (app) | 1 | ✅ |
| `staged_argument_actions_declare_no_trailing_ellipsis` | Rust (app) | 1 | ✅ |
| `the_editor_binds_every_keyboard_verb_the_fixture_names` | Rust (app) | whole chord table | ✅ |
| `the_editor_declares_every_io_action_and_chord_the_fixture_names` | Rust (app) | 3 io actions | ✅ |
| viewer `status_contract_tests` (6 laws) | Rust (app) | 5 states × 3 surfaces | ✅ |
| `testGeneration3dPreviewStatusContract` | TS twin | 5 states, 3 surfaces | ✅ |
| `testGeneration3dDocumentIoSurface` | TS twin | 7 export / 7 import / 3 editor actions | ✅ |

The two shell laws run in one command each: `bun nx run
@semio-tech/framework-renderer-react:window-scope-check` (new nx target + `⚖️gate⌨️window-scope🌐️shell`
launch entry) and `cargo test -p semio-framework-os-renderer-wgpu --lib`.

---

## 6. Restage

`🗑️generated/generate-chord/restage-1.txt` — `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx
run @semio-tech/framework-os-dev:activate-generation3d-react-dev`, **exit 0, 4 m 43 s**, "Activated
generation3d react dev: 11 completed components (changed)". This is the first successful restage since
the peer's tool declaration landed; fix-forward #1 is what unblocked it.

---

## 7. Battery scoreboard

`cd <ticket> && bun 🐍️react-battery.mjs` — ONE stage, all sixteen probes, sequential.
`🗑️generated/generate-chord/battery-full-1.txt`, scoreboard
`🗑️generated/react-verify/scoreboard.json` (`2026-09-14T03:51:16Z → 04:12:06Z`).

**15 of 16 green, 1251 s, 14 page errors.**

| probe | ok | s | steps | page errors |
|---|---|---|---|---|
| boot | ✅ | 101 | 2/2 | 0 |
| journey | ✅ | 122 | 23/23 | 2 |
| interact | ✅ | 80 | 5/5 | 0 |
| generate-mode | ✅ | 35 | 11/11 | 1 |
| flow-window | ✅ | 71 | 2/2 | 0 |
| flow-wire | ✅ | 121 | 3/3 | 1 |
| flow-reorganize | ✅ | 127 | 1/1 | 0 |
| cancel-preview | ✅ | 11 | 2/2 | 0 |
| keyboard-verbs | ✅ | 27 | 5/8 (3 rows undecided by design) | 0 |
| io-surface | ✅ | 23 | 6/6 | 0 |
| export-encoding | ✅ | 20 | 2/2 | 0 |
| status-parity | ✅ | 199 | 10/10 | 2 |
| role-switch | ✅ | 35 | 7/7 | 1 |
| i18n-a11y | ✅ | 57 | 5/5 | 1 |
| customization-persistence | ✅ | 26 | 2/2 | 0 |
| **gaps** | ❌ | 196 | **10/11** | 6 |

`gaps`, step by step:

| step | ok |
|---|---|
| boot | ✅ |
| dock-resize | ✅ |
| export-after-generate | ✅ |
| wire-undo | ✅ |
| doc-panel-select | ✅ |
| **inspection-edit** | ✅ — `present:true`, `invoked:["patchFlowWidgets"]`, `height` 6 → 7 |
| **inspection-preview-rearm** | ❌ — `previewArmed:true`, payload `3644:cd770756` unchanged (§9) |
| catalogue-add | ✅ |
| actions-pane-de | ✅ |
| window-focus | ✅ — `atBoot: "procedural-main"`, previously `null` |
| **generate-chord** | ✅ — `activeWindow:"generation3d-generations"`, `chordInvoked:["addGeneration", …]` |

`journey` 23/23 and `generate-mode` 11/11 on this stage are also the runtime proof of fix (3): both
drive the navbar's `mod+alt+arrowright` mode step repeatedly, which the app half was swallowing before
the shell's chrome chords were reserved.

---

## 8. Probe repairs (`🐍️react-gap-probe.mjs`)

Three of the gap probe's steps were measuring the probe's own leftovers rather than the app. All three
are precondition repairs; not one assertion was weakened.

| step | what it was really reading | repair |
|---|---|---|
| `actions-pane-de` | the window chrome's pane toggles, covered by three side panels the fold pass never folded — a panel tab folds its panel only on a **re-press of the already-active tab**, and one pass over three tabs is three fresh picks | press each tab twice |
| `export-after-generate` | the Actions pane toggle pressed unconditionally — after an earlier step left the pane OPEN, the press FOLDED it and every row the step needed vanished | unfold only a pane that reads `data-folded="true"` |
| `inspection-edit` | the inspector's first selected node, which a wire cut two steps earlier leaves pointing at a neuron (`replace` in the `node` granularity does not clear the `handle` one) — so the panel was truthfully painting no value control for a neuron | clear the selection and re-select the slider until the inspector names it |
| `inspection-edit` (2) | **two subjects in one verdict** — the inspector's control editing the document, and the preview re-evaluating. A working inspector read as broken and the actual owner of the failure was invisible | split into `inspection-edit` and `inspection-preview-rearm`, each keeping its whole assertion |
| `inspection-preview-rearm` | "the preview re-evaluated" as a change in the payload's **length**, which two different meshes of the same topology share (a column whose height changes keeps every digit count), measured against an EMPTY preview that cannot express anything at all | compare a per-payload DIGEST, and restore an armed preview first by switching the example away and back (re-picking the option already shown dispatches nothing — measured) |

`🐍️actions-pane-recon.mjs` is what measured the first two (`🗑️generated/generate-chord/actions*/`),
`🐍️preview-rearm-recon.mjs` the third (`🗑️generated/generate-chord/rearm*/`).

---

## 9. Handed to the peer lane, not fixed here

**The edit preview does not re-evaluate after a document patch, and the run restarts in a storm.**
Two readings of one pipeline:

- `inspection-preview-rearm`: with the preview armed (3 meshes) and `patchFlowWidgets` landing
  (`height` 6 → 7 in the document), the published payload is **byte- and digest-identical** before and
  after — `3644:cd770756` both sides. Not a weak predicate: the digest was added precisely to rule
  that out.
- One `gaps` session logged **1053 `toolRunStart` invocations in 242 s**
  (`🗑️generated/react-verify/gaps/console.txt`) with the edit preview stuck at `[]`; a healthy session
  of `🐍️preview-rearm-recon.mjs` over the same gestures logs 9–12.

The peer lane's own unit laws already name both — `re_pushing_an_unchanged_closure_owes_the_settled_run_nothing`
and `a_later_set_contributions_owes_the_settled_run_a_restart` are red with `left: ["toolRunStart"]`,
and `generation_preview_is_one_app_transient_shared_by_two_generation_windows` is red with
`transient=0`. It is guest-side in `🧵️preview-eval`. **Not this lane's to fix** — their migration is
live in that file, and a second author in it would collide.

The keyboard route was cleared of suspicion first: undoing the same wire cut by CHORD and by the
Actions-pane ROW both restore the wire and both leave the preview armed
(`🗑️generated/generate-chord/rearm2/`, `rearm-chord/`), so the chord path this lane rewrote is not
what empties it.

---

## 10. What is NOT claimed

- **Nothing about the wgpu renderer's runtime.** The Rust twin compiles, its three laws pass, and
  `dispatch_app_keybinding` is wired to it — but no wgpu browser run was taken in this lane.
- **16/16 was the goal and 15/16 is the result.** The sixteenth is `gaps`, red on exactly one of its
  eleven steps, for a defect this lane measured precisely and does not own (§9). No assertion was
  removed to get closer to green: the `inspection-*` split gave the preview its OWN red line instead
  of letting it hide inside the inspector's.
- **The generation3d crate's `--lib` suite is not green as a whole.** After the fix-forwards it
  COMPILES (it did not before) and every law in §5 passes, but ~17 further tests are red inside the
  peer's own live migration (`toolRunStart` bridging, `flowEvalTick` re-arm counts, tessellate
  re-arm, example-switch arming). Those are their lane's, not fixed here, and none of them is a
  regression from this lane's edits.
- **`@semio-tech/framework-renderer-react:typecheck` is not clean**, and was not before: the same
  16 pre-existing ShellHost errors and the repo-library errors remain. This lane added none (verified
  by diffing the error list before and after) and removed two by typing `retainedPaste`.
- **The `previewEval` mode-tool references are a contract reading, not a decision made with the peer.**
  If their intent is that a surface-started run should be expressible without a mode reference, the
  builder's rule — not these three mode definitions — is where that belongs.
- **The two new recon probes are diagnostics, not gates.** They are kept because they are what
  measured §2.1 and §2.3; nothing runs them automatically.
