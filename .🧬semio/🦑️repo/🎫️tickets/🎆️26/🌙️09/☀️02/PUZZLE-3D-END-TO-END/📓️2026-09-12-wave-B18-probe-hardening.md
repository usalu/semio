# Wave B18 — probe hardening, outliner row selection, context-menu precondition

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Files this wave owns: `🔍️browser-probe.ts`,
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`,
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`.

---

## 1 Probe hardening (`🔍️browser-probe.ts`)

### 1.1 Navigation-safe evaluates — the `--reload-between-groups` crash

Battery #46b died at `reloading page before group mutate` with
`Execution context was destroyed, most likely because of a navigation` raised from
`🔍️browser-probe.ts:91`, which was `snapshot()`'s `page.evaluate`: the group loop's `page.goto` resolved on
the OLD document's `domcontentloaded` and `waitForBoot`'s first `snapshot()` then evaluated into a context
the new document had already torn down.

Three additions, all top-level so every helper shares them:

| helper | what it does |
|---|---|
| `NAVIGATION_RACE_RE` | the Playwright rejection texts that mean "the page moved under you" — `Execution context was destroyed`, `Most likely the page has been closed`, `frame was detached`, `Cannot find context with specified id`, `navigation` |
| `evalSafe(body, fallback)` | runs one `page.evaluate`; on a navigation race it awaits `page.waitForLoadState("domcontentloaded")`, waits 750 ms and evaluates ONCE more, then returns `fallback` rather than throwing |
| `countSafe(locator)` | the same guard for `locator.count()`, which rejects identically and is what the intro-dialog / welcome-tour skips call |
| `gotoShell(label)` | the ONE navigation the probe performs: `page.goto(…, {waitUntil:"domcontentloaded"})` **plus** an explicit `waitForLoadState("domcontentloaded")` before any evaluate runs |

`snapshot()` and `dumpInstances()` now read through `evalSafe`; `waitForBoot`'s two `.count()` calls read
through `countSafe`. The boot navigation and the `--reload-between-groups` navigation both go through
`gotoShell`, and the group loop still calls `waitForBoot(…)` after each reload — which is what re-runs the
intro-dialog skip and the welcome-tour dismissal, so a rebooted group starts from the same chrome state as
a cold one.

### 1.2 The guest-death fault family, and a `guest-alive` verdict per group

B13 decoded the terminal fault out of `shard 0 worker fault [handler/turn] actor=puzzle#1`
(`📓️2026-09-11-wave-B13-export-history-locale.md` §5). Its four surface strings are now a named family:

```ts
const GUEST_DEATH_RE = /reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected/i;
```

- all four added to `FAULT_RE` (so they are counted at all) **and** to `HARD_FAULT_RE` (so they are never
  filed as collateral). `Agent disconnected` is therefore a HARD fault whenever it appears in the CONSOLE.
- `COLLATERAL_NOTICE_RE` is untouched: it filters DOM notice strings inside `locked-refusal` and friends,
  a different code path from the console fault stream, and a transient banner in the notice rail still must
  not fail a feature step.
- `guestDeathFaults[]` is collected separately and reported in the NDJSON, the summary and the markdown
  (its own `### guest death` section).

At the END of every blast-radius group the probe now emits `guest-alive-<group>` (`guest-alive-read`,
`guest-alive-mutate`, `guest-alive-replace`). Its evidence is the cheapest pair of attributes the host
already publishes — no round trip to the guest, so it costs nothing once the guest is gone:

| signal | where it comes from | reads |
|---|---|---|
| `[data-plugin-recovery]` | `📌️ChromePanels/🟦️.tsx` `PluginRecoveryPanel` — the "Plugin Recovery / This program crashed." body the shell swaps a dead program for | absent while alive |
| `[data-instances-json]` / `[data-status-json]` per `data-surface-id` | `🌐️World3dHost/🟦️.tsx` — the scene payload stops arriving the moment the actor traps | ≥1 instance on at least one surface |
| `canvas` count | same host | ≥2 |
| `guestDeathFaults.length` | the family above | 0 |

The verdict note carries all four plus `firstHardFaultAt`, so a red one says WHICH of them broke.

### 1.3 `first-hard-fault-at`

`noteFault` stamps `firstHardFaultAt` (seconds since `t0`) the first time `HARD_FAULT_RE` matches. It now
appears in

