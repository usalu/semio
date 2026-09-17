# 📓️ S3 — Review: correctness of the end-to-end lazy/streaming tree loop

Read-only review, 2026-09-17 ~10:15. **Two peers were actively editing the SDK and host-observer code while
this review was being written** (F1 on the host observer, F2 on the SDK ledger) — every finding below cites
the mtime of the file as read, and the plugin SDK region in particular was rewritten under me mid-review (see
§0). Re-check line numbers if either file moves again before this is acted on.

Files read (mtime at read time):
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — Sep 17 10:06
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — Sep 17 09:17 (region read), file later touched again at 10:09, re-checked unchanged in the cited region
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — Sep 17 09:46
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — Sep 17 01:07 (re-checked unchanged at 10:12)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelWindowing` — read twice, 10:11 and again after it changed under me; the version cited below is the **second** read (post-reservation-ledger rewrite)

## 0. What changed under this review

Neither `p3-sdk.md` nor `p4a-tree-element.md` describes the code currently on disk. Two extensions exist that
no design doc mentions:
- **Host**: `capTreeWindowRequests` / `treeWindowVisibleRowsForViewport` / `TREE_WINDOW_BODY_ROWS_MAX = 112`
  (`🌳️Tree/🟦️.tsx:722-867`), wired into `useTreeWindowObserver` (`🗣️Interpreter/🟦️.tsx:1553-1599`). This is a
  body-wide row cap added after `📓️w3-browser-verification.md` §6.2 caught the SDK faulting at
  `nodes: 129 > max_nodes: 128`.
- **Guest**: `TreeWindows` was rewritten, while this review was in progress, from a flat
  "charge in document order" ledger to a **reservation ledger** — every host request reserves
  `min(rows, 128) + 1` up front, "in request order," before any container is built, specifically to stop an
  early-built container starving a later, host-addressed one (`🔌️plugin/🦀️.rs:5978-6110`).

Both are real fixes for real, previously-observed faults. §1 below shows they do not compose to a safe bound.

## 1. SEVERITY: HIGH — CONFIRMED — the host's row cap and the guest's node ledger use different constants and don't add up, so a single windowed section near the cap silently under-materialises every render

**Where:**
- Host cap: `🌳️Tree/🟦️.tsx:736` — `export const TREE_WINDOW_BODY_ROWS_MAX = 112;` (comment: *"112 is 128 less
  headroom the root, its sections and the group rows above the leaves occupy"*).
- Guest ledger: `🔌️plugin/🦀️.rs:6040` — `const LEDGER: usize = UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM;`
  with `UI_DOCUMENT_NODES = 128` (`🧬️contract/📃️document/🦀️.rs:95`) and `TREE_WINDOW_FIXED_NODE_HEADROOM = 16`
  (`🔌️plugin/🦀️.rs:5949`) → **`LEDGER = 111`**.
- Debit path: `tree_window_section` (`🔌️plugin/🦀️.rs:6144-6154`) calls `windows.debit(1)` for the section's own
  node **before** calling `windows.slice(...)`, which then `grant()`s the row request out of the **same**
  111-record pool (`🔌️plugin/🦀️.rs:6088-6110`).

**Failure scenario:** a body with exactly ONE open windowed section, no other containers, no nesting. The
observer measures it, the request is capped to at most `TREE_WINDOW_BODY_ROWS_MAX = 112` rows
(`🌳️Tree/🟦️.tsx:861-877`, `capTreeWindowRequests`). On the guest: `debit(1)` → ledger 111→110. `slice()` then
`grant(min(112, 128, total-offset)) = grant(112)`, but `nodes_remaining() = 110`, so `len = 110`. **The host
asked for 112 rows and receives 110, every single render**, because the render always starts a fresh 111-slot
ledger and the section's own node is not accounted for in the host's 112-row budget at all. This is not a
one-off race — it reproduces identically forever as long as the request stays the same, since neither side's
constant depends on the other and nothing narrows the gap over time. The TS comment's mental model ("16 covers
root + windowed sections + group rows, 112 is pure leaf capacity") does not match the Rust ledger's actual
accounting: `TREE_WINDOW_FIXED_NODE_HEADROOM` is reserved for a **different, unrelated** category — plain
`PanelTreeBuilder::section`/`tree_item` rows that carry no window at all (`🔌️plugin/🦀️.rs:5935-5941`) — not for
the windowed sections' own container nodes, which are debited out of the *same* 111 the rows compete for. Any
body with more than one open windowed container, or any nesting, only widens the shortfall — the SDK doc
comment even says so explicitly now: *"a nested `tree_window_item` is charged twice ... so nesting
under-materialises"* (`🔌️plugin/🦀️.rs:5999-6001`).

**Consequence for the user:** a container's spacer arithmetic stays internally consistent (no crash, no `+N`;
`data-tree-window-length` always reflects the true row count), but the last few rows of a large requested
window never arrive — the loading ring never resolves for them, because the shortfall recurs identically on
every refresh. This is the same class of bug §6.2 of `w3-browser-verification.md` caught (`129 vs 128`), just
shrunk from "whole-body fault" to "silent, permanent partial truncation," which is *harder* to notice in
manual testing because nothing errors.

**Suggested fix:** derive `TREE_WINDOW_BODY_ROWS_MAX` from the *same* constants the guest ledger uses, not an
independently chosen number: `TREE_WINDOW_BODY_ROWS_MAX = UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM - (expected open container count)`, or simpler and safer, make the guest tell the host how much it can
afford (e.g. stamp `nodes_remaining`/`reserved` back on the response, letting the host's cap adapt) rather than
keeping two independently-authored magic numbers in sync by hand across a wasm boundary.

## 2. SEVERITY: HIGH — CONFIRMED — the row-index↔pixel mapping assumes every row in a windowed container is exactly one row tall, but a materialised row can itself be an open, taller, nested windowed container

**Where:**
- `treeWindowVisibleRowsForViewport` (`🌳️Tree/🟦️.tsx:770-787`): `visibleRows = Math.ceil(overlapPx / rowHeightPx)`
  (:782) and `firstVisibleRow = Math.floor(Math.max(0, viewportTop - container.top) / rowHeightPx)` (:784).
- `treeWindowRequestAtRows` (`🌳️Tree/🟦️.tsx:790-793`) turns `firstVisibleRow` directly into the requested
  `offset` for that container.
- `TreeDataItemView` (`🌳️Tree/🟦️.tsx:3204-3302`) recursively renders `childItems.map((childItem) =>
  <TreeDataItemView .../>)` (:3291-3293) — i.e. a materialised row of a windowed container is a full nested
  `TreeItem` that can independently be `open` and windowed (`tree_window_item`, design §5: "object › vortices,
  load case › loads"). This is not hypothetical: `w3-browser-verification.md` §4's fem3d House table already
  shows exactly this nesting live — `fem3d-play-artifact.combinations` (1/2 materialised) containing the `uls`
  container (2/3), itself a separate windowed group with its own spacer.

**The bug:** `container.top`/`container.height` (measured via `getBoundingClientRect()`, per P4a §1.3) DO
correctly capture the true, variable-height extent of a container's content (spacers + rows + any expanded
nested content) — that part is sound. But `firstVisibleRow` and `visibleRows` both convert a *pixel* distance
into a *row count* by dividing by the single scalar `rowHeightPx`, which is only valid if every one of that
container's own immediate child rows is exactly one row tall. As soon as one of a container's materialised
rows is itself an **open** nested windowed item — rendering at `rowHeightPx × (1 + spacer/rows/spacer for its
own children)` instead of `rowHeightPx` — every row *after* it in that parent's list sits at a pixel offset
the uniform-pitch division cannot predict.

**Concrete scenario:** parent container "combinations", `total = 50`, materialised window `offset:0, len:10`.
Row index 3 of those 10 is an open nested item ("uls") whose own window shows 15 of its own rows, so it renders
at `~16 × rowHeightPx` instead of `1 × rowHeightPx`. The user scrolls the ancestor `Scrollable` down past this
expanded region. At the point where the viewport top has moved `20 × rowHeightPx` past `container.top`,
`firstVisibleRow = floor(20) = 20` — but the *real* row index at that scroll position is still inside the
first 10 materialised rows (rows 0-9 occupy roughly `9 + 16 = 25` row-heights of actual pixel space, not 10).
The container is asked for an `offset` around row 12-20, which is **outside** what's already materialised and
does not correspond to what's actually on screen at that scroll position. The next render changes the
container's own geometry (a different row may now be the tall one, or the previously-tall row collapses back
under contention with §1's ledger), which changes `container.height`, which re-triggers the `ResizeObserver`,
which re-measures and computes a *different* wrong `firstVisibleRow` — this is exactly the "oscillation
between two windows" the task brief asks to rule out, and it is not ruled out: nothing in `treeWindowRequestsForViewport`, `treeWindowReportSignatureV1`, or the 40 ms debounce accounts for a
container's own row pitch varying by row.

**Why this hasn't been seen yet:** `w3-browser-verification.md` §5 shows step (e) — the only step that
actually scrolls — was **SKIP** on every lane, because the observer was bound to the wrong element
(`scroll-area-viewport` instead of `scroll-area`). That defect has since been fixed on disk
(`treeWindowScrollViewport`, `🗣️Interpreter/🟦️.tsx:1503-1511`, confirmed present at time of reading) but **not
re-verified in a browser**. Once real scrolling works, this is the next thing standing between the design and
a correctly streaming nested container — cad and fem3d's flat catalogues never nest a windowed item inside a
windowed item's *materialised* (as opposed to closed) rows deep enough to trigger it during w3's runs, but the
mechanism (`tree_window_item` inside `tree_window_section`'s rows) is exactly what P3's own API supports and
what fem3d's `combinations`/`uls` pair already demonstrates in production data.

**Suggested minimal fix:** stop deriving `firstVisibleRow`/`visibleRows` from a pixel-distance ÷ uniform-pitch
calculation. Stamp each MATERIALISED ROW of a windowed container (not just the container itself) with a
measurable marker (e.g. `data-tree-window-row`, direct children only — `:scope >`), and have the observer find
"how many of the container's own rows precede the viewport top" by walking/binary-searching those rows' actual
`getBoundingClientRect()` tops, the same way it already measures containers. This keeps the existing container-
level `top`/`height` contract (still needed to decide whether a container intersects the viewport at all) but
replaces the *within-container* row arithmetic with real geometry instead of an assumed constant pitch. This is
additive to the existing observer (one more `querySelectorAll` per container) and does not change the wire
contract (`TreeWindow{total,offset}` and the DOM `data-tree-window-*` attributes are untouched).

## 3. SEVERITY: MEDIUM — CONFIRMED — a container that is "open" but currently off-screen keeps re-requesting a full viewport's worth of rows every render, competing with visible containers for the same shared budget

**Where:** `createTreeWindowSchedulerV1.viewStateFields()`, `🛠️ShellHelpers/🟦️.tsx:2169-2184`, specifically
line 2180:
```ts
const rows = window?.rows ?? (open === false ? 0 : (viewportRows ?? TREE_WINDOW_DEFAULT_ROWS));
```
`nodeKeys` is the union of `state.open.keys()` and `state.windows.keys()` (:2173). `state.windows` for a given
node key is **replaced wholesale** on every `reportWindows` call with whatever the observer currently measures
(`🌳️Tree/🟦️.tsx` + `🗣️Interpreter/🟦️.tsx:1573-1576` only report containers that overlap the viewport —
off-screen ones are dropped, per `treeWindowVisibleRowsForViewport`'s `if (overlapPx <= 0) continue`). So a
container the user opened, scrolled through, and then scrolled *past* still has `state.open.get(key) === true`
but no entry in `state.windows` — and the fallback above sends `rows: viewportRows ?? 48` for it, at `offset: 0`
(`window?.offset ?? 0`, also :2181), i.e. **as if it had just been opened and never measured**, indistinguishable
from the genuinely-just-opened case this fallback exists to serve (the comment at :2177-2179 is explicit about
only that intended case).

**Consequence, cross-checked against both ledger designs in `🔌️plugin/🦀️.rs`:**
- Under the *previous* document-order ledger, an off-screen-but-open container charged its rows unconditionally
  before any later container in the body, regardless of visibility.
- Under the *current* (mid-review) reservation ledger, "request order" is not visual/document order at all —
  it is the order requests appear in `ViewModel::tree_windows`, which `viewStateFields()` sorts by
  **node key, alphabetically** (`🛠️ShellHelpers/🟦️.tsx:2171-2173`, and P4b §4 confirms this is deliberate, for
  wire byte-stability). An off-screen container whose node key sorts earlier reserves
  `min(rows, 128) + 1` (`🔌️plugin/🦀️.rs:6027-6032`) ahead of a currently-visible container that sorts later,
  purely by name — the exact starvation the reservation ledger was written to prevent, re-introduced one layer
  up, because the host cannot tell the guest "this one is off-screen, don't bother."

Also note this resets the off-screen container's remembered scroll position to `offset: 0` (not its last known
offset) — a container the user scrolled halfway through, then scrolled away from, then scrolls back to, will
briefly show it from the top again rather than where they left it, until the observer re-measures.

**Suggested fix:** distinguish "opened, not yet measured" (should get the `viewportRows` fallback at offset 0)
from "was measured, now off-screen" (should send `rows: 0` or simply be omitted from the flattened request, or
at minimum keep its last known `offset`/`rows` rather than resetting to the first-paint default) in
`TreeWindowBodyState` — e.g. keep the last-seen `{offset, rows}` per node key even after it drops out of the
observer's current report, and only use the `viewportRows` fallback when no entry has ever existed for that key.

## 4. SEVERITY: LOW/MEDIUM — PLAUSIBLE, not observed triggered — `node_key` collisions are unenforced and silently misroute state

**Where:**
- Guest: `TreeWindows::seat`/`request` (`🔌️plugin/🦀️.rs:6075-6081`) does a linear `find` by `node_key` string
  with no uniqueness check — two different windowed containers built with the same author `id` in the same
  body share one entry.
- Host: `treeWindowVisibleRowsForViewport`'s `visible.set(container.key, ...)` (`🌳️Tree/🟦️.tsx:784`) and
  `treeWindowContainersUnder` (`🗣️Interpreter/🟦️.tsx:1516-1530`) key a `Map`/array by the same
  `data-tree-window-key` string with no dedup or collision detection — the later element in DOM order silently
  overwrites the earlier one's measurement in the `Map`.

**Scenario (not observed in the audited apps, but nothing prevents it):** an app author reuses an id — e.g. a
top-level section and a nested group item both happen to use `"materials"` as their window's node key (plausible
across independently-written panel builders sharing one body, or a catalogue id that coincides with an entity
id). The host tracks one open/window state for `"materials"`; scrolling or opening either container moves the
*other* container's window too, and the geometry `Map` silently drops one of the two real measurements. Given
`design-virtualised-tree.md` never states an app-wide (vs. per-container) uniqueness requirement for these ids,
and P3/P4a's tests don't cover a collision case, this is a real gap rather than a defended invariant — flagging
for wave-2 app authors and for a cheap debug-assert in `TreeWindows::seated` (duplicate `node_key` within one
body) rather than as something currently broken.

## 5. Status of previously-known defects (not new findings — included for completeness)

- `w3-browser-verification.md` §5 (observer bound to the non-scrolling `scroll-area-viewport` instead of
  `scroll-area`) — **fix present on disk**: `treeWindowScrollViewport` (`🗣️Interpreter/🟦️.tsx:1503-1511`) now
  walks up from the tree root, nearest-first, checking `[data-slot="scroll-area"]` before falling back to
  computed `overflow-y: auto|scroll`. Not re-verified in a browser since this landed.
- `w3-browser-verification.md` §6.2 (`nodes: 129 > max_nodes: 128` on fem3d House) — **partially addressed**:
  the new `TreeWindows` reservation ledger (`🔌️plugin/🦀️.rs:5960-6110`) and the host's
  `TREE_WINDOW_BODY_ROWS_MAX` cap (`🌳️Tree/🟦️.tsx:736`, wired at `🗣️Interpreter/🟦️.tsx:1576`) both exist
  specifically to prevent this. §1 above shows the two numbers don't actually compose to a safe bound yet, so
  the fault class is narrowed but not closed. Recommend re-running `🐍️tree-window-probe.mjs` (all three lanes)
  once §1 and §3 are fixed, since both bear directly on whether fem3d House's `combinations`/`uls`/`nodes`/
  `supports` combination reconciles cleanly.

## 6. Areas checked with no confirmed bug found

- **Hash-conditional re-render** (`plugin_refresh_ui`, `🔌️plugin/🦀️.rs` region around `ui_refresh_section`):
  traced that `window.offset`/`window.total` are always part of the serialised `node.root` JSON that is hashed
  (`TreeItemProps`/`TreeSectionProps.window` has no `skip_serializing_if` that could hide a genuine change), and
  a change in materialised row COUNT (even with `offset`/`total` unchanged) changes the JSON via the child node
  list regardless. No path found where a real window change produces an identical hash.
- **Keyboard navigation over spacers**: no arrow-key/row-walk navigation logic exists in `🌳️Tree/🟦️.tsx` at all
  (only pointer-driven `onClick`/hover-path code was found) — not a gap introduced by this design, and spacers
  correctly render as plain non-interactive `aria-hidden` blocks with no `TreeAlignedRow`/role, so nothing can
  focus or select one.
- **`interaction_domain` / pick synthesis over a windowed tree**: `PanelTreeBuilder::interaction_domain` stamps
  one tree-level `Activate` binding; rows built via `tree_window_section`/`tree_window_item` carry no per-row
  binding, matching P4b's pick-synthesis precedence (own binding → pick → activatable control). No discrepancy
  found between what P1/P3 stamp and what P4b's `treeItemToTreeData`/`dispatchTreePick` expect.
