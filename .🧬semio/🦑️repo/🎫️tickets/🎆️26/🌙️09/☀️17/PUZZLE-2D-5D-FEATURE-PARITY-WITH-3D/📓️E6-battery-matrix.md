# E6 — 3d/2d battery matrix, current failures, and a 5d battery design

Read-only audit. Sources: `🔍️browser-probe.ts` in both END-TO-END tickets, the newest `🗑️generated/probe-*.md` /
`battery-*.txt` runs in each, and `Board2dHost`/`World3dHost` under `🧰️framework`. No code, tickets, or generated
folders were modified.

## 1. Step inventory

**3d battery** (`.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`) registers 38 `add(name, section, group, …)`
steps, `§0`–`§25`, each `group ∈ {read, mutate, replace}`; a full `--battery` run drives every one in that group order
plus a `guest-alive-<group>` checkpoint per group. Section `§7` is **declared in the reference checklist
(`📓️2026-09-09-user-feature-checklist.md`) but never registered as a step** — that's hover (see §4 below).

**2d battery** (`.🧬semio/…/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts`) registers 23 `register(name, group, …)`
steps, grouped only as `read | mutate | replace` (no `§`-numbering), each producing one or more `section/step`
verdicts (e.g. the single `fill` step emits four verdicts: `fill-tab-present`, `fill-count-measure`,
`fill-distribution-groups`, `fill-run-places-nodes`, `fill-finalize-keeps-placements`).

## 2. Matrix — 3d step → 2d coverage

| 3d step (§, group) | 2d step | Coverage |
|---|---|---|
| `window-content` (§1, read) | `windows` | mapped |
| `activate-perspective` (§1, read) | — | **MISSING** — 3d-only concept (perspective/orthographic camera activation) |
| `camera-gestures` (§2, read) | `camera-wheel` | partial — 2d only wheel-zoom; no pan/orbit/reset gestures |
| `projection-options` (§3, read) | — | **MISSING** — 2d has no projection (ortho-only 2D plane) |
| `window-options` (§4, read) | — | **MISSING** — no per-window options step in 2d |
| `example-switch` (§5, replace) | `example-concrete-forest`, `example-nakagin` | mapped |
| `pick-object` (§6, mutate) | `click-select` | mapped |
| `selection-surfaces` (§6/§16, mutate — inspector-fresh/updates-while-open) | `click-select` + `inspector-shows-selected-node`, `inspector-updates-while-open`, `inspector-fresh-on-open` | mapped (2d currently the more thorough of the two here) |
| — (no §7 step exists) | — | **hover is MISSING from BOTH batteries** (see §4) |
| `locked-refusal` (§8/§16, mutate) | — | **MISSING** — no lock/hide + refusal-notice step in 2d |
| `gumball-drag` (§8, mutate) | `drag-node`, `engagement-move` | partial — 2d drags directly (no gumball widget); pose-change assertion exists but not the gumball-specific gesture |
| `brush-stroke` (§9, mutate) | `utilities` (toggle-presence only) | **MISSING the actual paint action** — 2d never strokes/places via brush, only checks the toggle exists |
| `volume-brush` (§10, mutate) | — | **MISSING** |
| `relocate` (§11, mutate) | `engagement-move`, `drag-node` | partial |
| `tool-category` (§12, mutate) | `utilities` | partial — presence only, not full category inventory |
| `fill-tab` (§12, mutate) | `fill` (`fill-tab-present`) | mapped |
| `fill-abort-engagement` (§12, mutate) | — | **MISSING** — no abort/pause/step path tested in 2d |
| `fill-wait-ready` (§12, mutate) | folded into `fill-run-places-nodes`'s wait-for-completion | partial |
| `fill-apply-max` (§12, mutate) | folded into `fill-run-places-nodes` / `fill-finalize-keeps-placements` | partial — no dedicated slider/weights check |
| `fill-history` (§12, mutate) | — | **MISSING** — fill-specific undo not tested; `undo` step is generic and runs later on an unrelated edit |
| `suggestions-open` (§13, mutate) | — | **MISSING entirely** — no candidate popup, hover-to-preview, accept, or close/dismiss coverage in 2d |
| `engagement-bar` (§14, mutate) | `engagement-move` | partial — action exists, dedicated bar/grammar UI not asserted |
| `context-menu` (§15, mutate) | `context-menu` | mapped (2d checks the menu opens and captures its row text once) |
| `context-menu-rows` (§15, mutate — full row vocabulary incl. duplicate/select-same-kind/zoom/delete/hide-show/lock-unlock) | `context-menu`, `select-same-kind` (2d actually **exercises** select-same-kind as a click, 3d only checks the row label exists) | partial, asymmetric — 2d exercises one row 3d only names; 3d names five/six rows 2d never clicks (duplicate, zoom-to-selection, hide-show, lock-unlock) |
| `outliner-rows` (§17, mutate — hide/lock/show-restores) | — | **MISSING** — no outliner hide/lock/show-restore step in 2d |
| `catalogue-panel` (§18, mutate) | `catalogue-add` | mapped, 2d version is a simpler click-adds-node smoke test |
| `settings-panel` (§19, read) | `settings` | mapped |
| `history-open` / `undo-unwind` / `undo-redo` / `undo-once` (§20, read+replace) | `undo` (single, one-shot) | partial — no history-panel-open check, no redo, no multi-step unwind |
| `clipboard-copy-paste` (§21, mutate) | — | **MISSING entirely** — no copy/cut/paste in 2d |
| `selection-keybindings` (§22, mutate) | `delete` (delete key only) | partial — 3d checks a broader keybinding set, 2d only delete |
| `add-object-dialog` (§23, read) | — | **MISSING** — 2d adds via catalogue click, never opens/drives an add-object dialog |
| `marquee-drag` / `marquee-click` (§6, mutate) | `marquee` (drag only) | partial — no click-marquee variant in 2d |
| `export-import` (§24, replace) | `export`, `import` | mapped — both currently exercised, `import-round-trip` FAILs (see §3.2) |
| `locale-switch` (§25, read) | `locale` | mapped |
| — | `explore-chrome`, `inspector-timing`, `guest-alive` | 2d-only scaffolding/smoke steps, no 3d counterpart needed |

