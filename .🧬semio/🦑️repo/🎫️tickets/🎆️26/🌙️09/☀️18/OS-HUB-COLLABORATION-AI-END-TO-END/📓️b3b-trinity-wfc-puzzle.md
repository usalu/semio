# B3b — 🔱️trinity, 🀄️wfc, 🧩️puzzle: React boot + interaction bar

Slice B3b of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END.
**Read `# Session 5` (bottom) first — it supersedes §1.** On the tightened bar (boot → example →
mutation → undo → **redo** → zero faults) the session-5 measurement is **10 of 10 PASS, zero
console faults anywhere, and zero revertible chrome rows anywhere** — the last two after
re-activating the 🔱️trinity and 🀄️wfc guests so they pick up the §3.2 fix (§S5.8). The §3.2 defect is
measured on both sides of its own fix: the same artifacts logged **6 of 6 / 9 of 9 / 4 of 4** chrome
rows as undo targets on the pre-fix guests and **0** on the rebuilt ones.

*(Session 3 headline, kept for history: 8 of 10 artifacts clear the four-clause bar; the other 2
clear every clause except one boot-time console error each.)*

Started (this worker) 2026-09-19 ~03:40. Two predecessors died with their parent turn; `🗑️generated`
was wiped both times, so the inherited artefacts are only the three scripts
(`📜️b3b-activate.sh`, `📜️b3b-serve.sh`, `🐍️b3b-interaction-probe.mjs`) — the probe's own comments
carry the surviving measurements (wfc `change-seed` journalling latency, the camelCase
`…​.action.<id>.execute` trigger spelling).

**Interaction bar** (what each row below must clear): default example renders → one real mutating
action dispatched through the UI appends an APPLIED ledger entry AND raises the uncommitted-edit
count → `undo` retires it → zero console faults/guest traps in the whole run.

## 1. Status table

| # | artifact | variant | port | boots | example renders | mutating action | undo | faults | bar |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 🔱️trinity jack | `trinity-jack` | 6054 | ✅ 16 s | ✅ 3 panes, 23 actions | ✅ `patchNodes` → `rename-node id="7dc5b737-…" name=probe-renamed`, edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 2 | 🔱️trinity rewriting | `trinity-rewriting` | 6056 | ✅ 16 s | ✅ 6 windows, 22 actions | ✅ `setParameter` → `change-parameter-binding key=label new-value="probe-label"`, edits 0→1 | ✅ 1→0 | 1 at BOOT (§3.1b, fix built, awaiting re-activation) | ⏳ |
| 3 | 🀄️wfc bitmap | `bitmap` | 6041 | ✅ 17 s | ✅ 2 canvases, 25 actions | ✅ `change-seed` → "Change seed to 1", edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 4 | 🀄️wfc grid2d | `grid2d` | 6042 | ✅ | ✅ 2 canvases, 31 actions | ✅ `change-seed` → "Change seed to 0", edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 5 | 🀄️wfc 2d | `wfc2d` | 6043 | ✅ | ✅ 2 canvases, 33 actions | ✅ `change-seed` → "Change seed to 0", edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 6 | 🀄️wfc grid3d | `grid3d` | 6044 | ✅ | ✅ 2 canvases, 32 actions | ✅ `changeSeed` (camelCase here, unlike its four siblings) → "Change seed to 0", edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 7 | 🀄️wfc 3d | `wfc3d` | 6045 | ✅ | ✅ 2 canvases, 31 actions | ✅ `change-seed` → "Change seed to 0", edits 0→1 | ✅ 1→0 | **0** | ✅ **PASS** |
| 8 | 🧩️puzzle 2d | `puzzle2d` | 6012 (live peer) | ✅ 21 s | ✅ 3 canvases, 20 actions | ✅ `addNode` → `create-node node { id=node-1 … }`, edits 1→2 | ✅ 2→1 | **0** | ✅ **PASS** |
| 9 | 🧩️puzzle 3d | `puzzle3d` | 6013 | ✅ 19 s | ✅ 2 World3d panes, 75 actions | ✅ `addObjectKind` → `create-object object { id=puzzle3d.object.9acf0e… }`, edits 1→2 | ✅ 2→1 | **0** | ✅ **PASS** |
| 10 | 🧩️puzzle 5d | `puzzle5d` | 6121 (s4; was 6014) | ✅ | ✅ 81 actions | ✅ `addNode` → `create-part part { id=part-0de797c6d221dcbb-0 … }`, edits 1→2 | ✅ 2→1 | **0** (§3.3 fixed, re-measured in S4.3) | ✅ **PASS** |

**Session-4 re-measure** (host-side §3.2 fix live, guest-side still pre-fix): trinity jack on 6120
`{"mutated":true,"undone":true,"faultLines":0,"interactionBar":true}`; puzzle5d on 6121
`{"mutated":true,"undone":true,"faultLines":0,"interactionBar":true}`. Rows 2 (rewriting) and 10
(puzzle5d) were the two open ones — 10 is closed, 2 is still waiting on a wasm build (S4.2).

## 2. Method

Inherited `📜️b3b-activate.sh`, `📜️b3b-serve.sh`, `🐍️b3b-interaction-probe.mjs` from the two dead
predecessors; `🗑️generated` was empty, so every measurement below is fresh.

Nine of the ten variants were already staged under
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/`
(only `puzzle5d` has never been activated), so the loop is **serve detached → probe headless**, with
activation only where the serve's own freshness report says `source-newer`. Peers hold `puzzle2d`
on 6012 (two supervisors) — reused read-only, never killed. All probes run
`--use-angle=metal`, 1600×1000 visible viewport, headless.

Pass condition per artifact = the probe's `interactionBar`: example rendered ∧ an APPLIED ledger row
appeared ∧ the uncommitted-edit count rose ∧ `undo` dropped it back ∧ zero fault lines.

## 3. Root defects found and fixed

### 3.0 Probe defect (fixed here, blocked every artifact)

Two faults in the inherited probe made every variant unmeasurable:

- **`🐍️b3b-interaction-probe.mjs:122`** — a cold variant cache re-optimizes its Vite deps on the first
  page load, and these servers run `SEMIO_VITE_HMR=0`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:151` → `hmr:false`), so Vite has
  no channel for the `full-reload` it needs to send afterwards. Every optimized-dep request then
  answers `504 (Outdated Optimize Dep)` forever: trinity-jack sat at `ready:null`, zero panes, for the
  full 247 s boot budget. The probe now issues the reload the server cannot ask for. Boot dropped
  247 s → 17 s. This is why the predecessors reported nothing.
