# 📓️ W3 — browser verification of the virtualised tree windows

Packet **W3-prep / W3-run**, 2026-09-17. Normative input: `📓️design-virtualised-tree.md` §3/§6,
`📓️p4a-tree-element.md` §2 (DOM attribute contract), `📓️p4b-host-wiring.md` §3/§4 (observer, scheduler),
`📓️a3-cad.md`.

Probe: `🐍️tree-window-probe.mjs` (checked in, lane-agnostic — every lane is described by env alone).
Raw evidence: `🗑️generated/w3/<lane>/{report.json,console.txt,*.png}`.

---

## 1. Lanes run

| lane | url | `SEMIO_PROBE_TREE_NS` | guest build | out dir |
|---|---|---|---|---|
| **cad** | `http://127.0.0.1:6020/?plugin=cad` | `cad-play-document` | **restaged by this packet** (see §2) | `🗑️generated/w3/cad` |
| **fem3d — default example** | `http://127.0.0.1:6087/?plugin=fem3d` | `fem3d-play-artifact` | peer's serve, read-only, **not** restaged | `🗑️generated/w3/fem3d-default` |
| **fem3d — House example** | `http://127.0.0.1:6087/?plugin=fem3d` | `fem3d-play-artifact` | peer's serve, read-only, **not** restaged | `🗑️generated/w3/fem3d-house` |

```
cd ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING"
SEMIO_PROBE_URL="http://127.0.0.1:6020/?plugin=cad"    SEMIO_PROBE_TREE_NS=cad-play-document    SEMIO_PROBE_OUT=w3/cad           bun 🐍️tree-window-probe.mjs
SEMIO_PROBE_URL="http://127.0.0.1:6087/?plugin=fem3d"  SEMIO_PROBE_TREE_NS=fem3d-play-artifact  SEMIO_PROBE_OUT=w3/fem3d-default bun 🐍️tree-window-probe.mjs
SEMIO_PROBE_URL="http://127.0.0.1:6087/?plugin=fem3d"  SEMIO_PROBE_TREE_NS=fem3d-play-artifact  SEMIO_PROBE_EXAMPLE=House \
                                                       SEMIO_PROBE_OUT=w3/fem3d-house    bun 🐍️tree-window-probe.mjs
```

**The peer's served fem3d guest is NOT stale.** Zero `.more` row keys, zero `+N` labels, and all twelve
containers carry the full `data-tree-window-*` mirror. The peer restaged after fem's migration; nothing was
restaged for fem by this packet.

## 2. cad restage

`bun nx run @semio-tech/framework-os-dev:activate-cad-react-dev` with
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react CAD_JS_RENDERER_PLAY_PORT=6020`
→ *"Successfully ran target … and 21 tasks it depends on"*, **3 m 9 s**, cache 3/22.
Log: `🗑️generated/w3/activate-cad-react.txt`. Restaged dist confirmed at 03:41 in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/📐️cad/`
(`semio_s_plugin_cad_component.core.wasm` 61.8 MB, `🔣️.json`, `🛂️.descriptor.semio`, `🌉️bridge.js`).

## 3. Step table

| step | law | cad | fem3d default | fem3d House |
|---|---|---|---|---|
| a | boot + Artifact panel visible | **PASS** 34 rows, 9 containers | **PASS** 58 rows, 12 containers | **PASS** 58 rows, 12 containers |
| b | no `.more` key, no `+N` label anywhere in the panel | **PASS** 0/0 | **PASS** 0/0 | **PASS** 0/0 |
| c | spacers mirror `total/offset/length`, rows ≤ 128 | **PASS** 9/9 exact | **PASS** 12/12 exact | **PASS** 12/12 exact |
| d | an OPEN container has `total > length` | **SKIP** document fits | **SKIP** document fits | **PASS** 6 streaming containers |
| e | scroll moves a container's `offset` (host→guest round trip) | **SKIP** ⚠️ §5 defect | **SKIP** ⚠️ §5 defect | **SKIP** ⚠️ §5 defect |
| f | collapse empties (total kept), re-expand restores | **PASS** | **PASS** | **PASS** |
| g | a host-closed container stays closed across a body refresh | **PASS** | **PASS** | **PASS** |
| h | a leaf-row click selects it, no console fault | **PASS** | **PASS** | **PASS** |
| i | no pageerror / non-`[DEBUG]` console error | **FAIL** 1 (dev-serve 404) | **FAIL** 1 (dev-serve 404) | **FAIL** 4 (§6) |
| | **totals** | **6 PASS / 1 FAIL / 2 SKIP** | **6 PASS / 1 FAIL / 2 SKIP** | **7 PASS / 1 FAIL / 1 SKIP** |

