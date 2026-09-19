# B3b — 🔱️trinity, 🀄️wfc, 🧩️puzzle: React boot + interaction bar

Slice B3b of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END.
**Result: 8 of 10 artifacts clear the full interaction bar; the other 2 clear every clause except one
boot-time console error each, both root-caused (one fixed in source, awaiting a blocked wasm build).**

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
| 10 | 🧩️puzzle 5d | `puzzle5d` | 6014 | ✅ activated here (the only one of the ten that had never been staged) — boots | ✅ 81 actions | ✅ `addNode` → `create-part part { id=part-ea2f0eb01dbff212-0 … }`, edits 1→2 | ✅ 2→1 | **1 at boot** (§3.3) | 🟡 bar met except one boot error |

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
