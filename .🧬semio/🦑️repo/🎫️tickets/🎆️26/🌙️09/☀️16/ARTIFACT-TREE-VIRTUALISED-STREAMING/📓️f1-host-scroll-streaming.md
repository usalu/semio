# 📓️ F1 — the React host's scroll → report → refresh loop

Packet **F1**, wave 4, 2026-09-17. Inputs: `📓️wave4-resume-brief.md`, `📓️design-virtualised-tree.md` §6,
`📓️p4a-tree-element.md`, `📓️p4b-host-wiring.md`, `📓️w3-browser-verification.md` §5 and — mid-packet — the
coordinator's four normative decisions off `📓️s3-review-streaming-loop.md`.

Files changed (all absolute, all surgical; no `🗑️generated` folder swept, no ticket opened/closed/reopened):

| file | what |
| --- | --- |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | `TREE_WINDOW_PATH_SEPARATOR`, `treeWindowPathOf`, `TreeData{Item,Section}.windowPath`, `data-tree-window-path`; `TREE_WINDOW_BODY_ROWS_MAX` → `TREE_WINDOW_BODY_NODE_BUDGET` + `1 + rows` cost model in `capTreeWindowRequests`; `TreeWindowRowMeasure` + `TreeWindowContainerMeasure.rows`; `treeWindowRowIndexAt` (real row geometry) inside `treeWindowVisibleRowsForViewport`; `treeWindowRowIndexOf`; `data-tree-window-row` stamped on every materialised row (`TreeItemProps.windowRowIndex`, all three `TreeItem` layouts, threaded from both data views) |
| `…/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | re-export list only: renamed constant, `treeWindowRowIndexOf`, `TreeWindowRowMeasure`, `TREE_WINDOW_PATH_SEPARATOR`, `treeWindowPathOf` |
| `…/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx` | window-path law, row-index stamping law, variable-row-pitch law, `🧮️BodyRowsBudget` → `🧮️BodyNodeBudget` rewritten for the node cost model |
| `…/💻️os/…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` | `treeWindowScrollViewport` rewritten; `treeWindowDocumentScroller`, `treeWindowViewportMetrics`, `treeWindowRowsUnder`, `treeWindowBodyRequestsV1`, `reportDuplicateTreeWindowKeys` added; `treeWindowContainersUnder` re-based on client space + row measures + path keying; window paths computed in the walk; a windowed row's DOM id path-qualified; `useTreeWindowObserver` measure/listen rules |
| `…/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx` | 7 new laws (incl. one node key under two parents = two independent windows) (scroll-element selection ×3, client-box measurement, settled-window idempotence ×2, nested container, duplicate key), existing laws re-based on real row geometry |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `TreeWindowBodyState.measured`; `reportWindows` records measured + open; `setOpen(true)` forgets a stale measurement; `viewStateFields` fallback narrowed |
| `🎫️…/🐍️tree-window-probe.mjs` | selects containers by `[data-tree-window-path]` (same element, same ownership filters) and reports `path` beside `key` |
| `…/🧱️elements/🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx` | 4 new laws (off-screen vs never-measured, fold-wins-over-report, body switch, path-keyed fold of one of two containers sharing a key) |

`🏛️ShellHost/🟦️.tsx` was **not** touched — its wiring (scheduler, `settled`, `viewStateFields` in both view
states, per-tab memo cache) was already correct and is unchanged.

---

## 1. The defect W3 §5 found, and why the one-line fix was not enough

`treeWindowScrollViewport` returned `[data-slot="scroll-area-viewport"]`, the **content** div of
`📜️Scrollable`, whose height is its content's — `scrollHeight === clientHeight`, `scrollTop` permanently 0.
A previous agent had already changed it to prefer `[data-slot="scroll-area"]`. That is necessary but states
the fix as a selector, and a selector is exactly what was wrong the first time. The rule is now behavioural:

```
candidates = [the guest <Tree>'s own root, then every ancestor] that either carry data-slot="scroll-area"
             or compute overflow-y: auto | scroll | overlay
viewport    = the nearest candidate that ACTUALLY overflows (scrollHeight − clientHeight > 1)
            ?? the nearest 📜️Scrollable          (a body that has not grown past its box yet)
            ?? the nearest candidate at all       (a tree outside any 📜️Scrollable)
            ?? document.scrollingElement          (a tree on a plain scrolling page)
```

Three things this adds over the selector fix:

- **The guest `<Tree>`'s own root is in the chain.** It carries `overflow-auto`; wherever the chain above it
  bounds its height it *is* the scroller, and a walk that starts at `root.parentElement` steps straight over it.
- **`null` is no longer an answer.** A tree with no scrolling ancestor used to return early and never report a
  single window. It now measures against the page, and — because the page's own scroll event fires on the
  DOCUMENT, not on `documentElement` — the observer adds a second `scroll` listener on the window for that case.
- **It is re-resolved on every store revision**, so a first-paint body that only overflows once its rows arrive
  stops being measured against the wrong box.

### Measurement, re-based
`viewportHeight` is `clientHeight`, never `getBoundingClientRect().height`: the rect is the border box and
includes borders and a horizontal scrollbar, so the old reading reported rows the reader cannot see and ran the
overlap a row past the bottom edge. Container tops are now **relative to the viewport's content origin**
(`rect.top − (viewportRect.top + viewport.clientTop)`, with `viewportTop = 0`) instead of
`rect.top − viewportRect.top + scrollTop`. Only the difference `viewportTop − top` reaches the rule, so the two
spaces agree for an ordinary scroller — but the old one double-counted the scroll for the document scrolling
element (whose own rect moves with the scroll, `top === −scrollY`) and ignored the border on a bordered one.
A container whose rect is zero-height (a CLOSED section still renders its content element, `hidden`) is skipped.

## 2. Variable row pitch — coordinator decision (2)

`firstVisibleRow = floor(pixels / rowHeight)` is true only while every child of a container is one row tall. A
materialised row can itself be an open windowed group rendering its whole subtree, so every row after it sits at
an offset no uniform pitch predicts; the parent is then asked for a window far below what is on screen, which
**evicts the subtree the reader just opened**, which changes the geometry, which re-measures differently.

Now: every materialised row of a windowed container is stamped `data-tree-window-row="<offset + i>"` (on the
row's own outer element — leaf row, group header row, or property row), and the observer reads those rows'
real `getBoundingClientRect().top` the same way it reads the containers'. `treeWindowRowIndexAt` binary-searches
them; only the two spacer bands use the uniform pitch, where it is exact by construction. Without row
measurements the whole container falls back to the flat pitch, which is right for a container none of whose rows
is expanded (and is what the pure-helper tests exercise).

Ownership of a row is `element.closest("[data-tree-window-key]") === container`, **not** `:scope >` — a row with
a context menu is wrapped in one and would vanish from a direct-child query, while a nested group's own rows must
stay with that group. Rows are sorted by TOP, not by index.

`visibleRows` is likewise counted in the container's own rows (`last − first + 1`), not in viewport pixels: a
section whose entire visible extent is one expanded child needs **one** row of itself, not a viewport's worth,
and the body budget is spent where the rows actually are.

## 3. One body budget, one cost model — coordinator decision (1)

```ts
export const TREE_WINDOW_BODY_NODE_BUDGET = 103;   // F2 lowered 111 → 103 to match the real ledger
```

in `🌳️Tree/🟦️.tsx` (exact spelling, one line, for **F2's Rust parity law to grep**). It is a hand-written
const with the Rust constant named in its docstring — the schema generator emits no constants today, so there is
nothing to source it from yet.

`capTreeWindowRequests` now prices a body the way the guest's ledger does: **every container costs `1 + rows`**,
nested ones included, each counted once, summed over every windowed container in the body — not just the visible
ones. The trim order is unchanged (overscan first, uniformly, down to zero; then rows off the containers whose
centre is furthest from the viewport's, down to one row and finally to none). A container trimmed to zero rows
**stays in the answer** as a spacer-only window: it exists in the body either way and still costs its node, and
dropping it from the wire would only hand the guest back its own default window.

The observer runs the whole body through this: `treeWindowBodyRequestsV1(containers, viewportHeight, rowHeight)`
= the visible containers' windows ∪ every other container at `{offset: its current DOM offset, rows: 0}`, capped.

**Loading ring.** `isWindowPending` (open ∧ `total > 0` ∧ zero rows) is unchanged. With the budget matched, a
*visible* container is never answered zero rows unless more than ~55 windowed containers are on screen at once,
so the ring only ever shows while a request for rows that intersect the viewport is in flight. An off-screen
container at `rows: 0` does wear the ring, unseen, and resolves the moment it is scrolled into view. The residual
case — a body with >55 simultaneously visible windowed containers — is listed as O3 below.

## 4. Off-screen containers — coordinator decision (3)

The observer's report is now the **whole body**, so the scheduler's existing wholesale replacement of
`state.windows` *is* the pruning: a key absent from a report is a key that is not in the body (a folded ancestor,
a document switch), and a key that is merely scrolled past arrives explicitly at `rows: 0` with the offset it
already holds — scrolling back to it lands where the reader left it, and it never competes for rows nobody is
looking at. Offsets are clamped to `< total` on the way in (they come from `data-tree-window-offset`, which the
guest already clamped, and from `capTreeWindowRequests`, which keeps `offset + rows ≤ total`).

`TreeWindowBodyState` gains `measured: Set<nodeKey>`, which separates the two cases the `viewportRows`
first-paint fallback used to conflate:

| state | asked for |
| --- | --- |
| open, never in the body yet (the chevron was just clicked) | `viewportRows ?? TREE_WINDOW_DEFAULT_ROWS` at offset 0 |
| in the body, viewport covers it | its measured window |
| in the body, scrolled past | `rows: 0` at its last offset |
| was in the body, is not now | `rows: 0` (the open flag survives; a fold the reader chose must outlive a branch closing over it) |

An explicit `setOpen(key, true)` takes the key back out of `measured` and drops its stale window — that gesture
*is* "give me rows", and without it a container that was measured, folded and re-opened would ask for zero rows
and never appear.

A report also records `open: true` for every container it measured (a measured container is a rendered one), but
**never over an explicit entry** — a report in flight when the reader folds must not re-open what they just closed.

Request order stays byte-stable by `(bodyKey, nodeKey)`; every `rows > 0` request fits by the cap, so F2's
order-independence on the guest is what makes that safe.

## 5. Duplicate keys — coordinator decision (4)

`reportDuplicateTreeWindowKeys` fires one `console.error("[tree-window] duplicate key …")` per
`(bodyKey, nodeKey)` per mount. The host cannot repair it — the key is what the guest addresses its containers
by — so it names the key, the body, and what goes wrong (one open state and one window shared, one measurement
silently overwriting the other).

## 5b. A window is addressed by its PATH, not by its node key

The node key of a windowed container is also the **pick target id** the tree-level `interactionSelect`
dispatches (`targets: [{granularity, id: record.key}]`), so it cannot be namespaced to make it unique — and it
is not unique: `📐️cad` builds the same `object.id` under four pane sections of one body and `🏗️fem` builds
`case.id` and `combination.id` in one body (`📓️f2-sdk-body-node-ledger.md` §10). Under key identity those
containers shared one open state, one window, and each other's measurements.

```ts
export const TREE_WINDOW_PATH_SEPARATOR = "␟";   // 🌳️Tree/🟦️.tsx, one line, U+241F (PRINTABLE)
treeWindowPathOf(parentWindowPath, windowKey)          // enclosing windowed containers, outermost first, then its own
```

Byte-for-byte the SDK's `TreeWindows::path_of` (`🔌️plugin/🦀️.rs:6070`). **A top-level section's path IS its
key**, so every flat body — requests, view state, laws, probe expectations — is byte-identical to before.
A nested container's path is `parent␟child`.

🧯️ **The separator is U+241F SYMBOL FOR UNIT SEPARATOR, the printable glyph — not U+001F, the control
character it depicts.** A window path crosses the wasm boundary inside `PluginViewState.treeWindows`, and
`parseResolvedPluginViewState` (`🛂️manifest/🟦️.ts:977`) refuses any identifier matching
`[\u0000-\u001f\u007f]`. The view context is admitted as ONE object, so the first nested path took the whole
crossing down: on the fem3d House lane every `refreshUi` answered `view context: invalid identifier` and every
unrelated `interactionSelect` was refused `dispatch-failed` (`🗑️generated/w5/fem3d-house/report.json` steps
e9/h/i). U+241F is one code point, printable, greppable, absent from every authored node key, and passes.

**And the host no longer trusts its own paths.** `viewStateFields()` validates every path against the same
identifier law before flattening it (mirrored in `treeWindowSendableIdentifierV1`, with a law that runs the
REAL `parseResolvedPluginViewState` over the scheduler's output); a path that fails is OMITTED with one
`console.error("[tree-window] unsendable path …")` per body + path. Tree-window state is a convenience and
must never be able to break a refresh or an unrelated action again.

What is keyed by path: `TreeWindowBodyState.open` / `.windows` / `.measured`, the observer's container map and
report signature, `treeWindows[].nodeKey` on the wire, duplicate detection, and the controlled open-state
lookup. What is **not**: `data-tree-window-key`, `walk.pickTargets` and every dispatched pick — those stay the
authored `record.key`.

**One further collision had to go with it.** `<Tree>` keys its expansion map by the ROW's DOM id, and
`uiNodeDomId(surface, key, …)` is `<surface>/<key>` — so the two `shared` rows above carried the *same* HTML
id and folded together whatever the host asked. A windowed container's row id is therefore
`uiNodeDomId(surface, windowPath, …)`; identical to before for a top-level container, and `walk.pickTargets`
still maps that id back to the bare `record.key`, so the pick does not notice. Unwindowed rows are untouched.

The path is computed **structurally**, while the interpreter walks records to `TreeData` (each row is handed
its parent's window path), and mirrored into the DOM as `data-tree-window-path` — one attribute read, no
ancestor walk in the observer, and a path that is visible to the browser probe.

## 6. No request/refresh feedback loop

Three independent stops, all now true by construction:

1. **The answer is a function of the container geometry alone** — `total`, the container's extent, its rows'
   tops. A window's own answer reproduces the geometry that asked for it: the leading spacer grows to `offset`
   rows exactly as the slice moves down by `offset` rows, the total extent is unchanged, the scroll position is
   unchanged, so the recomputed request is byte-identical. Proved as a law
   (`asks for exactly the window it already has once the guest has answered`): before = `{offset 0, len 1}` →
   asks `{32, 26}`; after = `{offset 32, len 26}` with the slice at its real tops → asks `{32, 26}`.
2. **`treeWindowReportSignatureV1`** — an identical answer calls nothing, so a scroll frame that lands inside the
   same row, a `ResizeObserver` tick and a store revision all cost zero refreshes.
3. **`createTreeWindowSchedulerV1.reportWindows`** — an unchanged map schedules nothing.

Scroll-position stability is the leading spacer's job and is unchanged from P4a: the container's extent is
`total × rowHeight` whatever the window, so moving the window never moves the scrollbar.

The `ResizeObserver` now also observes the tree root, not just the viewport: a sibling branch folding or a nested
window streaming in changes which rows this one shows under a fixed viewport, and that must re-report without
waiting for the next store revision.

---

## 7. The DOM / scroll contract `🐍️tree-window-probe.mjs` step (e) must now satisfy

Per panel body, with `scroller` = the element `treeWindowScrollViewport` resolves (in practice
`[data-slot="scroll-area"]`, the one with `scrollHeight − clientHeight > 1`). **`data-tree-window-path` is new**;
nothing else in this contract changed with path identity, and on a flat body every value is what it was.

| # | assertion |
| --- | --- |
| e1 | `scroller.scrollHeight − scroller.clientHeight > 1` — the observed element really scrolls. The inner `[data-slot="scroll-area-viewport"]` must NOT be the scroller (`scrollHeight === clientHeight` there). |
| e2 | Every windowed container carries `data-tree-window-{key,path,total,offset,length}` on its `tree-section-content` / `tree-item-content` / `tree-property-content` element, with `offset + length ≤ total`. |
| e2b | **`-path` is the identity**, joined by `␟` (U+241F, printable): a top-level container's `-path` equals its `-key`; a nested one's is `<parent path>\u001f<key>`. The same `-key` may appear twice in a body (that is legal and not a duplicate); the same `-path` may not. Every `treeWindows[].nodeKey` the host sends equals some container's `-path`. |
| e3 | Each such container's own rows carry `data-tree-window-row`, values exactly `offset … offset+length−1` in ascending top order, selected by `[data-tree-window-row]` filtered on `closest('[data-tree-window-path]') === container`. |
| e4 | Container extent: `rect.height ≈ total × rowHeight + Σ(nested expanded extents)`; leading spacer `data-tree-window-rows === offset`, trailing `=== total − offset − length`; a zero-row spacer is absent. |
| e5 | **Streaming.** Set `scroller.scrollTop = k × rowHeight` for a `k` well past the current window, wait ≤ 500 ms, re-read: `data-tree-window-offset` has moved and the new `[offset, offset+length)` covers the row under `scroller.scrollTop` — i.e. `offset ≤ rowAt(scrollTop) < offset + length`, where `rowAt` is read off the `data-tree-window-row` tops, not off `scrollTop / rowHeight`. |
| e6 | **Stability.** `scroller.scrollTop` and `scroller.scrollHeight` are unchanged (±1 px) across that refresh. |
| e7 | **Idempotence.** With the scroll position held, no further `refreshUi` for that body occurs in the next 1 s (count `partial` refreshes for `panelBodies: [bodyKey]`), and `data-tree-window-offset` stops moving. |
| e8 | **Budget.** `Σ over the body's containers of (1 + data-tree-window-length) ≤ TREE_WINDOW_BODY_NODE_BUDGET` (103 as of 2026-09-17; read the constant, do not hard-code it), and no `nodes: N vs max_nodes` fault in the console. |
| e9 | **Lazy expand.** Click a closed container whose `total > 0`: the header wears `.border-loading` and its content has a full-`total` spacer for at most one refresh, then `length > 0` and the ring clears. |
| e10 | **Nesting.** Expand a group inside a windowed section, scroll into it: the SECTION's `data-tree-window-offset` still covers that group's own row index (the group's content element is still in the DOM), and the group's `-path` is `<section key>\u001f<group key>`. |
| e11 | No duplicate `data-tree-window-path` within one body, and no `[tree-window] duplicate key` console error. (A duplicate `-key` under different parents is expected and must NOT be reported.) |
| e13 | **No view-context rejection.** No `view context: invalid identifier`, no `[os-shell] tree window refresh failed`, no `[tree-window] unsendable path` in the console — every path the host sends is a legal identifier. |
| e14 | **Nothing open stays empty.** No container is `total > 0 && length === 0` for longer than one refresh once it is in the DOM: an off-screen one that has never materialised is seeded with one row, and a visible one gets its whole window. |
| e12 | Unchanged from before: no `.more` key, no `+N` label anywhere in the body. |

## 7b. The w5 browser probe's failures, triaged

| step | lane | verdict |
| --- | --- | --- |
| **i** — console clean | fem3d | **Product, fixed.** `view context: invalid identifier` on every refresh: the U+001F separator (§5b). Fixed by U+241F + the `viewStateFields` identifier guard. |
| **h** — leaf pick selects | fem3d | **Product, same cause.** The pick's dispatch view state carries `treeWindows`, so one nested path refused every `interactionSelect` as `dispatch-failed`. Nothing wrong with the pick itself. |
| **e9** — lazy expand | fem3d | **Product, same cause.** Expanding `combinations` needs a refresh to materialise its 2 rows, and every refresh was being rejected. |
| **e10** — parent shows the owner row | both | **PROBE defect, fixed in `🐍️tree-window-probe.mjs`.** `ownerRowIndex` used `el.parentElement.closest("[data-tree-window-row]")`, but `TreeItem` renders an expandable row as a fragment of `[header row, branch content]`, so the header carrying `data-tree-window-row` is a **preceding SIBLING** of the content element, never an ancestor — `closest()` could not reach it and returned `null` for all nine containers, nested or not. Now: the owner is the last of the PARENT's own `[data-tree-window-row]` elements whose top is ≤ this container's. |
| **e9** — lazy expand | **cad** | **Both.** (a) *Product*: a default-open container below the fold was answered `rows: 0` on first paint, which renders as an open container with a full-height spacer, no rows and a pending ring — `structure-classic` 0/11 at a 440 px viewport. It now gets a one-row seed while off screen (`TREE_WINDOW_OFFSCREEN_SEED_ROWS`), and its real window within one refresh of scrolling in; new law `seeds a never-materialised container below the fold, then gives it a real window when it scrolls in`. (b) *Probe*: `prepared: "already-closed"` was inferred from `rows === 0`, but that container was **open** — its header's `aria-expanded` is `"true"` (`isExpandable` is `total > 0`, `open` is the guest default) — so the probe's click FOLDED it instead of expanding it, and it stayed at zero rows for the whole 15 s. The probe must read `aria-expanded` / `data-state`, never row count, to decide whether a container is closed; the ring it saw on `cad-play-document.shape` is the same "open, nothing materialised yet" state on a different container. |

The user-visible law both halves now satisfy: **a container scrolled into view materialises its rows within one
refresh, and no open container stays at zero rows.** `e14` in §7 states it for the probe.

## 8. What was run, in the foreground, and what it said

| gate | result |
| --- | --- |
| `@semio-tech/ui-react` — `test long "🌳️Tree"` | **32 passed / 32** (23 before this packet). |
| `@semio-tech/ui-react` — `test long` (whole corpus) | **17 failed / 761 passed (778)**, 2 files. **Zero in `🌳️Tree`** (`grep -c "FAIL .*🌳️Tree"` → 0). Pre-existing / peer-owned — evidence below. Log: `🗑️generated/f1/vitest-ui-react.txt`. |
| `@semio-tech/ui-react:typecheck` | 664 errors repo-wide, **0 in any file this packet wrote**. The only `🌳️Tree` lines are the six P4a already recorded as pre-existing (`📖️stories` 66/100, `🧪️tests/🧩️component` 111/153/154/169), byte-identical. `🌳️Tree/🟦️.tsx` and `🎯️targets/⚛️react/🟦️.tsx`: zero. Log: `🗑️generated/f1/typecheck-ui-react.txt`. |
| `@semio-tech/framework-renderer-react` — `test long "🗣️Interpreter"` | **113 passed / 113** (the whole in-source Interpreter corpus, including the 13 tree-window laws). Log: `🗑️generated/f1/vitest-renderer-interpreter.txt`. |
| `@semio-tech/framework-renderer-react` — `test long "ShellHelpers"` | **22 passed / 22** across 3 files (tree-windows 11, `🧩️component`, `⏯️tool-run-panel`). Log: `🗑️generated/f1/vitest-renderer-shellhelpers.txt`. |
| `@semio-tech/framework-renderer-react:typecheck` | 858 errors repo-wide, **0 in any line this packet wrote**. The four in files it touches are byte-identical to `HEAD` (verified with `git show HEAD:<path>`): `🗣️Interpreter/🟦️.tsx(308,24)` `treeSectionsCatalogueDragMime`, `(2228,164)` `import.meta.dir`, `🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder(140,45)`, `🛠️ShellHelpers/🟦️.tsx(656,26)` `Uint8Array`/`BlobPart`. All four are TS-lib/typing drift on untouched lines. Log: `🗑️generated/f1/typecheck-renderer-react.txt`. |

`test long` on the WHOLE renderer corpus still exceeds its own 300 s harness budget (pre-existing, P4b's O3), so
the two suites above were run by path filter.

### The 17 ui-react failures are not this packet's

- 16 of them are in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s own in-source tests; the other is
  `📨️UIDialog`. This packet's only edit to that react-target file is three names added to the
  re-export lists — the rest of its working-tree diff is a **peer's** concurrent rewrite of
  `normalizeEngagementActionText` / `applySearchSpaceAction` / the new `searchSpaceConfirmsLine` (that peer's own
  `Shell components > Search…` failures were still red at 10:43 and green by 11:21, with no change from me).
- Several fail with `ENOENT` on paths that do not exist in the repo at all
  (`🧰️framework/🎨️styling/🖌️ui/🎨️.css`, `🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`) — stale fixture
  roots, recorded as pre-existing by P4a on the same corpus.
- The rest are icon-keyframes / celebrate / context-menu / control-chrome / `UIIntroduction` / `WindowChrome` /
  `UIDialog` / `Diagram`-perf assertions. None touches a tree, a window, paging, `setPanelPage`,
  `UI_BUILT_CHILDREN_MAX` or `UI_VALUE_PAGE_ROWS`. `tree helpers > WindowChrome stamps data-dim` is P4a's
  documented in-suite pollution case (green in isolation).
- P4a measured 17 on this corpus before wave 4; the count has moved in both directions with peer churn in that
  one file and has been 17, 18 and 24 during this packet — never once inside `🌳️Tree`.

## 9. A peer's broken file blocked the renderer gates for 45 minutes

From 10:38 to ~11:15 every renderer invocation died in `vite:esbuild` on two files **this packet does not
touch**, both being rewritten by a peer with a brace-stripping automated edit (`if (x) { … }` → `if (x)   …`,
the signature of `📓️project-codex-rename-plan-codemod-incident.md`):

```
🧱️elements/🏛️ShellHost/🟦️.tsx:1839/6801/7299  ERROR: Cannot use a declaration in a single-statement context
🧱️elements/🔌️PluginRuntime/🟦️.tsx:2950        ERROR: Unexpected "}"
```

Every `🗣️Interpreter` / `🛠️ShellHelpers` test imports through one of them. I did not touch either file. The peer
landed their fix around 11:15 and **all the gates in §8 were then run for real, against the real modules** —
the numbers above are those runs, not a workaround. (While blocked I ran the changed helpers as a transplant
harness under `🗑️generated/f1`, 11/11 green; it has been deleted now that the real suites pass.)

## 10. What F2 must match

1. **`TREE_WINDOW_BODY_NODE_BUDGET`** lives on one line in
   `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`, spelled exactly like that, for the Rust parity law to
   grep. F2 has since lowered it 111 → **103** to match the real ledger; the Tree tests now DERIVE their
   expectations from the constant rather than pinning a number, so the next parity change costs no test edits.
2. **The cost model the host prices with is `1 + rows` per windowed container, every container in the body,
   nested ones once.** The host guarantees `Σ(1 + rows) ≤ TREE_WINDOW_BODY_NODE_BUDGET` over the whole report. If the guest charges
   anything else — a nested `tree_window_item` twice, a per-body fixed headroom on top — the two sides diverge
   again exactly as §1 of the review describes.
3. **Every container in the body appears in `ViewModel.tree_windows`**, including off-screen ones at `rows: 0`.
   Those must materialise spacers only and cost one node. They are sorted by `(bodyKey, nodeKey)`, so the guest
   must be order-independent.
4. **`rows: 0` must not be read as "closed"** — the container stays open and keeps its `offset`.
5. **Window identity is the PATH** (`TreeWindows::path_of`, verified byte-identical to
   `treeWindowPathOf`): enclosing windowed containers' keys, outermost first, then its own, joined by U+001F;
   a top-level container's path IS its key. Every `treeWindows[].nodeKey` the host sends is a path. `node_key`
   in `TreeWindowRequest` therefore means "path"; the authored key is only ever the pick target id.
6. **`TREE_WINDOW_PATH_SEPARATOR` is now `"␟"` (U+241F), not U+001F** — see §5b. The Rust constant and the
   parity law (`🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs:267`, which still matches `\u001f`) must move with it,
   or a path the host sends will not be the path the guest resolves.
7. **Duplicate `node_key` within one body**: the host now logs `[tree-window] duplicate key …` once, on a
   duplicate **path**. Two containers sharing a node key under different parents are legal and silent. F2's SDK
   refusal should name the same thing so the two messages read as one fault.

## 11. Open issues

- **O1 — nothing is "written, not run".** Every gate in §8 was executed after the peer's file parsed again.
  What is NOT covered by a test is the browser: §7's contract has not been re-measured by
  `🐍️tree-window-probe.mjs` on any lane, because starting serves is the coordinator's job.
- **O2 — `direction === "up"` trees.** The spacer bands swap DOM order there, and the two uniform bands in
  `treeWindowRowIndexAt` assume leading-on-top. Row measurements are sorted by top so the materialised band is
  still correct, but an `up` tree's spacer arithmetic is not. No guest tree renders `up` today (`TreeView` never
  passes `direction`), so this is latent, not broken.
- **O3 — a body with more than ~55 simultaneously visible windowed containers** cannot give every container even
  one row inside 111 nodes, and `capTreeWindowRequests` will take some to `rows: 0` while they are on screen —
  those wear a loading ring that does not resolve until the reader scrolls. That is the budget being genuinely
  exhausted, not a host defect; it wants an app-side answer (fewer top-level containers) if it ever shows up.
- **O4 — `treeViewportRows` is a running maximum** and never shrinks when a panel is made smaller (P4b's O5,
  unchanged). It only widens a never-measured container's first-paint request.
- **O5 — wgpu is still unwired** (design §6.4, P4b's O6). `TREE_WINDOW_BODY_NODE_BUDGET` and the row-index
  contract have no wgpu counterpart; that target paints spacers but requests nothing.
- **O6 — `O1` of P4b stands**: the Interpreter still restates `🌳️Tree`'s private role-prefixed open-state id
  convention. Unchanged by this packet.