**Not in either battery, confirmed by source read, not inference:** two-window independence (neither probe opens a
second *browser* window/tab of the same document to check independent state — both only exercise the panes the
single React shell boots), redo beyond one step, weights-panel-driven fill distribution, and window-options.

## 3. Current failures

### 3.1 — 3d (newest results, `.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/`)

The newest *report* by mtime, `probe-2026-09-13T03-31-42.md`, is **not a battery result** — it is a 183 s boot
timeout: `windows=0 canvases=0` throughout, with one collateral fault, `http 504 …@tailwindcss_postcss.js` (an
Outdated-Optimize-Dep Vite error). That's probe/dev-serve infra, not a product verdict — it never reached step 1. It
should not be read as the current 3d state.

The newest *full-battery numeric run* is `battery-2026-09-13-64-6013-B55.txt` (Sep 13 02:01), **PASS=89 FAIL=10** —
better than the umbrella ticket's cited "last known 74 PASS / 25 FAIL" (`battery-2026-09-13-57-6013.txt`, Sep 12
16:54; two later intermediate runs, 58/60/61, run 60–73/25 and 71/25 and 64/33, before 62–64 climb back to 84/13,
83/14, 89/10). The 74/25 baseline is stale; use 89/10 as current. Its FAIL steps, with evidence:

- `locked-refusal-notice` — `instances=3@step-start locked=true flag="locked true" grabbed=true poseChanged=false
  notices=[] waitedMs=30135` — the lock flag and grab state are right but the expected on-screen refusal notice never
  appears. Looks like a real product gap (locked-object gesture-refusal UI), not a probe issue.
- `gumball-scene-delta` — `sceneDelta=false poseLen=799 waitedMs=31007` — gumball drag produces no scene delta after
  30 s of waiting; product bug or probe selector missing the live gumball handle.
- `relocate-pose-delta` — `beforeLen=800 afterLen=800 instances=3 waitedMs=30460` — identical before/after pose
  length: the relocate gesture is a no-op in this run.
- `outliner-show-restores` — `restored=false afterShowHead=[…"Hexagonal Cut Concrete Forest Left Hide
  Lock"…"Left Hide Unlock"…]` — one row still reads "Unlock" after the restore step, i.e. the lock toggle it flipped
  never flipped back.
- `fill-tab: FAILED TimeoutError: click: Timeout 5000ms exceeded.` — a plain Playwright click timeout, i.e. the
  Fill tab target wasn't clickable/present within 5 s; likely a DOM-timing/selector fragility in the probe rather
  than a product defect, but not confirmed either way from this evidence alone.
- `example-switch` — `example=Concrete Forest` (bare, no detail) — switching examples failed to reach Nakagin.
- `export-only`, `export-names-the-example` — `download=none … exporting Concrete Forest and Nakagin must not both
  land as one constant name` — export produced no download at all in this run, cascading into the naming check.