Screenshots per lane (same names in all three out dirs): `1-boot.png`, `2-panel.png`, `3-no-scroll.png`
(the ancestor-chain capture taken when step e finds nothing scrollable), `5-collapse.png`, `6-reexpand.png`,
`7-refresh.png`, `8-pick.png`. `4-scroll-bottom.png` is absent everywhere — step e never got that far.

## 4. Container tables (step c evidence)

**cad** — `🗑️generated/w3/cad/report.json` → `containers`, screenshot `2-panel.png`:

| container | length/total | leading | trailing |
|---|---|---|---|
| `cad-play-document.shape` | 1/1 | 0 | 0 |
| `cad-play-document.building` | 11/11 | 0 | 0 |
| `cad-play-document.energy` | 1/1 | 0 | 0 |
| `cad-play-document.structure-classic` | 11/11 | 0 | 0 |
| `cad-play-document.nodes` | 1/1 | 0 | 0 |
| the four `…references.*` sections | 0/1 (closed) | 0 | 1 |

Every closed reference section stamps its `total` with **zero children and a one-row trailing spacer** — §6.1's
closed case, exactly. Zero-row spacers are genuinely not emitted (the five open sections carry no spacer
element at all).

**cad cannot exercise streaming.** `forest_working_scene()` is a handful of objects per pane and the app ships
exactly **one** example (`setActiveExample` has a single `ActionArgOption`,
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2451-2453`), so the
probe had no larger example to switch to. `TYPOLOGY_CATALOG` (the catalogue panel's windowed list) is 8 entries.

**fem3d House** — `containersAfterExample`, screenshot `fem3d-house/2-panel.png`. Switch evidence:
`picked:House changed=true after 17s, rowsBack=44, settled=true after 9s`.

| container | length/total | leading | trailing | spacer arithmetic |
|---|---|---|---|---|
| `fem3d-play-artifact.nodes` | **16/63** | 0 | **47** | ✅ `0 + 16 + 47 = 63` |
| `fem3d-play-artifact.supports` | **8/63** | 0 | **55** | ✅ |
| `fem3d-play-artifact.solids` | **1/8** | 0 | **7** | ✅ |
| `fem3d-play-artifact.load-cases` | 2/3 | 0 | 1 | ✅ |
| `fem3d-play-artifact.combinations` | 1/2 | 0 | 1 | ✅ |
| `uls` | 2/3 | 0 | 1 | ✅ |
| `fem3d-play-artifact.materials` | 0/3 (closed) | 0 | 3 | ✅ |
| `dead` / `live` | 1/1, 2/2 | 0 | 0 | ✅ (no spacer emitted) |

**This is the §6.1 contract working end to end in a real browser**: a 63-node document materialises 16 rows and
pays for the other 47 with a spacer of exactly `47 × treeRowHeightPx`, with no `.more` row and no `+N` label
anywhere. `offset + length ≤ total` holds in every container in every lane, and no container ever exceeded
`TREE_WINDOW_ROWS_MAX` (128).

## 5. ⚠️ Defect — the window observer binds to an element that never scrolls

**Step (e) is unverifiable in the React host as shipped, on every lane**, and the cause is a selector, not the
document size. `3-no-scroll.png` and `report.json → scrollChain` capture it.

The probe shrank the browser to 1600×520 and re-measured; the element
`useTreeWindowObserver` picks still reported `scrollHeight == clientHeight` (fem3d House: **3720 == 3720**,
extent 0). The element that actually scrolls is its **parent**:

```
slot="scroll-area"           overflowY: auto   clientHeight: 466   scrollHeight: 3722   extent: 3256   ← scrolls
slot="scroll-area-viewport"  overflowY: visible clientHeight: 3720  scrollHeight: 3720   extent:    0   ← observed
```

Suspected file:line —

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🟦️.tsx:34-38` — `data-slot="scroll-area"` is the div
  that carries `overflow-y-auto`; **`:43`** — `data-slot="scroll-area-viewport"` is its inner, unbounded
  content div (`min-h-0 min-w-0 w-full`, no height constraint).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1493` —
  `treeWindowScrollViewport` does `root.closest('[data-slot="scroll-area-viewport"]')` **first** and returns
  it, so the `overflow-y: auto|scroll` fallback at `:1495-1500` (which would have found `scroll-area`) is
  never reached.

Consequence, straight from §3 of `📓️p4b-host-wiring.md`: the `scroll` listener and `ResizeObserver` are
attached to a node whose `scrollTop` is permanently `0`, so **no scroll ever produces a new report**;
`treeWindowRequestsForViewport` is always called with `viewportTop = 0` and `viewportHeight =` the whole
content height, so it always answers `offset: 0`. A windowed tree can therefore never stream past its first
page in the React host — the spacers scroll under the user, but the rows behind them are never requested.
A second-order effect: `viewportRows = ceil(3720 / rowHeight)` is reported instead of what fits on screen.

The one-character-class fix (prefer whichever of `scroll-area` / `scroll-area-viewport` actually scrolls, or
select `[data-slot="scroll-area"]`) is **not applied here** — per the packet brief, host code is not patched
by this agent. Once it lands, re-run all three lanes; step (e) needs no probe change.

## 6. Console findings

`[DEBUG]` (AGENTS.md's temporary-log prefix) and a peer's `[DBGARCH]` both ride the `console.error` lane and
are classified as diagnostics, not faults: cad 33, fem3d default 508, fem3d House 1029 such lines. Zero
`pageerror` on every lane. Zero `[DEBUG]` lines mentioning `tree-window`, `fixed-capacity` or
`refreshUi dropped` on any lane.

Real faults:

1. **All three lanes — `404 /🧩️extension-modules/watch`** on the dev serve (both ports). A dev-serve route
   gap, unrelated to this ticket; it is the sole reason cad and fem3d-default fail step (i). (`/favicon.ico`
   404s are filtered out by the probe as a dev-serve asset.)
2. **fem3d House only — three `retained surface render fault, scoped to that surface`**, ~41.8 s in, right
   as the House body lands:
   - `1:fem3d-model` — `items: 4098` vs `max_items: 4097`
   - `1:fem3d-results` — `items: 4098` vs `max_items: 4097`
   - **`1:framework.panel.artifact` — `nodes: 129` vs `max_nodes: 128`** ← on topic

   The third one is the artifact **tree panel** surface overflowing the retained-surface node ceiling by
   exactly one. `max_nodes` is `SURFACE_RECONCILE_FIXED_NODES` at
   `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:583` (`SurfaceReconcileLimits::default`), the same
   128 as `UI_DOCUMENT_NODES` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:95`) and
   `UI_BUILT_CHILDREN_MAX` (`…/🧬️contract/🏗️builder/🦀️.rs:51`). Per-container windowing bounds each
   *container* at 128; the House body has **8 open windowed containers materialising 33 rows between them**
   (`containersAfterExample`), and the whole body still reconciles to 129 surface nodes. Whether the shared
   whole-body budget is the cause was **not** investigated further, per the brief — it is named here as the
   suspected §7-budget gap for whoever owns it. The fault is scoped (the panel still rendered and steps f–h
   all passed afterwards), and the probe cannot work around it.

