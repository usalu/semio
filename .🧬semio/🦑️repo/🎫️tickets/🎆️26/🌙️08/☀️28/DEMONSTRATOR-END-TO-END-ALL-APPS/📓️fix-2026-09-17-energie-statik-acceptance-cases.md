# 📓️ Acceptance suite: `energie` + `statik` pane cases (2026-09-17)

Scope: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` (+ a comment-only wording fix in
`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts`). No app, plugin, brand or build file was
touched; no server was started, restarted or killed.

---

## 1. What landed

`PANE_CASES` now has eight entries in the order the drift guard demands:
`generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik`.

### `energie` (index 3) — four windows

| window kind id | window element id | surface | expectContent | measured |
| --- | --- | --- | --- | --- |
| `energy.model.3d` | `framework.window.energyModel3d` | `world3d` | true | 9 meshes / 9 instances |
| `framework.window.tree` | `framework.window.frameworkWindowTree` | `tree` (new) | true | 46 `role="treeitem"` rows, headed `BESTEST 600 (vashrae-140-5.2)` |
| `framework.window.table` | `framework.window.frameworkWindowTable` | `table` | true | 1 `[data-row-id]` row (`1 / Zone / 129.6 / 1 / true / true`) |
| `energy.simulation` | `framework.window.energySimulation` | `tree` (new) | true | 15 `role="treeitem"` rows, headed `Kein Energiesimulationslauf` |

### `statik` (index 7) — two windows

| window kind id | window element id | surface | expectContent | measured |
| --- | --- | --- | --- | --- |
| `fem3d-model` | `framework.window.fem3dModel` | `world3d` | true | 3 meshes / 47 instances |
| `fem3d-results` | `framework.window.fem3dResults` | `world3d` | true | 3 meshes / 47 instances, caption `Case: dead` |

### New `"tree"` surface kind

Energy's Structure and Energy-simulation windows are **not** surface hosts — `TreeWindowKit` builds a
`TreeView` and the framework `Tree` element (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`,
rows at ~1936/2013/2088) renders one `role="treeitem"` + `data-slot="tree-item-row"` element per row.
There is no `.semio-…-host` / `.semio-…-empty` pair to grade, so a new `treeRowCount()` helper asserts the
`role="tree"` body attaches and counts `role="treeitem"` rows through the existing
`settleContentCount()` re-read loop. Same discipline as the rest of the suite: production DOM only, no
test-only instrumentation.

Also changed (comment-only): the suite header now says "eight live panes" and mentions the tree-row
evidence route; the `ExpectedWindow` docstring points at this note for the two new entries; the e2e
config's "six panes" docstring is reworded to "eight".

---

## 2. Evidence

**Descriptors.**
- `✏️s/🔌️plugins/🏗️fem/🔣️.json` → `s.fem.fem3d@1/*#editor` declares exactly `fem3d-model` and
  `fem3d-results`, both `surfaceKind: "world-3d"`, default layout a 50/50 row. Confirmed against
  `…/🪟️windows/🧱️model/🦀️.rs:17` (`FEM3D_WINDOW_MODEL = "fem3d-model"`).
- `✏️s/🔌️plugins/🔋️energy/🔣️.json` is **STALE** for the energy editor: it lists only three window kinds,
  declares `framework.window.tree` as `surfaceKind: "block-list"`, and omits `energy.model.3d` entirely.
  The editor source is authoritative and disagrees:
  `✏️s/🔌️plugins/🔋️energy/…/✏️editor/🦀️.rs:2565-2568` registers `structure`, `zones`, `simulation` **and**
  `model_window`, and `…/🎭️modes/✏️edit/🦀️.rs:34-44`'s `layout()` puts the model viewport at 0.55 of the
  row with the other three stacked in the remaining column. The live DOM matches the source. Expectations
  were written from source + live DOM, not from the descriptor. *(Worth a separate descriptor-regen pass —
  not done here, out of slice.)*

**Live lanes (read-only, nothing restaged or rebuilt).** `:6106` (energy), `:6087` (fem3d) and `:6029`
(demonstrator) were all up and answered `200`. Probes were fresh Playwright scripts in the session
scratchpad modelled on the ticket probes, launched with `--use-angle=metal` per the
`headless-chromium-swiftshader-webgl` note.