- `import-same-file-idempotent`, `import-distinct`, `import-distinct-records-history` — all three read `not
  reachable — export-only produced no file for THIS document` — these three FAILs are **downstream of the
  export-only FAIL**, not independent defects; fixing export collapses them to PASS or reveals the real import
  behaviour underneath.

Net: of the 10 nominal FAILs, 3 (`import-*`) are one cascading export failure, `fill-tab` reads as a probe
click-timing issue, and the remaining six (`locked-refusal-notice`, `gumball-scene-delta`, `relocate-pose-delta`,
`outliner-show-restores`, `example-switch`, `export-only`/`export-names-the-example`) read as real product gaps in
lock/gumball/relocate/outliner/example-switch/export.

### 3.2 — 2d (newest run, `probe-2026-09-17T02-14-35.md`, PASS=33 FAIL=4 FAULTS=2)

Four FAILs, quoted verbatim from the report:

1. `16-inspection/inspector-fresh-on-open` —
   `{"rows":["Schema puzzle.2d.fixture","Extension puzzle.2d","Nodes 0","Edges 0"]}`. Prior lines show the probe
   picked a second node (`second=1493671a-…`) then opened the inspector fresh, but `selection=[]` and the inspector
   still shows the *document* schema/nodes/edges rows, not the node's own fields — the inspector did not pick up the
   fresh selection on open.
2. `8-transform/engagement-move` —
   `{"first":"puzzle2d.fill.1","before":[325.329…,50.327…],"after":[325.329…,50.327…],"waitedMs":20022}` — before
   and after pose are bit-identical after a 20 s wait: the engagement-move gesture is a no-op, mirroring 3d's
   `relocate-pose-delta`/`gumball-scene-delta` FAILs above (same symptom class: transform gesture dispatched but no
   scene delta lands).
3. `20-history/undo-changes-document` — `{"before":1297,"after":1297,"parsed":"true","waitedMs":20361}` — node/edge
   count identical before and after undo; undo is parsed as acknowledged (`"parsed":"true"`) but the document itself
   didn't change.
4. `24-import/import-round-trip` —
   `{"chooser":false,"before":101,"after":101,"edges":96,"expected":{"nodes":180,"edges":179},"parsed":"true",
   "waitedMs":120461}` — `chooser:false` means the file-picker never surfaced; the step waited 120 s then gave up,
   document stayed at 101/96 instead of the expected 180/179. Same shape as 3d's `export-only`/`import-*` chain:
   an earlier plumbing step (file chooser wiring) never engaged.

Two FAULTS: both are the console line `warning: semio dev · 58 staged plugin module(s) are behind their source —
the host in this page may speak a newer wire contract than the guest it is talking to:` followed by ~58 `[stale] …
run: bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` lines. This string is captured as one
console message and matches the probe's `FAULT_RE` only via the catch-all `|puzzle2d-/i` alternative (present in
nearly every `[stale] … activate-puzzle2d-react-dev` line), and it recurs twice over the run (once at boot, once
mid-run). This is a **dev-environment staleness notice** (unstaged/unactivated plugin modules for other apps sharing
the host, not puzzle2d itself), not a functional defect — it inflates the FAULTS counter without indicating a real
regression. Worth tightening `FAULT_RE`/`noteFault` to exclude this banner if it keeps firing.

## 4. Designing the 5d battery

**Current state:** `5d.puzzle` is a declared `onArtifactKind` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/🔣️.json` (alongside `2d.puzzle`/`3d.puzzle`),
and a `puzzle5d` Rust artifact crate exists (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d`), but **no dedicated React
dev-serve target or host component for puzzle5d was found** (`framework-os-dev:activate-puzzle5d-react-dev` does
not exist as a target; searched for `puzzle5d-react` app dirs — none). A 5d Playwright battery cannot be written
against a live serve yet; the ticket's own umbrella description implies the app itself is in scope, not just the
probe.

**DOM hooks that already exist and are reusable, read directly from the two hosts:**

- `Board2dHost` (`🧰️framework/…/🧱️elements/🖥️Board2dHost/🟦️.tsx`) publishes, per board pane, on the element carrying
  `data-surface-id`: `data-board-nodes`, `data-board-edges`, `data-board-handles`, `data-board-positions-json`,
  `data-board-selection-json`, `data-board-camera-json`, `data-board-hovered-id`, `data-board-active-utility`. The
  2d probe's `boardVitals()` helper already reads all of these without a guest round trip — directly reusable.