- the summary line: `battery PASS=n FAIL=n FAULTS=n first-hard-fault-at=<s> guest-death-faults=n`
- the summary NDJSON record (`firstHardFaultAt`, `guestDeathFaults`)
- EVERY verdict NDJSON record (same two fields), so a coordinator can sort verdicts into "before the guest
  died" and "after"
- the markdown fault header and the final `done …` line.

### 1.4 What did NOT change

The NDJSON stream, its `[section, step, verdict]` join key, and every existing verdict name and step name
are untouched. The only new verdict names are the three `guest-alive-<group>` ones.

---

## 2 Outliner row selection — the hop table

### 2.1 The chain, source-side

| hop | where | what it does | state |
|---|---|---|---|
| 0 | `✏️editor/📌️panels/🗿️artifact/🦀️.rs` `object_row` → `selectable_item` → `select_action` | authors the row with `ActionBinding { trigger: Trigger::Activate, action: INTERACTION_SELECT_ACTION_ID }` and args `{domainId, merge:"replace", method:"pick", targets:[{granularity:"object", id}]}` on `PUZZLE3D_PLAY_CONTROLLER_ID` | **correct** — the guest already dispatches exactly the descriptor the task specified; no guest arm change was needed |
| 1 | `🎬️action.rs` `Trigger` (`#[serde(rename_all = "camelCase")]`) | `Activate` → `"activate"` on the wire | **correct** — matches the host's `b.trigger === "activate"` lookup |
| 2 | `🗣️Interpreter/🟦️.tsx` `treeItemToTreeData` | `onClick: activateBinding ? () => dispatchTrigger(context, record, "activate") : …` | **correct** |
| 3 | `🌳️Tree/🟦️.tsx` `TreeItem` | turns that `onClick` into DOM | **BROKEN — fixed this wave, see §2.2** |
| 4 | `🏛️ShellHost/🟦️.tsx` `applyLeftoverInteractionView` | `interactionViewFromLeftoverOutput(response.output)` → `publishLeftoverWorldSelectionV1(overlay)`; prints `[DEBUG] leftover InteractionView` | observable, see §2.3 |
| 5 | `🌐️World3dHost/🟦️.tsx` | `mergeWorldInteractionWithLeftoverV1(parseInteraction(scene.interactionJson), leftoverWorldSelectionOverlayV1())` → `data-interaction-json`, instance `selected`, and `leftoverTreeItemSelectedV1` → the row's `isSelected` | observable, see §2.3 |

### 2.2 Hop 3 — the defect, and the fix

`TreeItem` renders four row layouts (`default` × leaf/expandable, `property` × leaf/expandable). Only the
`default` LEAF layout put `onClick` on the `role="treeitem"` shell — the element carrying `id`. The
`default` EXPANDABLE layout wired it to `[data-slot="tree-label"]` alone, which `stopPropagation`s, so a
click on the row's padding, on the gap before the row actions, or on the stretch a short label leaves empty
reached nothing at all. (The two `property` layouts are the inspector's, and are left as they are: a
property leaf activates from its label, a property group FOLDS from it.)

The puzzle3d outliner's object rows are expandable — `object_row` nests the object's vortices through
`try_children(vortices)` — so `panel:puzzle3d-play-document/seed-left-001` is exactly such a row, which is
why B15 measured "a real object row is addressable and clicked by id, nothing ends up selected".

Second defect in the same component: NONE of `TreeItem`'s `role="treeitem"` rows published `aria-selected`
— not the leaf either (only the unrelated `SortableTreeItem` did). That is both an accessibility defect for
a `selectionMode="single"` tree and the reason B15 could read no selection off the DOM: "the only
`aria-selected` nodes are the two dock tabs".