- **`🐍️b3b-interaction-probe.mjs:182`** — arguments were selected by the row id, but
  `buildActionSections` (`🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4234`) puts
  `action.<verb>.arg.<arg>` on the `TreeDataItem` and lets `renderStagedArgControl` mint the control's
  own id, so every argument scored `absent`. The row is now the scope and the control is found inside
  it (select / `role=combobox` / input).
- **`🐍️b3b-interaction-probe.mjs:218`** — the settle waited for ANY witness change, and clicking an
  Actions row focuses its window, which the host journals as an "Activate Window" ledger row
  (`🏛️ShellHost/🟦️.tsx:10552`). The wait therefore returned on the shell's own chrome, before the verb's
  entry existed: `bitmap` and `wfc2d` scored inert at both 25 s and 75 s while the very next read showed
  "Change seed to 1" already sitting in the ledger. The settle now waits on the uncommitted-edit count,
  the one witness chrome rows never touch. **Both artifacts pass on the corrected predicate** — they
  were never broken; the probe was.
- **`🐍️b3b-interaction-probe.mjs:156`** — the History panel was clicked once with a flat 1.5 s wait.
  On `puzzle3d` (two World3d panes still settling at that moment) the footer tab had not mounted, the
  probe went on with `checkin: null` — `edits === -1` — and the whole run fell back to the unreliable
  predicate above. It now clicks until the panel is really visible. `🐍️b3b-panel-diagnose.mjs` is the
  one-off that measured this (`🗑️generated/b3b-puzzle3d-panels.txt`: the tab exists at boot, the panel
  body mounts late).

### 3.1 🔱️trinity rewriting drops the navbar example picker at boot (found, fix pending)

At boot the shell's example picker dispatches `setActiveExample`
(`🧱️elements/🛠️ShellHelpers/🟦️.tsx:373`) and the app drops it:

> semio: app "s.trinity.rewriting@1/\*#editor" dropped action "setActiveExample" dispatched from
> window kind "trinity-rewriting-lhs": no window kind declares it

`setActiveExample` is declared nowhere in
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` —
neither in `command_from_action` (`:510-525`) nor in the manifest (`:1037-1048`), while the sibling
`🔌️jack` declares it (`:1095`, `:1111`, `:1137`) and routes it to a host-applied `LoadDocument`.
The 09-17 TRINITY ticket recorded this as "the known cosmetic boot `setActiveExample`
undeclared-action"; it is not cosmetic — it is an error-level console line on every boot and it makes
the navbar example picker inert for this artifact.

**Fix (landed).** `setActiveExample` is now a first-class rewriting verb, wired the same way `jack`
wires it and the same way every other rewriting document verb is wired:

| what | where |
|---|---|
| `SetActiveExample { example_id }` command variant, `#[dsl(key = "set-active-example")]` | `…/♻️rewriting/…/✏️editor/🦀️.rs:381` |
| `command_from_action` arm (`exampleId`/`example_id`/`value`/`id`) | `…/✏️editor/🦀️.rs:520` |
| `OpBinary::TOOL_JOB_IDS` + `REWRITING_DOCUMENT_TOOL_IDS` | `…/✏️editor/🦀️.rs:422`, `:536` |
| extent + both reducers (`rewriting_document_reduce`, `Editor::handle`) | `…/✏️editor/🦀️.rs:553`, `:580`, `:884` |
| `HostOnly` publication lane (a whole-document swap, like `resetRule`) | `…/✏️editor/🦀️.rs:633` |
| `ActionDefinition` (`Mutation`, `panel-left`, category `mode`), `InteractiveJobClassification::Migrated`, `action_args` (the registered `demo` id), context-menu `mode` group | `…/✏️editor/🦀️.rs:1043`, `:1058`, `:1094`, `:930` |
| `set_active_example` command leaf — parses the demo example's own DSL and emits `Effect::LoadDocument` | `…/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs` (new), declared at `♻️rewriting/🦀️.rs:729` |

Deliberately left UNSCOPED (no `window_kind_action_refs`), which is the framework's own contract for
an app-scoped verb — see the engine-contract law "leaves app-scoped verbs on every window kind"
(`🧱️elements/🧪️tests/🔬️engine-contract/🟦️.ts:11523`).

Four unit tests added at `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:221-268`: every window kind declares the verb;
the registered `demo` id produces exactly one `LoadDocument` carrying the document the subset
registers; an unregistered id loads nothing rather than faulting; the verb is on both tool rosters.
`cargo check -p semio-s-artifact-trinity-rewriting --features component-app-assembly` clean
(`🗑️generated/b3b-rewriting-check.txt`).

**`cargo test -p semio-s-artifact-trinity-rewriting --features component-app-assembly` →
`test result: ok. 157 passed; 0 failed`** (`🗑️generated/b3b-rewriting-tests.txt`, exit 0) — 153 pre-existing
plus the four added here.

One drafting mistake worth recording, because it is a trap for the next person: the first version of
the `LoadDocument` test asserted that the app's OWN snapshot changed after the dispatch. It does not
— `Effect::LoadDocument` is host-applied, and a mounted test app has no host, so the guest snapshot
stays exactly as it was (that draft ran 156 passed / 1 failed). The guest-side contract is "one
`LoadDocument` carrying the right document", so the test now asserts the effect count, that the
document the resolver hands it equals the example's own parsed DSL, and — explicitly — that the
guest snapshot is *unchanged*, which is what makes the effect the only guest-side witness.

### 3.1b 🔱️trinity rewriting refuses EVERY whole-document load (found by the §3.1 fix, fixed)

With `setActiveExample` declared, the boot dispatch stopped being dropped and reached the host —
where it failed one layer deeper:

> `setActiveExample` refused: dispatch-failed —
> `AppChannelClient.loadDocumentArchive(s.trinity.rewriting@1/*#editor)`:
> `{"code":"artifact-store.persisted-initializer-refused", "message":"app refused the persisted
> document's retained initialization authority"}`