## 7. Probe hardening (what was a probe bug, not a product bug)

Recorded so the next agent does not re-derive it. Four false failures were found and fixed in the probe:

1. **`[DEBUG]`/`[DBGARCH]` console.error noise** was counted as faults (479–1029 lines per lane).
2. **The panel rail button toggles.** A blind second click on "Artifact" after the example switch *closed* the
   panel the probe had opened; the probe then waited 150 s for rows that could never come. `ensurePanel()` now
   only clicks while the tree is absent.
3. **Reading the tree mid-flight.** A whole-document replace lands ~17 s after the click and in several
   refreshes; comparing a pre-switch reading to a post-switch one invented a `total 16 → 63` "failure".
   The probe now waits for the tree signature to *change*, then to go quiet for 8 s.
4. **Clicking invisible rows.** Rows inside a collapsed container stay in the DOM with a zero box
   (`Error: row f0_0 has no box`); row selection now filters on a measured box.

Two outcomes are recorded as **SKIP**, never as a silent PASS: a document that fits (step d) and a viewport
that cannot scroll (step e). Both print their verdict text into `report.json`.

Also worth propagating: **`grep` in this repo silently returns zero matches without `-a`** (it resolves to
`ugrep`, which treats these files as binary). A first pass here wrongly concluded P4b's host wiring was
missing; with `grep -a` it is all present.

## 8. Verdict

The §6.1 DOM contract, the §6.2 controlled-expansion channel and the §6.3 pick synthesis are **confirmed
working in a real browser on both migrated apps**: correct spacer arithmetic in 33 containers across three
lanes, a genuine 16-of-63 streamed slice on fem3d House, collapse/re-expand, expansion surviving a body
refresh, and domain picks with no fault. Two things are **not** confirmed and both are named above with
file:line: the scroll→`reportWindows`→`refreshUi` round trip (§5, blocked by the observer's viewport
selector) and the whole-body 128-node surface budget under a large document (§6.2).