- `World3dHost` (`🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx`) publishes on its `data-surface-id` element:
  `data-selection-json`, `data-guest-selection-json`, `data-instances-json`, `data-target-volumes-json`,
  `data-engagement-preview-json`, `data-camera-json`, `data-viewport-camera-json`, `data-vortices-json` (per the
  feature checklist, `vortices-json` carries the `hovered`/`hoveredVortexFullId` flags), `data-suggestion-menu-json`,
  `data-interaction-json` (also carries `hoveredVortexFullId`), `data-status-json`, `data-sun-json`,
  `data-meshes-json`, plus gesture-scoped attrs set imperatively elsewhere in the file: `data-gumball-hits`,
  `data-vortex-hits`, `data-hover-paint-id`. **Neither host exposes a single bundled "vitals" attribute the way
  Board2dHost does** — a 5d probe reading the world pane has to assemble node/edge/selection state from several JSON
  attributes rather than one `data-board-*` block.
- Note: hover is *readable* from the DOM on both hosts (`data-board-hovered-id` on 2d;
  `data-vortices-json`/`data-interaction-json` hover fields on 3d) even though **neither existing battery asserts
  on it** — §7 was never registered in the 3d probe and 2d's `boardVitals()` reads `hovered` but no `register()`
  step ever verdicts on it. A 5d probe (or a fix to the two existing ones) can close this gap immediately since the
  data is already there.

**Steps directly reusable from the 2d probe** (same helpers, same assertions, pointed at the board pane of a 5d
window): `example-inventory`, `windows`/vitals-published, `panels`, `camera-wheel`, `click-select`, `marquee`,
`drag-node`, `utilities` (toggle presence), `fill` (all five verdicts), `delete`, `context-menu`, `catalogue-add`,
`select-same-kind`, `export`, `settings`, `locale`, `guest-alive`.

**Steps directly reusable from the 3d probe** (pointed at the world pane): `pick-object`, `camera-gestures`,
`projection-options` (if 5d's world pane keeps perspective/ortho toggle), `brush-stroke`, `volume-brush`,
`suggestions-open`, `locked-refusal`, `gumball-drag`, `relocate`, `engagement-bar`, `context-menu-rows`,
`outliner-rows`, `add-object-dialog`, `selection-keybindings`, `export-import`.

**What a 5d `🔍️browser-probe.ts` must check that neither existing probe does — paired-pane behaviour:**

1. **Select in board pane → highlighted in world pane.** Click a node in the 2d-style board, read
   `data-board-selection-json` on the board surface, then read `data-selection-json`/`data-guest-selection-json` on
   the paired world surface and assert the same id set appears in both within one settle window. (Reverse direction
   too: select in world pane, assert board's `data-board-selection-json` updates.)
2. **Fill places parts visible in both panes.** Run the fill tool from either pane's utility bar, then assert
   `data-board-nodes` on the board surface and the instance count derived from `data-instances-json` on the world
   surface both increase by the same delta, and that the *same* new ids appear in `data-board-selection-json`-style
   listings on one side and `data-instances-json` on the other (paired-document identity, not just count parity).
3. **Camera independence, document identity.** Wheel-zoom the board pane (`data-board-camera-json` changes) and
   assert the world pane's `data-camera-json`/`data-viewport-camera-json` does **not** change — camera state must
   stay per-pane while node/edge/selection state stays shared, the inverse of the single-document
   two-window-independence gap noted in §2.
4. **Hover paired across panes** (the gap both existing batteries leave open): hover an object in one pane, read
   `data-board-hovered-id` / `data-vortices-json`'s hover field on the *other* pane, assert they name the same
   entity.
5. **Undo/redo affects both panes identically** — run `undo` from either pane's history stack and assert node/edge
   counts on both surfaces regress together (this also closes the "no redo" gap from §2 for at least one plugin).
6. **Boot/guest-alive** must snapshot `data-plugin-recovery` and both surfaces' vitals at once, since a 5d pane pair
   shares one guest actor — a corpse in either pane should show up as a guest-death fault for the whole step, not a
   silent partial read (mirrors the existing `guest-alive-<group>` pattern in the 3d probe, which the 2d probe only
   has as a single end-of-run step, not per-group).

## Sources read

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` (4351 lines, 38 `add()` steps)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts` (888 lines, 23 `register()` steps)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-user-feature-checklist.md` (§7 hover, §13 suggestions)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/probe-2026-09-13T03-31-42.md` (newest report, boot-timeout, not a real battery)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/battery-2026-09-13-{57,58,59,60,61,62,63,64}-6013*.txt` (numeric battery history)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated/probe-2026-09-17T02-14-35.{md,ndjson}` (newest 2d run)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/🎫️ticket.json`