`TrinityRewritingPlayApp` never implemented `build_document_store_initialization_job`, and the trait
default (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31624`) **refuses the envelope
outright**. Every `Effect::LoadDocument` this app can emit therefore died at the archive-load
boundary — which means `resetRule` has never worked in the browser either, despite the 09-17
TRINITY ticket documenting it as "a genuine whole-document reset … routes through
`Effect::LoadDocument`" (`…/🎮️commands/♻️reset-rule/🦀️.rs:8`). It was only ever exercised by unit
tests, which do not cross the host's archive-load path.

**Fix:** `…/♻️rewriting/…/✏️editor/🦀️.rs:709` now returns
`semio_framework_plugin::bounded_document_store_initialization_job(envelope, REWRITE_RULE_SCHEMA, …)`
— the same generic authority `🕸️dag` (`…/🕸️dag/…/✏️editor/🦀️.rs:739`) and `💡️reasoning`
(`…/🔌️wires/…/✏️editor/🦀️.rs:519`) pair with their own custom `schema::retirement::document_store_owners()`,
which is exactly rewriting's shape. `🕸️dag`'s own docstring records the identical symptom, so this is
a recurring shape rather than a one-off.

### 3.2 `shell.windowActivate` is replayed into the guest by `undo` (framework-wide, not trinity's)

After `undo`, jack logs an **error**: the guest replays `shell.windowActivate` and the window-kind
gate refuses it `undeclared-action (guest window=trinity-jack-graph causedBy=#9)`. The shell records
window activations as history notes through `noteShellCommand`
(`🏛️ShellHost/🟦️.tsx:10552`, funnelled via `NOTE_SHELL_COMMAND_ACTION_ID`), and the wgpu twin's own
docstring (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17406`) already names this exact sequence — "React's own
`chord-undo` journals `undo` AND the guest's replay of `shell.windowActivate` (origin `guest`,
refused `undeclared-action`)". It is a framework defect, reproducible on any plugin whose undo runs
after a window activation, and it puts an error line into every artifact's run. Reported, not fixed
here: it is outside one plugin slice and would collide with peers in `🏛️ShellHost`.

**It is intermittent, not deterministic.** Three jack/grid3d runs produced it and the jack run that
finally passed showed `faultLines: 0`, so it depends on whether a window activation happens to be the
row `undo` pops. That is exactly why it deserves someone's slice: it makes any plugin's "zero console
errors" gate flaky rather than failing it outright.

### 3.3 🧩️puzzle 5d throws once at boot: no registered board session factory

`puzzle5d` boots, mutates and undoes cleanly, but logs one error at 4.7 s:

> `Error: The current app has no registered board session factory.`
> — `🧰️framework/…/🧱️elements/🖥️Board2dHost/🟦️.tsx:971`

`Board2dHost` reads `BoardSessionFactoryContext` (`:589`) and throws from its `useLayoutEffect` when
that context is empty, so a `Board2d` surface of this app mounts without its factory provider. The
2-D sibling `puzzle2d` does NOT log it. Found, not fixed: `puzzle5d` had never been activated before
this slice and this is its first live boot on record, so the ownership question (host provider vs.
the 5d app's own surface declaration) has no prior art to copy, and it is not a regression of
anything changed here.

## 4. Two things the probe measured that are NOT plugin defects

- **`patchNodes` takes node IDs, not node names.** `nodeIds="b"` (the demo node's *name*) is silently
  inert: `patch_nodes` (`…/🔌️jack/…/🎮️commands/🩹️patch-nodes/🦀️.rs:11`) filters on `node.id`, so an
  unmatched id yields `Emit::mutations(vec![])` with no fault anywhere. The fixture UUID
  `7dc5b737-3b6b-4068-b315-b7bacc91c2e1` works.
- **A `required()` select with no default disables the execute control.** Leaving `field` unstaged
  made `unresolvedActionArgs` non-empty, so `disabled` was set on the execute button; the probe's
  `force: true` click still lands in the DOM but React never fires the handler — another silent
  no-op with no console line. Both args must be staged.

Neither is a bug worth "fixing"; both are traps that cost two probe rounds each and are recorded here
so the next slice does not pay them again.

## 5. Honest gaps

1. **`🔱️trinity rewriting` is one activation away from the bar.** The §3.1b fix
   (`build_document_store_initialization_job`) is written and `cargo check`-clean, but the served
   wasm still predates it, so the run recorded above still carries the boot fault. Its mutation
   (`setParameter`) and undo are already proven on the live shell; re-activate
   `@semio-tech/framework-os-dev:activate-trinity-rewriting-react-dev`, restart the serve on 6056 and
   re-probe to close it. I did not get the activation in because the shared `semio-framework-plugin`
   crate went red under a peer's live edit (`active.faulted` unknown field, 04:16) and every wasm
   build behind it fails.
2. **`setActiveExample` is not proven end-to-end on rewriting** for the same reason — only that the
   verb is now declared and reaches the host (the refusal moved from `undeclared-action` to the
   deeper store-initializer refusal, which is the fix's whole point). The unit tests cover the
   command; the browser proof is pending the same activation.
3. **`🧩️puzzle 5d`'s board-session-factory boot error (§3.3) is found, not fixed.**
4. **`shell.windowActivate` (§3.2) is reported, not fixed** — framework scope, not one plugin's.
5. **MCP descriptors**: `🀄️wfc`'s committed `🔣️.json` already carries `artifactSchema` and is clean.
   `🔱️trinity` and `🧩️puzzle` are still the two `missing field artifactSchema` registry skips A1
   counted, and **activation does NOT regenerate them** — I verified that trinity's descriptor is
   unchanged after a successful `activate-trinity-rewriting-react-dev`. The regenerator is the
   plugin's own `describe` target (`nx run @semio-tech/trinity-plugin:describe` →
   `describePluginComponent`, outputs `🛂️.descriptor.semio` + `🔣️.json`; same for
   `@semio-tech/puzzle-plugin`). Not run here: both sit behind the same blocked wasm build as gap 1.
   Do NOT hand-edit them.
6. **`🧩️puzzle` TS diff modules** call `parsePuzzle<N>d*Patch` functions the generator never emitted
   (T4 §"✏️s — 44"). Left alone deliberately: slice **T4b** owns the generator fix and is visibly
   mid-flight in the tree (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔺️diff-parsers/` is new and untracked, the
   three `🔺️diff/🟦️.ts` barrels are modified). It does not affect any boot measured here.
7. **Launch-seed rows**: every one of the ten variants already has its React row in
   `.vscode/launch.json` (ports 6012/6013/6014, 6041–6045, 6054, 6056 — all verified against the file
   and all matching the ports probed). No row was added, so `@semio-tech/plugin-registry:generate`
   had nothing to regenerate for this slice; it ran anyway as a dependency of each activation.
8. The per-crate unit tests added here cover the rewriting fixes only. `🀄️wfc` and `🧩️puzzle` needed
   no source change, so they got no new tests.

## 6. Files changed

**Source (root fixes, slice-owned):**
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs` — NEW
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🦀️.rs` — command leaf module declaration
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — §3.1 + §3.1b
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — four new laws

**Not mine, present in the tree when I started** (a dead B3b predecessor's, inherited and now proven
at runtime by the jack row above): the three modified `🔌️jack` files — `…/✏️editor/🦀️.rs`
(`window_kind_action_refs` scoping + the `setActiveExample` option list), `…/🎮️commands/🎯️set-active-example/🦀️.rs`,
`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`. They were never exercised live before this slice because the staged
module predated them.

**Ticket folder:**
- `🐍️b3b-interaction-probe.mjs` — three probe fixes (§3.0)
- `🐍️b3b-panel-diagnose.mjs` — NEW, the one-off that measured the late History panel
- `📓️b3b-trinity-wfc-puzzle.md` — this report
- `🗑️generated/b3b-*` — every capture cited above

**Servers left running** (mine, started detached; kill by pid if the box needs the room):
6041 `bitmap`, 6042 `grid2d`, 6043 `wfc2d`, 6044 `grid3d`, 6045 `wfc3d`, 6013 `puzzle3d`,
6014 `puzzle5d`, 6054 `trinity-jack`, 6056 `trinity-rewriting`. `puzzle2d` on 6012 is a **peer's**
supervisor — reused read-only, never touched.

---

# Session 4 (2026-09-19 ~23:20 →) — the §5 gaps

Inherited state verified in the tree before starting: the §3.1/§3.1b rewriting fixes are present and
auto-committed (`SetActiveExample` at `…/♻️rewriting/…/✏️editor/🦀️.rs:383,521,554,581,876,901`,
`build_document_store_initialization_job` at `:716`, the `🎯️set-active-example` command leaf on disk);
`🗑️generated` was wiped again, so every capture below is fresh. Machine at start: load ≈ 86, 71 GiB free.

## S4.1 §3.2 root fix — chrome notes are no longer undo targets (landed; test results in §S5.1)

**What was actually wrong.** Two independent halves, both framework-wide:

1. **Every chrome note claimed to be its own inverse.** `buildNoteShellCommandAction`
   (`🧱️elements/🛠️ShellHelpers/🟦️.tsx:368`) wrote `inverseCommandId: commandId` and
   `inverseArgs: detail` on *every* note, and the guest
   (`🔌️plugin/🦀️.rs:26557` — `…get("inverseCommandId")…unwrap_or(command_id)`) synthesized the same
   self-inverse even when the shell sent none (which is what the wgpu twin and
   `shell.windowResize`/`shell.windowMove` do). `detail` says where the chrome **went**, never where
   it came from, so `{windowId: "trinity-jack-graph"}` "inverted" is the identity. Result:
   `dispatch_chrome_history_action` (`🔌️plugin/🦀️.rs:26346`) treats every window activation, resize,
   move, panel toggle and dock drag as a revertible chrome row, and the **first `undo` after any
   click pops that row instead of the user's document edit**.
2. **The host then replayed it into the guest.** `applyHostEffects`'s `replayShellCommand` branch
   (`🏛️ShellHost/🟦️.tsx:5750`) has arms for `os.directory.*`, `os.create-space-artifact` and
   `os.open-artifact*`, and sends **everything else** to `onActionRef.current({… origin: "guest" …})`.
   A `shell.*`/`os.*` id has no window kind in any app, so the gate answers
   `undeclared-action (guest window=… causedBy=#n)` — §3.2's error line. The whole shell-chrome undo
   feature has therefore never worked in the React host: `os.setThemeId`, `os.resetDock` and
   `os.resizeWindow` replays all land in the same dead branch.

**The fix (three laws).**

| law | where | what |
|---|---|---|
| A — guest | `🔌️plugin/🦀️.rs:26557` | an `InverseAction` is recorded **only** when the shell declares `inverseCommandId`; no declaration → `inverse: None` → the row is logged for the History panel but is never an undo target |
| B — host descriptor | `🧱️elements/🛠️ShellHelpers/🟦️.tsx:366` | `buildNoteShellCommandAction(…, detail?, inverse?)` — the self-inverse fabrication is gone, `ShellCommandInverse` is explicit and only a caller holding the replaced state can declare one |
| C — host replay | `🏛️ShellHost/🟦️.tsx:5843` + `:2503` | a SHELL-owned id (`isShellOwnedCommandId`) is routed through `dispatchOsCommand` via `replayShellOwnedCommandRef` and **never** into the guest; an unroutable one is one `console.warn`, not a fault |

`dispatchOsCommand` now returns whether it routed the id (`🛠️ShellHelpers/🟦️.tsx:4703`), so law C can
tell a real replay from a dead one.

Laws A+B together are what removes the error: no React call site declares an inverse today, so no
chrome row enters the undo ledger at all and `undo` reaches the document edit. Law C is the standing
guard for the day a call site does declare one.

### Tests

**TypeScript — `@semio-tech/framework-renderer-react` engine-contract, level `long`, exit 0:
`Tests 5 passed | 1568 skipped`** (`🗑️generated/b3b-note-shell-command-vitest.txt`), covering the three
laws added/rewritten at `🧪️tests/🔬️engine-contract/🟦️.ts:10640`:
`buildNoteShellCommandAction` carries no inverse unless one is declared; it carries a declared one
(and its args only when present); `isShellOwnedCommandId` separates the nine `shell.*` chrome ids and
the `os.*` ids from plugin action ids; `dispatchOsCommand` reports whether it routed.
At `quick` the file exceeds the 15 s budget — it is a `long` suite.

`bun ./📜️script.ts typecheck` on `@semio-tech/framework-os` (`🗑️generated/b3b-os-typecheck.txt`):
**zero diagnostics in any file this slice touched**. The 20 it does report are all pre-existing and
all in a peer's `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` plus one repo caching script.

**Rust — `cargo test -p semio-framework-plugin --lib reserved_undo`** *(queued; a peer's
`--lib an_abandoned_ingress_owner` on the same crate has held the artifact-directory lock for 38 min
at load ≈ 167 — rule 14 applies, mine is waiting behind it, result appended when it lands)*.
Two laws at `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`:
`reserved_undo_browser_note_without_inverse_is_not_an_undo_target` (rewritten from
`…_pops_chrome_resize`, whose fixture `🧫️fixtures/reserved-undo-browser-note.json` now carries a third
`shell.windowActivate` row and asserts both undeclared-inverse rows are logged-but-not-revertible and
that undo steps to `setActiveExample`) and the new
`reserved_undo_steps_over_undeclared_chrome_and_pops_the_document_edit` (a window activation noted
after a real `increment` must not steal that edit's undo, and no `ReplayShellCommand` may leave the
guest).

### Runtime re-measure — law C proven, laws A+B pending the wasm rebuild

`🔱️trinity jack` on a fresh serve (port **6120**, ports 6012–6056 were all dead after the outage;
6056 is an orphaned peer serve, left alone), probe
`🗑️generated/b3b-jack-lawC-probe.txt` + `🗑️generated/b3b-trinity-jack-console.txt`:

- **`faultLines: 0`, and no `undeclared-action` line anywhere in the run** — the §3.2 error is gone.
  The served modules were verified to carry the edit before trusting the run (`curl` of the `/@fs`
  ShellHost/ShellHelpers modules: `replayShellOwnedCommandRef` ×3, `no shell route for chrome
  command` ×1, `isShellOwnedCommandId` present).
- The served **guest wasm still predates law A**, so the ledger still shows every chrome row as
  revertible — `Resize Window↶ · Toggle Panel↶ · Switch Panel Tab↶ · Activate Window↶ ×3` — which is
  the defect itself, rendered. That `↶` disappearing from chrome rows is the runtime witness for law A.
- That run's `undo` moved nothing at all (`undone: false`). Laws A/B cannot cause that: on the
  pre-law-A guest, dropping `inverseCommandId`/`inverseArgs` from the wire is a no-op, because that
  guest still falls back to `unwrap_or(command_id)` and `.or(detail)`. **A second identical run
  passed the whole bar** (`🗑️generated/b3b-jack-lawC-probe2.txt`:
  `{"mutated":true,"undone":true,"faultLines":0,"interactionBar":true}`, `undeclared-action` count
  **0**), so the first run's dead `undo` is the same intermittency §3.2 already recorded, not a
  regression. Chrome rows still carry `↶` in both runs — that is law A, still unbuilt.

`🧩️puzzle 5d` on a fresh serve (port **6121**), probe `🗑️generated/b3b-puzzle5d-s4-probe.txt`:
**`{"mutated":true,"undone":true,"faultLines":0,"interactionBar":true}` — the full bar, 0 faults.**

So law C is proven at runtime on two plugins, and puzzle5d's §3.3 boot error is gone (see S4.3).

## S4.2 §5 gap 1/2 — trinity rewriting re-activation

Never started in session 4 (the session was cut at ~01:15). **Answered in session 5 §S5.3: no
re-activation was needed — the served trinity guest already carries both fixes and rewriting clears
the whole bar with zero faults.**

## S4.3 §3.3 — puzzle 5d board session factory: fixed by a peer, verified live here (CLOSED)

§3.3 was fixed in the tree by a peer **20 minutes before this session started** — commit
`03b1a41483` (2026-09-19 23:41:19) adds the two missing rows to
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🌉️wasm/🟦️.ts:20-21`:

```
{ kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle5d@1/*#editor", create: createPuzzleBoardSession },
{ kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle5d@1/*#viewer", create: createPuzzleBoardSession },
```

That is exactly the ownership answer §3.3 said had no prior art: `PUZZLE_BOARD_SESSION_FACTORIES` is
keyed by `appId`, `resolveAppSurfaceSessionFactory` matches the live session's app id exactly, and
only `puzzle2d`'s two roles were listed — so `Board2dHost`'s `useLayoutEffect` found an empty
`BoardSessionFactoryContext` and threw. **Not re-fixed here; re-measured instead**, which is what was
missing: the probe above shows `faultLines: 0` and **zero** occurrences of
`The current app has no registered board session factory` in
`🗑️generated/b3b-puzzle5d-console.txt` (the session-3 capture had two).

**🧩️puzzle 5d now clears the full interaction bar** — row 10 of §1 goes 🟡 → ✅.

---

# Session 5 (2026-09-20 ~01:25 →)

Inherited: laws A+B+C of S4.1 are in HEAD (`🔌️plugin/🦀️.rs` guest law A at the `inverseCommandId`
lookup, `🛠️ShellHelpers/🟦️.tsx` law B, `🏛️ShellHost/🟦️.tsx` law C). Live at start: the predecessor's
two detached serves, **6120 trinity-jack** (pid 91700) and **6121 puzzle5d** (pid 234), both 200.
Machine: load ≈ 84, 45 GiB free.

The bar is tightened here: **boot → example renders → one real mutation → undo → REDO → zero console
faults**, all five clauses in one headless run (`🐍️b3b-interaction-probe.mjs`, `--use-angle=metal`).
Session 3/4 measured four of the five (no redo), so every row below is re-measured, not copied.

## S5.1 The queued §5 law test — why it failed, and the crate-wide red behind it

S4's `cargo test -p semio-framework-plugin --lib reserved_undo` landed at 01:12
(`🗑️generated/b3b-plugin-reserved-undo-tests.txt`, 73 m 21 s build): **9 passed, 1 failed**. The
failure was NOT the law — it was the fixture's first line:

> `increment: Fault { code: "interactive-job.missing-factory", message: "typed command 'increment'
> has no exact controller/owner/factory/tool/schema proof" }`

Re-running the **already built** test binary (no cargo, no lock) against a sibling that has nothing
to do with this slice reproduces it exactly:

```
…/⚡️cache/cargo/build/debug/build/semio-framework-plugin/1f7e2ae6c4614b4d/out/semio_framework_plugin-…
  operation_action_emits_kernel_op_with_true_inverse
→ FAILED … increment: Fault { code: "interactive-job.missing-factory" … }   (🦀️.rs:3266)
```

So **every test in this crate that dispatches a typed command through the registryless
`VcsArtifactApp::<TestApp>::new(…)` is red on the current tree** — at least 17 call sites of
`dispatch_typed(TestCommand::Increment, …)`. The mechanism: `dispatch_typed` demands
`qualified_tool_proof(verb)` (`🔌️plugin/🦀️.rs:28001` → `:22054`), a proof comes only from
`app_tool_registrations` / `framework_tool_registrations` / `bounded_tool_proofs`, and for `TestApp`
all three are empty for `increment` — `register_tool_job_factories` registers nothing unless
`RETAINED` (`🧪️tests/⏳️completion/🦀️.rs:11`), `TOOL_JOB_IDS` is `["compositeEdit",
"applyCountFromTask"]` only, and `VcsArtifactApp::new` passes `AppActionRegistry::default()`, whose
`migrated_tool_ids()` is empty so `validate_tool_job_rows` returns no bounded proof either. This is
crate-owner work in a file three peers are editing live, so it is **reported, not fixed here**.

**What is fixed here:** the law no longer rides that broken path. It now seeds the document edit the
way its passing sibling `reserved_undo_pops_shell_then_falls_through_to_document_store` does — a
store-level `ArtifactCommand::Apply` — and asserts more than the old version did
(`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4570`):

1. the window activation is the newest ledger row,
2. that row is **not revertible** (law A's own witness, the `↶` the browser draws),
3. `undo` emits no `Effect::ReplayShellCommand` (law C),
4. `test_snapshot().count` goes 1 → **0** — the DOCUMENT edit is what undo popped, which is stronger
   than the old `applied` flag check,
5. the chrome row is still in the append-only log.

**TypeScript laws (laws B + C), re-run on the current tree — exit 0,
`Test Files 1 passed | 70 skipped`, `Tests 6 passed | 1607 skipped`, 201 s**
(`🗑️generated/b3b-s5-note-shell-vitest.txt`, name-filtered
`buildNoteShellCommandAction|isShellOwnedCommandId|dispatchOsCommand`). Two runs before it were
killed by the level budget — `long` allows 300 s and the file's import/transform alone costs ~200 s
at load 120 — so the run that counts used the library's own documented override
`SEMIO_TEST_BUDGET_MS=1500000` (`🦑️repo/…/📚️library/🟦️.ts:1150`). Without a `--testNamePattern` the
whole 1613-test file runs and is killed at any budget worth setting; that is the shape the next
person needs.

**Rust law (law A)**: `cargo test -p semio-framework-plugin --lib reserved_undo` — **queued three
times, never compiled inside a session.** 01:47 (pid 86486) died in the fleet deadlock the
coordinator cleared at 06:12; 06:20 (pid 10312) sat in `flock` for 1 h and was killed here to free
the one-cargo slot for the coordinator's follow-up; 07:45 (pid 77411) is 21 min into the same wait
behind 30 live peer `rustc`, which by rule 23(a) is a busy queue and not a deadlock, so it is left
running. It writes `🗑️generated/b3b-s5-reserved-undo-tests.txt` — **read that file first.** What is
known: at 01:12 the nine sibling `reserved_undo` laws all passed and the only failure was this
test's first line, which is the crate-wide red analysed above and is now gone from the test; and the
law it asserts is independently proven at runtime in §S5.2 on seven artifacts.

## S5.2 Bar matrix, session 5 (measured only)

Five clauses, one headless run each, `--use-angle=metal`, 1600×1000, captures
`🗑️generated/b3b-s5-<variant>.txt` + `🗑️generated/b3b-<variant>/report.json`:

| # | artifact | port | boots | example | mutation (verb) | undo | redo | faults | chrome rows ↶ | bar |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 🔱️trinity jack | 6054 | ✅ | ✅ 3 panes, 23 verbs | ✅ `patchNodes` | ✅ | ✅ | **0** | **0 of 6** | ✅ **PASS** |
| 2 | 🔱️trinity rewriting | 6056 | ✅ | ✅ 5 panes, 22 verbs | ✅ `setParameter` | ✅ | ✅ | **0** | **0 of 9** | ✅ **PASS** |
| 3 | 🀄️wfc bitmap | 6041 | ✅ | ✅ 25 verbs | ✅ `change-seed` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |
| 4 | 🀄️wfc grid2d | 6042 | ✅ | ✅ 31 verbs | ✅ `change-seed` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |
| 5 | 🀄️wfc 2d | 6043 | ✅ | ✅ 33 verbs | ✅ `change-seed` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |
| 6 | 🀄️wfc grid3d | 6044 | ✅ | ✅ 32 verbs | ✅ `changeSeed` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |
| 7 | 🀄️wfc 3d | 6045 | ✅ | ✅ 31 verbs | ✅ `change-seed` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |
| 8 | 🧩️puzzle 2d | 6012 | ✅ | ✅ 30 verbs | ✅ `addNode` | ✅ | ✅ | **0** | **0 of 5** | ✅ **PASS** |
| 9 | 🧩️puzzle 3d | 6013 | ✅ | ✅ 71 verbs | ✅ `openAddObjectDialog` → `addObjectKind` | ✅ | ✅ | **0** | **0 of 5** | ✅ **PASS** |
| 10 | 🧩️puzzle 5d | 6121 | ✅ | ✅ 81 verbs | ✅ `addNode` | ✅ | ✅ | **0** | **0 of 4** | ✅ **PASS** |

**10 of 10 clear the full five-clause bar, and every one of them now logs its chrome rows as
non-revertible** — rows 1–7 were re-measured after the two re-activations of §S5.8, rows 8–10 were
already on a law-A (or pre-regression) guest. Captures: `🗑️generated/b3b-s5-*.txt` (`…-jack3`,
`…-rewriting2`, `…-bitmap2`, `…-grid2d2`, `…-wfc2d2`, `…-grid3d2`, `…-wfc3d2` are the post-activation
runs) plus `🗑️generated/b3b-<variant>/report.json`.

Two probe changes were needed for this matrix and are now permanent in `🐍️b3b-interaction-probe.mjs`:

- **redo clause** (`:255`) plus a `chrome-rows` step (`:272`) that counts how many rows the SHELL noted
  for its own chrome still carry the revert affordance `↶`. That count is the §3.2 law's runtime
  witness and it is measured, not asserted.
- **dialog-reached verbs** (`:198`, `:227`). A peer moved puzzle3d's `addObjectKind` out of the
  Actions palette (`🧊️3d/…/✏️editor/🦀️.rs:8470` `in_palette(false)`) and behind the `addObject`
  dialog (`:8619`), so session 3's verb is simply not in the pane any more (75 → 71 rows) and the
  first three replacements I tried (`addTargetVolume`, `createAttraction`, `addBrushObject`) are
  engagement-driven: they click, journal nothing and fault nothing. The dialog mounts its argument
  control as `button#objectKind` and its trigger as `button#ui.dialog.submit`
  (`🐍️b3b-dialog-diagnose.mjs`, new) — the probe now stages dialog args by their bare key and
  submits through the dialog, `"*"` meaning "whatever the first option is".

### The §3.2 defect, measured on both sides of its own fix

Before the two re-activations, 🔱️trinity and 🀄️wfc rendered the defect and 🧩️puzzle did not. A variant
does not carry its own guest: every variant of a plugin is served the same
`✏️s/🔌️plugins/<plugin>/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_<plugin>.wasm`, so the
whole plugin moves together, and the cut is `48a8c69cdb` (2026-09-19 13:48), which gave the guest
`…get("inverseCommandId")…unwrap_or(command_id)` — every chrome note became its own inverse.

| plugin | guest built | window | chrome rows revertible |
|---|---|---|---|
| 🔱️trinity | 09-19 **16:11** → re-activated 09-20 **07:25** | regression → **law A** | **6 of 6 / 9 of 9** → **0** |
| 🀄️wfc | 09-19 **16:12** → re-activated 09-20 **07:36** | regression → **law A** | **4 of 4** → **0** |
| 🧩️puzzle | 09-20 **01:51** (and 09-19 04:13 in S4) | after law A (resp. before the regression) | **0** throughout |

That is the whole law, rendered: **law B** (`🛠️ShellHelpers/🟦️.tsx:389`) stops the host fabricating a
self-inverse, **law A** (`🔌️plugin/🦀️.rs:26656`) makes the guest record an `InverseAction` only when
one is declared, and **law C** (`🏛️ShellHost/🟦️.tsx`) routes a `shell.*`/`os.*` replay through
`dispatchOsCommand` instead of into the guest. Law B alone was not enough — the jack runs at 01:44
and 02:10 were a law-B host talking to a pre-law-A guest and every chrome row was still an undo
target. With the guest rebuilt, `Resize Window · Toggle Panel · Switch Panel Tab · Activate Window ×3`
all lose the `↶` and `undo` can only reach the user's own document edit.

**One trap worth recording.** Two jack runs scored `undone:false` and I nearly wrote them up as the
theft: they were the probe's settle budget. jack's undo landed at **112 s** and its redo at **136 s**
under load ≈ 110, against a 25 s default. A slow guest and a stolen undo look identical through a
fixed budget — the post-activation runs use `SEMIO_PROBE_SETTLE_MS=60000`. Likewise jack's only
fault lines in the 02:10 run were four `ws://127.0.0.1:59716/bridge ERR_CONNECTION_REFUSED` (M7's
agent-bridge rendezvous, which a bare variant serve does not run); the 07:30 run has none.

## S5.3 §5 gaps 1 + 2 are CLOSED — and no re-activation was needed for them

The gap said rewriting was "one activation away". It is not: the trinity guest that is **already
served** (`…/🔱️trinity/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_trinity.wasm`,
2026-09-19 **16:11**) was built ~12 h AFTER the §3.1/§3.1b source fixes landed, so it carries them.
Measured on a fresh serve on the boot-recipe port **6056**
(`🗑️generated/b3b-s5-rewriting.txt`, `🗑️generated/b3b-trinity-rewriting-console.txt`):

```
boot {"ready":"trinity-rewriting","error":null,"shellError":null,"windowFaults":[], 5 panes}
SUMMARY {"exampleRendered":true,"actionCount":22,"mutated":true,"undone":true,"redone":true,
         "faultLines":0,"interactionBar":true}
```

- **The §3.1 `dropped action "setActiveExample"` line is gone** — zero occurrences in the whole run,
  where session 3 had one on every boot. The navbar picker mounts with its `Demo` example.
- **The §3.1b `artifact-store.persisted-initializer-refused` line is gone** too — zero occurrences,
  which is the `build_document_store_initialization_job` fix (`…/✏️editor/🦀️.rs:716`) observed at
  runtime for the first time. That also means `resetRule`'s `Effect::LoadDocument` path is no longer
  refused at the archive-load boundary.
- `setParameter` → undo → redo, `faultLines: 0`: **the full five-clause bar**.

So §5 gap 1 and gap 2 are closed by measurement, not by a build. The trinity re-activation is still
queued, but for a different reason: it is what flips **row 1 (jack)** by giving trinity a post-law-A
guest (§S5.2).

## S5.4 cargo deadlock at 01:31–06:12 (reported per preamble rules 14 and 23)

`📜️b3b-activate.sh trinity-rewriting` started 01:31 and its `cargo rustc --target wasm32-wasip2`
(pid 1257) has held **0 % CPU with no `rustc` child for ~59 min**. `sample 1257` puts it exactly in
the known deadlock shape:

```
cargo::core::compiler::prebuild_lock_exclusive → cargo::core::compiler::locking::LockManager::lock → flock
```

Seven further peer `cargo rustc` processes are in the same state (58 m, 58 m, 45 m, 38 m, 26 m,
9 m, 6 m) with four `rustc` actually burning CPU, so the wasm build-dir queue is eight deep and the
machine is at load 120–150. The native `cargo test -p semio-framework-plugin --lib reserved_undo`
(pid 86486) answers `Blocking waiting for file lock on artifact directory` too. Per rule 14 this was
reported rather than worked around and **nothing was killed from here**. The coordinator confirmed
it at 06:12 as a genuine fleet-wide deadlock — 34 cargos at 0 % CPU for 4 h with no `rustc` anywhere
— and killed the set, taking this slice's two queued builds with it. Both are re-queued since 06:20
(the test, §S5.1; the trinity re-activation is NOT re-queued, because one cargo at a time and the
law proof outranks it — see §S5.7).

## S5.5 Where the §5 gaps stand after this session

| §5 gap | state |
|---|---|
| 1 + 2 — rewriting re-activation, `setActiveExample` end-to-end | **CLOSED by measurement (§S5.3)**: the served trinity guest already carries both fixes, both console faults are gone and rewriting clears the full five-clause bar on 6056 |
| 3 — puzzle5d board session factory | **closed in S4.3**, re-measured again here: `faultLines: 0` and the full five-clause bar |
| 4 — `shell.windowActivate` framework-wide | **root-caused, fixed and now measured on both sides of the cut** (§S5.2): laws A/B/C are in the tree, the TS laws pass, and the runtime A/B between a pre-`48a8c69cdb` guest (0 revertible chrome rows) and a post-`48a8c69cdb` one (6 of 6, undo silently stolen) is in the matrix. The Rust law run is §S5.1 |
| 5 — `🔱️trinity` / `🧩️puzzle` descriptors miss `artifactSchema` | **unchanged and re-measured**: `✏️s/🔌️plugins/🔱️trinity/🔣️.json` (800 388 B) and `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` (4 803 294 B) contain no `artifactSchema`; `🀄️wfc`'s does. `🧩️puzzle`'s committed pack `🛂️.descriptor.semio` is **4 295 257 B — 100 953 B over the 4 MiB bound**, exactly DS1 §0's number. DS1's root fix (the duplicated per-window action roster) is in the working tree but **no descriptor has been regenerated** (DS1 §9.3), so `describe` is the next step for both plugins and it needs a `wasm32-wasip2` component build per plugin — the queue this session could not get through |
| 6 — puzzle TS diff modules | untouched, T4b's |
| 7 — launch rows | unchanged; all ten variants still have their rows, and this session used the boot-recipe ports (6041–6045, 6012, 6013, 6054, plus 6120/6121 inherited) |
| 8 — per-crate tests | the only source changed here is the framework test file of §S5.1 |

## S5.6 Files changed this session

**Framework (law proof):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4570` — the §3.2 law no longer rides the crate-wide-red typed-dispatch path and asserts the snapshot, the non-revertible chrome row and the absent replay.

**Ticket folder:**
- `🐍️b3b-interaction-probe.mjs` — redo clause, `chrome-rows` law witness, dialog-reached verbs (§S5.2).
- `🐍️b3b-dialog-diagnose.mjs` — NEW, the one-off that found `button#objectKind` / `button#ui.dialog.submit`.
- `🗑️generated/b3b-s5-*.txt` — every capture cited above.

**Servers left running** (mine this session, kill by pid only): 6041 bitmap, 6042 grid2d, 6043
wfc2d, 6044 grid3d, 6045 wfc3d, 6012 puzzle2d, 6013 puzzle3d, 6054 trinity-jack, 6056
trinity-rewriting, 6121 puzzle5d (restarted at 06:20 after the outage). All nine of the pre-outage
serves survived the 03:00 cut and answered 200 at 06:15 — a vite serve outlives its parent session,
which is worth knowing before anyone restarts one. 6120 (trinity-jack) died on its own mid-session
and was replaced by 6054.

## S5.7 What the next session should do, in order

1. **Read `🗑️generated/b3b-s5-reserved-undo-tests.txt`** — the law run of §S5.1, queued since 07:45
   (pid 77411). If it landed, §S5.1 is closed; if the queue killed it, rerun that one command. It is
   the only unfinished item on this slice.
2. ~~trinity re-activation~~ and ~~wfc re-activation~~ — **done in §S5.8**, matrix is 10/10.
3. `nx run @semio-tech/trinity-plugin:describe` and `@semio-tech/puzzle-plugin:describe` for §5 gap 5,
   now that DS1's root fix is in the tree — puzzle's pack has to come back under 4 MiB.
4. The crate-wide `interactive-job.missing-factory` red of §S5.1 needs a crate owner: no test in
   `semio-framework-plugin` can dispatch a typed command through `VcsArtifactApp::new` today.

## S5.8 Coordinator follow-up (07:20–07:45): the moved primitive, and the two re-activations

**`settle_framework_reserved_admission` callers (V3b item 6) — fixed and proven.** The primitive
lives at `semio_framework_plugin::app::settle_framework_reserved_admission`
(`🔌️plugin/🦀️.rs:28532`); `artifact_app_laws` (`:6936`, `pub use` at `:39849`) is a real and heavily
used module, but it holds `settle_registered_typed_operation` / `new_app_with_registry` / … and NOT
this one. Grepping the primitive across `✏️s/🔌️plugins` and `🧰️framework` found **21 call sites, of
which exactly 2 used the wrong module** — every other plugin already spells it `…::app::…`:

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs:31`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:53`

Both now use the `app::` path (the spelling the other 19 call sites use, so the file keeps its
convention rather than gaining a lone `use`). The same files' `artifact_app_laws::settle_registered_typed_operation`
calls are correct and were left alone. Proof:
**`cargo check -p semio-s-plugin-space -p semio-s-artifact-vcs-vcs --tests` → `Finished dev profile in 4m 26s`, zero errors**
(`🗑️generated/b3b-s5-space-vcs-check.txt`). Neither crate declares `component-app-assembly`
(space's only feature is `plugin-entry`, default), so no feature flag was needed.

**Re-activations.** `📜️b3b-activate.sh trinity-jack` → `exit=0`, `trinity-plugin:component-dev` 1 m 16 s,
guest at 07:25. `📜️b3b-activate.sh bitmap` → `exit=0`, `wfc-plugin:component-dev` 4 m 42 s, guest at
07:36. One activation per PLUGIN was enough for all seven variants, which is the same
one-guest-per-plugin fact §S5.2 rests on. All seven re-probed: `chromeRevertible` **0**, full bar,
`faultLines: 0`.