Fixed in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`:

- `TreeItem`, `default` expandable layout: `onClick` moved to the row shell (`event.detail > 1` still
  yields to `onDoubleClick`); the label span keeps only its class.
- `TreeItem`, all three `role="treeitem"` rows it renders (property, `default` expandable, `default` leaf):
  `aria-selected={isSelected}`.
- `SortableTreeItem`, both expandable layouts: the same `onClick` move, for parity (it already published
  `aria-selected` everywhere).
- The `property` layout's expandable label deliberately keeps FOLDING rather than selecting — an inspector
  group heading is not an entity — and is documented as such; only its `aria-selected` was added.
- The fold chevron, the branch-navigation buttons, the row actions and the drag handles each already
  `stopPropagation`, so they keep owning their own clicks; verified by law.

The wgpu twin (`🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs` `render_tree_item`) never had this split: it registers ONE
`HitTarget` over `label_rect`, which spans from the label x to the row's right edge for every row,
expandable included. The React fix brings React into line with the twin; no wgpu change.

### 2.3 What the probe now measures at each hop

`selectViaOutliner` clicks the row's BACKGROUND first and, only if that lands nothing, its LABEL — the
background is the strictly harder target, so a background landing proves the label one too and a
background-only failure names the defect precisely. Three independent observables per attempt:

- `clickTreeRowPoint(id, "background"|"label")` — clicks by viewport rect and reports `data-tree-row-kind`,
  the label width, the row width and the element actually under the point. The background point is
  **searched, never assumed**: it scans the row at mid-height for the first x inside no descendant
  `button`/`role=button`/`input`/`a`, outside `[data-slot="tree-label"]`, and whose `elementFromPoint` is
  still inside the row. The first version took "row right edge − 10 px" and hit the row's `Lock` action,
  which LOCKED the object and made the next selection be refused — a measured false red.
- `worldInteraction()` — `data-interaction-json` per surface (`selectedIds`, `hoverTarget`) plus the
  `selected` flags inside `data-instances-json`. This is hop 5's output and, because the attribute merges
  the leftover overlay, also hop 4's.
- `leftoverViewTail(since)` — `[DEBUG] leftover InteractionView` lines that appeared since the click.
  Present ⇒ hops 1-4 ran and the failure is at hop 5. Absent ⇒ the failure is at hop 1-3.

The `context-menu-selection-precondition` note now carries
`rowKind=… labelWidth=…/… landedVia=background|label|neither dispatchObserved=… selectedIds=[…]`.

---

## 3 Context-menu precondition — a path that lands

`context-menu-rows` used to right-click the canvas at a hardcoded `0.78 × 0.42` fraction of its box, which
is wherever the framing happened to leave the table. It now projects a REAL instance:

- `projectInstance(id)` reads `data-instances-json` (`WorldInstanceRecord.position`) and
  `data-viewport-camera-json` (`world3dCameraDomJson`: `position`/`target`/`up`/`fov`/`projection`, fov
  defaulting to the host's own 45; the live pane reports 50) off `#puzzle3d-main-perspective`, builds the view basis and projects the
  instance to viewport pixels. Returns `null` when the point is behind the camera or no scene is published.
- `landOnInstance(id)` hovers that point and widens into a ±52 px spiral until a surface reports a
  `hoverTarget` or a selected instance, so the pick is CONFIRMED rather than assumed; it logs the offset it
  needed. On `#47` it always reports `projected-blind`, because no world surface ever publishes a hover
  (§6.3) — the point itself is right, as the menu that opens over it proves.
- The step right-clicks that point and left-clicks FIRST ONLY when the outliner selection landed nothing:
  a left click on the canvas is a pick that clears the selection, and clearing it is what produced
  `rows=0`. It falls back to the old fraction only when no instance is projectable, and logs
  `via=projected-hover|projected-blind|table-fraction`.
- The menu is POLLED for (up to 10 s), not waited for: `World3dHost.onContextMenu` awaits
  `openSurfaceContextMenu`, a round trip to the guest, before it calls `setContextMenu` — one fixed 1.5 s
  wait measured `rows=5` and `rows=0` on two otherwise identical runs.