- `:6106` energy, `demo` example: four window bodies exactly as tabled above. `demo` and `bestest-600`
  have **byte-identical** `artifactJson` in the manifest, so the standalone lane grades the same document
  the brand default names. Shell ready at ~15 s, all bodies filled ~2 s later.
- `:6087` fem3d: `demo` boots ready at ~14 s, both world windows filled ~1 s later (3 meshes / 47
  instances). Switching the example combobox to **Concrete Forest** republished both windows within
  ~1.0 s to 2 meshes / 234 instances. No console or page errors either side of the switch.
- `:6029` demonstrator (the running serve — read only): the landing page already renders all eight
  `[data-demonstrator-pane-card]`s in the expected order, so the brand agent's 8-pane
  `DEMONSTRATOR_PANES` is live.
  - `/#energie` → shell `ready` at **11.5 s**, all four windows carrying content at **15.3 s**, zero page
    errors, zero significant console errors. Counts exactly as tabled.
  - `/#statik` → shell `ready` at **4.6 s**, both windows filled at **6.0 s**, zero page errors, zero
    significant console errors.

---

## 3. Timing — and the concrete-forest question for the coordinator

**Timing is a non-issue.** The heavy path in fem3d is the *solve*, not the boot
(`📓️fem3d-interactive-2026-09-16.md` clocks the House example at ~11 s in wasm), and neither authored
assertion waits on a solve: the Results window draws the same solid set as Model from the first frame and
recolours it when a solve lands. Both panes settle far inside the suite's existing
`SHELL_READY_TIMEOUT_MS` (120 s), `SURFACE_CONTENT_TIMEOUT_MS` (60 s) and `TEST_TIMEOUT_MS` (240 s), so
**no per-pane timeout relaxation was added** — nothing here resembles verfolgen's slow tile paint.

**Open question (coordinator's call, deliberately not changed here).** `ENTWERFEN_MIT_BESTAND_STATIK_BRAND`
sets `defaults: { exampleId: "concrete-forest" }`, but the served `/#statik` pane renders **3 meshes / 47
instances**, which is the shape of fem3d's `demo` example; `concrete-forest` measures **2 meshes / 234
instances** on the standalone lane. So the served pane looks like it is still booting `demo` — either the
brand default is not reaching the fem3d shell, or the serve's shell config predates the brand edit. This
does not weaken the authored case (it asserts a non-empty scene and passes on either example), but if the
demonstrator is meant to show the concrete forest, someone should chase it. The energy side cannot be
distinguished this way because `demo` and `bestest-600` are the same bytes.

---

## 4. What was run

- `bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler --lib DOM,ESNext
  --skipLibCheck --allowImportingTsExtensions "♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts"`
  → **clean, exit 0**. The repo-root `tsconfig.json` includes the whole tree, so the spec was checked on
  its own; it imports only `node:fs`, `node:path` and `@playwright/test`.
  Per `feedback-require-warnings-as-proof-of-typecheck`, a **control run** proved the checker really
  analysed the file: a copy under `temp/` with two deliberate errors appended reported exactly
  `TS2322: Type 'string' is not assignable to type 'number'` and
  `TS2322: Type '"nope"' is not assignable to type 'WindowSurfaceKind'` and nothing else. Copy deleted.
  (The e2e config could not be type-checked in isolation — its `@semio-tech/repo-lib` import drags in
  hundreds of pre-existing `TS5097`/`TS2339` diagnostics from the framework tree that are unrelated to the
  one-line comment edit.)
- **Drift guard only**, twice (before and after the note edits), via a throwaway Playwright config under
  `temp/` (since deleted):
  `bunx playwright test --config … -g "drift guard"` → `1 passed`. A throwaway config was necessary
  because the real `🔨️modules/🧪️e2e/🎚️config/🟦️.ts` fails to load outside the Nx target
  (`SyntaxError: Cannot use 'import.meta' outside a module` from the transitive `@semio-tech/repo-lib`
  import under Playwright's own loader) — worth knowing, but out of slice.
- The full Playwright `test-e2e` target was **not** run (a peer has one live and it rebuilds components).
  No dev server was started, recycled or killed; `:6029`, `:6106` and `:6087` were only read.