- `menu.group.*` rows are hovered open and the submenu re-read before the vocabulary verdict, because
  `hide-show` and `lock-unlock` are authored INSIDE `menu.group.hand` — the guest's own law asserts that
  grouping (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`: "hide/lock rows should be grouped under hand"), so reading the
  top level alone reported them missing when they were merely folded.

---

## 4 Laws

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`, new region `🌲️RowActivation`,
four tests:

1. an expandable row activates from the row SHELL and from its label alike, and announces `aria-selected`
2. the fold chevron and a row action stay out of row activation
3. a double click on an expandable row goes to `onDoubleClick`, never to `onClick`
4. a leaf row announces `aria-selected="true"` when selected and `"false"` when not, and still activates

Both halves were proven to FAIL without the fix (see §5).

---

## 5 Verification

All foreground, all with `pgrep -f "browser[-]probe|lane[-]probe"` clear (the bracket form matters: a plain
`pgrep -f "browser-probe"` inside an `until` loop matches its OWN command line and never exits — it also
makes the lock look held to every other agent).

### 5.1 Probe, `#47` on `:6013`

Parse-only dry run, `bun 🔍️browser-probe.ts --only=boot --port=6013`:

```
[161.9s] battery PASS=3 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
[161.9s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=6
```

`bun 🔍️browser-probe.ts --only=context-menu-rows,outliner-rows --port=6013`
(`--only` takes STEP names; the task's list is VERDICT names — these two steps carry all of them). Full log
`🗑️generated/probe-2026-09-11T17-25-28.md` / `.ndjson`.

```
[80.5s] battery PASS=10 FAIL=3 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

| verdict | B15 (#46) | this run (#47, this wave's fixes) |
|---|---|---|
| `boot` | PASS | PASS |
| `context-menu-selection-precondition` | **FAIL** `selected=0` | **PASS** `landedVia=background dispatchObserved=true` |
| `context-menu-opens` | **FAIL** `rows=0` | **PASS** `rows=5` + 2 folded |
| `context-menu-object-vocabulary` | **FAIL** all six missing | **PASS** `duplicate, select-same-kind, zoom, menu.group.hand, delete, hide-show, lock-unlock` |
| `context-menu-zoom-row-action-is-registered` | FAIL (row absent) | **PASS** `zoom` → `focusSelection` |
| `context-menu-zoom-moves-camera` | not reached | **FAIL** — handed over, §6 |
| `outliner-panel-opens` | PASS | PASS |
| `outliner-hide-control-present` | — | PASS |
| `outliner-hide-applies` | — | **FAIL** — handed over, §6 |
| `outliner-show-restores` | — | **FAIL** (unreachable behind the above) |
| `guest-alive-mutate` | (new) | **PASS** `recovery=[] canvases=2 surfaces=[{"surface":"1","instances":1,…}] guestDeathFaults=0` |
| `battery-hard-faults` / `battery-faults` | PASS | PASS |

The measured row click, which is the whole hop-3 story in one line:

```
selectViaOutliner background spot={"x":146.1875,"y":62.59375,"part":"background","hit":"div#-[tree-gutter]",
  "kind":"group","has":true,"labelWidth":12,"rowWidth":160}
  leftoverLines=["warning: [DEBUG] leftover InteractionView {\"selectedIds\":[\"seed-left-001\"],
    \"publishedIds\":[\"seed-left-001\"],\"locked\":{\"seed-left-001\":false},\"gumball\":true,\"hoverTarget\":null}"]
  ariaSelected=["panel:puzzle3d-play-document/seed-left-001=Hexagonal Cut Concrete Forest Left Hide Lock"]
… landedVia=background dispatchObserved=true
```

`labelWidth=12/160` is the point: the object row's label span is **12 px of a 160 px row**, so B15's
`force: true` centre click landed on the row BACKGROUND — which, before this wave, was inert on an
expandable row. The background now dispatches (`div[tree-gutter]`, inside the row shell), the leftover
overlay publishes `seed-left-001`, and the row announces `aria-selected` — the second half being the
attribute this wave added, without which no probe could read the selection off the DOM at all.

The context menu confirms the selection reached the GUEST, not just the host: `5 Delete (1 Object)`.

### 5.2 Laws

`bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`

```
 Test Files  1 passed (1)
      Tests  12 passed (12)
```

Proven live against the code they cover — each half of the fix removed in turn, the law re-run, and the
file restored in the same command:

| removed | result |
|---|---|
| `aria-selected` + shell `onClick` on the expandable row | `Tests 1 failed \| 11 passed (12)` — `AssertionError: expected null to be 'true'` |
| the shell `onClick` alone | `Tests 1 failed \| 11 passed (12)` — `AssertionError: expected "vi.fn()" to be called 1 times, but got 0 times` |
| (restored) | `Test Files 1 passed (1)` / `Tests 12 passed (12)` |

### 5.3 Suites — no new failure

| suite | result |
|---|---|
| ui-react vitest (whole config) | `Test Files 5 failed \| 17 passed (22)` / `Tests 4 failed \| 708 passed (712)` |
| renderer-react vitest, `SEMIO_TEST_LEVEL=long` | `Test Files 3 failed \| 21 passed (24)` / `Tests 9 failed \| 877 passed (886)` |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json` | no error in `🌳️Tree/🟦️.tsx` or its test; the reported errors are pre-existing (`🧪️tests/🧪️docklayoutstore`) |

The renderer-react nine are **exactly** B13's and B15's nine (6 `🧪️tests/🧩️package-integration`, 2
`🔌️PluginRuntime`, 1 engine-contract `buildNoteShellCommandAction`). Of the ui-react failures, three
(`package entry and self-alias`, `UIDialog … nested owned kind picker`, `UIIntroduction … glass boxes`) and
the unresolvable `.storybook/🧪️tests/🧭️scope-resolution` file are B10/B13's pre-existing list. The fourth,
`Diagram … cursorizes live controlled 20,000-node`, is a wall-clock budget flake under fleet load: run
alone it is `Test Files 1 passed (1)` / `Tests 42 passed (42)`.

No Rust law was needed — the guest already authors the correct `interactionSelect` binding (§2.1 hop 0), so
nothing in `semio-s-artifact-puzzle-3d` changed this wave.

---

## 6 Handover — what is still red, with evidence

### 6.1 `context-menu-zoom-moves-camera` (§15)

The `zoom` row is registered and carries `focusSelection` (not the unregistered `zoomToSelection` the
checklist predicted), the selection is live (`5 Delete (1 Object)`), and clicking it leaves the Perspective
camera bit-identical:

```
before={"position":[11.5131,-3.767,5.9804],"target":[0,0,0],"up":[0,0,1],"zoom":1,"fov":50,"projection":"perspective"}
after ={"position":[11.5131,-3.767,5.9804],"target":[0,0,0],"up":[0,0,1],"zoom":1,"fov":50,"projection":"perspective"}
newFaults=0 newHardFaults=0
```

No fault is raised, so the action is accepted and dropped somewhere between `focusSelection` and the
viewport camera. Owner: whoever holds the camera/frame lane.

### 6.2 `outliner-hide-applies` / `outliner-show-restores` (§17)

`outliner-hide-control-present` PASSes — the row's `Hide` / `Lock` buttons are addressable
(`data-slot="action"`, one pair per row) — and clicking `Hide` leaves the row dump byte-identical, so
`Show` has nothing to restore. The guest authors those buttons as `setSelectionFlag` with the negated flag
(`flag_args(entity, id, "hidden", !hidden)`, `📌️panels/🗿️artifact/🦀️.rs`), so this is a hop past the
authoring. Not investigated further this wave.

### 6.3 Hop 5's WORLD half — the surfaces never reflect the selection

Every run, with a selection the guest itself acknowledges:

```
interaction=[{"surface":"1","selectedIds":[],"hovered":null,"selectedInstances":[]},
             {"surface":"1","selectedIds":[],"hovered":null,"selectedInstances":[]}]
surfaceStatus=["1=","1="]
```

The leftover overlay published `["seed-left-001"]` and the TREE half renders it (`aria-selected`), but
BOTH world surfaces publish an empty `selectedIds`, no selected instance, and an empty
`data-status-json` — and both report `data-surface-id="1"`, the same id for two panes. So either
`mergeWorldInteractionWithLeftoverV1` is not reaching these hosts' `data-interaction-json`, or the two
panes share one surface identity. This is B17's trap / the leftover lane's territory, not fixed here.

Consequence for measurement: `landOnInstance` always reports `projected-blind` — the projected point is
right (the context menu opens over it and the guest resolves the object), but no surface ever publishes a
`hoverTarget`, so the hover confirmation can never go green while 6.3 stands.

### 6.4 A note for whoever reruns the battery

`--only=` takes STEP names (`context-menu-rows`, `outliner-rows`), not verdict names. And the context menu
needs the outliner selection ALIVE: the step no longer left-clicks the canvas first when the outliner
landed, because that pick clears the selection and was what produced `rows=0`. A fixed post-right-click
wait is also not enough — `World3dHost.onContextMenu` awaits a guest round trip, so the step polls up to
10 s (`context-menu opened after poll rows=5`).
